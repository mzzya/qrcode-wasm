mod utils;

use base64::{ engine::general_purpose };
// use qrcode::{ render::svg, EcLevel, QrCode };
use qrcode_generator::QrCodeEcc;
use wasm_bindgen::prelude::*;

#[wasm_bindgen]
extern "C" {
    #[wasm_bindgen(js_namespace = console)]
    fn log(s: &str);
}

#[wasm_bindgen]
#[derive(Copy, Clone, Debug)]
pub enum FormatEnum {
    Png,
    Svg,
}

#[wasm_bindgen]
#[derive(Debug)]
pub struct Options {
    pub width: Option<usize>,
    pub height: Option<usize>,
    pub format: Option<FormatEnum>,
    pub level: Option<u8>,
}

#[wasm_bindgen]
impl Options {
    #[wasm_bindgen(constructor)]
    pub fn new() -> Options {
        Options::default()
    }
}

impl Default for Options {
    fn default() -> Self {
        Options {
            width: Some(200),
            height: Some(200),
            format: Some(FormatEnum::Png),
            level: Some(1),
        }
    }
}

#[wasm_bindgen]
pub fn qr_code(data: String, _options: Option<Options>) -> String {
    let options = _options.unwrap_or_default();
    let level = match options.level {
        Some(0) => QrCodeEcc::Low,
        Some(1) => QrCodeEcc::Medium,
        Some(2) => QrCodeEcc::Quartile,
        Some(3) => QrCodeEcc::High,
        _ => QrCodeEcc::Medium,
    };
    // log(&format!("Hello, {:?}", &options));
    match options.format.unwrap_or(FormatEnum::Png) {
        FormatEnum::Png => {
            let mut enc = base64::write::EncoderWriter::new(Vec::new(), &general_purpose::STANDARD);

            qrcode_generator
                ::to_png_to_writer_from_str(
                    data,
                    level,
                    options.width.unwrap_or(200),
                    &mut enc
                )
                .unwrap();
            let delegate = enc.finish().unwrap();
            let base64_string = String::from_utf8(delegate).unwrap();
            return format!("data:image/png;base64,{}", base64_string);
        }
        FormatEnum::Svg => {
            let svg: String = qrcode_generator
                ::to_svg_to_string(
                    data,
                    level,
                    options.width.unwrap_or(200),
                    None::<&str>
                )
                .unwrap();
            return svg;
        }
        _ => {
            return "".to_string();
        }
    }
}
