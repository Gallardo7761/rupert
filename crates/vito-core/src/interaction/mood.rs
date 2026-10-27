use nekosbest::Category;

use super::{Interaction, Kind};

/// Emotions, without a target.
pub const MOOD: &[Interaction] = &[
    Interaction {
        name: "blush",
        description: "Sonrójate",
        category: Category::Blush,
        kind: Kind::Solo("se sonrojó"),
    },
    Interaction {
        name: "cry",
        description: "Llora",
        category: Category::Cry,
        kind: Kind::Solo("se puso a llorar"),
    },
    Interaction {
        name: "smile",
        description: "Sonríe",
        category: Category::Smile,
        kind: Kind::Solo("sonrió"),
    },
    Interaction {
        name: "laugh",
        description: "Ríete",
        category: Category::Laugh,
        kind: Kind::Solo("se echó a reír"),
    },
    Interaction {
        name: "teehee",
        description: "Ríete por lo bajini",
        category: Category::Teehee,
        kind: Kind::Solo("se rio por lo bajini"),
    },
    Interaction {
        name: "happy",
        description: "Muestra tu alegría",
        category: Category::Happy,
        kind: Kind::Solo("está que no cabe de alegría"),
    },
    Interaction {
        name: "pout",
        description: "Haz pucheros",
        category: Category::Pout,
        kind: Kind::Solo("está haciendo pucheros"),
    },
    Interaction {
        name: "smug",
        description: "Pon cara de suficiencia",
        category: Category::Smug,
        kind: Kind::Solo("puso cara de suficiencia"),
    },
    Interaction {
        name: "shocked",
        description: "Quédate en shock",
        category: Category::Shocked,
        kind: Kind::Solo("se quedó en shock"),
    },
    Interaction {
        name: "confused",
        description: "Muestra tu confusión",
        category: Category::Confused,
        kind: Kind::Solo("no entiende nada"),
    },
    Interaction {
        name: "bored",
        description: "Muestra tu aburrimiento",
        category: Category::Bored,
        kind: Kind::Solo("se está aburriendo"),
    },
];
