use crate::components::{
    badge::{Badge, BadgeVariant},
    table::{Table, TableBody, TableCell, TableHead, TableHeader, TableRow},
};
use crate::concepts::repository::Repository;
use leptos::prelude::*;

/**
 * RepositoryTable Component
 *
 * DEPENDENCY MAP:
 *
 * Parents (Files that import this component):
 *   └─ app/src/pages/repositories/components/mod.rs
 *
 * Dependencies (External files that this component imports):
 *   ├─ app/src/components/table.rs
 *   ├─ app/src/components/badge.rs
 *   └─ app/src/concepts/repository/mod.rs
 */
#[component]
pub fn RepositoryTable(
    repositories: Vec<Repository>,
    #[prop(optional)] on_repo_click: Option<Callback<String>>,
) -> impl IntoView {
    let handle_click = move |full_name: String| {
        if let Some(callback) = on_repo_click.as_ref() {
            callback.run(full_name);
        }
    };

    view! {
        <div class="overflow-x-auto">
            <Table>
                <TableHead>
                    <TableRow>
                        <TableHeader>"Repository"</TableHeader>
                        <TableHeader>"Stars"</TableHeader>
                        <TableHeader>"Language"</TableHeader>
                        <TableHeader>"Contributors"</TableHeader>
                        <TableHeader>"Updated"</TableHeader>
                    </TableRow>
                </TableHead>
                <TableBody>
                    {repositories.into_iter().map(|repo| {
                        let full_name = repo.full_name.clone();
                        view! {
                            <TableRow
                                class="cursor-pointer hover:bg-base-200"
                                on:click=move |_| handle_click(full_name.clone())
                            >
                                <TableCell>
                                    <div>
                                        <a
                                            href=format!("https://github.com/{}", repo.full_name.clone())
                                            target="_blank"
                                            class="font-semibold hover:underline"
                                            on:click=|ev| ev.stop_propagation()
                                        >
                                            {repo.name.clone()}
                                        </a>
                                        {repo.description.clone().map(|desc| view! {
                                            <p class="text-sm text-gray-600 mt-1">{desc}</p>
                                        })}
                                    </div>
                                </TableCell>
                                <TableCell>
                                    <div class="flex items-center gap-1">
                                        <span class="text-yellow-500">{"★"}</span>
                                        <span>{repo.stars}</span>
                                    </div>
                                </TableCell>
                                <TableCell>
                                    {repo.language.clone().map(|lang| view! {
                                        <Badge variant=BadgeVariant::Ghost>{lang}</Badge>
                                    })}
                                </TableCell>
                                <TableCell>
                                    <span>{repo.contributors.len()}</span>
                                </TableCell>
                                <TableCell>
                                    <span class="text-sm text-gray-600">
                                        {repo.updated_at.format("%Y-%m-%d").to_string()}
                                    </span>
                                </TableCell>
                            </TableRow>
                        }
                    }).collect::<Vec<_>>()}
                </TableBody>
            </Table>
        </div>
    }
}
