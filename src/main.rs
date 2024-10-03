use std::fs::File;
use std::io::{BufRead, BufReader};
use std::collections::HashMap;

fn generate_deck_from_file() -> Option<HashMap<String, usize>> {
    let file = File::open("deck.txt").expect("Error: Could not open file deck.txt");
    let reader = BufReader::new(file);

    let mut deck: HashMap<String, usize> = HashMap::new();
    let mut total_count = 0;

    for line in reader.lines() {
        let line = line.expect("Error: Could not read line from file");

        let parts: Vec<&str> = line.splitn(2, ' ').collect();

        if parts.len() != 2 {
            println!("Error: Invalid input format in file.");
            return None;
        }

        let count: usize = match parts[0].parse() {
            Ok(n) => n,
            Err(_) => {
                println!("Error: Invalid number format in file.");
                return None;
            }
        };

        let card_name = parts[1].trim().to_string();
        
        *deck.entry(card_name).or_insert(0) += count;
        total_count += count;
    }

    if total_count != 60 {
        println!("Error: The total number of cards must be exactly 60. The file contains {} cards.", total_count);
        return None;
    }

    Some(deck)
}

fn ncr(n: usize, k: usize) -> usize {
    if k > n {
        return 0;
    }
    let mut result = 1;
    for i in 0..k {
        result *= n - i;
        result /= i + 1;
    }
    result
}

fn fixed_cards_prob(deck: &HashMap<String, usize>, hand: &HashMap<String, usize>) -> usize {
    let mut num_possibilities: usize = 1;

    for (card, count) in hand {
        num_possibilities *= ncr(deck[card], *count);
    }

    num_possibilities
}

fn calc_prob_old(deck: &HashMap<String, usize>, hand: &HashMap<String, usize>, num_cards: usize) -> f64 {
    let deck_size: usize = deck.values().sum();
    let num_fixed_cards: usize = hand.values().sum();
    let num_wildcards = num_cards - num_fixed_cards;

    let max_ways = ncr(deck_size, num_cards);

    let fixed_ways = fixed_cards_prob(deck, hand);

    let wildcard_ways = ncr(deck_size - num_fixed_cards, num_wildcards);

    println!("Number of ways to make hand: {}", fixed_ways * wildcard_ways);

    (fixed_ways * wildcard_ways) as f64 / max_ways as f64
}

fn main() {
    let deck: HashMap<String, usize>;
    if let Some(card_list) = generate_deck_from_file() {
        // println!("Deck successfully loaded. Here are the cards and their counts:");
        // for (card, count) in &card_list {
        //     println!("{}: {}", card, count);
        // }
        deck = card_list;
    } else {
        println!("Failed to generate deck from file.");
        return;
    }

    let num_cards = 7;
    let hand = HashMap::from([
        // ("Polluted Delta".to_string(), 2),
        // ("Force of Will".to_string(), 2),
        ("Underground Sea".to_string(), 1),
        // ("Animate Dead".to_string(), 2),
        // ("Ponder".to_string(), 3),
    ]);
    let prob = calc_prob_old(&deck, &hand, num_cards);
    
    println!("Probability of getting hand: {}", prob);

}