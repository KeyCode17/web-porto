use dioxus::prelude::*;
use crate::styles::theme;

#[component]
pub fn Chat() -> Element {
    let mut messages: Signal<Vec<(String, String)>> = use_signal(|| vec![]);
    let mut input = use_signal(|| String::new());
    let mut model_loaded = use_signal(|| false);
    let mut loading_model = use_signal(|| false);
    let mut download_progress = use_signal(|| 0.0f64);

    let send_message = move |_| {
        let msg = input.read().clone();
        if msg.trim().is_empty() {
            return;
        }
        messages.push(("You".to_string(), msg.clone()));
        input.set(String::new());
        // Placeholder response
        let response = format!(
            "I'm a placeholder AI. You said: '{}'. The real llama.cpp model will be integrated soon!",
            msg
        );
        messages.push(("AI".to_string(), response));
    };

    rsx! {
        div { style: "padding: 6rem 2rem; min-height: 100vh;",
            div { style: "max-width: 800px; margin: 0 auto;",
                h1 {
                    style: "font-size: 5rem; font-weight: 700; color: {theme::DEEP_NAVY}; text-transform: uppercase; margin-bottom: 1rem;",
                    "CHAT"
                }
                p {
                    style: "color: {theme::MUTED_TEAL}; margin-bottom: 2rem; font-family: {theme::FONT_MONO}; font-size: 0.9rem;",
                    "AI runs entirely in your browser. No data sent to any server."
                }

                if !*model_loaded.read() {
                    if *loading_model.read() {
                        // Loading state with progress bar
                        div {
                            style: "border: 3px solid {theme::DEEP_NAVY}; padding: 2rem; text-align: center;",
                            p {
                                style: "font-weight: 700; color: {theme::DEEP_NAVY}; margin-bottom: 1rem;",
                                "DOWNLOADING MODEL... {download_progress:.0}%"
                            }
                            div {
                                style: "height: 4px; background: {theme::MUTED_TEAL}; width: 100%;",
                                div {
                                    style: "height: 100%; background: {theme::DEEP_NAVY}; width: {download_progress}%; transition: width 0.3s;",
                                }
                            }
                        }
                    } else {
                        // Download button
                        button {
                            onclick: move |_| {
                                // For now, instantly "load" the model
                                // Real implementation will download a WASM model
                                loading_model.set(true);
                                download_progress.set(100.0);
                                model_loaded.set(true);
                                loading_model.set(false);
                            },
                            style: "font-size: 1.2rem; font-weight: 700; padding: 1rem 2rem; background: {theme::DEEP_NAVY}; color: {theme::MINT_WHITE}; border: none; cursor: pointer; text-transform: uppercase; font-family: inherit; letter-spacing: 0.05em;",
                            "DOWNLOAD MODEL (~50MB)"
                        }
                    }
                } else {
                    // Chat interface
                    div {
                        style: "border: 3px solid {theme::DEEP_NAVY}; min-height: 400px; max-height: 60vh; overflow-y: auto; padding: 1rem; margin-bottom: 1rem;",
                        if messages.read().is_empty() {
                            p {
                                style: "color: {theme::MUTED_TEAL}; font-family: {theme::FONT_MONO}; font-size: 0.9rem;",
                                "Ask me anything about this person..."
                            }
                        }
                        for (role, content) in messages.read().iter() {
                            {
                                let border_color = if role == "You" { theme::DEEP_NAVY } else { theme::MUTED_TEAL };
                                rsx! {
                                    div {
                                        style: "margin-bottom: 1rem; padding: 0.8rem; border-left: 3px solid {border_color};",
                                        p {
                                            style: "font-family: {theme::FONT_MONO}; font-size: 0.75rem; color: {theme::MUTED_TEAL}; text-transform: uppercase; margin-bottom: 0.3rem;",
                                            "{role}"
                                        }
                                        p { style: "color: {theme::DEEP_NAVY}; line-height: 1.6;", "{content}" }
                                    }
                                }
                            }
                        }
                    }
                    // Input area
                    div {
                        style: "display: flex; gap: 1rem;",
                        input {
                            style: "flex: 1; padding: 1rem; border: 3px solid {theme::DEEP_NAVY}; font-size: 1rem; font-family: inherit; background: {theme::MINT_WHITE}; color: {theme::DEEP_NAVY}; outline: none;",
                            value: "{input}",
                            placeholder: "Type your message...",
                            oninput: move |e: Event<FormData>| input.set(e.value()),
                            onkeypress: move |e: KeyboardEvent| {
                                if e.key() == Key::Enter {
                                    let msg = input.read().clone();
                                    if msg.trim().is_empty() {
                                        return;
                                    }
                                    messages.push(("You".to_string(), msg.clone()));
                                    input.set(String::new());
                                    let response = format!(
                                        "I'm a placeholder AI. You said: '{}'. The real llama.cpp model will be integrated soon!",
                                        msg
                                    );
                                    messages.push(("AI".to_string(), response));
                                }
                            },
                        }
                        button {
                            onclick: send_message,
                            style: "padding: 1rem 2rem; background: {theme::DEEP_NAVY}; color: {theme::MINT_WHITE}; border: none; cursor: pointer; font-weight: 700; text-transform: uppercase; font-family: inherit;",
                            "SEND"
                        }
                    }
                }
            }
        }
    }
}
