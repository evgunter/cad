//! Node-to-kernel wiring (spec D3: wire, don't invent): each F4 node
//! maps to an EXISTING public kernel op; every editor-side geometric
//! judgment (direction normalization, the revolve axis's in-plane
//! projection, full-vs-partial classification) goes through the
//! kernel's decided-predicate door, never a raw comparison.

use std::collections::BTreeMap;
use std::sync::Arc;

use geom_core::k_stats::decide;
use geom_core::{Affine3, Band, Decide, Margin, Mat3, Point2, Point3, Sign, Tol, Vec2, Vec3};
use sweep::blend::BlendKind;
use sweep::{Extrusion, Revolution, RevolveAxis, extrude, revolve};
use topo::splitting::{SplitPart, SplitPlane, split};
use topo::transform::transform_rigid;
use topo::{
    Body, BooleanDeclarations, BooleanResult, CarriedContacts, CarriedVf, CarriedVv, ContactClass,
    DATUM_UNIT_NORM, FacePairDeclaration, GeomSource, UnitVec3, UnitVec3Error, VfContact,
    VvContact,
};

use super::anchor::{self, ProfileNaming, ProfilePre, ProfileValue};
use super::slots::{self, SlotValues};
use super::{BooleanValue, DatumValue, NodeErrorKind, NodeResult, SplitSide, ValuePayload};
use crate::names::{self, NameTable};
use crate::node::{Axis3, BooleanOp, Datum, Node, PatternKind, RecipeNodeId, SlotId};
use crate::program::ProfileProgram;

type Results<T> = BTreeMap<RecipeNodeId, NodeResult<T>>;

/// An op's product: the payload, its eagerly-emitted name table (N4 —
/// emission lives HERE in the wire layer, spec D4), and the DECLARED
/// CONTACT RECORDS the value carries (ASM-R2b D-1).
///
/// # The contacts channel (D-1)
///
/// Declared records are keyed in the op's OUTPUT BODY 0 arena. Exactly
/// one op family fills the field: [`wire_instantiate_part`], which
/// carries the referenced part's own records across the document seam.
/// A boolean's records ride its payload instead
/// ([`BooleanValue::Body::contacts`], the `BooleanBody` contract that
/// predates this channel) — `product::sources_of` is the one
/// place the two homes reconcile, so nothing downstream has to know
/// which op put records where.
///
/// **Invariant**: a multi-output op (`Split`, `Pattern`) never fills
/// this field — "output body 0" would be a lie for its other bodies.
/// Nothing today has records to carry through such an op, and the day
/// something does, the channel grows a per-output shape rather than
/// silently mis-keying.
pub(crate) struct OpOut<T: Decide> {
    pub payload: ValuePayload<T>,
    pub names: Arc<NameTable>,
    pub contacts: Arc<topo::ContactRecords>,
}

impl<T: Decide> OpOut<T> {
    /// An op that declares no contact — every op but instantiate (see
    /// the type docs for why a boolean is not an exception).
    fn plain(payload: ValuePayload<T>, names: Arc<NameTable>) -> Self {
        Self {
            payload,
            names,
            contacts: Arc::new(topo::ContactRecords::default()),
        }
    }
}

type OpResult<T> = Result<OpOut<T>, NodeErrorKind>;
/// A bare payload (datum/profile lanes — empty tables).
type PayloadResult<T> = Result<ValuePayload<T>, NodeErrorKind>;

/// The LANE half of an evaluation's environment: where profile
/// geometry comes from at `T`, and the parameter environment it is
/// elaborated over. The two travel together because they are one
/// decision — a guided elaboration is guided over SOME environment,
/// and a call site that could pass the lift without the environment
/// could elaborate the lift's second pass over a different box than
/// the node's slots were evaluated at.
#[derive(Debug)]
pub(crate) struct LaneEnv<'a, T> {
    /// Where profile geometry comes from at this evaluation's scalar
    /// (M10-P PP5): the f64 elaboration embedded, or a guided
    /// elaboration at `T`.
    pub lift: super::ProfileLift,
    /// The evaluation's parameter environment — nominals, or nominals
    /// widened by [`crate::analysis::ParamBox`] (E6's leaf replay).
    pub params: &'a crate::expr::ParamEnv<T>,
}

impl<T> Clone for LaneEnv<'_, T> {
    fn clone(&self) -> Self {
        *self
    }
}

impl<T> Copy for LaneEnv<'_, T> {}

/// The evaluation-wide context an op may need beyond its own inputs:
/// the boolean candidate-generation switch, the document seam, the
/// mate solve, and the lane environment. Bundled rather than passed
/// one by one — an op's ARGUMENTS are its inputs and slots, and
/// everything here is ambient to the run.
pub(crate) struct OpEnv<'a, T: Decide> {
    pub boolean_sweep: topo::SweepStrategy,
    pub parts: &'a super::parts::PartCache<'a, T>,
    /// The document's mate solve, run once per evaluation (ASM-R2a
    /// D-5): every instance's pose relative to its cluster gauge, and
    /// every mate's role.
    pub poses: &'a crate::mate::SolvedPoses,
    /// Where profile geometry comes from, and over which environment.
    pub lane: LaneEnv<'a, T>,
}

/// Runs one node's op against its (already Ok) inputs and evaluated
/// slots, emitting the node's name table alongside the payload.
/// `profile_pre` is the profile node's f64 precompute (present exactly
/// for `Node::Profile` — computed in `eval_node`'s resolution stage,
/// outside the verdict bracket).
#[allow(clippy::too_many_arguments)] // the 8th is the run-tolerance witness, not a duty of its own
pub(crate) fn run_op<T>(
    id: RecipeNodeId,
    node: &Node<ProfileProgram>,
    doc: &crate::doc::Doc<ProfileProgram>,
    results: &Results<T>,
    vals: &SlotValues<T>,
    payload_values: Option<&[T]>,
    profile_pre: Option<&ProfilePre>,
    env: &OpEnv<'_, T>,
    tol: Tol,
) -> OpResult<T>
where
    T: Decide
        + super::ContentBits
        + geom_core::Bounds
        + Send
        + Sync
        + topo::AtRestPolicy
        + crate::analysis::AxisScalar,
{
    match node {
        Node::Datum(d) => Ok(OpOut::plain(wire_datum(d, vals, tol)?, names::empty())),
        Node::Profile(program) => Ok(OpOut::plain(
            wire_profile(program, profile_pre, env.lane, tol)?,
            names::empty(),
        )),
        Node::Extrude { profile, .. } => wire_extrude(id, *profile, results, vals, tol),
        Node::Revolve { profile, axis, .. } => {
            wire_revolve(id, *profile, *axis, results, vals, tol)
        }
        Node::Loft { profiles, .. } => wire_loft(id, profiles, doc, vals, env.lane, tol),
        Node::Sweep { profile, path, .. } => wire_sweep(*profile, *path, doc, vals, env.lane, tol),
        Node::Fillet {
            target, selection, ..
        } => wire_blend(
            &crate::verbs::blend::fillet(),
            id,
            *target,
            selection,
            doc,
            results,
            vals,
            tol,
        ),
        Node::Chamfer {
            target, selection, ..
        } => wire_blend(
            &crate::verbs::blend::chamfer(),
            id,
            *target,
            selection,
            doc,
            results,
            vals,
            tol,
        ),
        Node::Split { target, tool } => wire_split(id, *target, *tool, results, tol),
        Node::Boolean { op, a, b, declare } => wire_boolean(
            id,
            *op,
            *a,
            *b,
            *declare,
            doc,
            results,
            env.boolean_sweep,
            tol,
        ),
        Node::Transform { input, .. } => wire_transform(id, *input, results, vals, tol),
        Node::Pattern { input, kind, .. } => wire_pattern(id, *input, kind, results, vals, tol),
        Node::PlacedUnion { input, kind, .. } => wire_placed_union(
            id,
            *input,
            kind,
            node.placement_rule_fault(),
            results,
            vals,
            tol,
        ),
        Node::Declare { pairs } => Ok(OpOut::plain(
            ValuePayload::Declarations(pairs.clone()),
            names::empty(),
        )),
        Node::Measure { expr, refs } => {
            wire_measure(node, expr, refs, payload_values, doc, results, tol)
        }
        Node::Assertion {
            measure,
            bound,
            dir,
        } => wire_assertion(*measure, bound, *dir, payload_values, results, tol),
        Node::InstantiatePart {
            doc_ref, interface, ..
        } => {
            let placement = env.poses.placement(doc, id).map_err(NodeErrorKind::Mate)?;
            wire_instantiate_part(id, doc_ref, interface, placement, env, tol)
        }
        // A mate DENOTES NO BODY (A12): it evaluates to its role in
        // the solve, which the product gather skips exactly as it
        // skips a `Declare`. A refusing mate fails typed here rather
        // than at the instance it would have placed, so the message
        // names the mate that is wrong.
        Node::Mate { .. } => match env.poses.fault(id) {
            Some(fault) => Err(NodeErrorKind::Mate(Box::new(fault.clone()))),
            None => Ok(OpOut::plain(
                ValuePayload::Mate(
                    env.poses
                        .role(id)
                        .unwrap_or(crate::mate::MateRole::Declaring),
                ),
                names::empty(),
            )),
        },
    }
}

/// ASM-2A D-3: materialize an instance through the shipped doors.
///
/// Resolve (memoized per reference), take the referenced document's A10
/// PRODUCT — what a document MEANS is its product, one rule everywhere
/// — place it with the kernel's own `transform_rigid`, and hand back a
/// body-denoting value. Nothing here is assembly-specific machinery:
/// the placed body is an ordinary `Body` — one solid or N (a
/// sub-assembly's product is multi-solid, and ONE rigid map carries all
/// of its solids, because a rigid map of a body is a rigid map of every
/// solid in it) — so the root gather and the export door consume it
/// with no new arms, and the graft into the evaluating document's
/// materialization is the gather's own (D-3's "graft into the
/// evaluating document" IS `product`, because an instantiate node is a
/// root of the assembly).
fn wire_instantiate_part<T>(
    id: RecipeNodeId,
    doc_ref: &crate::ident::DocRef,
    interface: &crate::node::InterfaceRecord,
    placement: crate::placement::Frame,
    env: &OpEnv<'_, T>,
    tol: Tol,
) -> OpResult<T>
where
    T: Decide
        + super::ContentBits
        + geom_core::Bounds
        + Send
        + Sync
        + topo::AtRestPolicy
        + crate::analysis::AxisScalar,
{
    let part = env
        .parts
        .get(doc_ref, tol)
        .map_err(|fault| NodeErrorKind::Part {
            doc_ref: *doc_ref,
            fault,
        })?;
    // ASM-R2b D-4/D-5 — A4's "does it actually fit", at the level this
    // node can answer it. Every crossing declaration names an entity
    // of the PART's product; the pinned document is whatever the pin
    // currently says, so a pin move (A13 clause 4) that changed the
    // part's contact face reaches here as a crossing whose reference
    // no longer resolves. INVARIANT: the check runs on every
    // evaluation, not only at the moving edit — an edit-time-only gate
    // would bless a document loaded from disk with a hand-moved pin.
    // The GEOMETRIC half of the fit gate is the assembly's at-rest
    // door (`crate::assembly::assemble`), which certifies the mate's
    // declaration against the placed faces; this is the structural
    // half, and it is the half that names the crossing.
    for crossing in &interface.crossings {
        let crate::node::InterfaceCrossing::Mate { mate, inner, .. } = crossing;
        if part.names.lookup(inner).is_none() {
            return Err(NodeErrorKind::CrossingUnverified {
                instance: id,
                mate: *mate,
                name: Box::new(inner.clone()),
            });
        }
    }
    // The identity fast-path is admitted only for a BIT-exact identity
    // frame: any other value could round, and `transform_rigid` is what
    // decides whether it stayed rigid.
    let mut placed = if placement.is_identity_bits() {
        (*part.body).clone()
    } else {
        transform_rigid(&part.body, &placement.affine::<T>(), tol)
            .map_err(NodeErrorKind::Transform)?
    };
    // N6 composition, the Transform precedent: `transform_rigid`
    // cleared the source records, so each description is re-stamped
    // with the part's own source wrapped by THIS placing node. Keys are
    // stable across the op, and the identity path never cleared them.
    compose_placed(&part.body, &mut placed, id, 0);
    let table = names::name_in_part(id, &part.names, &placed).map_err(NodeErrorKind::Naming)?;
    // ASM-R2b D-1: the part's OWN declared contacts survive
    // instantiation. INVARIANT — the records ride the placement
    // UNCHANGED, because `transform_rigid` is key-stable (its own
    // contract, the same one `compose_placed` above depends on) and the
    // identity fast path clones keys verbatim. Re-deriving them from
    // the placed geometry is exactly the scan-to-bless move F1 bans;
    // the declaration is inherited, never rediscovered.
    Ok(OpOut {
        payload: ValuePayload::Body(Arc::new(placed)),
        names: table,
        contacts: Arc::clone(&part.contacts),
    })
}

/// Stamps every UNSOURCED description of `body` with this node's
/// minted [`GeomSource`]s, one shared index space in deterministic
/// arena order (D1/N6: every description minted by evaluation carries
/// its recipe source; pass-through descriptions keep the source they
/// arrived with). Per-evaluation identity — exactly the scope N6's
/// binding caveat allows.
fn stamp_minted<T: Decide>(body: &mut Body<T>, node: RecipeNodeId) {
    let mut idx: u32 = 0;
    let surfaces: Vec<_> = body
        .surfaces()
        .map(|(k, _)| k)
        .filter(|&k| body.surface_source(k).is_none())
        .collect();
    for k in surfaces {
        // Stamping a just-enumerated live key cannot fail.
        let _ = body.set_surface_source(k, GeomSource::minted(node.0, idx));
        idx += 1;
    }
    let curves: Vec<_> = body
        .curves()
        .map(|(k, _)| k)
        .filter(|&k| body.curve_source(k).is_none())
        .collect();
    for k in curves {
        let _ = body.set_curve_source(k, GeomSource::minted(node.0, idx));
        idx += 1;
    }
    let points: Vec<_> = body
        .points()
        .map(|(k, _)| k)
        .filter(|&k| body.point_source(k).is_none())
        .collect();
    for k in points {
        let _ = body.set_point_source(k, GeomSource::minted(node.0, idx));
        idx += 1;
    }
}

/// Re-stamps `placed`'s descriptions with `input`'s sources wrapped
/// by placing node `by` at `instance` (N6: the transform node
/// composes into `expr`). Keys are stable across `transform_rigid`,
/// so the input's rows map key-for-key.
fn compose_placed<T: Decide>(
    input: &Body<T>,
    placed: &mut Body<T>,
    by: RecipeNodeId,
    instance: u32,
) {
    let surfaces: Vec<_> = input
        .surfaces()
        .filter_map(|(k, _)| {
            input
                .surface_source(k)
                .map(|s| (k, s.placed(by.0, instance)))
        })
        .collect();
    for (k, src) in surfaces {
        let _ = placed.set_surface_source(k, src);
    }
    let curves: Vec<_> = input
        .curves()
        .filter_map(|(k, _)| input.curve_source(k).map(|s| (k, s.placed(by.0, instance))))
        .collect();
    for (k, src) in curves {
        let _ = placed.set_curve_source(k, src);
    }
    let points: Vec<_> = input
        .points()
        .filter_map(|(k, _)| input.point_source(k).map(|s| (k, s.placed(by.0, instance))))
        .collect();
    for (k, src) in points {
        let _ = placed.set_point_source(k, src);
    }
}

/// The (Ok) value of an input node.
fn value_of<T: Decide>(
    results: &Results<T>,
    input: RecipeNodeId,
) -> Result<&super::NodeValue<T>, NodeErrorKind> {
    match results.get(&input) {
        Some(NodeResult::Ok(v)) => Ok(v),
        // Failed/Poisoned inputs never reach run_op (poison
        // propagation happens first); an absent entry is a dangling
        // reference.
        _ => Err(NodeErrorKind::MissingInput { input }),
    }
}

/// A single-body operand: a Body value, or a boolean's non-empty
/// result. Splits and patterns need PR 3's naming layer to select a
/// part — typed refusal, not a guess.
fn body_operand<T: Decide>(
    results: &Results<T>,
    input: RecipeNodeId,
) -> Result<Arc<Body<T>>, NodeErrorKind> {
    let v = value_of(results, input)?;
    match &v.payload {
        ValuePayload::Body(b) => Ok(Arc::clone(b)),
        ValuePayload::Boolean(BooleanValue::Body { body, .. }) => Ok(Arc::clone(body)),
        ValuePayload::Boolean(BooleanValue::Empty) => Err(NodeErrorKind::EmptyOperand { input }),
        other => Err(NodeErrorKind::WrongOperand {
            input,
            expected: "body",
            found: other.kind_name(),
        }),
    }
}

/// The linear classification band (kernel-ambient tolerance).
fn band(tol: Tol) -> Result<Band, NodeErrorKind> {
    Band::linear(tol).map_err(NodeErrorKind::Band)
}

/// Normalizes a direction-valued vector; decided-zero length refuses,
/// in-band indeterminacy escalates (all through the one door). Shared
/// with the mate solve's derived-offset derivation, so a direction is
/// decided under the same predicate wherever it is read.
pub(crate) fn unit<T: Decide>(
    v: Vec3<T>,
    role: &'static str,
    band: Band,
) -> Result<Vec3<T>, NodeErrorKind> {
    match decide("eval_direction_norm", Margin::norm3(v), band) {
        Ok(Sign::Positive) => Ok(v.normalize()),
        Ok(_) => Err(NodeErrorKind::DegenerateDirection { role }),
        Err(source) => Err(NodeErrorKind::Escalated {
            predicate: "eval_direction_norm",
            source,
        }),
    }
}

/// A Length-valued `[Expr; 3]` triple as a point.
fn point3<T: Decide>(vals: &SlotValues<T>, f: fn(Axis3) -> SlotId) -> Option<Point3<T>> {
    let v = slots::vec3(vals, f)?;
    Some(Point3::new(v.x, v.y, v.z))
}

fn need_scalar<T: Decide>(vals: &SlotValues<T>, slot: SlotId) -> Result<T, NodeErrorKind> {
    slots::scalar(vals, slot).ok_or(NodeErrorKind::MissingSlot { slot })
}

fn need_vec3<T: Decide>(
    vals: &SlotValues<T>,
    f: fn(Axis3) -> SlotId,
) -> Result<Vec3<T>, NodeErrorKind> {
    slots::vec3(vals, f).ok_or(NodeErrorKind::MissingSlot { slot: f(Axis3::X) })
}

fn need_point3<T: Decide>(
    vals: &SlotValues<T>,
    f: fn(Axis3) -> SlotId,
) -> Result<Point3<T>, NodeErrorKind> {
    point3(vals, f).ok_or(NodeErrorKind::MissingSlot { slot: f(Axis3::X) })
}

/// A slot's vector as a datum direction, through the kernel type's own
/// constructor: the normalization and the two refusals live there, and
/// this layer only names the ROLE the refusal is about.
fn datum_unit<T: Decide>(
    v: Vec3<T>,
    role: &'static str,
    band: Band,
) -> Result<UnitVec3<T>, NodeErrorKind> {
    UnitVec3::new(v, band).map_err(|e| match e {
        UnitVec3Error::Degenerate => NodeErrorKind::DegenerateDirection { role },
        UnitVec3Error::Escalated(source) => NodeErrorKind::Escalated {
            predicate: DATUM_UNIT_NORM,
            source,
        },
    })
}

fn wire_datum<T: Decide>(d: &Datum, vals: &SlotValues<T>, tol: Tol) -> PayloadResult<T> {
    Ok(ValuePayload::Datum(match d {
        Datum::Plane { .. } => DatumValue::Plane {
            origin: need_point3(vals, SlotId::Origin)?,
            normal: datum_unit(
                need_vec3(vals, SlotId::Normal)?,
                "datum plane normal",
                band(tol)?,
            )?,
        },
        Datum::Axis { .. } => DatumValue::Axis {
            origin: need_point3(vals, SlotId::Origin)?,
            dir: datum_unit(
                need_vec3(vals, SlotId::Direction)?,
                "datum axis direction",
                band(tol)?,
            )?,
        },
        Datum::Point { .. } => DatumValue::Point {
            position: need_point3(vals, SlotId::Origin)?,
        },
    }))
}

/// The profile node's F64 PRECOMPUTE (LIB-SWITCH §4b): the resolved
/// program replays through `profile::replay` — the driver, the ONLY
/// path from steps to geometry — then the assembled `Profile<f64>`
/// validates at f64 (the C6 structure-selection gate, which also
/// yields the canonical form the program-anchor naming map is derived
/// from). Runs OUTSIDE the verdict bracket (`eval_node`): these are
/// structure decisions, the successor of the stored f64 bits, not
/// per-lane op decisions. VQ6 is closed here and in the op below: the
/// replay-time junction checks and both validations run under the
/// SAME `Tolerance::get()` the evaluation pins.
pub(crate) fn prepare_profile(
    program: &ProfileProgram,
    resolved: &[Vec<profile::Step<f64>>],
    tol: Tol,
) -> Result<ProfilePre, NodeErrorKind> {
    let mut loops = Vec::with_capacity(resolved.len());
    let mut replay_records = Vec::with_capacity(resolved.len());
    for (li, steps) in resolved.iter().enumerate() {
        let (lp, record) = profile::replay_recording(steps, tol).map_err(|error| {
            NodeErrorKind::ProfileReplay {
                loop_: li as u32,
                error,
            }
        })?;
        loops.push(lp);
        replay_records.push(record);
    }
    let profile_f64 = profile::Profile::new(program.plane, loops);
    let (validated_f64, canonical) = profile_f64
        .validate_recording(tol)
        .map_err(NodeErrorKind::Profile)?;
    let naming = anchor::derive_naming(&validated_f64, &profile_f64.loops).ok_or({
        // A canonical loop failed to match any program loop — an
        // internal invariant break, typed (the loop coordinate is not
        // recoverable from the failed derivation; 0 names the walk).
        NodeErrorKind::ProfileAnchor { loop_: 0 }
    })?;
    Ok(ProfilePre {
        profile_f64,
        naming,
        structure: profile::ProfileStructure {
            replay: replay_records,
            canonical,
        },
    })
}

/// The lift's SECOND PASS (M10-P PP1/PP5): the same program resolved at
/// the lane scalar and elaborated there, GUIDED by pass 1's record.
///
/// This is where a profile parameter finally reaches the lane — a
/// `Dual` seed on a fillet radius carries its tangent all the way to
/// the vertex it moves, an interval parameter widens the loop it
/// describes — while structure stays exactly what the f64 pass chose.
///
/// **The interval half is reachable**: the environment is the
/// evaluation's own ([`LaneEnv::params`]), so an evaluation carrying a
/// [`crate::analysis::ParamBox`] widens the loops this program
/// describes. The DUAL half — document-level seeding, a binding with a
/// non-zero tangent — has no door yet; a `Dual` binding's tangent is
/// still zero, and until seeding lands the capability is exercised one
/// door down, at the program-resolve seam this function calls, which is
/// where `editor-core`'s `m10_p_lift` suite drives it.
/// The naming is pass 1's verbatim (PP4): names are program-structural
/// indices, and the canonical permutation they hang off is pinned by
/// the record, so `T`-valued geometry changes no name.
fn lane_profile<T: Decide + geom_core::Bounds>(
    program: &ProfileProgram,
    lane: LaneEnv<'_, T>,
    pre: &ProfilePre,
    tol: Tol,
) -> Result<profile::ValidatedProfile<T>, NodeErrorKind> {
    let resolved = program
        .resolve(lane.params)
        .map_err(|(slot, source)| NodeErrorKind::Expr { slot, source })?;
    let mut loops = Vec::with_capacity(resolved.len());
    for (li, steps) in resolved.iter().enumerate() {
        // One record per program loop, by construction of pass 1.
        //
        // The fallback is an EMPTY record, and what that buys depends on
        // the loop: a loop with a fillet in it refuses loudly at the
        // first resolution (the guide runs off the end of the record,
        // which `Guide::consume` refuses rather than falling through to
        // free selection), while a loop with NO fillet — a rectangle, a
        // circle — has nothing to consume and would elaborate happily
        // against an empty record. The missing record is an internal
        // break either way; this comment says which half of the
        // vocabulary is actually holding the line, because the other
        // half is the shape check in `replay_guided`, not this.
        let record = pre.structure.replay.get(li).cloned().unwrap_or_default();
        let lp = profile::replay_guided(steps, &record, tol).map_err(|error| {
            NodeErrorKind::ProfileLaneReplay {
                loop_: li as u32,
                step: error.step,
                structure: match error.kind {
                    profile::ReplayErrorKind::Path(profile::PathError::Structure(r)) => Some(r),
                    _ => None,
                },
            }
        })?;
        loops.push(lp);
    }
    let plane = profile::SketchPlane::new(anchor::embed_affine::<T>(&program.plane.placement));
    profile::Profile::new(plane, loops)
        .validate_guided(tol, &pre.structure.canonical)
        .map_err(NodeErrorKind::Profile)
}

fn wire_profile<T: Decide + geom_core::Bounds>(
    program: &ProfileProgram,
    pre: Option<&ProfilePre>,
    lane: LaneEnv<'_, T>,
    tol: Tol,
) -> PayloadResult<T> {
    let Some(pre) = pre else {
        // Unreachable by eval_node's stage order; typed, never a panic.
        return Err(NodeErrorKind::MissingSlot {
            slot: SlotId::Profile {
                loop_: 0,
                step: 0,
                arg: crate::node::StepArg::PointX,
            },
        });
    };
    let validated = match lane.lift {
        super::ProfileLift::Pinned => anchor::embed_profile::<T>(&pre.profile_f64)
            .validate(tol)
            .map_err(NodeErrorKind::Profile)?,
        super::ProfileLift::Guided => lane_profile::<T>(program, lane, pre, tol)?,
    };
    Ok(ValuePayload::Profile(Arc::new(ProfileValue {
        validated,
        naming: pre.naming.clone(),
    })))
}

/// Applies the program-anchor rewrite to an emitted table (identity
/// anchors skip the rebuild). A collision is an internal bug (the
/// rewrite is a bijection per loop), refused typed.
fn anchored(
    table: Arc<NameTable>,
    naming: &ProfileNaming,
) -> Result<Arc<NameTable>, NodeErrorKind> {
    if naming.is_identity() {
        return Ok(table);
    }
    match anchor::remap_table(&table, naming) {
        Some(t) => Ok(Arc::new(t)),
        None => Err(NodeErrorKind::Naming(names::NamingError::Emission {
            what: "program-anchor rewrite collided (bijection invariant broken)",
        })),
    }
}

fn wire_extrude<T: Decide>(
    id: RecipeNodeId,
    profile: RecipeNodeId,
    results: &Results<T>,
    vals: &SlotValues<T>,
    tol: Tol,
) -> OpResult<T> {
    let v = value_of(results, profile)?;
    let ValuePayload::Profile(vp) = &v.payload else {
        return Err(NodeErrorKind::WrongOperand {
            input: profile,
            expected: "profile",
            found: v.payload.kind_name(),
        });
    };
    let distance = need_scalar(vals, SlotId::Distance)?;
    let mut built = extrude(&vp.validated, Extrusion::Distance(distance), tol)
        .map_err(NodeErrorKind::Extrude)?;
    // Eager N4 emission from the emitter's own maps, BEFORE the
    // structural handoff is dropped — then the program-anchor rewrite
    // (canonical → program indices; LIB-SWITCH §6).
    let table = names::name_extrude(id, &built).map_err(NodeErrorKind::Naming)?;
    let table = anchored(table, &vp.naming)?;
    stamp_minted(&mut built.body, id);
    Ok(OpOut::plain(
        ValuePayload::Body(Arc::new(built.body)),
        table,
    ))
}

fn wire_revolve<T: Decide + geom_brep::PcurveFittedLane>(
    id: RecipeNodeId,
    profile: RecipeNodeId,
    axis: RecipeNodeId,
    results: &Results<T>,
    vals: &SlotValues<T>,
    tol: Tol,
) -> OpResult<T> {
    let pv = value_of(results, profile)?;
    let ValuePayload::Profile(vp) = &pv.payload else {
        return Err(NodeErrorKind::WrongOperand {
            input: profile,
            expected: "profile",
            found: pv.payload.kind_name(),
        });
    };
    let av = value_of(results, axis)?;
    let ValuePayload::Datum(DatumValue::Axis { origin, dir }) = &av.payload else {
        return Err(NodeErrorKind::WrongOperand {
            input: axis,
            expected: "datum axis",
            found: av.payload.kind_name(),
        });
    };
    let dir = dir.get();
    // The kernel's RevolveAxis lives in SKETCH-PLANE coordinates: the
    // 3-D datum axis must lie in the profile's plane (decided; a
    // definite out-of-plane component is a typed refusal, spec D3's
    // "wire, don't invent" — projecting silently would be invention).
    let place = vp.validated.plane().placement;
    let (u, v_axis, n) = (place.linear.c0, place.linear.c1, place.linear.c2);
    let plane_origin = Point3::new(
        place.translation.x,
        place.translation.y,
        place.translation.z,
    );
    let rel = *origin - plane_origin;
    let b = band(tol)?;
    // The two in-plane checks share a verdict shape but NOT a
    // dimension, so they take separate doors (review of the clause-(i)
    // rollout, MAJ-1): the origin residual is a metre projection onto
    // the unit normal (`of`); the direction residual `dir·n̂` is a
    // unit·unit SINE — dimensionless against the metre band, the
    // audit's class-(c) shape. Ledger row F15: the honest form is the
    // sine levered at the profile's radial extent, which lives
    // kernel-side — that fix is F15's own unit; flagged, not cast.
    let in_plane = |name: &'static str,
                    verdict: Result<Sign, geom_core::Indeterminate>|
     -> Result<(), NodeErrorKind> {
        match verdict {
            Ok(Sign::Zero) => Ok(()),
            Ok(_) => Err(NodeErrorKind::AxisNotInSketchPlane { axis }),
            Err(source) => Err(NodeErrorKind::Escalated {
                predicate: name,
                source,
            }),
        }
    };
    in_plane(
        "revolve_axis_origin_in_plane",
        decide("revolve_axis_origin_in_plane", Margin::of(rel.dot(n)), b),
    )?;
    in_plane(
        "revolve_axis_dir_in_plane",
        geom_core::k_stats::decide_flagged("revolve_axis_dir_in_plane", dir.dot(n), b, "F15"),
    )?;
    let axis2 = RevolveAxis {
        origin: Point2::new(rel.dot(u), rel.dot(v_axis)),
        dir: Vec2::new(dir.dot(u), dir.dot(v_axis)),
    };
    // Full vs partial (kernel contract: exactly-full must SAY Full):
    // |θ| coincident with τ at tolerance classifies Full; anything
    // else wires Partial and the kernel's own angle classification
    // rules on it (out-of-range partials refuse loudly there).
    let angle = need_scalar(vals, SlotId::RevolveAngle)?;
    let abs_angle = angle.max(-angle);
    // Ledger row F14 (found by the clause-(i) migration): |θ| − τ is
    // RADIANS against the linear band — dimensionless; the honest
    // lever (the profile's radial extent) lives kernel-side. Flagged,
    // not cast.
    let revolution = match geom_core::k_stats::decide_flagged(
        "revolve_full_vs_partial",
        abs_angle - T::tau(),
        b,
        "F14",
    ) {
        Ok(Sign::Zero) => Revolution::Full,
        Ok(_) => Revolution::Partial(angle),
        Err(source) => {
            return Err(NodeErrorKind::Escalated {
                predicate: "revolve_full_vs_partial",
                source,
            });
        }
    };
    let mut built =
        revolve(&vp.validated, axis2, revolution, tol).map_err(NodeErrorKind::Revolve)?;
    let table = names::name_revolve(id, &built).map_err(NodeErrorKind::Naming)?;
    let table = anchored(table, &vp.naming)?;
    stamp_minted(&mut built.body, id);
    Ok(OpOut::plain(
        ValuePayload::Body(Arc::new(built.body)),
        table,
    ))
}

/// **The verb dispatch's one refusal translation**: the kernel door
/// attached the verb, `verbs::run` carried the refusal through
/// unaltered, and this layer READS the verb off it rather than
/// re-deriving which door it called — one discrimination point per
/// layer, and one site for it here so no two doors can drift.
///
/// Exhaustive over [`verbs::VerbError`] with no wildcard arm, so a verb
/// family whose refusal is not a blend's breaks here rather than
/// arriving as one.
fn verb_refused(refusal: verbs::VerbError) -> NodeErrorKind {
    match refusal {
        verbs::VerbError::Blend(sweep::blend::BlendRefusal { verb, error }) => {
            NodeErrorKind::Blend { verb, error }
        }
    }
}

/// **The blend pair's ONE lowering**, driven by the verb's
/// correspondence ([`crate::verbs::blend`]) rather than written twice.
///
/// The shape is the same for both verbs and always was: resolve the
/// frozen selection through the target's name table into edge keys,
/// evaluate the size slot to `T`, build the kernel verb, run it, emit
/// names from the birth record under THIS node's id. What the
/// correspondence supplies is the four literals that differ — the size
/// slot, the selection-refusal label, which verb to build, and what to
/// call a missing record.
///
/// # Fillet
///
/// **Constant-radius rolling-ball fillets on a SELECTION of the
/// target's edges** (M5 PR 12; the selection is M6-5).
///
/// # Chamfer
///
/// **Equal-setback flat chamfers on a SELECTION of the target's
/// edges** — the fillet's twin, and the reason this function is one
/// function.
///
/// # Refusals
///
/// The selection resolves through the TARGET's name table into edge
/// keys. Resolution failures are the N5 typed trio VERBATIM
/// ([`NodeErrorKind::BlendSelectionResolve`]) — a selection is a
/// commitment (the blend nodes' freeze semantics), so a name that
/// stopped resolving refuses loudly rather than shrinking the set.
///
/// Failure of the op itself is a TYPED refusal
/// ([`NodeErrorKind::Blend`]) carrying the kernel's own error
/// unaltered, exactly as the split/boolean arms carry theirs. The input
/// body is never passed through: a blend that did not happen must read
/// as a failed node, not as a silently sharp solid.
///
/// # Naming
///
/// **The assembly emits a FULL table**: the kernel hands over
/// per-entity birth records and the emitter translates them, never
/// matching geometry. A blend result therefore always carries birth
/// records, and the totality check covers every role it mints; an empty
/// table would be a silent naming dead end, so this layer refuses
/// rather than accepting one — `naming: None` is a kernel bug, and
/// falling back to an empty table would leave every downstream
/// reference into this body silently unresolvable.
///
/// The role vocabulary is SHARED between the two verbs, and what tells
/// a chamfer's strip from a fillet's blend at a selector is which node
/// minted it (RECIPE-DOORS D3) — which is why one lowering can serve
/// both without their names colliding.
///
/// What that is worth, precisely (`m6_5_downstream.rs`): the appearance
/// store resolves an attribute onto a fillet-minted face, the resolve
/// ladder answers `Resolved` for every role this door mints, and such a
/// reference survives an upstream bump. A BOOLEAN over a filleted body
/// is still not reachable — the kernel refuses
/// `FallbackExtentUnsupported` on the sphere octants every fillet
/// result carries, even against a disjoint operand — and that frontier,
/// which predates M6-5, is pinned executed in the same file. The naming
/// side is ready; the kernel side is not.
// The 8th is the verb's correspondence — which is what collapses two
// of these functions into one, so it is the parameter that REMOVES
// duplication rather than adding a duty.
#[allow(clippy::too_many_arguments)]
fn wire_blend<T: Decide + geom_core::Bounds + geom_brep::PcurveFittedLane>(
    verb: &crate::verbs::blend::BlendVerb<T>,
    id: RecipeNodeId,
    target: RecipeNodeId,
    selection: &[names::StableName],
    doc: &crate::doc::Doc<ProfileProgram>,
    results: &Results<T>,
    vals: &SlotValues<T>,
    tol: Tol,
) -> OpResult<T> {
    let body = body_operand(results, target)?;
    let size = need_scalar(vals, verb.size_slot)?;
    let target_table = Arc::clone(&value_of(results, target)?.name_table);
    let edges = resolve_selection(verb.selection_label, selection, doc, &target_table)?;
    let out = (verb.build)(edges, size)
        .run(&body, tol)
        .map_err(verb_refused)?;
    let rec = out
        .naming
        .as_ref()
        .ok_or(NodeErrorKind::Naming(names::NamingError::Emission {
            what: verb.no_records,
        }))?;
    let table =
        (verb.emitter)(id, target, &target_table, &out.body, rec).map_err(NodeErrorKind::Naming)?;
    let mut body = out.body;
    // The blend's own surfaces, curves and points are minted HERE
    // (D1/N6); the supports' pass-through descriptions keep the source
    // they arrived with.
    stamp_minted(&mut body, id);
    Ok(OpOut::plain(ValuePayload::Body(Arc::new(body)), table))
}

/// The mid-evaluation N5 refusal ladder, shared by every door that
/// resolves an AUTHORED name against the tables the run has built so
/// far ([`resolve_selection`], [`resolve_declarations`]).
///
/// Mid-evaluation there is no prior run and no whole-evaluation
/// index, so [`mod@crate::resolve`]'s full ladder does not apply:
/// what is left is three rungs, in this order.
///
/// 1. [`ladder::live`] — the minting node must still be in the
///    document. Ids are never reused, so an id below the mint counter
///    was DELETED and one at/above it was never this document's
///    (`ForeignNode`). This rung outranks every later refusal,
///    including a door's own, and the [`ladder::Live`] token enforces
///    that rather than asking for it: reading a table needs the token,
///    so no refusal ABOUT the tables — the ladder's own rungs, or a
///    door's, like the declare door's both-operands — can be reached
///    before this one has passed.
/// 2. [`ladder::Landing::Tied`] → `Ambiguous`. The tie row IS the
///    ambiguity (N5), so the tied set expressed in names is the name
///    itself, and the witness carries the multiplicity and the
///    minting site.
/// 3. [`ladder::Landing::Absent`] → `Vanished`, through
///    [`crate::resolve::ResolveError::vanished_fallback`]: no prior
///    run is consultable mid-evaluation, so there is no evidence to
///    weigh and nothing to bank, which is exactly the payload that
///    constructor names.
///
/// The refusals come out BOXED, which is how both doors' error
/// variants carry a `ResolveError` anyway.
///
/// A door supplies [`ladder::Landing`]s — one per table it resolves
/// through — and keeps its own arity: which table to consult, what a
/// multi-table hit means, and what kind of entity it will accept are
/// the door's business. Which typed refusal comes out is this
/// module's, and has one home.
///
/// **What is shared with [`mod@crate::resolve`], and what is not.**
/// Every PAYLOAD is that module's, minted by one constructor each —
/// [`crate::resolve::ResolveError::node_gone`] for the
/// deleted-vs-foreign split, [`crate::resolve::ResolveError::ambiguous`]
/// for the tie and its witness,
/// [`crate::resolve::ResolveError::vanished_fallback`] for the
/// no-evidence vanish — so neither ladder restates the other's refusal
/// and the two cannot drift about what a stranded, tie-marked or
/// vanished name looks like.
///
/// What is NOT shared is how rung 3 is REACHED. That module arrives at
/// the same fallback only after a diagnosis ladder over two
/// evaluations comes up empty; here it is the immediate answer,
/// because mid-evaluation there is neither a prior run nor a
/// whole-evaluation index to run that ladder against. Same value, two
/// different roads, and only the value is worth holding in one place.
///
/// What stays here is what is this module's subject: the rung ORDER,
/// the [`ladder::Live`] token that enforces it, and a door's arity.
mod ladder {
    use crate::names::{EntityRef, Entry, NameTable, StableName};
    use crate::program::ProfileProgram;
    use crate::resolve::ResolveError;

    /// Where a name landed in ONE table (rungs 2 and 3, as data).
    pub(super) enum Landing {
        /// Exactly one entity carries the name.
        Unique(EntityRef),
        /// The name is a tie row of this width.
        Tied(u32),
        /// This table does not carry the name.
        Absent,
    }

    /// Proof that rung 1 passed: `name`'s minting node is live.
    ///
    /// Constructible only by [`live`], and required by BOTH [`landing`]
    /// and [`resolve`]. That is what enforces the rung order rather
    /// than documenting it: a door cannot read a table before the
    /// `NodeGone` check, so a door's own refusal — which is a refusal
    /// ABOUT what the tables say — cannot preempt rung 1 either. The
    /// declare door needs two landings to know a name sits in both
    /// operands, and it cannot have one without this token.
    ///
    /// Carrying the name also means [`landing`] and [`resolve`] cannot
    /// disagree about WHICH name they are answering for: the tie width
    /// in an `Ambiguous` payload is measured on the same name the
    /// payload is built from, by construction.
    pub(super) struct Live<'n>(&'n StableName);

    /// Reads one table for the live name (N4: resolution IS this read).
    pub(super) fn landing(live: &Live<'_>, table: &NameTable) -> Landing {
        match table.lookup(live.0) {
            Some(Entry::Unique(ent)) => Landing::Unique(*ent),
            Some(Entry::Tied(ents)) => Landing::Tied(ents.len() as u32),
            None => Landing::Absent,
        }
    }

    /// Rung 1: `NodeGone` with the deleted-vs-foreign split, taken
    /// from the one home that mints it ([`ResolveError::node_gone`]).
    pub(super) fn live<'n>(
        name: &'n StableName,
        doc: &crate::doc::Doc<ProfileProgram>,
    ) -> Result<Live<'n>, Box<ResolveError>> {
        match ResolveError::node_gone(name, doc) {
            None => Ok(Live(name)),
            Some(gone) => Err(Box::new(gone)),
        }
    }

    /// Rungs 2 and 3: the entity, or the refusal its landing earns.
    pub(super) fn resolve(
        live: Live<'_>,
        landing: Landing,
    ) -> Result<EntityRef, Box<ResolveError>> {
        let name = live.0;
        match landing {
            Landing::Unique(ent) => Ok(ent),
            // The tie row is the name itself: a door resolves the
            // authored name against one table, so there is no widened
            // base to tie against here.
            Landing::Tied(width) => Err(Box::new(ResolveError::ambiguous(
                name,
                name.clone(),
                name.node,
                width,
            ))),
            Landing::Absent => Err(Box::new(ResolveError::vanished_fallback(name))),
        }
    }
}

/// Resolves a fillet's edge selection against the target's name table
/// (M6-5). Single-operand, so simpler than
/// [`resolve_declarations`] — but the refusal vocabulary is the SAME
/// N5 trio, deliberately: the two sites answer the same question, and
/// they answer it through the same [`ladder`], which owns rung order
/// and payload shapes. What stays here is this door's arity — one
/// table — and its kind refusal: a selection names EDGES.
///
/// The returned keys are in TARGET-ARENA order, not selection order,
/// so the kernel sees the deterministic order every derived list in
/// this kernel inherits (D9) regardless of how the recipe sorted.
fn resolve_selection(
    verb: BlendKind,
    selection: &[names::StableName],
    doc: &crate::doc::Doc<ProfileProgram>,
    target: &NameTable,
) -> Result<Vec<topo::EdgeKey>, NodeErrorKind> {
    use crate::names::EntityKey;

    if selection.is_empty() {
        return Err(NodeErrorKind::BlendSelectionEmpty { verb });
    }
    let mut keys = Vec::with_capacity(selection.len());
    for name in selection {
        let refused = |error| NodeErrorKind::BlendSelectionResolve { verb, error };
        let live = ladder::live(name, doc).map_err(refused)?;
        let landing = ladder::landing(&live, target);
        let ent = ladder::resolve(live, landing).map_err(refused)?;
        let EntityKey::Edge(k) = ent.key else {
            return Err(NodeErrorKind::BlendSelectionKind {
                verb,
                name: Box::new(name.clone()),
                found: ent.key.kind(),
            });
        };
        keys.push(k);
    }
    // D9 order; the kernel refuses a repeated edge itself, so a
    // duplicate that survived canonicalization still fails loudly.
    keys.sort_unstable();
    Ok(keys)
}

/// **A measurement sink** (E3): resolve the node's references, read
/// the carriers they sit on, run the closed form, hand back a typed F1
/// quantity. No body in, no body out.
///
/// # Where a reference resolves
///
/// At the node the reference NAMES AS ITS READING SITE
/// ([`crate::MeasureRef::at`]), which is what makes the answer the
/// PLACED carrier rather than the authored one — a transform is
/// identity-preserving, so the minting node's value still holds the
/// unmoved geometry. `at` is a DAG edge ([`Node::inputs`]), so it has
/// evaluated by the time this runs. Resolution takes the SAME
/// mid-evaluation [`ladder`] the fillet selection and the declare door
/// take: rung 1 is the live-node check, then the tie, then the
/// vanished row, with N5's typed trio coming out of all three.
///
/// # Only the references the expression READS are resolved
///
/// A reference no primitive indexes is carried data, not a
/// measurement input, so it is neither resolved nor interrogated: an
/// unused reference to a datum (which has no carrier at all) must not
/// fail a measure that never asks about it. The indices the expression
/// actually reads are the domain, and the slots left empty are filled
/// with [`super::measure::Carrier::Unread`], which no closed form can
/// reach — `Node::measure_fault` has already bounded every index, so a
/// read of one is a kernel bug and says so.
fn wire_measure<T: Decide>(
    node: &Node<ProfileProgram>,
    expr: &crate::measure::MeasureExpr,
    refs: &[crate::node::MeasureRef],
    leaves: Option<&[T]>,
    doc: &crate::doc::Doc<ProfileProgram>,
    results: &Results<T>,
    tol: Tol,
) -> OpResult<T> {
    // The backstop for a node that reached evaluation malformed: the
    // construction and load doors both refuse this, so reaching it
    // means a hand-built value bypassed them — refused typed rather
    // than indexed past the end of the reference list.
    if let Some(fault) = node.measure_fault() {
        return Err(NodeErrorKind::MeasureMalformed(fault));
    }
    let mut read = std::collections::BTreeSet::new();
    let mut prims = Vec::new();
    expr.primitives(&mut prims);
    for prim in prims {
        read.extend(prim.refs());
    }
    let mut carriers = Vec::with_capacity(refs.len());
    for (index, r) in refs.iter().enumerate() {
        let index = u32::try_from(index).unwrap_or(u32::MAX);
        if !read.contains(&index) {
            carriers.push(super::measure::Carrier::Unread);
            continue;
        }
        let name = &r.name;
        let refused = |error| NodeErrorKind::MeasureRefResolve { error };
        let live = ladder::live(name, doc).map_err(refused)?;
        let value = value_of(results, r.at)?;
        let landing = ladder::landing(&live, &value.name_table);
        let ent = ladder::resolve(live, landing).map_err(refused)?;
        let body =
            crate::names::interrogate::output_body(&value.payload, ent.body).map_err(|error| {
                NodeErrorKind::MeasureRefUnreadable {
                    name: Box::new(name.clone()),
                    error,
                }
            })?;
        carriers.push(super::measure::carrier_of(body, ent));
    }
    let mut cursor = 0usize;
    let value = super::measure::eval_measure(
        expr,
        &carriers,
        leaves.unwrap_or(&[]),
        &mut cursor,
        band(tol)?,
    )
    .map_err(|refusal| match refusal {
        super::measure::PrimitiveRefusal::Unsupported(u) => NodeErrorKind::MeasureUnsupported(u),
        super::measure::PrimitiveRefusal::Escalated { predicate, source } => {
            NodeErrorKind::Escalated { predicate, source }
        }
        super::measure::PrimitiveRefusal::NonFinite(source) => {
            NodeErrorKind::MeasureNonFinite { source }
        }
        super::measure::PrimitiveRefusal::NotParallel {
            verb,
            a,
            b,
            predicate,
        } => NodeErrorKind::MeasureNotParallel {
            verb,
            a,
            b,
            predicate,
        },
    })?;
    Ok(OpOut::plain(
        ValuePayload::Measure {
            value,
            dim: expr.dim(),
        },
        names::empty(),
    ))
}

/// **An assertion's verdict** (E10): compare the measure this node
/// references against its bound, and report.
///
/// Report-ONLY, and the shape says so: the value that comes out is a
/// verdict, no op in the vocabulary accepts a verdict as an operand,
/// and nothing here touches the measure's own value or the document.
/// A `Violated` verdict costs the run exactly one payload.
fn wire_assertion<T: Decide>(
    measure: RecipeNodeId,
    bound_expr: &crate::expr::Expr,
    dir: crate::measure::AssertionDir,
    payload_values: Option<&[T]>,
    results: &Results<T>,
    tol: Tol,
) -> OpResult<T> {
    let mv = value_of(results, measure)?;
    let ValuePayload::Measure { value, dim } = &mv.payload else {
        return Err(NodeErrorKind::WrongOperand {
            input: measure,
            expected: "measure",
            found: mv.payload.kind_name(),
        });
    };
    // The bound's DECLARED dimension is what must agree — read off the
    // expression, never inferred from the evaluated number, which has
    // no dimension left (units erase at the evaluation boundary).
    if bound_expr.dim() != *dim {
        return Err(NodeErrorKind::AssertionDimension {
            measured: *dim,
            bound: bound_expr.dim(),
        });
    }
    // The bound is this node's ONE payload expression, evaluated in
    // the same stage every other payload expression is: a miss means
    // `payload_exprs` and this arm disagree about what the node
    // carries, which is a kernel bug, not a document fault.
    let Some(bound) = payload_values.and_then(|v| v.first().copied()) else {
        unreachable!(
            "an assertion's bound is its only payload expression, yet the evaluated payload \
             vector has none"
        )
    };
    Ok(OpOut::plain(
        ValuePayload::Assertion(crate::measure::decide_assertion(
            *value,
            bound,
            dir,
            band(tol)?,
        )),
        names::empty(),
    ))
}

fn wire_split<T: Decide + geom_brep::PcurveFittedLane>(
    id: RecipeNodeId,
    target: RecipeNodeId,
    tool: RecipeNodeId,
    results: &Results<T>,
    tol: Tol,
) -> OpResult<T> {
    let body = body_operand(results, target)?;
    let tv = value_of(results, tool)?;
    let ValuePayload::Datum(DatumValue::Plane { origin, normal }) = &tv.payload else {
        return Err(NodeErrorKind::WrongOperand {
            input: tool,
            expected: "datum plane",
            found: tv.payload.kind_name(),
        });
    };
    let plane = SplitPlane {
        origin: *origin,
        normal: normal.get(),
    };
    let result = split(&body, &plane, tol).map_err(NodeErrorKind::Split)?;
    // Pass-through descriptions keep their sources (the clone carried
    // them); the split's fresh section planes get THIS node's (D1).
    let side = |part: SplitPart<T>| match part {
        SplitPart::Body(mut b) => {
            stamp_minted(&mut b, id);
            SplitSide::Body(Arc::new(b))
        }
        SplitPart::Empty => SplitSide::Empty,
    };
    let above = side(result.above);
    let below = side(result.below);
    let as_body = |s: &SplitSide<T>| match s {
        SplitSide::Body(b) => Some(Arc::clone(b)),
        SplitSide::Empty => None,
    };
    let target_table = Arc::clone(&value_of(results, target)?.name_table);
    let (ab, bb) = (as_body(&above), as_body(&below));
    let table = names::name_split(
        id,
        ab.as_deref(),
        bb.as_deref(),
        &result.naming,
        target,
        &target_table,
        &body,
        normal.get(),
        tol,
    )
    .map_err(NodeErrorKind::Naming)?;
    Ok(OpOut::plain(ValuePayload::Split { above, below }, table))
}

// `Bounds` rides along for the boolean lane only (M5 PR 8): the sweep's
// BVH candidate generation reads coordinate brackets — the L7 driver-code
// allowance, threaded from `run_op`'s service bound.
#[allow(clippy::too_many_arguments)] // one parameter per named input; strategy is the §4.4 door
fn wire_boolean<T: Decide + geom_core::Bounds + geom_brep::PcurveFittedLane>(
    id: RecipeNodeId,
    op: BooleanOp,
    a: RecipeNodeId,
    b: RecipeNodeId,
    declare: Option<RecipeNodeId>,
    doc: &crate::doc::Doc<ProfileProgram>,
    results: &Results<T>,
    boolean_sweep: topo::SweepStrategy,
    tol: Tol,
) -> OpResult<T> {
    // F5 threading (M4 PR 5): the Declare input's name pairs resolve
    // through the OPERANDS' name tables into the kernel's declared
    // coincidence data. Resolution failures are the N5 typed errors —
    // no silent drop, no best-effort gluing.
    let mut kernel_decls = BooleanDeclarations::none();
    if let Some(d) = declare {
        let dv = value_of(results, d)?;
        let ValuePayload::Declarations(pairs) = &dv.payload else {
            return Err(NodeErrorKind::WrongOperand {
                input: d,
                expected: "declarations",
                found: dv.payload.kind_name(),
            });
        };
        let a_table = Arc::clone(&value_of(results, a)?.name_table);
        let b_table = Arc::clone(&value_of(results, b)?.name_table);
        kernel_decls = resolve_declarations(pairs, doc, &a_table, &b_table)?;
    }
    let body_a = body_operand(results, a)?;
    let body_b = body_operand(results, b)?;
    match topo::boolean_op_with(op, &body_a, &body_b, &kernel_decls, boolean_sweep, tol)
        .map_err(|err| refusal_menu(results, a, b, err))?
    {
        BooleanResult::Empty => Ok(OpOut::plain(
            ValuePayload::Boolean(BooleanValue::Empty),
            names::empty(),
        )),
        BooleanResult::Body(bb) => {
            let a_table = Arc::clone(&value_of(results, a)?.name_table);
            let b_table = Arc::clone(&value_of(results, b)?.name_table);
            let table = names::name_boolean(
                id,
                &bb.body,
                &bb.naming,
                &names::OperandCtx {
                    node: a,
                    table: &a_table,
                    body: &body_a,
                },
                &names::OperandCtx {
                    node: b,
                    table: &b_table,
                    body: &body_b,
                },
                tol,
            )
            .map_err(NodeErrorKind::Naming)?;
            let mut body = bb.body;
            // Seam chords / minted descriptions get THIS node's
            // sources; everything carried keeps its own (D1).
            stamp_minted(&mut body, id);
            Ok(OpOut::plain(
                ValuePayload::Boolean(BooleanValue::Body {
                    body: Arc::new(body),
                    kind: bb.kind,
                    contacts: Arc::new(bb.contacts),
                }),
                table,
            ))
        }
    }
}

/// The refusal-menu lift (register R3, LIB-PYG5; SELECT-DESIGN §3d):
/// a kernel [`topo::BooleanError::UndeclaredCoincidence`] becomes
/// [`NodeErrorKind::UndeclaredContact`] carrying the raise site's
/// face pair as the detector's own [`names::FlushFinding`] shape —
/// keys resolved to StableNames through the OPERANDS' name tables,
/// the ladder's decided relation carried through. NOTHING is
/// re-detected and no decide runs on this error path (the SEL2
/// rejection of post-hoc re-detection stands); every other
/// `BooleanError` wraps under [`NodeErrorKind::Boolean`] unaltered.
///
/// If either key resolves to no Face name — an emitter-coverage
/// invariant break (`vocabulary_coverage_is_total` pins coverage),
/// not an authoring state — the plain `Boolean` wrapping is
/// preserved: the boolean's refusal is never masked by its own menu.
fn refusal_menu<T: Decide>(
    results: &Results<T>,
    a: RecipeNodeId,
    b: RecipeNodeId,
    err: topo::BooleanError,
) -> NodeErrorKind {
    let topo::BooleanError::UndeclaredCoincidence {
        diag,
        pair,
        relation,
    } = err
    else {
        return NodeErrorKind::Boolean(err);
    };
    // The finding's contract orders the pair (a-side, b-side); the
    // raise sites order it by discovery. Relation is orientation-
    // symmetric, so the swap changes nothing else. A same-operand
    // pair (the F7 gate) keeps its raise order — both names resolve
    // in that one operand's table.
    let ordered = if pair[0].0 == topo::Operand::B && pair[1].0 == topo::Operand::A {
        [pair[1], pair[0]]
    } else {
        pair
    };
    let name_of = |(operand, face): (topo::Operand, topo::FaceKey)| {
        let node = match operand {
            topo::Operand::A => a,
            topo::Operand::B => b,
        };
        face_name(value_of(results, node).ok()?, face)
    };
    let (Some(na), Some(nb)) = (name_of(ordered[0]), name_of(ordered[1])) else {
        return NodeErrorKind::Boolean(topo::BooleanError::UndeclaredCoincidence {
            diag,
            pair,
            relation,
        });
    };
    NodeErrorKind::UndeclaredContact {
        finding: Box::new(names::FlushFinding {
            pair: (na, nb),
            class: names::ContactClass::Rest,
            evidence: names::FlushEvidence {
                relation,
                // Shared-source pairs never refuse Undeclared (rung 1
                // answers Ok), so the deciding rung here is always the
                // geometric one.
                rung: names::FlushRung::DecidedCoincident,
            },
        }),
        diag,
    }
}

/// The reverse of a table lookup: the FACE name denoting `face` in
/// one operand's value, or `None` (the caller's invariant-break
/// fallback). A boolean operand is single-body (`body_operand`
/// refused everything else), so within this value a face key
/// identifies its entity without a body check; a `Tied` entry
/// containing the key still DENOTES it (the tie is the table's
/// fact). Ties or multiple denoting names resolve to the canonical
/// least name — deterministic, and any denoting name identifies the
/// pair for the declare arm.
fn face_name<T: Decide>(
    v: &super::NodeValue<T>,
    face: topo::FaceKey,
) -> Option<crate::names::StableName> {
    use crate::names::{EntityKey, EntityKind, EntityRef, Entry};
    let mut found: Option<&crate::names::StableName> = None;
    for (name, entry) in v.name_table.iter() {
        if name.kind != EntityKind::Face {
            continue;
        }
        let refs: &[EntityRef] = match entry {
            Entry::Unique(e) => core::slice::from_ref(e),
            Entry::Tied(t) => t,
        };
        if refs.iter().any(|ent| ent.key == EntityKey::Face(face))
            && found.is_none_or(|prev| name < prev)
        {
            found = Some(name);
        }
    }
    found.cloned()
}

/// Resolves one Declare payload's name pairs against the two operand
/// tables into the kernel's [`BooleanDeclarations`] (F5, M4 PR 5).
///
/// v1 vocabulary: cross-operand Face–Face pairs (coincident-plane
/// glue intents) and same-operand Vertex–Vertex / Vertex–Face pairs
/// (carried 3′ contacts). Everything else refuses typed. Resolution
/// scope is deliberately the OPERANDS' tables (spec D4: "resolve
/// through the operands' name tables") — a name minted elsewhere in
/// the document is Vanished HERE even if some other node still
/// carries it.
///
/// **Twinned with [`resolve_selection`]** (M6-5): the fillet's
/// selection resolves through the same [`ladder`], which owns rung
/// order and payload shapes. What stays here is this door's arity —
/// TWO operand tables, so a name carried by both is unresolvable
/// (`DeclareBothOperands`, ranked below the ladder's first rung) —
/// and its pair vocabulary.
fn resolve_declarations(
    pairs: &[((names::StableName, names::StableName), ContactClass)],
    doc: &crate::doc::Doc<ProfileProgram>,
    a_table: &NameTable,
    b_table: &NameTable,
) -> Result<BooleanDeclarations, NodeErrorKind> {
    use crate::names::EntityKey;
    use ladder::Landing;
    use topo::Operand;

    let resolve_one = |name: &names::StableName| -> Result<(Operand, EntityKey), NodeErrorKind> {
        let refused = |error| NodeErrorKind::DeclareResolve { error };
        // Rung 1 first, and not by convention: reading either table
        // needs the token `live` returns, so a dead minting node
        // refuses NodeGone before the side-picking below can run.
        let live = ladder::live(name, doc).map_err(refused)?;
        // Side-picking is this door's own. A name PRESENT in both
        // operands (unique or tied, either counts as present) is not
        // an N5 failure — it is this door declining to guess a side.
        let (op, landing) = match (
            ladder::landing(&live, a_table),
            ladder::landing(&live, b_table),
        ) {
            // In neither table: the side is arbitrary, and rung 3
            // refuses Vanished on the `Absent` carried through.
            (Landing::Absent, Landing::Absent) => (Operand::B, Landing::Absent),
            (Landing::Absent, b) => (Operand::B, b),
            (a, Landing::Absent) => (Operand::A, a),
            _ => {
                return Err(NodeErrorKind::DeclareBothOperands {
                    name: Box::new(name.clone()),
                });
            }
        };
        Ok((op, ladder::resolve(live, landing).map_err(refused)?.key))
    };

    let mut out = BooleanDeclarations::none();
    for ((n1, n2), class) in pairs {
        let class = *class;
        let (o1, k1) = resolve_one(n1)?;
        let (o2, k2) = resolve_one(n2)?;
        let unsupported = || NodeErrorKind::DeclareUnsupportedPair {
            kinds: (n1.kind, n2.kind),
            cross_operand: o1 != o2,
        };
        match ((o1, k1), (o2, k2)) {
            // Cross-operand face pair: the coincident-plane intent.
            ((Operand::A, EntityKey::Face(fa)), (Operand::B, EntityKey::Face(fb)))
            | ((Operand::B, EntityKey::Face(fb)), (Operand::A, EntityKey::Face(fa))) => {
                out.coincident_faces
                    .push(FacePairDeclaration::new(fa, fb, class));
            }
            // Same-operand carried contacts.
            ((oa, EntityKey::Vertex(va)), (ob, EntityKey::Vertex(vb))) if oa == ob => {
                let c: &mut CarriedContacts = match oa {
                    Operand::A => &mut out.carried_a,
                    Operand::B => &mut out.carried_b,
                };
                // The AUTHORED class, carried — not re-defaulted. The
                // whole point of the payload change is that this door
                // no longer has to guess.
                c.vv.push(CarriedVv {
                    pair: VvContact { a: va, b: vb },
                    class,
                });
            }
            ((oa, EntityKey::Vertex(v)), (ob, EntityKey::Face(f)))
            | ((ob, EntityKey::Face(f)), (oa, EntityKey::Vertex(v)))
                if oa == ob =>
            {
                let c: &mut CarriedContacts = match oa {
                    Operand::A => &mut out.carried_a,
                    Operand::B => &mut out.carried_b,
                };
                c.vf.push(CarriedVf {
                    rest: VfContact { vertex: v, face: f },
                    class,
                });
            }
            _ => return Err(unsupported()),
        }
    }
    Ok(out)
}

fn wire_transform<T: Decide + geom_brep::PcurveFittedLane>(
    id: RecipeNodeId,
    input: RecipeNodeId,
    results: &Results<T>,
    vals: &SlotValues<T>,
    tol: Tol,
) -> OpResult<T> {
    let body = body_operand(results, input)?;
    let translation = need_vec3(vals, SlotId::Translation)?;
    let rot_axis = unit(
        need_vec3(vals, SlotId::RotationAxis)?,
        "transform rotation axis",
        band(tol)?,
    )?;
    let angle = need_scalar(vals, SlotId::RotationAngle)?;
    // PR 1's die convention: rotate about the axis THROUGH THE WORLD
    // ORIGIN by `angle`, then translate.
    let map = Affine3::from_parts(Mat3::rotation_about(rot_axis, angle), translation);
    let mut placed = transform_rigid(&body, &map, tol).map_err(NodeErrorKind::Transform)?;
    // N6 composition: `transform_rigid` cleared the source records
    // (its geometric rewrite invalidates the bit-identity claim); the
    // recipe layer re-stamps each description with the INPUT's source
    // wrapped by this placing node (keys are stable across the op).
    // Unsourced input descriptions stay unsourced — never invented.
    compose_placed(&body, &mut placed, id, 0);
    // Identity-preserving pass-through (spec D2): the transform
    // contributes NO RolePath segment — `transform_rigid` is
    // key-stable (arenas rewritten in place of a clone), so the
    // input's table rows hold verbatim: same names, same keys, the
    // N1 derivation-path semantics (the name still points at the
    // MINTING node; the placement is recipe context, not identity).
    let table = Arc::clone(&value_of(results, input)?.name_table);
    Ok(OpOut::plain(ValuePayload::Body(Arc::new(placed)), table))
}

/// The resolved operands of a stepped placement rule: what the rule's
/// math consumes once every slot or expression is evaluated and every
/// direction is unit (through [`unit()`]'s decided normalization).
pub(crate) enum SteppedOperands<T: geom_core::Real> {
    /// A linear rule: unit direction, spacing per step.
    Linear {
        /// The stepping direction, already unit.
        direction: Vec3<T>,
        /// The per-step translation distance along it.
        spacing: T,
    },
    /// A circular rule: the datum axis and the angle per step.
    Circular {
        /// A point on the rotation axis.
        origin: Point3<T>,
        /// The axis direction, already unit.
        dir: Vec3<T>,
        /// The rotation angle per step.
        step: T,
    },
}

/// The rigid map of placement `i` under a STEPPED rule (linear or
/// circular) — **the one home of the stepped placement rule's math**,
/// read by both placement-rule nodes (through [`stepped_map`]) and by
/// the mate solve's derived-offset derivation, so a pattern, a placed
/// union, and a mate to a pattern-placed member all derive one and the
/// same copy map, bit for bit.
///
/// Index 0 is the identity by construction (`i = 0` scales the step to
/// zero), which is why callers may take the prototype VERBATIM as
/// instance 0 rather than mapping it. `i as f64` is exact far beyond
/// any representable pattern (2^53).
pub(crate) fn stepped_rule_map<T: Decide>(ops: &SteppedOperands<T>, i: i64) -> Affine3<T> {
    let step = T::from_f64(i as f64);
    match ops {
        SteppedOperands::Linear { direction, spacing } => {
            Affine3::translation(*direction * (*spacing * step))
        }
        SteppedOperands::Circular {
            origin,
            dir,
            step: angle,
        } => Affine3::rotation_about_axis(*origin, *dir, *angle * step),
    }
}

/// [`stepped_rule_map`] behind the evaluation's slot reads. Slot reads
/// stay INSIDE this function so a rule's operands are demanded exactly
/// when a step actually uses them.
fn stepped_map<T: Decide>(
    kind: &PatternKind,
    i: i64,
    results: &Results<T>,
    vals: &SlotValues<T>,
    tol: Tol,
) -> Result<Affine3<T>, NodeErrorKind> {
    let ops = match kind {
        PatternKind::Linear { .. } => SteppedOperands::Linear {
            direction: unit(
                need_vec3(vals, SlotId::Direction)?,
                "pattern direction",
                band(tol)?,
            )?,
            spacing: need_scalar(vals, SlotId::Spacing)?,
        },
        PatternKind::Circular { axis, .. } => {
            let av = value_of(results, *axis)?;
            let ValuePayload::Datum(DatumValue::Axis { origin, dir }) = &av.payload else {
                return Err(NodeErrorKind::WrongOperand {
                    input: *axis,
                    expected: "datum axis",
                    found: av.payload.kind_name(),
                });
            };
            SteppedOperands::Circular {
                origin: *origin,
                dir: dir.get(),
                step: need_scalar(vals, SlotId::Step)?,
            }
        }
        // An explicit rule steps nothing: its frames ARE the maps, and
        // a caller that reached here read the rule wrong.
        PatternKind::Explicit(_) => {
            return Err(NodeErrorKind::PlacementRule(
                crate::node::PlacementRuleFault::CountSpelling,
            ));
        }
    };
    Ok(stepped_rule_map(&ops, i))
}

fn wire_pattern<T: Decide + geom_brep::PcurveFittedLane>(
    id: RecipeNodeId,
    input: RecipeNodeId,
    kind: &PatternKind,
    results: &Results<T>,
    vals: &SlotValues<T>,
    tol: Tol,
) -> OpResult<T> {
    // A pattern's count is its structural SLOT; an explicit placement
    // list would be a second answer to the same question, which the
    // edit door refuses — this is the same refusal, reached only by a
    // hand-built document.
    if kind.placements().is_some() {
        return Err(NodeErrorKind::PlacementRule(
            crate::node::PlacementRuleFault::CountSpelling,
        ));
    }
    let body = body_operand(results, input)?;
    let n = slots::count(vals, SlotId::Count).ok_or(NodeErrorKind::MissingSlot {
        slot: SlotId::Count,
    })?;
    if n < 1 {
        return Err(NodeErrorKind::NonPositiveCount { count: n });
    }
    let mut instances = Vec::new();
    // Instance 0 is the input body itself (identity placement, no op
    // re-run — `stepped_map` at i = 0 IS the identity).
    instances.push(Arc::clone(&body));
    for i in 1..n {
        let map = stepped_map(kind, i, results, vals, tol)?;
        let mut placed = transform_rigid(&body, &map, tol).map_err(NodeErrorKind::Transform)?;
        // N6 composition, per structural instance (`Placed { node,
        // instance: i, .. }`): distinct instances are distinct
        // sources — their descriptions genuinely differ.
        compose_placed(&body, &mut placed, id, i as u32);
        instances.push(Arc::new(placed));
    }
    // Instance(i) wrapping (A8/N1): every master entity name wraps
    // per structural index; `transform_rigid` key-stability means
    // instance keys equal master keys.
    let master = Arc::clone(&value_of(results, input)?.name_table);
    let table = names::name_pattern(id, &master, n, &instances).map_err(NodeErrorKind::Naming)?;
    Ok(OpOut::plain(ValuePayload::Instances(instances), table))
}

/// The group boolean (GROUP-BOOLEAN-DESIGN, ratified A′): one
/// prototype, a placement rule, ONE body out.
///
/// Three steps, in this order and no other:
///
/// 1. **The maps**, in placement order (D9) — a stepped rule's per-index
///    map ([`stepped_map`], shared with the pattern node), or the
///    listed frames verbatim.
/// 2. **The certificate**, BEFORE anything is built: one
///    [`topo::Separation`] over the prototype, queried per placement
///    pair. Disjointness is certified, never declared — the graft door
///    this lowers through asserts nothing about its operands (#382),
///    so an unproved arrangement refuses typed rather than shipping a
///    body whose solids may interpenetrate. Nothing is placed until
///    the certificate holds, so a refusal costs one tree, not N
///    transformed bodies.
/// 3. **The lowering**: `graft_disjoint_all_keyed` per placed copy, in
///    placement order, into one aggregate. No new kernel op and no new
///    kernel naming record — `BooleanNaming` stays two-operand,
///    because no seam happens here.
///
/// Every placement is MAPPED, including index 0 — unlike the pattern
/// node, which may hand back the prototype verbatim for its identity
/// instance, a placed union has no reason to special-case a map that an
/// explicit rule need not make the identity.
fn wire_placed_union<T: Decide + geom_core::Bounds + geom_brep::PcurveFittedLane>(
    id: RecipeNodeId,
    input: RecipeNodeId,
    kind: &PatternKind,
    fault: Option<crate::node::PlacementRuleFault>,
    results: &Results<T>,
    vals: &SlotValues<T>,
    tol: Tol,
) -> OpResult<T> {
    // The rule gate, FIRST and through the node's own door — the same
    // one `apply` and the snapshot check read, so an empty placement
    // list, a non-finite frame or an improper one refuses HERE with its
    // own name rather than downstream as a poison-box separation
    // "failure" or a kernel rigidity refusal (review MAJOR-1/MINOR-2).
    // Unreachable through `apply`; this is the hand-built-document
    // backstop.
    if let Some(fault) = fault {
        return Err(NodeErrorKind::PlacementRule(fault));
    }
    let body = body_operand(results, input)?;
    let maps: Vec<Affine3<T>> = match kind.placements() {
        Some(frames) => frames.iter().map(|f| f.affine::<T>()).collect(),
        None => {
            let n = slots::count(vals, SlotId::Count).ok_or(NodeErrorKind::MissingSlot {
                slot: SlotId::Count,
            })?;
            if n < 1 {
                return Err(NodeErrorKind::NonPositiveCount { count: n });
            }
            (0..n)
                .map(|i| stepped_map(kind, i, results, vals, tol))
                .collect::<Result<_, _>>()?
        }
    };
    topo::Separation::of(body.as_ref(), tol)
        .map_err(NodeErrorKind::Boolean)?
        .certify(&maps)
        .map_err(|topo::PlacementsMeet { i, j }| NodeErrorKind::PlacementsUncertified { i, j })?;
    let mut fused = topo::Body::new();
    let mut bridges: Vec<topo::GraftKeys> = Vec::with_capacity(maps.len());
    let mut targets: Vec<topo::SolidKey> = Vec::new();
    for (i, map) in maps.iter().enumerate() {
        let mut placed = transform_rigid(&body, map, tol).map_err(NodeErrorKind::Transform)?;
        // N6 composition, per structural instance — the pattern node's
        // rule verbatim: distinct instances are distinct sources.
        compose_placed(&body, &mut placed, id, i as u32);
        // Placement 0 MINTS the destination solids (one per prototype
        // solid, provenance carried); every later placement grafts ONTO
        // those same solids, so the fused body has the prototype's own
        // solid structure with N shells in each. That is what a union
        // of separated bodies already means here — the pairwise
        // `Boolean(Union)` chain this node replaces produces exactly
        // that shape, which is also the only shape the seamed boolean
        // path accepts as an operand.
        let keys = if i == 0 {
            let keys = topo::graft_disjoint_all_keyed(&mut fused, &placed, tol)
                .map_err(NodeErrorKind::Boolean)?;
            targets = keys.solids().to_vec();
            keys
        } else {
            topo::graft_disjoint_all_onto_keyed(&mut fused, &targets, &placed, tol)
                .map_err(NodeErrorKind::Boolean)?
        };
        bridges.push(keys);
    }
    // Instance(i) wrapping (A8/N1), re-keyed onto the ONE output body
    // through each instance's graft bridge.
    let master = Arc::clone(&value_of(results, input)?.name_table);
    let table =
        names::name_placed_union(id, &master, &bridges, &fused).map_err(NodeErrorKind::Naming)?;
    Ok(OpOut::plain(ValuePayload::Body(Arc::new(fused)), table))
}

// ---------------------------------------------------------------------
// M5 PR 10: the definitional §10.3/§10.4 nodes
// ---------------------------------------------------------------------

/// The Sweep node's frontier — the ONE remaining
/// [`NodeErrorKind::CurvedSolidFrontier`] door (M5 PR 10 fix pass,
/// review MAJOR-1; narrowed at M6-3 when the loft body landed and the
/// former `LOFT_FRONTIER` text retired with its frontier). Kept as a
/// constant so the acceptance rows assert the SAME text the node
/// produces.
///
/// §10.4's rigid-profile sweep needs the path as ONE curve. The recipe
/// layer cannot supply one: a `Node::Sweep`'s `path` operand is a
/// profile, a validated profile's loop is a CLOSED chain, and a closed
/// chain has two or more segments — even the minimal two-vertex loop is
/// two half-turn arcs. So there is no recipe-expressible path, and the
/// honest node-layer answer is a single refusal naming what is
/// missing: a joined-path composition lane (banked past M6 — the
/// PR 10 MAJ ruling, reaffirmed by the M6-3 spec §1).
///
/// `sweep::sweep_geometry` AND `sweep::sweep_body` are live and
/// exercised through the library API; it is only this NODE lane that
/// is gated, at one door, so the message cannot imply an expressible
/// case that does not exist.
///
/// **Honest correction (#207).** That sentence was written at M6-3 and
/// was NOT true as written until #207 closed. `sweep_body` with any
/// CURVED path refused at assembly — the skin fit synthesized a weight
/// channel for integral sections, the walls came out bitwise rational,
/// and `nurbs_span_meter` poisoned (the meter had no rational arm
/// then; M7's rational span meter has since given it one, so a
/// rational wall is no longer fatal on its own) — so between M6-3 and
/// #207 the machinery had zero successful curved-path callers anywhere
/// in the tree, and only the straight-path/uniform-loft slice was
/// exercised. The claim stands today on a real caller:
/// `sweep/tests/m7_skin_integral.rs` builds, validates and measures a
/// quarter-torus elbow, and `step-export/tests/m7_swept_elbow.rs` puts
/// it on the wire.
pub(crate) const SWEEP_FRONTIER: &str = "a swept solid: the recipe's path operand is a profile LOOP — always \
     a closed chain of two or more segments, even at the minimal \
     two-vertex circle — while §10.4's rigid-profile sweep needs the \
     path as ONE curve, so every recipe-expressible sweep waits on a \
     joined-path composition lane; the swept BODY machinery itself is \
     live — sweep::sweep_body at the library API";

/// One section of a loft, taken from the RECIPE's own `f64`
/// description rather than from the evaluated `T` payload.
///
/// Structure selection is `f64` (C6/D9): the skinned surface's knots,
/// degrees, and control bits must be identical in every scalar lane,
/// and every lane's profile is the same stored `f64` description
/// embedded through `from_f64`. Taking the description directly is
/// therefore not a shortcut — it is the only way the Interval lane
/// encloses the SAME surface the `f64` lane defines.
fn section_of<T: Decide + geom_core::Bounds>(
    doc: &crate::doc::Doc<ProfileProgram>,
    id: RecipeNodeId,
    lane: LaneEnv<'_, T>,
    tol: Tol,
) -> Result<(sweep::Section, Affine3<f64>, ProfileNaming), NodeErrorKind> {
    let Some(Node::Profile(program)) = doc.nodes.get(&id) else {
        return Err(NodeErrorKind::WrongOperand {
            input: id,
            expected: "profile node",
            found: "not a profile node",
        });
    };
    // LIB-SWITCH §4b at the loft/sweep seam: the section is the
    // node's program RESOLVED at f64 and REPLAYED — the same C6/D9
    // pipeline the profile node runs. The profile's own validation
    // door still runs first, so a bad section reads as a profile
    // error at the NODE (the §2 compatibility contract) before the
    // library door re-gates it — and the f64 canonical form yields
    // the program-anchor naming map for the loft emitter's refs.
    let resolved = program
        .resolve(&doc.param_env::<f64>())
        .map_err(|(slot, source)| NodeErrorKind::Expr { slot, source })?;
    // The f64 ladder is `prepare_profile` ITSELF, not a copy of it:
    // this seam and the profile node's used to run the same four steps
    // side by side, and two copies of a pipeline are two places for a
    // gate to be added to only one. Sharing the function is what makes
    // "the duplicate ladder did not fork" a fact about the code rather
    // than a claim about two diffs.
    let pre = prepare_profile(program, &resolved, tol)?;
    // The lift's second pass runs HERE TOO, as a GATE. A loft's or a
    // sweep's section stays f64 — the skinned surface's knots, degrees
    // and control bits must be identical in every lane, which is the
    // C6/D9 argument above and is untouched — but the certify-or-abort
    // answer must not depend on which node consumes the profile. A
    // parameter box the extrude ladder refuses to certify is refused
    // here as well, and by the same predicate.
    if lane.lift == super::ProfileLift::Guided {
        lane_profile::<T>(program, lane, &pre, tol)?;
    }
    let place = pre.profile_f64.plane.placement;
    // The sections are the REPLAYED loops (program order — exactly
    // the stored-loop handoff LIB-U3 established, one derivation
    // earlier): positions, bulges, declared joints verbatim.
    Ok((pre.profile_f64.loops, place, pre.naming))
}

/// A structural (Count) slot, refused typed when absent or unusable.
fn need_count(vals: &SlotValues<impl Decide>, slot: SlotId) -> Result<usize, NodeErrorKind> {
    let n = slots::count(vals, slot).ok_or(NodeErrorKind::MissingSlot { slot })?;
    usize::try_from(n).map_err(|_| NodeErrorKind::NonPositiveCount { count: n })
}

/// The Loft node (M6-3: the frontier flipped to the BUILDER — the
/// §10.3 walls plus the M5-LOG item-6 assembly, tiers 1–3 green at
/// rest).
fn wire_loft<T: Decide + geom_brep::PcurveFittedLane + geom_core::Bounds>(
    id: RecipeNodeId,
    profiles: &[RecipeNodeId],
    doc: &crate::doc::Doc<ProfileProgram>,
    vals: &SlotValues<T>,
    lane: LaneEnv<'_, T>,
    tol: Tol,
) -> OpResult<T> {
    let v_degree = need_count(vals, SlotId::VDegree)?;
    let mut sections = Vec::with_capacity(profiles.len());
    let mut places = Vec::with_capacity(profiles.len());
    let mut first_naming = ProfileNaming::default();
    for (i, pid) in profiles.iter().enumerate() {
        let (chain, place, naming) = section_of::<T>(doc, *pid, lane, tol)?;
        sections.push(chain);
        places.push(place);
        if i == 0 {
            // The loft emitter's profile refs are canonical (loop,
            // segment) indices of the SECTION combinatorics; sections
            // must correspond, so the FIRST section's anchor is the
            // rewrite for the emitted table. PINNED LIMITATION
            // (reported; review NOTE): a later section authored
            // rotated/reversed relative to section 0 anchors to
            // section 0's map, not its own — acceptable while the
            // kernel requires corresponding sections; revisit if loft
            // ever accepts per-section reparametrization.
            first_naming = naming;
        }
    }
    // The geometry/profile doors keep their historical node-error
    // shapes (the §2 compatibility contract predates the builder);
    // assembly-proper refusals arrive as the M6-3 `Loft` kind.
    let mut built =
        sweep::loft_body::<T>(&sections, &places, v_degree, tol).map_err(|e| match e {
            sweep::LoftError::Skin(s) => NodeErrorKind::Skin(s),
            sweep::LoftError::Profile(p) => NodeErrorKind::Profile(p),
            other => NodeErrorKind::Loft(other),
        })?;
    // Eager N4 emission from the builder's own maps, BEFORE the
    // structural handoff is dropped (the extrude idiom).
    let table = names::name_loft(id, &built).map_err(NodeErrorKind::Naming)?;
    let table = anchored(table, &first_naming)?;
    stamp_minted(&mut built.body, id);
    Ok(OpOut::plain(
        ValuePayload::Body(Arc::new(built.body)),
        table,
    ))
}

/// The Sweep node (M5 PR 10 fix pass, review MAJOR-1: ONE honest
/// arm).
///
/// Every RECIPE door still runs first — the structural slots, and both
/// operands through [`section_of`] — because a Sweep on a datum, or
/// with a bad Count, is a recipe error and must read as one. What the
/// node cannot do is reach the geometry: see [`SWEEP_FRONTIER`] for
/// why no recipe-expressible path exists to sweep.
fn wire_sweep<T: Decide + geom_core::Bounds>(
    profile: RecipeNodeId,
    path: RecipeNodeId,
    doc: &crate::doc::Doc<ProfileProgram>,
    vals: &SlotValues<T>,
    lane: LaneEnv<'_, T>,
    tol: Tol,
) -> OpResult<T> {
    let _stations = need_count(vals, SlotId::Stations)?;
    let _v_degree = need_count(vals, SlotId::VDegree)?;
    let _ = section_of::<T>(doc, profile, lane, tol)?;
    let _ = section_of::<T>(doc, path, lane, tol)?;
    Err(NodeErrorKind::CurvedSolidFrontier {
        what: SWEEP_FRONTIER,
    })
}
