use dioxus::prelude::*;
use wasm_bindgen::prelude::*;
use wasm_bindgen_futures::JsFuture;
use crate::data::load_faq;
use crate::faq_engine::FaqEngine;
use crate::styles::theme;

/// Inject the EmbeddingEngine JS into the page (runs once)
fn inject_embedding_engine() {
    let window = web_sys::window().unwrap();
    if let Ok(val) = js_sys::Reflect::get(&window, &"EmbeddingEngine".into()) {
        if !val.is_undefined() && !val.is_null() {
            return;
        }
    }
    let result = js_sys::eval(r#"
        (function() {
            let pipeline = null;
            let ready = false;
            window.EmbeddingEngine = {
                async loadModel() {
                    try {
                        const transformers = await import(
                            "https://cdn.jsdelivr.net/npm/@huggingface/transformers@3"
                        );
                        const createPipeline = transformers.pipeline;
                        transformers.env.allowLocalModels = true;
                        pipeline = await createPipeline("feature-extraction", "/models/all-MiniLM-L6-v2", {
                            local_files_only: true,
                            dtype: "fp16",
                        });
                        ready = true;
                    } catch(e) {
                        console.error("[EmbeddingEngine] Load failed:", e);
                        throw e;
                    }
                },
                async embed(text) {
                    if (!pipeline) throw new Error("Model not loaded");
                    const output = await pipeline(text, { pooling: "mean", normalize: true });
                    const arr = Array.from(output.data);
                    return JSON.stringify(arr);
                },
                isReady() {
                    return ready;
                },
            };
        })();
    "#);
    if let Err(e) = result {
        web_sys::console::error_1(&format!("[Rust] Failed to inject EmbeddingEngine: {:?}", e).into());
    }
}

fn get_embedding_engine() -> Option<js_sys::Object> {
    let window = web_sys::window()?;
    let engine = js_sys::Reflect::get(&window, &"EmbeddingEngine".into()).ok()?;
    if engine.is_undefined() || engine.is_null() {
        return None;
    }
    Some(engine.into())
}

fn call_load_model() -> Result<js_sys::Promise, JsValue> {
    let engine = get_embedding_engine()
        .ok_or_else(|| JsValue::from_str("EmbeddingEngine not found"))?;
    let func = js_sys::Reflect::get(&engine, &"loadModel".into())?;
    let func = js_sys::Function::from(func);
    let result = func.call0(&engine)?;
    Ok(js_sys::Promise::from(result))
}

fn call_embed(text: &str) -> Result<js_sys::Promise, JsValue> {
    let engine = get_embedding_engine()
        .ok_or_else(|| JsValue::from_str("EmbeddingEngine not found"))?;
    let func = js_sys::Reflect::get(&engine, &"embed".into())?;
    let func = js_sys::Function::from(func);
    let result = func.call1(&engine, &JsValue::from_str(text))?;
    Ok(js_sys::Promise::from(result))
}

/// Sleep for given milliseconds (WASM-compatible)
async fn sleep_ms(ms: i32) {
    let promise = js_sys::Promise::new(&mut |resolve, _| {
        web_sys::window()
            .unwrap()
            .set_timeout_with_callback_and_timeout_and_arguments_0(&resolve, ms)
            .unwrap();
    });
    let _ = JsFuture::from(promise).await;
}

#[component]
pub fn Chat() -> Element {
    use_hook(|| inject_embedding_engine());

    let mut messages: Signal<Vec<(String, String)>> = use_signal(|| vec![]);
    let mut input = use_signal(|| String::new());
    let mut engine: Signal<FaqEngine> = use_signal(|| FaqEngine::new(load_faq()));
    let mut model_loaded = use_signal(|| false); // actual download done
    let mut model_ready = use_signal(|| false);  // progress animation done, show chat
    let mut model_loading = use_signal(|| false);
    let mut model_error = use_signal(|| false);
    let mut sending = use_signal(|| false);
    let mut progress = use_signal(|| 0.0_f64);

    let start_download = move |_| {
        if *model_loading.read() || *model_ready.read() {
            return;
        }
        model_loading.set(true);
        model_error.set(false);
        progress.set(0.0);

        // Spawn actual model loading
        spawn(async move {
            match call_load_model() {
                Ok(promise) => {
                    match JsFuture::from(promise).await {
                        Ok(_) => {
                            model_loaded.set(true);
                            // Don't set model_loading=false here!
                            // The progress animation task will set model_ready
                            // which transitions the UI to the chat view.
                        }
                        Err(_) => {
                            model_error.set(true);
                            model_loading.set(false);
                        }
                    }
                }
                Err(_) => {
                    model_error.set(true);
                    model_loading.set(false);
                }
            }
        });

        // Spawn fake progress animation
        spawn(async move {
            loop {
                let current = *progress.read();
                let done = *model_loaded.read();
                let error = *model_error.read();

                if error {
                    break;
                }

                if done {
                    // Smooth finish to 100%
                    if current < 100.0 {
                        let step = ((100.0 - current) * 0.08).max(0.3);
                        progress.set((current + step).min(100.0));
                        sleep_ms(30).await;
                    } else {
                        // Brief pause at 100% before showing chat
                        sleep_ms(400).await;
                        model_loading.set(false);
                        model_ready.set(true);
                        break;
                    }
                } else if current < 85.0 {
                    // Normal pace: 0-85%
                    let step = 0.4 + (0.3 * (1.0 - current / 85.0));
                    progress.set(current + step);
                    sleep_ms(50).await;
                } else if current < 99.0 {
                    // Slow crawl: 85-99%
                    let step = 0.05 + (0.1 * (1.0 - (current - 85.0) / 14.0));
                    progress.set(current + step);
                    sleep_ms(200).await;
                } else {
                    // Stuck at 99%, just wait
                    sleep_ms(100).await;
                }
            }
        });
    };

    let mut send_message = move || {
        let msg = input.read().clone();
        if msg.trim().is_empty() || *sending.read() || !*model_ready.read() {
            return;
        }
        messages.push(("You".to_string(), msg.clone()));
        input.set(String::new());

        sending.set(true);
        spawn(async move {
            if let Ok(promise) = call_embed(&msg) {
                match JsFuture::from(promise).await {
                    Ok(val) => {
                        let json = val.as_string().unwrap_or_default();
                        let embedding: Vec<f32> = serde_json::from_str(&json).unwrap_or_default();
                        if !embedding.is_empty() {
                            let answer = engine.with_mut(|e| e.query_with_embedding(&msg, &embedding));
                            messages.push(("AI".to_string(), answer));
                        } else {
                            let answer = engine.with_mut(|e| e.query(&msg));
                            messages.push(("AI".to_string(), answer));
                        }
                    }
                    Err(_) => {
                        let answer = engine.with_mut(|e| e.query(&msg));
                        messages.push(("AI".to_string(), answer));
                    }
                }
            } else {
                let answer = engine.with_mut(|e| e.query(&msg));
                messages.push(("AI".to_string(), answer));
            }
            sending.set(false);
        });
    };

    let is_ready = *model_ready.read();
    let is_loading = *model_loading.read();
    let has_error = *model_error.read();
    let is_sending = *sending.read();
    let pct = *progress.read();

    rsx! {
        div { style: "padding: 6rem 2rem; min-height: 100vh;",
            div { style: "max-width: 800px; margin: 0 auto;",
                h1 {
                    style: "font-size: 5rem; font-weight: 700; color: {theme::DEEP_NAVY}; text-transform: uppercase; margin-bottom: 1rem;",
                    "CHAT"
                }
                p {
                    style: "color: {theme::MUTED_TEAL}; margin-bottom: 1rem; font-family: {theme::FONT_MONO}; font-size: 0.9rem;",
                    "Ask me anything about Karyudi. Powered by AI, runs entirely in your browser. English only."
                }

                // Gate: must download model first
                if !is_ready {
                    div {
                        style: "border: 3px solid {theme::DEEP_NAVY}; min-height: 400px; display: flex; flex-direction: column; align-items: center; justify-content: center; padding: 2rem; margin-bottom: 1rem;",
                        if is_loading {
                            p {
                                style: "font-family: {theme::FONT_MONO}; font-size: 2rem; font-weight: 700; color: {theme::DEEP_NAVY}; margin-bottom: 0.5rem;",
                                "{pct:.1}%"
                            }
                            p {
                                style: "font-family: {theme::FONT_MONO}; font-size: 0.85rem; color: {theme::MUTED_TEAL}; margin-bottom: 1.5rem;",
                                "Loading AI model..."
                            }
                            // Progress bar
                            div {
                                style: "width: 300px; height: 6px; background: {theme::MUTED_TEAL}; position: relative; overflow: hidden;",
                                div {
                                    style: "width: {pct:.1}%; height: 100%; background: {theme::DEEP_NAVY}; transition: width 0.1s linear;",
                                }
                            }
                        } else if has_error {
                            p {
                                style: "font-family: {theme::FONT_MONO}; font-size: 1rem; color: {theme::DEEP_NAVY}; margin-bottom: 1rem;",
                                "Failed to load model. Check console for details."
                            }
                            button {
                                onclick: start_download,
                                style: "padding: 1rem 2rem; background: {theme::DEEP_NAVY}; color: {theme::MINT_WHITE}; border: none; cursor: pointer; font-weight: 700; text-transform: uppercase; font-family: {theme::FONT_MONO}; font-size: 1rem;",
                                "RETRY"
                            }
                        } else {
                            p {
                                style: "font-family: {theme::FONT_MONO}; font-size: 0.9rem; color: {theme::MUTED_TEAL}; margin-bottom: 0.5rem;",
                                "This chatbot uses a neural network to understand your questions."
                            }
                            p {
                                style: "font-family: {theme::FONT_MONO}; font-size: 0.9rem; color: {theme::MUTED_TEAL}; margin-bottom: 2rem;",
                                "The model runs locally in your browser — no data leaves your device."
                            }
                            button {
                                onclick: start_download,
                                style: "padding: 1rem 2rem; background: {theme::DEEP_NAVY}; color: {theme::MINT_WHITE}; border: none; cursor: pointer; font-weight: 700; text-transform: uppercase; font-family: {theme::FONT_MONO}; font-size: 1rem;",
                                "LOAD AI MODEL (~44MB)"
                            }
                        }
                    }
                } else {
                    // Chat interface (only shown after model loaded)
                    div {
                        style: "border: 3px solid {theme::DEEP_NAVY}; min-height: 400px; max-height: 60vh; overflow-y: auto; padding: 1rem; margin-bottom: 1rem;",
                        if messages.read().is_empty() {
                            p {
                                style: "color: {theme::MUTED_TEAL}; font-family: {theme::FONT_MONO}; font-size: 0.9rem;",
                                "Ask me anything about Karyudi..."
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
                                        p {
                                            style: "color: {theme::DEEP_NAVY}; line-height: 1.6; white-space: pre-line;",
                                            "{content}"
                                        }
                                    }
                                }
                            }
                        }
                        if is_sending {
                            div {
                                style: "margin-bottom: 1rem; padding: 0.8rem; border-left: 3px solid {theme::MUTED_TEAL};",
                                p {
                                    style: "font-family: {theme::FONT_MONO}; font-size: 0.75rem; color: {theme::MUTED_TEAL}; text-transform: uppercase; margin-bottom: 0.3rem;",
                                    "AI"
                                }
                                p {
                                    style: "color: {theme::MUTED_TEAL}; font-style: italic;",
                                    "Thinking..."
                                }
                            }
                        }
                    }
                    div {
                        style: "display: flex; gap: 1rem;",
                        input {
                            style: "flex: 1; padding: 1rem; border: 3px solid {theme::DEEP_NAVY}; font-size: 1rem; font-family: inherit; background: {theme::MINT_WHITE}; color: {theme::DEEP_NAVY}; outline: none;",
                            value: "{input}",
                            placeholder: "Type your message...",
                            oninput: move |e: Event<FormData>| input.set(e.value()),
                            onkeypress: move |e: KeyboardEvent| {
                                if e.key() == Key::Enter {
                                    send_message();
                                }
                            },
                        }
                        button {
                            onclick: move |_| send_message(),
                            disabled: is_sending,
                            style: "padding: 1rem 2rem; background: {theme::DEEP_NAVY}; color: {theme::MINT_WHITE}; border: none; cursor: pointer; font-weight: 700; text-transform: uppercase; font-family: inherit;",
                            if is_sending { "..." } else { "SEND" }
                        }
                    }
                }
            }
        }
    }
}
