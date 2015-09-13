/* Copyright 2015 Virgil Dupras
 *
 * This software is licensed under the "LGPLv3" License as described in the "LICENSE" file,
 * which should be included with this package. The terms are also available at
 * http://www.gnu.org/licenses/lgpl-3.0.html
 */

use reddit::{LinkType, Listing};
extern crate reddit;

fn print_listing(listing: &Listing) {
    for link in listing.links().iter() {
        let typedesc = match link.type_() {
            LinkType::Url => "U",
            LinkType::Text => "T",
        };
        println!("{} {} {} {} ({} comments)", typedesc, link.score(), link.url(), link.title(), link.num_comments());
    }
}

fn main() {
    let listing = Listing::get().unwrap();
    print_listing(&listing);
    println!("Oh, and here's a second page, because we can!");
    print_listing(&listing.next().unwrap());
}
