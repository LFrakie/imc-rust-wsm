use wasm_bindgen::prelude::*;

#[wasm_bindgen]
pub fn calcular_imc(peso: f64, altura: f64) -> f64 {
    peso / (altura * altura)
}

