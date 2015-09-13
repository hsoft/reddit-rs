# reddit-rs

*A Rust library to consume Reddit's API*

## Status

Early development, incomplete featureset

## Features

* Fetch the frontpage listing.
* Fetch subsequent pages.

Coming soon:

* Authentication
* Subreddit listings

## Example

```rust
use reddit::Listing;

// Get the front page listing, unauthenticated.
let listing = Listing::get().unwrap();
for link in listing.links().iter() {
    println!("{} {}", link.url(), link.title());
}
```

## Documentation

Not yet.
