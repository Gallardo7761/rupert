use std::fmt;

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum Coin {
    Heads,
    Tails,
}

impl fmt::Display for Coin {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Coin::Heads => write!(f, "Cara 😀"),
            Coin::Tails => write!(f, "Cruz ❌"),
        }
    }
}

pub fn dice_faces(input: &str) -> Option<u32> {
    match input.trim().to_lowercase().as_str() {
        "d4" => Some(4),
        "d6" => Some(6),
        "d8" => Some(8),
        "d10" => Some(10),
        "d12" => Some(12),
        "d20" => Some(20),
        "d100" => Some(100),
        _ => None,
    }
}

pub fn roll(faces: u32) -> u32 {
    rand::random_range(1..=faces)
}

pub fn flip_coin() -> Coin {
    if rand::random::<bool>() {
        Coin::Heads
    } else {
        Coin::Tails
    }
}

pub fn banana_size() -> i32 {
    rand::random_range(-5..=47)
}

pub fn ship_percentage(a: u64, b: u64) -> u8 {
    let (lo, hi) = if a <= b { (a, b) } else { (b, a) };
    let mut z = lo.wrapping_mul(0x9E37_79B9_7F4A_7C15) ^ hi;
    z = (z ^ (z >> 30)).wrapping_mul(0xBF58_476D_1CE4_E5B9);
    z = (z ^ (z >> 27)).wrapping_mul(0x94D0_49BB_1331_11EB);
    z ^= z >> 31;
    (z % 101) as u8
}

#[cfg(test)]
mod tests {
    use super::*;

    const VALID_DICE: [(&str, u32); 7] = [
        ("d4", 4),
        ("d6", 6),
        ("d8", 8),
        ("d10", 10),
        ("d12", 12),
        ("d20", 20),
        ("d100", 100),
    ];

    #[test]
    fn valid_dice_are_parsed() {
        for (text, faces) in VALID_DICE {
            assert_eq!(dice_faces(text), Some(faces), "falla con {text}");
        }
    }

    #[test]
    fn parsing_ignores_case_and_spaces() {
        assert_eq!(dice_faces("D20"), Some(20));
        assert_eq!(dice_faces("  d6 "), Some(6));
    }

    #[test]
    fn invalid_dice_are_rejected() {
        for text in ["", "d", "d0", "d7", "6", "abc", "2d6"] {
            assert_eq!(dice_faces(text), None, "{text:?} debería ser inválido");
        }
    }

    #[test]
    fn roll_stays_in_range_and_reaches_both_extremes() {
        for (_, faces) in VALID_DICE {
            let rolls: Vec<u32> = (0..10_000).map(|_| roll(faces)).collect();
            let min = rolls.iter().copied().min().expect("hay tiradas");
            let max = rolls.iter().copied().max().expect("hay tiradas");
            assert_eq!(min, 1, "d{faces} nunca saca 1");
            assert_eq!(max, faces, "d{faces} nunca saca {faces}");
        }
    }

    #[test]
    fn coin_shows_both_faces() {
        let flips: Vec<Coin> = (0..1_000).map(|_| flip_coin()).collect();
        assert!(flips.contains(&Coin::Heads));
        assert!(flips.contains(&Coin::Tails));
    }

    #[test]
    fn banana_stays_in_range_and_reaches_both_extremes() {
        let sizes: Vec<i32> = (0..10_000).map(|_| banana_size()).collect();
        assert_eq!(sizes.iter().copied().min(), Some(-5));
        assert_eq!(sizes.iter().copied().max(), Some(47));
    }

    #[test]
    fn ship_is_symmetric() {
        for (a, b) in [(1, 2), (42, 7), (u64::MAX, 3), (1, 1)] {
            assert_eq!(ship_percentage(a, b), ship_percentage(b, a), "({a}, {b})");
        }
    }

    #[test]
    fn ship_stays_in_range() {
        for a in 1..200u64 {
            for b in 1..200u64 {
                assert!(ship_percentage(a, b) <= 100, "({a}, {b})");
            }
        }
    }

    #[test]
    fn ship_uses_the_whole_range() {
        let values: Vec<u8> = (1..200u64)
            .flat_map(|a| (1..200u64).map(move |b| ship_percentage(a, b)))
            .collect();
        assert!(values.iter().any(|&p| p < 10));
        assert!(values.iter().any(|&p| p > 90));
    }
}
