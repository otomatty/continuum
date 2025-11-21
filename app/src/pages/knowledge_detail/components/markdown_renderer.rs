use leptos::prelude::*;
use pulldown_cmark::{html, Options, Parser};

/**
 * MarkdownRenderer Component
 *
 * DEPENDENCY MAP:
 *
 * Parents (Files that import this component):
 *   └─ app/src/pages/knowledge_detail/mod.rs
 *
 * Dependencies (External files that this component imports):
 *   ├─ leptos::prelude
 *   └─ pulldown_cmark
 */
#[component]
pub fn MarkdownRenderer(content: String) -> impl IntoView {
    let html_content = parse_markdown(&content);

    view! {
        <div class="prose prose-lg max-w-none">
            <div inner_html=html_content />
        </div>
    }
}

fn parse_markdown(markdown: &str) -> String {
    // Use pulldown-cmark for secure and robust markdown parsing
    // This library properly handles HTML escaping and prevents XSS attacks
    let mut options = Options::empty();
    options.insert(Options::ENABLE_STRIKETHROUGH);
    options.insert(Options::ENABLE_TABLES);
    options.insert(Options::ENABLE_FOOTNOTES);
    options.insert(Options::ENABLE_TASKLISTS);

    let parser = Parser::new_ext(markdown, options);
    let mut html_output = String::new();
    html::push_html(&mut html_output, parser);

    html_output
}
