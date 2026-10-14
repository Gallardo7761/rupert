use serenity::all::{CommandInteraction, Permissions};

pub fn has_permission(cmd: &CommandInteraction, required: Permissions) -> bool {
    cmd.member
        .as_ref()
        .and_then(|m| m.permissions)
        .is_some_and(|p| p.contains(required))
}
