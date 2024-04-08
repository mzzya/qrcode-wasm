//! Test suite for the Web and headless browsers.

#![cfg(target_arch = "wasm32")]

extern crate wasm_bindgen_test;
use wasm_bindgen_test::*;
use std::option::Option::Some;

// use qrcode_wasm::qr;
use qrcode_wasm::qr_code;
use qrcode_wasm::Options;
use qrcode_wasm::FormatEnum;

wasm_bindgen_test_configure!(run_in_browser);

#[wasm_bindgen_test]
fn pass() {
    assert_eq!(1 + 1, 2);
}

#[wasm_bindgen_test]
fn qr_code_test() {
    let data1 = qr_code(
        "https://rustwasm.github.io/docs/wasm-bindgen/wasm-bindgen-test/browsers.html".to_string(),
        Some(Options {
            width: Some(200),
            height: Some(200),
            format: Some(FormatEnum::Svg),
            level: Some(1),
        })
    );

    console_log!("qr_code_test png {}", data1);

    let data2 = qr_code(
        "https://rustwasm.github.io/docs/wasm-bindgen/wasm-bindgen-test/browsers.html".to_string(),
        Some(Options {
            width: None,
            height: None,
            format: Some(FormatEnum::Png),
            level: None,
        })
    );
    console_log!("qr_code_test png {}", data2);
}