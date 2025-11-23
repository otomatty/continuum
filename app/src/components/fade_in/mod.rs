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
        use std::cell::RefCell;
        use std::rc::Rc;
        use wasm_bindgen::prelude::*;
        use wasm_bindgen::JsCast;

        // SAFETY: WASM is single-threaded, so Rc/RefCell and Closure (which contains *mut u8) are safe to use
        // even though they're not marked as Send. All code runs on the main thread.
        #[derive(Clone)]
        struct SendWrapper<T>(T);
        unsafe impl<T> Send for SendWrapper<T> {}
        unsafe impl<T> Sync for SendWrapper<T> {}

        let observer_rc = SendWrapper(Rc::new(RefCell::new(None::<web_sys::IntersectionObserver>)));
        let closure_rc = SendWrapper(Rc::new(RefCell::new(
            None::<Closure<dyn FnMut(web_sys::js_sys::Array)>>,
        )));

        let effect_closure = SendWrapper({
            let observer_rc = observer_rc.clone();
            let closure_rc = closure_rc.clone();

            move |_| {
                if let Some(element) = node_ref.get() {
                    let element: &web_sys::Element = element.as_ref();

                    // SAFETY: In WASM, Closure is safe to use even though it contains *mut u8
                    // because WASM is single-threaded. We wrap it to assert Send safety.
                    let closure = SendWrapper(Closure::wrap(Box::new(
                        move |entries: web_sys::js_sys::Array| {
                            if let Some(entry) = entries
                                .get(0)
                                .dyn_ref::<web_sys::IntersectionObserverEntry>()
                            {
                                if entry.is_intersecting() {
                                    set_is_visible.set(true);
                                }
                            }
                        },
                    )
                        as Box<dyn FnMut(_)>));

                    let observer_options = {
                        let opts = web_sys::IntersectionObserverInit::new();
                        opts.set_root_margin("0px");
                        opts.set_threshold(&web_sys::js_sys::Array::of1(&0.1_f64.into()));
                        opts
                    };

                    if let Ok(observer) = web_sys::IntersectionObserver::new_with_options(
                        closure.0.as_ref().unchecked_ref(),
                        &observer_options,
                    ) {
                        observer.observe(element);
                        *observer_rc.0.borrow_mut() = Some(observer);
                        *closure_rc.0.borrow_mut() = Some(closure.0);
                    }
                }
            }
        });

        Effect::new(move |arg| effect_closure.0(arg));

        // Clean up observer and closure when component unmounts
        // SAFETY: WASM is single-threaded, so Rc/RefCell are safe to use even though they're not Send
        let cleanup_wrapper = SendWrapper((observer_rc.clone(), closure_rc.clone()));
        on_cleanup(move || {
            let (obs_rc, cl_rc) = &cleanup_wrapper.0;
            if let Some(observer) = obs_rc.0.borrow_mut().take() {
                observer.disconnect();
            }
            // closure is dropped here, which is important for memory management
            cl_rc.0.borrow_mut().take();
        });
    }

    #[cfg(not(feature = "hydrate"))]
    {
        // SSR: show immediately
        set_is_visible.set(true);
    }

    let delay_class = delay
        .as_ref()
        .map(|d| format!("delay-{}", d))
        .unwrap_or_default();
    let base_class = if is_visible.get() {
        "fade-in-visible"
    } else {
        "fade-in"
    };
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
