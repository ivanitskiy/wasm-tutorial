use wasm_bindgen::prelude::*;
use web_sys::console;

/// .
///
/// # Examples
///
/// ```
/// use wasm_tutorial::sum;
///
/// assert_eq!(sum(2, 3), 5);
/// ```
#[wasm_bindgen]
pub fn sum(x: u32, y: u32) -> u32 {
    console::log_1(&JsValue::from_str("about to calculate a sum of 2 u32 numbers....!"));
    x + y
}

#[wasm_bindgen(start)]
pub fn run_app() -> Result<(), JsValue> {
    console::log_1(&JsValue::from_str("run_app - is an entry point for the wasm application!"));
    Ok(())
}


#[cfg(test)]
mod tests {
    use crate::sum;
    #[test]
    fn test_sum() {
        let result = sum(2, 2);
        assert_eq!(result, 4);
    }
}
