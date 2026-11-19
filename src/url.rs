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

    let (host, path) = match rest.find('/') {
        Some(i) => (&rest[..i], &rest[i..]),
        None => (rest, "/"),
    };
    let u = Url {
        scheme: scheme.to_string(),
        host: host.to_string(),
        port: 80,
        path: path.to_string(),
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

    #[test]
    fn host_and_path() {
        let res = parse_url("http://example.com/index.html").unwrap();
        assert_eq!(res.host, "example.com");
        assert_eq!(res.path, "/index.html");
    }

    #[test]
    fn host_and_path_without_params() {
        let res = parse_url("http://example.com").unwrap();
        assert_eq!(res.host, "example.com");
        assert_eq!(res.path, "/");
    }
}
