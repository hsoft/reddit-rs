# reddit-rs

*A Rust library to consume Reddit's API*

## Status

Early development, incomplete featureset

## Features

* Fetch the frontpage listing.
* Fetch subsequent pages.
* Subreddit listings

Coming soon:

* Authentication

## Example

```rust
use reddit::Listing;

// Get the front page listing, unauthenticated.
let listing = Listing::get_frontpage().unwrap();
for link in listing.links().iter() {
    println!("{} {}", link.url(), link.title());
}
```

## Documentation

Not yet.

