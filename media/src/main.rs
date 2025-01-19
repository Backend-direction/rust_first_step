mod content;

use content::media::Media;
use content::catalog::Catalog;

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
}
