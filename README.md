# Jeton

Jeton (pronounced as `/ʒətɔ̃/`) is a blazing fast tokenizer written in Rust.

# Example

```rs

use jeton::prelude::*;

fn main() {
    let tokenizer = TokenizerBuilder::new()
        .with_pre_tokenizer(ByteLevel::default())
        .with_post_processor(ByteLevel::default())
        .with_decoder(ByteLevel::default())
        .build();

    tokenizer.exec(vec![
        "Hello, world!",
        "This is a sample text"
    ]);
}

```

# License

This project is licensed under [**The MIT License**](https://en.wikipedia.org/wiki/MIT_License 'The MIT License').
Please read [`LICENSE`](./LICENSE 'LICENSE') for more information.
