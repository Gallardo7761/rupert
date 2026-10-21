use std::collections::VecDeque;

use crate::music::track::Track;

#[derive(Debug, Default)]
pub struct Queue {
    current_track: Option<Track>,
    next_tracks: VecDeque<Track>,
}

impl Queue {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn push(&mut self, track: Track) -> usize {
        if self.current_track.is_none() {
            self.current_track = Some(track);
            return 0;
        }

        self.next_tracks.push_back(track);
        self.next_tracks.len()
    }

    pub fn insert(&mut self, track: Track) -> usize {
        if self.current_track.is_none() {
            self.current_track = Some(track);
            return 0;
        }

        self.next_tracks.push_front(track);
        1
    }

    pub fn advance(&mut self) -> Option<&Track> {
        self.current_track = self.next_tracks.pop_front();
        self.current()
    }

    pub fn clear(&mut self) {
        self.current_track = None;
        self.next_tracks.clear();
    }

    pub fn current(&self) -> Option<&Track> {
        self.current_track.as_ref()
    }

    pub fn upcoming(&self) -> impl Iterator<Item = &Track> {
        self.next_tracks.iter()
    }

    pub fn is_empty(&self) -> bool {
        self.current_track.is_none() && self.next_tracks.is_empty()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn track(title: &str) -> Track {
        Track::new(
            title.to_string(),
            format!("https://youtu.be/{title}"),
            None,
            "author".to_string(),
            None,
            "user".to_string(),
        )
    }

    fn titles(queue: &Queue) -> Vec<&str> {
        queue.upcoming().map(Track::title).collect()
    }

    #[test]
    fn new_queue_is_empty() {
        let queue = Queue::new();
        assert!(queue.is_empty());
        assert!(queue.current().is_none());
        assert_eq!(queue.upcoming().count(), 0);
    }

    #[test]
    fn first_push_plays_now() {
        let mut queue = Queue::new();
        assert_eq!(queue.push(track("a")), 0);
        assert_eq!(queue.current().map(Track::title), Some("a"));
        assert!(!queue.is_empty());
    }

    #[test]
    fn next_pushes_wait_in_order() {
        let mut queue = Queue::new();
        queue.push(track("a"));
        assert_eq!(queue.push(track("b")), 1);
        assert_eq!(queue.push(track("c")), 2);
        assert_eq!(queue.current().map(Track::title), Some("a"));
        assert_eq!(titles(&queue), ["b", "c"]);
    }

    #[test]
    fn advance_plays_next_until_empty() {
        let mut queue = Queue::new();
        queue.push(track("a"));
        queue.push(track("b"));

        assert_eq!(queue.advance().map(Track::title), Some("b"));
        assert_eq!(titles(&queue), Vec::<&str>::new());
        assert!(queue.advance().is_none());
        assert!(queue.is_empty());
    }

    #[test]
    fn advance_on_empty_queue_does_nothing() {
        let mut queue = Queue::new();
        assert!(queue.advance().is_none());
        assert!(queue.is_empty());
    }

    #[test]
    fn push_after_finishing_plays_now() {
        let mut queue = Queue::new();
        queue.push(track("a"));
        queue.advance();
        assert_eq!(queue.push(track("b")), 0);
        assert_eq!(queue.current().map(Track::title), Some("b"));
    }

    #[test]
    fn clear_removes_everything() {
        let mut queue = Queue::new();
        queue.push(track("a"));
        queue.push(track("b"));
        queue.clear();
        assert!(queue.is_empty());
        assert!(queue.current().is_none());
    }
}
