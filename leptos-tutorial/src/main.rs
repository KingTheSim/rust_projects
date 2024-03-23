use leptos::*;

#[component]
pub fn App() -> impl IntoView {
    let (value, set_value) = create_signal(0);
    let is_odd = move || value() & 1 == 1;
    let odd_text = move || if is_odd() { Some("How odd!") } else { None };

    view! {
        <NumericInput/>
        <h1>"Control Flow"</h1>

        // Simple UI to update and show a value
        <button on:click=move |_| set_value.update(|n| *n += 1)>"+1"</button>

        <p>"Value is: " {value}</p>

        <hr/>

        <h2>
            <code>"Option<T>"</code>
        </h2>
        // For any `T` that implements `IntoView`,
        // so does `Option<T>`

        <p>{odd_text}</p>
        // This means you can use `Option` methods on it
        <p>{move || odd_text().map(|text| text.len())}</p>

        <h2>"Conditional Logic"</h2>
        // You can do dynamic conditional if-then-else
        // logic in several ways
        // 
        // a. An "if" expression in a function
        // This will simply re-render every time the value
        // changes, which makes it good for lightweight UI
        <p>{move || if is_odd() { "Odd" } else { "Even" }}
        </p>

        // b. Toggling some kind of class
        // This is smart for an element that's going to
        // toggled often, because it doesn't destroy
        // it in between states
        // (you can find the `hidden` class in `index.html`)
        <p class:red=is_odd>"Appears if odd."</p>

        // c. The <Show/> component
        // This only renders the fallback and the child
        // once, lazily, and toggles between them when
        // needed. This makes it more efficient in many cases
        // than a {move || if ...} block
        <Show when=is_odd fallback=|| view! { <p>"Even Steven."</p> }>
            <p>"Oddment"</p>
        </Show>

        // d. Because `bool::then()` converts a `bool` to
        // `Option`, you can use it to create a show/hide toggled
        {move || is_odd().then(|| view! { <p>"Oddity!"</p> })}

        <h2>"Convert between Types"</h2>
        // e. Note: if branches return different types,
        // you can convert between them with
        // `.into_any()` (for different HTML element types)
        // or `.into_view()` (for all view types)
        {move || match is_odd() {
            true if value() == 1 => view! { <pre>"One"</pre> }.into_any(),
            false if value() == 2 => view! { <p>"Two"</p> }.into_any(),
            _ => view! { <textarea>{value()}</textarea> }.into_any(),
        }}
    }
}

#[component]
fn NumericInput() -> impl IntoView {
    let (value, set_value) = create_signal(Ok(0));

    let on_input = move |ev| set_value(event_target_value(&ev).parse::<i32>());

    view! {
        <h1>"Error Handling"</h1>
        <label>
            "Type a number (or not!)" <input type="number" on:input=on_input/>
            <ErrorBoundary fallback=|errors| {
                view! {
                    <div class="error">
                        <p>"Not a number! Errors: "</p>
                        <ul>
                            {move || {
                                errors
                                    .get()
                                    .into_iter()
                                    .map(|(_, e)| view! { <li>{e.to_string()}</li> })
                                    .collect_view()
                            }}
                        </ul>
                    </div>
                }
            }>

                <p>"You entered " <strong>{value}</strong></p>
            </ErrorBoundary> <p>"You entered " <strong>{value}</strong></p>
        </label>
    }
}

fn main() {
    leptos::mount_to_body(App)
}
