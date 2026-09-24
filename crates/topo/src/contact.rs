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

/// What a person at the viewer reads for [`FIT_DEFERRAL`]. The constant
/// itself is the wire's and the mate door's sentence (quoted verbatim
/// by `editor-core`), so it cannot change; a rendered contradiction
/// that carries it as its steer says this instead, in the user's terms
/// rather than as a variant name and a roadmap note.
pub const FIT_DEFERRAL_FOR_USERS: &str = "a designed gap between the faces cannot be declared yet";

/// The recourse for a contradicted declaration. The declaration is
/// what the geometry refutes, so re-declaring it is no way through:
/// the declaration is corrected or removed, or the geometry is moved
/// until it holds.
pub const CONTRADICTION_RECOURSE: &str =
    "Recourse: correct or remove the declaration, or move the geometry so it holds";

/// A contradiction's reason in words, read off the predicate that
/// decided it. Every contradiction site carries an invalid margin
/// standing for a definite relation, so the predicate IS the reason;
/// its name, and the margin, ride in `Debug`.
pub(crate) fn contradiction_reason(diag: &Indeterminate) -> &'static str {
    match diag.predicate {
        Some("contact_rest_senses_opposed") => {
            "the two faces face the same way, so neither rests against the other"
        }
        Some("contact_tangent_opposed") => {
            "the two faces face the same way along the edge, so they cannot touch from \
             opposite sides"
        }
        Some("contact_tangent_independent") => "the faces cross at the edge rather than touch",
        Some("contact_tangent_parallel") => "the faces are not tangent along the edge",
        Some("contact_tangent_on_1" | "contact_tangent_on_2") => {
            "the edge does not lie on both faces"
        }
        Some("carrier_kind") => "the two faces lie on different kinds of surface",
        Some("bool_plane_offset") => "the two faces' planes are parallel but apart",
        Some("bool_plane_parallel") => "the two faces' planes are not parallel",
        Some("bool_plane_orient") => "the two faces face the same way",
        Some(p) if p.starts_with("carrier_") => "the two faces lie on different surfaces",
        _ => "the geometry definitely disagrees with it",
    }
}

/// A steer as the rendered contradiction says it: `FIT_DEFERRAL` in the
/// user's words, any other steer as written.
pub(crate) fn steer_clause(steer: Option<&'static str>) -> String {
    match steer {
        Some(s) if s == FIT_DEFERRAL => format!("; {FIT_DEFERRAL_FOR_USERS}"),
        Some(s) => format!("; {s}"),
        None => String::new(),
    }
}

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
///
/// `Clone` and `PartialEq` because the refusal is CARRIED: the census
/// wraps it in
/// [`ValidationError::CensusUnsupported`](crate::ValidationError::CensusUnsupported)'s
/// cause, and that error is `Clone + PartialEq` so a consumer can hold
/// and compare a whole report. No `Eq` — the diagnostics carry `f64`
/// margins, which is why `ValidationError` has none either.
#[derive(Clone, Debug, PartialEq)]
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

// The human-readable rendering (LIB-DOORS F6 shape): each arm states
// the PROBLEM in the contact vocabulary — declaration, definite
// counter-evidence, in-band residue, the certifiable set — plus the
// recourse where an author has one.
//
// The diagnostics render through `Indeterminate::payload`, never the
// bare `Indeterminate` Display: the bare one ends in
// `COINCIDENCE_RECOURSE`, whose "lower the tolerance" arm is wrong at
// a contact site, and these arms supply [`CONTACT_RECOURSE`]
// themselves. That is the same composition the tier-3′ census and the
// boolean's own refusals make, so one contact story is told in one
// sentence shape wherever it surfaces.
//
// [`Self::NotCertifiable`] carries no recourse: a declaration cannot
// move a configuration inside the certifiable set, so the two-arm menu
// would be a false lead, and `what` is the only honest steering there
// is.
impl core::fmt::Display for ContactRefusal {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        match self {
            // The reason in words, never the margin payload: every
            // contradiction carries an invalid margin standing for a
            // definite relation, which the payload would render as
            // "indeterminate".
            Self::Contradicted { diag, steer } => write!(
                f,
                "the declared contact is contradicted: {}. {CONTRADICTION_RECOURSE}{}",
                contradiction_reason(diag),
                steer_clause(*steer),
            ),
            Self::Escalated { diag } => write!(
                f,
                "whether the declared faces touch is too close to call ({}), and no \
                 declaration can decide it; {CONTACT_RECOURSE}",
                diag.payload()
            ),
            Self::Undeclared { diag } => write!(
                f,
                "the faces touch on the geometry's own evidence ({}) with no declaration \
                 behind them — near-coincidence never silently becomes contact; \
                 {CONTACT_RECOURSE}",
                diag.payload()
            ),
            Self::NotCertifiable { what } => write!(
                f,
                "this contact cannot be certified ({what}), and a declaration cannot move a \
                 configuration into the certifiable set"
            ),
        }
    }
}

impl std::error::Error for ContactRefusal {}

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
