use crate::prelude::*;

#[system]
#[read_component(Point)]
#[read_component(ChasingPlayer)]
#[read_component(Health)]
#[read_component(Player)]
#[read_component(FieldOfView)]
pub fn chasing(ecs: &SubWorld<'_>, commands: &mut CommandBuffer, #[resource] map: &Map) {
    let mut movers = <(Entity, &Point, &ChasingPlayer, &FieldOfView)>::query();
    let mut collidable_positions = <(Entity, &Point, &Health)>::query();
    let mut player = <(&Point, &Player)>::query();

    let player_pos = player.iter(ecs).find_map(|(pos, _)| Some(*pos)).unwrap();
    let player_idx = decode_map_index(player_pos.x, player_pos.y);

    let search_targets = vec![player_idx];
    let dijkstra_map = DijkstraMap::new(SCREEN_WIDTH, SCREEN_HEIGHT, &search_targets, map, 1024.0);

    movers
        .iter(ecs)
        .for_each(|(monster_entity, monster_pos, _, fov)| {
            if !fov.visible_tiles.contains(&player_pos) {
                return;
            }

            let monster_idx = decode_map_index(monster_pos.x, monster_pos.y);
            if let Some(destination) =
                DijkstraMap::find_lowest_exit(&dijkstra_map, monster_idx, map)
            {
                let distance = DistanceAlg::Pythagoras.distance2d(*monster_pos, player_pos);
                let destination = if distance > 1.2 {
                    // 1.2 just to be safe in case of float imprecision
                    map.index_to_point2d(destination)
                } else {
                    player_pos
                };
                let mut attacked = false;
                collidable_positions
                    .iter(ecs)
                    .filter(|(_, target_pos, _)| **target_pos == destination)
                    .for_each(|(victim, _, _)| {
                        if ecs
                            .entry_ref(*victim)
                            .unwrap()
                            .get_component::<Player>()
                            .is_ok()
                        {
                            commands.push((
                                (),
                                WantsToAttack {
                                    attacker: *monster_entity,
                                    victim: *victim,
                                },
                            ));
                        }
                        attacked = true;
                    });

                if !attacked {
                    commands.push((
                        (),
                        WantsToMove {
                            entity: *monster_entity,
                            destination,
                        },
                    ));
                }
            }
        });
}
