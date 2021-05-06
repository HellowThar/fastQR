use wasm_bindgen::prelude::*;
use qrcode_generator::{ QrCodeEcc };

#[wasm_bindgen(js_name = genQrCode)]
pub fn gen_qr_code(url: String) -> String { 
    qrcode_generator::to_svg_to_string(&url, QrCodeEcc::High, 1024, None::<&str>).unwrap()

}