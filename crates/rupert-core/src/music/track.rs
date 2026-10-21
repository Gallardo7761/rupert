use std::time::Duration;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Track {
    title: String,
    url: String,
    thumbnail: Option<String>,
    author: String,
    duration: Option<Duration>,
    requested_by: String,
}

impl Track {
    pub fn new(
        title: String,
        url: String,
        thumbnail: Option<String>,
        author: String,
        duration: Option<Duration>,
        requested_by: String,
    ) -> Self {
        Self {
            title,
            url,
            thumbnail,
            author,
            duration,
            requested_by,
        }
    }

    pub fn title(&self) -> &str {
        &self.title
    }

    pub fn url(&self) -> &str {
        &self.url
    }

    pub fn thumbnail(&self) -> Option<&str> {
        self.thumbnail.as_deref()
    }

    pub fn author(&self) -> &str {
        &self.author
    }

    pub fn duration(&self) -> Option<Duration> {
        self.duration
    }

    pub fn requested_by(&self) -> &str {
        &self.requested_by
    }
}

pub fn format_duration(duration: Duration) -> String {
    let seconds = duration.as_secs() % 60;
    let minutes = duration.as_secs() / 60 % 60;
    let hours = duration.as_secs() / 3600;
    if hours > 0 {
        format!("{hours}:{minutes:02}:{seconds:02}")
    } else {
        format!("{minutes}:{seconds:02}")
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_formatting() {
        assert_eq!(format_duration(Duration::from_secs(0)), "0:00");
        assert_eq!(format_duration(Duration::from_secs(187)), "3:07");
        assert_eq!(format_duration(Duration::from_secs(3765)), "1:02:45");
    }
}
