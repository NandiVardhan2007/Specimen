#![forbid(unsafe_code)]

use std::collections::{HashSet, VecDeque};
use url::Url;

use crate::driver::single_page::{CapturedEvidence, DriverOptions, SinglePageDriver};
use crate::policy::RateLimiter;
use crate::session::{Result, Session};

/// Options controlling the crawl driver behavior (SPEC §4.4).
#[derive(Debug, Clone)]
pub struct CrawlOptions {
    pub max_depth: u32,
    pub max_pages: usize,
    pub same_origin: bool,
    pub include_patterns: Vec<String>,
    pub exclude_patterns: Vec<String>,
    pub template_dedupe: bool,
    pub driver_options: DriverOptions,
}

impl Default for CrawlOptions {
    fn default() -> Self {
        Self {
            max_depth: 2,
            max_pages: 10,
            same_origin: true,
            include_patterns: Vec::new(),
            exclude_patterns: Vec::new(),
            template_dedupe: true,
            driver_options: DriverOptions::default(),
        }
    }
}

/// Aggregated evidence from a multi-page crawl run.
#[derive(Debug, Clone)]
pub struct CrawlGroupEvidence {
    pub start_url: Url,
    pub pages: Vec<CapturedEvidence>,
    pub visited_urls: Vec<Url>,
    pub skipped_duplicates: usize,
}

pub struct CrawlDriver;

impl CrawlDriver {
    /// Executes a breadth-first crawl starting from `start_url`.
    pub async fn run<S: Session>(
        session: &mut S,
        start_url: &Url,
        options: &CrawlOptions,
        rate_limiter: Option<&RateLimiter>,
    ) -> Result<CrawlGroupEvidence> {
        let mut queue = VecDeque::new();
        let mut visited = HashSet::new();
        let mut template_signatures = HashSet::new();
        let mut pages = Vec::new();
        let mut visited_urls = Vec::new();
        let mut skipped_duplicates = 0;

        queue.push_back((start_url.clone(), 0u32));
        visited.insert(normalize_url(start_url));

        while let Some((current_url, depth)) = queue.pop_front() {
            if pages.len() >= options.max_pages {
                break;
            }

            // Capture the page using SinglePageDriver
            let page_evidence = match SinglePageDriver::run(
                session,
                &current_url,
                &options.driver_options,
                rate_limiter,
            )
            .await
            {
                Ok(ev) => ev,
                Err(e) => {
                    eprintln!("Warning: failed to capture {}: {}", current_url, e);
                    continue;
                }
            };

            // Template de-duplication: detect repeated layout templates
            if options.template_dedupe {
                let structural_sig = compute_structural_signature(&page_evidence);
                if template_signatures.contains(&structural_sig) {
                    skipped_duplicates += 1;
                    continue;
                }
                template_signatures.insert(structural_sig);
            }

            visited_urls.push(current_url.clone());
            pages.push(page_evidence.clone());

            // Extract links from captured snapshots if within max_depth
            if depth < options.max_depth {
                let discovered_links = extract_links(&page_evidence, start_url, options);
                for link in discovered_links {
                    let norm = normalize_url(&link);
                    if !visited.contains(&norm) {
                        visited.insert(norm);
                        queue.push_back((link, depth + 1));
                    }
                }
            }
        }

        Ok(CrawlGroupEvidence {
            start_url: start_url.clone(),
            pages,
            visited_urls,
            skipped_duplicates,
        })
    }
}

fn normalize_url(url: &Url) -> String {
    let mut u = url.clone();
    u.set_fragment(None);
    u.set_query(None);
    let mut s = u.to_string();
    if s.ends_with('/') {
        s.pop();
    }
    s.to_ascii_lowercase()
}

fn compute_structural_signature(evidence: &CapturedEvidence) -> String {
    // Generate a simple structural hash based on snapshot node counts and path shape
    let mut total_nodes = 0;
    for (_, _, snap) in &evidence.snapshots {
        total_nodes += snap.node_count;
    }

    let path_depth = evidence.url.path_segments().map(|s| s.count()).unwrap_or(0);
    format!("depth:{}:nodes:{}", path_depth, total_nodes)
}

fn extract_links(evidence: &CapturedEvidence, start_url: &Url, options: &CrawlOptions) -> Vec<Url> {
    let mut links = Vec::new();
    let start_origin = start_url.origin();

    for (_, _, snap) in &evidence.snapshots {
        // Parse href targets from raw JSON or simple pattern
        let raw = &snap.raw_json;
        let mut idx = 0;
        while let Some(href_idx) = raw[idx..].find("href") {
            let abs_idx = idx + href_idx;
            let slice = &raw[abs_idx..];
            if let Some(quote_start) = slice.find('"') {
                let inner = &slice[quote_start + 1..];
                if let Some(quote_end) = inner.find('"') {
                    let candidate = &inner[..quote_end];
                    if let Ok(resolved) = start_url.join(candidate) {
                        if is_url_allowed(&resolved, &start_origin, options) {
                            links.push(resolved);
                        }
                    }
                    idx = abs_idx + quote_start + quote_end + 1;
                } else {
                    break;
                }
            } else {
                break;
            }
        }
    }

    links
}

fn is_url_allowed(url: &Url, start_origin: &url::Origin, options: &CrawlOptions) -> bool {
    let scheme = url.scheme();
    if scheme != "http" && scheme != "https" {
        return false;
    }

    if options.same_origin && url.origin() != *start_origin {
        return false;
    }

    let url_str = url.as_str();

    // Check include patterns
    if !options.include_patterns.is_empty()
        && !options.include_patterns.iter().any(|p| url_str.contains(p))
    {
        return false;
    }

    // Check exclude patterns
    if options.exclude_patterns.iter().any(|p| url_str.contains(p)) {
        return false;
    }

    true
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::session::MockSession;

    #[tokio::test]
    async fn test_crawl_driver_single_origin_cap() {
        let mut session = MockSession::default();
        let start_url = Url::parse("https://example.com/").unwrap();

        let options = CrawlOptions {
            max_depth: 1,
            max_pages: 3,
            same_origin: true,
            template_dedupe: false,
            ..Default::default()
        };

        let result = CrawlDriver::run(&mut session, &start_url, &options, None)
            .await
            .unwrap();

        assert_eq!(result.start_url, start_url);
        assert!(!result.pages.is_empty());
        assert!(result.pages.len() <= options.max_pages);
    }
}
