use crate::prelude::*;

#[system]
#[read_component(Health)]
#[read_component(Point)]
#[read_component(Name)]
pub fn tooltips(ecs: &SubWorld<'_>, #[resource] mouse_pos: &Point, #[resource] camera: &Camera) {
    let mut query = <(&Health, &Name, &Point)>::query();
    let offset = Point::new(camera.left_x, camera.top_y);
    let map_pos = *mouse_pos + offset;
    let mut draw_batch = DrawBatch::new();
    draw_batch.target(2);
    query
        .iter(ecs)
        .filter(|(_, _, pos)| **pos == map_pos)
        .for_each(|(hp, name, _)| {
            let screen_pos = *mouse_pos * 4;
            let display = format!("{} : {} hp", &name.0, hp.current);
            let offset = display.find(':').unwrap();
            draw_batch.print(screen_pos - Point::new(offset - 2, 0), &display);
        });
    draw_batch.submit(10100).expect("Batch error");
}
