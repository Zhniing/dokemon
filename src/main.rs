use std::collections::HashMap;

#[derive(Debug)]
struct Hero {
    name: String,
    lv: i32,  // Level
    hp: i32,  // Hit Points
    att: i32, // Attack
    def: i32, // Defense
    spa: i32, // Special Attack
    spd: i32, // Special Defense
    moves: Vec<String>,
}

impl Hero {
    fn new(name: String, lv: i32, base: &[i32], moves: Vec<String>) -> Hero {
        // Respectively: hp, att, def, spa, spd
        let ivs = [31, 31, 31, 31, 31];
        let evs = [252, 0, 0, 0, 0];
        Hero {
            name,
            lv,
            hp: calc_hp(base[0], ivs[0], evs[0], lv),
            att: calc_stat(base[1], ivs[1], evs[1], lv),
            def: calc_stat(base[2], ivs[2], evs[2], lv),
            spa: calc_stat(base[3], ivs[3], evs[3], lv),
            spd: calc_stat(base[4], ivs[4], evs[4], lv),
            moves,
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
fn dragon_slave(attacker: &Hero, target: &mut Hero) {
    let power = 90;
    let (att, def) = (attacker.spa, attacker.spd);
    target.hp -= calc_damage(attacker.lv, att, def, power) as i32;
}

/// 远程攻击
fn ranged_attack(attacker: &Hero, target: &mut Hero) {
    let power = 50;
    let (att, def) = (attacker.att, attacker.def);
    target.hp -= calc_damage(attacker.lv, att, def, power) as i32;
}

fn calc_damage(lv: i32, att: i32, def: i32, power: i32) -> f64 {
    (2.0 * lv as f64 + 10.0) / 250.0 * (att as f64 / def as f64) * power as f64 + 2.0
}

fn run_move(
    hero1: &Hero,
    hero2: &mut Hero,
    moves: &HashMap<String, fn(&Hero, &mut Hero)>,
    move_sel: &String,
) {
    match moves.get(move_sel) {
        Some(m) => m(hero1, hero2),
        None => (),
    };
}

fn main() {
    // Init moves
    let mut moves: HashMap<String, fn(&Hero, &mut Hero)> = HashMap::new();
    moves.insert("龙破斩".to_string(), dragon_slave);
    moves.insert("远程攻击".to_string(), ranged_attack);

    // Init heroes
    let lv = 50;
    let base_lina = [85, 60, 65, 130, 75, 98];
    let moves_lina1 = vec!["龙破斩".to_string(), "远程攻击".to_string()];
    let mut hero1 = Hero::new("火女1".to_string(), lv, &base_lina, moves_lina1);
    println!("英雄1: {:?}", hero1);
    let moves_lina2 = vec!["龙破斩".to_string(), "远程攻击".to_string()];
    let mut hero2 = Hero::new("火女2".to_string(), lv, &base_lina, moves_lina2);
    println!("英雄2: {:?}", hero2);

    // Start
    let mut round = 1;
    loop {
        println!("\n---------- 第{}回合 ----------\n", round);

        let move_sel = &hero1.moves[1]; // The move selected to use
        let hp_orig = hero2.hp; // The hp before executing the move
        run_move(&hero1, &mut hero2, &moves, move_sel);

        // Battle log
        print!("【{}】对【{}】使出【{}】", hero1.name, hero2.name, move_sel);
        print!(", 造成了【{}】点伤害", hp_orig - hero2.hp);
        println!(",【{}】的体力: {} -> {}", hero2.name, hp_orig, hero2.hp);

        // Check survival
        if hero2.hp <= 0 {
            break;
        }

        let move_sel = &hero2.moves[0]; // The move selected to use
        let hp_orig = hero1.hp; // The hp before executing the move
        run_move(&hero2, &mut hero1, &moves, move_sel);

        // Battle log
        print!("【{}】对【{}】使出【{}】", hero2.name, hero1.name, move_sel);
        print!(", 造成了【{}】点伤害", hp_orig - hero1.hp);
        println!(",【{}】的体力: {} -> {}", hero1.name, hp_orig, hero1.hp);

        // Check survival
        if hero1.hp <= 0 {
            break;
        }

        round += 1;
    }
}
