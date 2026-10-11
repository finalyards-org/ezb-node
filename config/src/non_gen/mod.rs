#![cfg(not(feature = "gen"))]

#[allow(dead_code)]
pub(crate) trait InvariantCheck {
    // For non-gen, only a single node type is enabled.
    const ROLE: &'static str =
        if cfg!(feature = "coordinator") { stringify!(coordinator) }
        else if cfg!(feature = "router") { stringify!(router) }
        //else if cfg!(feature = "end_device_NEVER_MIND") { stringify!(end_device) }
        else { unreachable!() };

    /// A documentary (not necessarily enforced) way to declare invariants. The 'gen' part should
    /// provide output that _always_ passes such checks.
    ///
    /// Note: If an invariant is declared for a struct, it must *cover all* potentially invalid
    ///     conditions.
    fn invariant_check(&self);
}
