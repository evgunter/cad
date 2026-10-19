//! **Four jobs**: node-to-kernel wiring, the mid-evaluation name
//! ladder, the union's fold, and the placement-rule arithmetic two
//! modules share.
//!
//! **The wiring** (spec D3: wire, don't invent). Each F4 node that
//! names a geometric operation maps to an EXISTING public kernel op;
//! every editor-side geometric judgment goes through the kernel's
//! decided-predicate door, never a raw comparison. Which value a node's
//! operand may be, and what it is told when it is not, is one door
//! ([`operand`]) speaking one vocabulary ([`super::family`],
//! [`super::phrase`]).
//!
//! **The mid-evaluation name ladder** ([`ladder`]). A name a slot reads
//! is a selection's ([`crate::VarDef::Select`]), and [`select`] is where
//! it is resolved, against the tables THIS run has built so far, in the
//! order [`ladder`] gives, into a key of the kind the selection holds.
//!
//! **The union's fold.** A union judges every member pair pairwise
//! before its fold and hands each fold step the pass's verdicts, read
//! through member lineage ([`given_pairs`]), with each operand's own
//! records carried in; the kernel takes them as [`BooleanInputs`].
//!
//! **The placement-rule arithmetic.** [`transform_map`],
//! [`SteppedOperands`], [`stepped_rule_map`] and the direction-role
//! words beside them ([`TRANSFORM_AXIS_ROLE`], [`DATUM_AXIS_ROLE`];
//! a linear rule's [`PATTERN_DIRECTION_ROLE`] is read inside its
//! constructor) are the ONE spelling of "where does a placer put
//! instance `i`". They are
//! `pub(crate)` because the mate solve's derived offset
//! (`crate::mate::member`) re-derives a placer's map from the recipe
//! and must get the same affine and the same refusal words as the
//! evaluation does.

use std::collections::BTreeMap;
use std::sync::Arc;

use geom_core::{
    Affine3, Band, Decide, OrthoAxis, Point2, Point3, RigidMotion, Sign, Tol, UnitVec3,
    UnitVec3Error, Vec2, Vec3,
};
use sweep::{Revolution, RevolveAxis};
use topo::splitting::SplitPart;
use topo::transform::transform_rigid;
use topo::{Body, BooleanInputs, DATUM_UNIT_NORM};

use super::anchor::{self, ProfilePre, ProfileValue};
use super::slots::{self, SlotValues};
use super::{BooleanValue, NodeErrorKind, NodeResult, PoseValue, SplitSide, ValuePayload};
use crate::decision::Decision;
use crate::names::{self, NameTable, SplitHalf};
use crate::node::{Axis3, Node, PatternKind, RecipeNodeId, SlotId};
use crate::program::ProfileProgram;
use topo::BooleanOp;

type Results<T> = BTreeMap<RecipeNodeId, NodeResult<T>>;

/// An op's product: the payload, its eagerly-emitted name table (N4;
/// emission lives in the wire layer, spec D4), and the DECLARED CONTACT
/// RECORDS the value carries (ASM-R2b D-1).
///
/// # The contacts channel (D-1)
///
/// Declared records are keyed in the op's OUTPUT BODY 0 arena. Only
/// [`wire_instantiate_part`] fills the field, carrying the referenced
/// part's records across the document seam. A boolean's records ride
/// its payload instead ([`BooleanValue::Body::contacts`]);
/// `product::sources_of` is the one place the two homes reconcile.
///
/// **Invariant**: a multi-output op (`Split`, `Pattern`) never fills
/// this field — "output body 0" would be a lie for its other bodies.
pub(crate) struct OpOut<T: Decide> {
    pub payload: ValuePayload<T>,
    /// Each output's names, by port (`NodeValue::outputs`): one for an
    /// op with one body output, a split's two halves apart; the
    /// fragment groups beside them empty for an op that forms none.
    pub outputs: Vec<super::OutputNames>,
    pub contacts: Arc<topo::ContactRecords>,
    /// Whose mate authored each of those records, and which mates of
    /// the documents below could not be minted at all; same arena, same
    /// one op.
    pub carried: Arc<crate::assembly::CarriedDeclarations>,
    /// How many parts each output body is (`NodeValue::parts`).
    pub parts: usize,
    /// The coincidences the op decided from values, named in its
    /// inputs' tables (`NodeValue::coincidences`).
    pub coincidences: Arc<[crate::coincide::NamedCoincidence]>,
    /// What each input a carried record cites is
    /// (`NodeValue::cited_inputs`).
    pub cited_inputs: Arc<[crate::coincide::CitedInput]>,
    /// A placement's solved motion (`NodeValue::motion`).
    pub motion: Option<RigidMotion<T>>,
    /// A placement's space (`NodeValue::space`), `None` for every other
    /// op, whose value lives in its inputs'.
    pub space: Option<crate::mate::Space>,
}

impl<T: Decide> OpOut<T> {
    /// An op that declares no contact — every op but instantiate.
    fn plain(payload: ValuePayload<T>, names: Arc<NameTable>) -> Self {
        Self {
            payload,
            outputs: vec![super::OutputNames::new(names, Arc::default())],
            contacts: Arc::new(topo::ContactRecords::default()),
            carried: Arc::new(crate::assembly::CarriedDeclarations::default()),
            parts: 1,
            coincidences: Arc::new([]),
            cited_inputs: Arc::new([]),
            motion: None,
            space: None,
        }
    }

    /// This output, a placement's, sitting at `motion` in `space`.
    fn moved(self, (motion, space): (Option<RigidMotion<T>>, crate::mate::Space)) -> Self {
        Self {
            motion,
            space: Some(space),
            ..self
        }
    }

    /// This output, its records' carried citations naming `inputs` by
    /// position.
    fn citing(self, inputs: Vec<crate::coincide::CitedInput>) -> Self {
        Self {
            cited_inputs: inputs.into(),
            ..self
        }
    }

    /// This output with coincidence rows already named.
    fn with_coincidences(self, coincidences: Vec<crate::coincide::NamedCoincidence>) -> Self {
        Self {
            coincidences: coincidences.into(),
            ..self
        }
    }

    /// This output with the kernel's coincidence rows, named in the
    /// op's inputs' tables.
    fn recording(
        self,
        rows: &[topo::Coincidence],
        inputs: &crate::coincide::RowInputs<'_>,
    ) -> Result<Self, NodeErrorKind> {
        let named = crate::coincide::name_rows(rows, inputs).map_err(NodeErrorKind::Naming)?;
        Ok(self.with_coincidences(named))
    }

    /// This output, each body of it `parts` parts: a placer's or a
    /// projection's output carries its input's count through.
    fn carrying(self, parts: usize) -> Self {
        Self { parts, ..self }
    }
}

impl<T: Decide> OpOut<T> {
    /// This output with the fragment groups its emitter formed beside
    /// its one table.
    fn grouped(self, groups: Arc<names::FragmentGroups>) -> Self {
        let outputs = self
            .outputs
            .iter()
            .map(|names| super::OutputNames::new(names.shared_table(), Arc::clone(&groups)))
            .collect();
        Self { outputs, ..self }
    }

    /// An op whose outputs carry names apart (a split's two halves):
    /// `outputs` by port.
    fn per_output(payload: ValuePayload<T>, outputs: Vec<super::OutputNames>) -> Self {
        Self {
            outputs,
            ..Self::plain(payload, names::empty())
        }
    }
}

type OpResult<T> = Result<OpOut<T>, NodeErrorKind>;

/// **The table a read holds**: the names of the one output the read
/// names, its value already resolved to that output (`reads_projected`
/// for a split's port or a family's member, the operation's own value
/// otherwise). Empty for an output that carries no names.
fn read_table<T: Decide>(value: &super::NodeValue<T>) -> Arc<NameTable> {
    value
        .output(0)
        .map_or_else(names::empty, super::OutputNames::shared_table)
}
/// A bare payload (datum/profile lanes — empty tables).
type PayloadResult<T> = Result<ValuePayload<T>, NodeErrorKind>;

/// The LANE half of an evaluation's environment: where profile
/// geometry comes from at `T`, and the parameter environments it is
/// elaborated over. They travel together so that no call site can
/// elaborate the lift over a different box than the node's slots were
/// evaluated at.
#[derive(Debug)]
pub(crate) struct LaneEnv<'a, T> {
    /// Where profile geometry comes from at this evaluation's scalar
    /// (M10-P PP5): the f64 elaboration embedded, or a guided
    /// elaboration at `T`.
    pub lift: super::ProfileLift,
    /// The evaluation's parameter environment — nominals, or nominals
    /// widened by [`crate::analysis::ParamBox`] (E6's leaf replay).
    pub params: &'a crate::expr::VarEnv<T>,
    /// The document's own f64 parameter environment, under no box and
    /// no seed, built once per evaluation beside `params`. Every
    /// f64-pinned decision the evaluation makes reads it — the nominal
    /// slots an authored frame's placement is minted from
    /// ([`mint_frame_placement`]), a profile's program resolution,
    /// the placement the pinned lift embeds — so it is what a content key owes
    /// ([`super::tag::slot`]). Nothing under evaluation builds a second
    /// one.
    pub nominal: &'a crate::expr::VarEnv<f64>,
}

impl<T> Clone for LaneEnv<'_, T> {
    fn clone(&self) -> Self {
        *self
    }
}

impl<T> Copy for LaneEnv<'_, T> {}

/// The evaluation-wide context an op may need beyond its own inputs:
/// the boolean candidate-generation switch, the document seam's part
/// table, and the lane environment — everything ambient to the
/// run, as opposed to the op's inputs and slots.
pub(crate) struct OpEnv<'a, T: Decide> {
    pub boolean_sweep: topo::SweepStrategy,
    pub parts: &'a super::parts::Reached<T>,
    /// Where profile geometry comes from, and over which environment.
    pub lane: LaneEnv<'a, T>,
}

/// Runs one node's op against its (already Ok) inputs and evaluated
/// slots, emitting the node's name table alongside the payload.
/// `profile_pre` is the profile node's f64 precompute (present exactly
/// for `Node::Profile` — computed in `eval_node`'s resolution stage,
/// inside the node's verdict frame and ahead of this op).
#[allow(clippy::too_many_arguments)] // the run-tolerance witness and the pre-key reads are not duties of their own
pub(crate) fn run_op<T>(
    id: RecipeNodeId,
    node: &Node<ProfileProgram>,
    doc: &crate::doc::Doc<ProfileProgram>,
    results: &Results<T>,
    vals: &SlotValues<T>,
    payload: Option<&super::Payload<T>>,
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
        + crate::analysis::AxisScalar
        + crate::analysis::SeedScalar
        + crate::measure::MinClearanceLane
        + crate::mate::SolveScalar,
{
    use crate::OperandSlot as O;
    let unprojected = results;
    // A boolean's, a union's and a measure's reads are projected one by
    // one: two ports of one split are two bodies, which one map keyed by
    // the split cannot hold. Every other node reads at most one body.
    let per_operand = matches!(
        node,
        Node::Subtract { .. }
            | Node::Union { .. }
            | Node::Intersect { .. }
            | Node::Measure { .. }
            | Node::Place { .. }
    );
    let projected = if per_operand {
        None
    } else {
        reads_projected(node, &body_rows(node, doc), doc, (results, vals), tol)?
    };
    let results = projected.as_ref().unwrap_or(results);
    let project_each = |rows: &[(O, crate::VarId)]| {
        rows.iter()
            .map(|&row| reads_projected(node, &[row], doc, (unprojected, vals), tol))
            .collect::<Result<Vec<_>, _>>()
    };
    // An operand reads an output; the op reads the operation's value.
    // Every read resolves here: an unresolved one refused the node
    // before its op was reached (`eval::read_at`).
    let at = |slot, var| super::read_at(doc, slot, var);
    match node {
        Node::Profile(program) => {
            let payload = wire_profile(program, profile_pre, env.lane, tol)?;
            let rows = match &payload {
                ValuePayload::Profile(value) => {
                    crate::coincide::name_junctions(id, &value.validated, &value.pieces)
                        .map_err(NodeErrorKind::Naming)?
                }
                _ => Vec::new(),
            };
            Ok(OpOut::plain(payload, names::empty()).with_coincidences(rows))
        }
        Node::Extrude { profile, .. } => {
            wire_extrude(id, at(O::Profile, *profile)?, results, vals, tol)
        }
        Node::Revolve { profile, .. } => {
            wire_revolve(id, at(O::Profile, *profile)?, results, vals, tol)
        }
        Node::Loft { first, then, .. } => {
            let sections = core::iter::once(*first)
                .chain(then.iter().map(|section| section.profile))
                .enumerate()
                .map(|(i, var)| at(O::Section(u32::try_from(i).unwrap_or(u32::MAX)), var))
                .collect::<Result<Vec<_>, _>>()?;
            let stations: Vec<&[crate::node::StationMove]> = then
                .iter()
                .map(|section| section.station.as_slice())
                .collect();
            wire_loft(id, &sections, &stations, results, vals, tol)
        }
        Node::Sweep { profile, path, .. } => {
            let (profile, path) = (at(O::Profile, *profile)?, at(O::Path, *path)?);
            wire_sweep(profile, path, results, vals)
        }
        Node::Tube { window, .. } => wire_tube(id, window, vals, tol),
        Node::HollowTube { window, .. } => wire_hollow_tube(id, window, vals, tol),
        Node::Fillet { selection, .. } => wire_blend(
            &crate::verbs::blend::fillet(),
            id,
            &select(doc, results, O::Selection, *selection)?,
            results,
            vals,
            tol,
        ),
        Node::Chamfer { selection, .. } => wire_blend(
            &crate::verbs::blend::chamfer(),
            id,
            &select(doc, results, O::Selection, *selection)?,
            results,
            vals,
            tol,
        ),
        Node::Shell { open, .. } => wire_shell(
            &crate::verbs::shell::shell(),
            id,
            &select(doc, results, O::Open, *open)?,
            results,
            vals,
            tol,
        ),
        Node::Split { target, tool } => wire_split(
            &crate::verbs::split::split(),
            id,
            (target.read, at(O::Target, target.read)?),
            (
                &pose::eval_pose(doc, unprojected, O::Tool, *tool, env.lane.params, tol)?,
                *tool,
            ),
            results,
            tol,
        ),
        Node::Subtract { from, tool } => {
            let each = project_each(&[(O::From, from.read), (O::Cut, tool.read)])?;
            let [ra, rb] = [&each[0], &each[1]].map(|p| p.as_ref().unwrap_or(unprojected));
            wire_subtract(
                &crate::verbs::boolean::boolean(),
                id,
                (from.read, at(O::From, from.read)?, ra),
                (tool.read, at(O::Cut, tool.read)?, rb),
                env.boolean_sweep,
                tol,
            )
        }
        Node::Union { members } | Node::Intersect { members } => {
            let op = if matches!(node, Node::Union { .. }) {
                BooleanOp::Union
            } else {
                BooleanOp::Intersect
            };
            wire_combine(
                &crate::verbs::boolean::boolean(),
                (op, node, members),
                id,
                (unprojected, vals),
                doc,
                env.boolean_sweep,
                tol,
            )
        }
        Node::Place {
            shapes,
            constraints,
        } => {
            // Each shape projected by itself: two halves of one split
            // are two bodies, which one map keyed by the split cannot
            // hold.
            let rows: Vec<(O, crate::VarId)> = shapes
                .iter()
                .enumerate()
                .map(|(i, shape)| (O::Shape(u32::try_from(i).unwrap_or(u32::MAX)), shape.read))
                .collect();
            let each = project_each(&rows)?;
            let reads = rows
                .iter()
                .zip(&each)
                .map(|(&(seat, read), projected)| {
                    Ok((
                        at(seat, read)?,
                        read,
                        projected.as_ref().unwrap_or(unprojected),
                    ))
                })
                .collect::<Result<Vec<_>, NodeErrorKind>>()?;
            let reach = reads
                .iter()
                .map(|&(at, _, values)| copy_reach(values, at))
                .try_fold(0.0_f64, |held, r| r.map(|r| held.max(r)));
            let placed = fold_with(
                doc,
                unprojected,
                unprojected,
                id,
                constraints,
                reach,
                env,
                tol,
            )?;
            wire_place(id, &reads, placed.0.as_ref(), tol).map(|out| out.moved(placed))
        }
        // The world DENOTES NO BODY and defines no variable.
        Node::World => unreachable!("the world is scheduled nowhere (`eval::schedule`)"),
        Node::Pattern { input, kind, .. } => {
            let input = at(O::Input, *input)?;
            let rule = RuleEnv {
                doc,
                written: &written(doc, id),
                params: env.lane.params,
                unprojected,
            };
            wire_pattern(id, input, kind, &rule, results, vals, tol)
        }
        // No `id`: the projection mints no description and no name, so
        // nothing it produces is stamped or keyed by this node.
        Node::Part { of, select: _ } => wire_part(at(O::Of, *of)?, results, vals),
        // The rule gate, FIRST, through the node's own door (the one
        // `apply` reads): a bad placement list refuses with its own
        // name rather than downstream as a separation or rigidity
        // refusal. Hand-built-document backstop.
        Node::PlacedUnion { input, kind, .. } => match node.placement_rule_fault(tol) {
            Some(fault) => Err(NodeErrorKind::PlacementRule(fault)),
            None => {
                let input = at(O::Input, *input)?;
                let rule = RuleEnv {
                    doc,
                    written: &written(doc, id),
                    params: env.lane.params,
                    unprojected,
                };
                wire_placed_union(id, input, kind, &rule, results, vals, tol)
            }
        },
        Node::Measure { primitive } => {
            let each = project_each(&body_rows(node, doc))?;
            let at = |reference: usize| {
                each.get(reference)
                    .and_then(Option::as_ref)
                    .unwrap_or(unprojected)
            };
            wire_measure(primitive, doc, &at, tol)
        }
        Node::Assertion { value, .. } => {
            // Which endpoints this assertion may read: structural, off
            // the measures under its value, so the same at every scalar.
            let certified = super::measure::certified(doc, *value);
            let reads = node.payload_reads(doc).unwrap_or_default();
            let (Some(payload), [(_, value_dim), (_, bound_dim)]) = (payload, reads.as_slice())
            else {
                unreachable!("an assertion reads its value and its bound, and both were evaluated")
            };
            wire_assertion(*value_dim, *bound_dim, certified, node, payload, tol)
        }
        Node::InstantiatePart {
            doc_ref,
            interface,
            constraints,
        } => {
            // The copy before placement is what its mates' `on` poses
            // read: the part's product where the part put it.
            let unplaced = wire_instantiate_part(id, doc_ref, interface, None, env, tol)?;
            let reach = payload_reach(&unplaced.payload, id);
            // `on` reads the copy alone (the door's `on_reads_the_copy`),
            // so the copy is all its reads need.
            let own: Results<T> =
                BTreeMap::from([(id, NodeResult::Ok(super::NodeValue::scratch(unplaced)))]);
            let placed = fold_with(doc, &own, unprojected, id, constraints, reach, env, tol)?;
            let out = wire_instantiate_part(id, doc_ref, interface, placed.0, env, tol)?;
            Ok(out.moved(placed))
        }
    }
}

/// **A copy's reach in its own coordinates** — a shape's body's, as
/// [`crate::mate::part_reach`] bounds a part's — for the fold's arm.
fn copy_reach<T: crate::mate::SolveScalar>(
    results: &Results<T>,
    at: RecipeNodeId,
) -> Result<f64, crate::mate::LeverRefusal> {
    match results.get(&at) {
        Some(NodeResult::Ok(value)) => payload_reach(&value.payload, at),
        _ => Ok(0.0),
    }
}

/// [`copy_reach`] of a value's payload: its body's, or the farthest of
/// its bodies' (copies, instances, a split's halves). A value that is no
/// body, or an empty boolean, reaches nowhere.
fn payload_reach<T: crate::mate::SolveScalar>(
    payload: &ValuePayload<T>,
    at: RecipeNodeId,
) -> Result<f64, crate::mate::LeverRefusal> {
    let farthest = |bodies: &mut dyn Iterator<Item = &topo::Body<T>>| {
        bodies
            .map(|body| body_reach(body, at))
            .try_fold(0.0_f64, |held, r| r.map(|r| held.max(r)))
    };
    match payload {
        ValuePayload::Body(body) | ValuePayload::Boolean(BooleanValue::Body { body, .. }) => {
            body_reach(body, at)
        }
        ValuePayload::Copies(copies) => farthest(&mut copies.bodies.iter().map(|b| &**b)),
        ValuePayload::Instances(bodies) => farthest(&mut bodies.iter().map(|b| &**b)),
        ValuePayload::Split { above, below } => farthest(
            &mut [above, below].into_iter().filter_map(|side| match side {
                SplitSide::Body(b) => Some(&**b),
                SplitSide::Empty => None,
            }),
        ),
        ValuePayload::Boolean(BooleanValue::Empty)
        | ValuePayload::Profile(_)
        | ValuePayload::Measure { .. }
        | ValuePayload::MeasureUnavailable { .. }
        | ValuePayload::Assertion(_) => Ok(0.0),
    }
}

/// [`copy_reach`] of one body, read off the node `at`.
fn body_reach<T: crate::mate::SolveScalar>(
    body: &topo::Body<T>,
    at: RecipeNodeId,
) -> Result<f64, crate::mate::LeverRefusal> {
    crate::mate::part_reach(body)
        .map(crate::mate::SolveScalar::upper)
        .and_then(|hi| {
            if hi.is_finite() {
                Ok(hi)
            } else {
                Err(crate::mate::ReachRefusal::NoFiniteBound)
            }
        })
        .map_err(|refusal| crate::mate::LeverRefusal::Shape { shape: at, refusal })
}

/// **A placement's bundle, folded where it evaluates** (INTENT stage 3
/// C), its `on` poses read in `on_results` (an instance's own copy
/// before placement among them) and its `to` poses in `to_results`.
/// Each mate's `on` is read off the copied shapes before placement, in
/// their own coordinates, and its `to` off the target space's geometry,
/// or the world read inside the mate; the bundle folded
/// ([`crate::mate::solve::solve_bundle`]). Answers the copy's motion in
/// its target space's coordinates (`None` the identity) and the space
/// it joins: a loose copy roots its own.
///
/// # Errors
///
/// [`NodeErrorKind::Mate`] with the bundle's fault.
#[allow(clippy::too_many_arguments)]
fn fold_with<T>(
    doc: &crate::doc::Doc<ProfileProgram>,
    on_results: &Results<T>,
    to_results: &Results<T>,
    id: RecipeNodeId,
    constraints: &[crate::mate::Constraint],
    reach: Result<f64, crate::mate::LeverRefusal>,
    env: &OpEnv<'_, T>,
    tol: Tol,
) -> Result<(Option<RigidMotion<T>>, crate::mate::Space), NodeErrorKind>
where
    T: crate::mate::SolveScalar,
{
    use crate::mate::{MateFault, MateRef, MateSide, Space, Target};
    let fault = |f: MateFault| NodeErrorKind::Mate(Box::new(f));
    let b = band(tol)?;
    let params = env.lane.params;
    let mut mates = Vec::with_capacity(constraints.len());
    let mut values = Vec::new();
    // Each mate's two poses, its target's sense and the frame of the
    // body its target is read off: what a value's chart reads.
    let mut charted = Vec::new();
    let mut target_space: Option<Space> = None;
    for c in constraints {
        let mate = MateRef::new(id, c.id);
        let m = match &c.kind {
            crate::mate::ConstraintKind::Mate(m) => m,
            crate::mate::ConstraintKind::Value(v) => {
                let slot = match v.coord {
                    crate::mate::Coord::Slide(_) => SlotId::ValueSlide { constraint: c.id },
                    crate::mate::Coord::Spin => SlotId::ValueSpin { constraint: c.id },
                };
                let by = crate::expr::eval_var(v.by, v.coord.dimension(), params)
                    .map_err(|source| NodeErrorKind::Expr { slot, source })?;
                values.push(crate::mate::value::ReadValue {
                    value: mate,
                    mate: v.mate,
                    coord: v.coord,
                    by,
                });
                continue;
            }
        };
        let unevaluated = |side, kind| {
            fault(MateFault::SideUnevaluated {
                mate,
                side,
                refusal: Box::new(super::NodeRefusal::from(kind)),
            })
        };
        let seat = |side| crate::OperandSlot::Side {
            constraint: c.id,
            side,
        };
        let on = pose::eval_pose(
            doc,
            on_results,
            seat(MateSide::On),
            crate::pose::RoleRead::plain(m.on),
            params,
            tol,
        )
        .map_err(|kind| unevaluated(MateSide::On, kind))?;
        let on = crate::mate::solve::tightened(on);
        let (to, space, body) = match &m.to {
            Target::Pose(var) => {
                let to = pose::eval_pose(
                    doc,
                    to_results,
                    seat(MateSide::To),
                    crate::pose::RoleRead::plain(*var),
                    params,
                    tol,
                )
                .map_err(|kind| unevaluated(MateSide::To, kind))?;
                let space = doc
                    .read_ports(*var)
                    .iter()
                    .find_map(|(at, _)| match to_results.get(at) {
                        Some(NodeResult::Ok(value)) => value.space,
                        _ => None,
                    })
                    .unwrap_or(Space::World);
                // The body's own frame: its placement's motion in the
                // space, the identity for the space's own geometry. Two
                // bodies refuse only where a value asks for the chart.
                let at = crate::mate::solve::chart_body(doc, *var);
                let body = at.clone().map(|body| {
                    body.and_then(|at| match to_results.get(&at) {
                        Some(NodeResult::Ok(value)) => value.motion,
                        _ => None,
                    })
                    .unwrap_or_else(geom_core::RigidMotion::identity)
                });
                // The target in the body's own coordinates, where the
                // body is a copy its placement moved: what a frame lane's
                // chart is read in.
                let own = at
                    .ok()
                    .flatten()
                    .filter(|_| T::spins_by_frames())
                    .and_then(|at| {
                        pose::own_direction(doc, to_results, crate::pose::RoleRead::plain(*var), at)
                    });
                (to, space, (body, own))
            }
            Target::World(world) => {
                world
                    .kind()
                    .map_err(|f| fault(MateFault::WorldRead { mate, fault: f }))?;
                let to = pose::eval_world(world, params, b)
                    .map_err(|kind| unevaluated(MateSide::To, kind))?;
                (
                    to,
                    Space::World,
                    (Ok(geom_core::RigidMotion::identity()), None),
                )
            }
        };
        match target_space {
            None => target_space = Some(space),
            Some(held) if held != space => {
                return Err(fault(MateFault::TargetsDiffer {
                    placement: id,
                    constraints: constraints.iter().map(|c| c.id).collect(),
                }));
            }
            Some(_) => {}
        }
        let to = crate::mate::solve::tightened(to);
        let flipped = crate::mate::solve::target_flipped(doc, &m.to);
        charted.push((c.id, on.clone(), to.clone(), flipped, body));
        mates.push(crate::mate::solve::ReadMate { mate, on, to });
    }
    if mates.is_empty() {
        // A value names a mate of its bundle; with none, its own.
        if let Some(v) = values.first() {
            return Err(fault(MateFault::ValueNamesNoMate {
                value: v.value,
                mate: v.mate,
            }));
        }
        return Ok((None, Space::Loose { root: id }));
    }
    let reach = reach.map_err(|refusal| {
        fault(MateFault::Unleverable {
            mate: mates[0].mate,
            refusal: Box::new(refusal),
        })
    })?;
    let arm = crate::mate::solve::bundle_arm(reach, &mates).map_err(|f| fault(*f))?;
    // A chart is read only for a mate a value names.
    let mut charts = std::collections::BTreeMap::new();
    for (mate, on, to, flipped, (body, own)) in charted {
        let Some(value) = values.iter().find(|v| v.mate == mate) else {
            continue;
        };
        let body = body.map_err(|why| {
            fault(MateFault::NoChart {
                value: value.value,
                why,
            })
        })?;
        charts.insert(
            mate,
            crate::mate::value::Chart::of(&on, &to, own.as_ref(), flipped, &body, arm, b)?,
        );
    }
    match crate::mate::solve::solve_bundle(
        id,
        &crate::mate::solve::equations_of(doc, constraints),
        &mates,
        &values,
        &charts,
    )? {
        crate::mate::solve::Folded::Loose => Ok((None, Space::Loose { root: id })),
        crate::mate::solve::Folded::Pinned(rep, rigid) => {
            // The one at-rest verdict, decided once (fork C-5). Where the
            // map is kept as the solve composed it, the motion the
            // composition made is the copy's.
            let placed = match crate::mate::solve::rest_verdict(&rep, reach, b) {
                Some(true) if T::carries_no_variation(&rep) => None,
                Some(true) => match T::identity_varying_as(&rep) {
                    Some(map) => Some((map, None)),
                    None => Some((rep, rigid.map(|c| (c, false)))),
                },
                // A box the band cannot decide at rest spans points the
                // value lane snaps and points it keeps: its map holds
                // both.
                None if T::holds_rest(&rep) => Some((rep, rigid.map(|c| (c, false)))),
                None => match rigid.and_then(|c| c.at_rest().map(|map| (c, map))) {
                    Some((c, map)) if T::holds_rest(&map) => Some((map, Some((c, true)))),
                    _ => Some((T::with_rest(&rep).unwrap_or(rep), None)),
                },
                Some(false) => Some((rep, rigid.map(|c| (c, false)))),
            };
            // A map the solve composed as motions is placed as the motion
            // it is; any other is admitted from its entries.
            let motion =
                |map, composed: Option<(crate::mate::value::Composed<T>, bool)>| match composed
                    .and_then(|(c, rest)| c.motion(rest, b))
                {
                    Some(motion) => Ok(Some(motion)),
                    None => crate::mate::solve::admitted(crate::mate::solve::marked(map), b),
                };
            Ok((
                match placed {
                    Some((map, composed)) => motion(map, composed)?,
                    None => None,
                },
                target_space.unwrap_or(Space::World),
            ))
        }
    }
}

/// ASM-2A D-3: materialize an instance through the shipped doors.
///
/// Read the part the evaluation entered, take the referenced document's A10
/// PRODUCT, place it with the kernel's own `transform_rigid`, and hand
/// back a body-denoting value. The placed body is an ordinary `Body` —
/// one solid or N, under ONE rigid map — so the root gather and the
/// export door consume it with no new arms.
fn wire_instantiate_part<T>(
    id: RecipeNodeId,
    doc_ref: &crate::ident::DocRef,
    interface: &crate::node::InterfaceRecord,
    map: Option<RigidMotion<T>>,
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
        + crate::analysis::AxisScalar
        + crate::analysis::SeedScalar
        + crate::measure::MinClearanceLane
        + crate::mate::SolveScalar,
{
    let part = env
        .parts
        .get(doc_ref, tol)
        .map_err(|fault| NodeErrorKind::Part {
            doc_ref: *doc_ref,
            fault,
        })?;
    // ASM-R2b D-4/D-5: the structural half of A4's fit gate (the
    // geometric half is `crate::assembly::assemble`). Every crossing
    // names an entity of the PART's product, so a pin move (A13 clause
    // 4) that changed the part's contact face arrives as a crossing
    // that no longer resolves. INVARIANT: this runs on every
    // evaluation, not only at the moving edit — an edit-time-only gate
    // would bless a document loaded from disk with a hand-moved pin.
    for crossing in &interface.crossings {
        let crate::node::InterfaceCrossing::Mate { outer, inner, .. } = crossing;
        if part.names.lookup(inner).is_none() {
            return Err(NodeErrorKind::CrossingUnverified {
                instance: id,
                outer: Box::new(outer.clone()),
                name: Box::new((**inner).clone()),
            });
        }
    }
    // The identity fast-path is the composition rule's
    // (`placement::Motion`): taken only for a BIT-exact identity, since
    // any other value could round, and `transform_rigid` is what
    // decides whether it stayed rigid.
    let placed = place(&part.body, map.as_ref(), tol)?;
    let table = names::name_in_part(id, &part.names, &placed).map_err(NodeErrorKind::Naming)?;
    // ASM-R2b D-1: the part's OWN declared contacts survive
    // instantiation with their cells UNCHANGED, because
    // `transform_rigid` is key-stable and the identity fast path clones
    // keys verbatim. Re-deriving them from the placed geometry is the
    // scan-to-bless move F1 bans. Each record cites the part's record
    // it is, through this instance ([`crate::coincide::CitedInput::Part`]).
    // The bookkeeping rows ride unchanged for the same reason; what is
    // added here is each row's first hop, this instance
    // ([`crate::assembly::PartRow::through`]).
    let carried = crate::assembly::CarriedDeclarations {
        refused: part.refused.as_ref().clone(),
        minted: part
            .minted
            .iter()
            .map(|row| {
                let (route, declaration, held) = row.through(id);
                crate::assembly::CarriedDeclaration {
                    route,
                    declaration,
                    held,
                }
            })
            .collect(),
        unminted: part
            .unminted
            .iter()
            .map(|row| {
                let (route, refusal, held) = row.through(id);
                crate::assembly::CarriedRefusal {
                    route,
                    refusal,
                    held,
                }
            })
            .collect(),
    };
    // The same material copy by copy, under the same map, for a reader
    // of one copy: each named as the instance names it, and placed when
    // that reader reads it (`KeyedCopy::motion`). A rigid map is
    // key-stable, so the table names the bodies wherever they stand.
    let members = part
        .copies
        .iter()
        .map(|copy| {
            let table = names::name_in_part_copy(id, &copy.table, &copy.bodies)
                .map_err(NodeErrorKind::Naming)?;
            let motion = match (map, copy.motion) {
                (None, inner) => inner,
                (Some(outer), None) => Some(outer),
                (Some(outer), Some(inner)) => {
                    Some(outer.compose(inner).map_err(NodeErrorKind::Motion)?)
                }
            };
            Ok(super::KeyedCopy {
                key: copy.key.clone(),
                bodies: copy.bodies.clone(),
                family: copy.family,
                parts: copy.parts,
                table,
                motion,
            })
        })
        .collect::<Result<Vec<_>, NodeErrorKind>>()?;
    Ok(OpOut {
        payload: ValuePayload::Copies(Arc::new(super::Copies {
            bodies: vec![Arc::new(placed)],
            members,
        })),
        outputs: vec![super::OutputNames::new(table, Arc::default())],
        contacts: Arc::new(part.contacts.carried_from(0)),
        carried: Arc::new(carried),
        parts: part.parts,
        // The part's own rows are its document's, read there.
        coincidences: Arc::new([]),
        cited_inputs: Arc::new([crate::coincide::CitedInput::Part(*doc_ref)]),
        motion: None,
        space: None,
    })
}

/// **A rigid placement of `body`**. `None` is the bit-exact identity
/// the instantiate door admits: a clone, so no re-certification is
/// owed.
///
/// # Errors
///
/// The kernel's own [`topo::transform::TransformError`] as
/// [`NodeErrorKind::Transform`].
fn place<T: Decide + topo::AtRestPolicy>(
    body: &Body<T>,
    map: Option<&RigidMotion<T>>,
    tol: Tol,
) -> Result<Body<T>, NodeErrorKind> {
    match map {
        None => Ok(body.clone()),
        Some(map) => transform_rigid(body, map, tol).map_err(NodeErrorKind::Transform),
    }
}

/// **One copy's bodies, where its reader reads them**: placed by the
/// copy's motion ([`super::KeyedCopy::motion`]), here, in the reader's
/// frame.
fn member_bodies<T: Decide + topo::AtRestPolicy>(
    member: &super::KeyedCopy<T>,
    tol: Tol,
) -> Result<Vec<Arc<Body<T>>>, NodeErrorKind> {
    match &member.motion {
        None => Ok(member.bodies.clone()),
        Some(motion) => place_each(&member.bodies, motion, tol),
    }
}

/// Every body of `bodies` placed by node `by`'s map number `j`, in
/// order: one placement of a pattern's master.
fn place_each<T: Decide + topo::AtRestPolicy>(
    bodies: &[Arc<Body<T>>],
    map: &RigidMotion<T>,
    tol: Tol,
) -> Result<Vec<Arc<Body<T>>>, NodeErrorKind> {
    bodies
        .iter()
        .map(|body| place(body, Some(map), tol).map(Arc::new))
        .collect()
}

/// The (Ok) value of an input node.
fn value_of<T: Decide>(
    results: &Results<T>,
    input: RecipeNodeId,
) -> Result<&super::NodeValue<T>, NodeErrorKind> {
    // Failed/Poisoned inputs never reach run_op (the node is poisoned
    // first), so the one standing that arrives is an absent entry: a
    // dangling reference.
    super::usable_in(results, input, || super::NodeStanding::NotInDocument {
        node: input,
    })
    .map_err(|_| NodeErrorKind::MissingInput { input })
}

/// The contact records `value` holds for its output body 0: a
/// boolean's own, or those the value's channel carries
/// ([`OpOut::contacts`]).
fn records_of<T: Decide>(value: &super::NodeValue<T>) -> Arc<topo::ContactRecords> {
    match &value.payload {
        ValuePayload::Boolean(BooleanValue::Body { contacts, .. }) => Arc::clone(contacts),
        _ => Arc::clone(&value.contacts),
    }
}

// OPERAND-DOOR BEGIN — the region the `wire_operand_door` suite's
// `source_rules` census reads; it requires the file's one
// `WrongOperand` construction to sit in here.

/// **The operand refusal, constructed** — the one site in this file
/// that writes [`NodeErrorKind::WrongOperand`]'s three fields.
///
/// Private to the two doors below, which are the two SOURCES of the
/// word for what was found — a value's payload, and a node's kind — so
/// no caller ever writes that word.
fn operand_refusal(
    input: RecipeNodeId,
    expected: &'static str,
    found: &'static str,
) -> NodeErrorKind {
    NodeErrorKind::WrongOperand {
        input,
        expected,
        found,
    }
}

/// **What kind is this operand, and refuse if it is not** — the one
/// home for that question, over a value.
///
/// `read` is the projection the consumer needs, `None` when the value
/// does not carry it. `expected` comes from [`super::family`] or
/// [`super::phrase`], never a literal here. `found:` is the family the
/// value actually carries, read off the payload HERE, so a refusal can
/// never answer with the negation of its own `expected:`.
///
/// # Errors
///
/// [`NodeErrorKind::MissingInput`] for a reference with no value
/// ([`value_of`]), and [`NodeErrorKind::WrongOperand`] when `read`
/// finds the value is not the operand asked for.
fn operand<'v, T: Decide, R>(
    results: &'v Results<T>,
    input: RecipeNodeId,
    expected: &'static str,
    read: impl FnOnce(&'v super::NodeValue<T>) -> Option<R>,
) -> Result<R, NodeErrorKind> {
    let v = value_of(results, input)?;
    read(v).ok_or_else(|| wrong_operand(v, input, expected))
}

/// [`operand`]'s refusal alone, for callers that already hold the
/// value.
fn wrong_operand<T: Decide>(
    v: &super::NodeValue<T>,
    input: RecipeNodeId,
    expected: &'static str,
) -> NodeErrorKind {
    operand_refusal(input, expected, v.payload.kind_name())
}

// OPERAND-DOOR END

/// **A body that becomes material of the result**: what every consumer
/// that combines or reshapes ONE body reads through (a blend, a shell,
/// a split's target, a boolean's and a union's members, a placed
/// union's prototype). It is [`read_body`] that also refuses a PRODUCT,
/// a body of several parts ([`NodeErrorKind::ProductOperand`];
/// `NodeValue::parts`): a solid is one piece of material, so material
/// is fused or reshaped one part at a time. The placers read
/// [`placeable_operand`] instead and carry a product through.
fn body_operand<T: Decide>(
    results: &Results<T>,
    input: RecipeNodeId,
) -> Result<Arc<Body<T>>, NodeErrorKind> {
    let v = value_of(results, input)?;
    if v.parts > 1 {
        return Err(NodeErrorKind::ProductOperand {
            input,
            parts: v.parts,
        });
    }
    read_body(results, input)
}

/// **A body operand, finished** for a door that takes finished bodies
/// (the Boolean's, the split's, the shell's and the blends'): [`body_operand`]'s
/// body through the at-rest gate
/// ([`topo::AtRestPolicy::gate_at_rest_kept`]), once per operand of the
/// node. The evaluator holds the bodies its nodes built with no verdict
/// kept beside them, so the consuming node pays the gate here.
///
/// # Errors
///
/// [`body_operand`]'s; [`NodeErrorKind::UnfinishedOperand`] where the
/// gate refuses the body.
fn finished_operand<T: Decide + geom_core::Bounds + topo::AtRestPolicy>(
    results: &Results<T>,
    input: RecipeNodeId,
    tol: Tol,
) -> Result<topo::AtRestBody<T>, NodeErrorKind> {
    let body = body_operand(results, input)?;
    T::gate_at_rest_kept((*body).clone(), tol)
        .map_err(|errors| NodeErrorKind::UnfinishedOperand { input, errors })
}

/// **A body read for its geometry**: a Body value, or a boolean's
/// non-empty result — [`placeable_operand`]'s `Body` arm, refusing the
/// other in its own one-body word. A reader consumes no material (a
/// datum's face frame reads a face), so a product is read as it is;
/// the product refusal is [`body_operand`]'s.
///
/// An instance's copies are read as its part product, one body, where
/// a selection names a face of it by the part-local name path: that is
/// how a selection reads one copy until it holds a member read (stage
/// G's question). A seat reading the copies whole refuses at its door,
/// by kind.
fn read_body<T: Decide>(
    results: &Results<T>,
    input: RecipeNodeId,
) -> Result<Arc<Body<T>>, NodeErrorKind> {
    let v = value_of(results, input)?;
    match placeable_operand(v, input) {
        Ok(Placeable::Body(b)) => Ok(b),
        Ok(Placeable::Copies(copies)) if copies.bodies.len() == 1 => {
            Ok(Arc::clone(&copies.bodies[0]))
        }
        Ok(Placeable::Instances(_) | Placeable::Copies(_))
        | Err(NodeErrorKind::WrongOperand { .. }) => {
            Err(wrong_operand(v, input, super::family::BODY))
        }
        Err(other) => Err(other),
    }
}

/// **The body a resolved name lives in**: [`read_body`] for a value of
/// one body, else the output body the name's row indexes — copy `i` of
/// a pattern's instances, one copy of a placement's — since a selection
/// names an entity of one body of the value it reads.
pub(super) fn read_named_body<T: Decide>(
    results: &Results<T>,
    input: RecipeNodeId,
    body: u32,
) -> Result<Arc<Body<T>>, NodeErrorKind> {
    let v = value_of(results, input)?;
    match placeable_operand(v, input) {
        Ok(Placeable::Instances(bodies)) => usize::try_from(body)
            .ok()
            .and_then(|k| bodies.get(k))
            .cloned()
            .ok_or_else(|| wrong_operand(v, input, super::family::BODY)),
        Ok(Placeable::Copies(copies)) if copies.bodies.len() > 1 => usize::try_from(body)
            .ok()
            .and_then(|k| copies.bodies.get(k))
            .cloned()
            .ok_or_else(|| wrong_operand(v, input, super::family::BODY)),
        _ => read_body(results, input),
    }
}

/// **What a placer places**: the two value shapes a rigid map is
/// defined over. The placers (`Transform`, `Pattern`) are
/// shape-preserving — `Body → Body`, `Instances → Instances`.
enum Placeable<T: Decide> {
    /// One body: a `Body` value or a boolean's non-empty result.
    Body(Arc<Body<T>>),
    /// Several placed bodies, in the input's own instance order; the
    /// input's name table indexes them by that order.
    Instances(Vec<Arc<Body<T>>>),
    /// A placement's copies, keyed: moved whole, each copy by the same
    /// map, the keys and every table kept.
    Copies(Arc<super::Copies<T>>),
}

impl<T: Decide> Placeable<T> {
    /// The bodies, in value order — one for a body, the list for
    /// instances — so a placer that walks its master reads one shape.
    fn bodies(&self) -> &[Arc<Body<T>>] {
        match self {
            Self::Body(b) => core::slice::from_ref(b),
            Self::Instances(bs) => bs,
            Self::Copies(copies) => &copies.bodies,
        }
    }
}

/// The operand of a placer, read off its evaluated value: one body
/// (a `Body` value or a boolean's non-empty result), or an `Instances`
/// value taken whole. Everything else refuses typed naming both
/// admitted shapes; an empty boolean is a typed absence.
fn placeable_operand<T: Decide>(
    v: &super::NodeValue<T>,
    input: RecipeNodeId,
) -> Result<Placeable<T>, NodeErrorKind> {
    match &v.payload {
        ValuePayload::Body(b) => Ok(Placeable::Body(Arc::clone(b))),
        ValuePayload::Boolean(BooleanValue::Body { body, .. }) => {
            Ok(Placeable::Body(Arc::clone(body)))
        }
        ValuePayload::Boolean(BooleanValue::Empty) => Err(NodeErrorKind::EmptyOperand { input }),
        ValuePayload::Instances(bodies) => Ok(Placeable::Instances(bodies.clone())),
        ValuePayload::Copies(copies) => Ok(Placeable::Copies(Arc::clone(copies))),
        _ => Err(wrong_operand(v, input, super::phrase::BODY_OR_INSTANCES)),
    }
}

/// The linear classification band (kernel-ambient tolerance).
pub(crate) fn band(tol: Tol) -> Result<Band, NodeErrorKind> {
    Band::linear(tol).map_err(NodeErrorKind::Band)
}

/// **The funnel site name** of this layer's direction-length
/// decision — a transform's rotation axis, a pattern's direction, and
/// the mate solve's re-derivation of both from the recipe. A roster
/// carrier (`docs/K-REPORT.md`, "The inventory method, restated"); one
/// constant, so the telemetry and an escalation report the same name.
pub(crate) const EVAL_DIRECTION_NORM: &str = "eval_direction_norm";

/// What each of this layer's decisions decides, in words
/// (`crate::decision::words` reads them).
pub(crate) const DECISION_WORDS: &[(&str, &str)] = &[
    (EVAL_DIRECTION_NORM, geom_core::DIRECTION_LENGTH_SUBJECT),
    (
        "revolve_full_vs_partial",
        "whether the revolve makes a full turn",
    ),
    (PATTERN_SPACING, "whether the pattern spacing is positive"),
    (PATTERN_STEP, "whether the pattern step is zero"),
    (
        PATTERN_STEP_TURN,
        "how many whole turns the pattern step holds",
    ),
];

/// Normalizes a direction-valued vector; a non-finite length refuses,
/// an underflowed one refuses, a decided-zero length refuses, in-band
/// indeterminacy escalates.
///
/// The decision is the kernel's one body
/// ([`geom_core::decide_unit_direction`]); this adds the funnel name
/// ([`EVAL_DIRECTION_NORM`]) and the ROLE word each refusal carries, so
/// a user reads which vector of theirs was refused.
///
/// **Two names, one body** (Ev's ruling on the direction-family home):
/// the layer that OWNS a value names its length decision. A datum's
/// directions are decided under [`DATUM_UNIT_NORM`] inside
/// [`UnitVec3::new`], because `PoseValue` has no unnormalized
/// spelling. The mate solve derives its offsets through this function,
/// so a circular pattern's datum axis is decided under this name on
/// the solve road and under [`DATUM_UNIT_NORM`] on the evaluation road:
/// same arithmetic, two names by road (`crate::mate::solve`,
/// `docs/K-REPORT.md`).
pub(crate) fn unit<T: Decide>(
    v: Vec3<T>,
    role: &'static str,
    band: Band,
) -> Result<UnitVec3<T>, NodeErrorKind> {
    let decision = Decision::DirectionNorm;
    UnitVec3::new(v, decision.name(), band).map_err(|e| {
        refusal(e, role, |source| {
            if role == crate::placement::PLACEMENT_AXIS_ROLE {
                NodeErrorKind::PlacementAxisEscalated {
                    predicate: decision.name(),
                    source,
                }
            } else {
                escalated(decision)(source)
            }
        })
    })
}

/// **The kernel refusal in this layer's vocabulary** — the ONE map for
/// every direction door.
///
/// `role` names the vector the CALLER passed, which is what a user
/// reads; `escalated` is the arm the door puts an escalation in: the
/// evaluation's ([`NodeErrorKind::Escalated`]), a placement frame's
/// rotation axis ([`NodeErrorKind::PlacementAxisEscalated`]), or a pose's
/// or datum's direction ([`pose_direction_escalated`]).
fn refusal(
    e: UnitVec3Error,
    role: &'static str,
    escalated: impl FnOnce(geom_core::Indeterminate) -> NodeErrorKind,
) -> NodeErrorKind {
    match e {
        UnitVec3Error::NonFiniteLength => NodeErrorKind::NonFiniteDirection { role },
        UnitVec3Error::UnderflowedLength => NodeErrorKind::UnderflowedDirection { role },
        UnitVec3Error::Degenerate => NodeErrorKind::DegenerateDirection { role },
        UnitVec3Error::Escalated(source) => escalated(source),
    }
}

/// A pose's or datum's direction length that could not be called,
/// decided under [`DATUM_UNIT_NORM`]: the arm D10's pose ground keeps,
/// [`NodeErrorKind::PoseDirectionEscalated`].
fn pose_direction_escalated(source: geom_core::Indeterminate) -> NodeErrorKind {
    NodeErrorKind::PoseDirectionEscalated {
        predicate: DATUM_UNIT_NORM,
        source,
    }
}

/// A named scalar slot of an evaluated node, with the typed backstop
/// for a slot the node does not carry. Shared with the mate solve's
/// derived offset, which reads the same nodes' slots.
pub(crate) fn need_scalar<T: Decide>(
    vals: &SlotValues<T>,
    slot: SlotId,
) -> Result<T, NodeErrorKind> {
    slots::scalar(vals, slot).ok_or(NodeErrorKind::MissingSlot { slot })
}

/// A named `[Expr; 3]` slot family of an evaluated node, as a vector
/// ([`need_scalar`]'s backstop, on the family's x component). Shared
/// with the mate solve for the same reason.
pub(crate) fn need_vec3<T: Decide>(
    vals: &SlotValues<T>,
    f: impl Fn(Axis3) -> SlotId,
) -> Result<Vec3<T>, NodeErrorKind> {
    let slot = f(Axis3::X);
    slots::vec3(vals, f).ok_or(NodeErrorKind::MissingSlot { slot })
}

fn need_vec2<T: Decide>(
    vals: &SlotValues<T>,
    f: fn(Axis3) -> SlotId,
) -> Result<Vec2<T>, NodeErrorKind> {
    slots::vec2(vals, f).ok_or(NodeErrorKind::MissingSlot { slot: f(Axis3::X) })
}

/// **A revolve's axis direction in its profile's plane**: the unit
/// vector at its heading from the profile's x axis, `(cos h, sin h)`.
/// Unit by construction, up to the rounding of the sine and cosine;
/// its readers decide it as they decide any authored direction.
fn axis_heading<T: Decide>(vals: &SlotValues<T>) -> Result<Vec2<T>, NodeErrorKind> {
    let (sin, cos) = need_scalar(vals, SlotId::AxisHeading)?.sin_cos();
    Ok(Vec2::new(cos, sin))
}

fn need_point2<T: Decide>(
    vals: &SlotValues<T>,
    f: fn(Axis3) -> SlotId,
) -> Result<Point2<T>, NodeErrorKind> {
    let v = need_vec2(vals, f)?;
    Ok(Point2::new(v.x, v.y))
}

/// **A direction refusal before it is spelled as a node error** — what
/// the kernel's door said and which vector said it.
///
/// Separate from its spelling because a pose's direction is decided
/// where the pose is made and spelled by the reader that needed it.
/// Every road spells it through [`DirectionRefusal::node_error`].
#[derive(Debug, Clone, Copy)]
pub struct DirectionRefusal {
    /// Which vector refused, in the words a user reads — "datum frame
    /// x axis", "datum frame y axis".
    pub role: &'static str,
    /// What the kernel's direction door said.
    pub error: UnitVec3Error,
}

impl DirectionRefusal {
    /// The node error this refusal spells, under [`DATUM_UNIT_NORM`],
    /// because on this road the kernel type owns the value. **The one
    /// spelling** from a refusal to a [`NodeErrorKind`]. `pub` because
    /// the refusal is: a consumer holding one asks for the raise here
    /// rather than re-spelling it.
    pub fn node_error(self) -> NodeErrorKind {
        refusal(self.error, self.role, pose_direction_escalated)
    }
}

/// A slot's vector as a datum direction, through the kernel type's own
/// constructor: the decision and its three refusals live there, and
/// this layer names the ROLE.
fn datum_unit<T: Decide>(
    v: Vec3<T>,
    role: &'static str,
    band: Band,
) -> Result<UnitVec3<T>, DirectionRefusal> {
    UnitVec3::new(v, DATUM_UNIT_NORM, band).map_err(|error| DirectionRefusal { role, error })
}

/// **A frame's authored pair, made orthonormal** — the one spelling of
/// it.
///
/// `u` is normalized and KEPT; `v` yields its component along `u`.
/// Gram-Schmidt states "these two span no plane" as a length, so a
/// parallel pair refuses at the same decided door every other
/// direction does, under the y axis's role.
///
/// The refusal comes out UNSPELLED ([`DirectionRefusal`]).
pub(crate) fn frame_axes<T: Decide>(
    origin: Point3<T>,
    u_raw: Vec3<T>,
    v_raw: Vec3<T>,
    band: Band,
) -> Result<RigidMotion<T>, DirectionRefusal> {
    RigidMotion::gram_schmidt(origin, u_raw, v_raw, DATUM_UNIT_NORM, band).map_err(|e| {
        DirectionRefusal {
            role: match e.axis {
                OrthoAxis::U => FRAME_X_ROLE,
                OrthoAxis::V => FRAME_Y_ROLE,
            },
            error: e.error,
        }
    })
}

/// The program-order coordinates of a profile's `n` loops, as the
/// `u32` a program loop is addressed by (a program edit's `loop_`, a
/// profile name's `loop_index`), or a naming refusal when `n` loops do
/// not fit: past `u32::MAX` a loop's coordinate would name another
/// loop's.
fn loop_coordinates(n: usize) -> Result<core::ops::Range<u32>, NodeErrorKind> {
    names::to_u32(
        n,
        "a profile holds more loops than a u32 loop index addresses",
    )
    .map(|n| 0..n)
    .map_err(NodeErrorKind::Naming)
}

/// The profile node's F64 PRECOMPUTE (LIB-SWITCH §4b): the resolved
/// program replays through `profile::replay` — the ONLY path from
/// steps to geometry — then the assembled `Profile<f64>` validates at
/// f64 (the C6 structure-selection gate, which also yields the
/// canonical form the naming anchor is derived from). Runs inside the
/// node's verdict frame (`eval_node`) ahead of the op, so its structure
/// decisions are logged as the node's own. Under the pinned lift the
/// validated form IS the op's value (`wire_profile`), so the node
/// decides each structure question once. VQ6: replay and validation run
/// under the same tolerance the evaluation pins.
pub(crate) fn prepare_profile(
    resolved: &[Vec<profile::Step<f64>>],
    ids: &[Vec<crate::node::StepId>],
    tol: Tol,
) -> Result<ProfilePre, NodeErrorKind> {
    let plane = profile::SketchPlane::xy();
    let mut loops = Vec::with_capacity(resolved.len());
    let mut replay_records = Vec::with_capacity(resolved.len());
    for (li, steps) in loop_coordinates(resolved.len())?.zip(resolved) {
        let (lp, record) = profile::replay_recording(steps, tol)
            .map_err(|error| NodeErrorKind::ProfileReplay { loop_: li, error })?;
        loops.push(lp);
        replay_records.push(record);
    }
    // The loops are the replay's own construction, so validation
    // decides no arc's consistency checks (D1).
    let replayed = profile::ConstructedProfile::new(plane, loops);
    let (validated_f64, canonical) = replayed
        .validate_recording(tol)
        .map_err(NodeErrorKind::Profile)?;
    // A canonical loop matched no program loop: an internal break.
    let naming = anchor::derive_naming(&validated_f64, replayed.loops())
        .ok_or(NodeErrorKind::ProfileAnchor)?;
    // The records above and the program's minted ids describe one
    // program, so a disagreement is an internal break.
    let pieces = super::ProfilePieces::publish(&naming, &replay_records, ids)
        .map_err(|fault| NodeErrorKind::ProfilePieces { fault })?;
    Ok(ProfilePre {
        validated_f64,
        naming,
        pieces,
        structure: profile::ProfileStructure {
            replay: replay_records,
            canonical,
        },
    })
}

/// The lift's SECOND PASS (M10-P PP1/PP5): the same program resolved at
/// the lane scalar and elaborated there, GUIDED by pass 1's record.
///
/// This is where a profile parameter reaches the lane, over the
/// evaluation's own environment ([`LaneEnv::params`]) — a `Dual` seed
/// on a fillet radius carries its tangent to the vertex it moves, an
/// interval parameter widens the loop it describes — while structure
/// stays exactly what the f64 pass chose.
///
/// The naming is pass 1's verbatim (PP4): names are canonical indices,
/// and the canonical permutation they hang off is pinned by the record,
/// so `T`-valued geometry changes no name.
fn lane_profile<T: Decide + geom_core::Bounds>(
    program: &ProfileProgram,
    plane: profile::SketchPlane<T>,
    lane: LaneEnv<'_, T>,
    pre: &ProfilePre,
    tol: Tol,
) -> Result<profile::ValidatedProfile<T>, NodeErrorKind> {
    let resolved = program
        .resolve(lane.params)
        .map_err(|(slot, source)| NodeErrorKind::Expr { slot, source })?;
    let mut loops = Vec::with_capacity(resolved.len());
    for (li, steps) in loop_coordinates(resolved.len())?.zip(&resolved) {
        let record = pre
            .structure
            .replay
            .get(li as usize)
            .ok_or(NodeErrorKind::ProfileRecordMissing { loop_: Some(li) })?;
        let lp = profile::replay_guided(steps, record, tol).map_err(|error| {
            NodeErrorKind::ProfileLaneReplay {
                loop_: li,
                step: error.step,
                structure: match error.kind {
                    profile::ReplayErrorKind::Path(profile::PathError::Structure(r)) => Some(r),
                    _ => None,
                },
            }
        })?;
        loops.push(lp);
    }
    profile::ConstructedProfile::new(plane, loops)
        .validate_guided(tol, &pre.structure.canonical)
        .map_err(NodeErrorKind::Profile)
}

fn wire_profile<T: Decide + geom_core::Bounds>(
    program: &ProfileProgram,
    pre: Option<&ProfilePre>,
    lane: LaneEnv<'_, T>,
    tol: Tol,
) -> PayloadResult<T> {
    let pre = pre.ok_or(NodeErrorKind::ProfileRecordMissing { loop_: None })?;
    let validated = match lane.lift {
        // The build path: the precompute's VALIDATED form embedded
        // through `from_f64`, every decision carried as the f64 one and
        // none remade (`ProfileLift`); the guided lift is where a
        // margin escalates. The op decides nothing: the node's log
        // under this lift is the precompute's.
        super::ProfileLift::Pinned => pre
            .validated_f64
            .clone()
            .lift_onto(profile::SketchPlane::xy())
            .map_err(NodeErrorKind::Profile)?,
        super::ProfileLift::Guided => {
            lane_profile::<T>(program, profile::SketchPlane::xy(), lane, pre, tol)?
        }
    };
    Ok(ValuePayload::Profile(Arc::new(ProfileValue {
        validated,
        naming: pre.naming.clone(),
        pieces: pre.pieces.clone(),
        edge_radii: edge_radii(program, pre),
    })))
}

/// **The per-edge radius expressions, in the sweep's own indexing** —
/// `ProfileProgram::segment_radii`'s answer laid out per CANONICAL loop
/// and segment, which is what a wall record is keyed by.
///
/// The door already answers in canonical positions, so this is a
/// lookup by position `(l, k)`; nothing here re-derives a permutation.
///
/// **A refusal here is the evaluation contradicting itself**: the
/// records were minted from this program by the same pre-pass, so no
/// document can reach one and no caller can repair it. It is a kernel
/// bug observed in a branch — `unreachable!`'s job (D9's D2 addendum),
/// as in `ProfileProgram::profile_edges_of`.
fn edge_radii(program: &ProfileProgram, pre: &ProfilePre) -> Vec<Vec<Option<crate::var::VarId>>> {
    pre.naming
        .loops
        .iter()
        .enumerate()
        .map(|(canonical_loop, anchor)| {
            let by_program_segment = program
                .segment_radii(&pre.structure, &pre.naming, anchor.program_loop)
                .unwrap_or_else(|e| {
                    unreachable!(
                        "this profile's structure record and its naming anchor were \
                         minted from this one program by one pre-pass, so the record \
                         describes THIS program — its steps, its segments and its \
                         radius arguments. Program loop {} is answered from a record \
                         that does not: {e}",
                        anchor.program_loop
                    )
                });
            (0..anchor.len)
                .map(|k| {
                    let want = super::CanonicalSegment {
                        loop_index: u32::try_from(canonical_loop)
                            .unwrap_or_else(|_| unreachable!("a profile's loop count fits in u32")),
                        segment: k,
                    };
                    // FIRST match: one segment carries at most one
                    // emission, because each arc's bulge is set once
                    // (`no_two_emissions_of_one_loop_name_the_same_segment`,
                    // `crates/profile/tests/path_program.rs`).
                    by_program_segment
                        .iter()
                        .find(|(e, _)| *e == want)
                        .map(|(_, var)| **var)
                })
                .collect()
        })
        .collect()
}

/// **The profile-operand verbs' ONE lowering**, driven by the verb's
/// correspondence ([`crate::verbs::sweep`]) rather than written twice.
///
/// It shares a SHAPE with `wire_blend` and `wire_subtract` — build the
/// verb, run it through its own door, read the birth record, emit
/// names — but no code, because the operand differs at every step.
///
/// The verb ARGUMENTS come in already resolved: a revolve's axis is a
/// line in its profile's own 2-D coordinates, with an angle classified
/// full or partial under a document-layer refusal. That is what the document MEANS, not a verb
/// parameter, so each node's arm resolves it and this body takes over
/// from the built verb.
///
/// # What it writes
///
/// The body and the name table (profile refs in canonical numbering).
fn wire_swept<T: Decide + geom_core::Bounds + topo::AtRestPolicy, A>(
    verb: &crate::verbs::sweep::ProfileVerb<T, A>,
    args: A,
    id: RecipeNodeId,
    profile: RecipeNodeId,
    results: &Results<T>,
    tol: Tol,
) -> OpResult<T> {
    let vp = operand(results, profile, super::family::PROFILE, |v| {
        match &v.payload {
            ValuePayload::Profile(vp) => Some(vp),
            _ => None,
        }
    })?;
    let record = (verb.build)(args)
        .run_profile(&vp.validated, tol)
        .map_err(verb_refused)?;
    // Eager N4 emission, BEFORE the structural handoff is taken apart.
    let out = (verb.read)(id, record, &vp.pieces, verb.foreign_record)?;
    Ok(OpOut::plain(
        ValuePayload::Body(Arc::new(out.body)),
        out.table,
    ))
}

/// **Extrudes a profile along its own normal** — the distance slot
/// read, and the generic lowering from there.
fn wire_extrude<T: Decide + geom_core::Bounds + topo::AtRestPolicy>(
    id: RecipeNodeId,
    profile: RecipeNodeId,
    results: &Results<T>,
    vals: &SlotValues<T>,
    tol: Tol,
) -> OpResult<T> {
    let distance = need_scalar(vals, SlotId::Distance)?;
    wire_swept(
        &crate::verbs::sweep::extrude(),
        (distance, sweep::ExtrudeSide::Along),
        id,
        profile,
        results,
        tol,
    )
}

/// **Revolves a profile about an axis line written in its own plane**
/// — the document semantics (the axis slots, the full-vs-partial
/// classification), and the generic lowering from there. The axis's
/// lift, the node's `axis` output, is read by the pose evaluator.
fn wire_revolve<T: Decide + geom_core::Bounds + topo::AtRestPolicy>(
    id: RecipeNodeId,
    profile: RecipeNodeId,
    results: &Results<T>,
    vals: &SlotValues<T>,
    tol: Tol,
) -> OpResult<T> {
    operand(results, profile, super::family::PROFILE, |v| {
        matches!(v.payload, ValuePayload::Profile(_)).then_some(())
    })?;
    // The kernel's `RevolveAxis` and the node's axis line are both in
    // the profile's own 2-D coordinates, so the wiring is the identity.
    let axis2 = RevolveAxis {
        origin: need_point2(vals, SlotId::Origin)?,
        dir: axis_heading(vals)?,
    };
    let b = band(tol)?;
    // Kernel contract: exactly-full must SAY Full. Anything else wires
    // Partial and the kernel's own classification rules on it.
    let angle = need_scalar(vals, SlotId::RevolveAngle)?;
    let revolution = match turns_off(Decision::RevolveFullTurn, angle, T::tau(), b)? {
        Sign::Zero => Revolution::Full,
        _ => Revolution::Partial(angle),
    };
    wire_swept(
        &crate::verbs::sweep::revolve(),
        (axis2, revolution),
        id,
        profile,
        results,
        tol,
    )
}

/// The spine frame and window a tube door takes: the tube's own
/// coordinates, and its slots.
///
/// The RESOLUTION is shared by the two tube arms; each then calls its
/// own public door. This layer decides nothing: the window and wall
/// verdicts stay the door's own; a check here would be a weaker second
/// opinion.
struct TubeArgs<T: geom_core::Real> {
    frame: geom_core::RigidMotion<T>,
    major_radius: T,
    window: sweep::TubeWindow<T>,
    minor_radius: T,
}

fn tube_args<T: Decide>(
    window: &crate::node::TubeWindow,
    vals: &SlotValues<T>,
) -> Result<TubeArgs<T>, NodeErrorKind> {
    Ok(TubeArgs {
        frame: RigidMotion::identity(),
        major_radius: need_scalar(vals, SlotId::TubeMajorRadius)?,
        window: match window {
            crate::node::TubeWindow::Full => sweep::TubeWindow::Full,
            crate::node::TubeWindow::Arc { .. } => sweep::TubeWindow::Arc {
                t0: need_scalar(vals, SlotId::TubeWindowStart)?,
                t1: need_scalar(vals, SlotId::TubeWindowEnd)?,
            },
        },
        minor_radius: need_scalar(vals, SlotId::TubeMinorRadius)?,
    })
}

/// **A solid tube** — `sweep::tube_along_arc` (RECIPE-DOORS D4 as
/// revised).
///
/// # Naming: the revolve template applies WHOLESALE
///
/// [`names::name_revolve`] reads only the `Revolved<T>` maps and the
/// pieces it names them by, never the profile that produced them, and
/// the tube doors return a `Revolved<T>` from the same machinery. So a
/// tube is named with NO new `RoleSeg` variants, spelled with the
/// section's structural locators ([`tube_pieces`]).
fn wire_tube<T: Decide + topo::AtRestPolicy>(
    id: RecipeNodeId,
    window: &crate::node::TubeWindow,
    vals: &SlotValues<T>,
    tol: Tol,
) -> OpResult<T> {
    let a = tube_args(window, vals)?;
    let built = sweep::tube_along_arc(a.frame, a.major_radius, a.window, a.minor_radius, tol)
        .map_err(|e| NodeErrorKind::Tube(Box::new(e)))?;
    let table =
        names::name_revolve(id, &built, &tube_pieces(&built)?).map_err(NodeErrorKind::Naming)?;
    Ok(OpOut::plain(
        ValuePayload::Body(Arc::new(built.body)),
        table,
    ))
}

/// **A tube's section, named structurally**: the door builds the
/// outer circle (and a hollow tube's bore) itself, so each piece is
/// named by its place in that construction under the tube node
/// (`names/README.md`, "N1, the profile pieces") — the shape the node
/// kind fixes, which nothing can renumber.
fn tube_pieces<T: Decide>(
    built: &sweep::Revolved<T>,
) -> Result<super::ProfilePieces, NodeErrorKind> {
    let counts: Vec<usize> = built.rims.iter().map(Vec::len).collect();
    super::ProfilePieces::section(&counts).map_err(NodeErrorKind::Naming)
}

/// **A hollow tube** — `sweep::tube_along_arc_hollow`, the OTHER
/// public door.
///
/// The wall crosses to it unexamined: its three verdicts are decided
/// kernel-side, and the last (the gap between the radii the body would
/// STORE) is about stored numbers this layer cannot see.
///
/// Naming is [`wire_tube`]'s: the cavity shell is the revolve's own
/// hole-loop vocabulary.
fn wire_hollow_tube<T: Decide + topo::AtRestPolicy>(
    id: RecipeNodeId,
    window: &crate::node::TubeWindow,
    vals: &SlotValues<T>,
    tol: Tol,
) -> OpResult<T> {
    let a = tube_args(window, vals)?;
    let wall = need_scalar(vals, SlotId::TubeWall)?;
    let built =
        sweep::tube_along_arc_hollow(a.frame, a.major_radius, a.window, a.minor_radius, wall, tol)
            .map_err(|e| NodeErrorKind::Tube(Box::new(e)))?;
    let table =
        names::name_revolve(id, &built, &tube_pieces(&built)?).map_err(NodeErrorKind::Naming)?;
    Ok(OpOut::plain(
        ValuePayload::Body(Arc::new(built.body)),
        table,
    ))
}

/// **The verb dispatch's one refusal translation**: this layer READS
/// the family off the refusal the `verbs` run door carried through,
/// rather than re-deriving which door it called.
///
/// Exhaustive over [`verbs::VerbError`] with no wildcard arm, so a new
/// refusal shape breaks here.
fn verb_refused<T: geom_core::Bounds>(refusal: verbs::VerbError<T>) -> NodeErrorKind {
    match refusal {
        verbs::VerbError::Blend(sweep::blend::BlendRefusal { verb, error }) => {
            NodeErrorKind::Blend { verb, error }
        }
        verbs::VerbError::Boolean(error) => NodeErrorKind::Boolean(error),
        verbs::VerbError::Extrude(error) => NodeErrorKind::Extrude(error),
        verbs::VerbError::Revolve(error) => NodeErrorKind::Revolve(error),
        verbs::VerbError::Split(error) => NodeErrorKind::Split(error),
        verbs::VerbError::Arity { verb, given } => NodeErrorKind::VerbArity { verb, given },
        // The kernel's error is generic over the lane scalar and this
        // enum is scalar-free, so it crosses as a TOTAL fold at the
        // lane's `f64` witness, never a rendering or a drop.
        verbs::VerbError::Shell(error) => {
            NodeErrorKind::Shell(Box::new(crate::verbs::shell::fold_shell_error_at(*error)))
        }
    }
}

/// **The blend pair's ONE lowering**, driven by the verb's
/// correspondence ([`crate::verbs::blend`]) rather than written twice.
///
/// Constant-radius rolling-ball fillets and equal-setback flat
/// chamfers on a SELECTION of the target's edges. The correspondence
/// supplies what differs: the size slot, the selection-refusal label,
/// which verb to build, and what to call a missing record.
///
/// # Refusals
///
/// The selection is resolved before this runs ([`select`]); an empty one
/// refuses [`NodeErrorKind::BlendSelectionEmpty`]. Failure of the op itself is [`NodeErrorKind::Blend`], carrying
/// the kernel's error unaltered; the input body is never passed
/// through, so a blend that did not happen reads as a failed node.
///
/// # Naming
///
/// The emitter translates per-entity birth records into a FULL table,
/// never matching geometry. `naming: None` is a kernel bug and refuses:
/// an empty table would leave every downstream reference into this
/// body silently unresolvable. The role vocabulary is SHARED by the two
/// verbs; which node minted a strip tells a chamfer from a fillet
/// (RECIPE-DOORS D3).
#[allow(clippy::too_many_arguments)] // `verb` is the correspondence that makes this one function
fn wire_blend<T: Decide + geom_core::Bounds + topo::AtRestPolicy>(
    verb: &crate::verbs::blend::BlendVerb<T>,
    id: RecipeNodeId,
    selection: &Selected<topo::EdgeKey>,
    results: &Results<T>,
    vals: &SlotValues<T>,
    tol: Tol,
) -> OpResult<T> {
    let (target_read, target) = (selection.read, selection.at);
    if selection.ents.is_empty() {
        return Err(NodeErrorKind::BlendSelectionEmpty {
            verb: verb.selection_label,
        });
    }
    let body = finished_operand(results, target, tol)?;
    let size = need_scalar(vals, verb.slots.size_slot)?;
    let target_table = read_table(value_of(results, target)?);
    // D9 order; the kernel refuses a repeated edge itself.
    let mut edges: Vec<topo::EdgeKey> = selection.ents.iter().map(|&(_, edge)| edge).collect();
    edges.sort_unstable();
    let out = (verb.build)(edges, size)
        .run(&body, tol)
        .map_err(verb_refused)?;
    // A blend's run produces the blend record by construction; another
    // family here is a kernel bug, refused typed.
    let naming = crate::verbs::read_record(out.record, verb.record, verb.foreign_record)?;
    let rec = naming.ok_or(NodeErrorKind::Naming(names::NamingError::Emission {
        what: verb.no_records,
    }))?;
    let table = (verb.emitter)(id, target, target_read, &target_table, &out.body, &rec)
        .map_err(NodeErrorKind::Naming)?;
    let inputs = crate::coincide::RowInputs {
        a: (target_read, &target_table),
        b: None,
        tool: None,
    };
    OpOut::plain(ValuePayload::Body(Arc::new(out.body)), table)
        .recording(&out.coincidences, &inputs)
}

/// **The shell's lowering**, driven by the verb's correspondence
/// ([`crate::verbs::shell`]): read the open faces in designation order,
/// build the kernel verb, run it through the seat's shell door,
/// emit names from the birth record under THIS node's id.
///
/// # Refusals
///
/// The open faces are resolved before this runs ([`select`]). An EMPTY
/// selection is the sealed hollow, not a refusal. Failure of the op itself is
/// [`NodeErrorKind::Shell`]; the input body is never passed through. A
/// scalar that cannot form the door's call at all — a dual — refuses
/// [`NodeErrorKind::ShellLaneUnsupported`]. An operand the at-rest gate
/// refuses is [`NodeErrorKind::UnfinishedOperand`]
/// ([`finished_operand`]), which no document reaches (every node's door
/// gates what it ships) and which never meets the lane refusal: a dual's
/// gate refuses nothing, and a certifying scalar has the door.
///
/// # Naming
///
/// The record is written by the doors as they act, so it is not an
/// `Option`; the emitter translates every row.
#[allow(clippy::too_many_arguments)] // the blend lowering's arguments, as `wire_blend`
fn wire_shell<T: Decide + geom_core::Bounds + topo::AtRestPolicy>(
    verb: &crate::verbs::shell::ShellVerb<T>,
    id: RecipeNodeId,
    open: &Selected<topo::FaceKey>,
    results: &Results<T>,
    vals: &SlotValues<T>,
    tol: Tol,
) -> OpResult<T> {
    let (target_read, target) = (open.read, open.at);
    let body = finished_operand(results, target, tol)?;
    let thickness = need_scalar(vals, verb.slots.size_slot)?;
    let target_table = read_table(value_of(results, target)?);
    // Designation order: the first designated face of a chart carries
    // its rim.
    let faces: Vec<topo::FaceKey> = open.ents.iter().map(|&(_, face)| face).collect();
    let built = (verb.build)(faces, thickness);
    // A scalar that may not certify has no shell door and refuses here
    // rather than at an unvalidated hollow.
    let door = <T as topo::AtRestPolicy>::shell_door()
        .ok_or(NodeErrorKind::ShellLaneUnsupported { scalar: T::NAME })?;
    let out = built.run_shell(&body, tol, door).map_err(verb_refused)?;
    let rec = crate::verbs::read_record(out.record, verb.record, verb.foreign_record)?;
    let table = (verb.emitter)(id, target, target_read, &target_table, &out.body, &rec)
        .map_err(NodeErrorKind::Naming)?;
    Ok(OpOut::plain(ValuePayload::Body(Arc::new(out.body)), table))
}

/// The mid-evaluation N5 refusal ladder, shared by every door that
/// resolves an AUTHORED name against the tables the run has built so
/// far.
///
/// Mid-evaluation there is no prior run and no whole-evaluation
/// index, so [`mod@crate::resolve`]'s full ladder does not apply. Three
/// rungs remain, asked in this order by [`ladder::resolve_key`], the
/// walk every door takes:
///
/// 1. [`ladder::live`] — the minting node must still be in the
///    document. Ids are never reused, so an id the mint log holds was
///    DELETED and one it does not hold was never this document's
///    (`ForeignNode`). The [`ladder::Live`] token makes this rung
///    outrank every later refusal, a door's own included.
/// 2. a tie row → `Ambiguous`: the tie row IS the ambiguity (N5).
/// 3. no row → `Vanished`, through [`ladder::vanished`]: there is no
///    evidence to weigh mid-evaluation.
///
/// A door names the ONE table its name is read against and keeps its
/// own arity (which kind it accepts). Every refusal PAYLOAD is [`mod@crate::resolve`]'s, minted
/// by one constructor each ([`crate::resolve::ResolveError::node_gone`],
/// [`crate::resolve::ResolveError::ambiguous`],
/// [`crate::resolve::ResolveError::vanished_fallback`]); what is this
/// module's is the rung ORDER. Refusals come out BOXED, as the doors'
/// error variants carry them.
mod ladder {
    use crate::names::interrogate::{Denoted, TableRefusal, key_in};
    use crate::names::{NameTable, NamingError, StableName};
    use crate::program::ProfileProgram;
    use crate::resolve::ResolveError;

    /// Proof that rung 1 passed: `name`'s minting node is live.
    ///
    /// Constructible only by [`live`], and required by [`resolve_key`],
    /// so no door can read a table — and so raise a refusal ABOUT the
    /// tables — before the `NodeGone` check. Carrying the name means the
    /// table is read for the same name.
    #[derive(Clone, Copy, Debug, PartialEq, Eq)]
    pub(super) struct Live<'n>(&'n StableName);

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

    /// **The single-table walk into a key of the kind `K`**: rung 1,
    /// then the table read through [`key_in`], which asks what the name
    /// denotes before how many entities answer to it, so a tie is a tie
    /// of the kind the reader takes.
    ///
    /// A name that does not denote a `K` is an invariant broken, not an
    /// author's fault: every door that writes a selection holds its
    /// names at its kind ([`crate::var::Select::fault`]), and every
    /// `NameTable` door that seats a row holds its key at its name's
    /// kind. It is asserted in debug and refused as the kernel bug it is.
    pub(super) fn resolve_key<K: Denoted>(
        name: &StableName,
        doc: &crate::doc::Doc<ProfileProgram>,
        table: &NameTable,
        refuse: impl Fn(Box<ResolveError>) -> super::NodeErrorKind,
    ) -> Result<(u32, K), super::NodeErrorKind> {
        let live = live(name, doc).map_err(&refuse)?;
        key_in::<K>(table, live.0).map_err(|refusal| match refusal {
            TableRefusal::NoSuchName => refuse(vanished(&live)),
            TableRefusal::Ambiguous { candidates } => refuse(tied(&live, candidates)),
            TableRefusal::Kind { found } => {
                debug_assert!(
                    false,
                    "a selection's name denotes {} {}, not the {} {} the selection holds",
                    found.article(),
                    found.noun(),
                    K::KIND.article(),
                    K::KIND.noun()
                );
                super::NodeErrorKind::Naming(NamingError::Emission {
                    what: "a selection's name denotes another entity kind than the selection holds",
                })
            }
        })
    }

    /// Rung 2's payload: the tie row is the name itself, as there is no
    /// widened base to tie against in one table.
    fn tied(live: &Live<'_>, width: usize) -> Box<ResolveError> {
        Box::new(ResolveError::ambiguous(
            live.0,
            live.0.clone(),
            live.0.node,
            width,
        ))
    }

    /// Rung 3's payload, from the one home that mints it.
    pub(super) fn vanished(live: &Live<'_>) -> Box<ResolveError> {
        Box::new(ResolveError::vanished_fallback(live.0))
    }
}

/// **A selection, evaluated** ([`select`]): the operation whose value
/// its body read holds, and the entities its names denote there, in the
/// selection's stored order, as keys of the kind `K` it holds.
pub(super) struct Selected<K> {
    /// The body read: what a blend's or a shell's carried names are
    /// keyed by ([`names::RoleSeg::From`]).
    pub(super) read: crate::VarId,
    /// The operation the body read is an output of.
    pub(super) at: RecipeNodeId,
    /// The entities, one per name: the output body each lies in, and its
    /// key.
    pub(super) ents: Vec<(u32, K)>,
}

/// **The selection `var` the node reads at `slot`, evaluated** (D10): its
/// body read resolved to the operation whose value holds it, then each
/// name through that value's table under the [`ladder`], into a key of
/// the kind `K` the seat reads. This is the one place a name a slot
/// reads is resolved.
///
/// What each name denotes is authored data, held at the selection's
/// kind by the doors that write one ([`crate::var::Select::fault`]), so
/// nothing here refuses an author over it ([`ladder::resolve_key`]).
///
/// # Errors
///
/// [`NodeErrorKind::UnresolvedRead`] for a body read no live operation
/// defines, [`NodeErrorKind::SelectResolve`] for a name the ladder
/// refuses, and [`NamingError::Emission`](names::NamingError::Emission)
/// for a caller whose `K` is not the selection's kind (a wiring bug,
/// [`resolve_selection`]).
fn select<T: Decide, K: names::interrogate::Denoted>(
    doc: &crate::doc::Doc<ProfileProgram>,
    results: &Results<T>,
    slot: crate::OperandSlot,
    var: crate::VarId,
) -> Result<Selected<K>, NodeErrorKind> {
    let Some(selection) = doc.selection(var) else {
        return Err(NodeErrorKind::UnresolvedRead { slot, var });
    };
    let at = super::read_at(doc, slot, selection.body)?;
    let table = &read_table(value_of(results, at)?);
    resolve_selection(doc, table, slot, var, at)
}

/// **A singleton selection `var`, evaluated at the kind its variable
/// holds** ([`select`]): where its body read lies, and its one entity,
/// for a reader that admits several kinds (a measure's reference, a
/// pose read off geometry).
///
/// # Errors
///
/// [`select`]'s.
fn select_one<T: Decide>(
    doc: &crate::doc::Doc<ProfileProgram>,
    results: &Results<T>,
    slot: crate::OperandSlot,
    var: crate::VarId,
) -> Result<(RecipeNodeId, names::EntityRef), NodeErrorKind> {
    fn one<K: names::interrogate::Denoted>(
        selected: Selected<K>,
    ) -> (RecipeNodeId, names::EntityRef) {
        let [(body, key)] = selected.ents.as_slice() else {
            unreachable!("a singleton selection holds one name (`Select::fault`)")
        };
        let ent = names::EntityRef {
            body: *body,
            key: key.entity(),
        };
        (selected.at, ent)
    }
    let Some((entity, false)) = doc.var(var).and_then(|held| held.kind().selection()) else {
        return Err(NodeErrorKind::UnresolvedRead { slot, var });
    };
    Ok(match entity {
        names::EntityKind::Face => one(select::<T, topo::FaceKey>(doc, results, slot, var)?),
        names::EntityKind::Edge => one(select::<T, topo::EdgeKey>(doc, results, slot, var)?),
        names::EntityKind::Vertex => one(select::<T, topo::VertexKey>(doc, results, slot, var)?),
        names::EntityKind::Body => unreachable!("a selection names faces, edges or vertices"),
    })
}

/// **A selection's names resolved in `table`**, the value read `at`:
/// [`select`]'s ladder walk, for a table already in hand.
fn resolve_selection<K: names::interrogate::Denoted>(
    doc: &crate::doc::Doc<ProfileProgram>,
    table: &names::NameTable,
    slot: crate::OperandSlot,
    var: crate::VarId,
    at: RecipeNodeId,
) -> Result<Selected<K>, NodeErrorKind> {
    let Some(selection) = doc.selection(var) else {
        return Err(NodeErrorKind::UnresolvedRead { slot, var });
    };
    // The key kind each caller asks for restates the seat's entity, so
    // it is checked once here against the variable's own kind — which
    // the edit and load doors hold to the seat's — rather than trusted
    // at every call site. A mismatch is this module's wiring, never the
    // author's: loud in a debug build, typed in a release one.
    let held = doc
        .var(var)
        .and_then(|held| held.kind().selection())
        .map(|(entity, _)| entity);
    if held != Some(K::KIND) {
        debug_assert!(
            false,
            "the {slot:?} seat is wired to read {} keys from a selection of {held:?}",
            K::KIND.noun()
        );
        return Err(NodeErrorKind::Naming(names::NamingError::Emission {
            what: "a seat is wired to read another entity kind than its selection holds",
        }));
    }
    let ents = selection
        .names
        .iter()
        .enumerate()
        .map(|(reference, name)| {
            ladder::resolve_key(name, doc, table, |error| NodeErrorKind::SelectResolve {
                slot,
                var,
                reference,
                error,
            })
        })
        .collect::<Result<_, _>>()?;
    Ok(Selected {
        read: selection.body,
        at,
        ents,
    })
}

/// One resolved measure reference, as a SELECTION rather than as a
/// carrier: the body the name landed in, where it was read, and which
/// entity it is.
///
/// [`super::measure::Carrier`] is the closed forms' view of the same
/// resolution; `min_clearance` needs this one (a body and a face
/// scope). Both come off one ladder walk in [`wire_measure`].
struct Measured<'v, T: Decide> {
    at: RecipeNodeId,
    index: u32,
    body: &'v topo::Body<T>,
    key: crate::names::EntityKey,
}

impl<'v, T: Decide> Measured<'v, T> {
    /// This selection as one side of a `min_clearance`: every face of
    /// the body (arena order) for a body read, the one face for a face
    /// selection.
    ///
    /// A `min_clearance` reference reads a `Body` or a `Face` and
    /// nothing else ([`crate::MeasureVerb::admits`]), which the edit and
    /// load doors hold as the seat's kind, so the key is one of those
    /// two by construction.
    fn clearance_operand(&self) -> crate::measure::MinClearanceOperand<'v, T> {
        let faces = match self.key {
            names::EntityKey::Body => self.body.faces().map(|(k, _)| k).collect(),
            names::EntityKey::Face(k) => vec![k],
            names::EntityKey::Edge(_) | names::EntityKey::Vertex(_) => {
                unreachable!("a min_clearance reference reads a body or a face (its seat's kind)")
            }
        };
        crate::measure::MinClearanceOperand {
            at: self.at,
            index: self.index,
            body: self.body,
            faces,
        }
    }
}

/// **A measurement** (E3): read the primitive's two references, read
/// the carriers they sit on, run the closed form, hand back a typed F1
/// quantity. No body in, no body out.
///
/// # What a reference reads
///
/// A `Body` read is that whole body; a `Face`, `Edge` or `Vertex` read
/// is a one-name selection, evaluated by [`select`] in the body it
/// states. The body
/// read is what makes the answer the placed carrier: a selection of a
/// transform's output names the moved geometry.
fn wire_measure<'r, T: Decide + crate::measure::MinClearanceLane>(
    primitive: &crate::measure::MeasurePrimitive<crate::VarId>,
    doc: &crate::doc::Doc<ProfileProgram>,
    results_at: &dyn Fn(usize) -> &'r Results<T>,
    tol: Tol,
) -> OpResult<T> {
    let dim = primitive.dim();
    let mut sides = Vec::with_capacity(2);
    for (index, &var) in primitive.refs().into_iter().enumerate() {
        // Each reference reads its own body: two ports of one split are
        // two bodies (`reads_projected`).
        let results = results_at(index);
        let slot = crate::OperandSlot::Measured(
            primitive.kind(),
            u8::try_from(index).unwrap_or_else(|_| unreachable!("a primitive reads two")),
        );
        let (at, ent) = if doc.selection(var).is_some() {
            select_one(doc, results, slot, var)?
        } else {
            let at = super::read_at(doc, slot, var)?;
            (
                at,
                names::EntityRef {
                    body: 0,
                    key: names::EntityKey::Body,
                },
            )
        };
        let value = value_of(results, at)?;
        let body = crate::names::interrogate::output_body(&value.payload, ent.body)
            .map_err(|error| NodeErrorKind::MeasureRefUnreadable { slot, var, error })?;
        let carrier = super::measure::carrier_of(body, ent);
        sides.push((
            Measured {
                at,
                index: ent.body,
                body,
                key: ent.key,
            },
            carrier,
        ));
    }
    let [(a, ca), (b, cb)] = sides.as_slice() else {
        unreachable!("a primitive reads two references")
    };
    let measured = if matches!(
        primitive,
        crate::measure::MeasurePrimitive::MinClearance { .. }
    ) {
        // The E7 engine's, over the bodies this evaluation built: it
        // wants bodies and face scopes, not carriers.
        let (oa, ob) = (a.clearance_operand(), b.clearance_operand());
        match T::min_separation(&oa, &ob) {
            Some(Ok(v)) => super::measure::finite(v),
            Some(Err(refusal)) => return Err(NodeErrorKind::MeasureClearanceRefused(refusal)),
            // No value at this scalar: saying so at the node lets an
            // assertion over it report `Unevaluated` instead of being
            // poisoned.
            None => {
                return Ok(OpOut::plain(
                    ValuePayload::MeasureUnavailable {
                        reason: crate::measure::MeasureUnavailableAt::NeedsEnclosure {
                            verb: primitive.verb(),
                            scalar: T::NAME,
                            door: "clearance::min_separation",
                        },
                        dim,
                    },
                    names::empty(),
                ));
            }
        }
    } else {
        super::measure::primitive(primitive, ca, cb, band(tol)?).and_then(super::measure::finite)
    };
    let value = measured.map_err(|refusal| match refusal {
        super::measure::PrimitiveRefusal::Unsupported(u) => NodeErrorKind::MeasureUnsupported(u),
        super::measure::PrimitiveRefusal::Escalated { predicate, source } => {
            NodeErrorKind::MeasureEscalated { predicate, source }
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
        ValuePayload::Measure { value, dim },
        names::empty(),
    ))
}

/// **An assertion's verdict** (E10): compare the value this node reads
/// against its bound, and report.
///
/// Report-ONLY: no op accepts a verdict as an operand, and nothing here
/// touches the value or the document.
fn wire_assertion<T: Decide>(
    value_dim: crate::expr::Dimension,
    bound_dim: crate::expr::Dimension,
    certified: crate::measure::Certified,
    node: &Node<ProfileProgram>,
    payload: &super::Payload<T>,
    tol: Tol,
) -> OpResult<T> {
    let Node::Assertion { relation, .. } = node else {
        unreachable!("an assertion's verdict is wired for an assertion")
    };
    // A measured value with no value at this scalar is not a failed
    // node, so the assertion answers with E10's third state and the
    // requirement stays visible in a build that cannot check it.
    if let Some((_, reason)) = payload.unavailable {
        return Ok(OpOut::plain(
            ValuePayload::Assertion(crate::measure::AssertionVerdict::Unevaluated {
                reason: crate::measure::UnevaluatedReason::MeasureUnavailable(reason),
            }),
            names::empty(),
        ));
    }
    // The bound's DECLARED dimension must agree (units erase at the
    // evaluation boundary), through the rule the document doors ask.
    if crate::node::AssertionBoundFault::against(value_dim, bound_dim).is_some() {
        return Err(NodeErrorKind::AssertionDimension {
            measured: value_dim,
            bound: bound_dim,
        });
    }
    let [value, bound] = payload.values.as_slice() else {
        unreachable!(
            "an assertion with a value has its value and its bound evaluated, yet the payload \
             vector holds {}",
            payload.values.len()
        )
    };
    Ok(OpOut::plain(
        ValuePayload::Assertion(crate::measure::decide_assertion(
            *value,
            *bound,
            *relation,
            band(tol)?,
            certified,
        )),
        names::empty(),
    ))
}

/// **The split's lowering**, driven by its correspondence
/// ([`crate::verbs::split`]): one body and one DATUM operand in, TWO
/// sides out under one record, stamped in one index space.
///
/// # Refusals
///
/// A tool that is not a plane datum is `WrongOperand`, decided before
/// any verb exists. Failure of the op itself is
/// [`NodeErrorKind::Split`] through [`verb_refused`]; nothing here
/// re-derives the plane.
fn wire_split<T: Decide + geom_core::Bounds + topo::AtRestPolicy>(
    verb: &crate::verbs::split::SplitVerb<T>,
    id: RecipeNodeId,
    (target_read, target): (crate::VarId, RecipeNodeId),
    (tool, tool_read): (&PoseValue<T>, crate::pose::RoleRead<crate::VarId>),
    results: &Results<T>,
    tol: Tol,
) -> OpResult<T> {
    let body = finished_operand(results, target, tol)?;
    let plane = (verb.tool)(tool).unwrap_or_else(|| {
        unreachable!(
            "the door admits {} alone at a split's tool, not {}",
            verb.tool_expected,
            tool.kind()
        )
    });
    let built = (verb.build)(plane);
    let out = built.run_split(&body, tol).map_err(verb_refused)?;
    let coincidences = out.coincidences;
    let naming = crate::verbs::read_record(out.record, verb.record, verb.foreign_record)?;
    let side = |part: SplitPart<T>| match part {
        SplitPart::Body(b) => SplitSide::Body(Arc::new(b)),
        SplitPart::Empty => SplitSide::Empty,
    };
    let above = side(out.above);
    let below = side(out.below);
    let as_body = |s: &SplitSide<T>| match s {
        SplitSide::Body(b) => Some(Arc::clone(b)),
        SplitSide::Empty => None,
    };
    let target_table = read_table(value_of(results, target)?);
    let (ab, bb) = (as_body(&above), as_body(&below));
    let emitted = (verb.emitter)(
        id,
        ab.as_deref(),
        bb.as_deref(),
        &naming,
        target,
        &target_table,
        &body,
        tol,
    )
    .map_err(NodeErrorKind::Naming)?;
    // Each half is its own output with its own names (N1, the scope).
    let outputs = emitted
        .iter()
        .map(|half| {
            super::OutputNames::new(
                Arc::clone(&half.table),
                Arc::new(names::FragmentGroups::minted(&half.groups)),
            )
        })
        .collect();
    OpOut::per_output(ValuePayload::Split { above, below }, outputs).recording(
        &coincidences,
        &crate::coincide::RowInputs {
            a: (target_read, &target_table),
            b: None,
            tool: Some(tool_read),
        },
    )
}

/// **The projection node** (DM3): ONE body out of a pattern's copies,
/// as the `Body` value every consumer already takes. Any value but
/// `Instances` refuses `WrongOperand`: a single body is NOT its own
/// instance 0, since nothing is several bodies until a node says so
/// (D3).
///
/// The body handed on is the instance's own `Arc`. The table is the
/// input's PROJECTED onto it ([`NameTable::project`]): the selected
/// body's rows re-keyed to body 0, names verbatim, no segment added.
/// So a selector spelled against the input's rows resolves here
/// unchanged, and one for another instance refuses as absent, never
/// re-anchored. `check_total` re-checks that the projection dropped
/// nothing the body still has.
fn wire_part<T: Decide>(
    of: RecipeNodeId,
    results: &Results<T>,
    vals: &SlotValues<T>,
) -> OpResult<T> {
    let value = value_of(results, of)?;
    let (body, index) = match &value.payload {
        ValuePayload::Instances(instances) => {
            let index = slots::count(vals, SlotId::Instance).ok_or(NodeErrorKind::MissingSlot {
                slot: SlotId::Instance,
            })?;
            // The index is into the value's FLAT list (`j·M + i`, the
            // layout `wire_pattern` fixes). A count past u32 is the
            // pattern's own emission bug, refused before any index is
            // judged against it.
            let count = names::output_body(instances.len()).map_err(NodeErrorKind::Naming)?;
            // A negative index and one past the end fail the same way.
            let ix = u32::try_from(index).ok().filter(|i| *i < count).ok_or(
                NodeErrorKind::InstanceOutOfRange {
                    input: of,
                    index,
                    count: instances.len(),
                },
            )?;
            (Arc::clone(&instances[ix as usize]), ix)
        }
        _ => return Err(wrong_operand(value, of, super::family::INSTANCES)),
    };
    let table = read_table(value)
        .project(index)
        .map_err(|dup| NodeErrorKind::Naming(names::NamingError::from(dup)))?;
    names::check_total(&table, &body, 0).map_err(NodeErrorKind::Naming)?;
    Ok(OpOut::plain(ValuePayload::Body(body), Arc::new(table)).carrying(value.parts))
}

/// **The instance of `of`'s `Instances` value the count in `slot`
/// picks**, with its index. The index is into the value's FLAT list
/// (`j·M + i`, the layout `wire_pattern` fixes); a negative index and
/// one past the end refuse alike ([`NodeErrorKind::InstanceOutOfRange`]).
/// A count past u32 is the pattern's own emission bug, refused before
/// any index is judged against it.
fn instance_of<T: Decide>(
    of: RecipeNodeId,
    instances: &[Arc<Body<T>>],
    vals: &SlotValues<T>,
    slot: SlotId,
) -> Result<(Arc<Body<T>>, u32), NodeErrorKind> {
    let index = slots::count(vals, slot).ok_or(NodeErrorKind::MissingSlot { slot })?;
    let count = names::output_body(instances.len()).map_err(NodeErrorKind::Naming)?;
    let ix = u32::try_from(index).ok().filter(|i| *i < count).ok_or(
        NodeErrorKind::InstanceOutOfRange {
            input: of,
            index,
            count: instances.len(),
        },
    )?;
    Ok((Arc::clone(&instances[ix as usize]), ix))
}

/// One half of a split's value, or [`NodeErrorKind::EmptyHalf`].
fn split_side<T: Decide>(
    of: RecipeNodeId,
    half: SplitHalf,
    above: &SplitSide<T>,
    below: &SplitSide<T>,
) -> Result<Arc<Body<T>>, NodeErrorKind> {
    match match half {
        SplitHalf::Above => above,
        SplitHalf::Below => below,
    } {
        SplitSide::Body(b) => Ok(Arc::clone(b)),
        SplitSide::Empty => Err(NodeErrorKind::EmptyHalf { input: of, half }),
    }
}

/// The reads `node`'s operands make, each at its seat, in field order: a
/// selection read as the body it states.
fn body_rows(
    node: &Node<ProfileProgram>,
    doc: &crate::doc::Doc<ProfileProgram>,
) -> Vec<(crate::OperandSlot, crate::VarId)> {
    node.operand_rows()
        .into_iter()
        .map(|(seat, var)| (seat, doc.selection(var).map_or(var, |select| select.body)))
        .collect()
}

/// **The value a read at a seat is**, where that is not the value of
/// the operation it reads: a split's port is that half (FORK-1: a
/// split defines two bodies); an indexed read at a body seat
/// ([`crate::BodyRead`], DM3) is the family's member its index picks
/// ([`instance_of`]). Either is the body alone with the operation's
/// part count: a half with its own output's table (N1, the scope), a
/// member with the family's table projected onto it
/// ([`NameTable::project`]), so an indexed read and the `Part` spelling
/// of it give one value. `results` with each such operation's entry
/// replaced, or `None` when no read in `rows` is either.
///
/// Every body seat resolves through here: the one-body nodes over all
/// their reads at once, a subtract, a union and an intersect read by
/// read (two reads of one operation are two bodies, which one map
/// keyed by the operation cannot hold).
fn reads_projected<T: Decide + topo::AtRestPolicy>(
    node: &Node<ProfileProgram>,
    rows: &[(crate::OperandSlot, crate::VarId)],
    doc: &crate::doc::Doc<ProfileProgram>,
    (results, vals): (&Results<T>, &SlotValues<T>),
    tol: Tol,
) -> Result<Option<Results<T>>, NodeErrorKind> {
    let mut replaced: Vec<(RecipeNodeId, u32, super::NodeValue<T>)> = Vec::new();
    for &(seat, var) in rows {
        if let Some(read) = node.body_read(seat)
            && let Some(key) = &read.key
        {
            let placement = super::read_at(doc, seat, var)?;
            replaced.push(copy_of(
                placement,
                key,
                read.is_indexed(),
                seat,
                (results, vals),
                tol,
            )?);
            continue;
        }
        if node
            .body_read(seat)
            .is_some_and(crate::BodyRead::is_indexed)
        {
            let family = super::read_at(doc, seat, var)?;
            let value = value_of(results, family)?;
            let ValuePayload::Instances(instances) = &value.payload else {
                return Err(wrong_operand(value, family, super::family::INSTANCES));
            };
            let (body, index) = instance_of(family, instances, vals, SlotId::Index { seat, k: 0 })?;
            // A family's one table holds every member at its place.
            let table = read_table(value)
                .project(index)
                .map_err(|dup| NodeErrorKind::Naming(names::NamingError::from(dup)))?;
            let names = super::OutputNames::new(Arc::new(table), Arc::default());
            replaced.push(projected(family, value, body, names, index)?);
            continue;
        }
        replaced.extend(split_port(var, doc, results)?);
    }
    Ok(with_replaced(results, replaced))
}

/// **One copy of a placement's copies, read by its key**
/// ([`crate::BodyRead::key`]): the copy alone with its own table, or for
/// a copy that is a family, the member at the read's index. A key that
/// names no copy, and a read at the other shape, refuse typed: whether
/// the key exists, and what it holds, is this evaluation's to answer.
fn copy_of<T: Decide + topo::AtRestPolicy>(
    placement: RecipeNodeId,
    key: &crate::CopyKey,
    indexed: bool,
    seat: crate::OperandSlot,
    (results, vals): (&Results<T>, &SlotValues<T>),
    tol: Tol,
) -> Result<(RecipeNodeId, u32, super::NodeValue<T>), NodeErrorKind> {
    let value = value_of(results, placement)?;
    let ValuePayload::Copies(copies) = &value.payload else {
        return Err(wrong_operand(value, placement, super::family::COPIES));
    };
    let Some((position, member)) = copies
        .members
        .iter()
        .enumerate()
        .find(|(_, member)| &member.key == key)
    else {
        return Err(NodeErrorKind::NoSuchCopy {
            input: placement,
            key: Box::new(key.clone()),
        });
    };
    if member.family != indexed {
        return Err(NodeErrorKind::CopyShape {
            input: placement,
            key: Box::new(key.clone()),
            family: member.family,
        });
    }
    let bodies = member_bodies(member, tol)?;
    let (body, table) = if indexed {
        let (body, index) = instance_of(placement, &bodies, vals, SlotId::Index { seat, k: 0 })?;
        let table = member
            .table
            .project(index)
            .map_err(|dup| NodeErrorKind::Naming(names::NamingError::from(dup)))?;
        (body, table)
    } else {
        let Some(body) = bodies.first() else {
            unreachable!("a copy that is a body holds one")
        };
        (Arc::clone(body), member.table.as_ref().clone())
    };
    names::check_total(&table, &body, 0).map_err(NodeErrorKind::Naming)?;
    Ok((
        placement,
        u32::try_from(position).unwrap_or(u32::MAX),
        super::NodeValue {
            payload: ValuePayload::Body(body),
            outputs: Arc::new([super::OutputNames::new(Arc::new(table), Arc::default())]),
            contacts: Arc::default(),
            carried: Arc::default(),
            parts: member.parts,
            ..value.clone()
        },
    ))
}

/// [`reads_projected`]'s split ports alone, for reads no seat holds: a
/// pose definition's reads are projected here on their own, since the
/// half a pose reads need not be the one its reader's body is.
pub(super) fn project_ports<T: Decide>(
    reads: &[crate::VarId],
    doc: &crate::doc::Doc<ProfileProgram>,
    results: &Results<T>,
) -> Result<Option<Results<T>>, NodeErrorKind> {
    let mut replaced = Vec::new();
    for &var in reads {
        replaced.extend(split_port(var, doc, results)?);
    }
    Ok(with_replaced(results, replaced))
}

/// The half a read of a split's port is, projected, keyed by the split;
/// `None` when `var` reads no split by port.
fn split_port<T: Decide>(
    var: crate::VarId,
    doc: &crate::doc::Doc<ProfileProgram>,
    results: &Results<T>,
) -> Result<Option<(RecipeNodeId, u32, super::NodeValue<T>)>, NodeErrorKind> {
    let Some((split, port)) = doc
        .var(var)
        .and_then(|held| held.def().output())
        .filter(|(split, _)| matches!(doc.node(*split), Some(Node::Split { .. })))
    else {
        return Ok(None);
    };
    let half = SplitHalf::of_output_body(u32::from(port))
        .unwrap_or_else(|| unreachable!("a split defines a port per half"));
    let value = value_of(results, split)?;
    let ValuePayload::Split { above, below } = &value.payload else {
        unreachable!("a split evaluates to its two sides")
    };
    let body = split_side(split, half, above, below)?;
    // The half's names are its own output's (N1, the scope).
    let names =
        value
            .output(port)
            .cloned()
            .ok_or(NodeErrorKind::Naming(names::NamingError::Emission {
                what: SPLIT_HALF_UNNAMED,
            }))?;
    projected(split, value, body, names, half.output_body()).map(Some)
}

/// `value`'s output `index` as the body alone, at output 0, with that
/// output's names.
fn projected<T: Decide>(
    of: RecipeNodeId,
    value: &super::NodeValue<T>,
    body: Arc<Body<T>>,
    names: super::OutputNames,
    index: u32,
) -> Result<(RecipeNodeId, u32, super::NodeValue<T>), NodeErrorKind> {
    names::check_total(names.table(), &body, 0).map_err(NodeErrorKind::Naming)?;
    Ok((
        of,
        index,
        super::NodeValue {
            payload: ValuePayload::Body(body),
            outputs: Arc::new([names]),
            contacts: Arc::default(),
            carried: Arc::default(),
            ..value.clone()
        },
    ))
}

/// `results` with each of `replaced` (an operation, the output it
/// projects, the body) standing in for its operation's entry, or `None`
/// when `replaced` is empty.
fn with_replaced<T: Decide>(
    results: &Results<T>,
    replaced: Vec<(RecipeNodeId, u32, super::NodeValue<T>)>,
) -> Option<Results<T>> {
    if replaced.is_empty() {
        return None;
    }
    // One map holds one body per operation: a node reading two bodies of
    // one operation is projected read by read (`run_op`'s `per_operand`).
    for (i, (at, index, _)) in replaced.iter().enumerate() {
        if replaced[..i].iter().any(|(a, b, _)| a == at && b != index) {
            unreachable!("a node read two bodies of {at:?} into one map: project it read by read");
        }
    }
    // The values this op can read: every entry that stands, cloned (a
    // value's geometry and tables are shared, not copied). The failed
    // ones are left out: every read goes through `value_of`, which
    // answers a failed entry and an absent one alike.
    let mut local: Results<T> = results
        .iter()
        .filter_map(|(&id, result)| match result {
            NodeResult::Ok(value) => Some((id, NodeResult::Ok(value.clone()))),
            NodeResult::Poisoned { through } => {
                Some((id, NodeResult::Poisoned { through: *through }))
            }
            _ => None,
        })
        .collect();
    for (at, _, value) in replaced {
        local.insert(at, NodeResult::Ok(value));
    }
    Some(local)
}

/// **One member of a union or an intersect, as the fold reads it**:
/// the read it came through (its names' key), its finished body and its
/// own table.
struct Member<T: Decide> {
    read: crate::VarId,
    body: Arc<topo::AtRestBody<T>>,
    table: Arc<NameTable>,
    /// The member's own contact records, carried into the fold step
    /// that joins it (D1 (ii)).
    records: Arc<topo::ContactRecords>,
}

// `Bounds` rides along for the boolean lane only: the sweep's BVH
// candidate generation reads coordinate brackets (the L7 driver-code
// allowance).
//
// The TWO-OPERAND lowering, kept apart from `wire_blend`'s: two operand
// tables, the contact-record carry and the typed empty success would
// otherwise become runtime arity.
/// **The subtract's lowering** (DM4): `from` cut by `tool`, through the
/// pair verb, each seat's entities named by its read
/// ([`names::RoleSeg::From`]).
fn wire_subtract<T: Decide + geom_core::Bounds + topo::AtRestPolicy>(
    verb: &crate::verbs::boolean::PairVerb<T>,
    id: RecipeNodeId,
    (a_read, a, results_a): (crate::VarId, RecipeNodeId, &Results<T>),
    (b_read, b, results_b): (crate::VarId, RecipeNodeId, &Results<T>),
    boolean_sweep: topo::SweepStrategy,
    tol: Tol,
) -> OpResult<T> {
    let a_table = read_table(value_of(results_a, a)?);
    let b_table = read_table(value_of(results_b, b)?);
    let inputs = BooleanInputs {
        given: None,
        records_a: records_of(value_of(results_a, a)?).carried(),
        records_b: records_of(value_of(results_b, b)?).carried(),
    };
    let body_a = finished_operand(results_a, a, tol)?;
    let body_b = finished_operand(results_b, b, tol)?;
    match (verb.build)(BooleanOp::Subtract, inputs)
        .run_pair(&body_a, &body_b, boolean_sweep, tol)
        .map_err(verb_refused)?
    {
        verbs::PairOut::Empty => Ok(OpOut::plain(
            ValuePayload::Boolean(BooleanValue::Empty),
            names::empty(),
        )),
        verbs::PairOut::Out(out) => {
            // Another record family from a boolean run is a kernel
            // bug, refused typed.
            let crate::verbs::boolean::BooleanRecord {
                kind,
                contacts,
                naming,
            } = crate::verbs::read_record(out.record, verb.record, verb.foreign_record)?;
            let emitted = (verb.emitter)(
                id,
                BooleanOp::Subtract,
                &out.body,
                &naming,
                &names::OperandCtx {
                    node: a,
                    read: a_read,
                    table: &a_table,
                    body: &body_a,
                },
                &names::OperandCtx {
                    node: b,
                    read: b_read,
                    table: &b_table,
                    body: &body_b,
                },
                tol,
            )
            .map_err(NodeErrorKind::Naming)?;
            let body = out.body.into_body();
            OpOut::plain(
                ValuePayload::Boolean(BooleanValue::Body {
                    body: Arc::new(body),
                    kind,
                    contacts: Arc::new(contacts),
                }),
                emitted.table,
            )
            .grouped(Arc::new(names::FragmentGroups::minted(&emitted.groups)))
            .citing(vec![
                crate::coincide::CitedInput::Read(a_read),
                crate::coincide::CitedInput::Read(b_read),
            ])
            .recording(
                &out.coincidences,
                &crate::coincide::RowInputs {
                    a: (a_read, &a_table),
                    b: Some((b_read, &b_table)),
                    tool: None,
                },
            )
        }
    }
}

/// **Where a row was read from**, beside its member-space cells.
#[derive(Clone, Debug, PartialEq, Eq)]
enum RowOrigin {
    /// A judged row, from the pair of members `pair` (operands A, B).
    Judged {
        /// The judged pair.
        pair: (usize, usize),
    },
    /// A fold row, with the fold-minted names [`names::fold_cell`]
    /// read as member cells (a crossing read as its edge), sorted. Two
    /// crossings of one member edge spell alike, and the names they were
    /// minted under are what keep their two points apart.
    Fold(Vec<names::StableName>),
}

/// Where `rows` holds the statement `row` makes, pushing it where none
/// does; `origins` is `rows`' [`RowOrigin`]s, kept beside it. A
/// statement is the unordered pair of cells and the relation between
/// them: which cell a decision read first follows operand order and
/// sweep order, and says nothing about the coincidence. Beyond that:
///
/// - two judged rows are one statement only when two different pairs
///   made them and they agree exactly, site, margin and discharge too:
///   that is a member read twice, whose pairs are bit-identical runs.
///   One pair's two rows are two decisions (an edge touching a curved
///   face at two isolated points), and both publish;
/// - two fold rows read from different minted names are two
///   statements, since each sites another point;
/// - a judged row states its fact for every fold row spelled as it.
fn stated_at(
    rows: &mut Vec<crate::coincide::NamedCoincidence>,
    origins: &mut Vec<RowOrigin>,
    row: crate::coincide::NamedCoincidence,
    origin: RowOrigin,
) -> usize {
    let [a, b] = &row.cells;
    let same = |(r, o): (&crate::coincide::NamedCoincidence, &RowOrigin)| {
        let [c, d] = &r.cells;
        let alike = r.relation == row.relation && ((a == c && b == d) || (a == d && b == c));
        alike
            && match (o, &origin) {
                (RowOrigin::Judged { pair: p }, RowOrigin::Judged { pair: q }) => {
                    p != q
                        && r.site == row.site
                        && r.margin == row.margin
                        && r.discharge == row.discharge
                }
                (RowOrigin::Fold(x), RowOrigin::Fold(y)) => x == y,
                _ => true,
            }
    };
    rows.iter()
        .zip(origins.iter())
        .position(same)
        .unwrap_or_else(|| {
            rows.push(row);
            origins.push(origin);
            rows.len() - 1
        })
}

/// **The judged rows a fold row restates** (DM4, "A row names member
/// cells"): a fold row putting a point of one member's straight edge on
/// another member's straight edge (`VertexFusion`, `OnCarrier`, both
/// cells member edges, as [`names::fold_cell`] reads a vertex the fold
/// minted on an edge) states that the two lines are one there. Their
/// pair's judgement already holds that as the rows putting an end of
/// either edge on the other edge itself: two such rows are two distinct
/// points on both lines, so the lines are one, and they back the
/// judgement's own record of the edges' overlap. Those rows, by their
/// place in `judged`; empty where the row is not of that shape, an edge
/// is not straight, or the judgement holds fewer than two such rows,
/// and the fold row is its own.
///
/// # Errors
///
/// [`UNION_ROW_CELL_UNRESOLVED`] where a cell of a judged row or of the
/// fold row is a name its member's table does not hold uniquely.
fn restated_overlap<T: Decide>(
    row: &crate::coincide::NamedCoincidence,
    judged: &[crate::coincide::NamedCoincidence],
    members: &[Member<T>],
) -> Result<Vec<usize>, names::NamingError> {
    use crate::coincide::NamedCell;
    use names::{EntityKey, Entry};
    fn edge_of(cell: &NamedCell) -> Option<(crate::VarId, &names::StableName)> {
        match cell {
            NamedCell::Entity { input, name } if name.kind == names::EntityKind::Edge => {
                Some((*input, name))
            }
            _ => None,
        }
    }
    let on_edge = |r: &crate::coincide::NamedCoincidence| {
        r.relation == topo::Relation::OnCarrier && r.site == topo::DecisionSite::VertexFusion
    };
    let [Some(e), Some(f)] = row.cells.each_ref().map(edge_of) else {
        return Ok(Vec::new());
    };
    if !on_edge(row) || e.0 == f.0 {
        return Ok(Vec::new());
    }
    let unresolved = || names::NamingError::Emission {
        what: UNION_ROW_CELL_UNRESOLVED,
    };
    let key = |input: crate::VarId, name: &names::StableName| {
        let member = members
            .iter()
            .find(|m| m.read == input)
            .ok_or_else(unresolved)?;
        match member.table.lookup(name) {
            Some(Entry::Unique(r)) => Ok((member, r.key)),
            Some(Entry::Tied(_)) | None => Err(unresolved()),
        }
    };
    let straight = |(input, edge): (crate::VarId, &names::StableName)| {
        Ok::<_, names::NamingError>(match key(input, edge)? {
            (m, EntityKey::Edge(k)) => m
                .body
                .get_edge(k)
                .and_then(|g| m.body.get_curve_geom(g.curve))
                .and_then(|g| g.certified())
                .is_some_and(|c| matches!(c.carrier(), geom::Curve3::Line { .. })),
            _ => false,
        })
    };
    if !(straight(e)? && straight(f)?) {
        return Ok(Vec::new());
    }
    type Edge<'n> = (crate::VarId, &'n names::StableName);
    // `vertex` is an end of `of`, in its member, and `cell` is `on`
    // itself.
    let end_on = |vertex: &NamedCell, cell: &NamedCell, of: Edge<'_>, on: Edge<'_>| {
        let NamedCell::Entity { input, name } = vertex else {
            return Ok::<_, names::NamingError>(false);
        };
        if edge_of(cell) != Some(on) || *input != of.0 || name.kind != names::EntityKind::Vertex {
            return Ok(false);
        }
        Ok(match (key(*input, name)?, key(of.0, of.1)?) {
            ((m, EntityKey::Vertex(v)), (_, EntityKey::Edge(k))) => m
                .body
                .edges_of_vertex(v)
                .is_some_and(|edges| edges.contains(&k)),
            _ => false,
        })
    };
    let mut restated = Vec::new();
    for (i, r) in judged.iter().enumerate() {
        if !on_edge(r) {
            continue;
        }
        let [c, d] = &r.cells;
        if end_on(c, d, e, f)? || end_on(c, d, f, e)? || end_on(d, c, e, f)? || end_on(d, c, f, e)?
        {
            restated.push(i);
        }
    }
    Ok(if restated.len() >= 2 {
        restated
    } else {
        Vec::new()
    })
}

/// A cell of a union's row names nothing its member's table holds
/// uniquely.
const UNION_ROW_CELL_UNRESOLVED: &str =
    "a union row's cell is no name its member's table holds uniquely";

/// A fold row in member space (DM4, "A row names member cells"): each
/// cell named in the node's own space, `own_read`, by its collapsed fold
/// name, is the member cell that name lies in, or stays the node's where
/// the fold minted it ([`names::fold_cell`]).
fn in_member_space(
    row: crate::coincide::NamedCoincidence,
    own_read: crate::VarId,
) -> Result<(crate::coincide::NamedCoincidence, Vec<names::StableName>), names::NamingError> {
    use crate::coincide::NamedCell;
    let mut minted = Vec::new();
    let mut cell = |cell: NamedCell| match cell {
        NamedCell::Entity { input, name } if input == own_read => {
            Ok(match names::fold_cell(&name)? {
                names::FoldCell::Member { read, name: of } => {
                    if !matches!(name.path.first(), Some(names::RoleSeg::From { .. })) {
                        minted.push(name);
                    }
                    NamedCell::Entity {
                        input: read,
                        name: of,
                    }
                }
                names::FoldCell::Minted(name) => NamedCell::Entity {
                    input: own_read,
                    name,
                },
            })
        }
        _ => Err(names::NamingError::Emission {
            what: UNION_FOLD_ROW_FOREIGN_CELL,
        }),
    };
    let [a, b] = row.cells;
    let cells = [cell(a)?, cell(b)?];
    minted.sort();
    Ok((crate::coincide::NamedCoincidence { cells, ..row }, minted))
}

/// **The union's and the intersect's lowering** (DM4): the SAME pair
/// verb, folded over the members in list order — `((m0 ∘ m1) ∘ m2) ∘ …`,
/// through the same `run_pair` door and the same refusal menu, one body
/// out in the `BooleanValue::Body` shape a subtract yields.
///
/// **One member builds that body and none the typed empty body**: no
/// boolean runs, and a lone member's entities are named by its read as
/// in any union ([`names::member_view`]), under this node's own body.
///
/// No new numeric decision is taken here: the geometry is the pair
/// verb's at every step. What the node adds is the NAMING — the fold's
/// tables record join depth, and `names::name_union` rewrites the last
/// one into read-keyed names. A read spelled twice is two members that
/// glue to each other (DM5): every cell meets its twin, and a glued row
/// is that one name (N3).
///
/// **Contact is judged before the fold, pairwise** (DM4's contact
/// rule): [`judge_pairwise_contact`] runs each member pair whose boxes
/// meet as its own two-member union or intersect, so its verdicts and refusals are the same in every member
/// order, and the face pairs each judgement consumed are the node's face
/// links (N2, [`names::UnionLinks`]). **One verdict per carrier pair**:
/// each pair a judgement glued ([`PairVerdict`]) is given to the fold
/// step that joins the later-listed of its two members, between that
/// member's face and every accumulation face that descends from the
/// other member's (the fold's lineage, [`names::UnionFold`]), whichever
/// of the two was the judgement's operand A; and the step decides no carrier pair of its own
/// (`topo::Verdicts::Given`). A refusal only a step raises names the
/// member that step folds in ([`NodeErrorKind::UnionFoldStep`]). Two
/// members are one pair: their one step is the judgement, run and
/// logged once, its refusals the pair's own.
///
/// **Nothing ∅-absorbing is invented** (D3). A member that evaluates to
/// an empty boolean refuses `EmptyOperand` naming that member. An
/// intersect whose pair or fold step is empty is the typed empty
/// result. A union fold step that returns empty from two real bodies is
/// a kernel bug, at every step including the last: it refuses
/// [`UNION_STEP_EMPTY`] and names no member, since each one is fine.
#[allow(clippy::too_many_arguments)] // one parameter per named input, as `wire_subtract`
fn wire_combine<T: Decide + geom_core::Bounds + topo::AtRestPolicy>(
    verb: &crate::verbs::boolean::PairVerb<T>,
    (op, node, bodies): (
        BooleanOp,
        &Node<ProfileProgram>,
        &crate::Bodies<crate::BodyRead<crate::VarId>>,
    ),
    id: RecipeNodeId,
    (results, vals): (&Results<T>, &SlotValues<T>),
    doc: &crate::doc::Doc<ProfileProgram>,
    boolean_sweep: topo::SweepStrategy,
    tol: Tol,
) -> OpResult<T> {
    // The members, read off the `Bodies` argument (DM4): each spelled
    // read, projected one by one (two ports of one split are two bodies,
    // which one map keyed by the split cannot hold), or each member of a
    // family read whole, in index order, its table the family's rows for
    // that member.
    use crate::OperandSlot as O;
    let members: Vec<Member<T>> = match bodies {
        crate::Bodies::Spelled(reads) => reads
            .iter()
            .enumerate()
            .map(|(i, member)| {
                let read = member.read;
                let slot = O::Member(u32::try_from(i).unwrap_or(u32::MAX));
                let at = super::read_at(doc, slot, read)?;
                let projected = reads_projected(node, &[(slot, read)], doc, (results, vals), tol)?;
                let results = projected.as_ref().unwrap_or(results);
                Ok::<_, NodeErrorKind>(Member {
                    read,
                    body: Arc::new(finished_operand(results, at, tol)?),
                    table: read_table(value_of(results, at)?),
                    records: records_of(value_of(results, at)?),
                })
            })
            .collect::<Result<Vec<_>, NodeErrorKind>>(),
        crate::Bodies::Family(family) => {
            // Every door refuses an indexed family argument
            // (`InputFault::IndexedFamily`); one reaching here would
            // union the whole family, so it is refused, not read.
            if family.is_member() {
                return Err(NodeErrorKind::Naming(names::NamingError::Emission {
                    what: UNION_FAMILY_INDEXED,
                }));
            }
            let read = &family.read;
            let at = super::read_at(doc, O::Members, *read)?;
            let value = value_of(results, at)?;
            let ValuePayload::Instances(bodies) = &value.payload else {
                return Err(wrong_operand(value, at, super::family::INSTANCES));
            };
            bodies
                .iter()
                .enumerate()
                .map(|(i, body)| {
                    let index = u32::try_from(i).unwrap_or(u32::MAX);
                    let table = read_table(value)
                        .project(index)
                        .map_err(|dup| NodeErrorKind::Naming(names::NamingError::from(dup)))?;
                    names::check_total(&table, body, 0).map_err(NodeErrorKind::Naming)?;
                    let body = T::gate_at_rest_kept((**body).clone(), tol)
                        .map_err(|errors| NodeErrorKind::UnfinishedOperand { input: at, errors })?;
                    Ok(Member {
                        read: *read,
                        body: Arc::new(body),
                        table: Arc::new(table),
                        // A family's records are the family's, not one
                        // member's: a member read whole cites none.
                        records: Arc::default(),
                    })
                })
                .collect::<Result<Vec<_>, NodeErrorKind>>()
        }
    }?;
    let members = members.as_slice();
    let empty = || {
        Ok(OpOut::plain(
            ValuePayload::Boolean(BooleanValue::Empty),
            names::empty(),
        ))
    };
    let (first, rest) = match members {
        [] => return empty(),
        [one] => return lone_member(id, one),
        [first, rest @ ..] => (first, rest),
    };
    let own: Vec<(crate::VarId, &NameTable)> =
        members.iter().map(|m| (m.read, m.table.as_ref())).collect();
    // The node's own read: the space a fold row is published in.
    let own_read =
        doc.output(id, 0)
            .ok_or(NodeErrorKind::Naming(names::NamingError::Emission {
                what: UNION_NO_OUTPUT,
            }))?;
    // Every member's read-keyed view, taken once for the pairwise
    // judgement and the fold. The FIRST member enters read-keyed too,
    // so every operand of every step is already in this node's name
    // space.
    let views = members
        .iter()
        .map(|m| {
            names::member_view(id, m.read, &m.table)
                .map(Arc::new)
                .map_err(NodeErrorKind::Naming)
        })
        .collect::<Result<Vec<_>, NodeErrorKind>>()?;
    let mut acc_body = Arc::clone(&first.body);
    let mut acc_table = Arc::clone(&views[0]);
    // Contact is judged here, pairwise, and nowhere else (DM4). Each
    // member's box is the separation certificate's hull, read through
    // the certificate's one box door; a judged pair's body is read only
    // for where its classified edge pieces lie ([`names::UnionLinks`]).
    let hulls = members
        .iter()
        .map(|m| {
            topo::Separation::of(m.body.as_ref(), tol)
                .map(|s| s.hull())
                .map_err(NodeErrorKind::Boolean)
        })
        .collect::<Result<Vec<_>, NodeErrorKind>>()?;
    // Two members are one pair: its one fold step IS its judgement, run
    // once and logged once, as the pair verb runs it. Three or more are
    // judged pairwise first, and each step is handed those verdicts.
    let pairwise = members.len() > 2;
    let judged = if !pairwise {
        Some((names::UnionLinks::default(), Vec::new(), Vec::new()))
    } else {
        let reads: Vec<crate::VarId> = members.iter().map(|m| m.read).collect();
        judge_pairwise_contact(&hulls, &reads, |a, b| {
            let out = (verb.build)(op, BooleanInputs::default())
                .run_pair(&members[a].body, &members[b].body, boolean_sweep, tol)
                .map_err(verb_refused)?;
            let verbs::PairOut::Out(out) = out else {
                return match op {
                    BooleanOp::Intersect => Ok(None),
                    _ => Err(NodeErrorKind::Naming(names::NamingError::Emission {
                        what: UNION_PAIR_EMPTY,
                    })),
                };
            };
            // The judgement's rows, whole, named in the two members' own
            // tables: the node's coincidences are its member pairs' (DM4),
            // each pair's in the order the pass visits it.
            let rows = crate::coincide::name_rows(
                &out.coincidences,
                &crate::coincide::RowInputs {
                    a: own[a],
                    b: Some(own[b]),
                    tool: None,
                },
            )
            .map_err(NodeErrorKind::Naming)?;
            match out.record {
                verbs::VerbRecord::Boolean {
                    naming, verdicts, ..
                } => Ok(Some((
                    (naming, out.body),
                    rows,
                    pair_verdicts(a, b, &verdicts),
                ))),
                _ => Err(NodeErrorKind::Naming(names::NamingError::Emission {
                    what: verb.foreign_record,
                })),
            }
        })?
    };
    let Some((mut links, judged, verdicts)) = judged else {
        return empty();
    };
    // One row per statement: two pairs that state one thing (a member
    // read twice, beside a third) publish it once.
    let mut coincidences = Vec::with_capacity(judged.len());
    let mut origins = Vec::with_capacity(judged.len());
    for (row, pair) in judged {
        stated_at(
            &mut coincidences,
            &mut origins,
            row,
            RowOrigin::Judged { pair },
        );
    }
    let judged_rows = coincidences.len();
    let mut last: Option<(topo::BooleanResultKind, Arc<topo::ContactRecords>)> = None;
    // What the end pass reads of every step: each face's member faces
    // and each step's discards.
    let mut fold = names::UnionFold::new(0, &acc_body);
    // The accumulation's own records in the node's terms: each cites a
    // fold row (`fold_rows`) or a member's record, by its place in the
    // member list. Each step carries them in, as it carries the joining
    // member's.
    let mut acc_records = Arc::new(first.records.carried_from(0));
    let mut fold_rows: Vec<(crate::coincide::NamedCoincidence, Vec<names::StableName>)> =
        Vec::new();
    // Each step's fragment groups, in fold order (`FragmentGroups::folded`).
    let mut step_groups = Vec::with_capacity(rest.len());
    for step in 0..rest.len() {
        let member_body = Arc::clone(&members[step + 1].body);
        let member_table = Arc::clone(&views[step + 1]);
        let mut inputs = BooleanInputs {
            given: None,
            records_a: acc_records.carried(),
            records_b: members[step + 1].records.carried(),
        };
        if pairwise {
            inputs.given = Some(given_pairs(&verdicts, step + 1, &fold, &acc_body)?);
        }
        // A pair's refusal is the pair's own; past two members a refusal
        // only a step raises names the member that step folds in.
        let step_refusal = |refusal| {
            if pairwise {
                NodeErrorKind::UnionFoldStep {
                    member: u32::try_from(step + 1).unwrap_or(u32::MAX),
                    read: members[step + 1].read,
                    refusal: Box::new(refusal),
                }
            } else {
                refusal
            }
        };
        match (verb.build)(op, inputs.clone())
            .run_pair(&acc_body, &member_body, boolean_sweep, tol)
            .map_err(|err| step_refusal(verb_refused(err)))?
        {
            verbs::PairOut::Empty if op == BooleanOp::Intersect => return empty(),
            // A union of two REAL bodies cannot be empty, so this is a
            // kernel bug. It is NOT attributed to a member, which would
            // send a caller to edit a member that is fine.
            verbs::PairOut::Empty => {
                return Err(NodeErrorKind::Naming(names::NamingError::Emission {
                    what: UNION_STEP_EMPTY,
                }));
            }
            verbs::PairOut::Out(out) => {
                if pairwise {
                    debug_assert_given(&out.coincidences, &inputs);
                }
                let verbs::VerbRecord::Boolean {
                    kind,
                    contacts,
                    naming,
                    verdicts: _,
                } = out.record
                else {
                    return Err(NodeErrorKind::Naming(names::NamingError::Emission {
                        what: verb.foreign_record,
                    }));
                };
                if !pairwise {
                    // The pair's rows and links are its one step's, in
                    // the two members' own tables and the order its
                    // records cite them, as a pair verb records them.
                    coincidences = crate::coincide::name_rows(
                        &out.coincidences,
                        &crate::coincide::RowInputs {
                            a: own[0],
                            b: Some(own[1]),
                            tool: None,
                        },
                    )
                    .map_err(NodeErrorKind::Naming)?;
                    links
                        .judged(0, 1, &naming, &out.body)
                        .map_err(NodeErrorKind::Naming)?;
                }
                // The step's records in the node's terms: a row the step
                // decided is a fold row, named in the node's own space
                // (#4323: a fold row is published where a record cites
                // it); an accumulation record stands for what it cited;
                // the joining member's is that member's.
                let base = fold_rows.len();
                let used = contacts.decided();
                if pairwise {
                    let cited =
                        used.iter()
                            .map(|&k| {
                                out.coincidences.get(k as usize).copied().ok_or(
                                    NodeErrorKind::Naming(names::NamingError::Emission {
                                        what: UNION_STEP_UNDECIDED_CITE,
                                    }),
                                )
                            })
                            .collect::<Result<Vec<_>, _>>()?;
                    let acc_view =
                        names::collapse_table(id, &acc_table).map_err(NodeErrorKind::Naming)?;
                    for row in crate::coincide::name_rows(
                        &cited,
                        &crate::coincide::RowInputs {
                            a: (own_read, &acc_view),
                            b: Some((own_read, &member_table)),
                            tool: None,
                        },
                    )
                    .map_err(NodeErrorKind::Naming)?
                    {
                        fold_rows
                            .push(in_member_space(row, own_read).map_err(NodeErrorKind::Naming)?);
                    }
                }
                let member = u32::try_from(step + 1).unwrap_or(u32::MAX);
                let held = Arc::clone(&acc_records);
                let refuse = |what| NodeErrorKind::Naming(names::NamingError::Emission { what });
                let contacts = contacts.try_recited(
                    |b| match b {
                        topo::Backing::Decided(k) if pairwise => used
                            .binary_search(&k)
                            .map(|rank| {
                                vec![topo::Backing::Decided(
                                    u32::try_from(base + rank).unwrap_or(u32::MAX),
                                )]
                            })
                            .map_err(|_| refuse(UNION_STEP_UNDECIDED_CITE)),
                        decided @ topo::Backing::Decided(_) => Ok(vec![decided]),
                        topo::Backing::Carried { input: 0, record } => held
                            .rows()
                            .get(record as usize)
                            .map(|c| c.cites.iter().collect())
                            .ok_or_else(|| refuse(UNION_CARRY_LOST_RECORD)),
                        topo::Backing::Carried { record, .. } => Ok(vec![topo::Backing::Carried {
                            input: member,
                            record,
                        }]),
                    },
                    |_| refuse(UNION_CARRY_UNCITED),
                )?;
                acc_records = Arc::new(contacts);
                last = Some((kind, Arc::clone(&acc_records)));
                // Minted under THIS node's id, its two sides keyed by the
                // fold's own reads, which tell an intermediate row from a
                // member's own name when the chain is collapsed.
                let emitted = (verb.emitter)(
                    id,
                    op,
                    &out.body,
                    &naming,
                    &names::OperandCtx {
                        node: id,
                        read: names::FOLD_A,
                        table: &acc_table,
                        body: &acc_body,
                    },
                    &names::OperandCtx {
                        node: id,
                        read: names::FOLD_B,
                        table: &member_table,
                        body: &member_body,
                    },
                    tol,
                )
                .map_err(NodeErrorKind::Naming)?;
                fold.step(step + 1, &naming, &out.body)
                    .map_err(NodeErrorKind::Naming)?;
                acc_table = emitted.table;
                step_groups.push(emitted.groups);
                acc_body = Arc::new(out.body);
            }
        }
    }
    // The members' own bodies and tables: where each member edge the
    // published table ranks pieces of is defined (`name_union`).
    let member_views: Vec<names::UnionMember<'_, T>> = members
        .iter()
        .map(|m| names::UnionMember {
            read: m.read,
            body: &m.body,
            table: &m.table,
        })
        .collect();
    let (table, published_groups) =
        names::name_union(id, &acc_body, &acc_table, &member_views, &fold, &links, tol)
            .map_err(NodeErrorKind::Naming)?;
    let body = (*acc_body).clone().into_body();
    // The LAST step's records are the result's: every earlier step's
    // were carried into the step after it.
    let Some((kind, contacts)) = last else {
        return Err(NodeErrorKind::Naming(names::NamingError::Emission {
            what: UNION_NO_STEP,
        }));
    };
    // A fold row the node's records cite is the published row stating
    // the same thing where there is one (DM4: a judged row, or a fold
    // row an earlier citation published), the judged rows it restates
    // where those state it ([`restated_overlap`]), and is published
    // after the judgement's otherwise; the records are recited onto the
    // rows they cite.
    let contacts = if pairwise {
        let used = contacts.decided();
        let mut at = Vec::with_capacity(used.len());
        for &k in &used {
            let row = fold_rows
                .get(k as usize)
                .cloned()
                .ok_or(NodeErrorKind::Naming(names::NamingError::Emission {
                    what: UNION_STEP_UNDECIDED_CITE,
                }))?;
            let (row, minted) = row;
            let restated = restated_overlap(&row, &coincidences[..judged_rows], members)
                .map_err(NodeErrorKind::Naming)?;
            at.push(if restated.is_empty() {
                vec![stated_at(
                    &mut coincidences,
                    &mut origins,
                    row,
                    RowOrigin::Fold(minted),
                )]
            } else {
                restated
            });
        }
        let refuse = |what| NodeErrorKind::Naming(names::NamingError::Emission { what });
        Arc::new((*contacts).clone().try_recited(
            |b| {
                match b {
                    topo::Backing::Decided(k) => used
                        .binary_search(&k)
                        .map(|rank| {
                            at[rank]
                                .iter()
                                .map(|&i| {
                                    topo::Backing::Decided(u32::try_from(i).unwrap_or(u32::MAX))
                                })
                                .collect()
                        })
                        .map_err(|_| refuse(UNION_STEP_UNDECIDED_CITE)),
                    carried @ topo::Backing::Carried { .. } => Ok(vec![carried]),
                }
            },
            |_| refuse(UNION_CARRY_UNCITED),
        )?)
    } else {
        contacts
    };
    Ok(OpOut::plain(
        ValuePayload::Boolean(BooleanValue::Body {
            body: Arc::new(body),
            kind,
            contacts,
        }),
        table,
    )
    .grouped(Arc::new(names::FragmentGroups::folded(
        id,
        &step_groups,
        &published_groups,
    )))
    .with_coincidences(coincidences)
    // The records may cite the members' own, each by its place in the
    // list.
    .citing(
        members
            .iter()
            .map(|m| crate::coincide::CitedInput::Read(m.read))
            .collect(),
    ))
}

/// **A union or an intersect of one body is that body** (DM4): no
/// boolean runs. Its entities are named by the member's read
/// ([`names::member_view`]), and the body itself by this node.
fn lone_member<T: Decide>(id: RecipeNodeId, one: &Member<T>) -> OpResult<T> {
    let table = names::name_lone_member(id, one.read, &one.table).map_err(NodeErrorKind::Naming)?;
    let body = (*one.body).clone().into_body();
    names::check_total(&table, &body, 0).map_err(NodeErrorKind::Naming)?;
    Ok(OpOut::plain(
        ValuePayload::Body(Arc::new(body)),
        Arc::new(table),
    ))
}

/// A union's or an intersect's family argument carries an index, which
/// every door refuses.
const UNION_FAMILY_INDEXED: &str =
    "a union's or an intersect's family argument carries an index the doors refuse";

/// A union or an intersect the document defines no output for.
const UNION_NO_OUTPUT: &str = "a union or an intersect defines no output variable";

/// A union fold row's cell is not named in the node's own space, the
/// only space a fold step's rows are named in.
const UNION_FOLD_ROW_FOREIGN_CELL: &str =
    "a union fold row's cell is not named in the node's own space";

/// A union fold step's record cites a row the step did not decide.
const UNION_STEP_UNDECIDED_CITE: &str = "a union fold step's record cites a row it did not decide";

/// A union fold step's record cites an accumulation record the fold
/// does not hold.
const UNION_CARRY_LOST_RECORD: &str =
    "a union fold step's record cites an accumulation record the fold does not hold";

/// A split's value holds no name table for one of its halves.
const SPLIT_HALF_UNNAMED: &str = "a split's value holds no name table for one of its halves";

/// A union fold step's record cites nothing once carried into the
/// node's terms.
const UNION_CARRY_UNCITED: &str = "a union fold step's record cites nothing in the node's terms";

/// A union's or an intersect's fold of two members or more ran no step.
const UNION_NO_STEP: &str = "a union fold of two members or more ran no step";

/// **DM4's contact rule: every member pair is judged as its own
/// two-member union, before the fold** — the one place a union's
/// contacts are decided.
///
/// Each pair of members whose closed boxes meet
/// ([`topo::Separation::hull`]) runs the pair verb as `m ∪ n`: it
/// glues what its margins decide one carrier and refuses a sliver, as
/// the pair boolean does. A pair whose boxes do not meet is not run.
///
/// **Whether it refuses depends on the members, never on their
/// order**: every pair whose boxes meet is judged, in the author's list
/// order (#4323: the list is the author's stated order, and nothing
/// here sorts it away). So the rows come out in that order, and where
/// two pairs would refuse, the one the list reaches first is the one
/// named. Within a pair, operand A is the member whose read `reads`
/// holds as the lesser (the earlier minted), the listed-first on a
/// read spelled twice: so one pair is one run of the pair verb in
/// every member order, and publishes one margin.
///
/// `judge(a, b)` runs the pair verb on members `a` (operand A) and `b`
/// (operand B) and hands back its record; this function decides which
/// pairs are judged and which member is A, and nothing about geometry.
///
/// Returns the member faces the judgements consumed
/// ([`names::UnionLinks`]): the one place a union's faces are linked
/// (N2), so the links are member order's no more than the verdict is;
/// the judgements' rows, each beside the pair that made it; and the carrier pairs they glued, the fold's
/// one verdict per pair ([`PairVerdict`]). `None` where a judged
/// intersection is empty.
#[allow(clippy::type_complexity)] // the judge's one record, or none for an empty intersection
fn judge_pairwise_contact<T: Decide>(
    hulls: &[bvh::Aabb],
    reads: &[crate::VarId],
    mut judge: impl FnMut(
        usize,
        usize,
    ) -> Result<
        Option<(
            (topo::BooleanNaming, topo::AtRestBody<T>),
            Vec<crate::coincide::NamedCoincidence>,
            Vec<PairVerdict>,
        )>,
        NodeErrorKind,
    >,
) -> Result<
    Option<(
        names::UnionLinks<T>,
        Vec<(crate::coincide::NamedCoincidence, (usize, usize))>,
        Vec<PairVerdict>,
    )>,
    NodeErrorKind,
> {
    let mut links = names::UnionLinks::default();
    let mut rows = Vec::new();
    let mut verdicts = Vec::new();
    // Each pair once, in the author's list order (#4323: the list is
    // the author's stated order; nothing here sorts it away).
    for p in 0..hulls.len() {
        for q in p + 1..hulls.len() {
            if !hulls[p].overlaps(&hulls[q]) {
                continue;
            }
            let (a, b) = if reads[q] < reads[p] { (q, p) } else { (p, q) };
            let Some(((naming, body), judged, glued)) = judge(a, b)? else {
                return Ok(None);
            };
            links
                .judged(a, b, &naming, &body)
                .map_err(NodeErrorKind::Naming)?;
            rows.extend(judged.into_iter().map(|r| (r, (a, b))));
            verdicts.extend(glued);
        }
    }
    Ok(Some((links, rows, verdicts)))
}

/// **One carrier pair a union's pairwise judgement glued**: the
/// kernel's verdict and the two members it judged, by member index.
/// The fold reads it through lineage ([`given_pairs`]) and decides
/// the pair nowhere else.
#[derive(Clone, Copy, Debug)]
struct PairVerdict {
    /// The judgement's operand A.
    a: usize,
    /// Its operand B.
    b: usize,
    /// The verdict, in the two members' keys.
    verdict: topo::GlueVerdict,
}

/// The carrier pairs the judgement of members `a` (operand A) and `b`
/// (operand B) glued: the kernel's verdicts ([`topo::GlueVerdict`]).
fn pair_verdicts(a: usize, b: usize, verdicts: &[topo::GlueVerdict]) -> Vec<PairVerdict> {
    verdicts
        .iter()
        .map(|&verdict| PairVerdict { a, b, verdict })
        .collect()
}

/// **The pass's verdicts for the fold step that folds in member
/// `member`**, the step's given pairs ([`BooleanInputs::given`]): each
/// verdict one of whose faces is `member`'s, between that face and
/// every accumulation face whose lineage holds the verdict's other
/// face. The judgement's operand A may be either member, so either face
/// may be `member`'s.
///
/// # Errors
///
/// An emission bug ([`UNION_VERDICTS_DISAGREE`]) where two verdicts
/// glue one accumulation face to one member face under two relations,
/// or an accumulation face carries no lineage.
fn given_pairs<T: Decide>(
    verdicts: &[PairVerdict],
    member: usize,
    fold: &names::UnionFold,
    acc: &topo::Body<T>,
) -> Result<Vec<topo::GluedPair>, NodeErrorKind> {
    use topo::Operand;
    let bug = |what| NodeErrorKind::Naming(names::NamingError::Emission { what });
    let mut given: std::collections::BTreeMap<
        (topo::FaceKey, topo::FaceKey),
        (Operand, topo::GlueVerdict),
    > = std::collections::BTreeMap::new();
    for (f, _) in acc.faces() {
        let parents = fold.member_faces(f).ok_or(bug(UNION_FACE_NO_LINEAGE))?;
        for v in verdicts {
            // The accumulation's side of the verdict, and `member`'s face.
            let (held, at) = if v.b == member {
                (Operand::A, (v.a, v.verdict.a()))
            } else if v.a == member {
                (Operand::B, (v.b, v.verdict.b()))
            } else {
                continue;
            };
            if !parents.contains(&at) {
                continue;
            }
            let joining = match held {
                Operand::A => v.verdict.b(),
                Operand::B => v.verdict.a(),
            };
            match given.entry((f, joining)) {
                std::collections::btree_map::Entry::Vacant(slot) => {
                    slot.insert((held, v.verdict));
                }
                std::collections::btree_map::Entry::Occupied(prior) => {
                    if prior.get().1.relation() != v.verdict.relation() {
                        return Err(bug(UNION_VERDICTS_DISAGREE));
                    }
                }
            }
        }
    }
    Ok(given
        .into_iter()
        .map(|((f, _), (held, verdict))| verdict.given_at(held, f))
        .collect())
}

/// **A fold step records no face-pair row** (DM4, the fold re-glues no
/// carrier pair): the verdict reuse, asserted where the step's rows come
/// back. A step seeds its glue from its given pairs unmeasured, so the
/// pairwise pass's rows are the union's and the step adds none; a
/// face-pair row from a step is a carrier pair judged again at the
/// piece's extent, whether a given pair re-verified or a fresh glue a
/// door that ignored the given verdicts added. Readings that record no
/// row are outside it: the merge's continuation reading, the
/// maximal-faces gate and the join's coaxial frame read a step's
/// carriers by their margins, as the pairwise pass read the members'.
fn debug_assert_given(rows: &[topo::Coincidence], inputs: &BooleanInputs) {
    if cfg!(debug_assertions) && inputs.given.is_some() {
        for row in rows {
            if let Some((fa, fb)) = row.face_pair() {
                debug_assert!(
                    false,
                    "a union fold step judged the carrier pair {fa:?}/{fb:?} itself: its \
                     verdict is the pairwise judgement's, read through lineage"
                );
            }
        }
    }
}

/// An accumulation face the fold's lineage does not hold.
const UNION_FACE_NO_LINEAGE: &str = "a union fold's accumulation face has no member-face lineage";

/// Two of a union's pairwise verdicts glue one accumulation face to one
/// member face under two relations.
const UNION_VERDICTS_DISAGREE: &str = "a union's pairwise judgements glue one accumulation face to one member face under two relations";

/// A union's pairwise judgement returned the typed empty from two real
/// bodies.
const UNION_PAIR_EMPTY: &str =
    "a union's pairwise judgement returned empty from two non-empty members";

/// A union fold step returned the typed empty from two real bodies.
const UNION_STEP_EMPTY: &str = "a union fold step returned empty from two non-empty operands";

/// The role word a stepped rule's LINEAR direction is normalized
/// under, inside [`SteppedOperands::linear`].
const PATTERN_DIRECTION_ROLE: &str = "pattern direction";

/// The role word a frame's authored +x direction is normalized under
/// ([`frame_axes`]).
pub(crate) const FRAME_X_ROLE: &str = "datum frame x axis";

/// The role word a frame's authored +y direction is normalized under,
/// after Gram-Schmidt ([`frame_axes`]).
pub(crate) const FRAME_Y_ROLE: &str = "datum frame y axis";

/// The role word a plane datum's normal is normalized under.
pub(crate) const PLANE_NORMAL_ROLE: &str = "datum plane normal";

/// The role word a DATUM AXIS's direction is normalized under — one
/// word on both roads, though the funnel name differs by road (see
/// [`unit()`]).
pub(crate) const DATUM_AXIS_ROLE: &str = "datum axis direction";

/// **A placement's copies** (D10): each shape read, projected to the
/// value it reads (a split's half, one copy of a placement's copies, a
/// family's member), placed at the one motion `map` its bundle solved
/// to (`None` the bit-exact identity) and named under this node.
///
/// Over one shape the copy has the shape's shape: a body is one copy
/// ([`names::name_placed`]), a family is a family, and a placement's
/// copies are those copies, keyed as they were. A boolean's empty result
/// places as itself: an empty copy, a typed success (F8) the gather
/// finds no solid in. Over several shapes the copies are keyed by read
/// ([`ValuePayload::Copies`], [`names::name_placed_copies`]), a family
/// among them a family under its key; two reads of one variable key
/// alike and refuse (FORK-DM4).
///
/// A copy of one body carries the body's declared contact records and
/// the declaration rows keyed with them (ASM-R2b D-1) with their cells
/// verbatim: a rigid placement keeps every arena key
/// (`transform_rigid`), so they key the copy as they keyed the body.
/// Each record cites the body's record it is, through the shape's read.
/// A split's half carries none, as its value carries none.
fn wire_place<T: Decide + topo::AtRestPolicy>(
    id: RecipeNodeId,
    shapes: &[(RecipeNodeId, crate::VarId, &Results<T>)],
    map: Option<&RigidMotion<T>>,
    tol: Tol,
) -> OpResult<T> {
    let placed = |body: &Body<T>| place(body, map, tol).map(Arc::new);
    let projected = |table: &names::NameTable, index: u32| {
        table
            .project(index)
            .map_err(|dup| NodeErrorKind::Naming(names::NamingError::from(dup)))
    };
    let citing: Vec<crate::coincide::CitedInput> = shapes
        .iter()
        .map(|&(_, read, _)| crate::coincide::CitedInput::Read(read))
        .collect();
    if let [(of, read, results)] = shapes {
        let value = value_of(results, *of)?;
        let (body, contacts) = match &value.payload {
            ValuePayload::Boolean(BooleanValue::Empty) => {
                return Ok(OpOut::plain(
                    ValuePayload::Boolean(BooleanValue::Empty),
                    names::empty(),
                ));
            }
            ValuePayload::Boolean(BooleanValue::Body { body, contacts, .. }) => {
                (body, Arc::clone(contacts))
            }
            ValuePayload::Body(body) => (body, Arc::clone(&value.contacts)),
            ValuePayload::Instances(bodies) => {
                let bodies = bodies
                    .iter()
                    .map(|b| placed(b))
                    .collect::<Result<Vec<_>, _>>()?;
                let table = name_family(id, &read_table(value), &bodies)?;
                return Ok(OpOut::plain(ValuePayload::Instances(bodies), table)
                    .carrying(value.parts)
                    .citing(citing));
            }
            ValuePayload::Copies(copies) => {
                let bodies = copies
                    .bodies
                    .iter()
                    .map(|b| placed(b))
                    .collect::<Result<Vec<_>, _>>()?;
                let table = name_family(id, &read_table(value), &bodies)?;
                let members = copies
                    .members
                    .iter()
                    .map(|member| {
                        let bodies = member_bodies(member, tol)?
                            .iter()
                            .map(|b| placed(b))
                            .collect::<Result<Vec<_>, _>>()?;
                        Ok(super::KeyedCopy {
                            key: member.key.clone(),
                            table: name_family(id, &member.table, &bodies)?,
                            family: member.family,
                            parts: member.parts,
                            bodies,
                            motion: None,
                        })
                    })
                    .collect::<Result<Vec<_>, NodeErrorKind>>()?;
                // The records ride the one body that keys them.
                let whole = bodies.len() == 1;
                return Ok(OpOut {
                    contacts: if whole {
                        Arc::new(value.contacts.carried_from(0))
                    } else {
                        Arc::default()
                    },
                    carried: if whole {
                        Arc::clone(&value.carried)
                    } else {
                        Arc::default()
                    },
                    ..OpOut::plain(
                        ValuePayload::Copies(Arc::new(super::Copies { bodies, members })),
                        table,
                    )
                    .carrying(value.parts)
                    .citing(citing)
                });
            }
            _ => return Err(wrong_operand(value, *of, super::phrase::BODY_OR_INSTANCES)),
        };
        let table = projected(&read_table(value), 0)?;
        let copy = place(body, map, tol)?;
        let table = names::name_placed(id, &table, &copy).map_err(NodeErrorKind::Naming)?;
        return Ok(OpOut {
            contacts: Arc::new(contacts.carried_from(0)),
            carried: Arc::clone(&value.carried),
            ..OpOut::plain(ValuePayload::Body(Arc::new(copy)), table)
                .carrying(value.parts)
                .citing(vec![crate::coincide::CitedInput::Read(*read)])
        });
    }
    // Several shapes: each copy keyed by its read, its bodies flat in
    // list order, each with the table its value names it by.
    let mut keys: Vec<crate::CopyKey> = Vec::with_capacity(shapes.len());
    let mut flat: Vec<(names::NameTable, Arc<Body<T>>)> = Vec::new();
    let mut spans: Vec<(core::ops::Range<u32>, bool, usize)> = Vec::with_capacity(shapes.len());
    let mut parts = 0;
    for &(of, read, results) in shapes {
        let key = crate::CopyKey::Read(read);
        if keys.contains(&key) {
            return Err(NodeErrorKind::CopiesShareAKey { key: Box::new(key) });
        }
        keys.push(key);
        let value = value_of(results, of)?;
        let start = u32::try_from(flat.len()).unwrap_or(u32::MAX);
        let family = match &value.payload {
            ValuePayload::Boolean(BooleanValue::Empty) => {
                return Err(NodeErrorKind::EmptyOperand { input: of });
            }
            ValuePayload::Boolean(BooleanValue::Body { body, .. }) | ValuePayload::Body(body) => {
                flat.push((projected(&read_table(value), 0)?, placed(body)?));
                false
            }
            ValuePayload::Instances(bodies) => {
                for (i, body) in bodies.iter().enumerate() {
                    let index = names::output_body(i).map_err(NodeErrorKind::Naming)?;
                    flat.push((projected(&read_table(value), index)?, placed(body)?));
                }
                true
            }
            _ => return Err(wrong_operand(value, of, super::phrase::BODY_OR_INSTANCES)),
        };
        parts += value.parts;
        spans.push((
            start..u32::try_from(flat.len()).unwrap_or(u32::MAX),
            family,
            value.parts,
        ));
    }
    let named: Vec<(&names::NameTable, &Body<T>)> = flat
        .iter()
        .map(|(table, body)| (table, body.as_ref()))
        .collect();
    let table = names::name_placed_copies(id, &named).map_err(NodeErrorKind::Naming)?;
    let members = keys
        .into_iter()
        .zip(spans)
        .map(|(key, (span, family, parts))| {
            let bodies = flat[span.start as usize..span.end as usize]
                .iter()
                .map(|(_, body)| Arc::clone(body))
                .collect();
            let table = table
                .reindex(|b| span.contains(&b).then(|| b - span.start))
                .map_err(|dup| NodeErrorKind::Naming(names::NamingError::from(dup)))?;
            Ok(super::KeyedCopy {
                key,
                bodies,
                family,
                parts,
                table: Arc::new(table),
                motion: None,
            })
        })
        .collect::<Result<Vec<_>, NodeErrorKind>>()?;
    Ok(OpOut::plain(
        ValuePayload::Copies(Arc::new(super::Copies {
            bodies: flat.into_iter().map(|(_, body)| body).collect(),
            members,
        })),
        table,
    )
    .carrying(parts)
    .citing(citing))
}

/// **A placed family, named**: body `i` of `bodies` is body `i` of the
/// value `table` names, each of its names qualified by the placement
/// ([`names::name_placed_copies`]).
fn name_family<T: Decide>(
    id: RecipeNodeId,
    table: &names::NameTable,
    bodies: &[Arc<Body<T>>],
) -> Result<Arc<names::NameTable>, NodeErrorKind> {
    let tables = (0..bodies.len())
        .map(|i| {
            let index = names::output_body(i).map_err(NodeErrorKind::Naming)?;
            table
                .project(index)
                .map_err(|dup| NodeErrorKind::Naming(names::NamingError::from(dup)))
        })
        .collect::<Result<Vec<_>, _>>()?;
    let named: Vec<(&names::NameTable, &Body<T>)> = tables
        .iter()
        .zip(bodies)
        .map(|(t, b)| (t, b.as_ref()))
        .collect();
    names::name_placed_copies(id, &named).map_err(NodeErrorKind::Naming)
}

/// **What `node`'s slots read as written** ([`crate::Doc::slot_expansion`]),
/// for a refusal that proposes a re-spelling of one: a slot the node
/// does not carry reads as nothing a proposal could build on.
pub(crate) fn written<P: crate::ProfilePayload>(
    doc: &crate::doc::Doc<P>,
    node: RecipeNodeId,
) -> impl Fn(SlotId) -> crate::Formula + '_ {
    move |slot| {
        doc.slot_expansion(node, slot).unwrap_or_else(|| {
            unreachable!("a refusal proposes a re-spelling of {slot:?}, a slot {node} carries")
        })
    }
}

pub(crate) mod pose;
mod stepped;
pub(crate) use stepped::{
    PATTERN_SPACING, PATTERN_STEP, PATTERN_STEP_TURN, SteppedOperands, stepped_rule_map,
};

/// **The full-turn decision**: the sign of `|angle| − whole`, `whole`
/// a whole number of turns, read by the revolve's full-or-partial
/// wiring (at one turn) and by a circular pattern's step. Radians
/// against the linear band (ledger row F14): the honest lever, the
/// radial extent the angle sweeps at, is the kernel's for the revolve
/// and not in hand for the pattern's derived offset.
fn turns_off<T: Decide>(
    decision: Decision,
    angle: T,
    whole: T,
    band: Band,
) -> Result<Sign, NodeErrorKind> {
    geom_core::k_stats::decide_flagged(decision.name(), angle.max(-angle) - whole, band, "F14")
        .map_err(escalated(decision))
}

/// `decision`, which could not be called, as the node's refusal.
fn escalated(decision: Decision) -> impl FnOnce(geom_core::Indeterminate) -> NodeErrorKind {
    move |source| NodeErrorKind::Escalated {
        predicate: decision.name(),
        source,
    }
}

/// [`stepped_rule_map`] behind the evaluation's slot reads, which stay
/// INSIDE so a rule's operands are demanded only when a step uses
/// them: placement 0 is the identity and reads none. A listed rule
/// refuses as `listed`, the mismatch it is on the caller's node.
/// What a stepped rule reads beyond its own slots: the document its
/// circular axis is read in, at the lane's parameters, and the rule's
/// written formulas.
struct RuleEnv<'a, T: Decide> {
    doc: &'a crate::doc::Doc<ProfileProgram>,
    written: &'a dyn Fn(SlotId) -> crate::Formula,
    params: &'a crate::expr::VarEnv<T>,
    /// The results before the reader's own split ports were projected,
    /// which the pose evaluator projects for its own reads.
    unprojected: &'a Results<T>,
}

fn stepped_map<T: Decide>(
    kind: &PatternKind,
    rule: &RuleEnv<'_, T>,
    listed: crate::node::CountMismatch,
    i: i64,
    vals: &SlotValues<T>,
    tol: Tol,
) -> Result<RigidMotion<T>, NodeErrorKind> {
    let ops = match kind {
        // An explicit rule's frames ARE the maps.
        PatternKind::Explicit(_) => {
            return Err(NodeErrorKind::PlacementRule(
                crate::node::PlacementRuleFault::CountSpelling { shape: listed },
            ));
        }
        _ if i == 0 => return Ok(RigidMotion::identity()),
        PatternKind::Linear { .. } => SteppedOperands::linear(
            need_vec3(vals, SlotId::Direction)?,
            need_scalar(vals, SlotId::Spacing)?,
            &crate::node::Axis3::ALL.map(|axis| (rule.written)(SlotId::Direction(axis))),
            band(tol)?,
        )?,
        PatternKind::Circular { axis, .. } => {
            let PoseValue::Axis { origin, dir } = pose::eval_pose(
                rule.doc,
                rule.unprojected,
                crate::OperandSlot::Axis,
                *axis,
                rule.params,
                tol,
            )?
            else {
                unreachable!("the door admits an axis alone at a circular rule's axis")
            };
            SteppedOperands::circular(
                origin,
                dir,
                need_scalar(vals, SlotId::Step)?,
                &(rule.written)(SlotId::Step),
                band(tol)?,
            )?
        }
    };
    stepped_rule_map(&ops, i)
}

/// **The pattern node**: a stepped rule over its input's value,
/// shape-preserving — a body or an `Instances` value is the MASTER,
/// placed whole — yielding `Instances`.
///
/// **The layout, placement-major** (D9): output body `j·M + i` is
/// placement `j` of the master's body `i`, `M` the master's body count
/// ([`names::flat_body_index`], which the name table is keyed by too).
/// Placement 0 is the master's own bodies verbatim; every master name
/// wraps `Instance(j)` per placement (A8/N1).
#[allow(clippy::too_many_arguments)] // the doc resolves the rule's axis
fn wire_pattern<T: Decide + topo::AtRestPolicy>(
    id: RecipeNodeId,
    input: RecipeNodeId,
    kind: &PatternKind,
    rule: &RuleEnv<'_, T>,
    results: &Results<T>,
    vals: &SlotValues<T>,
    tol: Tol,
) -> OpResult<T> {
    // A pattern's count is its structural SLOT; the edit door refuses
    // an explicit placement list, and this is the hand-built backstop.
    if kind.placements().is_some() {
        return Err(NodeErrorKind::PlacementRule(
            crate::node::PlacementRuleFault::CountSpelling {
                shape: crate::node::CountMismatch::ListedOnPattern,
            },
        ));
    }
    let value = value_of(results, input)?;
    let placeable = placeable_operand(value, input)?;
    let master = placeable.bodies();
    let n = slots::count(vals, SlotId::Count).ok_or(NodeErrorKind::MissingSlot {
        slot: SlotId::Count,
    })?;
    if n < 1 {
        return Err(NodeErrorKind::CountOutOfRange {
            slot: SlotId::Count,
            count: n,
            admits: 1..=i64::MAX,
        });
    }
    let naming = NodeErrorKind::Naming;
    let mut instances: Vec<Arc<Body<T>>> = master.to_vec();
    for j in 1..n {
        let map = stepped_map(
            kind,
            rule,
            crate::node::CountMismatch::ListedOnPattern,
            j,
            vals,
            tol,
        )?;
        instances.extend(place_each(master, &map, tol)?);
    }
    let table =
        names::name_pattern(id, &read_table(value), n, master.len(), &instances).map_err(naming)?;
    Ok(OpOut::plain(ValuePayload::Instances(instances), table).carrying(value.parts))
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
///    asserts nothing about its operands — so an unproved arrangement
///    refuses typed rather than shipping interpenetrating solids.
/// 3. **The lowering**: `graft_disjoint_all_keyed` per placed copy, in
///    placement order, into one aggregate. No seam happens, so no new
///    kernel naming record.
///
/// Every placement is MAPPED, including index 0: an explicit rule need
/// not make it the identity.
#[allow(clippy::too_many_arguments)] // the doc resolves the rule's axis
fn wire_placed_union<T: Decide + geom_core::Bounds + topo::AtRestPolicy>(
    id: RecipeNodeId,
    input: RecipeNodeId,
    kind: &PatternKind,
    rule: &RuleEnv<'_, T>,
    results: &Results<T>,
    vals: &SlotValues<T>,
    tol: Tol,
) -> OpResult<T> {
    let body = body_operand(results, input)?;
    let maps: Vec<RigidMotion<T>> = match kind.placements() {
        // Listed frames are literal: each is admitted at the decided
        // door.
        Some(frames) => {
            let band = band(tol)?;
            frames
                .iter()
                .map(|f| {
                    RigidMotion::check_rigid(&f.affine::<T>(), band).map_err(NodeErrorKind::Motion)
                })
                .collect::<Result<_, _>>()?
        }
        None => {
            let n = slots::count(vals, SlotId::Count).ok_or(NodeErrorKind::MissingSlot {
                slot: SlotId::Count,
            })?;
            if n < 1 {
                return Err(NodeErrorKind::CountOutOfRange {
                    slot: SlotId::Count,
                    count: n,
                    admits: 1..=i64::MAX,
                });
            }
            (0..n)
                .map(|i| {
                    stepped_map(
                        kind,
                        rule,
                        crate::node::CountMismatch::ListedWithCount,
                        i,
                        vals,
                        tol,
                    )
                })
                .collect::<Result<_, _>>()?
        }
    };
    let affines: Vec<Affine3<T>> = maps.iter().map(|m| m.to_affine()).collect();
    topo::Separation::of(body.as_ref(), tol)
        .map_err(NodeErrorKind::Boolean)?
        .certify(&affines)
        .map_err(|topo::PlacementsMeet { i, j }| NodeErrorKind::PlacementsUncertified { i, j })?;
    let mut fused = topo::Body::new();
    let mut bridges: Vec<topo::GraftKeys> = Vec::with_capacity(maps.len());
    for map in &maps {
        let placed = place(&body, Some(map), tol)?;
        // Every placement MINTS its own solids: the copies are certified
        // apart, so each is a piece of its own, and a body of N solids
        // is a boolean operand like any other.
        let keys =
            topo::graft_disjoint_all_keyed(&mut fused, &placed).map_err(NodeErrorKind::Boolean)?;
        bridges.push(keys);
    }
    // Instance(i) wrapping (A8/N1), re-keyed onto the ONE output body
    // through each instance's graft bridge.
    let master = read_table(value_of(results, input)?);
    let table =
        names::name_placed_union(id, &master, &bridges, &fused).map_err(NodeErrorKind::Naming)?;
    Ok(OpOut::plain(ValuePayload::Body(Arc::new(fused)), table))
}

// ---------------------------------------------------------------------
// The definitional §10.3/§10.4 nodes
// ---------------------------------------------------------------------

/// The Sweep node's frontier — the ONE
/// [`NodeErrorKind::CurvedSolidFrontier`] door, a constant so the
/// acceptance rows assert the SAME text, which says why.
pub(crate) const SWEEP_FRONTIER: &str = "a swept solid: the recipe's path operand is a profile LOOP — a \
     closed chain of segments, or a full circle as one segment at one \
     vertex — while §10.4's rigid-profile sweep needs the path as \
     ONE open curve, so every recipe-expressible sweep waits on a \
     joined-path composition lane; the swept BODY machinery itself is \
     live — sweep::sweep_body at the library API";

/// **A loft section: the section operand's Profile value at `T`**, as
/// an extrude reads its profile ([`wire_swept`]): the canonical form
/// the profile node validated at this scalar, on its own sketch plane
/// (the section's placement), and the pass-1 pieces its walls and
/// seams are named by. Nothing is re-resolved; the profile node's
/// errors are the operand's own.
fn section_value<T: Decide>(
    results: &Results<T>,
    id: RecipeNodeId,
) -> Result<Arc<ProfileValue<T>>, NodeErrorKind> {
    operand(results, id, super::family::PROFILE, |v| match &v.payload {
        ValuePayload::Profile(vp) => Some(Arc::clone(vp)),
        _ => None,
    })
}

/// A structural (Count) slot read as a `usize`, refused typed when
/// absent or outside `least..=most`: the slot's own domain, which the
/// caller states (pass `usize::MAX` for a domain with no upper end).
/// The refusal carries that domain as the slot admits it, clamped to
/// what an `i64` holds, so `admits` always names values that evaluate.
fn need_count(
    vals: &SlotValues<impl Decide>,
    slot: SlotId,
    least: usize,
    most: usize,
) -> Result<usize, NodeErrorKind> {
    let n = slots::count(vals, slot).ok_or(NodeErrorKind::MissingSlot { slot })?;
    let wide = |u: usize| i64::try_from(u).unwrap_or(i64::MAX);
    usize::try_from(n)
        .ok()
        .filter(|u| (least..=most).contains(u))
        .ok_or(NodeErrorKind::CountOutOfRange {
            slot,
            count: n,
            admits: wide(least)..=wide(most),
        })
}

/// The Loft node: the §10.3 walls and their solid assembly, over its
/// sections' Profile values at `T` ([`section_value`]): a section that
/// is a function of the document's variables lofts into walls that are
/// functions of them, and the structure is selected on the sections'
/// value channel (`sweep::loft_profiles`). At `Interval` a section wider
/// than a point lofts the family it spans.
fn wire_loft<T: Decide + topo::AtRestPolicy>(
    id: RecipeNodeId,
    profiles: &[RecipeNodeId],
    stations: &[&[crate::node::StationMove]],
    results: &Results<T>,
    vals: &SlotValues<T>,
    tol: Tol,
) -> OpResult<T> {
    let values = profiles
        .iter()
        .map(|pid| section_value(results, *pid))
        .collect::<Result<Vec<_>, _>>()?;
    // The v degree admits `1 ..= sections − 1` (`Node::Loft`). Fewer
    // than two sections admit no degree at all, and what is wrong then
    // is the section count, which the skin's own door names.
    if profiles.len() < 2 {
        return Err(NodeErrorKind::Skin(sweep::SkinError::TooFewSections {
            have: profiles.len(),
            need: 2,
        }));
    }
    let v_degree = need_count(vals, SlotId::VDegree, 1, profiles.len() - 1)?;
    // The first section is the loft's own plane; each after it stands
    // where its station carries that plane.
    let mut sections: Vec<profile::ValidatedProfile<T>> = Vec::with_capacity(values.len());
    for (k, value) in values.iter().enumerate() {
        let plane = match k.checked_sub(1) {
            None => profile::SketchPlane::xy(),
            Some(after) => station_plane(k, stations[after], vals)?,
        };
        sections.push(value.validated.on(plane));
    }
    let pieces: Vec<super::ProfilePieces> = values.iter().map(|v| v.pieces.clone()).collect();
    // Skin refusals keep their own node-error shape (the §2
    // compatibility contract); assembly refusals arrive as `Loft`.
    let built = sweep::loft_profiles::<T>(&sections, v_degree, tol).map_err(|e| match e {
        sweep::LoftError::Skin(s) => NodeErrorKind::Skin(s),
        other => NodeErrorKind::Loft(other),
    })?;
    // Eager N4 emission, BEFORE the structural handoff is dropped. The
    // skin pairs canonical segment `k` of every section into wall `k`,
    // so wall `k` is named by one locator per section.
    let table = names::name_loft(id, &built, &pieces).map_err(NodeErrorKind::Naming)?;
    Ok(OpOut::plain(
        ValuePayload::Body(Arc::new(built.body)),
        table,
    ))
}

/// **Section `section`'s plane**: the first section's, carried by the
/// station's moves in order, each a slide along or a spin about one of
/// the first section's own axes through its origin.
fn station_plane<T: Decide>(
    section: usize,
    station: &[crate::node::StationMove],
    vals: &SlotValues<T>,
) -> Result<profile::SketchPlane<T>, NodeErrorKind> {
    use crate::node::StationMove as M;
    let axis = |a: Axis3| match a {
        Axis3::X => UnitVec3::exact_x(),
        Axis3::Y => UnitVec3::exact_y(),
        Axis3::Z => UnitVec3::exact_z(),
    };
    let mut frame = PoseValue::Frame(RigidMotion::axes_xy(Point3::origin()));
    for (step, m) in station.iter().enumerate() {
        frame = match m {
            M::Slide { along, by: _ } => {
                let by = need_scalar(vals, SlotId::StationSlide { section, step })?;
                pose::slide(frame, axis(*along), by)?
            }
            M::Spin { about, by: _ } => {
                let by = need_scalar(vals, SlotId::StationSpin { section, step })?;
                pose::spin(frame, Point3::origin(), axis(*about), by)?
            }
        };
    }
    let PoseValue::Frame(frame) = frame else {
        unreachable!("a frame slid and spun is a frame")
    };
    Ok(profile::SketchPlane::from_frame(frame))
}

/// The Sweep node: ONE honest arm.
///
/// Every RECIPE door still runs first — the structural slots, and both
/// operands read as profiles ([`section_value`]) — because a Sweep on a
/// datum, or with a bad Count, is a recipe error and must read as one.
/// The geometry is out of reach: see [`SWEEP_FRONTIER`].
fn wire_sweep<T: Decide>(
    profile: RecipeNodeId,
    path: RecipeNodeId,
    results: &Results<T>,
    vals: &SlotValues<T>,
) -> OpResult<T> {
    // Each door runs for its refusal alone; what it answers has no
    // reader, because the geometry that would read it does not exist.
    // `Node::Sweep`: stations ≥ 2, and 1 ≤ v degree ≤ stations − 1.
    let stations = need_count(vals, SlotId::Stations, 2, usize::MAX)?;
    need_count(vals, SlotId::VDegree, 1, stations - 1)?;
    section_value(results, profile)?;
    section_value(results, path)?;
    Err(NodeErrorKind::CurvedSolidFrontier {
        what: SWEEP_FRONTIER,
    })
}

#[cfg(test)]
mod loop_coordinates_tests {
    #![allow(clippy::unwrap_used, clippy::expect_used, clippy::panic)]

    use super::{NodeErrorKind, loop_coordinates};
    use crate::names::NamingError;

    /// Every loop a `u32` addresses gets its own coordinate; one loop
    /// more refuses the profile rather than wrapping onto loop 0.
    #[test]
    fn addresses_every_loop_and_refuses_one_past_the_bound() {
        assert_eq!(loop_coordinates(3).ok(), Some(0..3));
        let max = usize::try_from(u32::MAX).expect("a 32-bit or wider usize");
        assert_eq!(loop_coordinates(max).ok(), Some(0..u32::MAX));
        let Some(past) = max.checked_add(1) else {
            return; // a 32-bit usize holds no count past the bound
        };
        assert!(
            matches!(
                loop_coordinates(past),
                Err(NodeErrorKind::Naming(NamingError::Emission { .. }))
            ),
            "one loop past u32::MAX is refused"
        );
    }
}

#[cfg(test)]
mod stepped_operand_tests {
    #![allow(clippy::unwrap_used, clippy::expect_used, clippy::panic)]

    use super::{NodeErrorKind, PATTERN_DIRECTION_ROLE, SteppedOperands, stepped_rule_map, unit};
    use crate::Formula;
    use crate::expr::{VarEnv, eval};
    use geom_core::{Affine3, Band, Tol, Vec3};

    fn band() -> Band {
        Band::linear(Tol::witness()).unwrap()
    }

    fn scalar(v: f64) -> Formula {
        Formula::scalar(v).unwrap()
    }

    fn value(e: &Formula) -> f64 {
        eval::<f64>(e, &VarEnv::default()).unwrap()
    }

    /// **The negative spacing's recourse, followed.** The refusal
    /// spells the authored direction negated; parsed back and written
    /// in with the spacing made positive, it steps every copy to the
    /// translation the negative spacing names, bit for bit up to the
    /// sign of a zero component (a zero literal is spelled `0.0`, and
    /// `p + 0.0` is `p + -0.0` for every coordinate but `-0.0`).
    #[test]
    fn the_reversed_direction_steps_where_the_negative_spacing_did() {
        let sum = Formula::add(scalar(0.1), scalar(0.2)).unwrap();
        for authored in [
            [scalar(1.0), scalar(0.0), scalar(0.0)],
            [scalar(3.0), scalar(4.0), scalar(0.0)],
            [scalar(-0.3), sum, scalar(1.9)],
        ] {
            let direction = Vec3::new(
                value(&authored[0]),
                value(&authored[1]),
                value(&authored[2]),
            );
            let Err(NodeErrorKind::NegativeSpacing { reversed, .. }) =
                SteppedOperands::linear(direction, -4.25, &authored, band())
            else {
                panic!("a negative spacing refuses for {authored:?}");
            };
            let unit_dir = unit(direction, PATTERN_DIRECTION_ROLE, band()).unwrap();
            let mirrored = |i: i64| Affine3::translation(unit_dir.get() * (-4.25 * i as f64));
            let written = reversed
                .clone()
                .map(|e| e.expect("the negation is within the expression bound"));
            let back = Vec3::new(value(&written[0]), value(&written[1]), value(&written[2]));
            let followed =
                SteppedOperands::linear(back, 4.25, &written, band()).expect("the recourse builds");
            let bits = |c: [f64; 12]| c.map(|v| (v + 0.0).to_bits());
            for i in 1..6 {
                assert_eq!(
                    bits(mirrored(i).components()),
                    bits(
                        stepped_rule_map(&followed, i)
                            .unwrap()
                            .to_affine()
                            .components()
                    ),
                    "copy {i} of {reversed:?} lands where the negative spacing put it"
                );
            }
        }
    }
}

#[cfg(test)]
mod union_row_tests {
    #![allow(clippy::unwrap_used, clippy::expect_used, clippy::panic)]

    use super::{RowOrigin, in_member_space, judge_pairwise_contact, stated_at};
    use crate::coincide::{NamedCell, NamedCoincidence};
    use crate::names::{CapEnd, EntityKind, RoleSeg, Sense, StableName};
    use crate::{RecipeNodeId, VarId};

    fn name(kind: EntityKind, node: RecipeNodeId, path: Vec<RoleSeg>) -> StableName {
        StableName { kind, node, path }
    }

    /// Member `m`'s entity `of`, as the union `union` keys it.
    fn from(union: RecipeNodeId, m: u64, of: StableName) -> StableName {
        name(
            of.kind,
            union,
            vec![RoleSeg::From {
                read: VarId::new(1, m),
                of: of.into(),
            }],
        )
    }

    fn row(a: NamedCell, b: NamedCell) -> NamedCoincidence {
        let margin = geom_core::MarginDiag::value(0.0);
        NamedCoincidence {
            cells: [a, b],
            relation: topo::Relation::OnCarrier,
            site: topo::DecisionSite::VertexFusion,
            margin,
            discharge: topo::Discharge::of(margin),
        }
    }

    /// **Two crossings of one member edge are two statements** (DM4, "A
    /// row names member cells"): member 1's edge `e` crossed by member
    /// 2's two caps, each crossing on member 3's edge `c`, spell alike as
    /// `(e, c)`, and both publish. A judged row stating `(e, c)` states
    /// it for both, and a fold row read from no minted name collapses
    /// into the judged row as it always did.
    #[test]
    fn two_crossings_of_one_member_edge_publish_two_rows() {
        let union = RecipeNodeId::new(0, 9);
        let own = VarId::new(1, 9);
        let edge = |m| {
            name(
                EntityKind::Edge,
                RecipeNodeId::new(0, m),
                vec![RoleSeg::Cap(CapEnd::Start)],
            )
        };
        let e = from(union, 1, edge(1));
        let c = NamedCell::Entity {
            input: VarId::new(1, 3),
            name: edge(3),
        };
        let crossing = |end| {
            let cap = name(
                EntityKind::Face,
                RecipeNodeId::new(0, 2),
                vec![RoleSeg::Cap(end)],
            );
            NamedCell::Entity {
                input: own,
                name: name(
                    EntityKind::Vertex,
                    union,
                    vec![RoleSeg::Crossing {
                        edge: e.clone().into(),
                        face: from(union, 2, cap).into(),
                        sense: Sense::Leaves,
                    }],
                ),
            }
        };
        let own_c = |cell: &NamedCell| match cell {
            NamedCell::Entity { input, name } => NamedCell::Entity {
                input: if *input == VarId::new(1, 3) {
                    own
                } else {
                    *input
                },
                name: if *input == VarId::new(1, 3) {
                    from(union, 3, name.clone())
                } else {
                    name.clone()
                },
            },
            other => other.clone(),
        };
        let folded: Vec<(NamedCoincidence, Vec<StableName>)> = [CapEnd::Start, CapEnd::End]
            .into_iter()
            .map(|end| in_member_space(row(crossing(end), own_c(&c)), own).unwrap())
            .collect();
        let member_e = NamedCell::Entity {
            input: VarId::new(1, 1),
            name: edge(1),
        };
        for (r, minted) in &folded {
            assert_eq!(
                r.cells,
                [member_e.clone(), c.clone()],
                "a crossing reads as its edge"
            );
            assert_eq!(minted.len(), 1, "and keeps the name it was minted under");
        }
        let publish = |judged: &[NamedCoincidence]| {
            let mut rows = Vec::new();
            let mut origins: Vec<RowOrigin> = Vec::new();
            for j in judged {
                stated_at(
                    &mut rows,
                    &mut origins,
                    j.clone(),
                    RowOrigin::Judged { pair: (0, 1) },
                );
            }
            let at: Vec<usize> = folded
                .iter()
                .map(|(r, m)| {
                    stated_at(
                        &mut rows,
                        &mut origins,
                        r.clone(),
                        RowOrigin::Fold(m.clone()),
                    )
                })
                .collect();
            (rows.len(), at)
        };
        assert_eq!(publish(&[]), (2, vec![0, 1]), "two crossings, two rows");
        let judged = row(member_e.clone(), c.clone());
        assert_eq!(
            publish(std::slice::from_ref(&judged)),
            (1, vec![0, 0]),
            "a judged row states the fact for each crossing spelled as it"
        );
        let mut rows = vec![judged];
        let mut origins: Vec<RowOrigin> = vec![RowOrigin::Judged { pair: (0, 1) }];
        let plain = row(c.clone(), member_e.clone());
        assert_eq!(
            stated_at(&mut rows, &mut origins, plain, RowOrigin::Fold(Vec::new())),
            0,
            "a fold row read from member cells alone is the judged row"
        );
    }

    /// **One pair's two rows are two statements** (DM4: a coincidence
    /// row is a decision): an edge touching a curved face at two
    /// isolated points is two rows with one cell pair and one relation,
    /// and both publish. Two pairs stating one row exactly, a member
    /// read twice, publish it once; two pairs whose rows differ in their
    /// margin are two rows.
    #[test]
    fn one_pairs_two_rows_publish_and_a_repeated_read_collapses() {
        let cell = |m: u64, kind| NamedCell::Entity {
            input: VarId::new(1, m),
            name: name(
                kind,
                RecipeNodeId::new(0, m),
                vec![RoleSeg::Cap(CapEnd::Start)],
            ),
        };
        let at = |margin: f64| {
            let margin = geom_core::MarginDiag::value(margin);
            NamedCoincidence {
                cells: [cell(1, EntityKind::Edge), cell(2, EntityKind::Face)],
                relation: topo::Relation::SurfaceTangent,
                site: topo::DecisionSite::TangentWitness,
                margin,
                discharge: topo::Discharge::of(margin),
            }
        };
        let publish = |judged: &[(NamedCoincidence, (usize, usize))]| {
            let mut rows = Vec::new();
            let mut origins = Vec::new();
            judged
                .iter()
                .map(|(r, pair)| {
                    stated_at(
                        &mut rows,
                        &mut origins,
                        r.clone(),
                        RowOrigin::Judged { pair: *pair },
                    )
                })
                .collect::<Vec<_>>()
        };
        assert_eq!(
            publish(&[(at(0.0), (0, 1)), (at(0.0), (0, 1))]),
            [0, 1],
            "one pair's two touches, both published"
        );
        assert_eq!(
            publish(&[(at(1e-9), (0, 2)), (at(1e-9), (1, 2))]),
            [0, 0],
            "a member read twice states one row"
        );
        assert_eq!(
            publish(&[(at(0.0), (0, 2)), (at(1e-9), (1, 2))]),
            [0, 1],
            "two pairs at two margins are two rows"
        );
    }

    /// **A judged pair's operand A is its lesser read**, whatever the
    /// list order, and the listed-first on a read spelled twice; the
    /// pairs are visited in list order.
    #[test]
    fn a_judged_pairs_operand_a_is_its_lesser_read() {
        let at = |x: f64| {
            bvh::Aabb::from_points([geom_core::Point3::new(x, 0.0, 0.0)]).expect("a point's box")
        };
        let hull = at(0.0);
        let hulls = vec![hull; 4];
        let reads = [
            VarId::new(7, 7),
            VarId::new(3, 3),
            VarId::new(5, 5),
            VarId::new(3, 3),
        ];
        let mut seen = Vec::new();
        let out = judge_pairwise_contact::<f64>(&hulls, &reads, |a, b| {
            seen.push((a, b));
            Ok(None)
        })
        .unwrap();
        assert!(out.is_none(), "an empty judgement ends the pass");
        assert_eq!(seen, [(1, 0)], "the first pair, its lesser read as A");
        for (p, q) in [(0, 1), (0, 2), (1, 2), (1, 3), (2, 3), (0, 3)] {
            let mut seen = Vec::new();
            let mut sub = vec![hull; 4];
            for (i, h) in sub.iter_mut().enumerate() {
                if i != p && i != q {
                    *h = at(100.0 * (i as f64 + 1.0));
                }
            }
            let _ = judge_pairwise_contact::<f64>(&sub, &reads, |a, b| {
                seen.push((a, b));
                Ok(None)
            });
            let [(a, b)] = seen[..] else {
                panic!("the pair ({p}, {q}) judged once: {seen:?}");
            };
            assert!(
                reads[a] < reads[b] || (reads[a] == reads[b] && a < b),
                "the pair ({p}, {q}) runs with {a} as A"
            );
        }
    }
}
