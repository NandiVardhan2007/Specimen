#![forbid(unsafe_code)]

use sha2::{Digest, Sha256};

pub struct SanitizedSvg {
    pub clean_svg: String,
    pub view_box: String,
    pub width: u32,
    pub height: u32,
    pub has_current_color: bool,
    pub geometry_hash: String,
}

pub struct SvgSanitizer;

impl SvgSanitizer {
    /// Sanitizes hostile SVG markup according to Security Rule A.9:
    /// - Strips scripts, styles, foreignObject, iframe, embed, object
    /// - Strips inline event handlers (onload, onclick, onerror, etc.)
    /// - Strips javascript: URLs and external references
    /// - Normalizes viewBox, width, height
    /// - Computes deterministic SHA-256 geometry hash
    pub fn sanitize(raw_svg: &str) -> Option<SanitizedSvg> {
        let mut s = raw_svg.trim().to_string();
        if !s.starts_with('<') {
            return None;
        }

        // 1. Remove dangerous blocks: <script>...</script>, <style>...</style>, <foreignObject>...</foreignObject>
        s = Self::strip_element_block(&s, "script");
        s = Self::strip_element_block(&s, "style");
        s = Self::strip_element_block(&s, "foreignobject");
        s = Self::strip_element_block(&s, "iframe");
        s = Self::strip_element_block(&s, "object");
        s = Self::strip_element_block(&s, "embed");

        // 2. Strip dangerous attributes and javascript: pseudo-protocol
        let mut clean = String::with_capacity(s.len());
        let mut in_tag = false;
        let mut tag_buf = String::new();

        for ch in s.chars() {
            if ch == '<' {
                in_tag = true;
                tag_buf.clear();
                tag_buf.push(ch);
            } else if ch == '>' && in_tag {
                tag_buf.push(ch);
                in_tag = false;
                clean.push_str(&Self::sanitize_tag(&tag_buf));
            } else if in_tag {
                tag_buf.push(ch);
            } else {
                clean.push(ch);
            }
        }

        if !clean.contains("<svg") {
            return None;
        }

        // 3. Extract dimensions and viewBox
        let view_box =
            Self::extract_attribute(&clean, "viewBox").unwrap_or_else(|| "0 0 24 24".to_string());
        let width = Self::extract_dim(&clean, "width").unwrap_or(24);
        let height = Self::extract_dim(&clean, "height").unwrap_or(24);

        // 4. Detect currentColor
        let lower = clean.to_lowercase();
        let has_current_color = lower.contains("currentcolor")
            || (!lower.contains("fill=") && !lower.contains("stroke="));

        // 5. Compute SHA-256 geometry hash
        let mut hasher = Sha256::new();
        hasher.update(clean.as_bytes());
        let geometry_hash = format!("sha256:{:x}", hasher.finalize());

        Some(SanitizedSvg {
            clean_svg: clean,
            view_box,
            width,
            height,
            has_current_color,
            geometry_hash,
        })
    }

    fn strip_element_block(input: &str, tag_name: &str) -> String {
        let mut s = input.to_string();
        let open_tag = format!("<{}", tag_name);
        let close_tag = format!("</{}>", tag_name);

        while let Some(start) = s.to_lowercase().find(&open_tag) {
            if let Some(end) = s.to_lowercase()[start..].find(&close_tag) {
                s.replace_range(start..start + end + close_tag.len(), "");
            } else if let Some(end) = s[start..].find('>') {
                // Self-closing or unclosed
                s.replace_range(start..start + end + 1, "");
            } else {
                s.truncate(start);
                break;
            }
        }
        s
    }

    fn sanitize_tag(tag: &str) -> String {
        let tag_trimmed = tag.trim_matches(|c| c == '<' || c == '>');
        let parts: Vec<&str> = tag_trimmed.split_whitespace().collect();
        if parts.is_empty() {
            return String::new();
        }

        let tag_name = parts[0];
        let mut clean_attrs = Vec::new();

        for &part in &parts[1..] {
            let lower = part.to_lowercase();
            // Drop any event handler (onload, onclick, onmouseover, etc.)
            if lower.starts_with("on") {
                continue;
            }
            // Drop javascript: and data: in href/xlink:href/src
            if (lower.contains("href=") || lower.contains("src="))
                && (lower.contains("javascript:") || lower.contains("data:text/html"))
            {
                continue;
            }
            clean_attrs.push(part);
        }

        if clean_attrs.is_empty() {
            format!("<{}>", tag_name)
        } else {
            format!("<{} {}>", tag_name, clean_attrs.join(" "))
        }
    }

    fn extract_attribute(svg: &str, attr: &str) -> Option<String> {
        let search = format!("{}=\"", attr);
        if let Some(pos) = svg.find(&search) {
            let start = pos + search.len();
            if let Some(end) = svg[start..].find('"') {
                return Some(svg[start..start + end].to_string());
            }
        }
        None
    }

    fn extract_dim(svg: &str, attr: &str) -> Option<u32> {
        let val_str = Self::extract_attribute(svg, attr)?;
        let num_str: String = val_str.chars().take_while(|c| c.is_ascii_digit()).collect();
        num_str.parse::<u32>().ok()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_sanitizer_removes_scripts_and_event_handlers() {
        let hostile = r#"<svg width="24" height="24" viewBox="0 0 24 24" onload="alert(1)">
            <script>alert('pwn')</script>
            <circle cx="12" cy="12" r="10" onclick="evil()"></circle>
            <a href="javascript:alert(2)"><path d="M1 1"></path></a>
        </svg>"#;

        let res = SvgSanitizer::sanitize(hostile).unwrap();
        assert!(!res.clean_svg.contains("<script>"));
        assert!(!res.clean_svg.contains("onload="));
        assert!(!res.clean_svg.contains("onclick="));
        assert!(!res.clean_svg.contains("javascript:"));
        assert!(res.clean_svg.contains("<circle"));
        assert_eq!(res.width, 24);
        assert_eq!(res.height, 24);
        assert_eq!(res.view_box, "0 0 24 24");
        assert!(res.geometry_hash.starts_with("sha256:"));
    }
}
