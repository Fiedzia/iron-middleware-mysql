# Mysql connection pool middleware for Iron web framework

This middleware will insert connection pool into request.extensions

Usage:

```rust
extern crate wordnet_stemmer;

use wordnet_stemmer::{WordnetStemmer, NOUN};

pub fn main(){
    let wn = ::WordnetStemmer::new("/home/maciej/nltk_data/corpora/wordnet/").unwrap();
    println!("{}", wn.lemma(NOUN, "dogs".to_owned());
}

```

See included example for details
