use crate::models::location::Location;
use reqwest::Client as ReqwestClient;

mod models;

pub struct Client {
    http_client: ReqwestClient,
    base_url: String,
    location: Location,
}

impl Client {
    pub fn new() {}
}

pub fn add(left: usize, right: usize) -> usize {
    left + right
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let result = add(2, 2);
        assert_eq!(result, 4);
    }
}
