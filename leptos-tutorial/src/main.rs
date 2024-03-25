use leptos::{ev::MouseEvent, *};

#[component]
pub fn App() -> impl IntoView {
    view! {
        <TakesChildren render_prop=|| view! { <p>"Hi there!"</p> }>
            "Some text"
            <span>"A span"</span>
        </TakesChildren>
        <WrapsChildren>
            "A"
            "B"
            "C"
            "D"
        </WrapsChildren>
    }
}

#[component]
pub fn TakesChildren<F, IV>(
    /// Takes a function (type F) that returns anything that can be
    /// converted into a View (type IV)
    render_prop: F,
    /// `children` takes the `Children` type
    children: Children,
) -> impl IntoView
where
    F: Fn() -> IV,
    IV: IntoView,
{
    view! {
        <h2>"Render Prop"</h2>
        { render_prop() }

        <h2>"Children"</h2>
        { children() }
    }
}

#[component]
pub fn WrapsChildren(children: Children) -> impl IntoView {
    let children = children()
        .nodes
        .into_iter()
        .map(|child| view! { <li>{child}</li> })
        .collect_view();
    view! {
        <ul>{children}</ul>
    }
}

fn main() {
    leptos::mount_to_body(App)
}
