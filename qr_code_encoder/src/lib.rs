use wasm_bindgen::prelude::*;
use qrcode_generator::{ QrCodeEcc };g

#[wasm_bindgen(js_name = genQrCode)]
pub fn gen_qr_code(url: String) -> String { 
    qrcode_generator::to_svg_to_string(&url, QrCodeEcc::Low, 1024, None::<&str>).unwrap()

}