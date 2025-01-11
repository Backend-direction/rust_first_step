#[derive(Debug)]

struct Deck {
    cards: Vec<String>,
}

impl Deck {
  fn new() -> Self {
    // List of suits - 'hearts, spades;
    let suits = ["Hearts", "Spades", "Diamonds"];
        
    // List of values - ace, two, three;
    let values = ["Ace", "Two", "Three"];

    let mut cards = vec![];

    // Double for loop;
    for suit in suits {
        for value in values {
            let card = format!("{} of {}", value, suit);
            cards.push(card);
        }
    }

    return Deck { cards };
  }
}

fn main() {
  let deck = Deck::new();

  println!("Here is your deck: {:#?}", deck);
}
