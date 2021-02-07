extern crate blurhash;
extern crate image;

use blurhash::decode;
use image::{Rgb, RgbImage};
use neon::prelude::*;
use serde_derive::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
struct BlurhashCss {
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

fn bytes_to_image_buffer(pixel_bytes: &[u8], width: u32, height: u32) -> RgbImage {
    let mut image_buffer = RgbImage::new(width, height);
    let number_of_channels = 4; // RGBa
    let bytes_per_row = width * number_of_channels;
    let r_index = 0;
    let g_index = 1;
    let b_index = 2;
    for y in 0..height {
        for x in 0..width {
            let r = pixel_bytes[(number_of_channels * x + r_index + y * bytes_per_row) as usize];
            let g = pixel_bytes[(number_of_channels * x + g_index + y * bytes_per_row) as usize];
            let b = pixel_bytes[(number_of_channels * x + b_index + y * bytes_per_row) as usize];
            image_buffer.put_pixel(x, y, Rgb([r, g, b]));
        }
    }
    image_buffer
}

fn blurhash_to_image_buffer(hash: String, width: u32, height: u32) -> RgbImage {
    let pixel_bytes = blurhash_to_bytes(hash, width, height);
    bytes_to_image_buffer(&pixel_bytes, width, height)
}

fn resize_image_buffer(image_buffer: RgbImage, new_width: u32, new_height: u32) -> RgbImage {
    image::imageops::resize(
        &image_buffer,
        new_width,
        new_height,
        image::imageops::FilterType::Nearest,
    )
}

fn blurhash_to_resized_image_buffer(
    hash: String,
    width: u32,
    height: u32,
    new_width: u32,
    new_height: u32,
) -> RgbImage {
    let image_buffer = blurhash_to_image_buffer(hash, width, height);
    resize_image_buffer(image_buffer, new_width, new_height)
}

fn blurhash_to_css_struct(hash: String, width: u32, height: u32) -> BlurhashCss {
    let new_width = 10;
    let new_height = 10;
    let resized_image_buffer =
        blurhash_to_resized_image_buffer(hash, width, height, new_width, new_height);
    let r_index = 0;
    let g_index = 1;
    let b_index = 2;
    let background_size = format!("100% {}%", 100.0 / new_height as f32);
    let mut background_position = Vec::new();
    let mut linear_gradients = Vec::new();

    for y in 0..new_height {
        let mut row_linear_gradients = Vec::new();

        for x in 0..new_width {
            let r = resized_image_buffer.get_pixel(x, y).data[r_index];
            let g = resized_image_buffer.get_pixel(x, y).data[g_index];
            let b = resized_image_buffer.get_pixel(x, y).data[b_index];

            let start_percent = if x == 0 {
                String::from("")
            } else {
                format!(" {}%", get_rounded_percentage_of(x, new_width))
            };

            let end_percent = if x == new_width {
                String::from("")
            } else {
                format!(" {}%", get_rounded_percentage_of(x + 1, new_width))
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
            background_position.push(format!(
                "0 {}%",
                get_rounded_percentage_of(y, new_height - 1)
            ));
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

fn blurhash_to_css(mut cx: FunctionContext) -> JsResult<JsObject> {
    let hash = cx.argument::<JsString>(0)?.value();
    let width = cx.argument::<JsNumber>(1)?.value();
    let height = cx.argument::<JsNumber>(2)?.value();
    let css_struct = blurhash_to_css_struct(hash, width as u32, height as u32);

    let object = JsObject::new(&mut cx);
    let background_image = cx.string(&css_struct.background_image);
    let background_position = cx.string(&css_struct.background_position);
    let background_repeat = cx.string(&css_struct.background_repeat);
    let background_size = cx.string(&css_struct.background_size);
    let filter = cx.string(&css_struct.filter);
    let transform = cx.string(&css_struct.transform);

    object
        .set(&mut cx, "backgroundImage", background_image)
        .unwrap();
    object
        .set(&mut cx, "backgroundPosition", background_position)
        .unwrap();
    object
        .set(&mut cx, "backgroundRepeat", background_repeat)
        .unwrap();
    object
        .set(&mut cx, "backgroundSize", background_size)
        .unwrap();
    object.set(&mut cx, "filter", filter).unwrap();
    object.set(&mut cx, "transform", transform).unwrap();

    Ok(object)
}

register_module!(mut cx, {
    cx.export_function("blurhashToCss", blurhash_to_css)
});

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
    fn creates_image_buffers() {
        let hash = String::from("eCF6B#-:0JInxr?@s;nmIoWUIko1%NocRk.8xbIUaxR*^+s;RiWAWU");
        let width = 400;
        let height = 600;
        let pixel_bytes = blurhash_to_bytes(hash, width, height);
        let image_buffer = bytes_to_image_buffer(&pixel_bytes, width, height);

        assert_eq!(image_buffer.dimensions(), (400, 600));
    }

    #[test]
    fn resizes_image_buffers() {
        let hash = String::from("eCF6B#-:0JInxr?@s;nmIoWUIko1%NocRk.8xbIUaxR*^+s;RiWAWU");
        let width = 400;
        let height = 600;
        let pixel_bytes = blurhash_to_bytes(hash, width, height);
        let image_buffer = bytes_to_image_buffer(&pixel_bytes, width, height);
        let new_width = 200;
        let new_height = 300;
        let resized_image_buffer = resize_image_buffer(image_buffer, new_width, new_height);

        assert_eq!(resized_image_buffer.dimensions(), (new_width, new_height));
    }

    #[test]
    fn decodes_blurhashes_into_a_resized_image_buffer() {
        let hash = String::from("eCF6B#-:0JInxr?@s;nmIoWUIko1%NocRk.8xbIUaxR*^+s;RiWAWU");
        let width = 400;
        let height = 600;
        let new_width = 10;
        let new_height = 10;
        let resized_image_buffer =
            blurhash_to_resized_image_buffer(hash, width, height, new_width, new_height);

        assert_eq!(resized_image_buffer.dimensions(), (10, 10));
    }

    #[test]
    fn converts_blurhashes_into_css_structs() {
        let hash = String::from("eCF6B#-:0JInxr?@s;nmIoWUIko1%NocRk.8xbIUaxR*^+s;RiWAWU");
        let width = 400;
        let height = 600;
        let css_struct = blurhash_to_css_struct(hash, width, height);

        assert_eq!(
            css_struct.background_position,
            "0 0,0 11%,0 22%,0 33%,0 44%,0 56%,0 67%,0 78%,0 89%,0 100%"
        );
        assert_eq!(css_struct.background_repeat, "no-repeat");
        assert_eq!(css_struct.background_size, "100% 10%");
        assert_eq!(css_struct.filter, "blur(24px)");
        assert_eq!(css_struct.background_image, "linear-gradient(90deg,rgb(143,156,100) 10%,rgb(158,164,124) 10% 20%,rgb(174,176,146) 20% 30%,rgb(181,183,155) 30% 40%,rgb(177,182,148) 40% 50%,rgb(164,174,132) 50% 60%,rgb(150,163,116) 60% 70%,rgb(139,153,105) 70% 80%,rgb(133,148,98) 80% 90%,rgb(131,146,94) 90% 100%),linear-gradient(90deg,rgb(122,139,78) 10%,rgb(134,145,99) 10% 20%,rgb(148,154,119) 20% 30%,rgb(158,161,128) 30% 40%,rgb(156,162,124) 40% 50%,rgb(145,155,111) 50% 60%,rgb(128,142,95) 60% 70%,rgb(116,132,84) 70% 80%,rgb(115,130,80) 80% 90%,rgb(119,132,80) 90% 100%),linear-gradient(90deg,rgb(103,121,57) 10%,rgb(109,123,74) 10% 20%,rgb(122,130,93) 20% 30%,rgb(135,139,105) 30% 40%,rgb(140,143,107) 40% 50%,rgb(130,136,98) 50% 60%,rgb(109,121,82) 60% 70%,rgb(94,109,69) 70% 80%,rgb(99,110,67) 80% 90%,rgb(111,117,71) 90% 100%),linear-gradient(90deg,rgb(111,121,72) 10%,rgb(116,123,89) 10% 20%,rgb(129,131,109) 20% 30%,rgb(145,143,124) 30% 40%,rgb(152,149,129) 40% 50%,rgb(143,142,122) 50% 60%,rgb(123,125,107) 60% 70%,rgb(106,110,91) 70% 80%,rgb(109,111,84) 80% 90%,rgb(119,118,84) 90% 100%),linear-gradient(90deg,rgb(130,132,97) 10%,rgb(138,136,117) 10% 20%,rgb(153,147,141) 20% 30%,rgb(168,161,157) 30% 40%,rgb(175,168,161) 40% 50%,rgb(167,161,153) 50% 60%,rgb(148,144,137) 60% 70%,rgb(130,126,119) 70% 80%,rgb(125,122,105) 80% 90%,rgb(129,125,99) 90% 100%),linear-gradient(90deg,rgb(132,134,105) 10%,rgb(142,140,126) 10% 20%,rgb(158,153,150) 20% 30%,rgb(174,167,167) 30% 40%,rgb(180,174,171) 40% 50%,rgb(173,169,164) 50% 60%,rgb(155,151,147) 60% 70%,rgb(135,132,127) 70% 80%,rgb(123,122,108) 80% 90%,rgb(121,121,97) 90% 100%),linear-gradient(90deg,rgb(113,121,90) 10%,rgb(123,127,109) 10% 20%,rgb(140,139,132) 20% 30%,rgb(156,153,149) 30% 40%,rgb(164,161,155) 40% 50%,rgb(159,156,149) 50% 60%,rgb(141,140,133) 60% 70%,rgb(118,119,111) 70% 80%,rgb(101,105,89) 80% 90%,rgb(94,101,74) 90% 100%),linear-gradient(90deg,rgb(94,106,60) 10%,rgb(100,108,77) 10% 20%,rgb(113,116,98) 20% 30%,rgb(130,128,115) 30% 40%,rgb(140,136,123) 40% 50%,rgb(137,133,119) 50% 60%,rgb(120,118,104) 60% 70%,rgb(96,98,81) 70% 80%,rgb(76,84,56) 80% 90%,rgb(68,78,36) 90% 100%),linear-gradient(90deg,rgb(100,103,29) 10%,rgb(100,101,50) 10% 20%,rgb(107,104,72) 20% 30%,rgb(120,112,88) 30% 40%,rgb(130,118,95) 40% 50%,rgb(128,116,91) 50% 60%,rgb(114,104,76) 60% 70%,rgb(95,89,55) 70% 80%,rgb(82,79,31) 80% 90%,rgb(79,77,12) 90% 100%),linear-gradient(90deg,rgb(115,109,20) 10%,rgb(114,106,48) 10% 20%,rgb(117,106,71) 20% 30%,rgb(126,110,84) 30% 40%,rgb(133,114,87) 40% 50%,rgb(132,112,80) 50% 60%,rgb(121,102,65) 60% 70%,rgb(107,92,49) 70% 80%,rgb(99,88,36) 80% 90%,rgb(99,88,31) 90% 100%)");
        assert_eq!(css_struct.transform, "scale(1.2)");
    }
}
