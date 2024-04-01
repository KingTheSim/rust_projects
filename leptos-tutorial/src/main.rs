use leptos::*;
use leptos_router::*;

#[component]
pub fn App() -> impl IntoView {
    view! {
        <Router>
            <h1>
                <code>"<Form/>"</code>
            </h1>
            <main>
                <Routes>
                    <Route path="" view=FormExample/>
                </Routes>
            </main>
        </Router>
    }
}

#[component]
pub fn FormExample() -> impl IntoView {
    let query = use_query_map();
    let name = move || query().get("name").cloned().unwrap_or_default();
    let number = move || query().get("number").cloned().unwrap_or_default();
    let select = move || query().get("select").cloned().unwrap_or_default();

    view! {
        <table>
            <tr>
                <td>
                    <code>"name"</code>
                </td>
                <td>{name}</td>
            </tr>
            <tr>
                <td>
                    <code>"number"</code>
                </td>
                <td>{number}</td>
            </tr>
            <tr>
                <td>
                    <code>"select"</code>
                </td>
                <td>{select}</td>
            </tr>
        </table>
        // <Form/> will navigate whenever submitted
        <h2>"Manual Submissions"</h2>
        <Form method="GET" action="">
            // input names determine query string key
            <input type="text" name="name" value=name/>
            <input type="number" name="number" value=number/>
            <select name="select">
                // `selected` will set which starts as selected
                <option selected=move || select() == "A">"A"</option>
                <option selected=move || select() == "B">"B"</option>
                <option selected=move || select() == "C">"C"</option>
            </select>
            // submitting should cause a client-side
            // navigation, not a full reload
            <input type="submit"/>
        </Form>
        // This <Form/> uses some JavaScript to submit
        // on every input
        <h2>"Automatic Submissions"</h2>
        <Form method="GET" action="">
            <input
                type="text"
                name="name"
                value=name
                // this oninput attribute will cause the
                // form to submit on every input to the field
                oninput="this.form.requestSubmit()"
            />
            <input type="number" name="number" value=number oninput="this.form.requestSubmit()"/>
            <select name="select" onchange="this.form.requestSubmit()">
                <option selected=move || select() == "A">"A"</option>
                <option selected=move || select() == "B">"B"</option>
                <option selected=move || select() == "C">"C"</option>
            </select>
            // submitting should cause a client-side
            // navigation, not a full reload
            <input type="submit"/>
        </Form>
    }
}

fn main() {
    leptos::mount_to_body(|| view! { <App/> })
}
