use crate::prelude::*;

#[allow(clippy::trivially_copy_pass_by_ref)]
#[system]
#[write_component(Health)]
#[read_component(Point)]
#[read_component(Player)]
#[read_component(Enemy)]
pub fn player_input(
    ecs: &mut SubWorld<'_>,
    #[resource] key: &Option<VirtualKeyCode>,
    #[resource] turn_state: &mut TurnState,
    commands: &mut CommandBuffer,
) {
    if let Some(key) = key {
        let delta = match key {
            VirtualKeyCode::Left => Point::new(-1, 0),
            VirtualKeyCode::Right => Point::new(1, 0),
            VirtualKeyCode::Up => Point::new(0, -1),
            VirtualKeyCode::Down => Point::new(0, 1),
            _ => Point::new(0, 0),
        };

        let mut players = <(Entity, &Point)>::query().filter(component::<Player>());
        let (player_entity, destination) = players
            .iter(ecs)
            .find_map(|(entity, pos)| Some((entity, *pos + delta)))
            .unwrap();
        let mut enemies = <(Entity, &Point)>::query().filter(component::<Enemy>());

        let mut did_something = false;
        if delta.x != 0 || delta.y != 0 {
            let mut hit_something = false;
            enemies
                .iter(ecs)
                .filter(|(_, pos)| **pos == destination)
                .for_each(|(entity, _)| {
                    hit_something = true;
                    did_something = true;
                    commands.push((
                        (),
                        WantsToAttack {
                            attacker: *player_entity,
                            victim: *entity,
                        },
                    ));
                });

            if !hit_something {
                did_something = true;
                commands.push((
                    (),
                    WantsToMove {
                        entity: *player_entity,
                        destination,
                    },
                ));
            }
        }
        if !did_something {
            if let Ok(health) = ecs
                .entry_ref(*player_entity)
                .unwrap()
                .get_component::<Health>()
            {
                let mut health = *health;
                health.current = i32::min(health.current + 1, health.max);
                dbg!(health);
                commands.add_component(*player_entity, health);
            }
        }
        *turn_state = TurnState::PlayerTurn;
    }
}
