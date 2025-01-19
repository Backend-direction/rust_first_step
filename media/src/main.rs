#[derive(Debug)]
enum Media {
  Book { title: String, author: String },
  Movie { title: String, director: String },
  Audiobook { title: String }
}

impl Media {
  fn description(&self) -> String {
    if let Media::Book { title, author} = self {
      return format!("Book: {}, {}", title, author);
    } else if let Media::Movie { title, director} = self {
      return format!("Movie: {}, {}", title, director);
    } else if let Media::Audiobook { title} = self {
      return format!("Audiobook: {}", title);
    } else {
      return String::from("Media description");
    }
  }
}


fn prin_media(media: Media) {
  println!("{:#?}", media);   
}

fn main() {
  let audiobook = Media::Audiobook {
      title: String::from("An audio book")
  };

  let desc = audiobook.description();

  print!("{}", desc);
}
