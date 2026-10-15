use nekosbest::Category;

use super::{Interaction, Kind};

/// Aggressive interactions, most of them with someone else.
pub const ROUGH: &[Interaction] = &[
    Interaction {
        name: "slap",
        description: "Dale una bofetada a alguien",
        category: Category::Slap,
        kind: Kind::Targeted("le dio una bofetada a"),
    },
    Interaction {
        name: "punch",
        description: "Dale un puñetazo a alguien",
        category: Category::Punch,
        kind: Kind::Targeted("le dio un puñetazo a"),
    },
    Interaction {
        name: "kick",
        description: "Dale una patada a alguien",
        category: Category::Kick,
        kind: Kind::Targeted("le dio una patada a"),
    },
    Interaction {
        name: "bonk",
        description: "Dale un coscorrón a alguien",
        category: Category::Bonk,
        kind: Kind::Targeted("le dio un coscorrón a"),
    },
    Interaction {
        name: "shoot",
        description: "Dispara a alguien",
        category: Category::Shoot,
        kind: Kind::Targeted("disparó a"),
    },
    Interaction {
        name: "bite",
        description: "Muerde a alguien",
        category: Category::Bite,
        kind: Kind::Targeted("mordió a"),
    },
    Interaction {
        name: "yeet",
        description: "Lanza a alguien por los aires",
        category: Category::Yeet,
        kind: Kind::Targeted("lanzó por los aires a"),
    },
    Interaction {
        name: "baka",
        description: "Llama baka a alguien",
        category: Category::Baka,
        kind: Kind::Targeted("llamó baka a"),
    },
    Interaction {
        name: "shake",
        description: "Zarandea a alguien",
        category: Category::Shake,
        kind: Kind::Targeted("zarandeó a"),
    },
    Interaction {
        name: "bleh",
        description: "Sácale la lengua a alguien",
        category: Category::Bleh,
        kind: Kind::Targeted("le sacó la lengua a"),
    },
    Interaction {
        name: "stare",
        description: "Mira fijamente a alguien",
        category: Category::Stare,
        kind: Kind::Targeted("se quedó mirando fijamente a"),
    },
    Interaction {
        name: "angry",
        description: "Enfádate",
        category: Category::Angry,
        kind: Kind::Solo("se enfadó"),
    },
    Interaction {
        name: "tableflip",
        description: "Vuelca la mesa",
        category: Category::Tableflip,
        kind: Kind::Solo("volcó la mesa"),
    },
];
