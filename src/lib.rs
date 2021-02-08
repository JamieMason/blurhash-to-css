extern crate blurhash;
extern crate console_error_panic_hook;
extern crate serde;
extern crate serde_json;
extern crate wasm_bindgen;

use blurhash::decode;
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

fn blurhash_to_bytes(hash: String, width: u32, height: u32) -> Vec<u8> {
    let hash_slice: &str = &hash[..];
    let punch = 1.0;
    decode(hash_slice, width, height, punch)
}

fn get_pixel(pixel_bytes: &[u8], x: u32, y: u32, width: u32, index: u32) -> u8 {
    let number_of_channels = 4; // RGBa
    let bytes_per_row = width * number_of_channels;
    pixel_bytes[(number_of_channels * x + index + y * bytes_per_row) as usize]
}

fn blurhash_to_css_struct(hash: String, width: u32, height: u32) -> BlurhashCss {
    panic::set_hook(Box::new(console_error_panic_hook::hook));
    let pixel_bytes = blurhash_to_bytes(hash, width, height);
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
    let css_struct = blurhash_to_css_struct(hash, width, height);
    serde_json::to_string(&css_struct).unwrap()
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
    fn decodes_blurhashes() {
        let hash = String::from("eCF6B#-:0JInxr?@s;nmIoWUIko1%NocRk.8xbIUaxR*^+s;RiWAWU");
        let width = 400;
        let height = 600;
        let pixel_bytes = blurhash_to_bytes(hash, width, height);
        let expected_rgba_bytes = 960000;

        assert_eq!(pixel_bytes.len(), expected_rgba_bytes);
    }

    #[test]
    fn converts_blurhashes_into_css_structs() {
        let hash = String::from("eCF6B#-:0JInxr?@s;nmIoWUIko1%NocRk.8xbIUaxR*^+s;RiWAWU");
        let width = 10;
        let height = 10;
        let css_struct = blurhash_to_css_struct(hash, width, height);

        assert_eq!(
            css_struct.background_position,
            "0 0,0 11%,0 22%,0 33%,0 44%,0 56%,0 67%,0 78%,0 89%,0 100%"
        );
        assert_eq!(css_struct.background_repeat, "no-repeat");
        assert_eq!(css_struct.background_size, "100% 10%");
        assert_eq!(css_struct.filter, "blur(24px)");
        assert_eq!(css_struct.background_image, "linear-gradient(90deg,rgb(144,157,98) 10%,rgb(153,162,114) 10% 20%,rgb(171,173,140) 20% 30%,rgb(183,183,156) 30% 40%,rgb(183,186,156) 40% 50%,rgb(174,182,144) 50% 60%,rgb(160,171,127) 60% 70%,rgb(147,160,113) 70% 80%,rgb(139,152,104) 80% 90%,rgb(134,149,98) 90% 100%),linear-gradient(90deg,rgb(132,148,87) 10%,rgb(140,152,101) 10% 20%,rgb(156,162,126) 20% 30%,rgb(169,171,141) 30% 40%,rgb(171,175,142) 40% 50%,rgb(163,171,131) 50% 60%,rgb(148,160,115) 60% 70%,rgb(134,149,101) 70% 80%,rgb(127,142,93) 80% 90%,rgb(126,140,89) 90% 100%),linear-gradient(90deg,rgb(109,129,62) 10%,rgb(114,130,73) 10% 20%,rgb(126,136,94) 20% 30%,rgb(139,145,109) 30% 40%,rgb(146,150,114) 40% 50%,rgb(141,148,107) 50% 60%,rgb(125,137,93) 60% 70%,rgb(107,123,78) 70% 80%,rgb(102,118,70) 80% 90%,rgb(109,121,71) 90% 100%),linear-gradient(90deg,rgb(103,119,58) 10%,rgb(105,119,67) 10% 20%,rgb(114,123,86) 20% 30%,rgb(129,132,104) 30% 40%,rgb(141,141,113) 40% 50%,rgb(140,140,111) 50% 60%,rgb(123,128,99) 60% 70%,rgb(101,111,83) 70% 80%,rgb(96,105,73) 80% 90%,rgb(108,112,74) 90% 100%),linear-gradient(90deg,rgb(121,127,83) 10%,rgb(124,128,93) 10% 20%,rgb(134,134,116) 20% 30%,rgb(150,146,135) 30% 40%,rgb(162,157,146) 40% 50%,rgb(162,157,144) 50% 60%,rgb(147,144,132) 60% 70%,rgb(126,125,115) 70% 80%,rgb(117,115,100) 80% 90%,rgb(122,119,94) 90% 100%),linear-gradient(90deg,rgb(132,134,100) 10%,rgb(137,137,113) 10% 20%,rgb(150,146,137) 20% 30%,rgb(167,160,159) 30% 40%,rgb(179,172,169) 40% 50%,rgb(178,172,167) 50% 60%,rgb(164,159,154) 60% 70%,rgb(144,140,136) 70% 80%,rgb(130,126,117) 80% 90%,rgb(127,124,104) 90% 100%),linear-gradient(90deg,rgb(123,128,97) 10%,rgb(128,131,109) 10% 20%,rgb(143,141,133) 20% 30%,rgb(160,155,154) 30% 40%,rgb(173,168,165) 40% 50%,rgb(173,169,164) 50% 60%,rgb(160,157,152) 60% 70%,rgb(139,137,133) 70% 80%,rgb(120,120,111) 80% 90%,rgb(111,113,93) 90% 100%),linear-gradient(90deg,rgb(101,112,74) 10%,rgb(105,114,84) 10% 20%,rgb(117,121,105) 20% 30%,rgb(135,134,125) 30% 40%,rgb(149,146,138) 40% 50%,rgb(151,148,139) 50% 60%,rgb(140,138,128) 60% 70%,rgb(118,119,109) 70% 80%,rgb(95,100,85) 80% 90%,rgb(81,90,63) 90% 100%),linear-gradient(90deg,rgb(94,102,40) 10%,rgb(94,102,51) 10% 20%,rgb(100,104,71) 20% 30%,rgb(114,112,91) 30% 40%,rgb(128,122,104) 40% 50%,rgb(133,125,106) 50% 60%,rgb(124,117,97) 60% 70%,rgb(103,100,78) 70% 80%,rgb(82,84,53) 80% 90%,rgb(71,76,28) 90% 100%),linear-gradient(90deg,rgb(109,106,16) 10%,rgb(108,105,34) 10% 20%,rgb(109,103,59) 20% 30%,rgb(117,106,78) 30% 40%,rgb(128,113,88) 40% 50%,rgb(132,115,87) 50% 60%,rgb(124,108,77) 60% 70%,rgb(109,96,59) 70% 80%,rgb(95,86,40) 80% 90%,rgb(90,83,26) 90% 100%)");
        assert_eq!(css_struct.transform, "scale(1.2)");
    }

    #[test]
    fn converts_blurhashes_into_css_json() {
        let hash = String::from("eCF6B#-:0JInxr?@s;nmIoWUIko1%NocRk.8xbIUaxR*^+s;RiWAWU");
        let width = 10;
        let height = 10;
        let css_json = blurhash_to_css(hash, width, height);

        assert_eq!(css_json, "{\"backgroundImage\":\"linear-gradient(90deg,rgb(144,157,98) 10%,rgb(153,162,114) 10% 20%,rgb(171,173,140) 20% 30%,rgb(183,183,156) 30% 40%,rgb(183,186,156) 40% 50%,rgb(174,182,144) 50% 60%,rgb(160,171,127) 60% 70%,rgb(147,160,113) 70% 80%,rgb(139,152,104) 80% 90%,rgb(134,149,98) 90% 100%),linear-gradient(90deg,rgb(132,148,87) 10%,rgb(140,152,101) 10% 20%,rgb(156,162,126) 20% 30%,rgb(169,171,141) 30% 40%,rgb(171,175,142) 40% 50%,rgb(163,171,131) 50% 60%,rgb(148,160,115) 60% 70%,rgb(134,149,101) 70% 80%,rgb(127,142,93) 80% 90%,rgb(126,140,89) 90% 100%),linear-gradient(90deg,rgb(109,129,62) 10%,rgb(114,130,73) 10% 20%,rgb(126,136,94) 20% 30%,rgb(139,145,109) 30% 40%,rgb(146,150,114) 40% 50%,rgb(141,148,107) 50% 60%,rgb(125,137,93) 60% 70%,rgb(107,123,78) 70% 80%,rgb(102,118,70) 80% 90%,rgb(109,121,71) 90% 100%),linear-gradient(90deg,rgb(103,119,58) 10%,rgb(105,119,67) 10% 20%,rgb(114,123,86) 20% 30%,rgb(129,132,104) 30% 40%,rgb(141,141,113) 40% 50%,rgb(140,140,111) 50% 60%,rgb(123,128,99) 60% 70%,rgb(101,111,83) 70% 80%,rgb(96,105,73) 80% 90%,rgb(108,112,74) 90% 100%),linear-gradient(90deg,rgb(121,127,83) 10%,rgb(124,128,93) 10% 20%,rgb(134,134,116) 20% 30%,rgb(150,146,135) 30% 40%,rgb(162,157,146) 40% 50%,rgb(162,157,144) 50% 60%,rgb(147,144,132) 60% 70%,rgb(126,125,115) 70% 80%,rgb(117,115,100) 80% 90%,rgb(122,119,94) 90% 100%),linear-gradient(90deg,rgb(132,134,100) 10%,rgb(137,137,113) 10% 20%,rgb(150,146,137) 20% 30%,rgb(167,160,159) 30% 40%,rgb(179,172,169) 40% 50%,rgb(178,172,167) 50% 60%,rgb(164,159,154) 60% 70%,rgb(144,140,136) 70% 80%,rgb(130,126,117) 80% 90%,rgb(127,124,104) 90% 100%),linear-gradient(90deg,rgb(123,128,97) 10%,rgb(128,131,109) 10% 20%,rgb(143,141,133) 20% 30%,rgb(160,155,154) 30% 40%,rgb(173,168,165) 40% 50%,rgb(173,169,164) 50% 60%,rgb(160,157,152) 60% 70%,rgb(139,137,133) 70% 80%,rgb(120,120,111) 80% 90%,rgb(111,113,93) 90% 100%),linear-gradient(90deg,rgb(101,112,74) 10%,rgb(105,114,84) 10% 20%,rgb(117,121,105) 20% 30%,rgb(135,134,125) 30% 40%,rgb(149,146,138) 40% 50%,rgb(151,148,139) 50% 60%,rgb(140,138,128) 60% 70%,rgb(118,119,109) 70% 80%,rgb(95,100,85) 80% 90%,rgb(81,90,63) 90% 100%),linear-gradient(90deg,rgb(94,102,40) 10%,rgb(94,102,51) 10% 20%,rgb(100,104,71) 20% 30%,rgb(114,112,91) 30% 40%,rgb(128,122,104) 40% 50%,rgb(133,125,106) 50% 60%,rgb(124,117,97) 60% 70%,rgb(103,100,78) 70% 80%,rgb(82,84,53) 80% 90%,rgb(71,76,28) 90% 100%),linear-gradient(90deg,rgb(109,106,16) 10%,rgb(108,105,34) 10% 20%,rgb(109,103,59) 20% 30%,rgb(117,106,78) 30% 40%,rgb(128,113,88) 40% 50%,rgb(132,115,87) 50% 60%,rgb(124,108,77) 60% 70%,rgb(109,96,59) 70% 80%,rgb(95,86,40) 80% 90%,rgb(90,83,26) 90% 100%)\",\"backgroundPosition\":\"0 0,0 11%,0 22%,0 33%,0 44%,0 56%,0 67%,0 78%,0 89%,0 100%\",\"backgroundRepeat\":\"no-repeat\",\"backgroundSize\":\"100% 10%\",\"filter\":\"blur(24px)\",\"transform\":\"scale(1.2)\"}");
    }
}
