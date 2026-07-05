use wasm_bindgen::prelude::*;

// 1. javescript integration: exposes the annotated rust function
// 2. type conversion: converts between rust types and javascript types
// 3. memory management: manage memo for the types passed between rust and javascript
// 4. function wrapping: wraps the rust function in sucn a way that it can be imported into javascript as a regular function
#[wasm_bindgen]
pub fn rust_generate_button_text(status: String) -> String {
    match status.to_uppercase().as_str() {
        "PENDING" => "edit".to_string(),
        "DONE" => "delete".to_string(),
        _ => "an error has occured".to_string(),
    }
}
