use leptos::prelude::*;

/// FadeIn Component
///
/// DEPENDENCY MAP:
///
/// Parents (Files that import this component):
///   └─ app/src/pages/home/components/
///
/// Dependencies (External files that this component imports):
///   └─ leptos::prelude

#[component]
pub fn FadeIn(
    children: Children,
    #[prop(optional, into)] class: Option<String>,
    #[prop(optional, into)] delay: Option<String>,
) -> impl IntoView {
    let (is_visible, set_is_visible) = signal(false);
    let node_ref = NodeRef::<leptos::html::Div>::new();

    #[cfg(feature = "hydrate")]
    {
        use wasm_bindgen::prelude::*;
        use wasm_bindgen::JsCast;
        use wasm_bindgen_futures::spawn_local;

        Effect::new(move |_| {
            let node_ref = node_ref.clone();
            let set_is_visible = set_is_visible.clone();

            spawn_local(async move {
                // Wait for the element to be mounted
                let mut attempts = 0;
                while node_ref.get().is_none() && attempts < 100 {
                    let promise = js_sys::Promise::new(&mut |resolve, _| {
                        if let Some(window) = web_sys::window() {
                            let _ = window.set_timeout_with_callback_and_timeout_and_arguments_0(
                                &resolve,
                                10,
                            );
                        }
                    });
                    let _ = wasm_bindgen_futures::JsFuture::from(promise).await;
                    attempts += 1;
                }

                if let Some(node) = node_ref.get() {
                    let element: &web_sys::Element = node.as_ref();

                    let closure = Closure::wrap(Box::new(move |entries: web_sys::js_sys::Array| {
                        if let Some(entry) = entries.get(0).dyn_ref::<web_sys::IntersectionObserverEntry>() {
                            if entry.is_intersecting() {
                                set_is_visible.set(true);
                            }
                        }
                    }) as Box<dyn FnMut(_)>);

                    let callback = closure.as_ref().unchecked_ref::<js_sys::Function>();
                    let observer_result = web_sys::IntersectionObserver::new_with_options(
                        callback,
                        &web_sys::IntersectionObserverInit::new()
                            .root_margin("0px")
                            .threshold(&web_sys::js_sys::Array::of1(&0.1_f64.into())),
                    );

                    if let Ok(observer) = observer_result {
                        let _ = observer.observe(element);
                    }

                    closure.forget();
                }
            });
        });
    }

    #[cfg(not(feature = "hydrate"))]
    {
        // SSR: show immediately
        set_is_visible.set(true);
    }

    let delay_class = delay.as_ref().map(|d| format!("delay-{}", d)).unwrap_or_default();
    let base_class = if is_visible.get() { "fade-in-visible" } else { "fade-in" };
    let combined_class = if delay_class.is_empty() {
        base_class.to_string()
    } else {
        format!("{} {}", base_class, delay_class)
    };
    let final_class = if let Some(custom_class) = class {
        format!("{} {}", combined_class, custom_class)
    } else {
        combined_class
    };

    view! {
        <div node_ref=node_ref class=final_class>
            {children()}
        </div>
    }
}

