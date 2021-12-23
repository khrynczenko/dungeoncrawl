use crate::prelude::*;

pub fn spawn_player(ecs: &mut World, pos: Point) {
    ecs.push((
        Player,
        Health {
            current: 20,
            max: 20,
        },
        pos,
        Render {
            color: ColorPair::new(WHITE, BLACK),
            glyph: to_cp437('@'),
        },
    ));
}

pub fn spawn_monster(ecs: &mut World, rng: &mut RandomNumberGenerator, pos: Point) {
    let (hp, name, glyph) = match rng.roll_dice(1, 10) {
        1..=8 => make_goblin(),
        _ => make_orc(),
    };

    ecs.push((
        Enemy,
        MovingRandomly,
        pos,
        Health {
            current: hp,
            max: hp,
        },
        Name(name),
        Render {
            color: ColorPair::new(WHITE, BLACK),
            glyph,
        },
    ));
}

fn make_goblin() -> (i32, String, FontCharType) {
    (1, String::from("Goblin"), to_cp437('g'))
}

fn make_orc() -> (i32, String, FontCharType) {
    (2, String::from("Orc"), to_cp437('o'))
}
