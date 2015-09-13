/* Copyright 2015 Virgil Dupras
 *
 * This software is licensed under the "LGPLv3" License as described in the "LICENSE" file,
 * which should be included with this package. The terms are also available at
 * http://www.gnu.org/licenses/lgpl-3.0.html
 */

use curl;
use rustc_serialize::{json, Decodable, Decoder};

use error::ClientError;

#[derive(Clone, Copy)]
pub enum LinkType {
    Url,
    Text,
}

/// A link as shown in a reddit Listing
pub struct Link {
    type_: LinkType,
    /// For URL links, it's the straight URL, for text link, it's the comments link.
    url: String,
    title: String,
    score: i32,
    num_comments: u32,
}

impl Link {
    pub fn type_(&self) -> LinkType {
        self.type_
    }

    pub fn url(&self) -> &str {
        &self.url
    }

    pub fn title(&self) -> &str {
        &self.title
    }

    pub fn score(&self) -> i32 {
        self.score
    }

    pub fn num_comments(&self) -> u32 {
        self.num_comments
    }
}

impl Decodable for Link {
    fn decode<D: Decoder>(decoder: &mut D) -> Result<Link, D::Error> {
        decoder.read_struct("root", 0, |d| {
            d.read_struct_field("data", 0, |d| {
                d.read_struct("data", 0, |d| {
                    let selftext: String = try!(d.read_struct_field("selftext", 0, Decodable::decode));
                    Ok(Link {
                        type_: if selftext == "" { LinkType::Url } else { LinkType::Text },
                        title: try!(d.read_struct_field("title", 0, Decodable::decode)),
                        url: try!(d.read_struct_field("url", 0, Decodable::decode)),
                        score: try!(d.read_struct_field("score", 0, Decodable::decode)),
                        num_comments: try!(d.read_struct_field("num_comments", 0, Decodable::decode)),
                    })
                })
            })
        })
    }
}

pub struct Listing {
    links: Vec<Link>,
    after: Option<String>,
}

impl Listing {
    fn get_with_params(params: &[(&str, &str)]) -> Result<Listing, ClientError> {
        let mut url = "https://www.reddit.com/.json".to_owned();
        if params.len() > 0 {
            let params_str = params.iter().fold("".to_owned(), |mut s, &(k, v)| { s.push_str(&format!("{}={}", k, v)); s });
            url.push('?');
            url.push_str(&params_str);
        }
        let resp = curl::http::handle().get(url).exec().unwrap();
        let body = String::from_utf8(resp.get_body().iter().map(|&u| u).collect()).unwrap();
        match json::decode(&body) {
            Ok(val) => Ok(val),
            Err(_) => Err(ClientError::Oops),
        }
    }

    pub fn get() -> Result<Listing, ClientError> {
        Self::get_with_params(&[])
    }

    pub fn links(&self) -> &Vec<Link> {
        &self.links
    }

    pub fn next(&self) -> Result<Listing, ClientError> {
        let params = match self.after {
            Some(ref after) => vec![("after", &after[..])],
            None => vec![],
        };
        Self::get_with_params(&params[..])
    }
}

impl Decodable for Listing {
    fn decode<D: Decoder>(decoder: &mut D) -> Result<Listing, D::Error> {
        decoder.read_struct("root", 0, |d| {
            d.read_struct_field("data", 0, |d| {
                let links = try!(d.read_struct_field("children", 0, |d| {
                    d.read_seq(|d, len| {
                        let mut links = Vec::new();
                        for i in 0..len {
                            let link: Link = try!(d.read_seq_elt(i, Decodable::decode));
                            links.push(link);
                        }
                        Ok(links)
                    })
                }));
                let after: Option<String> = d.read_struct_field("after", 0, Decodable::decode).ok();
                Ok(Listing {
                    links: links,
                    after: after,
                })
            })
        })
    }
}

