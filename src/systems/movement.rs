use crate::prelude::*;

#[allow(clippy::trivially_copy_pass_by_ref)]
#[system(for_each)]
#[read_component(Player)]
#[write_component(FieldOfView)]
pub fn movement(
    entity: &Entity,
    want_move: &WantsToMove,
    #[resource] map: &mut Map,
    #[resource] camera: &mut Camera,
    ecs: &mut SubWorld<'_>,
    commands: &mut CommandBuffer,
) {
    if map.can_enter_tile(want_move.destination) {
        let entity_ref =  ecs
            .entry_ref(want_move.entity)
            .unwrap();
        commands.add_component(want_move.entity, want_move.destination);

        if let Ok(fov) = entity_ref.get_component::<FieldOfView>() {
            commands.add_component(want_move.entity, fov.clone_dirty());
            if entity_ref.get_component::<Player>().is_ok() {
                fov.visible_tiles.iter().for_each(|tile_pos| {
                    map.revealed_tiles[decode_map_index(tile_pos.x, tile_pos.y)] = true;
                });
                camera.on_player_move(want_move.destination);
            }
        }

    }

    commands.remove(*entity);
}
