/* Copyright 2015 Virgil Dupras
 *
 * This software is licensed under the "LGPLv3" License as described in the "LICENSE" file,
 * which should be included with this package. The terms are also available at
 * http://www.gnu.org/licenses/lgpl-3.0.html
 */

pub use http::{authenticate, AuthToken};
pub use listing::{Listing, Link, LinkType};

extern crate curl;
extern crate rustc_serialize;

mod error;
mod http;
mod listing;

