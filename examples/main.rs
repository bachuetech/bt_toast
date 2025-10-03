use bt_toast::{Toast2, ToastMgr};
use dioxus::{document::{Link, Script}, prelude::*};

fn main() {
    launch(App);
}

fn App() -> Element {
    let mut toast_manager = use_signal(||ToastMgr::new(None));
    rsx!(
        Link {
            rel: "stylesheet",
            href: "https://cdn.jsdelivr.net/npm/bootstrap@5.3.8/dist/css/bootstrap.min.css",
            integrity: "sha256-2FMn2Zx6PuH5tdBQDRNwrOo60ts5wWPC9R8jK67b3t4=",
            crossorigin: "anonymous",
        }
        Script {
            src: "https://cdn.jsdelivr.net/npm/bootstrap@5.3.8/dist/js/bootstrap.min.js",
            integrity: "sha256-ew8UiV1pJH/YjpOEBInP1HxVvT/SfrCmwSoUzF9JIgc=",
            crossorigin: "anonymous",
        }

        document::Title { "BT Toast Example" }
        div { class: "form-container body-main",
            div { style: "display: flex; flex-direction: column; align-items: center; justify-content: center; height: 100vh; gap: 10px;",
                button { onclick: move |_| toast_manager.write().show_error("Error".to_string()),
                    "Error"
                }
                button { onclick: move |_| toast_manager.write().show_warning("Warning".to_string()),
                    "Warning"
                }
                button { onclick: move |_| toast_manager.write().show_success("Success".to_string()),
                    "Success"
                }
                button { onclick: move |_| toast_manager.write().show_info("Info".to_string()),
                    "Info"
                }
                button { onclick: move |_| toast_manager.write().waiting_toast(), "Waiting" }
                button {
                    onclick: move |_| {
                        let will_show_is = toast_manager.read().will_show().to_string();
                        toast_manager.write().show_info(will_show_is)
                    },
                    "Will Show?"
                }
                button { onclick: move |_| toast_manager.write().turn_off(), "Turn Off" }
            }
            Toast2 { signal_manager: toast_manager }
        }
    )
}
