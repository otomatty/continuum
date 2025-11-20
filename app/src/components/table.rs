use leptos::prelude::*;

#[component]
pub fn Table(
    #[prop(optional, into)] class: String,
    children: Children,
) -> impl IntoView {
    let table_class = if class.is_empty() {
        "table".to_string()
    } else {
        format!("table {}", class)
    };
    
    view! {
        <table class=table_class>
            {children()}
        </table>
    }
}

#[component]
pub fn TableHead(
    #[prop(optional, into)] class: String,
    children: Children,
) -> impl IntoView {
    view! {
        <thead class=class>
            {children()}
        </thead>
    }
}

#[component]
pub fn TableBody(
    #[prop(optional, into)] class: String,
    children: Children,
) -> impl IntoView {
    view! {
        <tbody class=class>
            {children()}
        </tbody>
    }
}

#[component]
pub fn TableRow(
    #[prop(optional, into)] class: String,
    children: Children,
) -> impl IntoView {
    view! {
        <tr class=class>
            {children()}
        </tr>
    }
}

#[component]
pub fn TableHeader(
    #[prop(optional, into)] class: String,
    children: Children,
) -> impl IntoView {
    view! {
        <th class=class>
            {children()}
        </th>
    }
}

#[component]
pub fn TableCell(
    #[prop(optional, into)] class: String,
    children: Children,
) -> impl IntoView {
    view! {
        <td class=class>
            {children()}
        </td>
    }
}

