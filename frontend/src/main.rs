use dioxus::prelude::*;
use std::time::Duration;

fn main() {
    dioxus::launch(app);
}

fn app() -> Element {
    let mut is_online = use_signal(|| false);

    use_coroutine(move |_: UnboundedReceiver<()>| async move {
        loop {
            // Poll the backend health endpoint
            let url = "http://127.0.0.1:8000/health";
            match reqwest::get(url).await {
                Ok(resp) => {
                    if resp.status().is_success() {
                        println!("Backend is Online");
                        is_online.set(true);
                    } else {
                        println!("Backend responded with {}", resp.status());
                        is_online.set(false);
                    }
                }
                Err(e) => {
                    println!("Failed to connect to backend: {}", e);
                    is_online.set(false);
                }
            }
            tokio::time::sleep(Duration::from_secs(2)).await;
        }
    });

    rsx! {
        div {
            style: "padding: 20px; font-family: sans-serif;",
            h1 { "Voice Box" }
            div {
                "Backend Status: "
                if *is_online.read() {
                    span { color: "green", "Online" }
                } else {
                    span { color: "red", "Offline" }
                }
            }
        }
    }
}
