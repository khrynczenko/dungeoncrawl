use crate::prelude::*;

const TILES_COUNT: usize = (SCREEN_WIDTH * SCREEN_HEIGHT) as usize;

#[derive(Copy, Clone, PartialEq)]
pub enum TileType {
    Wall,
    Floor,
}

pub struct Map {
    pub tiles: Vec<TileType>,
}

impl Map {
    pub fn new() -> Self {
        Self {
            tiles: vec![TileType::Floor; TILES_COUNT],
        }
    }

    pub fn render(&self, ctx: &mut BTerm, camera: &Camera) {
        ctx.set_active_console(0);
        for y in camera.top_y..camera.bottom_y {
            for x in camera.left_x..camera.right_x {
                if is_in_bounds(Point::new(x, y)) {
                    match self.tiles.get(decode_map_index(x, y)) {
                        Some(TileType::Wall) => ctx.set(
                            x - camera.left_x,
                            y - camera.top_y,
                            WHITE,
                            BLACK,
                            to_cp437('#'),
                        ),
                        Some(TileType::Floor) => ctx.set(
                            x - camera.left_x,
                            y - camera.top_y,
                            WHITE,
                            BLACK,
                            to_cp437('.'),
                        ),
                        None => (),
                    }
                }
            }
        }
    }

    pub fn can_enter_tile(&self, tile_xy: Point) -> bool {
        is_in_bounds(tile_xy)
            && self.tiles[decode_map_index(tile_xy.x, tile_xy.y)] == TileType::Floor
    }

    #[allow(clippy::unused_self)]
    pub fn try_idx(&self, point: Point) -> Option<usize> {
        if !is_in_bounds(point) {
            return None;
        }

        Some(decode_map_index(point.x, point.y))
    }
}

#[allow(clippy::cast_sign_loss)]
pub fn decode_map_index(x: i32, y: i32) -> usize {
    (x + y * SCREEN_WIDTH as i32) as usize
}

pub fn is_in_bounds(point: Point) -> bool {
    point.x >= 0 && point.y >= 0 && point.x < SCREEN_WIDTH && point.y < SCREEN_HEIGHT
}

#[cfg(test)]
mod tests {
    use super::{Map, Point, SCREEN_HEIGHT, SCREEN_WIDTH};

    #[test]
    fn decoding_map_index() {
        let x = 2;
        let y = 1;
        assert_eq!(super::decode_map_index(x, y), SCREEN_WIDTH as usize + 2);
    }

    #[test]
    fn checking_is_in_bounds() {
        assert!(!super::is_in_bounds(Point::new(SCREEN_WIDTH, 0)));
        assert!(!super::is_in_bounds(Point::new(0, SCREEN_HEIGHT)));
        assert!(super::is_in_bounds(Point::new(0, 0)));
        assert!(super::is_in_bounds(Point::new(1, 0)));
        assert!(super::is_in_bounds(Point::new(0, 1)));
    }

    #[test]
    fn can_enter_tile() {
        let map = Map::new();
        assert!(map.can_enter_tile(Point::new(1, 1)));
        assert!(!map.can_enter_tile(Point::new(-1, 1)));
        assert!(!map.can_enter_tile(Point::new(0, -1)));
        assert!(!map.can_enter_tile(Point::new(SCREEN_WIDTH, 0)));
        assert!(!map.can_enter_tile(Point::new(0, SCREEN_HEIGHT)));
    }
}
