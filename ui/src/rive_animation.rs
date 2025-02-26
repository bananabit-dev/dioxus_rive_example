use dioxus::prelude::*;

#[component]
pub fn RiveAnimation() -> Element {
    let init_script = include_str!("../../web/public/rive-init.js");
    
    // Create a unique ID for the script element
    let script_id = "rive-init-script";
    
    rsx! {
        // Add the script element with our initialization code
        script {
            id: "{script_id}",
            dangerous_inner_html: "{init_script}"
        }
        
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
