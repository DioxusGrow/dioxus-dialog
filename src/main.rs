#![allow(non_snake_case)]

use dioxus::prelude::*;
use dioxus_logger::tracing::{info, Level};

fn main() {
    // Init logger
    dioxus_logger::init(Level::INFO).expect("failed to init logger");
    info!("starting app");
    launch(App);
}

#[component]
fn App() -> Element {
    let mut show_dialog = use_signal(|| false);
    let message = use_signal(|| "hello".to_string());

    rsx! {
        // For Play CDN to try Tailwind only for develop. Delete for production.
        script { src: asset!("https://cdn.tailwindcss.com") }
        // For manganis. Don't delete.
        head::Link { rel: "stylesheet", href: asset!("./assets/tailwind.css") }
        div { class: "flex flex-row p-10 space-x-10",
            button {
                class: " ring-2 px-2 rounded-lg bg-blue-100 hover:bg-blue-200 shadow-md",
                onclick: move |_| *show_dialog.write() = true,
                "Open dialog"
            }
            h1 { "Message: {message}" }
        }
        Dialog { show: show_dialog, message }
    }
}

#[component]
fn Dialog(show: Signal<bool>, message: Signal<String>) -> Element {
    rsx! {
        div {
            class: "flex flex-row justify-center items-center",
            class: if !show() { "hidden" },
            div {
                class: "fixed top-0 left-0 h-full w-full bg-gray-500 opacity-50",
                class: if !show() { "hidden" },
                onclick: move |_| *show.write() = false
            }
            form {
                class: "relative  space-y-6 bg-white p-2 rounded-md",
                class: if !show() { "hidden" },
                id: "cool-form",
                style: "display: flex; flex-direction: column;",

                // You can attach a handler to the entire form
                oninput: move |ev| {
                    println!("Input event: {:#?}", ev);
                },
                onsubmit: move |ev| {
                    println!("Submit event: {:#?}", ev);
                },

                // Regular text inputs with handlers
                label { class: " font-bold text-xl", r#for: "message", "Message" }
                input {
                    class: " ring-2 rounded px-2",
                    r#type: "text",
                    name: "message",
                    oninput: move |ev| *message.write() = ev.value()
                }
                div { class: "flex flex-row space-x-5 ",
                    button {
                        class: "px-2 ring-2 rounded-xl hover:bg-blue-100 shadow-xl hover:shadow-none",
                        onclick: move |_| *show.write() = false,
                        "Cancel"
                    }
                    button {
                        class: "px-2 ring-2 rounded-xl hover:bg-blue-100 shadow-xl hover:shadow-none",
                        onclick: move |_| *show.write() = false,
                        r#type: "submit",
                        value: "Submit",
                        "Submit"
                    }
                }
            }
        }
    }
}
