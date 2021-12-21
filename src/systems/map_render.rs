use crate::prelude::*;

#[system]
pub fn map_render(#[resource] map: &Map, #[resource] camera: &Camera) {
    let mut draw_batch = DrawBatch::new();
    draw_batch.target(0);
    for y in camera.top_y..camera.bottom_y {
        for x in camera.left_x..camera.right_x {
            let pt = Point::new(x, y);
            let offset = Point::new(camera.left_x, camera.top_y);
            if is_in_bounds(Point::new(x, y)) {
                let idx = decode_map_index(x, y);
                let glyph = match map.tiles.get(idx) {
                    Some(&TileType::Wall) => to_cp437('#'),
                    Some(&TileType::Floor) => to_cp437('.'),
                    _ => break,
                };
                draw_batch.set(pt - offset, ColorPair::new(WHITE, BLACK), glyph);
            }
        }
    }

    draw_batch.submit(0).expect("Batch error");
}
