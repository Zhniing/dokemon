use std::collections::HashMap;
use std::process;

#[derive(Debug, PartialEq, Clone)]
enum Player {
    P1,
    P2,
}

#[derive(Debug, PartialEq, Clone)]
struct Hero {
    name: String,
    lv: i32,  // Level
    hp: i32,  // Hit Points
    att: i32, // Attack
    def: i32, // Defense
    spa: i32, // Special Attack
    spd: i32, // Special Defense
    spe: i32, // Speed
    moves: Vec<String>,
    owner: Player,
    id: i32, // Positon
}

impl Hero {
    fn new(
        name: String,
        lv: i32,
        base: &[i32],
        moves: Vec<String>,
        owner: Player,
        id: i32,
    ) -> Hero {
        // Respectively: hp, att, def, spa, spd
        let ivs = [31, 31, 31, 31, 31, 31];
        let evs = [252, 0, 0, 0, 0, 0];
        Hero {
            name,
            lv,
            hp: calc_hp(base[0], ivs[0], evs[0], lv),
            att: calc_stat(base[1], ivs[1], evs[1], lv),
            def: calc_stat(base[2], ivs[2], evs[2], lv),
            spa: calc_stat(base[3], ivs[3], evs[3], lv),
            spd: calc_stat(base[4], ivs[4], evs[4], lv),
            spe: calc_stat(base[5], ivs[5], evs[5], lv),
            moves,
            owner,
            id,
        }
    }
}

/// Calculate the hp stat
fn calc_hp(base: i32, iv: i32, ev: i32, lv: i32) -> i32 {
    (base * 2 + iv + ev / 4) * lv / 100 + 10 + lv
}

/// Calculate any stats aside from hp
fn calc_stat(base: i32, iv: i32, ev: i32, lv: i32) -> i32 {
    (base * 2 + iv + ev / 4) * lv / 100 + 5
}

/// 龙破斩
fn dragon_slave(heroes: &mut Vec<Hero>, agent: usize, target: usize) {
    let power = 90;
    let (att, def) = (heroes[agent].spa, heroes[target].spd);
    heroes[target].hp -= calc_damage(heroes[agent].lv, att, def, power) as i32;
}

/// 远程攻击
fn ranged_attack(heroes: &mut Vec<Hero>, agent: usize, target: usize) {
    let power = 50;
    let (att, def) = (heroes[agent].att, heroes[target].def);
    heroes[target].hp -= calc_damage(heroes[agent].lv, att, def, power) as i32;
}

fn calc_damage(lv: i32, att: i32, def: i32, power: i32) -> f64 {
    (2.0 * lv as f64 + 10.0) / 250.0 * (att as f64 / def as f64) * power as f64 + 2.0
}

fn init_moves() -> HashMap<String, fn(&mut Vec<Hero>, usize, usize)> {
    let mut moves: HashMap<String, fn(&mut Vec<Hero>, usize, usize)> = HashMap::new();

    moves.insert("龙破斩".to_string(), dragon_slave);
    moves.insert("远程攻击".to_string(), ranged_attack);

    moves
}

fn init_test_heroes(lv: i32, base: ([i32; 6], [i32; 6])) -> Vec<Hero> {
    let (p1, p2) = (Player::P1, Player::P2);

    let moves_lina1 = vec!["龙破斩".to_string(), "远程攻击".to_string()];
    let hero1 = Hero::new("火女1".to_string(), lv, &base.0, moves_lina1, p1, 0);
    println!("英雄1: {:?}", hero1);

    let moves_lina2 = vec!["龙破斩".to_string(), "远程攻击".to_string()];
    let hero2 = Hero::new("火女2".to_string(), lv, &base.1, moves_lina2, p2, 1);
    println!("英雄2: {:?}", hero2);

    vec![hero1, hero2]
}

fn print_battle_log(agent: &Hero, target: &Hero, hp_orig: i32, move_sel: &String) {
    print!(
        "【{}】对【{}】使出【{}】",
        agent.name, target.name, move_sel
    );
    print!(", 造成了【{}】点伤害", hp_orig - target.hp);
    println!(",【{}】的体力: {} -> {}", target.name, hp_orig, target.hp);
}

fn select_target(id: i32, heroes: &Vec<Hero>) -> i32 {
    assert!(heroes.len() > 1);
    assert!(id < heroes.len() as i32);
    if heroes.len() == 2 {
        id ^ 1 // 0 or 1
    } else {
        eprintln!("Not implemented yet!");
        process::exit(1);
    }
}

fn select_move(agent: &Hero) -> String {
    let move_index = agent.id as usize;
    agent.moves[move_index].clone() // The move selected to use
}

fn run_battle(mut heroes: Vec<Hero>, moves: &HashMap<String, fn(&mut Vec<Hero>, usize, usize)>) {
    // Start
    let mut round = 0;
    loop {
        round += 1;
        println!("\n---------- 第{}回合 ----------\n", round);

        let mut action_order = vec![0, 1];
        while action_order.len() > 0 {
            action_order.sort_by_key(|id| heroes[*id as usize].spe);

            let agent = action_order.pop().unwrap();
            let move_sel = select_move(&heroes[agent]);
            let target = select_target(heroes[agent].id, &heroes) as usize;
            let hp_orig = heroes[target].hp; // The hp before executing the move

            // TODO: perform moves after select_move() of both players
            moves.get(&move_sel).unwrap()(&mut heroes, agent, target);

            // Battle log
            print_battle_log(&heroes[agent], &heroes[target], hp_orig, &move_sel);

            // Check survival
            if heroes[target].hp <= 0 {
                return;
            }
        }
    }
}

pub fn run() {
    // Init
    let moves = init_moves();
    let lv = 50;
    let base_lina = ([85, 60, 65, 130, 75, 98], [85, 60, 65, 130, 75, 99]);
    let heroes = init_test_heroes(lv, base_lina);

    run_battle(heroes, &moves)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_run() {
        run();
    }

    #[test]
    fn test_select_target() {
        let lv = 50;
        let base = ([50, 50, 50, 50, 50, 50], [50, 50, 50, 50, 50, 49]);
        let heroes = init_test_heroes(lv, base);

        let target = select_target(0, &heroes);
        assert_eq!(target, 1);

        let target = select_target(1, &heroes);
        assert_eq!(target, 0);
    }
}
