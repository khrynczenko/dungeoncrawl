use super::MapArchitect;
use crate::prelude::*;

pub struct EmptyArchitect {}

impl MapArchitect for EmptyArchitect {
    fn new(&mut self, rng: &mut RandomNumberGenerator) -> MapBuilder {
        let mut mb = MapBuilder {
            map: Map::new(),
            rooms: Vec::new(),
            monster_spawns: Vec::new(),
            starting_player_position: Point::zero(),
            starting_amulet_position: Point::zero(),
        };
        mb.fill(TileType::Floor);
        mb.starting_player_position = Point::new(SCREEN_WIDTH / 2, SCREEN_HEIGHT / 2);
        mb.starting_amulet_position = mb.find_most_distant();
        for _ in 0..50 {
            mb.monster_spawns.push(Point::new(
                rng.range(1, SCREEN_WIDTH),
                rng.range(1, SCREEN_HEIGHT),
            ));
        }
        mb
    }
}
