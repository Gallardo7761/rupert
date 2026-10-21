const YOUTUBE_HOSTS: [&str; 4] = [
    "youtube.com",
    "m.youtube.com",
    "music.youtube.com",
    "youtu.be",
];

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Query {
    Url(String),
    Search(String),
}

pub fn parse(input: &str) -> Query {
    let input = input.trim();

    let Some(rest) = input
        .strip_prefix("https://")
        .or_else(|| input.strip_prefix("http://"))
    else {
        return Query::Search(input.to_string());
    };

    let host = rest.split_once('/').map_or(rest, |(host, _)| host);
    let host = host.strip_prefix("www.").unwrap_or(host);

    if YOUTUBE_HOSTS.contains(&host) {
        Query::Url(input.to_string())
    } else {
        Query::Search(input.to_string())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn url(s: &str) -> Query {
        Query::Url(s.to_string())
    }

    fn search(s: &str) -> Query {
        Query::Search(s.to_string())
    }

    #[test]
    fn youtube_links_are_urls() {
        for link in [
            "https://www.youtube.com/watch?v=abc",
            "https://youtube.com/watch?v=abc",
            "https://m.youtube.com/watch?v=abc",
            "https://music.youtube.com/watch?v=abc",
            "https://youtu.be/abc",
            "https://www.youtube.com/playlist?list=abc",
        ] {
            assert_eq!(parse(link), url(link));
        }
    }

    #[test]
    fn http_is_accepted() {
        let link = "http://youtube.com/watch?v=abc";
        assert_eq!(parse(link), url(link));
    }

    #[test]
    fn host_without_path_is_url() {
        assert_eq!(parse("https://youtu.be"), url("https://youtu.be"));
    }

    #[test]
    fn text_is_search() {
        assert_eq!(parse("bad bunny"), search("bad bunny"));
    }

    #[test]
    fn domain_without_scheme_is_search() {
        assert_eq!(
            parse("youtube.com tutorial"),
            search("youtube.com tutorial")
        );
    }

    #[test]
    fn other_links_are_search() {
        let link = "https://example.com/video";
        assert_eq!(parse(link), search(link));
    }

    #[test]
    fn lookalike_hosts_are_search() {
        for link in [
            "https://notyoutube.com/watch?v=abc",
            "https://youtube.com.evil.com/watch?v=abc",
        ] {
            assert_eq!(parse(link), search(link));
        }
    }

    #[test]
    fn input_is_trimmed() {
        assert_eq!(parse("   bad bunny   "), search("bad bunny"));
        assert_eq!(
            parse("  https://youtu.be/abc "),
            url("https://youtu.be/abc")
        );
    }
}
