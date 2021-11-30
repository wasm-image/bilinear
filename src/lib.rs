use wasm_bindgen::{prelude::*, Clamped};
use web_sys;

mod utils;

#[wasm_bindgen]
pub fn bilinear(
    image_data: web_sys::ImageData,
    target_width: u32,
    target_height: u32,
) -> Result<web_sys::ImageData, JsValue> {
    let source_data = image_data.data();
    let source_width = image_data.width();
    let source_height = image_data.height();

    // TODO: implement image positioning
    let source_x = 0;
    let source_y = 0;
    let target_x = 0;
    let target_y = 0;

    let is_source_valid = source_width > 0 || source_height > 0;
    let is_target_valid = target_width > 0 || target_height > 0;

    if is_source_valid && is_target_valid {
        let x_ratio = source_width / target_width;
        let y_ratio = source_height / target_height;
        let mut y = 0;
        let vec_len = target_width * target_height * 4;
        let mut resized_pixel_data: Vec<u8> = vec![0; (vec_len) as usize];

        while y < target_height {
            let dest_y = target_y + y;
            let source_y = y * y_ratio + source_y;
            let y_min = source_y;
            let y_max = std::cmp::min(source_y, source_height - 1);

            let should_continue_y = dest_y >= target_height;

            if !should_continue_y {
                let mut x = 0;

                while x < target_width {
                    let dest_x = target_x + x;

                    let source_x = x * x_ratio + source_x;
                    let x_min = source_x;
                    let x_max = std::cmp::min(source_x, source_width - 1);

                    let dest_index = (dest_y * target_width + dest_x) * 4;

                    resized_pixel_data[(dest_index) as usize] = utils::assign(
                        &source_data,
                        &source_width,
                        0,
                        source_x,
                        x_min,
                        x_max,
                        source_y,
                        y_min,
                        y_max,
                    );

                    resized_pixel_data[(dest_index + 1) as usize] = utils::assign(
                        &source_data,
                        &source_width,
                        1,
                        source_x,
                        x_min,
                        x_max,
                        source_y,
                        y_min,
                        y_max,
                    );

                    resized_pixel_data[(dest_index + 2) as usize] = utils::assign(
                        &source_data,
                        &source_width,
                        2,
                        source_x,
                        x_min,
                        x_max,
                        source_y,
                        y_min,
                        y_max,
                    );

                    resized_pixel_data[(dest_index + 3) as usize] = utils::assign(
                        &source_data,
                        &source_width,
                        3,
                        source_x,
                        x_min,
                        x_max,
                        source_y,
                        y_min,
                        y_max,
                    );

                    x += 1;
                }
            }
            y += 1
        }

        let resized_image = web_sys::ImageData::new_with_u8_clamped_array_and_sh(
            Clamped(&resized_pixel_data),
            target_width,
            target_height,
        )
        .unwrap();

        Ok(resized_image)
    } else {
        Err(JsValue::from("width and height should be greater than 0"))
    }
}
