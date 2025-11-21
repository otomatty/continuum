use leptos::prelude::*;
use crate::components::card::{Card, CardTitle, CardBody};

/**
 * LanguageBarChart Component
 *
 * DEPENDENCY MAP:
 *
 * Parents (Files that import this component):
 *   └─ app/src/pages/repository/components/mod.rs
 *
 * Dependencies (External files that this component imports):
 *   └─ app/src/components/card.rs
 */
#[component]
pub fn LanguageBarChart(
    languages: Vec<(String, f64)>,
) -> impl IntoView {
    let max_percentage = languages.iter()
        .map(|(_, p)| *p)
        .max_by(|a, b| a.partial_cmp(b).unwrap())
        .unwrap_or(100.0);

    // Generate colors for languages
    let language_colors: std::collections::HashMap<String, &str> = [
        ("Rust".to_string(), "#ce412b"),
        ("TypeScript".to_string(), "#3178c6"),
        ("JavaScript".to_string(), "#f7df1e"),
        ("Python".to_string(), "#3776ab"),
        ("Go".to_string(), "#00add8"),
        ("Java".to_string(), "#ed8b00"),
        ("C++".to_string(), "#00599c"),
        ("C".to_string(), "#a8b9cc"),
        ("Ruby".to_string(), "#cc342d"),
        ("PHP".to_string(), "#777bb4"),
    ].iter().cloned().collect();

    view! {
        <Card>
            <CardTitle>"Language Distribution"</CardTitle>
            <CardBody>
                <div class="space-y-3">
                    {languages.into_iter().map(|(lang, percentage)| {
                        let color = language_colors.get(&lang)
                            .copied()
                            .unwrap_or("#6b7280");
                        let bar_width = (percentage / max_percentage * 100.0).min(100.0);
                        
                        view! {
                            <div>
                                <div class="flex items-center justify-between mb-1">
                                    <span class="text-sm font-medium">{lang.clone()}</span>
                                    <span class="text-sm text-gray-600">{format!("{:.1}%", percentage)}</span>
                                </div>
                                <div class="w-full bg-gray-200 rounded-full h-4 overflow-hidden">
                                    <div 
                                        class="h-full transition-all duration-300"
                                        style=format!("width: {}%; background-color: {}", bar_width, color)
                                    ></div>
                                </div>
                            </div>
                        }
                    }).collect::<Vec<_>>()}
                </div>
            </CardBody>
        </Card>
    }
}

