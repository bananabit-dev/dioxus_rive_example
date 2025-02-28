use dioxus::prelude::*;

const RIVE_INIT_JS: Asset = asset!("/assets/rive-init.js");

#[component]
pub fn RiveAnimation() -> Element {
    rsx! {
        document::Script { src: RIVE_INIT_JS ,id: "rive-init-script"}
        
        // Add the Rive canvas container
        div {
            class: "rive-container",
            style: "width: 100%; max-width: 500px; margin: 0 auto; padding: 20px 0;",
            canvas {
                id: "rive-canvas",
                width: "100%",
                height: "300px",
                style: "border-radius: 8px; box-shadow: 0 4px 8px rgba(0, 0, 0, 0.1); background-color: #f9f9f9;"
            }
        }
    }
}
