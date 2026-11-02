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
//! **The union's naming pass.** A union or an intersect evaluates
//! through the kernel's n-ary door (`topo::nary_boolean`), which judges
//! the member pairs and folds the members; this layer names the
//! judgements and the steps it returns ([`fold_members`]).

use std::collections::BTreeMap;
use std::sync::Arc;

use geom_core::{
    Band, Decide, OrthoAxis, Point2, Point3, RigidMotion, Sign, Tol, UnitVec3, UnitVec3Error, Vec2,
    Vec3,
};
use sweep::{Revolution, RevolveAxis};
use topo::splitting::SplitPart;
use topo::transform::transform_rigid;
use topo::{Body, BooleanInputs, POSE_UNIT_NORM};

use super::anchor::{self, ProfilePre, ProfileValue};
use super::slots::{self, SlotValues};
use super::{BooleanValue, NodeErrorKind, NodeResult, PoseValue, SplitSide, ValuePayload};
use crate::decision::Decision;
use crate::names::{self, NameTable, SplitHalf};
use crate::node::{Axis3, Node, RecipeNodeId, SlotId};
use crate::program::ProfileProgram;
use topo::BooleanOp;

type Results<T> = BTreeMap<RecipeNodeId, NodeResult<T>>;

/// An op's product: the payload, its eagerly-emitted name table (N4;
/// emission lives in the wire layer, spec D4), and what its body
/// carries from its source ([`super::Carry`]).
///
/// The records a carry holds are keyed in the op's OUTPUT BODY 0 arena,
/// or each member's of a family in its own. A split never fills them:
/// "output body 0" would be a lie for its other half.
pub(crate) struct OpOut<T: Decide> {
    pub payload: ValuePayload<T>,
    /// Each output's names, by port (`NodeValue::outputs`): one for an
    /// op with one body output, a split's two halves apart; the
    /// fragment groups beside them empty for an op that forms none.
    pub outputs: Vec<super::OutputNames>,
    /// What its body carries (`NodeValue::carry`).
    pub carry: super::Carry<T>,
    /// The coincidences the op decided from values, named in its
    /// inputs' tables (`NodeValue::coincidences`).
    pub coincidences: Arc<[crate::coincide::NamedCoincidence]>,
    /// A placement's solved motion (`NodeValue::motion`).
    pub motion: Option<RigidMotion<T>>,
}

impl<T: Decide> OpOut<T> {
    /// An op that carries nothing from a source: a body of its own.
    fn plain(payload: ValuePayload<T>, names: Arc<NameTable>) -> Self {
        Self {
            payload,
            outputs: vec![super::OutputNames::new(names, Arc::default())],
            carry: super::Carry::default(),
            coincidences: Arc::new([]),
            motion: None,
        }
    }

    /// This output, a placement's, sitting at `motion` in its target.
    fn moved(self, motion: Option<RigidMotion<T>>) -> Self {
        Self { motion, ..self }
    }

    /// This output holding `contacts`, the records its op made, whose
    /// carried citations name `inputs` by position.
    fn holding(
        self,
        contacts: Arc<topo::ContactRecords>,
        inputs: Vec<crate::coincide::CitedInput>,
    ) -> Self {
        Self {
            carry: self.carry.recording(contacts, inputs),
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

    /// This output carrying `carry` from its source ([`super::Carry::copied`]).
    fn carrying(self, carry: super::Carry<T>) -> Self {
        Self { carry, ..self }
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

/// The LANE half of an evaluation's environment: the parameter
/// environments profile geometry is elaborated over at `T`. They
/// travel together so that no call site can elaborate the lift over a
/// different box than the node's slots were evaluated at.
#[derive(Debug)]
pub(crate) struct LaneEnv<'a, T> {
    /// The evaluation's parameter environment — nominals, or nominals
    /// widened by [`crate::analysis::ParamBox`] (E6's leaf replay).
    pub params: &'a crate::expr::VarEnv<T>,
    /// The document's own f64 parameter environment, under no box and
    /// no seed, built once per evaluation beside `params`. Every
    /// f64-pinned decision the evaluation makes reads it — a profile's
    /// program resolution — so it is what a content key owes
    /// ([`super::tag::slot`]). Nothing under evaluation builds a second
    /// one.
    pub nominal: &'a crate::expr::VarEnv<f64>,
    /// **The nominal document's results**, where this lane is not its
    /// evaluation and a profile reads a face's trace: an `f64`
    /// evaluation of the nominal, built beside this one
    /// ([`Self::nominal_results`]).
    pub nominal_results: Option<&'a Results<f64>>,
    /// Whether an `f64` lane over the nominal takes the first pass's
    /// form ([`super::PassTwo`]).
    pub pass_two: super::PassTwo,
}

impl<'a, T: super::ProfileLane> LaneEnv<'a, T> {
    /// **The nominal document's results**, which a profile reading a
    /// trace resolves its first pass off: `results` where this lane is
    /// the nominal `f64` evaluation, the evaluation built beside it
    /// otherwise.
    pub(crate) fn nominal_results<'r>(&self, results: &'r Results<T>) -> Option<&'r Results<f64>>
    where
        'a: 'r,
        T: Decide,
    {
        T::nominal_results(results, self.params, self.nominal).or(self.nominal_results)
    }

    /// **The first pass's validated form as this lane's value**, where
    /// this lane IS the first pass: an `f64` lane whose environment
    /// binds the nominal ([`super::ProfileLane`]). `None` wherever the
    /// second pass runs.
    pub(crate) fn pass_one<'p>(
        &self,
        pre: &'p ProfilePre,
    ) -> Option<&'p profile::ValidatedProfile<T>> {
        match self.pass_two {
            super::PassTwo::SkippedAtF64 => {
                T::pass_one(&pre.validated_f64, self.params, self.nominal)
            }
            #[cfg(any(test, feature = "test-support"))]
            super::PassTwo::Run => None,
        }
    }
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
    /// **A second computing frame**
    /// ([`super::EvalOptions::computing_twist`]): each construction's body
    /// built, then mapped by its node's variation of it
    /// ([`super::ComputingTwist::for_node`]), its frame the inverse, so
    /// every operation downstream computes in a frame the twist turned
    /// and its operands in frames turned apart. A twist that is no rigid
    /// motion refuses at each construction.
    pub twist: Option<super::ComputingTwist>,
    pub boolean_sweep: topo::SweepStrategy,
    pub parts: &'a super::parts::Reached<T>,
    /// The environments profile geometry is elaborated over.
    pub lane: LaneEnv<'a, T>,
}

/// **A node as its op reads it**: the node, with what
/// [`super::eval_node`] computed for it before its content key, by the
/// node's kind. Built there and not in [`run_op`], because a profile's
/// precompute decides on the node's behalf ahead of the key (its
/// decisions are the log's first entries) and an assertion's payload
/// feeds the key.
pub(crate) enum OpInput<'n, T: geom_core::Real> {
    /// A profile, with its program resolved at the nominal and the f64
    /// precompute made from it.
    Profile {
        program: &'n ProfileProgram,
        resolved: Vec<Vec<profile::Step<f64>>>,
        pre: Box<ProfilePre>,
        form: ProfileForm<T>,
        /// The frame the profile is drawn on, at the lane, where it is
        /// drawn on one.
        frame: Option<RigidMotion<T>>,
    },
    /// An assertion, with its value and bound evaluated
    /// ([`super::Payload`]) and the dimensions they are read at.
    Assertion {
        relation: crate::measure::AssertionRelation,
        dims: (crate::expr::Dimension, crate::expr::Dimension),
        payload: super::Payload<T>,
    },
    /// Any other node, which carries nothing computed ahead of its key.
    Other(&'n Node<ProfileProgram>),
}

/// **How a profile's lane value is formed** ([`super::ProfileLane`]).
pub(crate) enum ProfileForm<T: geom_core::Real> {
    /// The `f64` lane over the nominal IS the first pass: its validated
    /// form is the value.
    FirstPass(profile::ValidatedProfile<T>),
    /// Every other lane replays the program, resolved at the lane
    /// scalar, guided by the first pass's records.
    Lane(Vec<Vec<profile::Step<T>>>),
}

impl<T: geom_core::Real> OpInput<'_, T> {
    /// The assertion's payload, for the content key.
    pub(crate) fn payload(&self) -> Option<&super::Payload<T>> {
        match self {
            Self::Assertion { payload, .. } => Some(payload),
            Self::Profile { .. } | Self::Other(_) => None,
        }
    }

    /// The profile's program resolved at the nominal, for the content
    /// key.
    pub(crate) fn resolved_program(&self) -> Option<&[Vec<profile::Step<f64>>]> {
        match self {
            Self::Profile { resolved, .. } => Some(resolved),
            Self::Assertion { .. } | Self::Other(_) => None,
        }
    }

    /// The profile's program resolved at the lane scalar, where the
    /// lane replays it, for the content key.
    pub(crate) fn lane_program(&self) -> Option<&[Vec<profile::Step<T>>]> {
        match self {
            Self::Profile {
                form: ProfileForm::Lane(resolved),
                ..
            } => Some(resolved),
            Self::Profile {
                form: ProfileForm::FirstPass(_),
                ..
            }
            | Self::Assertion { .. }
            | Self::Other(_) => None,
        }
    }
}

/// Runs one node's op against its (already Ok) inputs and evaluated
/// slots, emitting the node's name table alongside the payload.
pub(crate) fn run_op<T>(
    id: RecipeNodeId,
    input: &OpInput<'_, T>,
    doc: &crate::doc::Doc<ProfileProgram>,
    results: &Results<T>,
    vals: &SlotValues<T>,
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
        + crate::mate::SolveScalar
        + super::ProfileLane,
{
    use crate::OperandSlot as O;
    // An assertion reads no operand, and a profile's frame and traced
    // faces were read ahead of its key (`trace::drawn`), so neither has
    // a read to project here.
    let node = match input {
        OpInput::Profile {
            program,
            pre,
            form,
            frame,
            ..
        } => {
            let value = wire_profile(program, pre, form, frame.as_ref(), tol)?;
            let rows = crate::coincide::name_junctions(id, &value.validated, &value.pieces);
            return Ok(
                OpOut::plain(ValuePayload::Profile(value), names::empty()).with_coincidences(rows)
            );
        }
        OpInput::Assertion {
            relation,
            dims,
            payload,
        } => {
            return wire_assertion(*dims, *relation, payload, tol);
        }
        OpInput::Other(node) => *node,
    };
    let unprojected = results;
    // A boolean's, a union's and a measure's reads are projected one by
    // one: two ports of one split are two bodies, which one map keyed by
    // the split cannot hold. Every other node reads at most one body.
    let per_operand = matches!(
        node,
        Node::Subtract { .. }
            | Node::Union { .. }
            | Node::Intersect { .. }
            | Node::Assemble { .. }
            | Node::Measure { .. }
            | Node::Place { .. }
    );
    let projected = if per_operand {
        None
    } else {
        reads_projected(node, &body_rows(node, doc), doc, (results, vals))?
    };
    let results = projected.as_ref().unwrap_or(results);
    let project_each = |rows: &[(O, crate::VarId)]| {
        rows.iter()
            .map(|&row| reads_projected(node, &[row], doc, (unprojected, vals)))
            .collect::<Result<Vec<_>, _>>()
    };
    // An operand reads an output; the op reads the operation's value.
    // Every read resolves here: an unresolved one refused the node
    // before its op was reached (`eval::read_at`).
    let at = |slot, var| super::read_at(doc, slot, var);
    match node {
        Node::Profile(_) | Node::Assertion { .. } => {
            unreachable!("`eval_node` gives a profile and an assertion their own `OpInput` arm")
        }
        Node::Extrude { profile, .. } => {
            wire_extrude(id, at(O::Profile, *profile)?, results, vals, tol)
                .and_then(|out| twisted(out, id, env, tol))
        }
        Node::Revolve { profile, .. } => {
            wire_revolve(id, at(O::Profile, *profile)?, results, vals, tol)
                .and_then(|out| twisted(out, id, env, tol))
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
                .and_then(|out| twisted(out, id, env, tol))
        }
        Node::Sweep { profile, path, .. } => {
            let (profile, path) = (at(O::Profile, *profile)?, at(O::Path, *path)?);
            wire_sweep(profile, path, results, vals).and_then(|out| twisted(out, id, env, tol))
        }
        Node::Tube { window, .. } => {
            wire_tube(id, window, vals, tol).and_then(|out| twisted(out, id, env, tol))
        }
        Node::HollowTube { window, .. } => {
            wire_hollow_tube(id, window, vals, tol).and_then(|out| twisted(out, id, env, tol))
        }
        Node::Fillet { selection, .. } => wire_blend(
            &crate::verbs::blend::fillet(),
            id,
            &select(doc, results, O::Selection, *selection)?,
            (doc, results),
            vals,
            tol,
        ),
        Node::Chamfer { selection, .. } => wire_blend(
            &crate::verbs::blend::chamfer(),
            id,
            &select(doc, results, O::Selection, *selection)?,
            (doc, results),
            vals,
            tol,
        ),
        Node::Shell { open, .. } => wire_shell(
            &crate::verbs::shell::shell(),
            id,
            &select(doc, results, O::Open, *open)?,
            (doc, results),
            vals,
            tol,
        ),
        Node::Split { target, tool } => wire_split(
            &crate::verbs::split::split(),
            id,
            (target.read, at(O::Target, target.read)?),
            (
                pose::eval_posed(doc, unprojected, O::Tool, *tool, env.lane.params, tol)?,
                *tool,
            ),
            (doc, results),
            tol,
        ),
        Node::Mirror { body, plane } => wire_mirror(
            body.read,
            at(O::Body, body.read)?,
            pose::eval_posed(doc, unprojected, O::Plane, *plane, env.lane.params, tol)?,
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
                doc,
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
            source,
            target: _,
            constraints,
        } => {
            let each = project_each(&[(O::Source, source.read)])?;
            let values = each[0].as_ref().unwrap_or(unprojected);
            let of = at(O::Source, source.read)?;
            let reach = copy_reach(values, of);
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
            wire_place(id, (of, source.read, values), placed.as_ref()).map(|out| out.moved(placed))
        }
        Node::Assemble { members } => {
            let rows: Vec<(O, crate::VarId)> = members
                .rows()
                .into_iter()
                .map(|(seat, read)| (seat, read.read))
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
            wire_assemble(id, &reads, tol)
        }
        // A space DENOTES NO BODY.
        Node::World | Node::Space => {
            unreachable!("a space is scheduled nowhere (`eval::schedule`)")
        }
        Node::Measure { primitive } => {
            let each = project_each(&body_rows(node, doc))?;
            let at = |reference: usize| {
                each.get(reference)
                    .and_then(Option::as_ref)
                    .unwrap_or(unprojected)
            };
            wire_measure(primitive, doc, &at, env.lane.params, tol)
        }
        Node::InstantiatePart {
            doc_ref,
            interface,
            target: _,
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
            let out = wire_instantiate_part(id, doc_ref, interface, placed, env, tol)?;
            Ok(out.moved(placed))
        }
    }
}

/// **A construction's value in the second computing frame**, where the
/// evaluation asks for one ([`OpEnv::twist`]): its body mapped by the
/// twist, its frame the twist's inverse, so it stands where it did in
/// its space. Itself everywhere else.
fn twisted<T: Decide + topo::AtRestPolicy>(
    out: OpOut<T>,
    id: RecipeNodeId,
    env: &OpEnv<'_, T>,
    tol: Tol,
) -> OpResult<T> {
    if let (Some(twist), ValuePayload::Body(body)) = (env.twist.as_ref(), &out.payload) {
        let twist = twist
            .for_node(id)
            .at::<T>(tol)
            .map_err(NodeErrorKind::Motion)?;
        let turned = transform_rigid(body.body(), &twist, tol).map_err(NodeErrorKind::Transform)?;
        let back = super::Frame::Own
            .moved(&twist.inverse().map_err(NodeErrorKind::Motion)?)
            .map_err(NodeErrorKind::Motion)?;
        return Ok(OpOut {
            payload: ValuePayload::Body(super::Framed::in_frame(Arc::new(turned), back)),
            ..out
        });
    }
    Ok(out)
}

/// **A copy's reach, measured from its space's origin** — a shape's
/// body's, as [`crate::mate::part_reach`] bounds a part's, each body
/// read in its own coordinates about where its frame puts that origin
/// ([`body_reach`]) — for the fold's arm.
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
    let farthest = |bodies: &mut dyn Iterator<Item = &super::Framed<T>>| {
        bodies
            .map(|body| body_reach(body, at))
            .try_fold(0.0_f64, |held, r| r.map(|r| held.max(r)))
    };
    match payload {
        ValuePayload::Body(body) | ValuePayload::Boolean(BooleanValue::Body { body, .. }) => {
            body_reach(body, at)
        }
        ValuePayload::Instances(bodies) => farthest(&mut bodies.iter()),
        ValuePayload::Split { above, below } => farthest(
            &mut [above, below].into_iter().filter_map(|side| match side {
                SplitSide::Body(b) => Some(b),
                SplitSide::Empty => None,
            }),
        ),
        ValuePayload::Boolean(BooleanValue::Empty)
        | ValuePayload::Profile(_)
        | ValuePayload::Measure { .. }
        | ValuePayload::MeasureUnavailable { .. }
        | ValuePayload::Assertion(_)
        | ValuePayload::Family => Ok(0.0),
    }
}

/// [`copy_reach`] of one body, read off the node `at`: from its space's
/// origin.
fn body_reach<T: crate::mate::SolveScalar>(
    body: &super::Framed<T>,
    at: RecipeNodeId,
) -> Result<f64, crate::mate::LeverRefusal> {
    body.space_origin()
        .map_err(|_| crate::mate::ReachRefusal::NoFiniteBound)
        .and_then(|about| crate::mate::part_reach_from(body.body(), about))
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
/// their own coordinates, and its `to` off a body in the target space,
/// in that space's coordinates; the bundle folded
/// ([`crate::mate::solve::solve_bundle`]), its values charted on the
/// target space's axes. Every mate's `to` is in the target space, which
/// the edit door checked and evaluation refuses a node a re-target left
/// otherwise before it runs ([`NodeErrorKind::SpaceMismatch`]). Answers
/// the copy's motion in its target space's coordinates (`None` for the
/// identity bundle, the one placement placed by no map).
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
) -> Result<Option<RigidMotion<T>>, NodeErrorKind>
where
    T: crate::mate::SolveScalar,
{
    use crate::mate::{MateFault, MateRef, MateSide};
    let fault = |f: MateFault| NodeErrorKind::Mate(Box::new(f));
    let b = band(tol)?;
    let params = env.lane.params;
    let mut mates = Vec::with_capacity(constraints.len());
    let mut values = Vec::new();
    // Each mate's two poses and its orientation: what the chart of the
    // one mate fixing a direction reads.
    let mut charted = Vec::new();
    for c in constraints {
        let mate = MateRef::new(id, c.id);
        let m = match &c.kind {
            crate::mate::ConstraintKind::Mate(m) => m,
            crate::mate::ConstraintKind::Value(v) => {
                let slot = match v.coord {
                    crate::mate::Coord::Slide(_) => SlotId::ValueSlide { constraint: c.id },
                    crate::mate::Coord::Spin(_) => SlotId::ValueSpin { constraint: c.id },
                };
                let by = crate::expr::eval_var(v.by, v.coord.dimension(), params)
                    .map_err(|source| NodeErrorKind::Expr { slot, source })?;
                values.push(crate::mate::value::ReadValue {
                    value: mate,
                    coord: v.coord,
                    by,
                    turns: params.turns.get(&v.by).copied(),
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
        let to = pose::eval_pose(
            doc,
            to_results,
            seat(MateSide::To),
            crate::pose::RoleRead::plain(m.to),
            params,
            tol,
        )
        .map_err(|kind| unevaluated(MateSide::To, kind))?;
        let to = crate::mate::solve::tightened(to);
        charted.push((
            mate,
            on.clone(),
            to.clone(),
            crate::mate::flipped(doc, m.to),
            m.orient,
        ));
        mates.push(crate::mate::solve::ReadMate { mate, on, to });
    }
    let equations = crate::mate::solve::equations_of(doc, constraints);
    if mates.is_empty() && crate::mate::solve::identity_bundle(doc, constraints) {
        // The identity bundle places by no map, once the rule has
        // admitted it.
        crate::mate::solve::solve_bundle(id, &equations, &mates, &values, None)?;
        return Ok(None);
    }
    // The chart of the one mate fixing a direction, read only where a
    // value is charted on it.
    let mut chart = None;
    if !mates.is_empty() {
        let reach = reach.map_err(|refusal| {
            fault(MateFault::Unleverable {
                mate: mates[0].mate,
                refusal: Box::new(refusal),
            })
        })?;
        let arm = crate::mate::solve::bundle_arm(reach, &mates).map_err(|f| fault(*f))?;
        for (mate, on, to, to_flipped, orient) in charted.iter().filter(|_| !values.is_empty()) {
            if let Some(found) =
                crate::mate::value::Chart::of(*mate, on, to, *to_flipped, *orient, arm, b)?
            {
                chart = Some(found);
                break;
            }
        }
    }
    match crate::mate::solve::solve_bundle(id, &equations, &mates, &values, chart)? {
        crate::mate::solve::Folded::Chained(motion) => Ok(Some(motion)),
        // Every placement but the identity bundle runs its map, at every
        // value and in every lane: a map the solve composed as motions
        // is placed as the motion it is, any other admitted from its
        // entries, by one door.
        crate::mate::solve::Folded::Pinned(map, composed) => Ok(Some(match composed {
            Some(composed) => composed.motion(b)?,
            None => crate::mate::solve::admitted(&map, b)?,
        })),
    }
}

/// ASM-2A D-3: materialize an instance through the shipped doors.
///
/// Read the part the evaluation entered, take the referenced document's A10
/// PRODUCT, and hand back a body-denoting value: the product's body,
/// shared, under its frame moved by the instance's map. The copy is an
/// ordinary `Body` value — one solid or N, under ONE rigid map — so the
/// product gather and the export door consume it with no new arms.
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
        + crate::mate::SolveScalar
        + super::ProfileLane,
{
    let part = env
        .parts
        .get(doc_ref, tol)
        .map_err(|fault| NodeErrorKind::Part {
            doc_ref: *doc_ref,
            fault,
        })?;
    // ASM-R2b D-4/D-5: the structural half of A4's fit gate (the
    // geometric half is `crate::checks::at_rest::assemble`). Every crossing
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
    // ASM-R2b D-1: the part's product is the source, and its copy
    // carries what any copy carries ([`copy_of`]): its records with
    // their cells UNCHANGED, because the copy is the part's body itself
    // (re-deriving them from the placed geometry is the scan-to-bless
    // move F1 bans), each citing the part's record it is through this
    // instance ([`crate::coincide::CitedInput::Part`]); and the part's
    // findings, each routed through this instance
    // ([`crate::checks::at_rest::PartRow::through`]). The part's census
    // decided the meetings inside its product: no door here holds them.
    copy_of(
        id,
        (&part.body, &part.names),
        &part.carry_through(id),
        map.as_ref(),
        &crate::coincide::CitedInput::Part(*doc_ref),
    )
}

/// **A placement's copy of one body** (D10 `[placement.placement]`):
/// `source` shared under `map`, named as the source names it
/// ([`names::name_copy`]), carrying `carry` as a copy carries it
/// ([`super::Carry::copied`]), its source `cites`. A copy is never
/// rebuilt.
fn copy_of<T: Decide>(
    id: RecipeNodeId,
    (source, table): (&super::Framed<T>, &NameTable),
    carry: &super::Carry<T>,
    map: Option<&RigidMotion<T>>,
    cites: &crate::coincide::CitedInput,
) -> OpResult<T> {
    let copy = place(source, map)?;
    let table = names::name_copy(id, table, &[copy.body()]).map_err(NodeErrorKind::Naming)?;
    Ok(OpOut::plain(ValuePayload::Body(copy), table)
        .carrying(carry.copied(super::carry::Stands::placed(map), cites)?))
}

/// **A rigid placement of `body`**: its body shared, its frame moved by
/// `map`. `None` is the identity bundle's placement by no map: the
/// value itself.
///
/// # Errors
///
/// [`NodeErrorKind::Motion`] where the composed frame overflows.
fn place<T: Decide>(
    body: &super::Framed<T>,
    map: Option<&RigidMotion<T>>,
) -> Result<super::Framed<T>, NodeErrorKind> {
    match map {
        None => Ok(body.clone()),
        Some(map) => body.moved(map).map_err(NodeErrorKind::Motion),
    }
}

/// Every body of `bodies` placed by `map`, in order: one placement of a
/// family.
fn place_each<T: Decide>(
    bodies: &[super::Framed<T>],
    map: Option<&RigidMotion<T>>,
) -> Result<Vec<super::Framed<T>>, NodeErrorKind> {
    bodies.iter().map(|body| place(body, map)).collect()
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

/// **A body operand, finished** for a door that takes finished bodies
/// (the Boolean's, the split's, the shell's and the blends'): [`read_body`]'s
/// body in its own frame, beside the value it was read off, through the
/// at-rest gate
/// ([`topo::AtRestPolicy::gate_at_rest_kept`]), once per operand of the
/// node. The evaluator holds the bodies its nodes built with no verdict
/// kept beside them, so the consuming node pays the gate here.
///
/// # Errors
///
/// [`read_body`]'s; [`NodeErrorKind::UnfinishedOperand`] where the
/// gate refuses the body.
fn finished_operand<T: Decide + geom_core::Bounds + topo::AtRestPolicy>(
    results: &Results<T>,
    input: RecipeNodeId,
    tol: Tol,
) -> Result<(topo::AtRestBody<T>, super::Framed<T>), NodeErrorKind> {
    let body = read_body(results, input)?;
    let finished = T::gate_at_rest_kept(body.body().clone(), tol)
        .map_err(|errors| NodeErrorKind::UnfinishedOperand { input, errors })?;
    Ok((finished, body))
}

/// **A body read for its geometry**: a Body value, or a boolean's
/// non-empty result — [`placeable_operand`]'s `Body` arm, refusing the
/// other in its own one-body word. A reader consumes no material (a
/// pose read off a face reads a face).
fn read_body<T: Decide>(
    results: &Results<T>,
    input: RecipeNodeId,
) -> Result<super::Framed<T>, NodeErrorKind> {
    let v = value_of(results, input)?;
    match placeable_operand(v, input) {
        Ok(Placeable::Body(b)) => Ok(b),
        Ok(Placeable::Instances(_)) | Err(NodeErrorKind::WrongOperand { .. }) => {
            Err(wrong_operand(v, input, super::family::BODY))
        }
        Err(other) => Err(other),
    }
}

/// **The body a resolved name lives in**: [`read_body`] for a value of
/// one body, else the output body the name's row indexes — member `i`
/// of a family — since a selection names an entity of one body of the
/// value it reads; `whole` where the name is of one of a family's
/// members ([`super::NodeValue::whole`]).
pub(super) fn read_named_body<T: Decide>(
    results: &Results<T>,
    input: RecipeNodeId,
    body: u32,
    whole: bool,
) -> Result<super::Framed<T>, NodeErrorKind> {
    let v = value_of(results, input)?;
    let v = if whole { v.whole() } else { v };
    match placeable_operand(v, input) {
        Ok(Placeable::Instances(bodies)) => usize::try_from(body)
            .ok()
            .and_then(|k| bodies.get(k))
            .cloned()
            .ok_or_else(|| wrong_operand(v, input, super::family::BODY)),
        Ok(Placeable::Body(b)) => Ok(b),
        Err(NodeErrorKind::WrongOperand { .. }) => {
            Err(wrong_operand(v, input, super::family::BODY))
        }
        Err(other) => Err(other),
    }
}

/// **What a placer places**: the two value shapes a rigid map is
/// defined over. A pattern is shape-preserving — `Body → Body`,
/// `Instances → Instances`.
enum Placeable<T: Decide> {
    /// One body: a `Body` value or a boolean's non-empty result.
    Body(super::Framed<T>),
    /// Several placed bodies, in the input's own instance order; the
    /// input's name table indexes them by that order.
    Instances(Vec<super::Framed<T>>),
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
        ValuePayload::Body(b) | ValuePayload::Boolean(BooleanValue::Body { body: b, .. }) => {
            Ok(Placeable::Body(b.clone()))
        }
        ValuePayload::Boolean(BooleanValue::Empty) => Err(NodeErrorKind::EmptyOperand { input }),
        ValuePayload::Instances(bodies) => Ok(Placeable::Instances(bodies.clone())),
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
/// the layer that OWNS a value names its length decision. A pose's
/// directions are decided under [`POSE_UNIT_NORM`] inside
/// [`UnitVec3::new`], because `PoseValue` has no unnormalized
/// spelling. The mate solve derives its offsets through this function,
/// so a circular pattern's axis is decided under this name on
/// the solve road and under [`POSE_UNIT_NORM`] on the evaluation road:
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
/// direction ([`pose_direction_escalated`]).
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

/// A pose's direction length that could not be called,
/// decided under [`POSE_UNIT_NORM`]: the arm D10's pose ground keeps,
/// [`NodeErrorKind::PoseDirectionEscalated`].
fn pose_direction_escalated(source: geom_core::Indeterminate) -> NodeErrorKind {
    NodeErrorKind::PoseDirectionEscalated {
        predicate: POSE_UNIT_NORM,
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

/// [`need_scalar`] of an angle slot, with the form it was written in.
pub(crate) fn need_angle<T: Decide>(
    vals: &SlotValues<T>,
    slot: SlotId,
) -> Result<geom_core::Angle<T>, NodeErrorKind> {
    slots::angle(vals, slot).ok_or(NodeErrorKind::MissingSlot { slot })
}

/// **A slot the node declares, as its evaluated value.**
///
/// # Panics
///
/// When `found` is `None`, which only a kernel bug reaches:
/// `slots::eval_slots` evaluates every row `Node::slots` declares for
/// the node, a reader asks only for the slots its node's kind declares,
/// and the slot's address fixes whether it is a scalar or a count, so
/// the value is always there.
#[track_caller]
fn declared<V>(found: Option<V>, slot: SlotId) -> V {
    let Some(value) = found else {
        unreachable!(
            "the node's {slot:?} slot is not among its evaluated values, though \
             `slots::eval_slots` evaluates every slot `Node::slots` declares and its reader \
             asks only for one its node declares"
        )
    };
    value
}

/// A declared scalar slot ([`declared`]).
#[track_caller]
fn slot_scalar<T: Decide>(vals: &SlotValues<T>, slot: SlotId) -> T {
    declared(slots::scalar(vals, slot), slot)
}

/// A declared angle slot, with the form it was written in
/// ([`declared`]).
#[track_caller]
fn slot_angle<T: Decide>(vals: &SlotValues<T>, slot: SlotId) -> geom_core::Angle<T> {
    declared(slots::angle(vals, slot), slot)
}

/// A declared count slot ([`declared`]).
#[track_caller]
fn slot_count<T: Decide>(vals: &SlotValues<T>, slot: SlotId) -> i64 {
    declared(slots::count(vals, slot), slot)
}

/// A declared `[Expr; 2]` slot pair as a sketch-plane point, each
/// component read through [`declared`] under its own slot.
#[track_caller]
fn slot_point2<T: Decide>(vals: &SlotValues<T>, f: fn(Axis3) -> SlotId) -> Point2<T> {
    Point2::new(
        slot_scalar(vals, f(Axis3::X)),
        slot_scalar(vals, f(Axis3::Y)),
    )
}

/// **A revolve's axis direction in its profile's plane**: the unit
/// vector at its heading from the profile's x axis, `(cos h, sin h)`.
/// Unit by construction, up to the rounding of the sine and cosine,
/// which a heading written as a fraction of a turn reads exactly
/// ([`geom_core::Angle`]); its readers decide it as they decide any
/// authored direction.
fn axis_heading<T: Decide>(vals: &SlotValues<T>) -> Vec2<T> {
    let (sin, cos) = slot_angle(vals, SlotId::AxisHeading).sin_cos();
    Vec2::new(cos, sin)
}

/// **A direction refusal before it is spelled as a node error** — what
/// the kernel's door said and which vector said it.
///
/// Separate from its spelling because a pose's direction is decided
/// where the pose is made and spelled by the reader that needed it.
/// Every road spells it through [`DirectionRefusal::node_error`].
#[derive(Debug, Clone, Copy)]
pub struct DirectionRefusal {
    /// Which vector refused, in the words a user reads — "frame x axis",
    /// "frame y axis".
    pub role: &'static str,
    /// What the kernel's direction door said.
    pub error: UnitVec3Error,
}

impl DirectionRefusal {
    /// The node error this refusal spells, under [`POSE_UNIT_NORM`],
    /// because on this road the kernel type owns the value. **The one
    /// spelling** from a refusal to a [`NodeErrorKind`]. `pub` because
    /// the refusal is: a consumer holding one asks for the raise here
    /// rather than re-spelling it.
    pub fn node_error(self) -> NodeErrorKind {
        refusal(self.error, self.role, pose_direction_escalated)
    }
}

/// A vector as a pose's direction, through the kernel type's own
/// constructor: the decision and its three refusals live there, and
/// this layer names the ROLE.
fn pose_unit<T: Decide>(
    v: Vec3<T>,
    role: &'static str,
    band: Band,
) -> Result<UnitVec3<T>, DirectionRefusal> {
    UnitVec3::new(v, POSE_UNIT_NORM, band).map_err(|error| DirectionRefusal { role, error })
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
    RigidMotion::gram_schmidt(origin, u_raw, v_raw, POSE_UNIT_NORM, band).map_err(|e| {
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
    u32::try_from(n).map(|n| 0..n).map_err(|_| {
        NodeErrorKind::Naming(names::NamingError::Emission {
            what: "a profile holds more loops than a u32 loop index addresses",
        })
    })
}

/// The profile node's F64 PRECOMPUTE (LIB-SWITCH §4b): the resolved
/// program replays through `profile::replay` — the ONLY path from
/// steps to geometry — then the assembled `Profile<f64>` validates at
/// f64 (the C6 structure-selection gate, which also yields the
/// canonical form the naming anchor is derived from). Runs inside the
/// node's verdict frame (`eval_node`) ahead of the op, so its structure
/// decisions are logged as the node's own. At `f64` the validated
/// form IS the op's value, on the lane's plane (`wire_profile`,
/// [`super::ProfileLane`]), so the node decides each structure question
/// once, and logs in the second pass's place what that pass decides
/// ([`ProfilePre::decided`]). VQ6: replay and
/// validation run under the same tolerance the evaluation pins.
pub(crate) fn prepare_profile(
    resolved: &[Vec<profile::Step<f64>>],
    ids: &[Vec<crate::node::StepId>],
    tol: Tol,
) -> Result<ProfilePre, NodeErrorKind> {
    let plane = profile::SketchPlane::xy();
    let (elaborated, recording) = geom_core::k_stats::detached(|| {
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
        Ok((replay_records, validated_f64, canonical))
    });
    let pinned = |name: &str| profile::PINNED_UNDER_GUIDANCE.contains(&name);
    let decided = geom_core::k_stats::Recorded {
        verdicts: (recording.recorded().verdicts.iter())
            .filter(|v| !pinned(v.predicate))
            .copied()
            .collect(),
        escalations: (recording.recorded().escalations.iter())
            .filter(|e| !pinned(e.predicate()))
            .copied()
            .collect(),
    };
    geom_core::k_stats::splice(recording);
    let (replay_records, validated_f64, canonical) = elaborated?;
    let naming = anchor::validated_naming(&canonical);
    let pieces = super::ProfilePieces::publish(&naming, &replay_records, ids);
    Ok(ProfilePre {
        validated_f64,
        naming,
        pieces,
        structure: profile::ProfileStructure {
            replay: replay_records,
            canonical,
        },
        decided,
    })
}

/// The lift's SECOND PASS (M10-P PP1/PP5): the same program, `resolved`
/// at the lane scalar (`eval_node`), elaborated there, GUIDED by pass
/// 1's record.
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
pub(crate) fn lane_profile<T: Decide + geom_core::Bounds>(
    resolved: &[Vec<profile::Step<T>>],
    plane: profile::SketchPlane<T>,
    pre: &ProfilePre,
    tol: Tol,
) -> Result<profile::ValidatedProfile<T>, NodeErrorKind> {
    let mut loops = Vec::with_capacity(resolved.len());
    for (li, steps) in loop_coordinates(resolved.len())?.zip(resolved) {
        let Some(record) = pre.structure.replay.get(li as usize) else {
            unreachable!(
                "the f64 pass recorded {} loops and the lane resolves loop {li}: both resolve \
                 the one program, one entry per loop",
                pre.structure.replay.len()
            )
        };
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

fn wire_profile<T: Decide + geom_core::Bounds + super::ProfileLane>(
    program: &ProfileProgram,
    pre: &ProfilePre,
    form: &ProfileForm<T>,
    frame: Option<&RigidMotion<T>>,
    tol: Tol,
) -> Result<Arc<ProfileValue<T>>, NodeErrorKind> {
    let plane = profile::SketchPlane::xy();
    let validated = match form {
        // The first pass's validated form is the value, on the
        // profile's own `xy`, and the node logs what the second pass it
        // skips decides.
        ProfileForm::FirstPass(first) => {
            geom_core::k_stats::relog(&pre.decided);
            first.clone().on_plane(plane)
        }
        ProfileForm::Lane(resolved) => lane_profile::<T>(resolved, plane, pre, tol)?,
    };
    Ok(Arc::new(ProfileValue {
        validated,
        naming: pre.naming.clone(),
        pieces: pre.pieces.clone(),
        edge_radii: edge_radii(program, pre),
        frame: frame.map(|f| super::Frame::Map(Arc::new(*f))),
    }))
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
/// bug observed in a branch — `unreachable!`'s job (D9 row 4).
fn edge_radii(program: &ProfileProgram, pre: &ProfilePre) -> Vec<Vec<Option<crate::var::VarId>>> {
    pre.naming
        .loops
        .iter()
        .enumerate()
        .map(|(canonical_loop, anchor)| {
            let by_program_segment = program
                .segment_radii(&pre.structure, anchor.program_loop)
                .unwrap_or_else(|e| {
                    unreachable!(
                        "this profile's structure record was minted from this one \
                         program by one pre-pass, so the record \
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
        ValuePayload::Body(vp.framed(Arc::new(out.body))),
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
    let distance = slot_scalar(vals, SlotId::Distance);
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
        origin: slot_point2(vals, SlotId::Origin),
        dir: axis_heading(vals),
    };
    let b = band(tol)?;
    // Kernel contract: exactly-full must SAY Full. Anything else wires
    // Partial and the kernel's own classification rules on it.
    let angle = slot_scalar(vals, SlotId::RevolveAngle);
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

fn tube_args<T: Decide>(window: &crate::node::TubeWindow, vals: &SlotValues<T>) -> TubeArgs<T> {
    TubeArgs {
        frame: RigidMotion::identity(),
        major_radius: slot_scalar(vals, SlotId::TubeMajorRadius),
        window: match window {
            crate::node::TubeWindow::Full => sweep::TubeWindow::Full,
            crate::node::TubeWindow::Arc { .. } => sweep::TubeWindow::Arc {
                t0: slot_scalar(vals, SlotId::TubeWindowStart),
                t1: slot_scalar(vals, SlotId::TubeWindowEnd),
            },
        },
        minor_radius: slot_scalar(vals, SlotId::TubeMinorRadius),
    }
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
    let a = tube_args(window, vals);
    let built = sweep::tube_along_arc(a.frame, a.major_radius, a.window, a.minor_radius, tol)
        .map_err(|e| NodeErrorKind::Tube(Box::new(e)))?;
    let table =
        names::name_revolve(id, &built, &tube_pieces(&built)).map_err(NodeErrorKind::Naming)?;
    Ok(OpOut::plain(
        ValuePayload::Body(super::Framed::own(Arc::new(built.body))),
        table,
    ))
}

/// **A tube's section, named structurally**: the door builds the
/// outer circle (and a hollow tube's bore) itself, so each piece is
/// named by its place in that construction under the tube node
/// (`names/README.md`, "N1, the profile pieces") — the shape the node
/// kind fixes, which nothing can renumber.
fn tube_pieces<T: Decide>(built: &sweep::Revolved<T>) -> super::ProfilePieces {
    let counts: Vec<usize> = built.rims.iter().map(Vec::len).collect();
    super::ProfilePieces::section(&counts)
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
    let a = tube_args(window, vals);
    let wall = slot_scalar(vals, SlotId::TubeWall);
    let built =
        sweep::tube_along_arc_hollow(a.frame, a.major_radius, a.window, a.minor_radius, wall, tol)
            .map_err(|e| NodeErrorKind::Tube(Box::new(e)))?;
    let table =
        names::name_revolve(id, &built, &tube_pieces(&built)).map_err(NodeErrorKind::Naming)?;
    Ok(OpOut::plain(
        ValuePayload::Body(super::Framed::own(Arc::new(built.body))),
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
/// supplies what differs: the size slot, which verb to build, and what
/// to call a missing record.
///
/// # Refusals
///
/// The selection is resolved before this runs ([`select`]). Failure of
/// the op itself is [`NodeErrorKind::Blend`], carrying
/// the kernel's error unaltered; the input body is never passed
/// through, so a blend that did not happen reads as a failed node.
/// An operand record the carry onto the blended body refuses is
/// [`NodeErrorKind::ContactCarry`].
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
    (doc, results): (&crate::doc::Doc<ProfileProgram>, &Results<T>),
    vals: &SlotValues<T>,
    tol: Tol,
) -> OpResult<T> {
    let (target_read, target) = (selection.read, selection.at);
    if selection.ents.is_empty() {
        unreachable!("an edge set holds at least one name (`Select::fault`), one entity per name");
    }
    // It computes in its operand's frame.
    let (body, operand) = finished_operand(results, target, tol)?;
    let size = slot_scalar(vals, verb.slots.size_slot);
    let target_value = value_of(results, target)?;
    let target_table = read_table(target_value);
    // D9 order; the kernel refuses a repeated edge itself.
    let mut edges: Vec<topo::EdgeKey> = selection.ents.iter().map(|&(_, edge)| edge).collect();
    edges.sort_unstable();
    let out = (verb.build)(edges, size)
        .run(&body, tol)
        .map_err(verb_refused)?;
    // A blend's run produces the blend record by construction; another
    // family here is a kernel bug, refused typed.
    let naming = crate::verbs::read_record(out.record, verb.record, verb.foreign_record)?;
    let Some(rec) = naming else {
        unreachable!(
            "{}, though the blend surgery is the record's only producer and always writes one",
            verb.no_records
        )
    };
    // The operand's records, as its door kept them, ride the blended
    // body through the lineage its birth records state. Caller-side,
    // until the blend's door carries them itself onto a finished body
    // that holds its records (heldrecords' ratified unit
    // `a-finished-body-holds-its-contact-records-at-every-door`).
    let carried = rec
        .carry_contacts(
            &target_value.carry.kept(doc, tol).contacts,
            &out.body,
            band(tol)?,
            T::region_lane(),
        )
        .map_err(|refusal| NodeErrorKind::ContactCarry { refusal })?;
    let table = (verb.emitter)(id, target, target_read, &target_table, &out.body, &rec)
        .map_err(NodeErrorKind::Naming)?;
    let inputs = crate::coincide::RowInputs {
        a: (target_read, &target_table),
        b: None,
        tool: None,
    };
    OpOut::plain(
        ValuePayload::Body(super::Framed::in_frame(
            Arc::new(out.body),
            operand.frame().clone(),
        )),
        table,
    )
    .holding(
        Arc::new(carried),
        vec![crate::coincide::CitedInput::Read(target_read)],
    )
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
    (doc, results): (&crate::doc::Doc<ProfileProgram>, &Results<T>),
    vals: &SlotValues<T>,
    tol: Tol,
) -> OpResult<T> {
    let (target_read, target) = (open.read, open.at);
    // It computes in its operand's frame.
    let (body, operand) = finished_operand(results, target, tol)?;
    let thickness = slot_scalar(vals, verb.slots.size_slot);
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
    let naming = crate::verbs::read_record(out.record, verb.record, verb.foreign_record)?;
    // The operand's records ride the thin solid through the shell's
    // lineage, as the split's ride its sides ([`wire_split`]).
    let shelled = topo::Shelled {
        body: out.body,
        naming,
    };
    let carried = shelled
        .carry_contacts(
            &value_of(results, target)?.carry.kept(doc, tol).contacts,
            band(tol)?,
            T::region_lane(),
        )
        .map_err(|refusal| NodeErrorKind::ContactCarry { refusal })?;
    let topo::Shelled { body, naming } = shelled;
    let table = (verb.emitter)(id, target, target_read, &target_table, &body, &naming)
        .map_err(NodeErrorKind::Naming)?;
    Ok(OpOut::plain(
        ValuePayload::Body(super::Framed::in_frame(
            Arc::new(body),
            operand.frame().clone(),
        )),
        table,
    )
    .holding(
        Arc::new(carried),
        vec![crate::coincide::CitedInput::Read(target_read)],
    ))
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
    use crate::names::{NameTable, StableName};
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
    pub(super) fn live<'n, P: crate::ProfilePayload>(
        name: &'n StableName,
        doc: &crate::doc::Doc<P>,
        read: crate::VarId,
    ) -> Result<Live<'n>, Box<ResolveError>> {
        if ResolveError::part_name(name, read, doc) {
            return Ok(Live(name));
        }
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
    /// kind, so it panics.
    pub(super) fn resolve_key<K: Denoted, P: crate::ProfilePayload>(
        name: &StableName,
        doc: &crate::doc::Doc<P>,
        read: crate::VarId,
        table: &NameTable,
        refuse: impl Fn(Box<ResolveError>) -> super::NodeErrorKind,
    ) -> Result<(u32, K), super::NodeErrorKind> {
        let live = live(name, doc, read).map_err(&refuse)?;
        key_in::<K>(table, live.0).map_err(|refusal| match refusal {
            TableRefusal::NoSuchName => refuse(vanished(&live)),
            TableRefusal::Ambiguous { candidates } => refuse(tied(&live, candidates)),
            TableRefusal::Kind { found } => unreachable!(
                "the selection's name {name:?} denotes {} {}, not the {} {} the selection holds, \
                 though every door that writes a selection holds its names at its kind and \
                 every table door seats a row at its name's kind",
                found.article(),
                found.noun(),
                K::KIND.article(),
                K::KIND.noun()
            ),
        })
    }

    /// Rung 2's payload: the tie row is the name itself.
    fn tied(live: &Live<'_>, width: usize) -> Box<ResolveError> {
        Box::new(ResolveError::ambiguous(live.0, live.0.node, width))
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
/// refuses.
///
/// # Panics
///
/// For a caller whose `K` is not the selection's kind, a wiring bug
/// ([`resolve_selection`]).
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
    let value = value_of(results, at)?;
    // A selection naming a family's members reads the family whole.
    let value = if crate::repetition::names_members(doc, selection) {
        value.whole()
    } else {
        value
    };
    resolve_selection(doc, &read_table(value), slot, var, at)
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
    let (Some((entity, crate::var::SelectionCount::One)), Some(selection)) = (
        doc.var(var).and_then(|held| held.kind().selection()),
        doc.selection(var),
    ) else {
        return Err(NodeErrorKind::UnresolvedRead { slot, var });
    };
    let [name] = selection.names.as_slice() else {
        unreachable!("a singleton selection holds one name (`Select::fault`)")
    };
    if name.kind != entity {
        unreachable!(
            "selection {var:?} holds {} {} name, though the edit and load doors hold a \
             selection's names at its kind, {} {}",
            name.kind.article(),
            name.kind.noun(),
            entity.article(),
            entity.noun()
        );
    }
    resolve_name(doc, results, slot, selection.body, name, |error| {
        NodeErrorKind::SelectResolve {
            slot,
            var,
            reference: 0,
            error,
        }
    })
}

/// **One name of the body `read` holds, resolved as a measure's
/// reference resolves it**: the read to the operation whose value
/// holds it, then the name through that value's table under the
/// [`ladder`], into an entity of the name's own kind. [`select_one`]
/// resolves a selection's one name here, and so does the quieting
/// rule's [`crate::checks::at_rest::row_quietable`] for a row's
/// cell, so the two read one entity.
///
/// # Errors
///
/// [`NodeErrorKind::UnresolvedRead`] for a read no live operation
/// defines, `refuse`'s for a name the ladder refuses.
pub(crate) fn resolve_name<P: crate::ProfilePayload, T: Decide>(
    doc: &crate::doc::Doc<P>,
    results: &Results<T>,
    slot: crate::OperandSlot,
    read: crate::VarId,
    name: &names::StableName,
    refuse: impl Fn(Box<crate::resolve::ResolveError>) -> NodeErrorKind,
) -> Result<(RecipeNodeId, names::EntityRef), NodeErrorKind> {
    use names::interrogate::Denoted;
    fn entity<K: Denoted>((body, key): (u32, K)) -> names::EntityRef {
        names::EntityRef {
            body,
            key: key.entity(),
        }
    }
    let at = super::read_at(doc, slot, read)?;
    let value = value_of(results, at)?;
    // A name of one of a family's members reads the family whole.
    let value = if crate::repetition::names_a_member(name, at) {
        value.whole()
    } else {
        value
    };
    let table = read_table(value);
    let ent = match name.kind {
        names::EntityKind::Face => entity(ladder::resolve_key::<topo::FaceKey, P>(
            name, doc, read, &table, refuse,
        )?),
        names::EntityKind::Edge => entity(ladder::resolve_key::<topo::EdgeKey, P>(
            name, doc, read, &table, refuse,
        )?),
        names::EntityKind::Vertex => entity(ladder::resolve_key::<topo::VertexKey, P>(
            name, doc, read, &table, refuse,
        )?),
        names::EntityKind::Body => unreachable!("a cell's name names a face, an edge or a vertex"),
    };
    Ok((at, ent))
}

/// **The body a resolved reference lands in**, as a measure reads it:
/// body `ent.body` of the value `at`, in its own frame. A reader of two
/// references reads them into one computing frame
/// ([`super::Framed::computing_among`]) before it compares them.
///
/// # Errors
///
/// [`NodeErrorKind::MissingInput`] for `at` with no value,
/// [`NodeErrorKind::MeasureRefUnreadable`] for a value holding no such
/// body.
pub(crate) fn reference_body<T: Decide>(
    results: &Results<T>,
    slot: crate::OperandSlot,
    var: crate::VarId,
    (at, ent): (RecipeNodeId, names::EntityRef),
) -> Result<&super::Framed<T>, NodeErrorKind> {
    crate::names::interrogate::output_body(&value_of(results, at)?.payload, ent.body)
        .map_err(|error| NodeErrorKind::MeasureRefUnreadable { slot, var, error })
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
    // author's.
    let held = doc
        .var(var)
        .and_then(|held| held.kind().selection())
        .map(|(entity, _)| entity);
    if held != Some(K::KIND) {
        unreachable!(
            "the {slot:?} seat is wired to read {} keys from a selection of {held:?}, though \
             the edit and load doors hold a selection variable's kind to its seat's",
            K::KIND.noun()
        );
    }
    let ents = selection
        .names
        .iter()
        .enumerate()
        .map(|(reference, name)| {
            ladder::resolve_key(name, doc, selection.body, table, |error| {
                NodeErrorKind::SelectResolve {
                    slot,
                    var,
                    reference,
                    error,
                }
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
fn wire_measure<'r, T: Decide + topo::AtRestPolicy + crate::mate::SolveScalar>(
    primitive: &crate::measure::MeasurePrimitive<crate::VarId>,
    doc: &crate::doc::Doc<ProfileProgram>,
    results_at: &dyn Fn(usize) -> &'r Results<T>,
    params: &crate::expr::VarEnv<T>,
    tol: Tol,
) -> OpResult<T> {
    let dim = primitive.dim();
    let mut cells = Vec::with_capacity(2);
    let mut poses = Vec::with_capacity(2);
    for (index, &var) in primitive.refs().into_iter().enumerate() {
        // Each reference reads its own body: two ports of one split are
        // two bodies (`reads_projected`).
        let results = results_at(index);
        let slot = crate::OperandSlot::Measured(
            primitive.kind(),
            u8::try_from(index).unwrap_or_else(|_| unreachable!("a primitive reads two")),
        );
        // A pose reference (D10 `[assertions.quiet]`): one read directly
        // off a cell is read off that cell ([`pose::read_off_cell`]); any
        // other (a split's tool, a construction) is evaluated.
        let off_cell = pose::read_directly_off(doc, var);
        if off_cell.is_none()
            && doc.var(var).is_some_and(|v| {
                matches!(
                    v.kind(),
                    crate::VarKind::Point
                        | crate::VarKind::Axis
                        | crate::VarKind::Plane
                        | crate::VarKind::Frame
                )
            })
        {
            let posed = pose::eval_posed(
                doc,
                results,
                slot,
                crate::pose::RoleRead::plain(var),
                params,
                tol,
            )?;
            poses.push((index, posed));
            continue;
        }
        let (read, kind) = off_cell.map_or((var, None), |(cell, kind)| (cell, Some(kind)));
        let (at, ent) = if doc.selection(read).is_some() {
            select_one(doc, results, slot, read)?
        } else {
            let at = super::read_at(doc, slot, read)?;
            (
                at,
                names::EntityRef {
                    body: 0,
                    key: names::EntityKey::Body,
                },
            )
        };
        let body = reference_body(results, slot, read, (at, ent))?;
        cells.push((index, at, ent, kind, body));
    }
    // Its two references are in one space (D′1): it measures in the
    // first cell's frame, or the second's where only that one is in its
    // space's coordinates ([`super::Framed::computing_among`]), and reads
    // the other cell into it through `relative`, a pose through its own
    // relative map. Two poses measure in their space's coordinates.
    let computing = super::Framed::computing_among(cells.iter().map(|(.., framed)| *framed))
        .or_else(|| cells.first().map(|(.., framed)| *framed));
    let bodies = cells
        .iter()
        .map(|(index, at, ent, kind, framed)| {
            let body = match computing {
                Some(computing) => computing.read_in(framed, tol)?,
                None => unreachable!("a cell gives the computing frame"),
            };
            Ok((*index, *at, *ent, *kind, body))
        })
        .collect::<Result<Vec<_>, NodeErrorKind>>()?;
    let b = band(tol)?;
    let mut posed = Vec::with_capacity(poses.len());
    for (index, read) in poses {
        let value = match computing {
            Some(computing) => read.read_into(computing.frame(), b)?,
            None => read.in_its_space(b)?,
        };
        let pose = super::measure::Pose::of(&value).unwrap_or_else(|| {
            unreachable!("a measure admits a point, an axis, a plane or a frame")
        });
        posed.push((index, pose));
    }
    let mut carriers: Vec<(usize, super::measure::Carrier<'_, T>)> = Vec::with_capacity(2);
    for (index, _, ent, kind, body) in &bodies {
        carriers.push((
            *index,
            match kind {
                None => super::measure::carrier_of(body, *ent),
                Some(kind) => pose::read_off_cell(body, *ent, *kind, b)?,
            },
        ));
    }
    carriers.extend(
        posed
            .into_iter()
            .map(|(index, pose)| (index, super::measure::Carrier::Pose { pose, reach: None })),
    );
    carriers.sort_by_key(|(index, _)| *index);
    let [(_, ca), (_, cb)] = carriers.as_slice() else {
        unreachable!("a primitive reads two references")
    };
    let measured = if matches!(
        primitive,
        crate::measure::MeasurePrimitive::MinClearance { .. }
    ) {
        // The E7 engine's, over the bodies this evaluation built: it
        // wants bodies and face scopes, not carriers.
        let Some(lane) = T::clearance_lane() else {
            // No value at this scalar: saying so at the node lets an
            // assertion over it report `Unevaluated` instead of being
            // poisoned.
            return Ok(OpOut::plain(
                ValuePayload::MeasureUnavailable {
                    reason: crate::measure::MeasureUnavailableAt::NeedsEnclosure {
                        verb: primitive.verb(),
                        scalar: T::NAME,
                        door: "topo::ClearanceLane",
                    },
                    dim,
                },
                names::empty(),
            ));
        };
        let measured = |i: usize| {
            let (_, at, ent, _, body) = &bodies[i];
            Measured {
                at: *at,
                index: ent.body,
                body: body.as_ref(),
                key: ent.key,
            }
        };
        let [_, _] = bodies.as_slice() else {
            unreachable!("a min_clearance reference reads a body or a face, never a pose")
        };
        let (a, b) = (measured(0), measured(1));
        let (oa, ob) = (a.clearance_operand(), b.clearance_operand());
        // The pair budget is the engine's small default: a drive of a few
        // hundred leaves pays this search a few hundred times.
        match lane.min_separation(
            &oa.side(),
            &ob.side(),
            oa.same_body(&ob),
            band(tol)?,
            topo::clearance::Budgets::MIN_SEPARATION,
        ) {
            Ok(Some(v)) => super::measure::finite(v),
            Ok(None) => {
                return Ok(OpOut::plain(
                    ValuePayload::MeasureUnavailable {
                        reason: crate::measure::MeasureUnavailableAt::NoUpperBound {
                            verb: primitive.verb(),
                            scalar: T::NAME,
                            door: "topo::clearance::min_separation",
                        },
                        dim,
                    },
                    names::empty(),
                ));
            }
            Err(refusal) => return Err(NodeErrorKind::MeasureClearanceRefused(refusal)),
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
    (value_dim, bound_dim): (crate::expr::Dimension, crate::expr::Dimension),
    relation: crate::measure::AssertionRelation,
    payload: &super::Payload<T>,
    tol: Tol,
) -> OpResult<T> {
    // A measured value with no value at this scalar is not a failed
    // node, so the assertion answers with E10's third state and the
    // requirement stays visible in a build that cannot check it.
    use super::measure::Observed;
    let (value, bound) = match payload.observed {
        [Observed::Unavailable(reason), _]
        | [Observed::Value(_), Observed::Unavailable(reason)] => {
            return Ok(OpOut::plain(
                ValuePayload::Assertion(crate::measure::AssertionVerdict::Unevaluated {
                    reason: crate::measure::UnevaluatedReason::MeasureUnavailable(reason),
                }),
                names::empty(),
            ));
        }
        [Observed::Value(value), Observed::Value(bound)] => (value, bound),
    };
    // The bound's DECLARED dimension must agree (units erase at the
    // evaluation boundary), through the rule the document doors ask.
    if crate::node::AssertionBoundFault::against(value_dim, bound_dim).is_some() {
        return Err(NodeErrorKind::AssertionDimension {
            measured: value_dim,
            bound: bound_dim,
        });
    }
    Ok(OpOut::plain(
        ValuePayload::Assertion(crate::measure::decide_assertion(
            value,
            bound,
            relation,
            band(tol)?,
        )),
        names::empty(),
    ))
}

/// **The split's lowering**, driven by its correspondence
/// ([`crate::verbs::split`]): one body and one POSE operand in, TWO
/// sides out under one record, stamped in one index space.
///
/// # Refusals
///
/// A tool that is not a plane pose is `WrongOperand`, decided before
/// any verb exists. Failure of the op itself is
/// [`NodeErrorKind::Split`] through [`verb_refused`]; nothing here
/// re-derives the plane. An operand record the carry onto the sides
/// refuses is [`NodeErrorKind::ContactCarry`].
fn wire_split<T: Decide + geom_core::Bounds + topo::AtRestPolicy>(
    verb: &crate::verbs::split::SplitVerb<T>,
    id: RecipeNodeId,
    (target_read, target): (crate::VarId, RecipeNodeId),
    (tool, tool_read): (pose::Posed<T>, crate::pose::RoleRead<crate::VarId>),
    (doc, results): (&crate::doc::Doc<ProfileProgram>, &Results<T>),
    tol: Tol,
) -> OpResult<T> {
    // It computes in its operand's frame, its tool read into it.
    let (body, operand) = finished_operand(results, target, tol)?;
    let tool = &tool.read_into(operand.frame(), band(tol)?)?;
    let plane = (verb.tool)(tool).unwrap_or_else(|| {
        unreachable!(
            "the door admits {} alone at a split's tool, not {}",
            verb.tool_expected,
            tool.kind()
        )
    });
    let built = (verb.build)(plane);
    let out = built.run_split(&body, tol).map_err(verb_refused)?;
    let naming = crate::verbs::read_record(out.record, verb.record, verb.foreign_record)?;
    // The operand's records, as its door kept them ([`super::Carry::kept`]),
    // ride each side through the split's lineage, each citing the
    // operand's record it is; a record the cut parts across the cut
    // retires, a touch between the two sides. Caller-side, until the
    // split's door carries them itself onto a finished body that holds
    // its records (heldrecords' ratified unit
    // `a-finished-body-holds-its-contact-records-at-every-door`).
    let split = topo::SplitResult {
        above: out.above,
        below: out.below,
        naming,
        coincidences: out.coincidences,
    };
    let (carried, _) = split
        .carry_contacts_across(
            &value_of(results, target)?.carry.kept(doc, tol).contacts,
            band(tol)?,
            T::region_lane(),
        )
        .map_err(|refusal| NodeErrorKind::ContactCarry { refusal })?;
    let topo::SplitResult {
        above,
        below,
        naming,
        coincidences,
    } = split;
    let side = |part: SplitPart<T>| match part {
        // The verdict the split's gate kept is not carried in a value yet (F2).
        SplitPart::Body(b) => SplitSide::Body(super::Framed::in_frame(
            Arc::new(b.into_body()),
            operand.frame().clone(),
        )),
        SplitPart::Empty => SplitSide::Empty,
    };
    let above = side(above);
    let below = side(below);
    let as_body = |s: &SplitSide<T>| match s {
        SplitSide::Body(b) => Some(Arc::clone(b.shared())),
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
    // Each side's carry at its half's output body, its records citing
    // the operand's read.
    let mut sides = [
        (SplitHalf::Above, carried.above),
        (SplitHalf::Below, carried.below),
    ];
    sides.sort_by_key(|(half, _)| half.output_body());
    let sides = sides.map(|(_, records)| {
        super::Carry::default().recording(
            Arc::new(records),
            vec![crate::coincide::CitedInput::Read(target_read)],
        )
    });
    OpOut::per_output(ValuePayload::Split { above, below }, outputs)
        .carrying(super::Carry {
            members: Some(Arc::from(sides)),
            ..super::Carry::default()
        })
        .recording(
            &coincidences,
            &crate::coincide::RowInputs {
                a: (target_read, &target_table),
                b: None,
                tool: Some(tool_read),
            },
        )
}

/// **A mirror** (D10 `[repetition]`): the body `source` defines,
/// reflected across `plane` by the kernel's reflection door
/// (`topo::mirror_body`), a construction, not a copy. The door keeps
/// every arena key, so the image carries the source's names verbatim,
/// and carries what the source carries as a copy carries it
/// ([`super::Carry::copied`]): its records, findings and census, each
/// meeting inside it the source's. No rigid motion
/// stands the source's pieces in the image, so it holds none.
fn wire_mirror<T: Decide + topo::AtRestPolicy>(
    read: crate::VarId,
    source: RecipeNodeId,
    plane: pose::Posed<T>,
    results: &Results<T>,
    tol: Tol,
) -> OpResult<T> {
    // It computes in its source's frame, its plane read into it.
    let body = read_body(results, source)?;
    let plane = &plane.read_into(body.frame(), band(tol)?)?;
    let PoseValue::Plane { normal, offset } = plane else {
        unreachable!(
            "the door admits a plane alone at a mirror's plane, not {}",
            plane.kind()
        )
    };
    let reflection =
        topo::Reflection::across_unit(PoseValue::plane_foot(*normal, *offset), *normal);
    let image = topo::mirror_body(body.body(), &reflection, tol).map_err(NodeErrorKind::Mirror)?;
    let value = value_of(results, source)?;
    let table = read_table(value)
        .project(0)
        .map_err(|dup| NodeErrorKind::Naming(names::NamingError::from(dup)))?;
    names::check_total(&table, &image, 0).map_err(NodeErrorKind::Naming)?;
    Ok(OpOut::plain(
        ValuePayload::Body(super::Framed::in_frame(
            Arc::new(image),
            body.frame().clone(),
        )),
        Arc::new(table),
    )
    .carrying(value.carry.copied(
        super::carry::Stands::Reflected,
        &crate::coincide::CitedInput::Read(read),
    )?))
}

/// **The body of `of`'s `Instances` value the count in `slot` picks**,
/// with its index into the value's list of bodies: a piece's family's
/// members in index order. A negative index and one past the end refuse
/// alike ([`NodeErrorKind::MemberOutOfRange`]). A list past u32 is its
/// minting op's own emission bug, refused before any index is judged
/// against it.
fn instance_of<T: Decide>(
    of: RecipeNodeId,
    instances: &[super::Framed<T>],
    vals: &SlotValues<T>,
    slot: SlotId,
) -> Result<(super::Framed<T>, u32), NodeErrorKind> {
    let index = slots::count(vals, slot).ok_or(NodeErrorKind::MissingSlot { slot })?;
    let count = names::output_body(instances.len()).map_err(NodeErrorKind::Naming)?;
    let ix = u32::try_from(index).ok().filter(|i| *i < count).ok_or(
        NodeErrorKind::MemberOutOfRange {
            input: of,
            at: vec![index],
        },
    )?;
    Ok((instances[ix as usize].clone(), ix))
}

/// **The member of `of`'s family an indexed read picks**, with its
/// place among the family's bodies: the member whose index values are
/// the read's `rank` index slots at `seat`, in order. A read of another
/// rank, or at values no member stands at, refuses as one out of range
/// ([`NodeErrorKind::MemberOutOfRange`]).
fn member_of<T: Decide>(
    of: RecipeNodeId,
    members: &super::members::Members<T>,
    instances: &[super::Framed<T>],
    vals: &SlotValues<T>,
    seat: crate::OperandSlot,
    rank: usize,
) -> Result<(super::Framed<T>, u32), NodeErrorKind> {
    let at = (0..rank)
        .map(|k| {
            let slot = SlotId::Index {
                seat,
                k: u8::try_from(k).unwrap_or(u8::MAX),
            };
            slots::count(vals, slot).ok_or(NodeErrorKind::MissingSlot { slot })
        })
        .collect::<Result<Vec<i64>, _>>()?;
    let out = || NodeErrorKind::MemberOutOfRange {
        input: of,
        at: at.clone(),
    };
    if rank != members.over.len() {
        return Err(out());
    }
    let narrow: Vec<u32> = at
        .iter()
        .map(|&v| u32::try_from(v).ok())
        .collect::<Option<_>>()
        .ok_or_else(out)?;
    let index = members.position(&narrow).ok_or_else(out)?;
    let body = instances.get(index).cloned().ok_or_else(out)?;
    Ok((
        body,
        names::output_body(index).map_err(NodeErrorKind::Naming)?,
    ))
}

/// One half of a split's value, or [`NodeErrorKind::EmptyHalf`].
fn split_side<T: Decide>(
    of: RecipeNodeId,
    half: SplitHalf,
    above: &SplitSide<T>,
    below: &SplitSide<T>,
) -> Result<super::Framed<T>, NodeErrorKind> {
    match match half {
        SplitHalf::Above => above,
        SplitHalf::Below => below,
    } {
        SplitSide::Body(b) => Ok(b.clone()),
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
        .map(|(seat, var)| {
            // A pose read directly off a cell reads that cell's body.
            let cell = pose::read_directly_off(doc, var).map_or(var, |(cell, _)| cell);
            (seat, doc.selection(cell).map_or(var, |select| select.body))
        })
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
fn reads_projected<T: Decide>(
    node: &Node<ProfileProgram>,
    rows: &[(crate::OperandSlot, crate::VarId)],
    doc: &crate::doc::Doc<ProfileProgram>,
    (results, vals): (&Results<T>, &SlotValues<T>),
) -> Result<Option<Results<T>>, NodeErrorKind> {
    let mut replaced: Vec<(RecipeNodeId, u32, super::NodeValue<T>)> = Vec::new();
    for &(seat, var) in rows {
        if let Some(pick) = doc.pick(var) {
            let placement = super::read_at(doc, seat, var)?;
            let indexed = node
                .body_read(seat)
                .is_some_and(crate::BodyRead::is_indexed);
            replaced.push(pick_of(
                (var, pick),
                placement,
                indexed,
                seat,
                (results, vals),
            )?);
            continue;
        }
        if node
            .body_read(seat)
            .is_some_and(crate::BodyRead::is_indexed)
        {
            let family = super::read_at(doc, seat, var)?;
            let value = value_of(results, family)?.whole();
            let ValuePayload::Instances(instances) = &value.payload else {
                return Err(wrong_operand(value, family, super::family::INSTANCES));
            };
            let rank = node.body_read(seat).map_or(0, |read| read.at.len());
            let (body, index) = match &value.members {
                Some(members) => member_of(family, members, instances, vals, seat, rank)?,
                None => instance_of(family, instances, vals, SlotId::Index { seat, k: 0 })?,
            };
            // A family's one table holds every member at its place.
            let table = read_table(value)
                .project(index)
                .map_err(|dup| NodeErrorKind::Naming(names::NamingError::from(dup)))?;
            let names = super::OutputNames::new(Arc::new(table), Arc::default());
            let (_, index_at, projected) = projected(family, value, body, names, index)?;
            // A member carries its own carry: its records, the pieces and
            // the census its copy holds where it stands.
            let carry = value.carry.member(index as usize).clone();
            replaced.push((family, index_at, super::NodeValue { carry, ..projected }));
            continue;
        }
        replaced.extend(split_port(var, doc, results)?);
    }
    Ok(with_replaced(results, replaced))
}

/// **One piece of a copy, read by its pick** ([`crate::VarDef::Pick`]):
/// the copy's pieces walked by the pick's path, the piece reached where
/// its holder stands, with its own names, and what it carries as a copy
/// carries it ([`super::Carry::copied`]), its source the piece's read;
/// for a piece that is a family, the member at the read's index. A path
/// naming no piece, and a read at the other shape, refuse typed: which
/// pieces a copy holds is this evaluation's to answer.
fn pick_of<T: Decide>(
    (var, pick): (crate::VarId, &crate::var::Pick),
    placement: RecipeNodeId,
    indexed: bool,
    seat: crate::OperandSlot,
    (results, vals): (&Results<T>, &SlotValues<T>),
) -> Result<(RecipeNodeId, u32, super::NodeValue<T>), NodeErrorKind> {
    let value = value_of(results, placement)?;
    let mut pieces = value.carry.pieces.as_ref();
    let mut reached: Option<&super::Piece<T>> = None;
    for (at, &step) in pick.of.iter().enumerate() {
        let piece = pieces
            .and_then(|held| held.piece(step))
            .ok_or(NodeErrorKind::NoSuchPiece { pick: var, at })?;
        pieces = piece.carry.pieces.as_ref();
        reached = Some(piece);
    }
    let Some(piece) = reached else {
        return Err(NodeErrorKind::NoSuchPiece { pick: var, at: 0 });
    };
    if piece.family != indexed {
        return Err(NodeErrorKind::PieceShape {
            pick: var,
            family: piece.family,
        });
    }
    let bodies = &piece.bodies;
    let (body, table, member_carry) = if indexed {
        let (body, index) = instance_of(placement, bodies, vals, SlotId::Index { seat, k: 0 })?;
        let table = piece
            .table
            .project(index)
            .map_err(|dup| NodeErrorKind::Naming(names::NamingError::from(dup)))?;
        (body, table, piece.carry.member(index as usize))
    } else {
        let Some(body) = bodies.first() else {
            unreachable!("a piece that is a body holds one")
        };
        (body.clone(), piece.table.as_ref().clone(), &piece.carry)
    };
    names::check_total(&table, body.body(), 0).map_err(NodeErrorKind::Naming)?;
    // The piece already stands where its holder stands.
    let carry = member_carry.copied(
        super::carry::Stands::Still,
        &crate::coincide::CitedInput::Read(piece.read),
    )?;
    Ok((
        placement,
        u32::MAX - var.0.ordinal(),
        super::NodeValue {
            payload: ValuePayload::Body(body),
            outputs: Arc::new([super::OutputNames::new(Arc::new(table), Arc::default())]),
            carry,
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
    let names = value.output(port).cloned().unwrap_or_else(|| {
        unreachable!(
            "split {split:?}'s value holds no name table for its {half:?} half (port {port}), \
             though the split's lowering names each half's output"
        )
    });
    let (of, index, projected) = projected(split, value, body, names, half.output_body())?;
    // A half carries its own side's records, keyed in its arena.
    let at = usize::try_from(half.output_body()).unwrap_or(usize::MAX);
    let carry = value.carry.member(at).clone();
    Ok(Some((of, index, super::NodeValue { carry, ..projected })))
}

/// `value`'s output `index` as the body alone, at output 0, with that
/// output's names.
fn projected<T: Decide>(
    of: RecipeNodeId,
    value: &super::NodeValue<T>,
    body: super::Framed<T>,
    names: super::OutputNames,
    index: u32,
) -> Result<(RecipeNodeId, u32, super::NodeValue<T>), NodeErrorKind> {
    names::check_total(names.table(), body.body(), 0).map_err(NodeErrorKind::Naming)?;
    Ok((
        of,
        index,
        super::NodeValue {
            payload: ValuePayload::Body(body),
            outputs: Arc::new([names]),
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

/// **One member of a union or an intersect, as read and not yet
/// finished**: the read it came through (its names' key), its body in
/// its own frame and its own table. The fold reads it into the frame it
/// computes in and finishes it there, once ([`MemberRead::finished_in`]).
#[derive(Clone)]
struct MemberRead<T: Decide> {
    read: crate::VarId,
    /// The node it is read from (error context).
    node: RecipeNodeId,
    /// Where it stands: its body in its own frame.
    framed: super::Framed<T>,
    table: Arc<NameTable>,
    /// What its body carries, as a reader reads it
    /// ([`super::Carry::kept`]): its records, carried into the fold step
    /// that joins it (D1 (ii)), and what a union or an intersect of it
    /// alone carries on ([`lone_member`]).
    carry: super::Carry<T>,
    /// The member as an intersect distributes over it, where it holds
    /// pieces ([`distribute`]).
    pieces: Option<Arc<distribute::Operand<T>>>,
}

/// **One member of a union or an intersect, as the fold reads it**:
/// a [`MemberRead`] read into the frame the fold computes in, its body
/// finished there.
#[derive(Clone)]
struct Member<T: Decide> {
    read: crate::VarId,
    /// Where it stands, in the frame the fold computes in.
    framed: super::Framed<T>,
    /// Its body finished in that frame, through the at-rest gate once.
    body: Arc<topo::AtRestBody<T>>,
    table: Arc<NameTable>,
    /// What its body carries ([`MemberRead::carry`]).
    carry: super::Carry<T>,
}

impl<T: Decide + topo::AtRestPolicy> MemberRead<T> {
    /// **This member read into `computing`'s frame and finished there**,
    /// through the at-rest gate once: its own body where the two are one
    /// frame, else its body mapped once ([`super::Framed::read_in`]).
    fn finished_in(
        &self,
        computing: &super::Framed<T>,
        tol: Tol,
    ) -> Result<Member<T>, NodeErrorKind> {
        let read = computing.read_in(&self.framed, tol)?;
        let framed = if Arc::ptr_eq(&read, self.framed.shared()) {
            self.framed.clone()
        } else {
            super::Framed::in_frame(Arc::clone(&read), computing.frame().clone())
        };
        let body = T::gate_at_rest_kept((*read).clone(), tol).map_err(|errors| {
            NodeErrorKind::UnfinishedOperand {
                input: self.node,
                errors,
            }
        })?;
        Ok(Member {
            read: self.read,
            framed,
            body: Arc::new(body),
            table: Arc::clone(&self.table),
            carry: self.carry.clone(),
        })
    }
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
/// ([`names::RoleSeg::From`]). A `from` that holds pieces is cut piece
/// by piece ([`distribute`]).
fn wire_subtract<T: Decide + geom_core::Bounds + topo::AtRestPolicy>(
    verb: &crate::verbs::boolean::PairVerb<T>,
    id: RecipeNodeId,
    (a_read, a, results_a): (crate::VarId, RecipeNodeId, &Results<T>),
    (b_read, b, results_b): (crate::VarId, RecipeNodeId, &Results<T>),
    doc: &crate::doc::Doc<ProfileProgram>,
    boolean_sweep: topo::SweepStrategy,
    tol: Tol,
) -> OpResult<T> {
    let from = distribute::Operand::read(results_a, a, a_read, doc, tol)?;
    let tool = distribute::Operand::read(results_b, b, b_read, doc, tol)?;
    // A body with pieces is finished piece by piece.
    let body_a = match from.carry.pieces {
        None => Some(from.finished(tol)?),
        Some(_) => None,
    };
    // The tool finished in its own frame, once: read as it stands
    // wherever a cut computes in that frame.
    let body_b = tool.finished(tol)?;
    subtract_from(
        verb,
        id,
        (&from, body_a),
        (&tool, &body_b),
        boolean_sweep,
        tol,
    )
}

/// `from` cut by `tool`, by piece where `from` holds pieces; `from`'s
/// body finished where the caller has finished it. Each cut computes in
/// its `from`'s frame, or the tool's where only that one is in its space's
/// coordinates, and
/// reads the other into it ([`super::Framed::relative`]);
/// `body_b` is the tool finished in its own frame.
fn subtract_from<T: Decide + geom_core::Bounds + topo::AtRestPolicy>(
    verb: &crate::verbs::boolean::PairVerb<T>,
    id: RecipeNodeId,
    (from, body_a): (&distribute::Operand<T>, Option<topo::AtRestBody<T>>),
    (b, body_b): (&distribute::Operand<T>, &topo::AtRestBody<T>),
    boolean_sweep: topo::SweepStrategy,
    tol: Tol,
) -> OpResult<T> {
    if from.carry.pieces.is_some() {
        let outs = from
            .units()?
            .iter()
            .map(|unit| {
                let out = subtract_from(verb, id, (unit, None), (b, body_b), boolean_sweep, tol)?;
                Ok((unit.read, out))
            })
            .collect::<Result<Vec<_>, NodeErrorKind>>()?;
        return distribute::assembled(id, outs, tol);
    }
    let body_a = match body_a {
        Some(body) => body,
        None => from.finished(tol)?,
    };
    // It computes in `from`'s frame unless only the tool is in its space's
    // coordinates
    // ([`super::Framed::computing_among`]).
    let computing = super::Framed::computing_among([&from.body, &b.body])
        .unwrap_or(&from.body)
        .clone();
    let body_a = from.finished_in(&computing, &body_a, tol)?;
    let body_b = &b.finished_in(&computing, body_b, tol)?;
    let inputs = BooleanInputs {
        records_a: from.carry.contacts.carried(),
        records_b: b.carry.contacts.carried(),
    };
    match (verb.build)(BooleanOp::Subtract, inputs)
        .run_pair(&body_a, body_b, boolean_sweep, tol)
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
                    node: from.node,
                    read: from.read,
                    table: &from.table,
                    body: &body_a,
                },
                &names::OperandCtx {
                    node: b.node,
                    read: b.read,
                    table: &b.table,
                    body: body_b,
                },
                tol,
            )
            .map_err(NodeErrorKind::Naming)?;
            let body = out.body.into_body();
            OpOut::plain(
                ValuePayload::Boolean(BooleanValue::Body {
                    body: super::Framed::in_frame(Arc::new(body), computing.frame().clone()),
                    kind,
                }),
                emitted.table,
            )
            .grouped(Arc::new(names::FragmentGroups::minted(&emitted.groups)))
            .holding(
                Arc::new(contacts),
                vec![
                    crate::coincide::CitedInput::Read(from.read),
                    crate::coincide::CitedInput::Read(b.read),
                ],
            )
            .recording(
                &out.coincidences,
                &crate::coincide::RowInputs {
                    a: (from.seat, from.spelled()?.as_ref()),
                    b: Some((b.read, &b.table)),
                    tool: None,
                },
            )
        }
    }
}

/// **One row a fold step returned**, until the records are recited.
enum FoldRow {
    /// It restates given pairs' decisions: the judged rows of their
    /// verdicts, by place ([`topo::NaryStep::restates`]).
    Restates(Vec<usize>),
    /// The step decided it, in member space ([`in_member_space`]).
    Decided(crate::coincide::NamedCoincidence),
}

/// Whether `a` and `b` make one statement: one relation between one
/// multiset of cells. Which cell a decision read first follows operand
/// order and sweep order, and says nothing about the coincidence.
fn one_statement(
    a: &crate::coincide::NamedCoincidence,
    b: &crate::coincide::NamedCoincidence,
) -> bool {
    fn sorted(r: &crate::coincide::NamedCoincidence) -> Vec<&crate::coincide::NamedCell> {
        let mut cells: Vec<&crate::coincide::NamedCell> = r.cells.iter().collect();
        cells.sort();
        cells
    }
    a.relation == b.relation && sorted(a) == sorted(b)
}

/// Whether the fold row `row` is the judged row `judged`: one statement
/// decided at one point ([`crate::coincide::NamedCoincidence::at_one_vertex`]).
/// One statement can hold at two points (an edge touching a curved face
/// at two isolated points), and a statement whose cells fix no point is
/// its own row.
fn restates(
    judged: &crate::coincide::NamedCoincidence,
    row: &crate::coincide::NamedCoincidence,
) -> bool {
    one_statement(judged, row) && judged.at_one_vertex(row)
}

/// Where `rows` holds the judged row `row` restates, pushing it where
/// none does. Two judged rows are one only when two different pairs
/// made them and they agree exactly, site, margin and discharge too:
/// that is a member read twice, whose pairs are bit-identical runs. One
/// pair's two rows are two decisions (an edge touching a curved face at
/// two isolated points), and both publish.
fn judged_at(
    rows: &mut Vec<crate::coincide::NamedCoincidence>,
    row: crate::coincide::NamedCoincidence,
) -> usize {
    rows.iter()
        .position(|r| {
            one_statement(r, &row)
                && r.origin != row.origin
                && r.site == row.site
                && r.margin == row.margin
                && r.discharge == row.discharge
        })
        .unwrap_or_else(|| {
            rows.push(row);
            rows.len() - 1
        })
}

/// **A fold step's row in member space** (DM4, "A row names member
/// cells"): each cell, named in the node's own space `own_read` by its
/// collapsed fold name, is the member cells it is made of
/// ([`names::member_cells`]). A point decided on a cell keeps
/// [`topo::Relation::OnCarrier`] where the point is one member vertex
/// and the cell one member cell, in the order the step read them; every
/// other point decided on a cell is the [`topo::Relation::Meet`] of the
/// member cells, sorted. `step` is the place of the member the step
/// folds in.
///
/// # Errors
///
/// [`crate::coincide::NamedCoincidence::meet`]'s refusals.
///
/// # Panics
///
/// For a row whose cells are not named in the node's own space, the
/// only space a fold step's rows are named in; for a row other than a
/// `Meet` that is not two cells, as only a `Meet` holds more
/// (`topo::Coincidence::more`); or for a decision other than a
/// point's on a cell whose cells are not one member cell each: a fold
/// step runs with the pairwise verdicts given (`topo::nary_boolean`),
/// so a carrier pair's row is the judgement's and no step returns one.
fn in_member_space(
    row: crate::coincide::NamedCoincidence,
    own_read: crate::VarId,
    step: u32,
) -> Result<crate::coincide::NamedCoincidence, names::NamingError> {
    use crate::coincide::NamedCell;
    let spelled = row
        .cells
        .iter()
        .map(|cell| match cell {
            NamedCell::Entity { input, name } if *input == own_read => names::member_cells(name),
            _ => unreachable!(
                "union fold step {step}'s row names {cell:?}, outside the node's own read \
                 {own_read:?}, though a fold step's rows are named in its two operands' tables, \
                 both keyed by that read"
            ),
        })
        .collect::<Vec<_>>();
    let entity =
        |(input, name): (crate::VarId, names::StableName)| NamedCell::Entity { input, name };
    let single = |cells: &[(crate::VarId, names::StableName)]| match cells {
        [one] => Some(one.clone()),
        _ => None,
    };
    let origin = crate::coincide::RowOrigin::Fold { step };
    let point = |cell: &NamedCell| matches!(cell, NamedCell::Entity { name, .. } if name.kind == names::EntityKind::Vertex);
    let two = |a: Vec<_>, b: Vec<_>, relation| match (single(&a), single(&b)) {
        (Some(a), Some(b)) => Ok(crate::coincide::NamedCoincidence {
            cells: vec![entity(a), entity(b)],
            relation,
            origin,
            ..row.clone()
        }),
        (a, b) => unreachable!(
            "union fold step {step} decided {relation:?} between cells made of several member \
             cells ({a:?}, {b:?}), though a step runs with the pairwise verdicts given and \
             decides no carrier pair itself"
        ),
    };
    let meet = |spelled: Vec<Vec<_>>| {
        crate::coincide::NamedCoincidence::meet(
            spelled.into_iter().flatten().map(entity).collect(),
            row.site,
            (row.margin, row.discharge),
            origin,
        )
    };
    // A meeting is every member cell its cells are made of.
    if row.relation == topo::Relation::Meet {
        return meet(spelled);
    }
    let mut spelled = spelled.into_iter();
    let (Some(p), Some(x), None) = (spelled.next(), spelled.next(), spelled.next()) else {
        unreachable!(
            "union fold step {step}'s {:?} row holds {} cells, though `name_rows` names a \
             kernel row's two, and more only on a `Meet`",
            row.relation,
            row.cells.len()
        )
    };
    match row.relation {
        // A point the step decided on a cell: one member vertex on one
        // member cell as read, else the meeting of every member cell
        // both are made of.
        topo::Relation::OnCarrier if point(&row.cells[0]) => match (single(&p), single(&x)) {
            (Some(v), Some(_)) if v.1.kind == names::EntityKind::Vertex => {
                two(p, x, topo::Relation::OnCarrier)
            }
            _ => meet(vec![p, x]),
        },
        // Any other `OnCarrier` is a carrier pair's (a sphere's centre
        // on a cylinder's axis, `CoaxialSphere`), which a fold step does
        // not decide: it runs with the pairwise judgement's verdicts
        // given (`topo::nary_boolean`), and a carrier pair's row is
        // the judgement's. So no cell of one is widened, and a step
        // that returns one made of several cells is a kernel bug.
        _ => two(p, x, row.relation),
    }
}

/// **The union's and the intersect's lowering** (DM4): the kernel's
/// n-ary door over the members in list order (`verbs::Members`,
/// `topo::nary_boolean`), which judges the member pairs and folds
/// `((m0 ∘ m1) ∘ m2) ∘ …` with the pair boolean, one body out in the
/// `BooleanValue::Body` shape a subtract yields.
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
/// rule), by the kernel: each member pair whose boxes meet runs as its
/// own two-member union or intersect, its operand A the lesser read
/// ([`member_ranks`]), so its verdicts and refusals are the same in
/// every member order, and the face pairs each judgement consumed are
/// the node's face links (N2, [`names::UnionLinks`]). **One verdict per
/// carrier pair**: the kernel gives each step the verdicts on the member
/// it joins, read through the lineage it advances
/// ([`topo::MemberLineage`], which [`names::UnionFold`] reads), and the
/// step decides no carrier pair of its own. A refusal only a step raises
/// names the member that step folds in
/// ([`NodeErrorKind::UnionFoldStep`]). Two members are one pair: their
/// one step is the judgement, run and logged once, its refusals the
/// pair's own.
///
/// **The node's rows are its evaluation's decisions** (DM4): the
/// judged rows, each marked by its pair ([`crate::coincide::RowOrigin`]),
/// then every row each fold step decided, in fold order, spelled as the
/// member cells that meet ([`in_member_space`]); a fold row stating what
/// a judged row states at the same point is that row ([`restates`]), and
/// one restating a given pair's decision is the judged rows of its
/// verdicts ([`topo::NaryStep::restates`]). The judged rows are the same in every
/// member order, modulo the members' places, and the fold's follow the
/// order, as a step's refusal does.
///
/// **Nothing ∅-absorbing is invented** (D3). A member that evaluates to
/// an empty boolean refuses `EmptyOperand` naming that member. An
/// intersect whose pair or fold step is empty is the typed empty
/// result. A union fold step that returns empty from two real bodies is
/// a kernel bug, at every step including the last: it panics and names
/// no member, since each one is fine.
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
    // Each member of a family read whole, in index order, its table the
    // family's rows for that member.
    let whole = |read: crate::VarId, at: RecipeNodeId, value: &super::NodeValue<T>| {
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
                names::check_total(&table, body.body(), 0).map_err(NodeErrorKind::Naming)?;
                Ok(MemberRead {
                    read,
                    node: at,
                    framed: body.clone(),
                    table: Arc::new(table),
                    // Each member's own, its records keyed in its arena.
                    carry: value.carry.member(i).kept(doc, tol),
                    pieces: None,
                })
            })
            .collect::<Result<Vec<_>, NodeErrorKind>>()
    };
    let members: Vec<MemberRead<T>> = match bodies {
        crate::Bodies::Spelled(reads) => {
            let mut members = Vec::with_capacity(reads.len());
            for (i, member) in reads.iter().enumerate() {
                let read = member.read;
                let slot = O::Member(u32::try_from(i).unwrap_or(u32::MAX));
                let at = super::read_at(doc, slot, read)?;
                let projected = reads_projected(node, &[(slot, read)], doc, (results, vals))?;
                let results = projected.as_ref().unwrap_or(results);
                let value = value_of(results, at)?.whole();
                // A family spelled as one read is its every member
                // (D10 `[repetition]`).
                if value.members.is_some() && !member.is_member() {
                    members.extend(whole(read, at, value)?);
                    continue;
                }
                let pieces = match op {
                    BooleanOp::Intersect if value.carry.pieces.is_some() => Some(Arc::new(
                        distribute::Operand::read(results, at, read, doc, tol)?,
                    )),
                    _ => None,
                };
                members.push(MemberRead {
                    read,
                    node: at,
                    framed: read_body(results, at)?,
                    table: read_table(value),
                    carry: value.carry.kept(doc, tol),
                    pieces,
                });
            }
            members
        }
        crate::Bodies::Family(family) => {
            // Every door refuses an indexed family argument
            // (`InputFault::IndexedFamily`); one reaching here would
            // union the whole family, so it is refused, not read.
            if family.is_member() {
                unreachable!(
                    "union {id:?}'s family argument {family:?} carries an index, though every \
                     door refuses an indexed family argument (`InputFault::IndexedFamily`)"
                );
            }
            let read = &family.read;
            let at = super::read_at(doc, O::Members, *read)?;
            whole(*read, at, value_of(results, at)?.whole())?
        }
    };
    // The node's own read: the space a fold step's rows are read in.
    let Some(own_read) = doc.output(id, 0) else {
        unreachable!(
            "union {id:?} defines no output variable, though the doors that insert an \
             operation define its outputs with it"
        )
    };
    combine_members(verb, op, id, own_read, members, boolean_sweep, tol)
}

/// **An intersect distributes over a member that holds pieces**
/// ([`distribute`]): the first such member's pieces each take its place
/// in turn, and what each intersect made is assembled. Every other
/// combine folds its members ([`fold_members`]).
fn combine_members<T: Decide + geom_core::Bounds + topo::AtRestPolicy>(
    verb: &crate::verbs::boolean::PairVerb<T>,
    op: BooleanOp,
    id: RecipeNodeId,
    own_read: crate::VarId,
    members: Vec<MemberRead<T>>,
    boolean_sweep: topo::SweepStrategy,
    tol: Tol,
) -> OpResult<T> {
    let held = members
        .iter()
        .enumerate()
        .find_map(|(i, m)| m.pieces.clone().map(|whole| (i, whole)));
    let Some((i, whole)) = held else {
        return fold_members(verb, op, id, own_read, &members, boolean_sweep, tol);
    };
    let outs = whole
        .units()?
        .into_iter()
        .map(|unit| {
            let mut replaced = members.clone();
            let unit = Arc::new(unit);
            replaced[i] = MemberRead {
                read: unit.read,
                node: unit.node,
                framed: unit.body.clone(),
                table: Arc::clone(&unit.table),
                carry: unit.carry.clone(),
                pieces: unit.carry.pieces.is_some().then(|| Arc::clone(&unit)),
            };
            let out = combine_members(verb, op, id, own_read, replaced, boolean_sweep, tol)?;
            // A row names the piece's cells as the member's read spells
            // them, unless another member is read through the piece's
            // own read, whose cells a row could not tell apart.
            let shared = members
                .iter()
                .enumerate()
                .any(|(j, m)| j != i && m.read == unit.read);
            let out = if shared {
                out
            } else {
                let rows = unit.respelled(&out.coincidences)?;
                out.with_coincidences(rows)
            };
            Ok((unit.read, out))
        })
        .collect::<Result<Vec<_>, NodeErrorKind>>()?;
    distribute::assembled(id, outs, tol)
}

/// **The union's fast path** (D10 `[repetition]`): the member pairs
/// that are rigid images of one body — a family's members, each a
/// [`super::Framed`] over its source's one `Arc` — and that the body's
/// separation certificate ([`topo::Separation`]) holds apart, read in
/// `computing`'s coordinates. Each such pair cannot meet, so the n-ary
/// pass judges only the rest (DM4's pairwise pass).
fn images_apart<T: Decide + geom_core::Bounds>(
    members: &[MemberRead<T>],
    computing: &super::Framed<T>,
    tol: Tol,
) -> Result<topo::Apart, NodeErrorKind> {
    let mut pairs = Vec::new();
    let mut seen = vec![false; members.len()];
    for (i, m) in members.iter().enumerate() {
        if seen[i] {
            continue;
        }
        let images: Vec<usize> = (i..members.len())
            .filter(|&j| Arc::ptr_eq(members[j].framed.shared(), m.framed.shared()))
            .collect();
        for &j in &images {
            seen[j] = true;
        }
        if images.len() < 2 {
            continue;
        }
        let maps = images
            .iter()
            .map(|&j| {
                Ok(computing
                    .relative(&members[j].framed)
                    .map_err(NodeErrorKind::Motion)?
                    .map_or_else(geom_core::Affine3::identity, |map| map.to_affine()))
            })
            .collect::<Result<Vec<geom_core::Affine3<T>>, NodeErrorKind>>()?;
        let certificate =
            topo::Separation::of(m.framed.body(), tol).map_err(NodeErrorKind::Boolean)?;
        pairs.extend(
            certificate
                .apart(&maps)
                .into_iter()
                .map(|(a, b)| (images[a], images[b])),
        );
    }
    Ok(topo::Apart::of(pairs))
}

/// The fold of a union's or an intersect's members ([`wire_combine`]).
fn fold_members<T: Decide + geom_core::Bounds + topo::AtRestPolicy>(
    verb: &crate::verbs::boolean::PairVerb<T>,
    op: BooleanOp,
    id: RecipeNodeId,
    own_read: crate::VarId,
    members: &[MemberRead<T>],
    boolean_sweep: topo::SweepStrategy,
    tol: Tol,
) -> OpResult<T> {
    let empty = || {
        Ok(OpOut::plain(
            ValuePayload::Boolean(BooleanValue::Empty),
            names::empty(),
        ))
    };
    // It computes in its first member's frame that is its space's,
    // else its first's ([`super::Framed::computing_among`]), reading
    // every other member into it through `relative`.
    let (members, apart) = match members {
        [] => return empty(),
        [one] => return lone_member(id, &one.finished_in(&one.framed, tol)?),
        [first, ..] => {
            let computing = super::Framed::computing_among(members.iter().map(|m| &m.framed))
                .unwrap_or(&first.framed)
                .clone();
            let apart = match op {
                BooleanOp::Union => images_apart(members, &computing, tol)?,
                BooleanOp::Intersect | BooleanOp::Subtract => topo::Apart::default(),
            };
            let finished = members
                .iter()
                .map(|m| m.finished_in(&computing, tol))
                .collect::<Result<Vec<_>, NodeErrorKind>>()?;
            (finished, apart)
        }
    };
    let members = members.as_slice();
    let first = &members[0];
    let own: Vec<(crate::VarId, &NameTable)> =
        members.iter().map(|m| (m.read, m.table.as_ref())).collect();
    // Every member's read-keyed view, taken once for the fold. The FIRST
    // member enters read-keyed too, so every operand of every step is
    // already in this node's name space.
    let views = members
        .iter()
        .map(|m| {
            names::member_view(id, m.read, &m.table)
                .map(Arc::new)
                .map_err(NodeErrorKind::Naming)
        })
        .collect::<Result<Vec<_>, NodeErrorKind>>()?;
    let reads: Vec<crate::VarId> = members.iter().map(|m| m.read).collect();
    let inputs = members
        .iter()
        .zip(member_ranks(&reads))
        .map(|(m, rank)| topo::MemberInputs {
            records: m.carry.contacts.carried(),
            rank,
        })
        .collect();
    let nary = match op {
        BooleanOp::Union => topo::NaryOp::Union,
        BooleanOp::Intersect => topo::NaryOp::Intersect,
        BooleanOp::Subtract => unreachable!(
            "union {id:?} lowers a subtract, though a subtract is a pair (`wire_subtract`)"
        ),
    };
    let bodies: Vec<&topo::AtRestBody<T>> = members.iter().map(|m| m.body.as_ref()).collect();
    // The kernel runs every judgement and step first, and the run holds
    // what it ran before it stopped. Each is named here in the order it
    // ran, so the first defect in that order is the one announced: a
    // naming refusal of an earlier judgement or step comes before a
    // later one's refusal or empty result (`[failure.taxonomy]`).
    let topo::NaryRun {
        judgements,
        steps,
        pairwise,
        end,
    } = {
        let mut run = (verb.members)(nary, inputs);
        run.apart = apart;
        run.run_members(&bodies, boolean_sweep, tol)
    };
    super::counters::judgements(judgements.len());
    // Two members are one pair: its one fold step IS its judgement, and
    // its refusals the pair's own. Three or more are judged pairwise
    // first, and a refusal only a step raises names the member that
    // step folds in.
    // The judgements' rows, whole, each named in its two members' own
    // tables: the node's coincidences are its member pairs' (DM4), each
    // pair's in the order the kernel judged it. The faces each judgement
    // consumed are the node's face links (N2, [`names::UnionLinks`]).
    let mut links = names::UnionLinks::default();
    let mut judged: Vec<Vec<crate::coincide::NamedCoincidence>> =
        Vec::with_capacity(judgements.len());
    for (k, j) in judgements.iter().enumerate() {
        #[cfg(any(test, feature = "test-support"))]
        if let Some(fault) = crate::test_support::judgement_naming_fault(k) {
            return Err(NodeErrorKind::Naming(fault));
        }
        #[cfg(not(any(test, feature = "test-support")))]
        let _ = k;
        judged.push(
            crate::coincide::name_rows(
                &j.body.coincidences,
                &crate::coincide::RowInputs {
                    a: own[j.a],
                    b: Some(own[j.b]),
                    tool: None,
                },
            )
            .map_err(NodeErrorKind::Naming)?,
        );
        links
            .judged(
                j.a,
                j.b,
                &j.body.naming,
                &j.body.body,
                [members[j.a].read, members[j.b].read],
                bodies[j.b],
            )
            .map_err(NodeErrorKind::Naming)?;
    }
    // One row per statement: two pairs that state one thing (a member
    // read twice, beside a third) publish it once. `published[j][r]` is
    // where judgement `j`'s row `r` is published.
    let mut coincidences = Vec::new();
    let mut published: Vec<Vec<usize>> = Vec::with_capacity(judged.len());
    for (j, rows) in judgements.iter().zip(&judged) {
        let place = |m: usize| u32::try_from(m).unwrap_or(u32::MAX);
        let pair = [place(j.a.min(j.b)), place(j.a.max(j.b))];
        published.push(
            rows.iter()
                .map(|row| {
                    let row = crate::coincide::NamedCoincidence {
                        origin: crate::coincide::RowOrigin::Judged { pair },
                        ..row.clone()
                    };
                    judged_at(&mut coincidences, row)
                })
                .collect(),
        );
    }
    let judged_rows = coincidences.len();
    let mut last: Option<(topo::BooleanResultKind, Arc<topo::ContactRecords>)> = None;
    // What the end pass reads of every step: each face's member faces
    // and each step's discards.
    let mut fold = names::UnionFold::new(0, &first.body);
    // The accumulation's own records in the node's terms: each cites a
    // fold row (`fold_rows`) or a member's record, by its place in the
    // member list. Row for row, they are the records the kernel carried
    // into the next step as the accumulation's own (checked at each
    // step), so a step's `Carried { input: 0, record }` indexes them.
    let mut acc_records = Arc::new(first.carry.contacts.carried_from(0));
    let mut acc_body: &topo::AtRestBody<T> = &first.body;
    let mut acc_table = Arc::clone(&views[0]);
    // Every step's rows, in fold order: a record's `Decided(k)` cites
    // the `k`-th until the records are recited onto the published rows.
    let mut fold_rows: Vec<FoldRow> = Vec::new();
    // Each step's fragment groups, in fold order (`FragmentGroups::folded`).
    let mut step_groups = Vec::with_capacity(steps.len());
    for (step, stepped) in steps.iter().enumerate() {
        let joined = stepped.member;
        let member_body: &topo::AtRestBody<T> = &members[joined].body;
        let member_table = Arc::clone(&views[joined]);
        let out = &stepped.body;
        let (kind, naming) = (out.kind, &out.naming);
        let member = u32::try_from(joined).unwrap_or(u32::MAX);
        if !pairwise {
            // The pair's rows and links are its one step's, in the two
            // members' own tables and the order its records cite them,
            // as a pair verb records them.
            coincidences = crate::coincide::name_rows(
                &out.coincidences,
                &crate::coincide::RowInputs {
                    a: own[0],
                    b: Some(own[1]),
                    tool: None,
                },
            )
            .map_err(NodeErrorKind::Naming)?
            .into_iter()
            .map(|row| crate::coincide::NamedCoincidence {
                origin: crate::coincide::RowOrigin::Judged { pair: [0, 1] },
                ..row
            })
            .collect();
            links
                .judged(
                    0,
                    1,
                    naming,
                    &out.body,
                    [members[0].read, members[1].read],
                    member_body,
                )
                .map_err(NodeErrorKind::Naming)?;
        }
        // The step's records in the node's terms: every row the step
        // decided is a fold row, named in the node's own space and read
        // in member space; an accumulation record stands for what it
        // cited; the joining member's is that member's.
        let base = fold_rows.len();
        let decided = out.coincidences.len();
        if pairwise {
            let acc_view = names::collapse_table(id, &acc_table).map_err(NodeErrorKind::Naming)?;
            let inputs_named = crate::coincide::RowInputs {
                a: (own_read, &acc_view),
                b: Some((own_read, &member_table)),
                tool: None,
            };
            for (k, row) in out.coincidences.iter().enumerate() {
                // A row restating a given pair's decision is the judged
                // rows of its verdicts, as published.
                if let Some(restated) = stepped.restates(k) {
                    let mut at = Vec::new();
                    for r in restated {
                        let i = published[r.judgement][r.row];
                        if !at.contains(&i) {
                            at.push(i);
                        }
                    }
                    fold_rows.push(FoldRow::Restates(at));
                    continue;
                }
                let named = crate::coincide::name_rows(std::slice::from_ref(row), &inputs_named)
                    .map_err(NodeErrorKind::Naming)?;
                let [named] = <[_; 1]>::try_from(named)
                    .unwrap_or_else(|_| unreachable!("one row names one row"));
                #[cfg(any(test, feature = "test-support"))]
                let as_named = named.clone();
                let spelled =
                    in_member_space(named, own_read, member).map_err(NodeErrorKind::Naming)?;
                #[cfg(any(test, feature = "test-support"))]
                crate::test_support::record_fold_row(&as_named, &spelled);
                fold_rows.push(FoldRow::Decided(spelled));
            }
        }
        let carried_in = steps[..step].last().map_or_else(
            || first.carry.contacts.carried(),
            |s| s.body.contacts.carried(),
        );
        if acc_records.carried() != carried_in {
            unreachable!(
                "union {id:?}'s fold step {step} was carried records that are not the \
                 accumulation's the node holds, row for row, though the kernel carries each \
                 step's records into the next"
            );
        }
        let held = Arc::clone(&acc_records);
        let contacts = out
            .contacts
            .clone()
            .recited(|b| match b {
                topo::Backing::Decided(k) if pairwise => {
                    if (k as usize) < decided {
                        vec![topo::Backing::Decided(
                            u32::try_from(base + k as usize).unwrap_or(u32::MAX),
                        )]
                    } else {
                        unreachable!(
                            "union {id:?}'s fold step {step} cites its row {k}, though it \
                             decided {decided}: a step's records cite the rows it returned"
                        )
                    }
                }
                decided @ topo::Backing::Decided(_) => vec![decided],
                topo::Backing::Carried { input: 0, record } => held
                    .rows()
                    .get(record as usize)
                    .map(|c| c.cites.iter().collect())
                    .unwrap_or_else(|| {
                        unreachable!(
                            "union {id:?}'s fold step {step} carries accumulation record \
                             {record}, which the fold does not hold, though the step's operand \
                             A is that accumulation"
                        )
                    }),
                topo::Backing::Carried { record, .. } => vec![topo::Backing::Carried {
                    input: member,
                    record,
                }],
            })
            .unwrap_or_else(|k| {
                unreachable!(
                    "union {id:?}'s fold step {step}'s record {k} cites nothing once carried, \
                     though every citation maps to one or more"
                )
            });
        acc_records = Arc::new(contacts);
        last = Some((kind, Arc::clone(&acc_records)));
        // Minted under THIS node's id, its two sides keyed by the fold's
        // own reads, which tell an intermediate row from a member's own
        // name when the chain is collapsed.
        let emitted = (verb.emitter)(
            id,
            op,
            &out.body,
            naming,
            &names::OperandCtx {
                node: id,
                read: names::FOLD_A,
                table: &acc_table,
                body: acc_body,
            },
            &names::OperandCtx {
                node: id,
                read: names::FOLD_B,
                table: &member_table,
                body: member_body,
            },
            tol,
        )
        .map_err(NodeErrorKind::Naming)?;
        fold.step(joined, naming, &out.body, &stepped.lineage)
            .map_err(NodeErrorKind::Naming)?;
        acc_table = emitted.table;
        step_groups.push(emitted.groups);
        acc_body = &out.body;
    }
    // Every judgement and step the kernel ran is named; now how it ended.
    match end {
        topo::NaryEnd::Built => {}
        topo::NaryEnd::Empty => return empty(),
        topo::NaryEnd::Refused(topo::NaryRefusal::Step { member, error }) => {
            return Err(NodeErrorKind::UnionFoldStep {
                member: u32::try_from(member).unwrap_or(u32::MAX),
                read: members[member].read,
                refusal: Box::new(NodeErrorKind::Boolean(*error)),
            });
        }
        topo::NaryEnd::Refused(refusal) => return Err(nary_refused(refusal)),
    }
    // The members' own bodies and tables: where each member edge the
    // published table names pieces of is defined (`name_union`).
    let member_views: Vec<names::UnionMember<'_, T>> = members
        .iter()
        .map(|m| names::UnionMember {
            read: m.read,
            body: &m.body,
            table: &m.table,
        })
        .collect();
    let (table, published_groups) =
        names::name_union(id, acc_body, &acc_table, &member_views, &fold, &links, tol)
            .map_err(NodeErrorKind::Naming)?;
    let body = acc_body.clone().into_body();
    // The LAST step's records are the result's: every earlier step's
    // were carried into the step after it.
    let Some((kind, contacts)) = last else {
        unreachable!(
            "union {id:?}'s fold of {} members ran no step, though a fold of two or more runs \
             one per member after the first",
            members.len()
        )
    };
    // The node's rows are its evaluation's decisions (DM4): the judged
    // rows, then every fold step's, a fold row a judged row restates
    // being that row; the records are recited onto the rows they cite.
    let contacts = if pairwise {
        let at: Vec<Vec<usize>> = fold_rows
            .into_iter()
            .map(|row| match row {
                FoldRow::Restates(judged) => judged,
                // A fold row the judgement already states is that row: a
                // record citing it cites the judged decision and its
                // margin, not the step's (which can differ in its last
                // bits, 0.0 against 2.22e-16).
                FoldRow::Decided(row) => vec![
                    coincidences[..judged_rows]
                        .iter()
                        .position(|judged| restates(judged, &row))
                        .unwrap_or_else(|| {
                            coincidences.push(row);
                            coincidences.len() - 1
                        }),
                ],
            })
            .collect();
        Arc::new(
            (*contacts)
                .clone()
                .recited(|b| match b {
                    topo::Backing::Decided(k) => at
                        .get(k as usize)
                        .map(|rows| {
                            rows.iter()
                                .map(|&i| {
                                    topo::Backing::Decided(u32::try_from(i).unwrap_or(u32::MAX))
                                })
                                .collect()
                        })
                        .unwrap_or_else(|| {
                            unreachable!(
                                "union {id:?}'s records cite fold row {k}, though the fold \
                                 returned {} rows",
                                at.len()
                            )
                        }),
                    carried @ topo::Backing::Carried { .. } => vec![carried],
                })
                .unwrap_or_else(|k| {
                    unreachable!(
                        "union {id:?}'s record {k} cites nothing once recited onto the node's \
                         rows, though a held-out fold row is one no record cites"
                    )
                }),
        )
    } else {
        contacts
    };
    Ok(OpOut::plain(
        ValuePayload::Boolean(BooleanValue::Body {
            body: super::Framed::in_frame(Arc::new(body), first.framed.frame().clone()),
            kind,
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
    .holding(
        contacts,
        members
            .iter()
            .map(|m| crate::coincide::CitedInput::Read(m.read))
            .collect(),
    ))
}

/// **A union or an intersect of one body is that body** (DM4): no
/// boolean runs. Its entities are named by the member's read
/// ([`names::member_view`]), and the body itself by this node; it
/// carries what the member carries, as a copy carries it
/// ([`super::Carry::copied`]).
fn lone_member<T: Decide>(id: RecipeNodeId, one: &Member<T>) -> OpResult<T> {
    let table = names::name_lone_member(id, one.read, &one.table).map_err(NodeErrorKind::Naming)?;
    names::check_total(&table, one.framed.body(), 0).map_err(NodeErrorKind::Naming)?;
    Ok(
        OpOut::plain(ValuePayload::Body(one.framed.clone()), Arc::new(table)).carrying(
            one.carry.copied(
                super::carry::Stands::Still,
                &crate::coincide::CitedInput::Read(one.read),
            )?,
        ),
    )
}

/// **Each member's rank** for the kernel's n-ary door
/// ([`topo::MemberInputs::rank`]): the place of its read among the
/// distinct reads, ascending, so a judged pair's operand A is the member
/// whose read is the lesser (the earlier minted), and a read spelled
/// twice ties, the listed-first then A. One pair is one run of the pair
/// verb in every member order, and publishes one margin.
fn member_ranks(reads: &[crate::VarId]) -> Vec<u32> {
    let mut distinct = reads.to_vec();
    distinct.sort_unstable();
    distinct.dedup();
    reads
        .iter()
        .map(|r| {
            let at = distinct
                .binary_search(r)
                .unwrap_or_else(|_| unreachable!("every read is among the distinct reads"));
            u32::try_from(at).unwrap_or(u32::MAX)
        })
        .collect()
}

/// **An n-ary refusal other than a fold step's**, which names no member
/// ([`wire_combine`] names a step's): the pair boolean's or the box
/// door's own, or two verdicts that disagree.
fn nary_refused(refusal: topo::NaryRefusal) -> NodeErrorKind {
    match refusal {
        topo::NaryRefusal::Hull { error, .. }
        | topo::NaryRefusal::Judgement { error, .. }
        | topo::NaryRefusal::Step { error, .. } => NodeErrorKind::Boolean(*error),
        topo::NaryRefusal::VerdictsDisagree { .. } => {
            NodeErrorKind::Naming(names::NamingError::Emission {
                what: UNION_VERDICTS_DISAGREE,
            })
        }
        topo::NaryRefusal::Arity { members } => unreachable!(
            "the n-ary door refused {members} members, though the lowering runs it over two or \
             more"
        ),
    }
}

/// Two of a union's pairwise verdicts glue one accumulation face to one
/// member face under two relations
/// ([`topo::NaryRefusal::VerdictsDisagree`]).
const UNION_VERDICTS_DISAGREE: &str = "a union's pairwise judgements glue one accumulation face to one member face under two relations";

/// The role word a frame's authored +x direction is normalized under
/// ([`frame_axes`]).
pub(crate) const FRAME_X_ROLE: &str = "frame x axis";

/// The role word a frame's authored +y direction is normalized under,
/// after Gram-Schmidt ([`frame_axes`]).
pub(crate) const FRAME_Y_ROLE: &str = "frame y axis";

/// The role word a plane pose's normal is normalized under.
pub(crate) const PLANE_NORMAL_ROLE: &str = "plane normal";

/// The role word an axis pose's direction is normalized under — one
/// word on both roads, though the funnel name differs by road (see
/// [`unit()`]).
pub(crate) const AXIS_DIRECTION_ROLE: &str = "axis direction";

/// **A placement** (D10 `[placement.placement]`): the copy of its
/// source's one body ([`copy_of`]), or of a family read whole, each
/// member placed by the one map and carrying its own carry.
fn wire_place<T: Decide + topo::AtRestPolicy>(
    id: RecipeNodeId,
    (of, read, results): (RecipeNodeId, crate::VarId, &Results<T>),
    map: Option<&RigidMotion<T>>,
) -> OpResult<T> {
    let value = value_of(results, of)?;
    let cites = crate::coincide::CitedInput::Read(read);
    let body = match &value.payload {
        ValuePayload::Boolean(BooleanValue::Empty) => {
            return Ok(OpOut::plain(
                ValuePayload::Boolean(BooleanValue::Empty),
                names::empty(),
            ));
        }
        ValuePayload::Boolean(BooleanValue::Body { body, .. }) | ValuePayload::Body(body) => body,
        ValuePayload::Instances(bodies) => {
            let bodies = place_each(bodies, map)?;
            let placed: Vec<&Body<T>> = bodies.iter().map(super::Framed::body).collect();
            let table =
                names::name_copy(id, &read_table(value), &placed).map_err(NodeErrorKind::Naming)?;
            // Each member's records cite the placement's one read.
            return Ok(
                OpOut::plain(ValuePayload::Instances(bodies), table).carrying(
                    value
                        .carry
                        .copied(super::carry::Stands::placed(map), &cites)?,
                ),
            );
        }
        _ => return Err(wrong_operand(value, of, super::phrase::BODY_OR_INSTANCES)),
    };
    let table = read_table(value)
        .project(0)
        .map_err(|dup| NodeErrorKind::Naming(names::NamingError::from(dup)))?;
    // A placement moves its source and re-mints no key, so the census
    // its source's door kept keys this copy too.
    copy_of(id, (body, &table), &value.carry, map, &cites)
}

/// **The assemble** (`⊔`, D10 `[operations.operators]`): each member
/// grafted beside the others as a piece of one body, in member order,
/// none decided against another. Its names are each member's keyed by
/// the member's read ([`names::name_assemble`]); its records are each
/// member's, carried through the graft and citing that member by its
/// read; its pieces are its members, each with its own body, names and
/// pieces. A member that is a family is a piece of several bodies,
/// each member's records keyed in its own arena.
fn wire_assemble<T: Decide + topo::AtRestPolicy>(
    id: RecipeNodeId,
    members: &[(RecipeNodeId, crate::VarId, &Results<T>)],
    tol: Tol,
) -> OpResult<T> {
    let mut fused = Body::new();
    // The frame it computes in: its first operand's that is in its
    // space's coordinates, else its first's, as the author lists them
    // ([`super::Framed::computing_among`]); every other body is read
    // into it through `relative`.
    let operands: Vec<&super::Framed<T>> = members
        .iter()
        .filter_map(|&(of, _, results)| value_of(results, of).ok())
        .flat_map(|value| match &value.whole().payload {
            ValuePayload::Boolean(BooleanValue::Body { body, .. }) | ValuePayload::Body(body) => {
                vec![body]
            }
            ValuePayload::Instances(bodies) => bodies.iter().collect(),
            _ => Vec::new(),
        })
        .collect();
    let mut computing: Option<super::Framed<T>> =
        super::Framed::computing_among(operands.iter().copied()).cloned();
    let mut named: Vec<(crate::VarId, Arc<NameTable>, Vec<topo::GraftKeys>)> =
        Vec::with_capacity(members.len());
    let mut pieces: Vec<super::Piece<T>> = Vec::with_capacity(members.len());
    let mut contacts = topo::ContactRecords::default();
    let mut citing: Vec<crate::coincide::CitedInput> = Vec::with_capacity(members.len());
    let mut parts = 0;
    for &(of, read, results) in members {
        // Every seat of an assemble takes a family whole.
        let value = value_of(results, of)?.whole();
        let (bodies, family) = match &value.payload {
            ValuePayload::Boolean(BooleanValue::Empty) => {
                return Err(NodeErrorKind::EmptyOperand { input: of });
            }
            ValuePayload::Boolean(BooleanValue::Body { body, .. }) | ValuePayload::Body(body) => {
                (vec![body.clone()], false)
            }
            ValuePayload::Instances(bodies) => (bodies.clone(), true),
            _ => return Err(wrong_operand(value, of, super::phrase::BODY_OR_INSTANCES)),
        };
        let table = read_table(value);
        let mut bridges = Vec::with_capacity(bodies.len());
        for body in &bodies {
            let first = computing.get_or_insert_with(|| body.clone());
            let read = first.read_in(body, tol)?;
            let keys = topo::graft_disjoint_all_keyed(&mut fused, read.as_ref())
                .map_err(NodeErrorKind::Boolean)?;
            bridges.push(keys);
        }
        // A member's own records ride the piece it is, re-keyed through
        // its graft (each member's of a family), citing the member by its
        // read.
        let input = u32::try_from(citing.len()).unwrap_or(u32::MAX);
        for (i, keys) in bridges.iter().enumerate() {
            crate::product::carry_contacts(
                &fused,
                &mut contacts,
                &value.carry.member(i).contacts.carried_from(input),
                keys,
            )
            .map_err(|what| NodeErrorKind::Naming(names::NamingError::Emission { what }))?;
        }
        citing.push(crate::coincide::CitedInput::Read(read));
        parts += value.carry.parts;
        pieces.push(super::Piece {
            read,
            bodies,
            family,
            table: Arc::clone(&table),
            carry: value.carry.clone(),
        });
        named.push((read, table, bridges));
    }
    let table = names::name_assemble(id, &named, &fused).map_err(NodeErrorKind::Naming)?;
    let frame = computing.map_or(super::Frame::Own, |first| first.frame().clone());
    let pieces = Arc::new(super::Pieces { by: id, pieces });
    Ok(OpOut::plain(
        ValuePayload::Body(super::Framed::in_frame(Arc::new(fused), frame)),
        table,
    )
    .carrying(super::Carry {
        contacts: Arc::new(contacts),
        cited_inputs: citing.into(),
        parts,
        // The census at this door, over its pieces: run where a reader
        // first asks for its verdict ([`crate::product::Census`]).
        census: Some(Arc::new(crate::product::Census::new(
            id,
            Arc::clone(&pieces),
        ))),
        censused: true,
        pieces: Some(pieces),
        findings: Arc::default(),
        members: None,
    }))
}

mod distribute;
pub(crate) mod pose;
pub(crate) mod trace;

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

/// A declared structural (Count) slot read as a `usize`, refused typed
/// when outside `least..=most`: the slot's own domain, which the
/// caller states (pass `usize::MAX` for a domain with no upper end).
/// The refusal carries that domain as the slot admits it, clamped to
/// what an `i64` holds, so `admits` always names values that evaluate.
fn need_count(
    vals: &SlotValues<impl Decide>,
    slot: SlotId,
    least: usize,
    most: usize,
) -> Result<usize, NodeErrorKind> {
    let n = slot_count(vals, slot);
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
    // The v degree admits `1 ..= sections − 1` (`Node::Loft`).
    if profiles.len() < 2 {
        unreachable!(
            "loft {id} reached evaluation through {} section(s): every edit door that writes a \
             loft and the load door refuse fewer than two (`edit::loft_structure_fault`)",
            profiles.len()
        );
    }
    let v_degree = need_count(vals, SlotId::VDegree, 1, profiles.len() - 1)?;
    // The first section is the loft's own plane; each after it stands
    // where its station carries that plane.
    let mut sections: Vec<profile::ValidatedProfile<T>> = Vec::with_capacity(values.len());
    for (k, value) in values.iter().enumerate() {
        if k > 0 && value.frame.is_some() {
            return Err(operand_refusal(
                profiles[k],
                super::phrase::FREE_PROFILE,
                "drawn profile",
            ));
        }
        let plane = match k.checked_sub(1) {
            None => profile::SketchPlane::xy(),
            Some(after) => station_plane(k, stations[after], vals)?,
        };
        sections.push(value.validated.clone().on_plane(plane));
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
        ValuePayload::Body(values[0].framed(Arc::new(built.body))),
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
                let by = need_angle(vals, SlotId::StationSpin { section, step })?;
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
/// pose, or with a bad Count, is a recipe error and must read as one.
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
mod union_row_tests {
    #![allow(clippy::unwrap_used, clippy::expect_used, clippy::panic)]

    use super::{in_member_space, judged_at, member_ranks};
    use crate::coincide::{NamedCell, NamedCoincidence, RowOrigin};
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
            cells: vec![a, b],
            relation: topo::Relation::OnCarrier,
            site: topo::DecisionSite::VertexFusion,
            margin,
            discharge: topo::Discharge::of(margin),
            origin: RowOrigin::Node,
        }
    }

    /// Member `m`'s entity of `kind`, a cap role at `end`.
    fn member(m: u64, kind: EntityKind, end: CapEnd) -> StableName {
        name(kind, RecipeNodeId::new(0, m), vec![RoleSeg::Cap(end)])
    }

    /// **A fold row is the member cells that meet** (DM4, "A row names
    /// member cells"): member 1's edge `e` crossed by member 2's two
    /// caps, each crossing decided on member 3's edge `c`, is two
    /// meetings of three member cells, one per cap; a crossing of a seam
    /// the fold minted is the seam's two faces and the crossing face; a
    /// member vertex on a member edge keeps its relation and the order
    /// the step read it in.
    #[test]
    fn a_fold_row_is_the_member_cells_that_meet() {
        let union = RecipeNodeId::new(0, 9);
        let own = VarId::new(1, 9);
        let in_own = |n: StableName| NamedCell::Entity {
            input: own,
            name: n,
        };
        let cell = |m: u64, n: StableName| NamedCell::Entity {
            input: VarId::new(1, m),
            name: n,
        };
        let e = member(1, EntityKind::Edge, CapEnd::Start);
        let c = member(3, EntityKind::Edge, CapEnd::Start);
        let crossing = |edge: StableName, face: StableName| {
            name(
                EntityKind::Vertex,
                union,
                vec![RoleSeg::Crossing {
                    edge: edge.into(),
                    face: face.into(),
                    sense: Sense::Leaves,
                }],
            )
        };
        let sorted = |mut v: Vec<NamedCell>| {
            v.sort();
            v
        };
        for end in [CapEnd::Start, CapEnd::End] {
            let cap = member(2, EntityKind::Face, end);
            let r = in_member_space(
                row(
                    in_own(crossing(
                        from(union, 1, e.clone()),
                        from(union, 2, cap.clone()),
                    )),
                    in_own(from(union, 3, c.clone())),
                ),
                own,
                2,
            )
            .unwrap();
            assert_eq!(
                r.relation,
                topo::Relation::Meet,
                "a crossing is no member point"
            );
            assert_eq!(
                r.cells,
                sorted(vec![cell(1, e.clone()), cell(2, cap), cell(3, c.clone())]),
                "the crossing's edge and face, and the cell it was decided on"
            );
            assert_eq!(r.origin, RowOrigin::Fold { step: 2 });
        }
        let (fa, fb, top) = (
            member(1, EntityKind::Face, CapEnd::Start),
            member(2, EntityKind::Face, CapEnd::Start),
            member(3, EntityKind::Face, CapEnd::End),
        );
        let seam = name(
            EntityKind::Edge,
            union,
            vec![RoleSeg::Seam {
                a: from(union, 1, fa.clone()).into(),
                b: from(union, 2, fb.clone()).into(),
            }],
        );
        let v = member(4, EntityKind::Vertex, CapEnd::Start);
        let r = in_member_space(
            row(
                in_own(from(union, 4, v.clone())),
                in_own(crossing(seam, from(union, 3, top.clone()))),
            ),
            own,
            3,
        )
        .unwrap();
        assert_eq!(
            (r.relation, r.cells),
            (
                topo::Relation::Meet,
                sorted(vec![
                    cell(1, fa),
                    cell(2, fb),
                    cell(3, top),
                    cell(4, v.clone())
                ])
            ),
            "a member vertex on a seam's crossing meets the seam's faces and the face"
        );
        let r = in_member_space(
            row(
                in_own(from(union, 4, v.clone())),
                in_own(from(union, 1, e.clone())),
            ),
            own,
            1,
        )
        .unwrap();
        assert_eq!(
            (r.relation, r.cells),
            (topo::Relation::OnCarrier, vec![cell(4, v), cell(1, e)]),
            "a member vertex on a member edge is the two-cell row, as read"
        );
        // A point decided on the very cell it is states nothing.
        let on_itself = NamedCoincidence {
            relation: topo::Relation::Meet,
            ..row(
                in_own(from(union, 1, member(1, EntityKind::Edge, CapEnd::Start))),
                in_own(from(union, 1, member(1, EntityKind::Edge, CapEnd::Start))),
            )
        };
        assert!(
            in_member_space(on_itself, own, 3).is_err(),
            "a meeting of one cell is refused"
        );
    }

    /// **A carrier pair's row is never a fold step's**: an `OnCarrier`
    /// between a member face and a face the fold merged from two is one
    /// no step decides, since a step runs with the pairwise verdicts
    /// given, so one that would be rewritten is a kernel bug.
    #[test]
    #[should_panic(expected = "decides no carrier pair itself")]
    fn a_fold_row_on_a_face_made_of_two_panics() {
        let union = RecipeNodeId::new(0, 9);
        let own = VarId::new(1, 9);
        let in_own = |n: StableName| NamedCell::Entity {
            input: own,
            name: n,
        };
        let merged = name(
            EntityKind::Face,
            union,
            vec![RoleSeg::Merged(vec![
                from(union, 1, member(1, EntityKind::Face, CapEnd::Start)),
                from(union, 2, member(2, EntityKind::Face, CapEnd::End)),
            ])],
        );
        let pair = row(
            in_own(from(union, 3, member(3, EntityKind::Face, CapEnd::End))),
            in_own(merged),
        );
        let _ = in_member_space(pair, own, 3);
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
        let at = |margin: f64, pair: [u32; 2]| {
            let margin = geom_core::MarginDiag::value(margin);
            NamedCoincidence {
                cells: vec![cell(1, EntityKind::Edge), cell(2, EntityKind::Face)],
                relation: topo::Relation::SurfaceTangent,
                site: topo::DecisionSite::TangentWitness,
                margin,
                discharge: topo::Discharge::of(margin),
                origin: RowOrigin::Judged { pair },
            }
        };
        let publish = |judged: &[NamedCoincidence]| {
            let mut rows = Vec::new();
            judged
                .iter()
                .map(|r| judged_at(&mut rows, r.clone()))
                .collect::<Vec<_>>()
        };
        assert_eq!(
            publish(&[at(0.0, [0, 1]), at(0.0, [0, 1])]),
            [0, 1],
            "one pair's two touches, both published"
        );
        assert_eq!(
            publish(&[at(1e-9, [0, 2]), at(1e-9, [1, 2])]),
            [0, 0],
            "a member read twice states one row"
        );
        assert_eq!(
            publish(&[at(0.0, [0, 2]), at(1e-9, [1, 2])]),
            [0, 1],
            "two pairs at two margins are two rows"
        );
    }

    /// **A member's rank is its read's place among the distinct reads**:
    /// the lesser read the lesser rank, whatever the list order, and a
    /// read spelled twice one rank, which the kernel breaks by list
    /// order (`topo::nary_boolean`).
    #[test]
    fn a_members_rank_is_its_reads_place() {
        let reads = [
            VarId::new(7, 7),
            VarId::new(3, 3),
            VarId::new(5, 5),
            VarId::new(3, 3),
        ];
        assert_eq!(member_ranks(&reads), [2, 0, 1, 0]);
    }
}
