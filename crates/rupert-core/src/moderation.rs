use crate::error::HierarchyError;

#[derive(Debug, Clone, Copy)]
pub struct Actor {
    pub id: u64,
    pub top_role: u16,
    pub is_owner: bool,
}

/// Checks whether `executor` may act on `target` through the bot.
///
/// The executor must be strictly above the target, unless they own the server.
pub fn check_member(executor: &Actor, target: &Actor, bot: &Actor) -> Result<(), HierarchyError> {
    if executor.id == target.id {
        return Err(HierarchyError::SelfTarget);
    }
    if target.id == bot.id {
        return Err(HierarchyError::TargetIsBot);
    }
    if target.is_owner {
        return Err(HierarchyError::TargetIsOwner);
    }
    if !executor.is_owner && executor.top_role <= target.top_role {
        return Err(HierarchyError::TargetTooHigh);
    }
    if bot.top_role <= target.top_role {
        return Err(HierarchyError::BotTooLow);
    }
    Ok(())
}

/// Checks whether `executor` may give or take a role at `position` through the bot.
pub fn check_role(executor: &Actor, position: u16, bot: &Actor) -> Result<(), HierarchyError> {
    if !executor.is_owner && executor.top_role <= position {
        return Err(HierarchyError::RoleTooHigh);
    }
    if bot.top_role <= position {
        return Err(HierarchyError::BotRoleTooLow);
    }
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    const BOT: Actor = actor(1, 50);
    const OWNER: Actor = Actor {
        id: 2,
        top_role: 0,
        is_owner: true,
    };

    const fn actor(id: u64, top_role: u16) -> Actor {
        Actor {
            id,
            top_role,
            is_owner: false,
        }
    }

    #[test]
    fn higher_can_act_on_lower() {
        assert_eq!(check_member(&actor(10, 20), &actor(11, 10), &BOT), Ok(()));
    }

    #[test]
    fn equal_roles_cannot_act_on_each_other() {
        assert_eq!(
            check_member(&actor(10, 20), &actor(11, 20), &BOT),
            Err(HierarchyError::TargetTooHigh)
        );
    }

    #[test]
    fn lower_cannot_act_on_higher() {
        assert_eq!(
            check_member(&actor(10, 10), &actor(11, 20), &BOT),
            Err(HierarchyError::TargetTooHigh)
        );
    }

    #[test]
    fn nobody_acts_on_themselves() {
        assert_eq!(
            check_member(&actor(10, 20), &actor(10, 20), &BOT),
            Err(HierarchyError::SelfTarget)
        );
        assert_eq!(
            check_member(&OWNER, &OWNER, &BOT),
            Err(HierarchyError::SelfTarget)
        );
    }

    #[test]
    fn nobody_acts_on_the_bot() {
        assert_eq!(
            check_member(&actor(10, 99), &BOT, &BOT),
            Err(HierarchyError::TargetIsBot)
        );
        assert_eq!(
            check_member(&OWNER, &BOT, &BOT),
            Err(HierarchyError::TargetIsBot)
        );
    }

    #[test]
    fn nobody_acts_on_the_owner() {
        assert_eq!(
            check_member(&actor(10, 99), &OWNER, &BOT),
            Err(HierarchyError::TargetIsOwner)
        );
    }

    #[test]
    fn owner_acts_on_anyone_below_the_bot() {
        assert_eq!(check_member(&OWNER, &actor(10, 49), &BOT), Ok(()));
        assert_eq!(
            check_member(&OWNER, &actor(10, 50), &BOT),
            Err(HierarchyError::BotTooLow)
        );
    }

    #[test]
    fn bot_must_be_above_target() {
        assert_eq!(
            check_member(&actor(10, 90), &actor(11, 60), &BOT),
            Err(HierarchyError::BotTooLow)
        );
    }

    #[test]
    fn members_without_roles_cannot_act() {
        assert_eq!(
            check_member(&actor(10, 0), &actor(11, 0), &BOT),
            Err(HierarchyError::TargetTooHigh)
        );
    }

    #[test]
    fn roles_below_own_can_be_managed() {
        assert_eq!(check_role(&actor(10, 20), 19, &BOT), Ok(()));
    }

    #[test]
    fn own_or_higher_roles_cannot_be_managed() {
        assert_eq!(
            check_role(&actor(10, 20), 20, &BOT),
            Err(HierarchyError::RoleTooHigh)
        );
        assert_eq!(
            check_role(&actor(10, 20), 30, &BOT),
            Err(HierarchyError::RoleTooHigh)
        );
    }

    #[test]
    fn owner_manages_roles_below_the_bot() {
        assert_eq!(check_role(&OWNER, 49, &BOT), Ok(()));
        assert_eq!(
            check_role(&OWNER, 50, &BOT),
            Err(HierarchyError::BotRoleTooLow)
        );
    }
}
