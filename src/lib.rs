extern crate blurhash;
extern crate console_error_panic_hook;
extern crate serde;
extern crate serde_json;
extern crate wasm_bindgen;

use blurhash::decode;
use gloo_utils::format::JsValueSerdeExt;
use serde::{Deserialize, Serialize};
use std::panic;
use wasm_bindgen::prelude::*;

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct BlurhashCss {
    background_image: String,
    background_position: String,
    background_repeat: String,
    background_size: String,
    filter: String,
    transform: String,
}

fn get_rounded_percentage_of(part: u32, whole: u32) -> u32 {
    let value = part as f32 / whole as f32 * 100 as f32;
    value.round() as u32
}

fn blurhash_to_bytes(hash: &String, width: u32, height: u32) -> Vec<u8> {
    let hash_slice: &str = &hash[..];
    let punch = 1.0;
    decode(hash_slice, width, height, punch)
}

fn get_pixel(pixel_bytes: &[u8], x: u32, y: u32, width: u32, index: u32) -> u8 {
    let number_of_channels = 4; // RGBa
    let bytes_per_row = width * number_of_channels;
    pixel_bytes[(number_of_channels * x + index + y * bytes_per_row) as usize]
}

fn blurhash_to_css_struct(hash: &String, width: u32, height: u32) -> BlurhashCss {
    panic::set_hook(Box::new(console_error_panic_hook::hook));
    let pixel_bytes = blurhash_to_bytes(&hash, width, height);
    let r_index = 0;
    let g_index = 1;
    let b_index = 2;
    let background_size = format!("100% {}%", 100.0 / height as f32);
    let mut background_position = Vec::new();
    let mut linear_gradients = Vec::new();

    for y in 0..height {
        let mut row_linear_gradients = Vec::new();

        for x in 0..width {
            let r = get_pixel(&pixel_bytes, x, y, width, r_index);
            let g = get_pixel(&pixel_bytes, x, y, width, g_index);
            let b = get_pixel(&pixel_bytes, x, y, width, b_index);

            let start_percent = if x == 0 {
                String::from("")
            } else {
                format!(" {}%", get_rounded_percentage_of(x, width))
            };

            let end_percent = if x == width {
                String::from("")
            } else {
                format!(" {}%", get_rounded_percentage_of(x + 1, width))
            };

            let linear_gradient = format!("rgb({},{},{}){}{}", r, g, b, start_percent, end_percent);
            row_linear_gradients.push(linear_gradient);
        }

        linear_gradients.push(format!(
            "linear-gradient(90deg,{})",
            row_linear_gradients.join(",")
        ));

        if y == 0 {
            background_position.push(String::from("0 0"));
        } else {
            background_position.push(format!("0 {}%", get_rounded_percentage_of(y, height - 1)));
        }
    }

    BlurhashCss {
        background_image: linear_gradients.join(","),
        background_position: background_position.join(","),
        background_repeat: String::from("no-repeat"),
        background_size: background_size,
        filter: String::from("blur(24px)"),
        transform: String::from("scale(1.2)"),
    }
}

#[wasm_bindgen]
pub fn blurhash_to_css(hash: String, width: u32, height: u32) -> String {
    let css_struct = blurhash_to_css_struct(&hash, width, height);
    serde_json::to_string(&css_struct).unwrap()
}

#[wasm_bindgen]
pub fn blurhashes_to_css(hashes: JsValue, width: u32, height: u32) -> String {
    let hashes: Vec<String> = hashes.into_serde().unwrap();
    let css_structs: Vec<BlurhashCss> = hashes
        .iter()
        .map(|hash| blurhash_to_css_struct(&hash, width, height))
        .collect();
    serde_json::to_string(&css_structs).unwrap()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn calculates_percentages() {
        assert_eq!(get_rounded_percentage_of(10, 100), 10);
        assert_eq!(get_rounded_percentage_of(25, 200), 13);
    }

    #[test]
    fn decodes_blurhash_into_bytes() {
        let hash = String::from("LEHLh[WB2yk8pyoJadR*.7kCMdnj");
        let width = 400;
        let height = 600;
        let pixel_bytes = blurhash_to_bytes(&hash, width, height);
        let expected_rgba_bytes = 960000;

        assert_eq!(pixel_bytes.len(), expected_rgba_bytes);
    }

    #[test]
    fn converts_blurhash_into_css_struct() {
        let hash = String::from("LEHLh[WB2yk8pyoJadR*.7kCMdnj");
        let width = 10;
        let height = 10;
        let css_struct = blurhash_to_css_struct(&hash, width, height);

        assert_eq!(
            css_struct.background_position,
            "0 0,0 11%,0 22%,0 33%,0 44%,0 56%,0 67%,0 78%,0 89%,0 100%"
        );
        assert_eq!(css_struct.background_repeat, "no-repeat");
        assert_eq!(css_struct.background_size, "100% 10%");
        assert_eq!(css_struct.filter, "blur(24px)");
        assert_eq!(css_struct.background_image, "linear-gradient(90deg,rgb(134,164,177) 10%,rgb(140,165,177) 10% 20%,rgb(153,170,177) 20% 30%,rgb(168,175,176) 30% 40%,rgb(178,179,173) 40% 50%,rgb(181,179,171) 50% 60%,rgb(176,177,170) 60% 70%,rgb(166,173,172) 70% 80%,rgb(152,169,176) 80% 90%,rgb(141,166,179) 90% 100%),linear-gradient(90deg,rgb(133,162,176) 10%,rgb(138,164,176) 10% 20%,rgb(152,168,175) 20% 30%,rgb(166,172,173) 30% 40%,rgb(176,175,170) 40% 50%,rgb(179,176,167) 50% 60%,rgb(174,174,167) 60% 70%,rgb(164,171,170) 70% 80%,rgb(151,167,174) 80% 90%,rgb(140,165,179) 90% 100%),linear-gradient(90deg,rgb(129,160,174) 10%,rgb(135,160,173) 10% 20%,rgb(148,162,169) 20% 30%,rgb(161,163,165) 30% 40%,rgb(171,164,159) 40% 50%,rgb(173,165,156) 50% 60%,rgb(169,165,157) 60% 70%,rgb(159,163,162) 70% 80%,rgb(147,162,170) 80% 90%,rgb(137,161,176) 90% 100%),linear-gradient(90deg,rgb(124,156,170) 10%,rgb(130,155,168) 10% 20%,rgb(142,153,162) 20% 30%,rgb(155,151,153) 30% 40%,rgb(164,149,144) 40% 50%,rgb(166,150,140) 50% 60%,rgb(162,151,142) 60% 70%,rgb(153,153,152) 70% 80%,rgb(142,155,163) 80% 90%,rgb(133,156,172) 90% 100%),linear-gradient(90deg,rgb(121,151,166) 10%,rgb(126,149,163) 10% 20%,rgb(137,144,153) 20% 30%,rgb(149,138,140) 30% 40%,rgb(158,134,127) 40% 50%,rgb(160,134,121) 50% 60%,rgb(156,138,127) 60% 70%,rgb(148,143,140) 70% 80%,rgb(138,148,156) 80% 90%,rgb(129,151,167) 90% 100%),linear-gradient(90deg,rgb(119,147,161) 10%,rgb(124,144,157) 10% 20%,rgb(135,137,146) 20% 30%,rgb(146,129,129) 30% 40%,rgb(155,124,114) 40% 50%,rgb(158,124,107) 50% 60%,rgb(154,130,114) 60% 70%,rgb(146,136,131) 70% 80%,rgb(136,142,149) 80% 90%,rgb(128,146,162) 90% 100%),linear-gradient(90deg,rgb(121,144,157) 10%,rgb(125,142,152) 10% 20%,rgb(135,135,141) 20% 30%,rgb(147,128,124) 30% 40%,rgb(156,124,108) 40% 50%,rgb(159,124,101) 50% 60%,rgb(157,129,108) 60% 70%,rgb(149,135,126) 70% 80%,rgb(138,140,143) 80% 90%,rgb(129,143,156) 90% 100%),linear-gradient(90deg,rgb(125,143,152) 10%,rgb(129,141,149) 10% 20%,rgb(139,137,139) 20% 30%,rgb(151,133,124) 30% 40%,rgb(160,131,111) 40% 50%,rgb(165,133,105) 50% 60%,rgb(163,136,111) 60% 70%,rgb(155,139,125) 70% 80%,rgb(143,141,140) 80% 90%,rgb(131,142,151) 90% 100%),linear-gradient(90deg,rgb(130,143,149) 10%,rgb(134,142,146) 10% 20%,rgb(144,142,139) 20% 30%,rgb(156,141,128) 30% 40%,rgb(166,142,119) 40% 50%,rgb(172,144,115) 50% 60%,rgb(170,146,118) 60% 70%,rgb(162,146,128) 70% 80%,rgb(148,144,139) 80% 90%,rgb(134,142,148) 90% 100%),linear-gradient(90deg,rgb(134,143,147) 10%,rgb(138,144,145) 10% 20%,rgb(147,145,139) 20% 30%,rgb(160,148,132) 30% 40%,rgb(171,151,125) 40% 50%,rgb(177,153,122) 50% 60%,rgb(176,153,125) 60% 70%,rgb(167,151,131) 70% 80%,rgb(152,147,139) 80% 90%,rgb(137,143,145) 90% 100%)");
        assert_eq!(css_struct.transform, "scale(1.2)");
    }

    #[test]
    fn converts_blurhash_into_css_json() {
        let hash = String::from("LEHLh[WB2yk8pyoJadR*.7kCMdnj");
        let width = 10;
        let height = 10;
        let css_json = blurhash_to_css(hash, width, height);

        assert_eq!(css_json, "{\"backgroundImage\":\"linear-gradient(90deg,rgb(134,164,177) 10%,rgb(140,165,177) 10% 20%,rgb(153,170,177) 20% 30%,rgb(168,175,176) 30% 40%,rgb(178,179,173) 40% 50%,rgb(181,179,171) 50% 60%,rgb(176,177,170) 60% 70%,rgb(166,173,172) 70% 80%,rgb(152,169,176) 80% 90%,rgb(141,166,179) 90% 100%),linear-gradient(90deg,rgb(133,162,176) 10%,rgb(138,164,176) 10% 20%,rgb(152,168,175) 20% 30%,rgb(166,172,173) 30% 40%,rgb(176,175,170) 40% 50%,rgb(179,176,167) 50% 60%,rgb(174,174,167) 60% 70%,rgb(164,171,170) 70% 80%,rgb(151,167,174) 80% 90%,rgb(140,165,179) 90% 100%),linear-gradient(90deg,rgb(129,160,174) 10%,rgb(135,160,173) 10% 20%,rgb(148,162,169) 20% 30%,rgb(161,163,165) 30% 40%,rgb(171,164,159) 40% 50%,rgb(173,165,156) 50% 60%,rgb(169,165,157) 60% 70%,rgb(159,163,162) 70% 80%,rgb(147,162,170) 80% 90%,rgb(137,161,176) 90% 100%),linear-gradient(90deg,rgb(124,156,170) 10%,rgb(130,155,168) 10% 20%,rgb(142,153,162) 20% 30%,rgb(155,151,153) 30% 40%,rgb(164,149,144) 40% 50%,rgb(166,150,140) 50% 60%,rgb(162,151,142) 60% 70%,rgb(153,153,152) 70% 80%,rgb(142,155,163) 80% 90%,rgb(133,156,172) 90% 100%),linear-gradient(90deg,rgb(121,151,166) 10%,rgb(126,149,163) 10% 20%,rgb(137,144,153) 20% 30%,rgb(149,138,140) 30% 40%,rgb(158,134,127) 40% 50%,rgb(160,134,121) 50% 60%,rgb(156,138,127) 60% 70%,rgb(148,143,140) 70% 80%,rgb(138,148,156) 80% 90%,rgb(129,151,167) 90% 100%),linear-gradient(90deg,rgb(119,147,161) 10%,rgb(124,144,157) 10% 20%,rgb(135,137,146) 20% 30%,rgb(146,129,129) 30% 40%,rgb(155,124,114) 40% 50%,rgb(158,124,107) 50% 60%,rgb(154,130,114) 60% 70%,rgb(146,136,131) 70% 80%,rgb(136,142,149) 80% 90%,rgb(128,146,162) 90% 100%),linear-gradient(90deg,rgb(121,144,157) 10%,rgb(125,142,152) 10% 20%,rgb(135,135,141) 20% 30%,rgb(147,128,124) 30% 40%,rgb(156,124,108) 40% 50%,rgb(159,124,101) 50% 60%,rgb(157,129,108) 60% 70%,rgb(149,135,126) 70% 80%,rgb(138,140,143) 80% 90%,rgb(129,143,156) 90% 100%),linear-gradient(90deg,rgb(125,143,152) 10%,rgb(129,141,149) 10% 20%,rgb(139,137,139) 20% 30%,rgb(151,133,124) 30% 40%,rgb(160,131,111) 40% 50%,rgb(165,133,105) 50% 60%,rgb(163,136,111) 60% 70%,rgb(155,139,125) 70% 80%,rgb(143,141,140) 80% 90%,rgb(131,142,151) 90% 100%),linear-gradient(90deg,rgb(130,143,149) 10%,rgb(134,142,146) 10% 20%,rgb(144,142,139) 20% 30%,rgb(156,141,128) 30% 40%,rgb(166,142,119) 40% 50%,rgb(172,144,115) 50% 60%,rgb(170,146,118) 60% 70%,rgb(162,146,128) 70% 80%,rgb(148,144,139) 80% 90%,rgb(134,142,148) 90% 100%),linear-gradient(90deg,rgb(134,143,147) 10%,rgb(138,144,145) 10% 20%,rgb(147,145,139) 20% 30%,rgb(160,148,132) 30% 40%,rgb(171,151,125) 40% 50%,rgb(177,153,122) 50% 60%,rgb(176,153,125) 60% 70%,rgb(167,151,131) 70% 80%,rgb(152,147,139) 80% 90%,rgb(137,143,145) 90% 100%)\",\"backgroundPosition\":\"0 0,0 11%,0 22%,0 33%,0 44%,0 56%,0 67%,0 78%,0 89%,0 100%\",\"backgroundRepeat\":\"no-repeat\",\"backgroundSize\":\"100% 10%\",\"filter\":\"blur(24px)\",\"transform\":\"scale(1.2)\"}");
    }
}
