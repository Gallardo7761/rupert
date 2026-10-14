use serenity::all::{ChannelId, ResolvedOption, ResolvedValue, User};

pub fn get_string(options: &[ResolvedOption], name: &str) -> Option<String> {
    options.iter().find_map(|opt| match opt {
        ResolvedOption {
            name: n,
            value: ResolvedValue::String(s),
            ..
        } if *n == name => Some(s.to_string()),
        _ => None,
    })
}

pub fn get_integer(options: &[ResolvedOption], name: &str) -> Option<i64> {
    options.iter().find_map(|opt| match opt {
        ResolvedOption {
            name: n,
            value: ResolvedValue::Integer(i),
            ..
        } if *n == name => Some(*i),
        _ => None,
    })
}

pub fn get_user(options: &[ResolvedOption], name: &str) -> Option<User> {
    options.iter().find_map(|opt| match opt {
        ResolvedOption {
            name: n,
            value: ResolvedValue::User(u, _),
            ..
        } if *n == name => Some((**u).clone()),
        _ => None,
    })
}

pub fn get_channel(options: &[ResolvedOption], name: &str) -> Option<ChannelId> {
    options.iter().find_map(|opt| match opt {
        ResolvedOption {
            name: n,
            value: ResolvedValue::Channel(c),
            ..
        } if *n == name => Some(c.id),
        _ => None,
    })
}

pub fn get_bool(options: &[ResolvedOption], name: &str) -> Option<bool> {
    options.iter().find_map(|opt| match opt {
        ResolvedOption {
            name: n,
            value: ResolvedValue::Boolean(b),
            ..
        } if *n == name => Some(*b),
        _ => None,
    })
}
