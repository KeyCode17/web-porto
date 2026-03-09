/**
 * Browser-side embedding engine using Transformers.js v3.
 * Loads all-MiniLM-L6-v2 from local /models/ directory.
 * Exposes window.EmbeddingEngine for Rust/WASM to call via wasm-bindgen.
 */

let pipeline = null;
let ready = false;

window.EmbeddingEngine = {
  /**
   * Load the model. Call once on page mount.
   * @returns {Promise<void>}
   */
  async loadModel() {
    const { pipeline: createPipeline } = await import(
      "https://cdn.jsdelivr.net/npm/@huggingface/transformers@3"
    );
    pipeline = await createPipeline("feature-extraction", "/models/all-MiniLM-L6-v2/", {
      local_files_only: true,
      quantized: true,
    });
    ready = true;
  },

  /**
   * Embed a text string into a 384-dim vector.
   * @param {string} text
   * @returns {Promise<string>} JSON array of 384 floats
   */
  async embed(text) {
    if (!pipeline) throw new Error("Model not loaded");
    const output = await pipeline(text, { pooling: "mean", normalize: true });
    const arr = Array.from(output.data);
    return JSON.stringify(arr);
  },

  /**
   * @returns {boolean}
   */
  isReady() {
    return ready;
  },
};
