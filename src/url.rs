#[derive(Debug, PartialEq)]
pub struct Url {
    pub scheme: String,
    pub host: String,
    pub port: u16,
    pub path: String,
    pub query: Vec<(String, String)>,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn build_url_struct() {
        let u = Url {
            scheme: "http".to_string(),
            host: "example.com".to_string(),
            port: 80,
            path: "/".to_string(),
            query: vec![],
        };
        assert_eq!(u.host, "example.com");
    }
}
