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
//! # Who consumes it
//!
//! `editor-core`'s lowering, at mint time — one door per source kind
//! (`param_source::attach_blend` for a verb's own scalar,
//! `param_source::attach_swept` for a scalar the operand profile
//! carries per edge). The document layer knows the expression, reads
//! this flow for the source that expression is, and attaches the
//! lowered token to exactly the fields the flow says it reached. The
//! declaration itself stays plain data in this crate — no consumer
//! here — and its own acceptance is that it is exhaustive over the
//! vocabulary's sources and names only role families the birth record
//! really mints, which `tests/param_flow.rs` executes rather than
//! asserts.
//!
//! # Where a value can COME from
//!
//! A verb's own scalar parameter is one source, and it was the only
//! one while every migrated verb had one. A sweep's is not: extruding
//! a circle mints a cylinder whose stored radius is the PROFILE
//! circle's radius, and no parameter of the extrude says anything
//! about it — the extrude's one scalar is a distance that reaches no
//! stored field at all. So a flow row names its source
//! ([`FlowSource`]), closed and exhaustive like everything else here,
//! and the two kinds differ in exactly one thing above: which
//! expression the consumer lowers. A verb scalar's expression is the
//! node's own slot; a profile edge's is the operand profile's slot,
//! at the address the profile layer holds it under. Both are document
//! vocabulary and neither is named here.
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
    /// [`Verb::Extrude`]'s `distance`.
    ExtrudeDistance,
    /// [`Verb::Revolve`]'s angle — the scalar a `Revolution::Partial`
    /// carries. `Revolution::Full` carries none, and the name still
    /// covers it: the flow is keyed on the parameter's NAME, and the
    /// row this name owns is empty for either spelling (an angle is an
    /// extent and reaches no stored field), so a full revolve has
    /// nothing to declare rather than a row that does not apply.
    RevolveAngle,
}

impl ScalarParam {
    /// Every scalar parameter in the vocabulary.
    pub const ALL: &'static [Self] = &[
        Self::FilletRadius,
        Self::ChamferDistance,
        Self::ExtrudeDistance,
        Self::RevolveAngle,
    ];

    /// Which verb the parameter belongs to.
    #[must_use]
    pub fn verb(self) -> VerbKind {
        match self {
            Self::FilletRadius => VerbKind::Fillet,
            Self::ChamferDistance => VerbKind::Chamfer,
            Self::ExtrudeDistance => VerbKind::Extrude,
            Self::RevolveAngle => VerbKind::Revolve,
        }
    }
}

/// **A scalar an OPERAND carries, per entity** — the second kind of
/// place a stored field's value can come from.
///
/// Closed, and one variant today because one is what the migrated
/// vocabulary reaches: a profile edge's carrier radius. The VALUE is
/// per edge — the wall swept from an edge stores that edge's radius
/// and no other edge's, which is what makes the flow honourable per
/// minted wall — but the ADDRESS the document holds it at need not be
/// that fine, and today it is not: a carrier loop is drawn at one
/// radius, so every edge it replays to reads the same expression at
/// one per-LOOP slot. A per-edge address is what a loop form with more
/// than one radius would need, and no consumer has one.
#[derive(Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum EdgeScalar {
    /// The radius of a CIRCULAR profile edge — the arc's carrier
    /// radius. A straight edge has none, and that is an answer, not a
    /// gap: the wall it sweeps stores no radius of the profile's (an
    /// extruded line is a plane; a revolved line's cylinder radius is
    /// its distance from the axis, which is derived from authored
    /// points rather than held as a scalar anywhere).
    Radius,
}

/// **Where the value that lands in a stored field came from.**
///
/// Closed with no wildcard arm (D3): a source kind added here is a
/// compile-time visit at every consumer, because what a consumer must
/// do differs per kind — a verb scalar's expression is read off the
/// verb's own node, an operand-carried one off the operand's.
#[derive(Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum FlowSource {
    /// One of the verb's own scalar parameters.
    Param(ScalarParam),
    /// A scalar the operand PROFILE carries per edge, landing in the
    /// field of the wall swept from that edge.
    ProfileEdge(EdgeScalar),
}

impl FlowSource {
    /// Every source in the vocabulary — the census the flow suite
    /// runs over, on the same footing as [`ScalarParam::ALL`].
    pub const ALL: &'static [Self] = &[
        Self::Param(ScalarParam::FilletRadius),
        Self::Param(ScalarParam::ChamferDistance),
        Self::Param(ScalarParam::ExtrudeDistance),
        Self::Param(ScalarParam::RevolveAngle),
        Self::ProfileEdge(EdgeScalar::Radius),
    ];
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
    /// The wall faces a sweep mints, per profile loop, per profile
    /// edge (`Extruded::side_faces`, `Revolved::walls`). The family
    /// whose rows are addressed BY THE OPERAND's entities rather than
    /// by the verb's — which is what lets an operand-carried source be
    /// honoured at all. The record groups the rows by LOOP, and that
    /// is the grouping the flow's consumer keys on, because the
    /// address the document holds a carrier radius at is per loop
    /// ([`EdgeScalar`]).
    SweptWalls,
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
    /// The radius a SWEPT WALL stores when the profile edge it was
    /// swept from is circular — the cylinder's `radius` for an
    /// extrusion, the torus's `minor_radius` for a revolution. One
    /// quantity, two spellings, exactly like
    /// [`FieldRole::BlendCarrierRadius`]: the wall is the circular
    /// edge dragged along the sweep, so its cross-sectional radius IS
    /// the edge's radius whichever surface kind the drag produces. A
    /// torus's `major_radius` is the distance from the axis and is
    /// NOT this field.
    SweptWallRadius,
}

impl FieldRole {
    /// The role family whose carriers hold this field.
    #[must_use]
    pub fn family(self) -> RoleFamily {
        match self {
            Self::BlendCarrierRadius => RoleFamily::Blends,
            Self::CornerCarrierRadius => RoleFamily::Corners,
            Self::BandCarrierMinorRadius => RoleFamily::Bands,
            Self::SweptWallRadius => RoleFamily::SweptWalls,
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
    /// Where the value comes from.
    pub source: FlowSource,
    /// The stored fields it reaches, in role order.
    pub fields: &'static [FieldRole],
}

/// The fillet's radius is the stored radius of every carrier the blend
/// mints: the blend faces', the corner spheres', and a closed chain's
/// torus minor radius.
const FILLET_FLOW: &[ParamFlow] = &[ParamFlow {
    source: FlowSource::Param(ScalarParam::FilletRadius),
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
    source: FlowSource::Param(ScalarParam::ChamferDistance),
    fields: &[],
}];

/// **The extrude's two rows: an extent that reaches nothing, and the
/// profile's own radius that reaches every circular wall.**
///
/// The DISTANCE is an extent. It positions the top cap and sets how
/// far the walls run, and neither is a stored scalar: a plane's data
/// is a placement, a cylinder's stored `radius` is the profile arc's,
/// and how far a wall runs lives in the topology's parameter
/// intervals, not in a surface field. The row is present and empty for
/// the reason [`ParamFlow`] gives — the chamfer setback's precedent
/// exactly.
///
/// The PROFILE EDGE's radius is the other half and the one that is
/// new: dragging a circular edge along the normal mints
/// `Surface::Cylinder { radius }` carrying the arc's own radius, so
/// the wall's stored radius IS the profile's declared one. That is the
/// row the equal-radius germ reads.
const EXTRUDE_FLOW: &[ParamFlow] = &[
    ParamFlow {
        source: FlowSource::Param(ScalarParam::ExtrudeDistance),
        fields: &[],
    },
    ParamFlow {
        source: FlowSource::ProfileEdge(EdgeScalar::Radius),
        fields: &[FieldRole::SweptWallRadius],
    },
];

/// **The revolve's two rows**, the same shape and the same reasons.
///
/// The ANGLE is an extent, and more plainly than the extrude's: it is
/// how far around the sweep runs, which lands in the wedge caps'
/// placements and the walls' parameter intervals. No surface stores an
/// angle.
///
/// The PROFILE EDGE's radius reaches the walls here too. Revolving a
/// circular edge about a coplanar axis clear of it mints
/// `Surface::Torus { minor_radius }` carrying the arc's own radius (an
/// on-axis arc centre mints `Surface::Sphere { radius }` with the same
/// number; a closed carrier loop cannot reach that case, since a
/// centre on the axis puts the loop across it and the door refuses).
///
/// What is NOT here, MEASURED: a revolved straight edge's cylinder
/// radius. That number is the edge's distance from the axis — derived
/// from two authored 2-D points and the axis's own origin and
/// direction, across three document nodes — and the profile layer
/// holds no slot for it. There is no address to lower, so there is no
/// row; inventing one would mean minting an expression the document
/// does not hold.
const REVOLVE_FLOW: &[ParamFlow] = &[
    ParamFlow {
        source: FlowSource::Param(ScalarParam::RevolveAngle),
        fields: &[],
    },
    ParamFlow {
        source: FlowSource::ProfileEdge(EdgeScalar::Radius),
        fields: &[FieldRole::SweptWallRadius],
    },
];

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

/// **The split has NO scalar parameters either, and its empty flow
/// says so for a reason of its own.** The payload is a plane — a
/// point and a unit normal, a DATUM value read off a datum node
/// upstairs — and a placement is not a scalar: no document slot
/// evaluates to a number that the split carries into anything it
/// mints. What it mints is section faces, and a section face's
/// carrier is a plane, whose stored data is a placement and nothing
/// else — `SurfaceField::belongs_to` names no field for the plane
/// kind — so even a scalar that reached one would have no field to
/// land in. `ScalarParam` gains no split variant, which is what keeps
/// the exhaustiveness census true; `tests/param_flow.rs` asserts this
/// emptiness beside a real split record, so it is a statement about a
/// run and not an untested constant.
const SPLIT_FLOW: &[ParamFlow] = &[];

impl VerbKind {
    /// This verb's parameter→field flow, one row per scalar parameter.
    #[must_use]
    pub fn param_flow(self) -> &'static [ParamFlow] {
        match self {
            Self::Fillet => FILLET_FLOW,
            Self::Chamfer => CHAMFER_FLOW,
            Self::Extrude => EXTRUDE_FLOW,
            Self::Revolve => REVOLVE_FLOW,
            Self::Boolean(_) => BOOLEAN_FLOW,
            Self::Split => SPLIT_FLOW,
        }
    }
}

impl<T: geom_core::Real> Verb<T> {
    /// This verb's parameter→field flow ([`VerbKind::param_flow`]) —
    /// a function of the verb's NAME, not of its payload.
    #[must_use]
    pub fn param_flow(&self) -> &'static [ParamFlow] {
        self.kind().param_flow()
    }
}

#[cfg(test)]
mod all_census {
    use super::{EdgeScalar, FlowSource, ScalarParam};

    /// **[`ScalarParam::ALL`] is every scalar parameter**, on the same
    /// compile-forced footing as `VerbKind::ALL`'s row and for a sharper
    /// reason: the flow suite's exhaustiveness test is computed OVER
    /// this list, so a parameter missing from it would make that test
    /// pass by not looking rather than by holding — the exact failure
    /// mode a census is supposed to remove.
    #[test]
    fn all_is_every_scalar_parameter() {
        let variants = match ScalarParam::FilletRadius {
            ScalarParam::FilletRadius => 4,
            ScalarParam::ChamferDistance => 4,
            ScalarParam::ExtrudeDistance => 4,
            ScalarParam::RevolveAngle => 4,
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
            "ScalarParam::ALL has drifted — it holds {} entries, the enum has {variants} variants",
            ScalarParam::ALL.len()
        );
    }

    /// **[`FlowSource::ALL`] is every source**, by the same
    /// compile-forced construction: the match below is exhaustive over
    /// the source vocabulary AND over each kind's own payload, so a
    /// variant added to any of the three enums fails this file until
    /// the count is rewritten, which then reds until `ALL` has grown.
    #[test]
    fn all_is_every_flow_source() {
        let rows = match FlowSource::Param(ScalarParam::FilletRadius) {
            FlowSource::Param(param) => match param {
                ScalarParam::FilletRadius
                | ScalarParam::ChamferDistance
                | ScalarParam::ExtrudeDistance
                | ScalarParam::RevolveAngle => 5,
            },
            FlowSource::ProfileEdge(edge) => match edge {
                EdgeScalar::Radius => 5,
            },
        };
        for (i, source) in FlowSource::ALL.iter().enumerate() {
            assert!(
                !FlowSource::ALL[..i].contains(source),
                "{source:?} appears twice in FlowSource::ALL"
            );
        }
        assert_eq!(
            FlowSource::ALL.len(),
            rows,
            "FlowSource::ALL has drifted — it holds {} entries, the vocabulary has {rows}",
            FlowSource::ALL.len()
        );
    }
}
