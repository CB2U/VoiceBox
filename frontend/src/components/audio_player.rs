use dioxus::prelude::*;

#[component]
pub fn AudioPlayer(audio_url: String) -> Element {
    rsx! {
        div {
            style: "display: flex; align-items: center; gap: 10px;",
            audio {
                controls: true,
                preload: "metadata",
                style: "max-width: 300px;",
                src: "{audio_url}"
            }
        }
    }
}
