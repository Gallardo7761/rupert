use nekosbest::Category;

use super::{Interaction, Kind};

pub const ACT: &[Interaction] = &[
    Interaction {
        name: "dance",
        description: "Baila",
        category: Category::Dance,
        kind: Kind::Solo("se puso a bailar"),
    },
    Interaction {
        name: "clap",
        description: "Aplaude",
        category: Category::Clap,
        kind: Kind::Solo("aplaudió"),
    },
    Interaction {
        name: "spin",
        description: "Da vueltas",
        category: Category::Spin,
        kind: Kind::Solo("se puso a dar vueltas"),
    },
    Interaction {
        name: "nod",
        description: "Asiente",
        category: Category::Nod,
        kind: Kind::Solo("asintió"),
    },
    Interaction {
        name: "nope",
        description: "Di que nope",
        category: Category::Nope,
        kind: Kind::Solo("dijo que nope"),
    },
    Interaction {
        name: "shrug",
        description: "Encógete de hombros",
        category: Category::Shrug,
        kind: Kind::Solo("se encogió de hombros"),
    },
    Interaction {
        name: "facepalm",
        description: "Llévate la mano a la cara",
        category: Category::Facepalm,
        kind: Kind::Solo("se llevó la mano a la cara"),
    },
    Interaction {
        name: "think",
        description: "Piensa",
        category: Category::Think,
        kind: Kind::Solo("se puso a pensar"),
    },
    Interaction {
        name: "sip",
        description: "Dale un sorbito a tu bebida",
        category: Category::Sip,
        kind: Kind::Solo("le dio un sorbito a su bebida"),
    },
    Interaction {
        name: "nom",
        description: "Come algo",
        category: Category::Nom,
        kind: Kind::Solo("se puso a comer"),
    },
    Interaction {
        name: "yawn",
        description: "Bosteza",
        category: Category::Yawn,
        kind: Kind::Solo("bostezó"),
    },
    Interaction {
        name: "sleep",
        description: "Échate a dormir",
        category: Category::Sleep,
        kind: Kind::Solo("se echó a dormir"),
    },
    Interaction {
        name: "lurk",
        description: "Cotillea desde las sombras",
        category: Category::Lurk,
        kind: Kind::Solo("está cotilleando desde las sombras"),
    },
    Interaction {
        name: "wag",
        description: "Mueve la colita",
        category: Category::Wag,
        kind: Kind::Solo("está moviendo la colita"),
    },
    Interaction {
        name: "nya",
        description: "Nya~",
        category: Category::Nya,
        kind: Kind::Solo("hizo nya~"),
    },
];
