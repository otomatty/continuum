mod components;

use crate::components::auth_guard::AuthGuard;
use crate::components::tabs::{Tab, TabList, TabPanel, Tabs};
use components::{DataExportButton, NotificationSettings, PrivacySettings, ProfileSettingsForm};
use leptos::prelude::*;

/**
 * SettingsPage Component
 *
 * DEPENDENCY MAP:
 *
 * Parents (Files that import this component):
 *   └─ app/src/lib.rs
 *
 * Dependencies (External files that this component imports):
 *   ├─ app/src/components/auth_guard/mod.rs
 *   ├─ app/src/pages/settings/components/mod.rs
 *   └─ app/src/components/tabs.rs
 */
#[component]
pub fn SettingsPage() -> impl IntoView {
    view! {
        <AuthGuard>
            <SettingsContent />
        </AuthGuard>
    }
}

/// Settings content component (requires authentication)
#[component]
fn SettingsContent() -> impl IntoView {
    view! {
        <div class="container mx-auto px-4 py-8">
            <h1 class="text-4xl font-bold mb-8">"Settings"</h1>

            <Tabs>
                <TabList>
                    <Tab index=0usize>"Profile"</Tab>
                    <Tab index=1usize>"Privacy"</Tab>
                    <Tab index=2usize>"Notifications"</Tab>
                    <Tab index=3usize>"Data Export"</Tab>
                </TabList>

                <TabPanel index=0usize>
                    <ProfileSettingsForm />
                </TabPanel>

                <TabPanel index=1usize>
                    <PrivacySettings />
                </TabPanel>

                <TabPanel index=2usize>
                    <NotificationSettings />
                </TabPanel>

                <TabPanel index=3usize>
                    <DataExportButton />
                </TabPanel>
            </Tabs>
        </div>
    }
}
