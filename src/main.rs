use std::cmp::min;
use rand::{rng, Rng};
use std::io;
use std::io::Write;

fn risk_probability(num_attackers: u8, num_defenders: u8, num_games: u32) -> (f32, f32) {
    let mut attack_wins = 0;
    let mut loss = 0;
    for _ in 0..num_games {
        let mut attacker_copy = num_attackers;
        let mut defender_copy = num_defenders;
        while attacker_copy > 0 && defender_copy > 0 {

            let attack_dice = min(attacker_copy, 3);
            let defender_dice = min(defender_copy, 2);


            let mut attack_throws = (0..attack_dice).into_iter()
                .map(|_| {rng().random_range(1..=6)})
                .collect::<Vec<u32>>();
            let mut defender_throws = (0..defender_dice).into_iter()
                .map(|_| {rng().random_range(1..=6)})
                .collect::<Vec<u32>>();

            attack_throws.sort_by(|a, b| b.cmp(a));
            defender_throws.sort_by(|a, b| b.cmp(a));


            for i in 0usize..min(attack_dice, defender_dice) as usize {
                if attack_throws[i] > defender_throws[i] {
                    defender_copy -= 1;
                } else {
                    attacker_copy -= 1;
                }
            }
        }
        if defender_copy == 0 {
            attack_wins += 1;
        }
        loss += (num_attackers - attacker_copy) as u32;
    }
    (attack_wins as f32 / num_games as f32, loss as f32 / num_games as f32)
}

fn main() {
    let mut input = String::new();

    print!("Number of attackers: ");
    io::stdout().flush().expect("failed to flush stdout");
    io::stdin().read_line(&mut input).expect("failed to read line");
    let attacker_n: u8 = input.trim().parse().expect("invalid integer");
    input.clear();

    print!("Number of defenders: ");
    io::stdout().flush().expect("failed to flush stdout");
    io::stdin().read_line(&mut input).expect("failed to read line");
    let defender_n: u8 = input.trim().parse().expect("invalid integer");
    input.clear();

    print!("Number of games to simulate: ");
    io::stdout().flush().expect("failed to flush stdout");
    io::stdin().read_line(&mut input).expect("failed to read line");
    let games_n: u32 = input.trim().parse().expect("invalid integer");

    let (avg_win, avg_losses) = risk_probability(attacker_n, defender_n, games_n);

    println!("\nPercentage of attacker win: {avg_win}");
    println!("Soldiers lost on average: {avg_losses}");
}
