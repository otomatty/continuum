use leptos::prelude::*;

/**
 * MarkdownRenderer Component
 *
 * DEPENDENCY MAP:
 *
 * Parents (Files that import this component):
 *   └─ app/src/pages/knowledge_detail/mod.rs
 *
 * Dependencies (External files that this component imports):
 *   └─ leptos::prelude
 */
#[component]
pub fn MarkdownRenderer(
    content: String,
) -> impl IntoView {
    // Simple markdown rendering - basic implementation
    // For production, consider using a markdown parser library like pulldown-cmark
    let html_content = parse_markdown(&content);

    view! {
        <div class="prose prose-lg max-w-none">
            <div inner_html=html_content />
        </div>
    }
}

fn parse_markdown(markdown: &str) -> String {
    // Basic markdown parsing - convert to HTML
    // This is a simplified implementation. For production, use a proper markdown parser.
    let html = markdown.to_string();
    
    // Convert line breaks to paragraphs
    let paragraphs: Vec<&str> = html.split("\n\n").collect();
    let mut result = String::new();
    
    for para in paragraphs {
        let mut para_html = para.to_string();
        
        // Headers (simple check for # at start)
        if para_html.starts_with("### ") {
            para_html = para_html.replacen("### ", "<h3>", 1);
            para_html.push_str("</h3>");
        } else if para_html.starts_with("## ") {
            para_html = para_html.replacen("## ", "<h2>", 1);
            para_html.push_str("</h2>");
        } else if para_html.starts_with("# ") {
            para_html = para_html.replacen("# ", "<h1>", 1);
            para_html.push_str("</h1>");
        } else {
            // Bold
            para_html = para_html.replace("**", "<strong>");
            let mut bold_count = 0;
            let mut new_para = String::new();
            for ch in para_html.chars() {
                if ch == '<' {
                    bold_count += 1;
                    new_para.push_str(if bold_count % 2 == 1 { "<strong>" } else { "</strong>" });
                } else {
                    new_para.push(ch);
                }
            }
            para_html = new_para;
            
            // Inline code
            let mut in_code = false;
            let mut code_buffer = String::new();
            let mut final_para = String::new();
            for ch in para_html.chars() {
                if ch == '`' {
                    if in_code {
                        final_para.push_str(&format!("<code>{}</code>", code_buffer));
                        code_buffer.clear();
                        in_code = false;
                    } else {
                        in_code = true;
                    }
                } else if in_code {
                    code_buffer.push(ch);
                } else {
                    final_para.push(ch);
                }
            }
            para_html = final_para;
            
            // Wrap in paragraph if not a header
            para_html = format!("<p>{}</p>", para_html);
        }
        
        result.push_str(&para_html);
    }
    
    result
}

