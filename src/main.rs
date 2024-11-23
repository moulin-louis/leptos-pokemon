use leptos::*;
use thaw::Button;

#[server(TestServerFn, "test_server_fn", "GetJson")]
pub async fn test_server_fn() -> Result<(), ServerFnError> {
    println!("hello from server");
    Ok(())
}

#[component]
fn App() -> impl IntoView {
    let (count, set_count) = create_signal(0);
    logging::log!("app mounted");
    view! {
        <Button on:click=move |_| {
            spawn_local(async {
                test_server_fn().await.unwrap();
            });
        }>
            "Click here to run server fn:" {count}
        </Button>
    }
}

fn main() {
    console_error_panic_hook::set_once();
    mount_to_body(|| {
        view! { <App /> }
    });
    println!("Hello, world!");
}
