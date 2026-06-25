use smart_default::SmartDefault;
use smart_is_default::SmartIsDefault;

// Default behavior: helpers are `pub`, so they are reachable from sibling
// modules / sibling files within the same crate.

#[derive(SmartDefault, SmartIsDefault, PartialEq, Debug)]
pub struct PublicHelpers {
    #[default = 7]
    pub n: i32,

    #[default("x".to_string())]
    pub s: String,
}

// A *separate* module inside the same test binary. Reaching across the
// module boundary to call `PublicHelpers::is_default__n` would be a
// privacy error if the helpers were not `pub`. This compiles, so the
// default visibility is public.
mod consumer {
    use super::PublicHelpers;

    pub fn helper_is_true_for_default() -> bool {
        let def = PublicHelpers::default();
        PublicHelpers::is_default__n(&def.n) && PublicHelpers::is_default__s(&def.s)
    }

    pub fn helper_is_false_for_non_default() -> bool {
        let other = PublicHelpers {
            n: 0,
            s: "y".to_string(),
        };
        !PublicHelpers::is_default__n(&other.n) && !PublicHelpers::is_default__s(&other.s)
    }
}

#[test]
fn default_helpers_are_public_and_reachable_across_modules() {
    assert!(consumer::helper_is_true_for_default());
    assert!(consumer::helper_is_false_for_non_default());
}

// Opt-in: `no_pub` keeps helpers module-private. They are still
// callable from inside the module that defines the type, so the derive
// continues to work and the auto-generated `Serialize` impl (if any)
// can still call them too.

#[derive(SmartDefault, SmartIsDefault, PartialEq, Debug)]
#[smart_is_default(no_pub)]
struct PrivateHelpers {
    #[default = 3]
    n: i32,

    #[default = true]
    flag: bool,
}

#[test]
fn private_helpers_are_still_callable_within_their_own_module() {
    let def = PrivateHelpers::default();
    assert!(PrivateHelpers::is_default__n(&def.n));
    assert!(PrivateHelpers::is_default__flag(&def.flag));

    let other = PrivateHelpers { n: 0, flag: false };
    assert!(!PrivateHelpers::is_default__n(&other.n));
    assert!(!PrivateHelpers::is_default__flag(&other.flag));
}
