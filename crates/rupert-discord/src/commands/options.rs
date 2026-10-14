use serenity::all::{ChannelId, PartialMember, ResolvedOption, ResolvedValue, Role, User};

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

/// The user and, if they are in the server, their member data.
pub fn get_member(options: &[ResolvedOption], name: &str) -> Option<(User, Option<PartialMember>)> {
    options.iter().find_map(|opt| match opt {
        ResolvedOption {
            name: n,
            value: ResolvedValue::User(u, m),
            ..
        } if *n == name => Some(((**u).clone(), m.cloned())),
        _ => None,
    })
}

pub fn get_role(options: &[ResolvedOption], name: &str) -> Option<Role> {
    options.iter().find_map(|opt| match opt {
        ResolvedOption {
            name: n,
            value: ResolvedValue::Role(r),
            ..
        } if *n == name => Some((**r).clone()),
        _ => None,
    })
}
