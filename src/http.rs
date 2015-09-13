/* Copyright 2015 Virgil Dupras
 *
 * This software is licensed under the "LGPLv3" License as described in the "LICENSE" file,
 * which should be included with this package. The terms are also available at
 * http://www.gnu.org/licenses/lgpl-3.0.html
 */

use rustc_serialize::{json, base64};
use rustc_serialize::base64::ToBase64;
use curl;

pub type AuthToken = String;

pub fn authenticate(username: &str, password: &str, appid: &str, appsecret: &str) -> AuthToken {
    let auth_pair = format!("{}:{}", appid, appsecret);
    let http_auth_hash = auth_pair.into_bytes()[..].to_base64(base64::Config {
        char_set: base64::CharacterSet::Standard,
        newline: base64::Newline::LF,
        pad: false,
        line_length: None,
    });
    #[derive(RustcDecodable)]
    struct Response {
        access_token: String,
    }
    let postdata = format!("grant_type=password&username={}&password={}", username, password);
    let resp = curl::http::handle().
        post("https://www.reddit.com/api/v1/access_token", &postdata[..]).
        header("Content-Type", "application/x-www-form-urlencoded").
        header("Authorization", &format!("Basic {}", http_auth_hash)).
        exec().unwrap();
    let body = String::from_utf8(resp.get_body().into_iter().map(|&u| u).collect()).unwrap();
    let result: Response = json::decode(&body).unwrap();
    result.access_token
}
