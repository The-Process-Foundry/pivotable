//! Get a remote file. This is used to download the config file.
//!
//! Based on example code at https://github.com/rustwasm/wasm-bindgen/tree/main/examples/fetch


use wasm_bindgen::prelude::*;
use wasm_bindgen::JsCast;
use wasm_bindgen_futures::JsFuture;
use web_sys::{Request, RequestInit, RequestMode, Response};
use gloo_utils::format::JsValueSerdeExt;


use yew::{Callback, platform::spawn_local};

#[wasm_bindgen]
pub async fn get(path: &str)  -> Result<JsValue, JsValue> {
    let mut opts = RequestInit::new();
    opts.method("GET");
    opts.mode(RequestMode::Cors);

    let request = Request::new_with_str_and_init(path, &opts)?;
    request
        .headers()
        .set("Accept", "text/x-yaml")?;

    let window = web_sys::window().unwrap();
    let resp_value = JsFuture::from(window.fetch_with_request(&request)).await?;

    // `resp_value` is a `Response` object.
    assert!(resp_value.is_instance_of::<Response>());
    let resp: Response = resp_value.dyn_into().unwrap();

    // Convert this other `Promise` into a rust `Future`.
    let text = JsFuture::from(resp.text()?).await?;
    Ok(text)
}


pub fn get_file(path: &'static str,  callback: Callback<Result<String, String>>) {
  spawn_local(async move {
    match get(path).await {
      Err(err) => {
        tracing::error!("Failed to retrieve config: {:?}", err);
        callback.emit(Err(format!("{:?}", err)));
      }
      Ok(json) => {
        tracing::debug!("Retrieved Config: {:?}", json);
        if !json.is_string() {
          let msg = format!("Did not return a text file: {:?}", json.js_typeof());
          tracing::error!("{}", msg);
          return callback.emit(Err(format!("{:?}", msg)));
        }

        let text =
          match json.as_string() {
            Some(text) => text,
            None => {
              tracing::warn!("File did not contain any text");
              String::new()
            }
          };
        callback.emit(Ok(text));
      }
  }})
}
