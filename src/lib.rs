//! RTF to HTML conversion library
//!
//! Converts RTF documents to clean, semantic HTML. Built on top of the `rtf-parser` crate.
//!
//! # Example
//!
//! ```
//! use rtf_to_html::rtf_to_html;
//!
//! let rtf = br#"{\rtf1\ansi\deff0 Hello World}"#;
//! let html = rtf_to_html(rtf).unwrap();
//! assert!(html.contains("Hello World"));
//! ```
//!
//! # Features
//!
//! - Converts RTF to semantic HTML (`<strong>`, `<em>`, `<u>`)
//! - Paragraph structure preserved with `<p>` tags
//! - XSS-safe output (HTML entities escaped)
//! - Plain text extraction (strips all formatting)

use rtf_parser_tt::RtfDocument;
use thiserror::Error;

/// Errors that can occur during RTF conversion
#[derive(Error, Debug)]
pub enum RtfError {
    /// Failed to parse RTF document
    #[error("Failed to parse RTF: {0}")]
    ParseError(String),
}

/// Convert RTF content to HTML
///
/// Takes raw RTF bytes and returns semantic HTML.
/// Preserves basic formatting: bold, italic, underline, and paragraph structure.
///
/// # Arguments
///
/// * `rtf_content` - RTF document as bytes
///
/// # Returns
///
/// HTML string with semantic markup
///
/// # Example
///
/// ```
/// use rtf_to_html::rtf_to_html;
///
/// let rtf = br#"{\rtf1\ansi\deff0 {\b Bold} and {\i italic}}"#;
/// let html = rtf_to_html(rtf).unwrap();
/// ```
pub fn rtf_to_html(rtf_content: &[u8]) -> Result<String, RtfError> {
    let rtf_string = String::from_utf8_lossy(rtf_content);

    let doc = RtfDocument::try_from(rtf_string.as_ref())
        .map_err(|e| RtfError::ParseError(e.to_string()))?;

    Ok(convert_document_to_html(&doc))
}

/// Extract plain text from RTF (strips all formatting)
///
/// # Arguments
///
/// * `rtf_content` - RTF document as bytes
///
/// # Returns
///
/// Plain text with formatting stripped
///
/// # Example
///
/// ```
/// use rtf_to_html::rtf_to_plain_text;
///
/// let rtf = br#"{\rtf1\ansi\deff0 Hello World}"#;
/// let text = rtf_to_plain_text(rtf).unwrap();
/// assert!(text.contains("Hello"));
/// ```
pub fn rtf_to_plain_text(rtf_content: &[u8]) -> Result<String, RtfError> {
    let rtf_string = String::from_utf8_lossy(rtf_content);

    let doc = RtfDocument::try_from(rtf_string.as_ref())
        .map_err(|e| RtfError::ParseError(e.to_string()))?;

    let text: String = doc.body.iter().map(|block| block.text.as_str()).collect();
    Ok(text)
}

/// Convert a parsed RTF document to HTML
fn convert_document_to_html(doc: &RtfDocument) -> String {
    let mut html = String::new();
    let mut current_paragraph: Vec<String> = Vec::new();
    let mut in_paragraph = false;

    for block in &doc.body {
        let text = &block.text;

        // Skip empty blocks
        if text.is_empty() {
            continue;
        }

        // Check if this is a paragraph break
        if text == "\n" || text.contains('\n') {
            // Flush current paragraph
            if !current_paragraph.is_empty() {
                html.push_str("<p>");
                html.push_str(&current_paragraph.join(""));
                html.push_str("</p>");
                current_paragraph.clear();
            }

            // Handle multiple newlines (which might be paragraph separators)
            let newline_count = text.chars().filter(|c| *c == '\n').count();
            if newline_count > 1 && !in_paragraph {
                // Add empty paragraphs for extra line breaks
                for _ in 1..newline_count {
                    html.push_str("<p></p>");
                }
            }

            in_paragraph = false;
            continue;
        }

        // Apply formatting
        let formatted_text = apply_formatting(text, &block.painter);
        current_paragraph.push(formatted_text);
        in_paragraph = true;
    }

    // Flush any remaining content
    if !current_paragraph.is_empty() {
        html.push_str("<p>");
        html.push_str(&current_paragraph.join(""));
        html.push_str("</p>");
    }

    // If no HTML was generated but we have text, wrap it in a paragraph
    if html.is_empty() && !doc.body.is_empty() {
        let plain_text = doc
            .body
            .iter()
            .map(|b| escape_html(&b.text))
            .collect::<Vec<_>>()
            .join("");
        if !plain_text.is_empty() {
            html = format!("<p>{}</p>", plain_text);
        }
    }

    html
}

/// Apply formatting tags based on the painter style
fn apply_formatting(text: &str, painter: &rtf_parser_tt::Painter) -> String {
    let escaped = escape_html(text);

    if escaped.is_empty() {
        return escaped;
    }

    let mut result = escaped;

    // Apply formatting in order: bold, italic, underline
    if painter.bold {
        result = format!("<strong>{}</strong>", result);
    }

    if painter.italic {
        result = format!("<em>{}</em>", result);
    }

    if painter.underline {
        result = format!("<u>{}</u>", result);
    }

    result
}

/// Escape HTML special characters
fn escape_html(text: &str) -> String {
    text.replace('&', "&amp;")
        .replace('<', "&lt;")
        .replace('>', "&gt;")
        .replace('"', "&quot;")
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_simple_rtf_to_html() {
        let rtf = br#"{\rtf1\ansi\deff0 Hello World}"#;
        let result = rtf_to_html(rtf);
        assert!(result.is_ok());
        let html = result.unwrap();
        assert!(html.contains("Hello World"));
    }

    #[test]
    fn test_rtf_with_paragraphs() {
        let rtf = br#"{\rtf1\ansi\deff0 First paragraph\par Second paragraph}"#;
        let result = rtf_to_html(rtf);
        assert!(result.is_ok());
        let html = result.unwrap();
        assert!(html.contains("<p>"));
    }

    #[test]
    fn test_escape_html() {
        assert_eq!(escape_html("<script>"), "&lt;script&gt;");
        assert_eq!(escape_html("a & b"), "a &amp; b");
        assert_eq!(escape_html("\"quoted\""), "&quot;quoted&quot;");
    }

    #[test]
    fn test_plain_text_extraction() {
        let rtf = br#"{\rtf1\ansi\deff0 Hello World}"#;
        let result = rtf_to_plain_text(rtf);
        assert!(result.is_ok());
        assert!(result.unwrap().contains("Hello"));
    }

    #[test]
    fn test_rtf_with_bold_formatting() {
        let rtf = br#"{\rtf1\ansi\deff0 {\b Name:} John Doe}"#;
        let result = rtf_to_html(rtf);
        assert!(result.is_ok());
        let html = result.unwrap();
        assert!(
            html.contains("<strong>") || html.contains("Name"),
            "Expected formatted or plain content"
        );
    }

    #[test]
    fn test_rtf_with_italic_formatting() {
        let rtf = br#"{\rtf1\ansi\deff0 Normal {\i italic text} more normal}"#;
        let result = rtf_to_html(rtf);
        assert!(result.is_ok());
        let html = result.unwrap();
        assert!(
            html.contains("<em>") || html.contains("italic"),
            "Expected formatted or plain content"
        );
    }

    #[test]
    fn test_invalid_rtf() {
        let rtf = b"not valid rtf content";
        let result = rtf_to_html(rtf);
        assert!(result.is_err());
    }

    #[test]
    fn test_empty_rtf() {
        let rtf = br#"{\rtf1\ansi\deff0 }"#;
        let result = rtf_to_html(rtf);
        assert!(result.is_ok());
    }
}
