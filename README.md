# rtf-to-html

Convert RTF documents to clean, semantic HTML.

[![Crates.io](https://img.shields.io/crates/v/rtf-to-html.svg)](https://crates.io/crates/rtf-to-html)
[![Documentation](https://docs.rs/rtf-to-html/badge.svg)](https://docs.rs/rtf-to-html)
[![License](https://img.shields.io/crates/l/rtf-to-html.svg)](LICENSE-MIT)

## Support

If you find this crate useful, consider supporting development:

- [GitHub Sponsors](https://github.com/sponsors/twelvetake)
- [Ko-fi](https://ko-fi.com/twelvetake)

## Features

- Converts RTF to semantic HTML (`<strong>`, `<em>`, `<u>`)
- Paragraph structure preserved with `<p>` tags
- XSS-safe output (HTML entities escaped)
- Plain text extraction (strips all formatting)
- Zero unsafe code

## Installation

```toml
[dependencies]
rtf-to-html = "0.1"
```

## Usage

### Convert RTF to HTML

```rust
use rtf_to_html::rtf_to_html;

let rtf = br#"{\rtf1\ansi\deff0 {\b Bold} and {\i italic} text}"#;
let html = rtf_to_html(rtf).unwrap();
// Output: <p><strong>Bold</strong> and <em>italic</em> text</p>
```

### Extract Plain Text

```rust
use rtf_to_html::rtf_to_plain_text;

let rtf = br#"{\rtf1\ansi\deff0 Hello World}"#;
let text = rtf_to_plain_text(rtf).unwrap();
// Output: Hello World
```

## Supported Formatting

| RTF | HTML Output |
|-----|-------------|
| `\b` (bold) | `<strong>` |
| `\i` (italic) | `<em>` |
| `\ul` (underline) | `<u>` |
| `\par` (paragraph) | `<p>` |

## Limitations

This crate focuses on basic text formatting. The following are **not** currently supported:

- Hyperlinks (`\field` with HYPERLINK)
- Embedded images or objects
- Tables
- Font sizes and colors
- Lists (numbered/bulleted)

Contributions welcome!

## License

Licensed under either of:

- Apache License, Version 2.0 ([LICENSE-APACHE](LICENSE-APACHE) or http://www.apache.org/licenses/LICENSE-2.0)
- MIT license ([LICENSE-MIT](LICENSE-MIT) or http://opensource.org/licenses/MIT)

at your option.

## Credits

Built by [TwelveTake Studios](https://twelvetake.com). Uses the [`rtf-parser`](https://crates.io/crates/rtf-parser) crate for RTF parsing.
