use crate::prelude::*;

pub fn render_tile(draw_batch: &mut DrawBatch, map: &Map, position: &Point, offset: &Point, tint: ColorPair) {
    let idx = decode_map_index(position.x, position.y);
    let glyph = match map.tiles.get(idx) {
        Some(&TileType::Wall) => to_cp437('#'),
        Some(&TileType::Floor) => to_cp437('.'),
        _ => return,
    };
    draw_batch.set(*position - *offset, tint, glyph);
}

#[system]
#[read_component(Player)]
#[read_component(FieldOfView)]
pub fn map_render(ecs: &SubWorld<'_>, #[resource] map: &Map, #[resource] camera: &Camera) {
    let mut player_query = <&FieldOfView>::query().filter(component::<Player>());
    let player_fov = player_query.iter(ecs).next().unwrap();

    let mut draw_batch = DrawBatch::new();
    draw_batch.target(0);

    for y in camera.top_y..camera.bottom_y {
        for x in camera.left_x..camera.right_x {
            let pt = Point::new(x, y);
            let offset = Point::new(camera.left_x, camera.top_y);
            if is_in_bounds(Point::new(x, y)) && player_fov.visible_tiles.contains(&pt) {
                render_tile(&mut draw_batch, map, &pt, &offset, ColorPair::new(WHITE, BLACK));
            } else if is_in_bounds(pt) {
                if map.revealed_tiles[decode_map_index(pt.x, pt.y)] == true {
                    render_tile(&mut draw_batch, map, &pt, &offset, ColorPair::new(GRAY, BLACK));
                }
            }
        }
    }

    draw_batch.submit(0).expect("Batch error");
}
