use crate::prelude::*;

#[system]
#[read_component(WantsToAttack)]
#[read_component(Player)]
#[write_component(Health)]
pub fn combat(ecs: &mut SubWorld<'_>, commands: &mut CommandBuffer) {
    let mut attackers = <(Entity, &WantsToAttack)>::query();
    let victims: Vec<(Entity, Entity)> = attackers
        .iter(ecs)
        .map(|(entity, attack)| (*entity, attack.victim))
        .collect();

    let player = *<(Entity, &Player)>::query().iter(ecs).next().unwrap().0;

    for (message, victim) in &victims {
        if let Ok(mut health) = ecs
            .entry_mut(*victim)
            .unwrap()
            .get_component_mut::<Health>()
        {
            health.current -= 1;
            if health.current < 1 && *victim != player {
                commands.remove(*victim);
            } else {
                commands.add_component(*victim, health.current - 1);
            }
            commands.remove(*message);
        }
    }
}
