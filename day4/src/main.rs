fn main() {
    let input = card_input();

    let used_card_count = simulate_card(&input);

    println!("{:?}", used_card_count);
}

// Expects a vector of cards that denotes how many following cards must be
// copied once.
fn simulate_card(cards: &Vec<u32>) -> u32 {
    let numcards = cards.len();
    let mut onhand_cards = vec![1;numcards];

    for ind in 0..numcards {
        // Must choose following cards
        for cd in 1..=cards[ind] {
            if ind + cd as usize >= numcards { continue }
            onhand_cards[ind + cd as usize] += onhand_cards[ind];
        }
    }

    println!("{:?}", onhand_cards);
    onhand_cards.iter().sum()
}

fn card_input() -> Vec<u32> {
    let input = include_str!("../input");

    // Card 1 will be at index 0, etc.
    let out: Vec<u32> = input.lines()
        .map(|x| x.split_once(':')
             .expect("Can strip prefix 'Card'").1
             .split_once('|')
             .unwrap()
        )
        .map(|(winners, numbers)| {
            let mut win_num: u32 = 0;
            let winners: Vec<&str> = winners.split_whitespace().collect();
            for num in numbers.split_whitespace() {
                if winners.contains(&num) { win_num += 1 }
            }
            win_num
        }).collect();

    out
}
