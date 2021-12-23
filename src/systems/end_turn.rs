use crate::prelude::*;

#[system]
#[read_component(Health)]
#[read_component(Player)]
#[read_component(AmuletOfYala)]
#[read_component(Point)]
pub fn end_turn(ecs: &SubWorld<'_>, #[resource] turn_state: &mut TurnState) {
    let mut player_hp = <&Health>::query().filter(component::<Player>());

    *turn_state = match turn_state {
        TurnState::PlayerTurn => TurnState::MonsterTurn,
        TurnState::MonsterTurn => TurnState::AwaitingInput,
        TurnState::AwaitingInput => TurnState::PlayerTurn,
        _ => *turn_state,
    };

    player_hp.iter(ecs).for_each(|hp| {
        if hp.current < 1 {
            *turn_state = TurnState::GameOver;
        }
    });

    let mut amulet_query = <&Point>::query().filter(component::<AmuletOfYala>());
    let amulet_pos = amulet_query.iter(ecs).next().unwrap();

    let mut player_query = <&Point>::query().filter(component::<Player>());
    let player_pos = player_query.iter(ecs).next().unwrap();

    if amulet_pos == player_pos {
        *turn_state = TurnState::Victory;
    }
}
