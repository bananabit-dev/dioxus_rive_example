use dioxus::prelude::*;
use crate::rive_animation::RiveAnimation;

#[component]
pub fn Hero() -> Element {
    rsx! {
        section { 
            class: "hero",
            style: "padding: 2rem; text-align: center; background-color: #f5f5f5;",
            
            div { 
                class: "hero-content",
                style: "margin-bottom: 2rem;",
                
                h1 { 
                    style: "font-size: 2.5rem; margin-bottom: 1rem; color: #333;",
                    "Rive Animation in Dioxus" 
                }
                
                p { 
                    style: "font-size: 1.2rem; margin-bottom: 1.5rem; color: #666;",
                    "A mobile-first animation example using Rive.js" 
                }
                
                div { 
                    class: "hero-links",
                    style: "display: flex; gap: 1rem; justify-content: center;",
                    
                    a {
                        class: "hero-link",
                        style: "display: inline-block; padding: 0.5rem 1rem; background-color: #4a6cf7; color: white; text-decoration: none; border-radius: 4px; font-weight: bold;",
                        href: "https://dioxuslabs.com/",
                        target: "_blank",
                        "Dioxus"
                    }
                    
                    a {
                        class: "hero-link",
                        style: "display: inline-block; padding: 0.5rem 1rem; background-color: #ff5757; color: white; text-decoration: none; border-radius: 4px; font-weight: bold;",
                        href: "https://rive.app/",
                        target: "_blank",
                        "Rive"
                    }
                }
            }
            
            // Add the Rive animation component
            RiveAnimation {}
        }
    }
}
