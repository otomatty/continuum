use leptos::prelude::*;
use crate::components::{button::Button, button::ButtonVariant, card::{Card, CardTitle, CardBody}};

/// Renders the home page of your application.
#[component]
pub fn HomePage() -> impl IntoView {
    // Creates a reactive value to update the button
    let count = RwSignal::new(0);

    view! {
        <div class="space-y-8">
            <div class="text-center py-12">
                <h1 class="text-5xl font-bold mb-4">"Welcome to Continuum"</h1>
                <p class="text-xl text-gray-600 mb-8">
                    "Your GitHub portfolio dashboard powered by Leptos"
                </p>
            </div>

            <div class="grid grid-cols-1 md:grid-cols-2 lg:grid-cols-3 gap-6">
                <Card>
                    <CardTitle>"Dashboard"</CardTitle>
                    <CardBody>
                        <p class="mb-4">"View your GitHub repositories and activity in one place."</p>
                        <a href="/dashboard" class="btn btn-primary">
                            "Go to Dashboard"
                        </a>
                    </CardBody>
                </Card>

                <Card>
                    <CardTitle>"Portfolio"</CardTitle>
                    <CardBody>
                        <p class="mb-4">"Showcase your projects and contributions."</p>
                        <a href="/portfolio/alice-dev" class="btn btn-secondary">
                            "View Portfolio"
                        </a>
                    </CardBody>
                </Card>

                <Card>
                    <CardTitle>"Interactive Demo"</CardTitle>
                    <CardBody>
                        <p class="mb-4">"Click the button below to test reactivity:"</p>
                        <Button 
                            variant=ButtonVariant::Ghost 
                            on_click=Callback::new(move |_| {
                                *count.write() += 1;
                            })
                            >
                                "Click Me: " {count}
                        </Button>
                    </CardBody>
                </Card>
            </div>

            <Card class="mt-8">
                <CardTitle>"Getting Started"</CardTitle>
                <CardBody>
                    <p class="mb-4">
                        "Continuum is a modern portfolio dashboard built with Leptos, Tailwind CSS v4, and DaisyUI."
                    </p>
                    <ul class="list-disc list-inside space-y-2 mb-4">
                        <li>"View your GitHub repositories"</li>
                        <li>"Track your contributions"</li>
                        <li>"Showcase your projects"</li>
                    </ul>
                    <Button variant=ButtonVariant::Primary>
                        "Get Started"
                    </Button>
                </CardBody>
            </Card>
        </div>
    }
}

