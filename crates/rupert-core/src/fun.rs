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
    rand::random_range(1..faces)
}