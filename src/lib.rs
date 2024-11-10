mod framework;
mod render;
mod widgets;

use framework::Framework;

#[cfg_attr(target_arch = "wasm32", wasm_bindgen(start))]
pub async fn run() {
    let mut fw = Framework::new();
    fw.async_run().await;
}
