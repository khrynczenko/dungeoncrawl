use crate::prelude::*;

const TILES_COUNT: usize = (SCREEN_WIDTH * SCREEN_HEIGHT) as usize;

#[derive(Copy, Clone, PartialEq)]
pub enum TileType {
    Wall,
    Floor,
}

pub struct Map {
    pub tiles: Vec<TileType>,
    pub revealed_tiles: Vec<bool>,
}

impl Map {
    pub fn new() -> Self {
        Self {
            tiles: vec![TileType::Floor; TILES_COUNT],
            revealed_tiles: vec![false; TILES_COUNT],
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

    fn is_valid_exit(&self, loc: Point, delta: Point) -> Option<usize> {
        let destination = loc + delta;
        if self.in_bounds(destination) {
            if self.can_enter_tile(destination) {
                let idx = self.point2d_to_index(destination);
                Some(idx)
            } else {
                None
            }
        } else {
            None
        }
    }
}

impl BaseMap for Map {
    fn get_available_exits(&self, idx: usize) -> SmallVec<[(usize, f32); 10]> {
        let loc = self.index_to_point2d(idx);
        let mut exits = SmallVec::new();
        if let Some(idx) = self.is_valid_exit(loc, Point::new(-1, 0)) {
            exits.push((idx, 1.0));
        }

        if let Some(idx) = self.is_valid_exit(loc, Point::new(1, 0)) {
            exits.push((idx, 1.0));
        }

        if let Some(idx) = self.is_valid_exit(loc, Point::new(0, -1)) {
            exits.push((idx, 1.0));
        }

        if let Some(idx) = self.is_valid_exit(loc, Point::new(0, 1)) {
            exits.push((idx, 1.0));
        }

        exits
    }

    fn get_pathing_distance(&self, idx1: usize, idx2: usize) -> f32 {
        DistanceAlg::Pythagoras.distance2d(self.index_to_point2d(idx1), self.index_to_point2d(idx2))
    }

    fn is_opaque(&self, idx: usize) -> bool {
        match self.tiles[idx] {
            TileType::Wall => true,
            TileType::Floor => false,
        }
    }
}

impl Algorithm2D for Map {
    fn dimensions(&self) -> Point {
        Point::new(SCREEN_WIDTH, SCREEN_HEIGHT)
    }

    fn in_bounds(&self, point: Point) -> bool {
        is_in_bounds(point)
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
