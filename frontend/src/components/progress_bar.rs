use dioxus::prelude::*;

#[component]
pub fn ProgressBar(progress: f64, label: String) -> Element {
    let percentage = (progress.clamp(0.0, 100.0) as u32).to_string() + "%";
    
    rsx! {
        div {
            style: "width: 100%; margin: 10px 0;",
            div {
                style: "display: flex; justify-content: space-between; margin-bottom: 5px; font-size: 14px; color: #555;",
                span { "{label}" }
                span { "{percentage}" }
            }
            div {
                style: "width: 100%; height: 10px; background-color: #ecf0f1; border-radius: 5px; overflow: hidden; border: 1px solid #bdc3c7;",
                div {
                    style: "width: {percentage}; height: 100%; background-color: #3498db; transition: width 0.3s ease-out;",
                }
            }
        }
    }
}
