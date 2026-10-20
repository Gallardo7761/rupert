use rand::seq::{IndexedRandom, IteratorRandom};

const BANDS: &str = include_str!("../../../assets/bands.txt");
const GAMES: &str = include_str!("../../../assets/games.txt");
const SHOWS: &str = include_str!("../../../assets/shows.txt");

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum RActivityType {
    Playing,
    Listening,
    Watching,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct RStatus {
    activity_type: RActivityType,
    name: &'static str,
}

impl RStatus {
    pub fn new(activity_type: RActivityType, name: &'static str) -> Self {
        Self {
            activity_type,
            name,
        }
    }

    pub fn name(&self) -> &'static str {
        self.name
    }

    pub fn activity_type(&self) -> RActivityType {
        self.activity_type
    }
}

impl RActivityType {
    pub const ALL: [RActivityType; 3] = [Self::Playing, Self::Listening, Self::Watching];

    pub fn names(&self) -> std::str::Lines<'static> {
        match self {
            Self::Playing => GAMES.lines(),
            Self::Listening => BANDS.lines(),
            Self::Watching => SHOWS.lines(),
        }
    }
}

pub fn random() -> RStatus {
    let mut rng = rand::rng();
    let activity_type = *RActivityType::ALL.choose(&mut rng).unwrap();
    let name = activity_type.names().choose(&mut rng).unwrap();
    RStatus::new(activity_type, name)
}

#[cfg(test)]
mod tests {
    use super::*;

    const MAX_NAME_LEN: usize = 128;

    fn all_names() -> impl Iterator<Item = &'static str> {
        RActivityType::ALL.iter().flat_map(|a| a.names())
    }

    #[test]
    fn every_type_has_names() {
        for activity_type in RActivityType::ALL {
            assert!(activity_type.names().next().is_some(), "{activity_type:?}");
        }
    }

    #[test]
    fn names_are_trimmed_and_not_empty() {
        for name in all_names() {
            assert!(!name.is_empty());
            assert_eq!(name, name.trim(), "{name:?}");
        }
    }

    #[test]
    fn names_fit_in_discord() {
        for name in all_names() {
            assert!(name.chars().count() <= MAX_NAME_LEN, "{name:?}");
        }
    }

    #[test]
    fn random_name_matches_its_type() {
        for _ in 0..1000 {
            let status = random();
            assert!(
                status.activity_type().names().any(|n| n == status.name()),
                "{status:?}"
            );
        }
    }
}
