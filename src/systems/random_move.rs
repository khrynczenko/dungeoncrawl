use crate::prelude::*;

#[system]
#[write_component(Point)]
#[read_component(MovingRandomly)]
pub fn random_move(ecs: &mut SubWorld<'_>, commands: &mut CommandBuffer) {
    let mut positions = <(Entity, &mut Point)>::query().filter(component::<MovingRandomly>());
    positions.iter_mut(ecs).for_each(|(&entity, pos)| {
        let mut rng = RandomNumberGenerator::new();
        let delta = match rng.range(0, 4) {
            0 => Point::new(-1, 0),
            1 => Point::new(1, 0),
            2 => Point::new(0, -1),
            _ => Point::new(0, 1),
        };

        let destination = *pos + delta;

        commands.push((
            (),
            WantsToMove {
                entity,
                destination,
            },
        ));
    });
}
