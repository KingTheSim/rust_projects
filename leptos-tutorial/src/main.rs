use gloo_timers::future::TimeoutFuture;
use leptos::{html::Input, *};
use uuid::Uuid;

#[component]
pub fn App() -> impl IntoView {
    let name = "Alice".to_string();
    view! {
        <Outer>
            <Inner clone:name>
                <Inmost name=name.clone()/>
            </Inner>
        </Outer>
    }
}

#[component]
pub fn Outer(children: ChildrenFn) -> impl IntoView {
    children()
}

#[component]
pub fn Inner(children: ChildrenFn) -> impl IntoView {
    children()
}

#[component]
pub fn Inmost(name: String) -> impl IntoView {
    view! {
        <p>{name}</p>
    }
}

fn main() {
    leptos::mount_to_body(App)
}
