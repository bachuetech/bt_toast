use dioxus::prelude::*;

///Default Waiting Message if None is received at initialization
const DEFAULT_WAITING_MSG: &str = "Please wait while your request is processed...";

///The ToastMgr struct is used to manage toast notifications in a Dioxus application using Bootstrap. It has four properties:
///    show_toast: A boolean indicating whether a toast notification should be shown.
///    toast_message: A string containing the message to be displayed in the toast notification.
///    toast_type: A character indicating the type of toast notification (I for info, S for success, W for waiting, C for caution/warning, E for error).
///    wait_msg: A string containing the default waiting message.
#[derive(Clone, PartialEq, Debug)]
pub struct ToastMgr{
    show_toast: bool,
    toast_message: String,
    toast_type: char,
    wait_msg: String,
}

impl ToastMgr {
    ///Creates a new instance of ToastMgr with an optional default waiting message. 
    /// If no default waiting message is provided, it defaults to "Please wait while your request is processed...".
    pub fn new(default_waiting_msg: Option<String>) -> Self{
        ToastMgr{
            show_toast: false,
            toast_message: String::new(),
            toast_type: 'N', //None
            wait_msg: default_waiting_msg.unwrap_or(DEFAULT_WAITING_MSG.to_owned()),
        }
    }

    ///Displays an info toast notification with the specified message.
    pub fn show_info(&mut self, msg: String){
        self.toast_message = msg;
        self.toast_type = 'I'; //Info
        self.show_toast = true;
    }

    ///Displays a success toast notification with the specified message.
    pub fn show_success(&mut self, msg: String){
        self.toast_message = msg;
        self.toast_type = 'S'; //Success
        self.show_toast = true;
    }

    ///Displays a warning toast notification with the specified message.
    pub fn show_warning(&mut self, msg: String){
        self.toast_message = msg;
        self.toast_type = 'C'; //Caution / WARNING
        self.show_toast = true;
    }        

    ///Displays an error toast notification with the specified message.
    pub fn show_error(&mut self, msg: String){
        self.toast_message = msg;
        self.toast_type = 'E'; //Error
        self.show_toast = true;
    }    
    
    ///Displays a waiting toast notification with the predefined waiting message.
    pub fn waiting_toast(&mut self){
        self.toast_type = 'W'; //Waiting
        self.show_toast = true;
    }

    ///Turns off any currently displayed toast notifications.
    pub fn turn_off(&mut self){
        if self.show_toast {
            self.show_toast = false;
        }
        if self.toast_type != 'N'{
            self.toast_type = 'N';
        }
    }

    ///Returns a boolean indicating whether a toast notification is scheduled to be shown.
    pub fn will_show(&self) -> bool{
        self.show_toast && self.toast_type != 'N'
    } 
}


#[component]
pub fn Toast2(mut signal_manager: Signal<ToastMgr>) -> Element {
    if !signal_manager.read().show_toast || signal_manager.read().toast_type == 'N'{
        return rsx! {};
    }

    match signal_manager.read().toast_type {
        'I' => {
            return rsx! {
                div { class: "toast-container position-fixed bottom-0 end-0 p-3",
                    div {
                        class: "toast align-items-center text-bg-info border-0 show",
                        role: "alert",
                        "aria-live": "assertive",
                        "aria-atomic": "true",
                        style: "min-width: 250px;",
                        div { class: "d-flex",
                            div { class: "toast-body", "{signal_manager.read().toast_message}" }
                            button {
                                r#type: "button",
                                class: "btn-close btn-close-white me-2 m-auto",
                                "aria-label": "Close",
                                onclick: move |_| signal_manager.write().show_toast = false,
                            }
                        }
                    }
                }
            }
        },        
        'S' => {
            return rsx! {
                div { class: "toast-container position-fixed bottom-0 end-0 p-3",
                    div {
                        class: "toast align-items-center text-bg-success border-0 show",
                        role: "alert",
                        "aria-live": "assertive",
                        "aria-atomic": "true",
                        style: "min-width: 250px;",
                        div { class: "d-flex",
                            div { class: "toast-body", "{signal_manager.read().toast_message}" }
                            button {
                                r#type: "button",
                                class: "btn-close btn-close-white me-2 m-auto",
                                "aria-label": "Close",
                                onclick: move |_| signal_manager.write().show_toast = false,
                            }
                        }
                    }
                }
            }
        },
        'W' => {
            return rsx! {
                div { class: "toast-container  position-fixed  bottom-0 end-0 p-3",
                    div {
                        class: "toast align-items-center text-bg-primary  border-0 show",
                        role: "alert",
                        "aria-live": "assertive",
                        "aria-atomic": "true",
                        style: "min-width: 250px;",
                        div { class: "d-flex",
                            div { class: "toast-body",
                                div {
                                    class: "spinner-border spinner-border-sm me-2",
                                    role: "status",
                                    "aria-hidden": "true",
                                }
                                "{signal_manager.read().wait_msg}"
                            }
                            button {
                                r#type: "button",
                                class: "btn-close btn-close-white me-2 m-auto",
                                "aria-label": "Close",
                                onclick: move |_| signal_manager.write().show_toast = false,
                            }
                        }
                    }
                }
            }            
        },
        'C' => {
            return rsx! {
                div { class: "toast-container position-fixed bottom-0 end-0 p-3",
                    div {
                        class: "toast align-items-center text-bg-warning border-0 show",
                        role: "alert",
                        "aria-live": "assertive",
                        "aria-atomic": "true",
                        style: "min-width: 250px;",
                        div { class: "d-flex",
                            div { class: "toast-body", "{signal_manager.read().toast_message}" }
                            button {
                                r#type: "button",
                                class: "btn-close btn-close-white me-2 m-auto",
                                "aria-label": "Close",
                                onclick: move |_| signal_manager.write().show_toast = false,
                            }
                        }
                    }
                }
            }            
        },        
        'E' => {
            return rsx! {
                div { class: "toast-container position-fixed bottom-0 end-0 p-3",
                    div {
                        class: "toast align-items-center text-bg-danger border-0 show",
                        role: "alert",
                        "aria-live": "assertive",
                        "aria-atomic": "true",
                        style: "min-width: 250px;",
                        div { class: "d-flex",
                            div { class: "toast-body", "{signal_manager.read().toast_message}" }
                            button {
                                r#type: "button",
                                class: "btn-close btn-close-white me-2 m-auto",
                                "aria-label": "Close",
                                onclick: move |_| signal_manager.write().show_toast = false,
                            }
                        }
                    }
                }
            }            
        },
        _ => return rsx! {},
    };
}