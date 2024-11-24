use leptos::*;
use thaw::Button;
use std::result::Result;

async fn make_request() -> Result<String, reqwest::Error> {
    let res = reqwest::Client::new().get("http://localhost:8000").send().await?;
    res.text().await
}

#[component]
fn App() -> impl IntoView {
    let (count, set_count) = create_signal(0);
    let (res, set_res) = create_signal("nothing".to_string());
    logging::log!("app mounted");
    let inc_counter = move |_| {
        set_count.update(|x| *x += 1);
    };

    let async_data = create_resource(|| (), |_| async move {
        make_request().await.unwrap()
    });


    view! {
        <div>
            <Button on_click=inc_counter>"Click here to increment counter:" {count}</Button>
            {move || match async_data() {
                None => view! { <p>"Loading"</p> }.into_view(),
                Some(x) => view! { <p>"data = " {x}</p> }.into_view(),
            }}
        </div>
    }
}

fn main() {
    console_error_panic_hook::set_once();
    mount_to_body(|| {
        view! { <App /> }
    });
    println!("Hello, world!");
}
