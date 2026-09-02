//! **The parameter→field flow**: which of a verb's scalar parameters
//! reaches which stored scalar field of which minted role family.
//!
//! # Why it lives beside the verb
//!
//! Only the operation knows where its parameters end up. A fillet's
//! radius IS the stored radius of every carrier it mints; a chamfer's
//! setback positions planes and is not stored anywhere. Nothing above
//! can derive that, and nothing below needs it — so the declaration
//! sits with the verb, as data.
//!
//! # What it is not, yet
//!
//! Plain data with NO consumer in this crate or any other. It is the
//! substrate for lowered parameter identity: a document layer that
//! knows a slot's expression address and reads this flow can attach an
//! opaque source token to exactly the fields the parameter reached, at
//! mint time. Until that lands, the acceptance for this declaration is
//! that it exists, is exhaustive over the vocabulary's scalar
//! parameters, and names only role families the birth record really
//! mints — which `tests/param_flow.rs` executes rather than asserts.
//!
//! # Scope: surface carriers
//!
//! The flow declares the stored fields of the SURFACE carriers a verb
//! mints, which is the granularity the per-field side records are keyed
//! at. Curve and point carriers minted by the same operation are out of
//! this declaration's scope; widening it is a change to what the
//! consumer keys on, not a change here alone.

use crate::verb::{Verb, VerbKind};

/// **A verb's scalar parameter, named.**
///
/// Closed over the whole migrated vocabulary rather than per verb, so
/// "every scalar parameter has a flow row" is a census a test can run
/// (`ScalarParam::ALL` against the union of the verbs' flows) instead
/// of a per-verb count copied into an assertion.
#[derive(Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum ScalarParam {
    /// [`Verb::Fillet`]'s `radius`.
    FilletRadius,
    /// [`Verb::Chamfer`]'s `distance`.
    ChamferDistance,
}

impl ScalarParam {
    /// Every scalar parameter in the vocabulary.
    pub const ALL: &'static [Self] = &[Self::FilletRadius, Self::ChamferDistance];

    /// Which verb the parameter belongs to.
    #[must_use]
    pub fn verb(self) -> VerbKind {
        match self {
            Self::FilletRadius => VerbKind::Fillet,
            Self::ChamferDistance => VerbKind::Chamfer,
        }
    }
}

/// **A family of entities a verb's birth record mints**, addressed by
/// the record row that carries it.
///
/// One variant per row of the record that a declared field can sit on.
/// The record has more rows than this — trimlines, feet, arcs, rim
/// trims, meridian fragments — and they are absent here because they
/// carry no surface field a parameter lands in, not because they are
/// forgotten.
#[derive(Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum RoleFamily {
    /// The blend faces, one per source edge (`BlendNaming::blends`).
    Blends,
    /// The corner faces, one per source vertex (`BlendNaming::corners`).
    Corners,
    /// The torus band faces, one per closed chain
    /// (`BlendNaming::bands`).
    Bands,
}

/// **One stored scalar field of one minted role family.**
///
/// Closed: a field a future verb stores a parameter in gets a variant,
/// and every match over this is visited.
#[derive(Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum FieldRole {
    /// The rolling ball's radius on a blend face's carrier — the
    /// cylinder's `radius` where the spine is straight, the torus's
    /// `minor_radius` where it is curved. One carrier, one field, two
    /// spellings of the same quantity.
    BlendCarrierRadius,
    /// The corner sphere's `radius`.
    CornerCarrierRadius,
    /// The rim band torus's `minor_radius`. Its `major_radius` is
    /// derived from the rim's own geometry and is NOT this field.
    BandCarrierMinorRadius,
}

impl FieldRole {
    /// The role family whose carriers hold this field.
    #[must_use]
    pub fn family(self) -> RoleFamily {
        match self {
            Self::BlendCarrierRadius => RoleFamily::Blends,
            Self::CornerCarrierRadius => RoleFamily::Corners,
            Self::BandCarrierMinorRadius => RoleFamily::Bands,
        }
    }
}

/// **Where one scalar parameter lands.**
///
/// An EMPTY `fields` is a statement, not an omission: it says the
/// parameter reaches no stored field of any minted carrier. Omitting
/// the row instead would be indistinguishable from forgetting it, which
/// is exactly the failure this declaration exists to make impossible.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct ParamFlow {
    /// The parameter.
    pub param: ScalarParam,
    /// The stored fields it reaches, in role order.
    pub fields: &'static [FieldRole],
}

/// The fillet's radius is the stored radius of every carrier the blend
/// mints: the blend faces', the corner spheres', and a closed chain's
/// torus minor radius.
const FILLET_FLOW: &[ParamFlow] = &[ParamFlow {
    param: ScalarParam::FilletRadius,
    fields: &[
        FieldRole::BlendCarrierRadius,
        FieldRole::CornerCarrierRadius,
        FieldRole::BandCarrierMinorRadius,
    ],
}];

/// The chamfer's setback POSITIONS its carriers and is stored in none
/// of them. Its arm gate admits plane–plane links only, so every face
/// it mints is a plane — a strip or a corner patch — and a plane's
/// stored data is a placement, not a distance: the setback moves the
/// origin and never becomes a field. The row is present and empty for
/// the reason [`ParamFlow`] gives.
const CHAMFER_FLOW: &[ParamFlow] = &[ParamFlow {
    param: ScalarParam::ChamferDistance,
    fields: &[],
}];

/// **The boolean has NO scalar parameters, so its flow has no rows —
/// and the empty slice is the declaration, not an omission.** The
/// payload is an operation selector and declared coincidence intents,
/// which are REFERENCES (arena keys), not scalars: nothing a document
/// slot evaluates ever lands in a field the boolean mints (its seam
/// geometry is derived from the operands, not parameterized). The
/// chamfer's row is empty at the `fields` level because its one scalar
/// reaches nothing; the boolean is empty one level up because there is
/// no scalar to write a row FOR — `ScalarParam` has no boolean
/// variant, which is what keeps the exhaustiveness census true.
/// `tests/param_flow.rs` asserts this emptiness beside a real boolean
/// birth record, so it is a statement about a run.
const BOOLEAN_FLOW: &[ParamFlow] = &[];

impl VerbKind {
    /// This verb's parameter→field flow, one row per scalar parameter.
    #[must_use]
    pub fn param_flow(self) -> &'static [ParamFlow] {
        match self {
            Self::Fillet => FILLET_FLOW,
            Self::Chamfer => CHAMFER_FLOW,
            Self::Boolean(_) => BOOLEAN_FLOW,
        }
    }
}

impl<T> Verb<T> {
    /// This verb's parameter→field flow ([`VerbKind::param_flow`]) —
    /// a function of the verb's NAME, not of its payload.
    #[must_use]
    pub fn param_flow(&self) -> &'static [ParamFlow] {
        self.kind().param_flow()
    }
}

#[cfg(test)]
mod all_census {
    use super::ScalarParam;

    /// **[`ScalarParam::ALL`] is every scalar parameter**, on the same
    /// compile-forced footing as `VerbKind::ALL`'s row and for a sharper
    /// reason: the flow suite's exhaustiveness test is computed OVER
    /// this list, so a parameter missing from it would make that test
    /// pass by not looking rather than by holding — the exact failure
    /// mode a census is supposed to remove.
    #[test]
    fn all_is_every_scalar_parameter() {
        let variants = match ScalarParam::FilletRadius {
            ScalarParam::FilletRadius => 2,
            ScalarParam::ChamferDistance => 2,
        };
        for (i, param) in ScalarParam::ALL.iter().enumerate() {
            assert!(
                !ScalarParam::ALL[..i].contains(param),
                "{param:?} appears twice in ScalarParam::ALL"
            );
        }
        assert_eq!(
            ScalarParam::ALL.len(),
            variants,
            "ScalarParam::ALL has drifted — the enum has {variants} variants"
        );
    }
}
