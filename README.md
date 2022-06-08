# JokeTeller - an API client for Sv443's JokeAPI

Current Version: `0.2.1`

```
[dependencies]
joketeller = "0.2.1"
```

View the [Sv443 API here](https://jokeapi.dev/).

This crate is in active development, there is full support for getting jokes, but all other API features are being added with time.

## Basic Usage
```rs
use joketeller::{
    Joker, Category, BlacklistFlag,
}

let mut joker_instance: Joker = Joker::new();

// Chainable API
joker_instance
    .add_categories(&mut vec![Category::Programming, Category::Pun])
    .add_blacklist_flags(&mut vec![BlacklistFlag::Explicit]);

// get JSON joke
let joke = joker_instance.get_joke().unwrap();

// get https url to make your own request
let built_api_url = joker_instance.build_url().unwrap();
```

## Documentation
The docs are always being updated, current docs can be found [here](https://docs.rs/joketeller/latest/joketeller/).