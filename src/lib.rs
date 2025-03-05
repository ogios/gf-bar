pub mod text;

#[allow(clippy::too_many_arguments)]
pub fn copy_pixmap(
    src_data: &[u8],
    src_width: usize,
    src_height: usize,
    dst_data: &mut [u8],
    dst_width: usize,
    dst_height: usize,
    x: isize,
    y: isize,
) {
    let (sx_start, dx_start, copy_width) = {
        let sx_start = (-x).max(0) as usize;
        let dx_start = x.max(0) as usize;

        let remaining_width_src = src_width.saturating_sub(sx_start) as isize;
        let remaining_width_dst = dst_width.saturating_sub(dx_start) as isize;

        let copy_width = remaining_width_src.min(remaining_width_dst).max(0) as usize;
        (sx_start, dx_start, copy_width)
    };

    let (sy_start, dy_start, copy_height) = {
        let sy_start = (-y).max(0) as usize;
        let dy_start = y.max(0) as usize;

        let remaining_height_src = src_height.saturating_sub(sy_start) as isize;
        let remaining_height_dst = dst_height.saturating_sub(dy_start) as isize;

        let copy_height = remaining_height_src.min(remaining_height_dst).max(0) as usize;
        (sy_start, dy_start, copy_height)
    };

    if copy_width == 0 || copy_height == 0 {
        return;
    }

    for row in 0..copy_height {
        let src_row = sy_start + row;
        let dst_row = dy_start + row;

        let src_start = (src_row * src_width + sx_start) * 4;
        let src_end = src_start + copy_width * 4;
        let dst_start = (dst_row * dst_width + dx_start) * 4;
        let dst_end = dst_start + copy_width * 4;

        dst_data[dst_start..dst_end].copy_from_slice(&src_data[src_start..src_end]);
    }
}
