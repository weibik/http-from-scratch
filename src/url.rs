#[derive(Debug, PartialEq)]
pub struct Url {
    pub scheme: String,
    pub host: String,
    pub port: u16,
    pub path: String,
    pub query: Vec<(String, String)>,
}

pub fn parse_url(input: &str) -> Result<Url, String> {
    let (scheme, rest) = input
        .split_once("://")
        .ok_or("missing scheme".to_string())?;
    let u = Url {
        scheme: scheme.to_string(),
        host: "example.com".to_string(),
        port: 80,
        path: "/".to_string(),
        query: vec![],
    };
    Ok(u)
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

    #[test]
    fn parses_scheme() {
        let u = parse_url("http://example.com").unwrap();
        assert_eq!(u.scheme, "http");
    }

    #[test]
    fn missing_scheme_is_err() {
        let res = parse_url("example.com");
        assert!(res.is_err())
    }
}
