use leptos::{ev::MouseEvent, *};

#[component]
pub fn App() -> impl IntoView {
    let (age, set_age) = create_signal(32);
    let (favorite_number, set_favorite_number) = create_signal(42);
    let clear_handler = move |_| {
        set_age(0);
        set_favorite_number(0);
    };
}

fn main() {
    leptos::mount_to_body(App)
}
