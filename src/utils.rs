pub fn interpolate(k: u32, k_min: u32, v_min: u32, k_max: u32, v_max: u32) -> u32 {
    if k_min == k_max {
        return v_min;
    };

    return (k - k_min) * v_max + (k_max - k) * v_min;
}

#[test]
pub fn interpolate_test() {
    let result = interpolate(1, 1, 4, 10, 12);

    assert_eq!(result, 36);
}

pub fn assign(
    source_data: &Vec<u8>,
    source_width: &u32,
    channel: u32,
    source_x: u32,
    x_min: u32,
    x_max: u32,
    source_y: u32,
    y_min: u32,
    y_max: u32,
) -> u8 {
    let min_index = (y_min * source_width + x_min) * 4 + channel;
    let max_index = (y_min * source_width + x_max) * 4 + channel;

    let v_min_from_src = source_data[min_index as usize] as u32;
    let v_max_from_src = source_data[max_index as usize] as u32;

    let v_min = interpolate(source_x, x_min, v_min_from_src, x_max, v_max_from_src);

    if y_max == y_min {
        return v_min as u8;
    } else {
        let min_index = (y_max * source_width + x_min) * 4 + channel;
        let max_index = (y_max * source_width + x_max) * 4 + channel;

        let v_min_from_src = source_data[min_index as usize] as u32;
        let v_max_from_src = source_data[max_index as usize] as u32;

        let v_max = interpolate(source_x, x_min, v_min_from_src, x_max, v_max_from_src);

        let channel_result = interpolate(source_y, y_min, v_min, y_max, v_max);

        return channel_result as u8;
    }
}

#[test]
pub fn assign_y_eq_test() {
    let vec_8 = vec![0; 640 * 480 * 4];
    let result = assign(&vec_8, &3, 3, 24, 24, 24, 0, 0, 0);

    assert_eq!(result, 0);
}

#[test]
pub fn assign_test() {
    let vec_8 = vec![0; 640 * 480 * 4];
    let result = assign(&vec_8, &3, 3, 24, 24, 24, 0, 0, 1);

    assert_eq!(result, 0);
}

