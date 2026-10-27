use nekosbest::Category;

use super::{Interaction, Kind};

/// Affectionate interactions with someone else.
pub const CUTE: &[Interaction] = &[
    Interaction {
        name: "hug",
        description: "Abraza a alguien",
        category: Category::Hug,
        kind: Kind::Targeted("le dio un abrazo a"),
    },
    Interaction {
        name: "kiss",
        description: "Dale un beso a alguien",
        category: Category::Kiss,
        kind: Kind::Targeted("le dio un beso a"),
    },
    Interaction {
        name: "peck",
        description: "Dale un besito rápido a alguien",
        category: Category::Peck,
        kind: Kind::Targeted("le dio un besito a"),
    },
    Interaction {
        name: "blowkiss",
        description: "Lánzale un beso a alguien",
        category: Category::Blowkiss,
        kind: Kind::Targeted("le lanzó un beso a"),
    },
    Interaction {
        name: "pat",
        description: "Acaricia a alguien",
        category: Category::Pat,
        kind: Kind::Targeted("acarició a"),
    },
    Interaction {
        name: "cuddle",
        description: "Acurrúcate con alguien",
        category: Category::Cuddle,
        kind: Kind::Targeted("se acurrucó con"),
    },
    Interaction {
        name: "handhold",
        description: "Agarra la mano de alguien",
        category: Category::Handhold,
        kind: Kind::Targeted("le agarró la mano a"),
    },
    Interaction {
        name: "tickle",
        description: "Hazle cosquillas a alguien",
        category: Category::Tickle,
        kind: Kind::Targeted("le hizo cosquillas a"),
    },
    Interaction {
        name: "poke",
        description: "Pincha a alguien",
        category: Category::Poke,
        kind: Kind::Targeted("pinchó a"),
    },
    Interaction {
        name: "highfive",
        description: "Choca esos cinco con alguien",
        category: Category::Highfive,
        kind: Kind::Targeted("chocó los cinco con"),
    },
    Interaction {
        name: "handshake",
        description: "Dale la mano a alguien",
        category: Category::Handshake,
        kind: Kind::Targeted("le estrechó la mano a"),
    },
    Interaction {
        name: "feed",
        description: "Dale de comer a alguien",
        category: Category::Feed,
        kind: Kind::Targeted("le dio de comer a"),
    },
    Interaction {
        name: "carry",
        description: "Lleva a alguien en brazos",
        category: Category::Carry,
        kind: Kind::Targeted("cogió en brazos a"),
    },
    Interaction {
        name: "kabedon",
        description: "Acorrala a alguien contra la pared",
        category: Category::Kabedon,
        kind: Kind::Targeted("acorraló contra la pared a"),
    },
    Interaction {
        name: "lappillow",
        description: "Ofrécele tu regazo a alguien",
        category: Category::Lappillow,
        kind: Kind::Targeted("le ofreció su regazo a"),
    },
    Interaction {
        name: "wave",
        description: "Saluda a alguien",
        category: Category::Wave,
        kind: Kind::Targeted("saludó a"),
    },
    Interaction {
        name: "wink",
        description: "Guíñale un ojo a alguien",
        category: Category::Wink,
        kind: Kind::Targeted("le guiñó un ojo a"),
    },
    Interaction {
        name: "salute",
        description: "Haz un saludo militar a alguien",
        category: Category::Salute,
        kind: Kind::Targeted("le hizo un saludo militar a"),
    },
    Interaction {
        name: "thumbsup",
        description: "Levántale el pulgar a alguien",
        category: Category::ThumbsUp,
        kind: Kind::Targeted("le levantó el pulgar a"),
    },
];
