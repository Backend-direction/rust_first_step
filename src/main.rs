use rand::{thread_rng, seq::SliceRandom};

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

  fn shuffle(&mut self) {
    let mut rng = thread_rng();

    self.cards.shuffle(&mut rng);
  }

  fn deal(&mut self, num_cards: usize)-> Vec<String> {
    return self.cards.split_off(
      self.cards.len() - num_cards
    );
  }
}

fn main() {
  let mut deck = Deck::new();

  // deck.shuffle();

  let cards = deck.deal(3);

  println!("Here is your deck: {:#?}", cards);
}
