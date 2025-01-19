#[derive(Debug)]
enum Media {
  Book { title: String, author: String },
  Movie { title: String, director: String },
  Audiobook { title: String },
  Podcast(u32),
  Placeholder
}

impl Media {
  fn description(&self) -> String {
  //   if let Media::Book { title, author} = self {
  //     return format!("Book: {}, {}", title, author);
  //   } else if let Media::Movie { title, director} = self {
  //     return format!("Movie: {}, {}", title, director);
  //   } else if let Media::Audiobook { title} = self {
  //     return format!("Audiobook: {}", title);
  //   } else {
  //     return String::from("Media description");
  //   }
  // }

    match self {
      Media::Book { title, author } => {
        format!("Book: {}, {}", title, author)
      },
      Media::Movie { title, director } => {
        format!("Movie: {}, {}", title, director)
      },
      Media::Audiobook { title } => {
        format!("Audiobook: {}", title)
      },
      Media::Podcast(id) => {
        format!("Podcast: {}", id)
      },
      Media::Placeholder => {
        format!("Placeholder")
      },
    }
  }
}

#[derive(Debug)]
struct Catalog {
  items: Vec<Media>
}

impl Catalog {
  fn new() -> Self {
    Catalog { items: vec![] }
  }

  fn add(&mut self, media: Media) {
    self.items.push(media);
  }

  fn get_by_index(&self, index: usize) -> MightHaveAValue {
    if self.items.len() > index {
     return MightHaveAValue::ThereIsAValue(&self.items[index])
    } else {
      return MightHaveAValue::NoValueAvailable;
    }
  }
}

#[derive(Debug)]
enum MightHaveAValue<'a> {
  ThereIsAValue(&'a Media),
  NoValueAvailable
}

fn prin_media(media: Media) {
  println!("{:#?}", media);   
}

fn main() {
  let audiobook = Media::Audiobook {
      title: String::from("An audio book")
  };
  let book = Media::Book {
    title: String::from("Fine book"),
    author: String::from("Rolling")
  };
  let podcast = Media::Podcast(10);
  let placeholder= Media::Placeholder;

  let mut catalog = Catalog::new();

  catalog.add(audiobook);
  catalog.add(book);
  catalog.add(podcast);
  catalog.add(placeholder);
  
  let item = catalog.get_by_index(00);
  
  println!("{:#?}", item);
}
