//! **The contact vocabulary** (CONTACT-DESIGN C3/C4) — the ONE place
//! the kernel names what a declared contact asserts, what a
//! verification decided, and what recourse a refusal offers.
//!
//! The vocabulary lives HERE, at the lowest crate that can hold it,
//! because three consumers must speak the same words and no two of
//! them can depend on each other:
//!
//! - the boolean's `UndeclaredContact`/`ContactContradicted` refusals
//!   carry the class the detector produced (SELECT-DESIGN §3d, "one
//!   vocabulary end-to-end") and `topo` cannot depend on `editor-core`;
//! - the recipe layer's flush detector mints findings in it;
//! - assembly mate nodes (ASM R2-b) declare in it.
//!
//! Upward layers RE-EXPORT these types; a parallel enum anywhere is
//! the bug this module exists to make impossible.

use geom_core::Indeterminate;

use crate::entity::FaceKey;

/// **The contact class a declaration asserts** (C4).
///
/// The kernel builds and verifies two classes:
///
/// - [`Rest`](Self::Rest) — conformal: same carrier, opposed senses,
///   gap ≡ 0. Generalizes S1's planar declared-REST vocabulary to
///   every carrier kind ([`mod@crate::boolean::carrier_eq`]).
/// - [`Tangent`](Self::Tangent) — curve/point touch: opposed
///   tangency, non-crossing, verified through the jet schedule.
///
/// **`Fit { gap: T }` is RESERVED, not built** (C4/C6): a
/// carrier-parallel pair at signed nominal gap `g₀ ≠ 0`. Its payload
/// shape is pinned now so the deferral stays additive — kernel-side
/// the gap is a RESOLVED scalar (`T`), recipe-side an `Expr` that the
/// resolver lowers to it. Two consequences of that shape are recorded
/// here rather than discovered later: the enum LOSES `Copy` when the
/// variant lands (a `T` payload is `Clone`, not `Copy`, for the
/// interval backends), and it becomes generic in `T`, so every
/// signature holding a class today will grow the parameter. The
/// variant itself waits for its first consumer — see
/// [`FIT_DEFERRAL`].
///
/// `#[non_exhaustive]`: `Fit` lands additively.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[non_exhaustive]
pub enum ContactClass {
    /// Conformal contact: same carrier, opposed senses, gap ≡ 0.
    Rest,
    /// Curve or point touch: opposed tangency, non-crossing.
    Tangent,
}

impl ContactClass {
    /// **Every class this enum can name**, in declaration order — the
    /// one enumeration, owned where the exhaustive matches live.
    ///
    /// `#[non_exhaustive]` means no downstream crate can enumerate the
    /// variants itself: a hand-kept list out there compiles clean when
    /// a variant lands here and silently omits it (measured: a planted
    /// third variant failed this crate at its designed fences and left
    /// a downstream `[Rest, Tangent]` literal green). This slice is
    /// the fix at the source — it sits beside [`ContactClass::name`]
    /// and [`ContactClass::content_tag`], whose exhaustive matches are
    /// the compile-time fence that forces the new variant through this
    /// impl block, where this list is the first thing in it.
    pub const ALL: &'static [ContactClass] = &[ContactClass::Rest, ContactClass::Tangent];

    /// The class's name, for messages.
    pub fn name(self) -> &'static str {
        match self {
            Self::Rest => "Rest",
            Self::Tangent => "Tangent",
        }
    }

    /// A **stable, distinct** tag for content-addressing (the recipe
    /// layer's memo keys).
    ///
    /// Lives here, not at the consumer, for one reason: the match is
    /// exhaustive only inside this crate. `ContactClass` is
    /// `#[non_exhaustive]`, so a downstream tag map needs a wildcard
    /// arm — and a wildcard in a content-key feed is a silent
    /// collision, two classes hashing alike. Written here, a new
    /// variant is a compile error at the one place that must decide
    /// its tag.
    ///
    /// Hand-assigned rather than a derived discriminant so the tags
    /// survive variant REORDERING: inserting `Fit` between the two
    /// must not re-key every existing declaration. Tags are
    /// process-internal (never persisted — the wire spelling is the
    /// recipe layer's own, and is a string), so growth is free, but an
    /// existing tag must never be reused for a new meaning.
    pub fn content_tag(self) -> u64 {
        match self {
            Self::Rest => 1,
            Self::Tangent => 2,
        }
    }
}

/// **The two-arm recourse menu** for a contact refusal (SELECT-DESIGN
/// §3d, ratified; C4's failure table verbatim): declare the named
/// class, or move the geometry.
///
/// This is deliberately NOT `geom_core::COINCIDENCE_RECOURSE`. That
/// sentence's third arm ("lower the tolerance") predates §3d's
/// ratification and is wrong at a contact site: a contact refusal is
/// about intent that was never recorded, and loosening ε cannot
/// supply intent — it can only hide the absence. The two-tolerance
/// principle keeps the three-arm sentence at every site whose
/// question really is "is this margin decidable"; contact sites ask
/// "did anyone declare this", and answer with these two arms.
pub const CONTACT_RECOURSE: &str = "declare the named contact class, or move the geometry";

/// **The `Fit` deferral, named** (AQ6): the recourse sentence for a
/// declared contact whose carriers are value-equal by authoring but
/// whose geometry carries a designed clearance.
///
/// A dead pointer ("use `Fit`") would be worse than silence, so the
/// sentence names the deferral itself: the variant is specified
/// (`Fit { gap }`, C6) and lands with its first consumer.
pub const FIT_DEFERRAL: &str = "a designed nonzero clearance is `Fit { gap }`, whose variant is \
     specified but not yet built — it lands with its first consumer";

/// **The trilean a contact verification returns** (C4's per-class
/// tables, AQ6's shape): every declaration states three lists, and
/// this is the verdict that says which one the geometry landed in.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ContactVerdict {
    /// Every must-verify-DEFINITE condition is definitely satisfied:
    /// the contact holds on the geometry's own evidence, and the
    /// declaration added nothing.
    Definite,
    /// The definite conditions hold and the residual margins landed
    /// in band — the declaration BRIDGES exactly this residue and
    /// nothing else (C4's invariant). A bridged verdict is only ever
    /// reachable with a declaration in hand.
    Bridged,
}

/// **A contact verification's typed refusal** — the two failing arms
/// of the trilean, kept off [`ContactVerdict`] so a caller cannot
/// pattern-match a refusal into a pass.
#[derive(Debug)]
pub enum ContactRefusal {
    /// Definite counter-evidence: the declaration is contradicted
    /// where the lie meets geometry. Every definite verdict wins over
    /// every declaration (C4's invariant).
    Contradicted {
        /// The contradicting predicate's diagnostics — the margin the
        /// message names.
        diag: Indeterminate,
        /// Extra recourse steering for this contradiction, when the
        /// shape of the counter-evidence has a named remedy (AQ6's
        /// designed-clearance arm points at [`FIT_DEFERRAL`]).
        steer: Option<&'static str>,
    },
    /// In-band geometry with no declaration to bridge it, or an
    /// escalation inside a definite condition: terminal, priced,
    /// honest.
    Escalated {
        /// The escalating predicate's diagnostics.
        diag: Indeterminate,
    },
    /// Geometry that touches with no backing declaration at all:
    /// near-coincidence NEVER silently becomes contact (F6).
    Undeclared {
        /// The site's diagnostics.
        diag: Indeterminate,
    },
    /// The configuration is outside the class's certifiable set — the
    /// demanded set IS the certifiable set (C3's order-k boundary),
    /// so this refuses typed rather than sampling. Carries what was
    /// asked for, never a fallback verdict.
    NotCertifiable {
        /// Why the configuration is outside the lane.
        what: &'static str,
    },
}

/// **A contact CLAIM**: this face pair, asserted to be in contact of
/// this class. What a declaration says, and what a refusal quotes back.
///
/// Key-based because it is the kernel's currency; the recipe layer's
/// name-based form carries the same class (G1: names cross the
/// boundary, keys never do).
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct DeclaredContact {
    /// The A-side face.
    pub a: FaceKey,
    /// The B-side face.
    pub b: FaceKey,
    /// The class the pair is claimed to be in contact of.
    pub class: ContactClass,
}

/// **The kernel contact finding** — "this face pair would verify as
/// this class, on this evidence" (SELECT-DESIGN §3a: a VALUE, never
/// itself a declaration).
///
/// A finding is a [`DeclaredContact`] PLUS the verdict that decided
/// it, by composition rather than by repeating the fields: the claim
/// and the evidence for it are different things, and a refusal carries
/// only the claim. Keeping the verdict off the claim is what stops the
/// pun the two failure gates would otherwise make — a contradiction
/// has no verification verdict to report, and it must not borrow
/// `Definite` to mean "the counter-evidence was definite".
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct ContactFinding {
    /// What was claimed.
    pub pair: DeclaredContact,
    /// The verdict the verify door reported. A finding is only ever
    /// minted on [`ContactVerdict::Definite`] — a bridged pair needs a
    /// declaration, so it cannot be evidence FOR one.
    pub verdict: ContactVerdict,
}

#[cfg(test)]
#[allow(clippy::unwrap_used, clippy::expect_used, clippy::panic)]
mod tests {
    use super::*;

    /// The two-arm menu is exactly two arms: §3d's ratified shape.
    /// The three-arm coincidence sentence must not leak in — its
    /// "lower the tolerance" arm cannot supply missing intent.
    #[test]
    fn recourse_has_two_arms_and_no_tolerance_lever() {
        assert!(CONTACT_RECOURSE.contains("declare"), "{CONTACT_RECOURSE}");
        assert!(CONTACT_RECOURSE.contains("move the geometry"));
        assert!(
            !CONTACT_RECOURSE.contains("tolerance"),
            "the #256 ruling: a contact refusal has no tolerance arm"
        );
        assert!(
            !geom_core::COINCIDENCE_RECOURSE.contains("contact class"),
            "the three-arm sentence stays the DECIDABILITY sentence"
        );
    }

    /// The deferral is NAMED, not a dead pointer: the sentence says
    /// both what the variant is and that it does not exist yet.
    #[test]
    fn fit_deferral_names_the_variant_and_the_deferral() {
        assert!(FIT_DEFERRAL.contains("Fit { gap }"), "{FIT_DEFERRAL}");
        assert!(FIT_DEFERRAL.contains("not yet built"), "{FIT_DEFERRAL}");
    }

    #[test]
    fn class_names_are_the_declaration_spelling() {
        assert_eq!(ContactClass::Rest.name(), "Rest");
        assert_eq!(ContactClass::Tangent.name(), "Tangent");
    }
}
