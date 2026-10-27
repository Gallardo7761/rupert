mod act;
mod cute;
mod gif;
mod mood;
mod rough;

pub use act::ACT;
pub use cute::CUTE;
pub use gif::{Gif, GifClient};
pub use mood::MOOD;
pub use rough::ROUGH;

use nekosbest::Category;

#[derive(Debug, Clone, Copy)]
pub enum Kind {
    Targeted(&'static str),
    Solo(&'static str),
}

#[derive(Debug, Clone, Copy)]
pub struct Interaction {
    pub name: &'static str,
    pub description: &'static str,
    pub category: Category,
    pub kind: Kind,
}

impl Interaction {
    pub fn sentence(&self, author: &str, target: Option<&str>) -> Option<String> {
        match (self.kind, target) {
            (Kind::Targeted(phrase), Some(target)) => Some(format!("{author} {phrase} {target}")),
            (Kind::Targeted(_), None) => None,
            (Kind::Solo(phrase), _) => Some(format!("{author} {phrase}")),
        }
    }
}

pub const MAX_SUGGESTIONS: usize = 25;

pub fn all() -> impl Iterator<Item = &'static Interaction> {
    [CUTE, ROUGH, MOOD, ACT].into_iter().flatten()
}

pub fn find(name: &str) -> Option<&'static Interaction> {
    all().find(|interaction| interaction.name == name)
}

pub fn suggest(query: &str) -> Vec<&'static Interaction> {
    let query = query.trim().to_lowercase();
    let starts = all().filter(|i| i.name.starts_with(&query));
    let contains = all().filter(|i| !i.name.starts_with(&query) && i.name.contains(&query));
    starts.chain(contains).take(MAX_SUGGESTIONS).collect()
}

#[cfg(test)]
mod tests {
    use std::collections::HashSet;

    use super::*;

    const GROUPS: [&[Interaction]; 4] = [CUTE, ROUGH, MOOD, ACT];

    const HUG: Interaction = Interaction {
        name: "hug",
        description: "Abraza a alguien",
        category: Category::Hug,
        kind: Kind::Targeted("le dio un abrazo a"),
    };

    const BLUSH: Interaction = Interaction {
        name: "blush",
        description: "Sonrójate",
        category: Category::Blush,
        kind: Kind::Solo("se sonrojó"),
    };

    #[test]
    fn targeted_sentence_includes_the_target() {
        assert_eq!(
            HUG.sentence("Ana", Some("Luis")).as_deref(),
            Some("Ana le dio un abrazo a Luis")
        );
    }

    #[test]
    fn targeted_without_target_is_none() {
        assert_eq!(HUG.sentence("Ana", None), None);
    }

    #[test]
    fn solo_sentence_ignores_the_target() {
        assert_eq!(
            BLUSH.sentence("Ana", None).as_deref(),
            Some("Ana se sonrojó")
        );
        assert_eq!(
            BLUSH.sentence("Ana", Some("Luis")).as_deref(),
            Some("Ana se sonrojó")
        );
    }

    #[test]
    fn names_are_unique_across_groups() {
        let mut seen = HashSet::new();
        for interaction in all() {
            assert!(
                seen.insert(interaction.name),
                "duplicated interaction: {}",
                interaction.name
            );
        }
    }

    #[test]
    fn groups_fit_in_a_discord_command() {
        for group in GROUPS {
            assert!(!group.is_empty() && group.len() <= 25);
        }
    }

    #[test]
    fn every_interaction_has_text() {
        for interaction in all() {
            let phrase = match interaction.kind {
                Kind::Targeted(phrase) | Kind::Solo(phrase) => phrase,
            };
            assert!(!interaction.description.is_empty() && !phrase.is_empty());
            assert!(interaction.description.chars().count() <= 100);
        }
    }

    #[test]
    fn all_covers_every_group() {
        let total: usize = GROUPS.iter().map(|group| group.len()).sum();
        assert_eq!(all().count(), total);
    }

    #[test]
    fn find_by_name() {
        assert_eq!(find("hug").map(|i| i.name), Some("hug"));
        assert!(find("patata").is_none());
        assert!(find("HUG").is_none());
    }

    #[test]
    fn empty_query_suggests_the_maximum() {
        assert_eq!(suggest("").len(), MAX_SUGGESTIONS);
    }

    #[test]
    fn suggestions_ignore_case_and_spaces() {
        let names: Vec<_> = suggest("  NO ").iter().map(|i| i.name).collect();
        assert!(names.contains(&"nod"));
        assert!(names.contains(&"nope"));
    }

    #[test]
    fn prefix_matches_come_first() {
        let names: Vec<_> = suggest("ki").iter().map(|i| i.name).collect();
        assert_eq!(names, ["kiss", "kick", "blowkiss"]);
    }

    #[test]
    fn unknown_query_suggests_nothing() {
        assert!(suggest("patata").is_empty());
    }
}
