//! **The contact vocabulary** (CONTACT-DESIGN C3/C4) — the ONE place
//! the kernel names what a glued or recorded contact is, what a
//! verification decided, and what recourse a refusal offers.
//!
//! The vocabulary lives HERE, at the lowest crate that can hold it,
//! because the boolean's `UnrecordedContact`/`ContactContradicted`
//! refusals and the census's findings carry the class the pair was
//! read as (SELECT-DESIGN §3d, "one vocabulary end-to-end"), and the
//! recipe layer's flush detector mints findings in it.
//!
//! Upward layers RE-EXPORT these types; a parallel enum anywhere is
//! the bug this module exists to make impossible.

use geom_core::Indeterminate;

use crate::entity::FaceKey;

/// **What a boolean glues a cross-operand face pair as** (C4's
/// continuation clause and its claim-free `Tangent`): the class its
/// decided relation makes it ([`crate::Relation::glued_class`]).
/// Public as what a refusal's read of a glued pair names
/// ([`crate::DeclarationRead`]); nothing takes it as an input.
///
/// - [`Rest`](Self::Rest) — the pair lies on ONE carrier with its senses
///   OPPOSED: a conformal contact.
/// - [`Continuation`](Self::Continuation) — the pair lies on ONE
///   carrier with its senses ALIGNED: one surface carried on, whether
///   the two abut along a boundary curve or overlap on a patch; a union
///   merges it.
/// - [`Tangent`](Self::Tangent) — the pair lies on DISTINCT carriers
///   tangent along a locus the witness lane certifies. It says nothing
///   of the materials: whether the pair is a contact, an overlap, a seam
///   or a nested touch is read where it is consumed (C7).
///
/// It is also the class a contact pair names ([`ContactPair`]'s
/// `class`), which the façade's curated lists carry.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum GlueClass {
    /// One carrier, senses opposed.
    Rest,
    /// One carrier, senses aligned.
    Continuation,
    /// Distinct carriers tangent along a certified locus.
    Tangent,
}

impl GlueClass {
    /// Every class.
    #[cfg(test)]
    pub(crate) const ALL: &'static [GlueClass] = &[Self::Rest, Self::Continuation, Self::Tangent];

    /// The class's name, for messages.
    pub fn name(self) -> &'static str {
        match self {
            Self::Rest => "Rest",
            Self::Continuation => "Continuation",
            Self::Tangent => "Tangent",
        }
    }

    /// Whether the class is ONE carrier (`Rest` or a continuation) —
    /// what the classification stages' same-carrier treatment and the
    /// merge stage consume. A `Tangent` pair's carriers are distinct.
    pub fn is_one_carrier(self) -> bool {
        matches!(self, Self::Rest | Self::Continuation)
    }

    /// The relation that glues a face pair under this class: the
    /// inverse of [`crate::Relation::glued_class`] on the relations
    /// that glue.
    #[must_use]
    pub const fn relation(self) -> crate::Relation {
        match self {
            Self::Rest => crate::Relation::SameOpposite,
            Self::Continuation => crate::Relation::SameOriented,
            Self::Tangent => crate::Relation::SurfaceTangent,
        }
    }

    /// **The one-carrier class two faces' senses make them**, the one
    /// place a sense is read as a class: opposed senses are a `Rest`
    /// contact, aligned senses a continuation (C4). Two distinct
    /// carriers are neither, and name no class.
    pub(crate) fn of_senses(relation: crate::boolean::CarrierRelation) -> Option<Self> {
        match relation {
            crate::boolean::CarrierRelation::SameOriented => Some(Self::Continuation),
            crate::boolean::CarrierRelation::SameOpposite => Some(Self::Rest),
            crate::boolean::CarrierRelation::Distinct => None,
        }
    }

    /// The carrier relation a one-carrier class glues under: its
    /// [`Self::relation`] as the carrier ladder spells it, the inverse of
    /// [`Self::of_senses`]. `None` for a tangency, whose carriers are
    /// distinct.
    pub(crate) const fn carrier_relation(self) -> Option<crate::boolean::CarrierRelation> {
        match self.relation() {
            crate::Relation::SameOriented => Some(crate::boolean::CarrierRelation::SameOriented),
            crate::Relation::SameOpposite => Some(crate::boolean::CarrierRelation::SameOpposite),
            _ => None,
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

/// [`CONTACT_RECOURSE`] with the recourse marker, as a finding the
/// viewer shows ends on it (`contact_recourse_is_one_sentence` holds
/// the two spellings to one).
pub(crate) const CONTACT_RECOURSE_MARKED: &str =
    "Recourse: declare the named contact class, or move the geometry";

/// The recourse for a contradicted declaration. The declaration is
/// what the geometry refutes, so re-declaring it is no way through:
/// the declaration is corrected or removed, or the geometry is moved
/// until it holds.
pub const CONTRADICTION_RECOURSE: &str =
    "Recourse: correct or remove the declaration, or move the geometry so it holds";

/// A contradiction's reason, as the message states it where the site
/// carries no fact of its own (a site that does, the Boolean's `Rest`
/// verification, states its `boolean::Contradiction`). One sentence for
/// every such site, because it is the one that is true at all of them:
/// the sites carry different margins (a definite relation
/// at the carrier and tangent tables, an in-band coincidence margin at
/// the Boolean's conformal screen), and the same carrier predicate
/// means "apart" at one and "one surface" at another, so no reading of
/// the predicate is true everywhere. The margin and its predicate ride
/// in the payload and `Debug`.
pub const CONTRADICTION_REASON: &str = "the geometry does not match it";

/// **A contact verification's typed refusal**.
///
/// `Clone` and `PartialEq` because the refusal is CARRIED: the census
/// wraps it in
/// [`ValidationError::CensusUnsupported`](crate::ValidationError::CensusUnsupported)'s
/// cause, and that error is `Clone + PartialEq` so a consumer can hold
/// and compare a whole report. No `Eq` — the diagnostics carry `f64`
/// margins, which is why `ValidationError` has none either.
#[derive(Clone, Debug, PartialEq)]
// The variant roster the sample-coverage row reads (test builds only).
#[cfg_attr(
    test,
    derive(strum::EnumDiscriminants),
    strum_discriminants(name(ContactRefusalKind), vis(pub(crate)), derive(strum::EnumIter))
)]
pub enum ContactRefusal {
    /// Definite counter-evidence: the declaration is contradicted
    /// where the lie meets geometry. Every definite verdict wins over
    /// every declaration (C4's invariant).
    Contradicted {
        /// The contradicting predicate's diagnostics — the margin the
        /// message names.
        diag: Indeterminate,
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
            // The one reason true at every site, never the margin
            // payload, which would render a definite contradiction as
            // "indeterminate" (`CONTRADICTION_REASON`).
            Self::Contradicted { .. } => write!(
                f,
                "the declared contact is contradicted: {CONTRADICTION_REASON}. \
                 {CONTRADICTION_RECOURSE}"
            ),
            Self::Escalated { diag } => write!(
                f,
                "whether the declared faces touch escalated ({}), and no declaration can \
                 bridge it; {CONTACT_RECOURSE}",
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

/// **A face pair and the contact class it is read as**: what a record
/// names, what a finding reports, and what a refusal quotes back.
///
/// Key-based because it is the kernel's currency; the recipe layer's
/// name-based form carries the same class (G1: names cross the
/// boundary, keys never do).
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct ContactPair {
    /// The A-side face.
    pub a: FaceKey,
    /// The B-side face.
    pub b: FaceKey,
    /// The class the pair is read as.
    pub class: GlueClass,
}

/// **The kernel contact finding** — "this face pair would verify as
/// this class" (SELECT-DESIGN §3a: a VALUE, never itself a
/// declaration). A finding is only ever minted on a definite reading.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct ContactFinding {
    /// What was found.
    pub pair: ContactPair,
}

#[cfg(test)]
#[allow(clippy::unwrap_used, clippy::expect_used, clippy::panic)]
mod tests {
    use super::*;

    /// Each class's relation glues back as that class.
    #[test]
    fn a_glue_class_is_its_relations_class() {
        for &class in GlueClass::ALL {
            assert_eq!(class.relation().glued_class(), Some(class), "{class:?}");
        }
    }

    /// The two-arm menu is exactly two arms: §3d's ratified shape.
    /// The three-arm coincidence sentence must not leak in — its
    /// "lower the tolerance" arm cannot supply missing intent.
    #[test]
    fn recourse_has_two_arms_and_no_tolerance_lever() {
        assert!(CONTACT_RECOURSE.contains("declare"), "{CONTACT_RECOURSE}");
        assert_eq!(
            CONTACT_RECOURSE_MARKED,
            format!("Recourse: {CONTACT_RECOURSE}")
        );
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

    #[test]
    fn class_names_are_the_declaration_spelling() {
        assert_eq!(GlueClass::Continuation.name(), "Continuation");
        assert_eq!(GlueClass::Rest.name(), "Rest");
        assert_eq!(GlueClass::Tangent.name(), "Tangent");
    }
}
