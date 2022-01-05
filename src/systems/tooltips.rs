use crate::prelude::*;

#[allow(clippy::trivially_copy_pass_by_ref)]
#[system]
#[read_component(Health)]
#[read_component(Point)]
#[read_component(Name)]
#[read_component(Player)]
#[read_component(FieldOfView)]
pub fn tooltips(ecs: &SubWorld<'_>, #[resource] mouse_pos: &Point, #[resource] camera: &Camera) {
    let mut player_fov_query = <&FieldOfView>::query().filter(component::<Player>());
    let player_fov = player_fov_query.iter(ecs).next().unwrap();

    let mut query = <(&Health, &Name, &Point)>::query();
    let offset = Point::new(camera.left_x, camera.top_y);
    let map_pos = *mouse_pos + offset;
    let mut draw_batch = DrawBatch::new();
    draw_batch.target(2);
    query
        .iter(ecs)
        .filter(|(_, _, pos)| **pos == map_pos)
        .filter(|(_, _, pos)| player_fov.visible_tiles.contains(*pos))
        .for_each(|(hp, name, _)| {
            let screen_pos = *mouse_pos * 4;
            let display = format!("{} : {} hp", &name.0, hp.current);
            let offset = display.find(':').unwrap();
            draw_batch.print(screen_pos - Point::new(offset - 2, 0), &display);
        });
    draw_batch.submit(10100).expect("Batch error");
}
