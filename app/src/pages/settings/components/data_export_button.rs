use crate::components::button::{Button, ButtonVariant};
use leptos::prelude::*;

/**
 * DataExportButton Component
 *
 * DEPENDENCY MAP:
 *
 * Parents (Files that import this component):
 *   └─ app/src/pages/settings/mod.rs
 *
 * Dependencies (External files that this component imports):
 *   └─ app/src/components/button.rs
 */
#[derive(Clone, Copy, PartialEq)]
pub enum ExportFormat {
    Json,
    Csv,
}

#[component]
pub fn DataExportButton(
    #[prop(optional)] on_export: Option<Callback<ExportFormat>>,
) -> impl IntoView {
    let handle_json_export = move |_| {
        if let Some(callback) = on_export.as_ref() {
            callback.run(ExportFormat::Json);
        }
        // In a real implementation, this would trigger a download
    };

    let handle_csv_export = move |_| {
        if let Some(callback) = on_export.as_ref() {
            callback.run(ExportFormat::Csv);
        }
        // In a real implementation, this would trigger a download
    };

    view! {
        <div class="space-y-4">
            <h2 class="text-2xl font-bold">"Data Export"</h2>
            <p class="text-gray-600">"Export your data in various formats"</p>

            <div class="flex gap-4">
                <Button
                    variant=ButtonVariant::Primary
                    on_click=Callback::new(handle_json_export)
                >
                    "Export as JSON"
                </Button>
                <Button
                    variant=ButtonVariant::Secondary
                    on_click=Callback::new(handle_csv_export)
                >
                    "Export as CSV"
                </Button>
            </div>
        </div>
    }
}
