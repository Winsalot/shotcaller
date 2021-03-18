use crate::*;

/// Spawn an elephant companion at a specific location
pub fn elephant_spawner_system(
    events: &Vec<SkillTriggerEvent<Skills>>,
    stat_def: &StatDefinitions<Stats>,
    game_events: &mut Vec<GameEvent>,
    teams: &mut Components<Team>,
    proximity_attacks: &mut Components<ProximityAttack>,
    simple_movements: &mut Components<SimpleMovement>,
    stats: &mut Components<StatSet<Stats>>,
    positions: &mut Components<Point>,
    entities: &mut Entities,
    companions: &mut Components<Companion>,
    sprites: &mut Components<Sprite>,
    sprite_indices: &mut Components<SpriteIndex>,
    summoned: &mut Components<Summon>,
    summoner: &mut Components<Summoner>,
) -> SystemResult {
    for ev in events.iter() {
        if ev.1 == Skills::ElephantSummon {
            let pos = positions.get(ev.0).unwrap().clone();
            let team = teams.get_mut(ev.0).unwrap().clone();
            let owner = ev.0.clone();

            let elephant = entities.create();
            positions.insert(elephant, pos.clone());
            summoned.insert(elephant, Summon{owner});
            companions.insert(ev.0, Companion::Elephant(elephant));
            simple_movements.insert(elephant, SimpleMovement);
            teams.insert(elephant, team);
            stats.insert(elephant, stat_def.to_statset());
            proximity_attacks.insert(elephant, ProximityAttack::new(CREEP_ATTACK_RADIUS));
            let bg = if team == Team::Me {
                RGBA::named(GREEN)
            } else {
                RGBA::named(RED)
            };
            sprites.insert(
                elephant,
                Sprite {
                    glyph: to_cp437('c'),
                    fg: RGBA::named(YELLOW),
                    bg,
                },
            );
            sprite_indices.insert(elephant, SpriteIndex(15));

            // Owner also gets a new component:
            if let Some(summoner_comp) = summoner.get_mut(ev.0){
                summoner_comp.summoned.push(elephant.clone());
            } 
            else {
                let summoner_comp = Summoner{summoned: vec![elephant.clone()]};
                summoner.insert(ev.0, summoner_comp);
            }
            game_events.push(GameEvent::EntitySummoned{owner: ev.0.clone(), summon: elephant.clone()});

        }
    }
    Ok(())
}
