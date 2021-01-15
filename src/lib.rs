extern crate wasm_bindgen;

use wasm_bindgen::prelude::*;

#[wasm_bindgen]
pub fn calculate_output(input: String, modulus_input: String) -> Result<String, ()> {
    let (base, exponent) = extract_base_and_exponent(input)?;
    let modulus = extract_modulus(modulus_input).unwrap()?;

    (base.pow(exponent) % modulus).to_string()
}

fn extract_base_and_exponent(input: String) -> Result<(u128, u32), ()> {
    let mut components = input.split('^');

    let base_component = components.next().map(str::parse);
    let exponent_component = components.next().map_or(Ok(1), str::parse);

    match (base_component, exponent_component) {
        (Some(Ok(base)), Ok(exponent)) => Ok((base, exponent)),
        _ => Err(()),
    }
}

fn extract_modulus(modulus_input: String) -> Result<u128, ()> {
    str::parse(&modulus_input).map_err(|_| ())
}
