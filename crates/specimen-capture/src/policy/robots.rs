use url::Url;

/// Represents a single rule in a robots.txt file.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum RuleType {
    Allow,
    Disallow,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Rule {
    pub rule_type: RuleType,
    pub path_pattern: String,
}

#[derive(Debug, Clone, PartialEq)]
pub struct AgentRecord {
    pub user_agents: Vec<String>,
    pub rules: Vec<Rule>,
    pub crawl_delay_s: Option<f64>,
}

/// An RFC 9309 compliant robots.txt parser and evaluator.
#[derive(Debug, Clone, Default)]
pub struct RobotsTxt {
    pub records: Vec<AgentRecord>,
}

impl RobotsTxt {
    pub fn parse(text: &str) -> Self {
        let mut records: Vec<AgentRecord> = Vec::new();
        let mut current_uas: Vec<String> = Vec::new();
        let mut current_rules: Vec<Rule> = Vec::new();
        let mut current_delay: Option<f64> = None;

        for raw_line in text.lines() {
            // Remove comments and trim whitespace
            let line = raw_line.split('#').next().unwrap_or("").trim();
            if line.is_empty() {
                continue;
            }

            if let Some((field, value)) = line.split_once(':') {
                let key = field.trim().to_ascii_lowercase();
                let val = value.trim();

                match key.as_str() {
                    "user-agent" => {
                        // If we already had rules, starting a new user-agent means the start of a new record block
                        if !current_rules.is_empty() {
                            records.push(AgentRecord {
                                user_agents: std::mem::take(&mut current_uas),
                                rules: std::mem::take(&mut current_rules),
                                crawl_delay_s: current_delay.take(),
                            });
                        }
                        current_uas.push(val.to_ascii_lowercase());
                    }
                    "disallow" => {
                        if !val.is_empty() {
                            current_rules.push(Rule {
                                rule_type: RuleType::Disallow,
                                path_pattern: val.to_string(),
                            });
                        }
                    }
                    "allow" => {
                        if !val.is_empty() {
                            current_rules.push(Rule {
                                rule_type: RuleType::Allow,
                                path_pattern: val.to_string(),
                            });
                        }
                    }
                    "crawl-delay" => {
                        if let Ok(d) = val.parse::<f64>() {
                            current_delay = Some(d);
                        }
                    }
                    _ => {
                        // Ignore unknown directives (e.g. sitemap, host)
                    }
                }
            }
        }

        if !current_uas.is_empty() || !current_rules.is_empty() {
            records.push(AgentRecord {
                user_agents: current_uas,
                rules: current_rules,
                crawl_delay_s: current_delay,
            });
        }

        RobotsTxt { records }
    }

    /// Checks if a path is allowed for the specified user agent.
    ///
    /// Per RFC 9309:
    /// - Matches the most specific user-agent group (or '*' wildcard fallback).
    /// - The most specific (longest) matching rule applies.
    /// - If an Allow and Disallow rule have the exact same match length, Allow wins.
    pub fn is_allowed(&self, path: &str, user_agent: &str) -> bool {
        let ua_lower = user_agent.to_ascii_lowercase();

        // 1. Find the best matching record
        let mut best_record: Option<&AgentRecord> = None;
        let mut wildcard_record: Option<&AgentRecord> = None;

        for record in &self.records {
            for agent in &record.user_agents {
                if agent == "*" {
                    wildcard_record = Some(record);
                } else if ua_lower.contains(agent) {
                    best_record = Some(record);
                    break;
                }
            }
            if best_record.is_some() {
                break;
            }
        }

        let chosen_record = best_record.or(wildcard_record);
        let rules = match chosen_record {
            Some(r) => &r.rules,
            None => return true, // No matching record means allowed
        };

        // 2. Evaluate rules against path
        let mut longest_match_len = 0;
        let mut allowed = true;

        for rule in rules {
            if path_matches(&rule.path_pattern, path) {
                let match_len = rule.path_pattern.len();
                if match_len > longest_match_len {
                    longest_match_len = match_len;
                    allowed = rule.rule_type == RuleType::Allow;
                } else if match_len == longest_match_len && rule.rule_type == RuleType::Allow {
                    // Allow takes precedence on tie
                    allowed = true;
                }
            }
        }

        allowed
    }

    /// Evaluates permission for a full Url.
    pub fn is_url_allowed(&self, url: &Url, user_agent: &str) -> bool {
        let mut path = url.path().to_string();
        if let Some(query) = url.query() {
            path.push('?');
            path.push_str(query);
        }
        self.is_allowed(&path, user_agent)
    }
}

fn path_matches(pattern: &str, path: &str) -> bool {
    let mut pat_chars = pattern.chars().peekable();
    let mut path_chars = path.chars().peekable();

    while let Some(pc) = pat_chars.next() {
        if pc == '$' && pat_chars.peek().is_none() {
            // End of path anchor
            return path_chars.peek().is_none();
        }

        if pc == '*' {
            let next_pc = match pat_chars.next() {
                Some(c) => c,
                None => return true, // Trailing * matches everything
            };

            // Consume until next_pc is found
            while let Some(&c) = path_chars.peek() {
                if c == next_pc {
                    break;
                }
                path_chars.next();
            }
            continue;
        }

        match path_chars.next() {
            Some(c) if c == pc => {}
            _ => return false,
        }
    }

    true
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_robots_basic_disallow() {
        let robots = RobotsTxt::parse(
            r#"
User-agent: *
Disallow: /admin
Disallow: /private/
Allow: /admin/public
"#,
        );

        assert!(!robots.is_allowed("/admin", "Specimen/1.0"));
        assert!(!robots.is_allowed("/admin/users", "Specimen/1.0"));
        assert!(robots.is_allowed("/admin/public", "Specimen/1.0"));
        assert!(robots.is_allowed("/admin/public/page", "Specimen/1.0"));
        assert!(!robots.is_allowed("/private/secrets", "Specimen/1.0"));
        assert!(robots.is_allowed("/blog", "Specimen/1.0"));
    }

    #[test]
    fn test_robots_specific_agent_override() {
        let robots = RobotsTxt::parse(
            r#"
User-agent: *
Disallow: /

User-agent: specimen
Allow: /public/
Disallow: /
"#,
        );

        assert!(robots.is_allowed("/public/tokens", "Specimen/0.1.0"));
        assert!(!robots.is_allowed("/secret", "Specimen/0.1.0"));
        assert!(!robots.is_allowed("/public/tokens", "Googlebot"));
    }
}
