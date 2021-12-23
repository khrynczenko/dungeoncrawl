use crate::prelude::*;

#[system]
#[read_component(WantsToAttack)]
#[write_component(Health)]
pub fn combat(ecs: &mut SubWorld<'_>, commands: &mut CommandBuffer) {
    let mut attackers = <(Entity, &WantsToAttack)>::query();
    let victims: Vec<(Entity, Entity)> = attackers
        .iter(ecs)
        .map(|(entity, attack)| (*entity, attack.victim))
        .collect();

    victims.iter().for_each(|(message, victim)| {
        if let Ok(mut health) = ecs
            .entry_mut(*victim)
            .unwrap()
            .get_component_mut::<Health>()
        {
            dbg!("Before {}", health.current);
            health.current -= 1;
            if health.current < 1 {
                commands.remove(*victim);
            } else {
                commands.add_component(*victim, health.current - 1);
            }
            dbg!("After {}", health.current);
            commands.remove(*message);
        }
    });
}
