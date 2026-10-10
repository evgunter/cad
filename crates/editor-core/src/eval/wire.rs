//! **Four jobs**: node-to-kernel wiring, the mid-evaluation name
//! ladder, the declaration routing that has no kernel op to wire to,
//! and the placement-rule arithmetic two modules share.
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
//! order [`ladder`] gives, and its entity's kind checked through
//! [`super::entity_door`]. A declared pair's names are the ladder's only
//! other reader.
//!
//! **The declaration routing.** A union's declared face pairs are SITED
//! at its members and consumed by a fold of pairwise booleans, so
//! something decides which fold step each pair belongs to and which
//! side of it each site takes: [`side_by_operand`] for the pair
//! boolean, [`route_declarations`] and [`look_through_fold`] for the
//! union, and the refusal menu beneath them. The kernel takes a
//! [`BooleanDeclarations`] already resolved to operands and entity
//! keys; every decision about where an authored pair resolves is made
//! here.
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

use geom_brep::OutwardNormal;
use geom_core::{
    Affine3, Band, Decide, Mat3, OrthoAxis, OrthoFrame, Point2, Point3, Sign, Tol, UnitVec3,
    UnitVec3Error, Vec2, Vec3,
};
use sweep::{Revolution, RevolveAxis};
use topo::splitting::SplitPart;
use topo::transform::transform_rigid;
use topo::{
    Body, BooleanDeclarations, CarriedContacts, CarriedVf, CarriedVv, DATUM_UNIT_NORM,
    FacePairDeclaration, VfContact, VvContact,
};

use super::anchor::{self, ProfilePre, ProfileValue};
use super::slots::{self, SlotValues};
use super::{BooleanValue, DatumValue, NodeErrorKind, NodeResult, SplitSide, ValuePayload};
use crate::names::{self, NameTable, SplitHalf};
use crate::node::{
    Axis3, BooleanOp, Datum, DeclaredPair, Node, PartSelect, PatternKind, RecipeNodeId, SitedRef,
    SlotId,
};
use crate::program::ProfileProgram;
use crate::resolve::FoldConsumption;

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
    pub names: Arc<NameTable>,
    /// The fragment groups the op's emitter formed
    /// (`names::FragmentGroups`); empty for an op that forms none.
    pub groups: Arc<names::FragmentGroups>,
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
}

impl<T: Decide> OpOut<T> {
    /// An op that declares no contact — every op but instantiate.
    fn plain(payload: ValuePayload<T>, names: Arc<NameTable>) -> Self {
        Self {
            payload,
            names,
            groups: Arc::default(),
            contacts: Arc::new(topo::ContactRecords::default()),
            carried: Arc::new(crate::assembly::CarriedDeclarations::default()),
            parts: 1,
            coincidences: Arc::new([]),
            cited_inputs: Arc::new([]),
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
    /// This output with the fragment groups its emitter formed.
    fn grouped(self, groups: Arc<names::FragmentGroups>) -> Self {
        Self { groups, ..self }
    }
}

type OpResult<T> = Result<OpOut<T>, NodeErrorKind>;
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
    /// ([`mint_frame_placement`]), a profile or section's program
    /// resolution ([`section_of`]), the placement the pinned lift
    /// embeds — so it is what a content key owes
    /// ([`super::tag::slot`]). Nothing under evaluation builds a second
    /// one.
    pub nominal: &'a crate::expr::VarEnv<f64>,
    /// The E4 seed this evaluation carries, by its variable (`None` on
    /// the build path). Consulted by the one place the lift cannot
    /// reach: a C6/D9-pinned section refuses a seed it would otherwise
    /// embed as a constant ([`section_of`]).
    pub seed: Option<crate::var::VarId>,
}

impl<T> Clone for LaneEnv<'_, T> {
    fn clone(&self) -> Self {
        *self
    }
}

impl<T> Copy for LaneEnv<'_, T> {}

/// The evaluation-wide context an op may need beyond its own inputs:
/// the boolean candidate-generation switch, the document seam, the
/// mate solve, and the lane environment — everything ambient to the
/// run, as opposed to the op's inputs and slots.
pub(crate) struct OpEnv<'a, T: Decide> {
    pub boolean_sweep: topo::SweepStrategy,
    pub parts: &'a super::parts::PartCache<'a, T>,
    /// The document's mate solve, run once per evaluation (ASM-R2a
    /// D-5) at its scalar: every instance's pose relative to its group
    /// root, and every mate's role.
    pub poses: &'a crate::mate::SolvedPoses<T>,
    /// The nodes whose inputs lie in two spaces, each naming the
    /// unplaced group it would compare (`mate::solve::spaces_of`).
    pub across: &'a std::collections::BTreeMap<RecipeNodeId, (RecipeNodeId, crate::mate::Unplaced)>,
    /// Where profile geometry comes from, and over which environment.
    pub lane: LaneEnv<'a, T>,
}

/// Runs one node's op against its (already Ok) inputs and evaluated
/// slots, emitting the node's name table alongside the payload.
/// `profile_pre` is the profile node's f64 precompute (present exactly
/// for `Node::Profile` — computed in `eval_node`'s resolution stage,
/// inside the node's verdict frame and ahead of this op).
#[allow(clippy::too_many_arguments)] // the 8th is the run-tolerance witness, not a duty of its own
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
        + super::SectionScalar
        + crate::mate::SolveScalar,
{
    use crate::OperandSlot as O;
    let unprojected = results;
    // A boolean's or a union's operands are projected one by one: two
    // ports of one split are two bodies, which one map keyed by the
    // split cannot hold. Every other node reads at most one body.
    let per_operand = matches!(node, Node::Boolean { .. } | Node::Union { .. });
    let projected = if per_operand {
        None
    } else {
        split_ports_projected(node, &reads_of(node, doc), doc, results)?
    };
    let results = projected.as_ref().unwrap_or(results);
    let project_each = |vars: &[crate::VarId]| {
        vars.iter()
            .map(|&var| split_ports_projected(node, &[var], doc, unprojected))
            .collect::<Result<Vec<_>, _>>()
    };
    // An operand reads an output; the op reads the operation's value.
    // Every read resolves here: an unresolved one refused the node
    // before its op was reached (`eval::read_at`).
    let at = |slot, var| super::read_at(doc, slot, var);
    let all = |slot: fn(u32) -> O, list: &[crate::VarId]| {
        list.iter()
            .enumerate()
            .map(|(i, &var)| at(slot(u32::try_from(i).unwrap_or(u32::MAX)), var))
            .collect::<Result<Vec<_>, _>>()
    };
    match node {
        Node::Datum(d) => Ok(OpOut::plain(
            wire_datum(d, doc, results, vals, tol)?,
            names::empty(),
        )),
        Node::Profile(program) => {
            let payload = wire_profile(
                program,
                at(O::Frame, program.frame)?,
                results,
                profile_pre,
                env.lane,
                tol,
            )?;
            let rows = match &payload {
                ValuePayload::Profile(value) => {
                    crate::coincide::name_junctions(id, &value.validated, &value.pieces)
                        .map_err(NodeErrorKind::Naming)?
                }
                _ => Vec::new(),
            };
            Ok(OpOut::plain(payload, names::empty()).with_coincidences(rows))
        }
        Node::Extrude { profile, side, .. } => {
            wire_extrude(id, at(O::Profile, *profile)?, *side, results, vals, tol)
        }
        Node::Revolve { profile, axis, .. } => {
            let (profile, axis) = (at(O::Profile, *profile)?, at(O::Axis, *axis)?);
            wire_revolve(id, profile, axis, doc, results, vals, tol)
        }
        Node::Loft { profiles, .. } => wire_loft(
            id,
            &all(O::Section, profiles)?,
            doc,
            results,
            vals,
            env.lane,
            tol,
        ),
        Node::Sweep { profile, path, .. } => {
            let (profile, path) = (at(O::Profile, *profile)?, at(O::Path, *path)?);
            wire_sweep(profile, path, doc, results, vals, env.lane, tol)
        }
        Node::Tube { frame, window, .. } => {
            wire_tube(id, at(O::Frame, *frame)?, window, results, vals, tol)
        }
        Node::HollowTube { frame, window, .. } => {
            wire_hollow_tube(id, at(O::Frame, *frame)?, window, results, vals, tol)
        }
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
            at(O::Target, *target)?,
            at(O::Tool, *tool)?,
            results,
            tol,
        ),
        Node::Boolean { op, a, b, declare } => {
            let each = project_each(&[*a, *b])?;
            let [ra, rb] = [&each[0], &each[1]].map(|p| p.as_ref().unwrap_or(unprojected));
            wire_boolean(
                &crate::verbs::boolean::boolean(),
                id,
                *op,
                (at(O::A, *a)?, ra),
                (at(O::B, *b)?, rb),
                [*a, *b],
                declare,
                doc,
                env.boolean_sweep,
                tol,
            )
        }
        Node::Union { members, declare } => {
            let projected = project_each(members)?;
            let each: Vec<&Results<T>> = projected
                .iter()
                .map(|p| p.as_ref().unwrap_or(unprojected))
                .collect();
            wire_union(
                &crate::verbs::boolean::boolean(),
                id,
                &all(O::Member, members)?,
                declare,
                doc,
                &each,
                env.boolean_sweep,
                tol,
            )
        }
        Node::Transform { input, placement } => {
            wire_transform(at(O::Input, *input)?, placement, results, vals, tol)
        }
        Node::PlaceInWorld { body, pose } => {
            let at = at(O::Body, *body)?;
            let port = doc.defined_by(*body).map_or(0, |(_, port)| port);
            wire_place_in_world(id, (at, *body), port, pose, results, vals, tol)
        }
        Node::Pattern { input, kind, .. } => {
            let input = at(O::Input, *input)?;
            wire_pattern(id, input, kind, doc, &written(doc, id), results, vals, tol)
        }
        // No `id`: the projection mints no description and no name, so
        // nothing it produces is stamped or keyed by this node.
        Node::Part { of, select } => wire_part(at(O::Of, *of)?, select, results, vals),
        // The rule gate, FIRST, through the node's own door (the one
        // `apply` reads): a bad placement list refuses with its own
        // name rather than downstream as a separation or rigidity
        // refusal. Hand-built-document backstop.
        Node::PlacedUnion { input, kind, .. } => match node.placement_rule_fault(tol) {
            Some(fault) => Err(NodeErrorKind::PlacementRule(fault)),
            None => {
                let input = at(O::Input, *input)?;
                wire_placed_union(id, input, kind, doc, &written(doc, id), results, vals, tol)
            }
        },
        Node::Measure { primitive } => wire_measure(primitive, doc, results, tol),
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
            doc_ref, interface, ..
        } => {
            if let Some(fault) = env.poses.fault(id) {
                return Err(NodeErrorKind::Mate(Box::new(fault.clone())));
            }
            let frame = instance_frame(doc, id, env.poses, env.lane.params, tol)?
                .unwrap_or(crate::placement::Motion::Identity);
            let pose = env
                .poses
                .pose(id)
                .unwrap_or_else(crate::mate::solve::Pose::identity);
            let map = pose.compose_around(frame).non_identity();
            wire_instantiate_part(id, doc_ref, interface, map, env, tol)
        }
        // A gauge DENOTES NO BODY (A11 (2)): its slots evaluated above,
        // and the instances on it read where it sits.
        Node::Gauge { .. } => Ok(OpOut::plain(ValuePayload::Gauge, names::empty())),
        // A mate DENOTES NO BODY (A12): it evaluates to its role in
        // the solve. A refusing mate fails here rather than at the
        // instance it would have placed, so the message names the mate.
        Node::Mate { .. } => match env.poses.fault(id) {
            Some(fault) => Err(NodeErrorKind::Mate(Box::new(fault.clone()))),
            None => Ok(OpOut::plain(
                ValuePayload::Mate(env.poses.role(id).unwrap_or_else(|| {
                    unreachable!(
                        "the solve of this document records a role for every mate in its \
                         order, or faults it, yet mate {id:?} has neither"
                    )
                })),
                names::empty(),
            )),
        },
    }
}

/// **The frame an instance's group sits in, in this lane** (A11 (5)):
/// its gauge chain composed with its group root's offset, evaluated at
/// `env`; the identity in an unplaced group's own space. `None` for a
/// node that is not a posed instance — the solve's own fault is the
/// op's to raise.
///
/// # Errors
///
/// [`NodeErrorKind::PlacementRefused`] carrying the refusal of the
/// gauge or root offset that did not evaluate.
pub(crate) fn instance_frame<T: Decide>(
    doc: &crate::doc::Doc<ProfileProgram>,
    id: RecipeNodeId,
    poses: &crate::mate::SolvedPoses<T>,
    env: &crate::expr::VarEnv<T>,
    tol: Tol,
) -> Result<Option<crate::placement::Motion<T>>, NodeErrorKind> {
    if poses.fault(id).is_some() {
        return Ok(None);
    }
    let (Some(space), Some(root)) = (poses.space(id), poses.root(id)) else {
        return Ok(None);
    };
    if let crate::mate::Space::Own { .. } = space {
        return Ok(Some(crate::placement::Motion::Identity));
    }
    let band = geom_core::predicate::Band::linear(tol)
        .map_err(|error| NodeErrorKind::Mate(Box::new(crate::mate::MateFault::Band { error })))?;
    crate::mate::solve::group_frame(doc, root, env, band)
        .map(Some)
        .map_err(|(node, error)| NodeErrorKind::PlacementRefused { node, error })
}

/// ASM-2A D-3: materialize an instance through the shipped doors.
///
/// Resolve (memoized per reference), take the referenced document's A10
/// PRODUCT, place it with the kernel's own `transform_rigid`, and hand
/// back a body-denoting value. The placed body is an ordinary `Body` —
/// one solid or N, under ONE rigid map — so the root gather and the
/// export door consume it with no new arms.
fn wire_instantiate_part<T>(
    id: RecipeNodeId,
    doc_ref: &crate::ident::DocRef,
    interface: &crate::node::InterfaceRecord,
    map: Option<geom_core::Affine3<T>>,
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
        + super::SectionScalar
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
        unplaced: part
            .unplaced
            .iter()
            .map(|row| {
                let (route, crate::assembly::UnplacedGroup { group, cause }, held) =
                    row.through(id);
                crate::assembly::CarriedUnplaced {
                    route,
                    group,
                    cause,
                    held,
                }
            })
            .collect(),
    };
    Ok(OpOut {
        payload: ValuePayload::Body(Arc::new(placed)),
        names: table,
        groups: Arc::default(),
        contacts: Arc::new(part.contacts.carried_from(0)),
        carried: Arc::new(carried),
        parts: part.parts,
        // The part's own rows are its document's, read there.
        coincidences: Arc::new([]),
        cited_inputs: Arc::new([crate::coincide::CitedInput::Part(*doc_ref)]),
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
    map: Option<&Affine3<T>>,
    tol: Tol,
) -> Result<Body<T>, NodeErrorKind> {
    match map {
        None => Ok(body.clone()),
        Some(map) => transform_rigid(body, map, tol).map_err(NodeErrorKind::Transform),
    }
}

/// Every body of `bodies` placed by node `by`'s map number `j`, in
/// order: one placement of a pattern's master.
fn place_each<T: Decide + topo::AtRestPolicy>(
    bodies: &[Arc<Body<T>>],
    map: &Affine3<T>,
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

/// **The same question asked of a NODE** — the recipe-side door, for
/// the roads that never hold a value. `found:` comes from
/// [`super::node_value_kind`], which answers over node kinds in the
/// same words as [`operand`] does over payloads.
///
/// # The two halves answer about the same node, except across a placer
///
/// `read` tests the node the reference NAMES; `found` walks a
/// [`Node::Transform`] chain to its source. A `Transform` over a
/// `Profile` would therefore refuse with `found: "profile"`, a refusal
/// that states nothing. That is unreachable: `wire_transform` reads
/// through [`placeable_operand`], which refuses a profile and poisons
/// every dependent, so this door never sees such an id. A placer that
/// became shape-preserving over profiles would break this.
///
/// # Errors
///
/// [`NodeErrorKind::MissingInput`] for a reference that names no live
/// node; otherwise [`NodeErrorKind::WrongOperand`] naming the family
/// the node lands in. [`super::node_value_kind`]'s own refusal seat is
/// dropped because none can arrive: the reference is one of the
/// consumer's INPUT edges, already evaluated `Ok`, and a transform
/// there that could refuse would have poisoned the consumer first.
fn node_operand<'d, P, R>(
    doc: &'d crate::doc::Doc<P>,
    input: RecipeNodeId,
    expected: &'static str,
    read: impl FnOnce(&'d Node<P>) -> Option<R>,
) -> Result<R, NodeErrorKind> {
    let node = doc
        .node(input)
        .ok_or(NodeErrorKind::MissingInput { input })?;
    match read(node) {
        Some(r) => Ok(r),
        None => Err(operand_refusal(
            input,
            expected,
            super::node_value_kind(doc, input).map_err(|seated| seated.1)?,
        )),
    }
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
fn read_body<T: Decide>(
    results: &Results<T>,
    input: RecipeNodeId,
) -> Result<Arc<Body<T>>, NodeErrorKind> {
    let v = value_of(results, input)?;
    match placeable_operand(v, input) {
        Ok(Placeable::Body(b)) => Ok(b),
        Ok(Placeable::Instances(_)) | Err(NodeErrorKind::WrongOperand { .. }) => {
            Err(wrong_operand(v, input, super::family::BODY))
        }
        Err(other) => Err(other),
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
}

impl<T: Decide> Placeable<T> {
    /// The bodies, in value order — one for a body, the list for
    /// instances — so a placer that walks its master reads one shape.
    fn bodies(&self) -> &[Arc<Body<T>>] {
        match self {
            Self::Body(b) => core::slice::from_ref(b),
            Self::Instances(bs) => bs,
        }
    }

    /// **The shape-preserving map**: `f` over every body with its
    /// index in the value, yielding the SAME shape as a payload. A
    /// placer decides only what `f` does to one body.
    fn map(
        &self,
        mut f: impl FnMut(&Body<T>, usize) -> Result<Body<T>, NodeErrorKind>,
    ) -> Result<ValuePayload<T>, NodeErrorKind> {
        Ok(match self {
            Self::Body(b) => ValuePayload::Body(Arc::new(f(b, 0)?)),
            Self::Instances(bs) => ValuePayload::Instances(
                bs.iter()
                    .enumerate()
                    .map(|(i, b)| f(b, i).map(Arc::new))
                    .collect::<Result<_, _>>()?,
            ),
        })
    }
}

/// The operand of a placer, read off its evaluated value: one body
/// (a `Body` value or a boolean's non-empty result), or an `Instances`
/// value taken whole. Everything else refuses typed naming both
/// admitted shapes; an empty boolean is a typed absence. The same
/// rule over NODE kinds lives in [`super::node_value_kind`].
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
        _ => Err(wrong_operand(v, input, super::phrase::BODY_OR_INSTANCES)),
    }
}

/// The linear classification band (kernel-ambient tolerance).
fn band(tol: Tol) -> Result<Band, NodeErrorKind> {
    Band::linear(tol).map_err(NodeErrorKind::Band)
}

/// **The funnel site name** of this layer's direction-length
/// decision — a transform's rotation axis, a pattern's direction, and
/// the mate solve's re-derivation of both from the recipe. A roster
/// carrier (`docs/K-REPORT.md`, "The inventory method, restated"); one
/// constant, so the telemetry and an escalation report the same name.
pub(crate) const EVAL_DIRECTION_NORM: &str = "eval_direction_norm";

/// What one of this layer's decisions decides, in words
/// (`crate::decision::words` reads them). `None` for a predicate this
/// layer does not own.
pub(crate) fn decision_words(predicate: &str) -> Option<&'static str> {
    Some(match predicate {
        EVAL_DIRECTION_NORM => geom_core::DIRECTION_LENGTH_SUBJECT,
        "revolve_full_vs_partial" => "whether the revolve makes a full turn",
        PATTERN_SPACING => "whether the pattern spacing is positive",
        PATTERN_STEP => "whether the pattern step is zero",
        PATTERN_STEP_TURN => "how many whole turns the pattern step holds",
        _ => return None,
    })
}

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
/// [`UnitVec3::new`], because `DatumValue` has no unnormalized
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
    UnitVec3::new(v, EVAL_DIRECTION_NORM, band).map_err(|e| refusal(e, role, EVAL_DIRECTION_NORM))
}

/// **The kernel refusal in this layer's vocabulary** — the ONE map for
/// both funnel names, which is its parameter.
///
/// `role` names the vector the CALLER passed, which is what a user
/// reads; `predicate` names the funnel site the length was decided
/// under, which is what an escalation is comparable by.
fn refusal(e: UnitVec3Error, role: &'static str, predicate: &'static str) -> NodeErrorKind {
    match e {
        UnitVec3Error::NonFiniteLength => NodeErrorKind::NonFiniteDirection { role },
        UnitVec3Error::UnderflowedLength => NodeErrorKind::UnderflowedDirection { role },
        UnitVec3Error::Degenerate => NodeErrorKind::DegenerateDirection { role },
        UnitVec3Error::Escalated(source) => NodeErrorKind::Escalated { predicate, source },
    }
}

/// A Length-valued `[Expr; 3]` triple as a point.
fn point3<T: Decide>(vals: &SlotValues<T>, f: fn(Axis3) -> SlotId) -> Option<Point3<T>> {
    let v = slots::vec3(vals, f)?;
    Some(Point3::new(v.x, v.y, v.z))
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

fn need_point3<T: Decide>(
    vals: &SlotValues<T>,
    f: fn(Axis3) -> SlotId,
) -> Result<Point3<T>, NodeErrorKind> {
    point3(vals, f).ok_or(NodeErrorKind::MissingSlot { slot: f(Axis3::X) })
}

fn need_vec2<T: Decide>(
    vals: &SlotValues<T>,
    f: fn(Axis3) -> SlotId,
) -> Result<Vec2<T>, NodeErrorKind> {
    slots::vec2(vals, f).ok_or(NodeErrorKind::MissingSlot { slot: f(Axis3::X) })
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
/// Separate from its spelling because a refusal is not always raised
/// where it is made: [`FramePlacement::Unreadable`] CARRIES one to the
/// reader that needed it. Every road spells it through
/// [`DirectionRefusal::node_error`].
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
    /// spelling** from a carried or raised refusal to a
    /// [`NodeErrorKind`]: [`NodeErrorKind::FrameDirection`]'s `Display`
    /// and its class ([`NodeErrorKind::class`]) read through it, so the
    /// carried refusal says and is what the frame's own raise says and
    /// is. `pub` because the carried refusal is: a consumer holding one
    /// asks for the raise here rather than re-spelling it.
    pub fn node_error(self) -> NodeErrorKind {
        refusal(self.error, self.role, DATUM_UNIT_NORM)
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

/// **What reading an authored frame's slots produced** — the frame, or
/// the direction door's refusal of `u` or of `v`'s residual.
///
/// `NoDirection` is whichever of [`UnitVec3Error`]'s facts the door
/// reported, not only "these two are parallel": an underflowed length,
/// or an undecided margin inside the band, which `f64` reaches too.
///
/// The refusal is an ANSWER rather than an `Err` so the read is not a
/// `Result<Result<_, _>, _>`: slot faults are the node's and stay in
/// the `Err`, while this one is the READ's, and its two callers dispose
/// of it differently — [`wire_datum`] raises it, the `f64` placement
/// carries it ([`FramePlacement::Unreadable`]).
enum FrameRead<T: geom_core::Real> {
    /// The orthonormal frame.
    Frame(OrthoFrame<T>),
    /// The direction door refused `u`, or `v`'s residual.
    NoDirection(DirectionRefusal),
}

/// **An authored frame from its evaluated slots** — the one spelling
/// of the read, shared by the frame's own evaluation at the lane
/// scalar ([`wire_datum`]) and by its f64 placement
/// ([`mint_frame_placement`]). `u` and `v` are orthonormalized through
/// [`frame_axes`] with `u` kept: keeping `v` would silently rotate
/// every profile drawn on the frame when only `v` was edited.
///
/// # Errors
///
/// [`NodeErrorKind::MissingSlot`] for a slot the values do not carry
/// (unreachable while `Node::slots` and the wire agree). A refused
/// DIRECTION is [`FrameRead::NoDirection`], not an error.
fn frame_from_slots<T: Decide>(
    vals: &SlotValues<T>,
    band: Band,
) -> Result<FrameRead<T>, NodeErrorKind> {
    let origin = need_point3(vals, SlotId::Origin)?;
    Ok(
        match frame_axes(
            origin,
            need_vec3(vals, SlotId::U)?,
            need_vec3(vals, SlotId::V)?,
            band,
        ) {
            Ok(frame) => FrameRead::Frame(frame),
            Err(refusal) => FrameRead::NoDirection(refusal),
        },
    )
}

/// **Where a frame's profiles take their placement from** — the DM1c
/// fork, decided ONCE on the frame's own behalf and carried on its
/// value ([`super::NodeValue::placement`]), so a profile drawn on the
/// frame READS this instead of evaluating the frame's expressions
/// again.
#[derive(Debug, Clone, Copy)]
#[non_exhaustive]
pub enum FramePlacement {
    /// An AUTHORED frame ([`Datum::Frame`]): its nine expressions at
    /// the document's nominal, resolved and orthonormalized. Its
    /// profiles place with THIS.
    Authored(profile::SketchPlane<f64>),
    /// An AUTHORED frame whose nominal read REFUSED a direction (`u`,
    /// or `v`'s residual; see [`FrameRead::NoDirection`]).
    ///
    /// Carried rather than raised, because the frame's own evaluation
    /// succeeded at the lane scalar and every reader of that value
    /// ([`frame_plane_lane`], the mate solve, a measure, the viewer)
    /// reads it correctly. The refusal belongs to the reader that
    /// needed the nominal placement: [`profile_plane_f64`] raises it.
    Unreadable(DirectionRefusal),
    /// A DERIVED frame ([`Datum::FaceFrame`], DM1c): nothing to mint.
    /// Its profiles are placed at the lane under every lift
    /// ([`frame_plane_lane`]); their 2-D structure record is assembled
    /// in the conventional `SketchPlane::xy()` ([`prepare_profile`]),
    /// which no decision reads.
    Derived,
}

/// **A frame node's placement, minted once** — [`FramePlacement`] for
/// a frame node, `None` for every node that is not one.
///
/// The match on the RECIPE node's kind is closed, so a new [`Datum`]
/// variant is a compile error here rather than a node whose profiles
/// silently place at the lane. `None` means "not a frame" and nothing
/// else; [`profile_plane_f64`] turns it into a
/// [`NodeErrorKind::WrongOperand`].
///
/// # Why an authored frame is read at `f64`
///
/// The placement feeds STRUCTURE selection, which must be
/// lane-identical (C6): the same document has to select the same
/// structure at `f64` and at the interval scalar. The frame's LANDED
/// value answers a different question (what a reader sees, what a
/// measure measures), so an authored frame is read at two scalars —
/// `Pinned` places with THIS read, `Guided` with [`frame_plane_lane`]'s.
///
/// `nominal` is the node's slots already evaluated at
/// [`LaneEnv::nominal`], so this evaluates no expression.
///
/// # Errors
///
/// [`NodeErrorKind::MissingSlot`] for a slot the values do not carry,
/// and the band's own refusal: faults of the NODE, unreadable at every
/// environment, where a direction refusal is carried as
/// [`FramePlacement::Unreadable`] instead.
pub(crate) fn mint_frame_placement(
    node: &Node<ProfileProgram>,
    nominal: &SlotValues<f64>,
    tol: Tol,
) -> Result<Option<FramePlacement>, NodeErrorKind> {
    let Node::Datum(datum) = node else {
        return Ok(None);
    };
    match datum {
        Datum::Frame { .. } => Ok(Some(match frame_from_slots(nominal, band(tol)?)? {
            FrameRead::Frame(f) => FramePlacement::Authored(profile::SketchPlane::from_frame(f)),
            FrameRead::NoDirection(refusal) => FramePlacement::Unreadable(refusal),
        })),
        Datum::FaceFrame { .. } => Ok(Some(FramePlacement::Derived)),
        Datum::Plane { .. }
        | Datum::Axis { .. }
        | Datum::Point { .. }
        | Datum::AxisInPlane { .. } => Ok(None),
    }
}

/// **A profile's `f64` placement — a READ of the frame's result**, not
/// a second evaluation of the frame's slots: the answer
/// [`mint_frame_placement`] minted on the frame node. The frame's
/// content key fixes everything the placement is a function of, so a
/// memo hit carries the same bits a recompute would mint.
///
/// `None` is a DERIVED frame and only that: "not a frame" and
/// "unreadable" have already been discharged into refusals.
///
/// The profile reads the frame's output ([`crate::Doc::upstream`]), so
/// it precedes every reader in the schedule and a failed frame poisons
/// them.
///
/// # Errors
///
/// [`NodeErrorKind::WrongOperand`] when the reference does not name a
/// frame; [`NodeErrorKind::FrameDirection`] where the frame carried a
/// direction refusal ([`FramePlacement::Unreadable`]), naming BOTH
/// `profile` and the frame because the section seam raises it on the
/// loft; and [`NodeErrorKind::MissingInput`] for a reference with no
/// value.
pub(crate) fn profile_plane_f64<T: Decide>(
    results: &Results<T>,
    profile: RecipeNodeId,
    plane: RecipeNodeId,
) -> Result<Option<profile::SketchPlane<f64>>, NodeErrorKind> {
    // A node that is not a frame carries no placement: that `None` is
    // the kind refusal.
    match operand(results, plane, super::phrase::DATUM_FRAME, |v| v.placement)? {
        FramePlacement::Authored(placement) => Ok(Some(placement)),
        FramePlacement::Derived => Ok(None),
        // Raised here, not at the frame, so a frame nobody draws on
        // does not poison the document; it names the frame because the
        // role word alone says which AXIS refused, not whose.
        FramePlacement::Unreadable(refusal) => Err(NodeErrorKind::FrameDirection {
            profile,
            frame: plane,
            refusal,
        }),
    }
}

/// **The sketch plane at the LANE scalar** — the frame's landed value,
/// which is where a parameter driving the frame is still carried.
///
/// The f64 read above is for STRUCTURE. A document parameter can drive
/// a frame's origin, and under an interval or dual run it has a
/// non-degenerate value; embedding the f64 placement into `T` would
/// drop that width from the plane — an enclosure that does not
/// enclose. So structure stays f64-pinned and lane-identical, and
/// magnitudes stay lane-live.
///
/// A DERIVED frame is read here under EVERY lift (DM1c): this by-value
/// read is the only placement it has.
///
/// # Errors
///
/// [`NodeErrorKind::WrongOperand`] when the landed value is not a
/// frame, through [`frame_value`].
pub(crate) fn frame_plane_lane<T: Decide>(
    results: &Results<T>,
    plane: RecipeNodeId,
) -> Result<profile::SketchPlane<T>, NodeErrorKind> {
    Ok(profile::SketchPlane::from_frame(frame_value(
        results, plane,
    )?))
}

/// A lane-scalar placement carried across to `f64`, exactly, where the
/// scalar is the `f64` lane — every component through
/// [`super::SectionScalar::pinned_f64`] — and `None` on any analysis
/// scalar. No component is inspected: the answer is the type's.
pub(crate) fn pinned_plane<T: super::SectionScalar>(
    plane: &profile::SketchPlane<T>,
) -> Option<profile::SketchPlane<f64>> {
    plane.try_map(|x| x.pinned_f64().ok_or(())).ok()
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
) -> Result<OrthoFrame<T>, DirectionRefusal> {
    OrthoFrame::gram_schmidt(origin, u_raw, v_raw, DATUM_UNIT_NORM, band).map_err(|e| {
        DirectionRefusal {
            role: match e.axis {
                OrthoAxis::U => FRAME_X_ROLE,
                OrthoAxis::V => FRAME_Y_ROLE,
            },
            error: e.error,
        }
    })
}

/// **A frame node's landed value** — the one destructure of
/// [`DatumValue::Frame`], for both readers that want it:
/// [`frame_plane_lane`] as a sketch plane, and an in-plane axis as the
/// pair its 2-D coordinates are written against. The witness comes out
/// whole, so neither reader can swap `u` for `v`.
fn frame_value<T: Decide>(
    results: &Results<T>,
    plane: RecipeNodeId,
) -> Result<OrthoFrame<T>, NodeErrorKind> {
    operand(results, plane, super::phrase::DATUM_FRAME, |v| {
        let ValuePayload::Datum(DatumValue::Frame(frame)) = &v.payload else {
            return None;
        };
        Some(*frame)
    })
}

fn wire_datum<T: Decide>(
    d: &Datum,
    doc: &crate::doc::Doc<ProfileProgram>,
    results: &Results<T>,
    vals: &SlotValues<T>,
    tol: Tol,
) -> PayloadResult<T> {
    Ok(ValuePayload::Datum(match d {
        Datum::Plane { .. } => DatumValue::Plane {
            origin: need_point3(vals, SlotId::Origin)?,
            normal: datum_unit(
                need_vec3(vals, SlotId::Normal)?,
                PLANE_NORMAL_ROLE,
                band(tol)?,
            )
            .map_err(DirectionRefusal::node_error)?,
        },
        Datum::Axis { .. } => DatumValue::Axis {
            origin: need_point3(vals, SlotId::Origin)?,
            dir: datum_unit(
                need_vec3(vals, SlotId::Direction)?,
                DATUM_AXIS_ROLE,
                band(tol)?,
            )
            .map_err(DirectionRefusal::node_error)?,
        },
        Datum::Point { .. } => DatumValue::Point {
            position: need_point3(vals, SlotId::Origin)?,
        },
        // u and v must span a plane; `frame_axes` decides that as a
        // length at the same door as every other direction.
        Datum::Frame { .. } => match frame_from_slots(vals, band(tol)?)? {
            FrameRead::Frame(frame) => DatumValue::Frame(frame),
            FrameRead::NoDirection(refusal) => return Err(refusal.node_error()),
        },
        // Coordinates IN a frame, lifted once here. A 2-D pair lifted
        // through the frame's own axes lies in the frame by
        // construction, so there is no in-plane residual to decide.
        Datum::AxisInPlane { frame: plane, .. } => {
            let f = frame_value(
                results,
                super::read_at(doc, crate::OperandSlot::Frame, *plane)?,
            )?;
            let (frame_origin, u, v) = (f.origin(), f.u(), f.v());
            let plane_origin = need_point2(vals, SlotId::Origin)?;
            let plane_dir = need_vec2(vals, SlotId::Direction)?;
            let lift = |d: Vec2<T>| u.get() * d.x + v.get() * d.y;
            DatumValue::AxisInPlane {
                plane_origin,
                plane_dir,
                origin: frame_origin + lift(Vec2::new(plane_origin.x, plane_origin.y)),
                // The frame's axes are orthonormal, so the lift
                // preserves length: decided-zero exactly when the
                // authored pair is.
                dir: datum_unit(lift(plane_dir), DATUM_AXIS_ROLE, band(tol)?)
                    .map_err(DirectionRefusal::node_error)?,
            }
        }
        // **The frame read off a face** (DM1): an authored frame's
        // value through the same `frame_axes` door. Every step is a
        // stored fact copied out or an N5 resolution; nothing here
        // decides a number.
        Datum::FaceFrame { face, .. } => {
            let face = select(doc, results, crate::OperandSlot::Face, *face)?;
            let body = read_body(results, face.at)?;
            let Some(key) = face
                .ents
                .first()
                .and_then(|ent| names::EntityKey::face(ent.key))
            else {
                unreachable!("a face selection holds one face")
            };
            // DM1b / DM2: the carrier's KIND is a stored tag; a
            // comparison of tags, not a predicate.
            let carrier = topo::readback::face_carrier_kind(&body, key)
                .map_err(|error| NodeErrorKind::FaceFrameReadback { error })?;
            if carrier != geom::SurfaceKind::Plane {
                return Err(NodeErrorKind::FaceFrameNotPlanar { carrier });
            }
            let pose = topo::readback::face_pose(&body, key)
                .map_err(|error| NodeErrorKind::FaceFrameReadback { error })?;
            // DM1a: the outward normal is the chart axis folded through
            // the sense beside the pose — the bit selects, nothing is
            // computed.
            let n = OutwardNormal::from_chart(pose.axis, pose.sense).vec();
            // A plane carrier always fixes its u-reference (readback's
            // rule 3), so the kind check above makes this unreachable.
            let u_ref = pose.u_ref.ok_or(NodeErrorKind::FaceFrameReadback {
                error: topo::readback::ReadbackError::NoCanonicalFrame {
                    carrier: "planar carrier without a u-reference",
                },
            })?;
            // The spin: sketch +x is the u-reference turned about the
            // outward normal. `u_ref` lies in the plane, so the rotation
            // is the two-term form; `v` is the right-handed third leg.
            let (sin, cos) = need_scalar(vals, SlotId::Spin)?.sin_cos();
            let u_raw = u_ref * cos + n.cross(u_ref) * sin;
            let v_raw = n.cross(u_raw);
            DatumValue::Frame(
                frame_axes(pose.origin, u_raw, v_raw, band(tol)?)
                    .map_err(DirectionRefusal::node_error)?,
            )
        }
    }))
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
    placement: Option<profile::SketchPlane<f64>>,
    resolved: &[Vec<profile::Step<f64>>],
    ids: &[Vec<crate::node::StepId>],
    tol: Tol,
) -> Result<ProfilePre, NodeErrorKind> {
    // The 2-D record's assembly frame; the conventional frame for a
    // derived frame's profile (DM1c). No decision below reads it: what
    // places the profile is `placement_f64`.
    let plane = placement.unwrap_or_else(profile::SketchPlane::xy);
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
    let profile_f64 = replayed;
    let naming = anchor::derive_naming(&validated_f64, profile_f64.loops()).ok_or({
        // A canonical loop matched no program loop: an internal break.
        // The loop coordinate is not recoverable; 0 names the walk.
        NodeErrorKind::ProfileAnchor { loop_: 0 }
    })?;
    // The records above and the program's minted ids describe one
    // program, so a disagreement is an internal break.
    let pieces = super::ProfilePieces::publish(&naming, &replay_records, ids)
        .map_err(|fault| NodeErrorKind::ProfilePieces { fault })?;
    Ok(ProfilePre {
        profile_f64,
        validated_f64,
        placement_f64: placement,
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
        // One record per program loop, by construction of pass 1. The
        // EMPTY fallback is not a guard: a loop with a fillet refuses at
        // the first resolution (`Guide::consume`), but a loop with none
        // would elaborate against it. What holds the line is the shape
        // check in `replay_guided`.
        let record = pre
            .structure
            .replay
            .get(li as usize)
            .cloned()
            .unwrap_or_default();
        let lp = profile::replay_guided(steps, &record, tol).map_err(|error| {
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
    frame: RecipeNodeId,
    results: &Results<T>,
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
        // The build path: the precompute's VALIDATED form embedded
        // through `from_f64`, every decision carried as the f64 one and
        // none remade (`ProfileLift`); the guided lift is where a
        // margin escalates. A DERIVED frame has no `f64` placement
        // (DM1c), so it is placed at the lane's own value. The op
        // decides nothing: the node's log under this lift is the
        // precompute's.
        super::ProfileLift::Pinned => {
            let plane = match &pre.placement_f64 {
                Some(placement) => placement.map(T::from_f64),
                None => frame_plane_lane(results, frame)?,
            };
            pre.validated_f64.clone().lift_onto(plane)
        }
        super::ProfileLift::Guided => {
            lane_profile::<T>(program, frame_plane_lane(results, frame)?, lane, pre, tol)?
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
/// It shares a SHAPE with `wire_blend` and `wire_boolean` — build the
/// verb, run it through its own door, read the birth record, emit
/// names — but no code, because the operand differs at every step.
///
/// The verb ARGUMENTS come in already resolved: a revolve's axis is a
/// node whose value must be an in-plane axis on the profile's own
/// frame, with an angle classified full or partial under a
/// document-layer refusal. That is what the document MEANS, not a verb
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

/// **Extrudes a profile along its sketch normal** — the distance slot
/// read, and the generic lowering from there.
fn wire_extrude<T: Decide + geom_core::Bounds + topo::AtRestPolicy>(
    id: RecipeNodeId,
    profile: RecipeNodeId,
    side: crate::node::ExtrudeSide,
    results: &Results<T>,
    vals: &SlotValues<T>,
    tol: Tol,
) -> OpResult<T> {
    let distance = need_scalar(vals, SlotId::Distance)?;
    wire_swept(
        &crate::verbs::sweep::extrude(),
        (distance, side),
        id,
        profile,
        results,
        tol,
    )
}

/// The frame a node is written against, read from the RECIPE.
///
/// "The same plane" means the same node: two frames that evaluate to
/// the same numbers are still two frames, and an evaluated comparison
/// would make a revolve's legality depend on a float coincidence.
fn written_against(
    doc: &crate::doc::Doc<ProfileProgram>,
    id: RecipeNodeId,
) -> Option<RecipeNodeId> {
    match doc.node(id)? {
        Node::Profile(p) => doc.operation_of(p.frame),
        Node::Datum(Datum::AxisInPlane { frame: plane, .. }) => doc.operation_of(*plane),
        _ => None,
    }
}

/// **Revolves a profile about an axis written in its own sketch
/// plane** — the document semantics (the axis operand, the same-frame
/// rule, the full-vs-partial classification), and the generic lowering
/// from there.
fn wire_revolve<T: Decide + geom_core::Bounds + topo::AtRestPolicy>(
    id: RecipeNodeId,
    profile: RecipeNodeId,
    axis: RecipeNodeId,
    doc: &crate::doc::Doc<ProfileProgram>,
    results: &Results<T>,
    vals: &SlotValues<T>,
    tol: Tol,
) -> OpResult<T> {
    // Decides only ORDER (`wire_swept` re-checks it): a node whose
    // profile and axis are both wrong refuses on the profile first.
    operand(results, profile, super::family::PROFILE, |v| {
        matches!(v.payload, ValuePayload::Profile(_)).then_some(())
    })?;
    let (plane_origin, plane_dir) = operand(
        results,
        axis,
        super::phrase::AXIS_IN_SKETCH_FRAME,
        |v| match &v.payload {
            ValuePayload::Datum(DatumValue::AxisInPlane {
                plane_origin,
                plane_dir,
                ..
            }) => Some((plane_origin, plane_dir)),
            _ => None,
        },
    )?;
    // The kernel's `RevolveAxis` and the authored axis are both in
    // SKETCH-PLANE coordinates, so the wiring is the identity and the
    // only question is whether the two nodes are written against the
    // SAME frame: an equality of node ids, with no band and no residual.
    let (axis_plane, profile_plane) = (written_against(doc, axis), written_against(doc, profile));
    if axis_plane != profile_plane {
        return Err(NodeErrorKind::AxisInDifferentPlane {
            axis,
            axis_plane,
            profile_plane,
        });
    }
    let axis2 = RevolveAxis {
        origin: *plane_origin,
        dir: *plane_dir,
    };
    let b = band(tol)?;
    // Kernel contract: exactly-full must SAY Full. Anything else wires
    // Partial and the kernel's own classification rules on it.
    let angle = need_scalar(vals, SlotId::RevolveAngle)?;
    let revolution = match turns_off("revolve_full_vs_partial", angle, T::tau(), b)? {
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

/// The spine frame and window a tube door takes, resolved from the
/// frame the node reads and its slots.
///
/// The RESOLUTION is shared by the two tube arms; each then calls its
/// own public door. This layer decides nothing: the frame is the
/// frame datum's, and the window and wall verdicts stay the door's
/// own; a check here would be a weaker second opinion.
struct TubeArgs<T: geom_core::Real> {
    frame: geom_core::OrthoFrame<T>,
    major_radius: T,
    window: sweep::TubeWindow<T>,
    minor_radius: T,
}

fn tube_args<T: Decide>(
    frame: RecipeNodeId,
    window: &crate::node::TubeWindow,
    results: &Results<T>,
    vals: &SlotValues<T>,
) -> Result<TubeArgs<T>, NodeErrorKind> {
    // The frame is read WHOLE — its origin the spine centre, its
    // normal the spine axis, its `u` the reference the window's angles
    // are measured from — as the frame's own door orthonormalized it,
    // so nothing is decided here.
    Ok(TubeArgs {
        frame: frame_value(results, frame)?,
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
    frame: RecipeNodeId,
    window: &crate::node::TubeWindow,
    results: &Results<T>,
    vals: &SlotValues<T>,
    tol: Tol,
) -> OpResult<T> {
    let a = tube_args(frame, window, results, vals)?;
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
    frame: RecipeNodeId,
    window: &crate::node::TubeWindow,
    results: &Results<T>,
    vals: &SlotValues<T>,
    tol: Tol,
) -> OpResult<T> {
    let a = tube_args(frame, window, results, vals)?;
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
    selection: &Selected,
    results: &Results<T>,
    vals: &SlotValues<T>,
    tol: Tol,
) -> OpResult<T> {
    let target = selection.at;
    if selection.ents.is_empty() {
        return Err(NodeErrorKind::BlendSelectionEmpty {
            verb: verb.selection_label,
        });
    }
    let body = finished_operand(results, target, tol)?;
    let size = need_scalar(vals, verb.slots.size_slot)?;
    let target_table = Arc::clone(&value_of(results, target)?.name_table);
    // D9 order; the kernel refuses a repeated edge itself.
    let mut edges: Vec<topo::EdgeKey> = selection
        .ents
        .iter()
        .filter_map(|ent| names::EntityKey::edge(ent.key))
        .collect();
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
    let table = (verb.emitter)(id, target, &target_table, &out.body, &rec)
        .map_err(NodeErrorKind::Naming)?;
    let inputs = crate::coincide::RowInputs {
        a: (target, &target_table),
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
    open: &Selected,
    results: &Results<T>,
    vals: &SlotValues<T>,
    tol: Tol,
) -> OpResult<T> {
    let target = open.at;
    let body = finished_operand(results, target, tol)?;
    let thickness = need_scalar(vals, verb.slots.size_slot)?;
    let target_table = Arc::clone(&value_of(results, target)?.name_table);
    // Designation order: the first designated face of a chart carries
    // its rim.
    let faces: Vec<topo::FaceKey> = open
        .ents
        .iter()
        .filter_map(|ent| names::EntityKey::face(ent.key))
        .collect();
    let built = (verb.build)(faces, thickness);
    // A scalar that may not certify has no shell door and refuses here
    // rather than at an unvalidated hollow.
    let door = <T as topo::AtRestPolicy>::shell_door()
        .ok_or(NodeErrorKind::ShellLaneUnsupported { scalar: T::NAME })?;
    let out = built.run_shell(&body, tol, door).map_err(verb_refused)?;
    let rec = crate::verbs::read_record(out.record, verb.record, verb.foreign_record)?;
    let table = (verb.emitter)(id, target, &target_table, &out.body, &rec)
        .map_err(NodeErrorKind::Naming)?;
    Ok(OpOut::plain(ValuePayload::Body(Arc::new(out.body)), table))
}

/// The mid-evaluation N5 refusal ladder, shared by every door that
/// resolves an AUTHORED name against the tables the run has built so
/// far.
///
/// Mid-evaluation there is no prior run and no whole-evaluation
/// index, so [`mod@crate::resolve`]'s full ladder does not apply. Three
/// rungs remain, in the order the ONE-TABLE doors ask them
/// ([`ladder::resolve_in`]):
///
/// 1. [`ladder::live`] — the minting node must still be in the
///    document. Ids are never reused, so an id the mint log holds was
///    DELETED and one it does not hold was never this document's
///    (`ForeignNode`). The [`ladder::Live`] token makes this rung
///    outrank every later refusal, a door's own included.
/// 2. [`ladder::Landing::Tied`] → `Ambiguous`: the tie row IS the
///    ambiguity (N5).
/// 3. [`ladder::Landing::Absent`] → `Vanished`, through
///    [`ladder::vanished`]: there is no evidence to weigh
///    mid-evaluation.
///
/// Rungs 2 and 3 are ordered by the DOOR. The one-table doors ask 1,
/// 2, 3. The declare door ([`resolve_declarations`]) asks 1 and 3 of
/// BOTH names, then its pair's kind question, then 2: a name that names
/// nothing says so before any question about the pair, and a pair the
/// vocabulary has no step for is unsupported however many entities
/// answer to either name, so the tie is the one per-name refusal the
/// kind question outranks.
///
/// A door supplies [`ladder::Landing`]s, one per table, and keeps its
/// own arity (which table, what a multi-table hit means, which kind it
/// accepts). Every refusal PAYLOAD is [`mod@crate::resolve`]'s, minted
/// by one constructor each ([`crate::resolve::ResolveError::node_gone`],
/// [`crate::resolve::ResolveError::ambiguous`],
/// [`crate::resolve::ResolveError::vanished_fallback`]); what is this
/// module's is the rung ORDER. Refusals come out BOXED, as the doors'
/// error variants carry them.
mod ladder {
    use crate::names::{EntityRef, Entry, NameTable, StableName};
    use crate::program::ProfileProgram;
    use crate::resolve::ResolveError;

    /// Where a name landed in ONE table (rungs 2 and 3, as data).
    pub(super) enum Landing {
        /// Exactly one entity carries the name.
        Unique(EntityRef),
        /// The name is a tie row of this width.
        Tied(usize),
        /// This table does not carry the name.
        Absent,
    }

    /// Proof that rung 1 passed: `name`'s minting node is live.
    ///
    /// Constructible only by [`live`], and required by BOTH [`landing`]
    /// and [`resolve`], so no door can read a table — and so raise a
    /// refusal ABOUT the tables — before the `NodeGone` check. Carrying
    /// the name means [`landing`] and [`resolve`] answer for the same
    /// name.
    #[derive(Clone, Copy, Debug, PartialEq, Eq)]
    pub(super) struct Live<'n>(&'n StableName);

    impl<'n> Live<'n> {
        /// The name rung 1 was paid on. `Copy` copies the proof, which
        /// is sound: it is about a name that cannot change under it.
        pub(super) fn name(self) -> &'n StableName {
            self.0
        }
    }

    /// Reads one table for the live name (N4: resolution IS this read).
    pub(super) fn landing(live: &Live<'_>, table: &NameTable) -> Landing {
        match table.lookup(live.0) {
            Some(Entry::Unique(ent)) => Landing::Unique(*ent),
            Some(Entry::Tied(ents)) => Landing::Tied(ents.len()),
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

    /// **The single-table walk**, all three rungs: rung 1 against the
    /// document, rungs 2 and 3 against ONE table, every refusal
    /// through `refuse` in the caller's own vocabulary. The declare
    /// door walks the rungs itself because it reads TWO tables.
    pub(super) fn resolve_in(
        name: &StableName,
        doc: &crate::doc::Doc<ProfileProgram>,
        table: &NameTable,
        refuse: impl Fn(Box<ResolveError>) -> super::NodeErrorKind,
    ) -> Result<EntityRef, super::NodeErrorKind> {
        let live = live(name, doc).map_err(&refuse)?;
        let landing = landing(&live, table);
        resolve(live, landing).map_err(refuse)
    }

    /// Rungs 2 and 3: the entity, or the refusal its landing earns.
    pub(super) fn resolve(
        live: Live<'_>,
        landing: Landing,
    ) -> Result<EntityRef, Box<ResolveError>> {
        let name = live.0;
        match landing {
            Landing::Unique(ent) => Ok(ent),
            // The tie row is the name itself: there is no widened base
            // to tie against in one table.
            Landing::Tied(width) => Err(Box::new(ResolveError::ambiguous(
                name,
                name.clone(),
                name.node,
                width,
            ))),
            Landing::Absent => Err(vanished(&live)),
        }
    }

    /// Rung 3's payload, from the one home that mints it.
    ///
    /// Split out of [`resolve`] for the declare door, which reaches this
    /// rung on its own.
    pub(super) fn vanished(live: &Live<'_>) -> Box<ResolveError> {
        Box::new(ResolveError::vanished_fallback(live.0))
    }
}

/// **A selection, evaluated** ([`select`]): the operation whose value
/// its body read holds, and the entities its names denote there, in the
/// selection's stored order.
pub(super) struct Selected {
    /// The operation the body read is an output of.
    pub(super) at: RecipeNodeId,
    /// The entities, one per name.
    pub(super) ents: Vec<names::EntityRef>,
}

/// **The selection `var` the node reads at `slot`, evaluated** (D10): its
/// body read resolved to the operation whose value holds it, then each
/// name through that value's table under the [`ladder`], then the
/// entity's kind against the selection's. This is the one place a name
/// a slot reads is resolved; a declared pair's names are the ladder's
/// other caller.
///
/// The road supplies the key and never the word for what it found:
/// that word arrives as a token only `entity_door` can mint, which is
/// why the door lives in a module this one is not an ancestor of
/// (`eval::entity_door`'s docs carry the other half).
///
/// # Errors
///
/// [`NodeErrorKind::UnresolvedRead`] for a body read no live operation
/// defines, [`NodeErrorKind::SelectResolve`] for a name the ladder
/// refuses, [`NodeErrorKind::SelectKind`] for an entity of another kind.
fn select<T: Decide>(
    doc: &crate::doc::Doc<ProfileProgram>,
    results: &Results<T>,
    slot: crate::OperandSlot,
    var: crate::VarId,
) -> Result<Selected, NodeErrorKind> {
    let (Some(selection), Some((entity, _))) = (
        doc.selection(var),
        doc.var(var).and_then(|held| held.kind().selection()),
    ) else {
        return Err(NodeErrorKind::UnresolvedRead { slot, var });
    };
    let at = super::read_at(doc, slot, selection.body)?;
    let table = &value_of(results, at)?.name_table;
    let mut ents = Vec::with_capacity(selection.names.len());
    for (reference, name) in selection.names.iter().enumerate() {
        let ent = ladder::resolve_in(name, doc, table, |error| NodeErrorKind::SelectResolve {
            slot,
            var,
            reference,
            error,
        })?;
        let read: fn(names::EntityKey) -> Option<()> = match entity {
            names::EntityKind::Face => |key| names::EntityKey::face(key).map(|_| ()),
            names::EntityKind::Edge => |key| names::EntityKey::edge(key).map(|_| ()),
            names::EntityKind::Vertex => |key| names::EntityKey::vertex(key).map(|_| ()),
            names::EntityKind::Body => unreachable!("a selection names faces, edges or vertices"),
        };
        super::entity_door::entity(ent.key, read, |found| NodeErrorKind::SelectKind {
            slot,
            var,
            name: Box::new(name.clone()),
            expected: entity,
            found,
        })?;
        ents.push(ent);
    }
    Ok(Selected { at, ents })
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
fn wire_measure<T: Decide + crate::measure::MinClearanceLane>(
    primitive: &crate::measure::MeasurePrimitive<crate::VarId>,
    doc: &crate::doc::Doc<ProfileProgram>,
    results: &Results<T>,
    tol: Tol,
) -> OpResult<T> {
    let dim = primitive.dim();
    let mut sides = Vec::with_capacity(2);
    for (index, &var) in primitive.refs().into_iter().enumerate() {
        let slot = crate::OperandSlot::Measured(
            primitive.kind(),
            u8::try_from(index).unwrap_or_else(|_| unreachable!("a primitive reads two")),
        );
        let (at, ent) = if doc.selection(var).is_some() {
            let selected = select(doc, results, slot, var)?;
            let [ent] = selected.ents.as_slice() else {
                unreachable!("a measure's selection is a singleton (its seat's kind)")
            };
            (selected.at, *ent)
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
/// [`NodeErrorKind::Split`] through [`verb_refused`]. The D7 pinch lane
/// lives inside the kernel door; nothing here re-derives the plane.
fn wire_split<T: Decide + geom_core::Bounds + topo::AtRestPolicy>(
    verb: &crate::verbs::split::SplitVerb<T>,
    id: RecipeNodeId,
    target: RecipeNodeId,
    tool: RecipeNodeId,
    results: &Results<T>,
    tol: Tol,
) -> OpResult<T> {
    let body = finished_operand(results, target, tol)?;
    let tv = value_of(results, tool)?;
    let wrong_tool = || wrong_operand(tv, tool, verb.tool_expected);
    let ValuePayload::Datum(datum) = &tv.payload else {
        return Err(wrong_tool());
    };
    let plane = (verb.tool)(datum).ok_or_else(wrong_tool)?;
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
    let target_table = Arc::clone(&value_of(results, target)?.name_table);
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
    OpOut::plain(ValuePayload::Split { above, below }, emitted.table)
        .grouped(Arc::new(names::FragmentGroups::minted(&emitted.groups)))
        .recording(
            &coincidences,
            &crate::coincide::RowInputs {
                a: (target, &target_table),
                b: None,
                tool: Some(tool),
            },
        )
}

/// **The projection node** (DM3): ONE body out of a split's or a
/// pattern's value, as the `Body` value every consumer already takes.
///
/// The selector and the value must agree in kind — a half against a
/// `Split`, an index against `Instances` — and any other pairing
/// refuses `WrongOperand`. A single body is NOT its own instance 0:
/// nothing is several bodies until a node says so (D3).
///
/// The body handed on is the half's or the instance's own `Arc`. The
/// table is the input's PROJECTED onto it ([`NameTable::project`]):
/// the selected body's rows re-keyed to body 0, names verbatim, no
/// segment added. So a selector spelled against the input's rows
/// resolves here unchanged, and one for another instance refuses as
/// absent, never re-anchored. `check_total` re-checks that the
/// projection dropped nothing the body still has.
fn wire_part<T: Decide>(
    of: RecipeNodeId,
    select: &PartSelect,
    results: &Results<T>,
    vals: &SlotValues<T>,
) -> OpResult<T> {
    let value = value_of(results, of)?;
    let (body, index) = match (select, &value.payload) {
        (PartSelect::SplitHalf(half), ValuePayload::Split { above, below }) => {
            (split_side(of, *half, above, below)?, half.output_body())
        }
        (PartSelect::Instance(_), ValuePayload::Instances(instances)) => {
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
        (PartSelect::SplitHalf(_), _) => {
            return Err(wrong_operand(value, of, super::family::SPLIT));
        }
        (PartSelect::Instance(_), _) => {
            return Err(wrong_operand(value, of, super::family::INSTANCES));
        }
    };
    let table = value
        .name_table
        .project(index)
        .map_err(|dup| NodeErrorKind::Naming(names::NamingError::from(dup)))?;
    names::check_total(&table, &body, 0).map_err(NodeErrorKind::Naming)?;
    Ok(OpOut::plain(ValuePayload::Body(body), Arc::new(table)).carrying(value.parts))
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

/// The body variables `node`'s operands read, in field order: a
/// selection read as the body it states.
fn reads_of(
    node: &Node<ProfileProgram>,
    doc: &crate::doc::Doc<ProfileProgram>,
) -> Vec<crate::VarId> {
    node.operand_rows()
        .into_iter()
        .map(|(_, var)| doc.selection(var).map_or(var, |select| select.body))
        .collect()
}

/// **A read of a split's port is that half** (FORK-1: a split defines
/// two bodies): `results` with each split `reads` names by port
/// replaced by the half the port is, projected exactly as
/// [`wire_part`] projects `Part { SplitHalf }` — the half's body, the
/// table's rows for that output, the split's part count — so the two
/// spellings give one value. `None` when `node` reads no split by port.
/// A part projection reads the split whole: its selector is the half.
fn split_ports_projected<T: Decide>(
    node: &Node<ProfileProgram>,
    reads: &[crate::VarId],
    doc: &crate::doc::Doc<ProfileProgram>,
    results: &Results<T>,
) -> Result<Option<Results<T>>, NodeErrorKind> {
    if matches!(node, Node::Part { .. }) {
        return Ok(None);
    }
    let ports: Vec<(RecipeNodeId, u8)> = reads
        .iter()
        .filter_map(|&var| doc.var(var)?.def().output())
        .filter(|(split, _)| matches!(doc.node(*split), Some(Node::Split { .. })))
        .collect();
    if ports.is_empty() {
        return Ok(None);
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
    for (split, port) in ports {
        let half = SplitHalf::ALL
            .into_iter()
            .find(|h| h.output_body() == u32::from(port))
            .unwrap_or_else(|| unreachable!("a split defines a port per half"));
        let value = value_of(results, split)?;
        let ValuePayload::Split { above, below } = &value.payload else {
            unreachable!("a split evaluates to its two sides")
        };
        let body = split_side(split, half, above, below)?;
        let table = value
            .name_table
            .project(half.output_body())
            .map_err(|dup| NodeErrorKind::Naming(names::NamingError::from(dup)))?;
        names::check_total(&table, &body, 0).map_err(NodeErrorKind::Naming)?;
        let projected = super::NodeValue {
            payload: ValuePayload::Body(body),
            name_table: Arc::new(table),
            fragment_groups: Arc::default(),
            contacts: Arc::default(),
            carried: Arc::default(),
            cited_inputs: Arc::default(),
            ..value.clone()
        };
        local.insert(split, NodeResult::Ok(projected));
    }
    Ok(Some(local))
}

// `Bounds` rides along for the boolean lane only: the sweep's BVH
// candidate generation reads coordinate brackets (the L7 driver-code
// allowance).
//
// The TWO-OPERAND lowering, kept apart from `wire_blend`'s: two operand
// tables, the declared pairs' N5 resolution, the contact-record
// carry and the typed empty success would otherwise become runtime
// arity.
#[allow(clippy::too_many_arguments)] // one parameter per named input; strategy is the §4.4 door
fn wire_boolean<T: Decide + geom_core::Bounds + topo::AtRestPolicy>(
    verb: &crate::verbs::boolean::PairVerb<T>,
    id: RecipeNodeId,
    op: BooleanOp,
    (a, results_a): (RecipeNodeId, &Results<T>),
    (b, results_b): (RecipeNodeId, &Results<T>),
    reads: [crate::VarId; 2],
    declare: &[DeclaredPair],
    doc: &crate::doc::Doc<ProfileProgram>,
    boolean_sweep: topo::SweepStrategy,
    tol: Tol,
) -> OpResult<T> {
    // F5 threading: the declared pairs' names resolve through the
    // OPERANDS' name tables; failures are the N5 typed errors, never a
    // silent drop. The kernel verb receives only the arena-key form.
    let a_table = Arc::clone(&value_of(results_a, a)?.name_table);
    let b_table = Arc::clone(&value_of(results_b, b)?.name_table);
    // **The site is the side**: each name resolves in the ONE table its
    // site designates, so a name carried by both operands is not
    // ambiguous.
    let kernel_decls = if declare.is_empty() {
        BooleanDeclarations::none()
    } else {
        let sided = side_by_operand(declare, (a, &a_table), (b, &b_table), doc)?;
        let (a_records, b_records) = (
            records_of(value_of(results_a, a)?),
            records_of(value_of(results_b, b)?),
        );
        resolve_declarations(
            &sided,
            doc,
            (&a_table, Some(&a_records)),
            (&b_table, Some(&b_records)),
        )?
    };
    let body_a = finished_operand(results_a, a, tol)?;
    let body_b = finished_operand(results_b, b, tol)?;
    match (verb.build)(op, kernel_decls)
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
                &out.body,
                &naming,
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
            .citing(reads.map(crate::coincide::CitedInput::Read).to_vec())
            .recording(
                &out.coincidences,
                &crate::coincide::RowInputs {
                    a: (a, &a_table),
                    b: Some((b, &b_table)),
                    tool: None,
                },
            )
        }
    }
}

/// **The n-ary union's lowering** (DM4): the SAME pair verb, folded
/// over the members in list order — `((m0 ∪ m1) ∪ m2) ∪ …`, through
/// the same `run_pair` door, the same refusal menu, and one body out
/// in the same `BooleanValue::Body` shape a pair union yields, so
/// every consumer of a union is unchanged.
///
/// No new numeric decision is taken here: the geometry is the pair
/// verb's at every step. What the node adds is the NAMING — the fold's
/// tables record join depth, and `names::name_union` rewrites the last
/// one into member-keyed names.
///
/// **Declarations are routed, not positioned** (DM4 as re-ruled). The
/// declared pairs name SITED entities, and [`route_declarations`]
/// sends each pair to the one step that joins its two sites; each
/// step's bucket is resolved by the pair boolean's own
/// [`resolve_declarations`] against that step's two tables.
///
/// **Contact is judged before the fold, pairwise** (DM4's contact
/// rule): [`judge_pairwise_contact`] runs each member pair whose boxes
/// meet, or that carries a declaration, as its own two-member union,
/// so its verdicts and refusals are the same in every member order, and
/// the face pairs each judgement consumed are the union's face links
/// (N2, [`names::UnionLinks`]). **One verdict per carrier pair**: each
/// pair a judgement glued ([`PairVerdict`]) is fed to every fold step
/// as a declared pair between the accumulation's faces that descend
/// from its first face (the fold's lineage, [`names::UnionFold`]) and
/// its second face, and the step decides no carrier pair of its own
/// ([`topo::Verdicts::Given`]). A refusal only a step raises names the
/// member that step folds in ([`NodeErrorKind::UnionFoldStep`]).
///
/// **Nothing ∅-absorbing is invented** (D3). A member that evaluates to
/// an empty boolean refuses `EmptyOperand` naming that member. A fold
/// step that returns empty from two real bodies is a kernel bug, at
/// every step including the last: it refuses
/// [`UNION_STEP_EMPTY`] and names no member, since each one is fine.
#[allow(clippy::too_many_arguments)] // one parameter per named input, as `wire_boolean`
fn wire_union<T: Decide + geom_core::Bounds + topo::AtRestPolicy>(
    verb: &crate::verbs::boolean::PairVerb<T>,
    id: RecipeNodeId,
    members: &[RecipeNodeId],
    declared: &[DeclaredPair],
    doc: &crate::doc::Doc<ProfileProgram>,
    results: &[&Results<T>],
    boolean_sweep: topo::SweepStrategy,
    tol: Tol,
) -> OpResult<T> {
    // Two or more is the node's contract, held at both edit doors
    // (`EditError::TooFewMembers`); fewer refuses typed rather than
    // silently denoting its own input.
    let Some((_, rest)) = members.split_first().filter(|(_, rest)| !rest.is_empty()) else {
        return Err(NodeErrorKind::VerbArity {
            verb: verbs::VerbKind::Boolean(BooleanOp::Union),
            given: verbs::Arity::One,
        });
    };
    // DM4 keys each member's names by the operation it reads, so two
    // members read out of one operation would share one key.
    for (j, &later) in members.iter().enumerate() {
        if let Some(i) = members[..j].iter().position(|&m| m == later) {
            let at = |k: usize| u32::try_from(k).unwrap_or(u32::MAX);
            return Err(NodeErrorKind::MembersShareAnOperation {
                operation: later,
                members: (at(i), at(j)),
            });
        }
    }
    // Every member's body and member-keyed view, taken once for the
    // pairwise judgement and the fold. The FIRST member enters
    // member-keyed too, so every operand of every step is already in
    // this node's name space.
    let own_tables = members
        .iter()
        .zip(results)
        .map(|(&m, results)| Ok(value_of(results, m)?.name_table.as_ref()))
        .collect::<Result<Vec<&NameTable>, NodeErrorKind>>()?;
    let operands = members
        .iter()
        .zip(results)
        .zip(&own_tables)
        .map(|((&m, results), own)| {
            Ok((
                Arc::new(finished_operand(results, m, tol)?),
                Arc::new(names::member_view(id, m, own).map_err(NodeErrorKind::Naming)?),
            ))
        })
        .collect::<Result<Vec<_>, NodeErrorKind>>()?;
    let mut acc_body = Arc::clone(&operands[0].0);
    let mut acc_table = Arc::clone(&operands[0].1);
    // Declarations are routed BEFORE the fold, one bucket per step.
    let buckets = route_declarations(id, members, declared, doc)?;
    // Contact is judged here, pairwise, and nowhere else (DM4). Each
    // member's box is the separation certificate's hull, read through
    // the certificate's one box door; a judged pair's body is discarded.
    let hulls = operands
        .iter()
        .map(|(body, _)| {
            topo::Separation::of(body.as_ref(), tol)
                .map(|s| s.hull())
                .map_err(NodeErrorKind::Boolean)
        })
        .collect::<Result<Vec<_>, NodeErrorKind>>()?;
    let tables: Vec<&NameTable> = operands.iter().map(|(_, t)| t.as_ref()).collect();
    let (links, mut coincidences, verdicts) = judge_pairwise_contact(
        id,
        members,
        &tables,
        &hulls,
        declared,
        doc,
        |p, q, decls| {
            let out = (verb.build)(BooleanOp::Union, decls)
                .run_pair(&operands[p].0, &operands[q].0, boolean_sweep, tol)
                .map_err(verb_refused)?;
            let verbs::PairOut::Out(out) = out else {
                return Err(NodeErrorKind::Naming(names::NamingError::Emission {
                    what: UNION_PAIR_EMPTY,
                }));
            };
            // The judgement's rows, named in the two members' own tables:
            // the union's coincidences are its pairwise judgement's (#4323:
            // the pass is the union's coincidence door, rows in member
            // space), each pair's in the author's list order. A vertex
            // identity is a row only where a record cites it (D1), and
            // the judgement's records are not the union's: the fold step
            // that keeps the touch publishes the row its record cites.
            let judged: Vec<topo::Coincidence> = out
                .coincidences
                .iter()
                .filter(|row| row.site != topo::DecisionSite::VertexFusion)
                .copied()
                .collect();
            let rows = crate::coincide::name_rows(
                &judged,
                &crate::coincide::RowInputs {
                    a: (members[p], own_tables[p]),
                    b: Some((members[q], own_tables[q])),
                    tool: None,
                },
            )
            .map_err(NodeErrorKind::Naming)?;
            let verdicts = pair_verdicts(p, q, &out.coincidences);
            match out.record {
                verbs::VerbRecord::Boolean { naming, .. } => Ok((naming, rows, verdicts)),
                _ => Err(NodeErrorKind::Naming(names::NamingError::Emission {
                    what: verb.foreign_record,
                })),
            }
        },
    )?;
    let mut last: Option<(topo::BooleanResultKind, Arc<topo::ContactRecords>)> = None;
    // What the end pass reads of every step: each face's member faces
    // and each step's discards.
    let mut fold = names::UnionFold::new(members[0], &acc_body);
    // Each step's fragment groups, in fold order (`FragmentGroups::folded`).
    let mut step_groups = Vec::with_capacity(rest.len());
    for step in 0..rest.len() {
        let (member_body, member_table) = &operands[step + 1];
        let (member_body, member_table) = (Arc::clone(member_body), Arc::clone(member_table));
        // This step's pairs, resolved against the two tables it joins by
        // the pair boolean's own door. The accumulation is presented
        // COLLAPSED — its `FromA`/`FromB` rows are the fold's internal
        // space, and a refusal names collapsed rows (`union_refusal`), so
        // this is the one space a caller can write.
        let mut decls = if buckets[step].is_empty() {
            BooleanDeclarations::none()
        } else {
            let acc_view = names::collapse_table(id, &acc_table).map_err(NodeErrorKind::Naming)?;
            let resolved = drop_consumed(look_through_fold(&buckets[step], &acc_view)?, &acc_view);
            // A step's records are not cited from the union, so a
            // same-operand pair has nothing to cite here.
            resolve_declarations(&resolved, doc, (&acc_view, None), (&member_table, None))?
        };
        given_verdicts(&mut decls, &verdicts, step + 1, members, &fold, &acc_body)?;
        let step_refusal = |refusal| NodeErrorKind::UnionFoldStep {
            member: rest[step],
            refusal: Box::new(refusal),
        };
        match (verb.build)(BooleanOp::Union, decls.clone())
            .run_pair(&acc_body, &member_body, boolean_sweep, tol)
            .map_err(|err| step_refusal(verb_refused(err)))?
        {
            // A union of two REAL bodies cannot be empty, so this is a
            // kernel bug. It is NOT attributed to a member, which would
            // send a caller to edit a member that is fine.
            verbs::PairOut::Empty => {
                return Err(NodeErrorKind::Naming(names::NamingError::Emission {
                    what: UNION_STEP_EMPTY,
                }));
            }
            verbs::PairOut::Out(out) => {
                debug_assert_given(&out.coincidences, &decls);
                let verbs::VerbRecord::Boolean {
                    kind,
                    contacts,
                    naming,
                } = out.record
                else {
                    return Err(NodeErrorKind::Naming(names::NamingError::Emission {
                        what: verb.foreign_record,
                    }));
                };
                // The last step's records are the union's, and cite that
                // step's rows: those are published after the pairwise
                // judgement's, named in the union's own space (#4323: a
                // fold row is published), and the records renumbered
                // onto them.
                let contacts = if step + 1 == rest.len() {
                    let acc_view =
                        names::collapse_table(id, &acc_table).map_err(NodeErrorKind::Naming)?;
                    crate::coincide::publish_cited(
                        &contacts,
                        &out.coincidences,
                        &crate::coincide::RowInputs {
                            a: (id, &acc_view),
                            b: Some((id, &member_table)),
                            tool: None,
                        },
                        &mut coincidences,
                    )
                    .map_err(NodeErrorKind::Naming)?
                } else {
                    contacts
                };
                last = Some((kind, Arc::new(contacts)));
                // Minted under THIS node's id, which tells an
                // intermediate row from a member's own name when the
                // chain is collapsed. Both operand contexts name this
                // node too: their tables are member-keyed views.
                let emitted = (verb.emitter)(
                    id,
                    &out.body,
                    &naming,
                    &names::OperandCtx {
                        node: id,
                        table: &acc_table,
                        body: &acc_body,
                    },
                    &names::OperandCtx {
                        node: id,
                        table: &member_table,
                        body: &member_body,
                    },
                    tol,
                )
                .map_err(NodeErrorKind::Naming)?;
                fold.step(rest[step], &naming, &out.body, &emitted.senses)
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
        .zip(&operands)
        .zip(&own_tables)
        .map(|((&node, (body, _)), table)| names::UnionMember { node, body, table })
        .collect();
    let (table, published_groups) =
        names::name_union(id, &acc_body, &acc_table, &member_views, &fold, &links, tol)
            .map_err(NodeErrorKind::Naming)?;
    let body = (*acc_body).clone().into_body();
    // The LAST step's record is the result's, contacts included. A
    // contact fed at an earlier step is consumed there and does not
    // reach the value — the pair verb's carry rule, exactly as a chain
    // of `wire_boolean`s would drop the inner record.
    let Some((kind, contacts)) = last else {
        return Err(NodeErrorKind::VerbArity {
            verb: verbs::VerbKind::Boolean(BooleanOp::Union),
            given: verbs::Arity::One,
        });
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
    .with_coincidences(coincidences))
}

/// **DM4's contact rule: every member pair is judged as its own
/// two-member union, before the fold** — the one place a union's
/// contacts are decided.
///
/// Each pair of members whose closed boxes meet
/// ([`topo::Separation::hull`]) runs the pair verb as `m ∪ n`, handed
/// only the declared pairs between `m` and `n`: it glues what its
/// margins decide one carrier, refuses a sliver, and refuses a
/// contradicted declaration as the pair boolean does. A pair
/// carrying a declaration is run whatever its boxes: the declaration is
/// a claim to verify, and verifying it here keeps the verdict
/// independent of whether the fold fed the pair or consumed its face
/// ([`drop_consumed`]), and tells a mistyped name from a consumed one.
///
/// **Whether it refuses depends on the members, never on their
/// order**: every pair that touches or carries a declaration is judged,
/// in the author's list order (#4323: the list is the author's stated
/// order, and nothing here sorts it away), the listed-first member as
/// operand A. So the rows come out in that order, and where two pairs
/// would refuse, the one the list reaches first is the one named.
///
/// `judge(p, q, decls)` runs the pair verb on members `p` (operand A)
/// and `q` (operand B) and hands back its record; this function decides
/// which pairs are judged and with what, and nothing about geometry.
///
/// Returns the member faces the judgements consumed
/// ([`names::UnionLinks`]): the one place a union's faces are linked
/// (N2), so the links are member order's no more than the verdict is;
/// the judgements' rows; and the carrier pairs they glued, the fold's
/// one verdict per pair ([`PairVerdict`]).
fn judge_pairwise_contact(
    id: RecipeNodeId,
    members: &[RecipeNodeId],
    tables: &[&NameTable],
    hulls: &[bvh::Aabb],
    declared: &[DeclaredPair],
    doc: &crate::doc::Doc<ProfileProgram>,
    mut judge: impl FnMut(
        usize,
        usize,
        BooleanDeclarations,
    ) -> Result<
        (
            topo::BooleanNaming,
            Vec<crate::coincide::NamedCoincidence>,
            Vec<PairVerdict>,
        ),
        NodeErrorKind,
    >,
) -> Result<
    (
        names::UnionLinks,
        Vec<crate::coincide::NamedCoincidence>,
        Vec<PairVerdict>,
    ),
    NodeErrorKind,
> {
    // The declared pairs between two DIFFERENT members, the member the
    // author listed first as operand A. `route_declarations` already
    // sited these pairs through the same door, so a refusal here is a
    // bug.
    let site = |r: &SitedRef, reference: usize| {
        member_site(id, members, r, reference, doc).map_err(|_| {
            NodeErrorKind::Naming(names::NamingError::Emission {
                what: PAIRWISE_SITE_UNROUTED,
            })
        })
    };
    let mut between: std::collections::BTreeMap<(usize, usize), Vec<SidedPair<'static>>> =
        std::collections::BTreeMap::new();
    for (k, ((r1, r2), class)) in declared.iter().enumerate() {
        let ((i, n1), (j, n2)) = (site(r1, 2 * k)?, site(r2, 2 * k + 1)?);
        if i == j {
            continue;
        }
        let (lo, hi) = (i.min(j), i.max(j));
        let op = |k: usize| {
            if k == lo {
                topo::Operand::A
            } else {
                topo::Operand::B
            }
        };
        between.entry((lo, hi)).or_default().push((
            (op(i), n1, 2 * k),
            (op(j), n2, 2 * k + 1),
            *class,
        ));
    }
    let mut links = names::UnionLinks::default();
    let mut rows = Vec::new();
    let mut verdicts = Vec::new();
    // Each pair once, in the author's list order (#4323: the list is
    // the author's stated order; nothing here sorts it away).
    for p in 0..members.len() {
        for q in p + 1..members.len() {
            let pairs = between.remove(&(p, q)).unwrap_or_default();
            if pairs.is_empty() && !hulls[p].overlaps(&hulls[q]) {
                continue;
            }
            let decls = if pairs.is_empty() {
                BooleanDeclarations::none()
            } else {
                resolve_declarations(&pairs, doc, (tables[p], None), (tables[q], None))?
            };
            let (naming, judged, glued) = judge(p, q, decls)?;
            links
                .judged(members[p], members[q], &naming)
                .map_err(NodeErrorKind::Naming)?;
            rows.extend(judged);
            verdicts.extend(glued);
        }
    }
    Ok((links, rows, verdicts))
}

/// **One carrier pair a union's pairwise judgement glued**: a face of
/// each of two members, by member index, and the class its decided
/// relation makes it. The fold reads it through lineage
/// ([`given_verdicts`]) and decides the pair nowhere else.
#[derive(Clone, Copy, Debug)]
struct PairVerdict {
    /// The listed-first member's face: `(member index, face)`.
    a: (usize, topo::FaceKey),
    /// The other member's face.
    b: (usize, topo::FaceKey),
    /// The class the judgement's relation makes the pair.
    class: topo::BooleanCoincidence,
}

/// The carrier pairs the judgement of members `p` (operand A) and `q`
/// (operand B) glued, read off its rows: each face pair of the two
/// operands a row relates as one carrier or a tangency.
fn pair_verdicts(p: usize, q: usize, rows: &[topo::Coincidence]) -> Vec<PairVerdict> {
    rows.iter()
        .filter_map(|row| {
            let [
                topo::RowCell::Input {
                    input: topo::Operand::A,
                    cell: topo::Cell::Face(fa),
                },
                topo::RowCell::Input {
                    input: topo::Operand::B,
                    cell: topo::Cell::Face(fb),
                },
            ] = row.cells
            else {
                return None;
            };
            let class = row.relation.glued_class()?;
            Some(PairVerdict {
                a: (p, fa),
                b: (q, fb),
                class,
            })
        })
        .collect()
}

/// **The pass's verdicts for the fold step that folds in member
/// `member`**, added to `decls` as declared pairs and `decls` marked
/// [`topo::Verdicts::Given`]: each verdict whose second face is
/// `member`'s, between that face and every accumulation face whose
/// lineage holds the verdict's first face. A pair the step's routed
/// declarations already name is left as declared.
///
/// # Errors
///
/// An emission bug ([`UNION_VERDICTS_DISAGREE`]) where two verdicts
/// glue one accumulation face to one member face under two classes, or
/// an accumulation face carries no lineage.
fn given_verdicts<T: Decide>(
    decls: &mut BooleanDeclarations,
    verdicts: &[PairVerdict],
    member: usize,
    members: &[RecipeNodeId],
    fold: &names::UnionFold,
    acc: &topo::Body<T>,
) -> Result<(), NodeErrorKind> {
    let bug = |what| NodeErrorKind::Naming(names::NamingError::Emission { what });
    let mut given: std::collections::BTreeMap<(topo::FaceKey, topo::FaceKey), _> = decls
        .coincident_faces
        .iter()
        .map(|d| ((d.a, d.b), (d.class, true)))
        .collect();
    for (f, _) in acc.faces() {
        let parents = fold.member_faces(f).ok_or(bug(UNION_FACE_NO_LINEAGE))?;
        for v in verdicts.iter().filter(|v| v.b.0 == member) {
            if !parents.contains(&(members[v.a.0], v.a.1)) {
                continue;
            }
            match given.entry((f, v.b.1)) {
                std::collections::btree_map::Entry::Vacant(slot) => {
                    slot.insert((v.class, false));
                }
                std::collections::btree_map::Entry::Occupied(held) => {
                    let (class, routed) = *held.get();
                    if !routed && class != v.class {
                        return Err(bug(UNION_VERDICTS_DISAGREE));
                    }
                }
            }
        }
    }
    for (&(a, b), &(class, routed)) in &given {
        if !routed {
            decls
                .coincident_faces
                .push(topo::FacePairDeclaration::new(a, b, class));
        }
    }
    decls.verdicts = topo::Verdicts::Given;
    Ok(())
}

/// **Every face-pair row a fold step recorded is a pair it was given**
/// (DM4, the fold re-glues no carrier pair): the verdict reuse, asserted
/// where the step's rows come back. A step's face-pair rows are its
/// declaration door's, so one naming a pair outside `decls` is a fresh
/// glue on a piece — what a glue door or recording site that ignored
/// `Verdicts::Given` would add. Readings that record no row are outside
/// it: the merge's continuation reading, the maximal-faces gate and the
/// join's coaxial frame read a step's carriers by their margins, as the
/// pairwise pass read the members'.
fn debug_assert_given(rows: &[topo::Coincidence], decls: &BooleanDeclarations) {
    if cfg!(debug_assertions) {
        for row in rows {
            if let [
                topo::RowCell::Input {
                    input: topo::Operand::A,
                    cell: topo::Cell::Face(fa),
                },
                topo::RowCell::Input {
                    input: topo::Operand::B,
                    cell: topo::Cell::Face(fb),
                },
            ] = row.cells
            {
                debug_assert!(
                    decls
                        .coincident_faces
                        .iter()
                        .any(|d| d.a == fa && d.b == fb),
                    "a union fold step decided the carrier pair {fa:?}/{fb:?} itself: its \
                     verdict is the pairwise judgement's, read through lineage"
                );
            }
        }
    }
}

/// An accumulation face the fold's lineage does not hold.
const UNION_FACE_NO_LINEAGE: &str = "a union fold's accumulation face has no member-face lineage";

/// Two of a union's pairwise verdicts glue one accumulation face to one
/// member face under two classes.
const UNION_VERDICTS_DISAGREE: &str =
    "a union's pairwise judgements glue one accumulation face to one member face under two classes";

/// A declared pair's site did not site in the pairwise judgement,
/// after [`route_declarations`] sited the same pair through the same
/// door ([`member_site`]).
const PAIRWISE_SITE_UNROUTED: &str =
    "a union's pairwise contact judgement met a declared site the routing did not refuse";

/// **A certified pair whose accumulation-side face the fold consumed
/// whole is satisfied, and leaves the step's bucket** (DM4, the
/// declaration channel).
///
/// Consumed whole means no face row of the accumulation descends from
/// the face ([`names::face_descends_from`]): another member contains
/// it, so the contact has nothing left to back. A face that survives
/// only in pieces never reaches here; [`look_through_fold`] refused it.
///
/// Only a CROSS pair's accumulation side is read. Its names were
/// resolved at their sites by [`judge_pairwise_contact`], so a name
/// absent here was consumed, not mistyped. A pair whose two sites are
/// one member passes through untouched.
fn drop_consumed<'n>(bucket: Vec<SidedPair<'n>>, acc_table: &NameTable) -> Vec<SidedPair<'n>> {
    let consumed = |(op, sided, _): &Side<'n>| {
        let face = sided.name();
        *op == topo::Operand::A
            && !acc_table
                .iter()
                .any(|(row, _)| names::face_descends_from(row, face))
    };
    bucket
        .into_iter()
        .filter(|(s1, s2, _)| s1.0 == s2.0 || !(consumed(s1) || consumed(s2)))
        .collect()
}

/// One declared pair as the shared resolver takes it: each side's
/// name in the table of the operand its SITE picked, and the class.
///
/// [`wire_boolean`] builds it from the names as authored
/// ([`side_by_operand`]); [`wire_union`] rewrites each into the node's
/// member space ([`route_declarations`]). Either way
/// [`resolve_declarations`] gets one shape.
type SidedPair<'n> = (Side<'n>, Side<'n>, topo::BooleanCoincidence);

/// One side of a [`SidedPair`]: the operand its site picked, its name,
/// and its place among the node's payload names
/// ([`crate::Node::payload_names`]), which a refusal of it carries.
type Side<'n> = (topo::Operand, SidedName<'n>, usize);

/// One side's name on its way to the shared resolver, and whether
/// rung 1 is already paid on it.
///
/// A pair boolean's authored name travels unchanged, carrying the
/// rung-1 token [`site_operand`] already paid. A union mints a NEW name
/// ([`names::member_name`], and maybe [`look_through_fold`]), which
/// pays its own.
#[derive(Clone, Debug, PartialEq, Eq)]
enum SidedName<'n> {
    /// The authored name, rung 1 paid.
    Live(ladder::Live<'n>),
    /// A name a union's door minted from the authored one.
    Rewritten(names::StableName),
}

impl SidedName<'_> {
    /// The name itself, whichever way it got here.
    fn name(&self) -> &names::StableName {
        match self {
            Self::Live(live) => live.name(),
            Self::Rewritten(name) => name,
        }
    }
}

/// **Which operand a declared side's SITE names** — one answer for
/// both declaring doors, with rung 1 paid before it is given.
///
/// The site question is about which TABLE a name is read in, so it
/// ranks below rung 1: a name whose minting node is gone says THAT,
/// whatever its site.
///
/// Two operands that read one operation (a split's two halves) share
/// its site, and the side is the one whose own table `holds` the name;
/// this siding lives until declared pairs retire.
///
/// The refusal for a site that is not an operand, or that two operands
/// share and the name does not tell apart, is the caller's (`absent`):
/// a site fault for a pair boolean
/// ([`NodeErrorKind::DeclareSiteNotAnOperand`]), the vanished name DM4
/// says it is for a union, whose member LIST `SetMembers` rewrites.
fn site_operand<'n>(
    r: &'n SitedRef,
    reference: usize,
    operands: &[RecipeNodeId],
    holds: impl Fn(usize, &names::StableName) -> bool,
    doc: &crate::doc::Doc<ProfileProgram>,
    absent: impl FnOnce(&ladder::Live<'n>) -> NodeErrorKind,
) -> Result<(usize, ladder::Live<'n>), NodeErrorKind> {
    let live = ladder::live(&r.name, doc)
        .map_err(|error| NodeErrorKind::DeclareResolve { error, reference })?;
    let sited: Vec<usize> = (0..operands.len())
        .filter(|&i| operands[i] == r.at)
        .collect();
    let side = match sited.as_slice() {
        [one] => Some(*one),
        several => {
            let mut holding = several.iter().copied().filter(|&i| holds(i, &r.name));
            holding.next().filter(|_| holding.next().is_none())
        }
    };
    match side {
        Some(i) => Ok((i, live)),
        None => Err(absent(&live)),
    }
}

/// **A pair boolean's declared pairs, sided by their sites** — `at ==
/// a` is operand A, `at == b` is operand B, and anything else refuses
/// typed.
///
/// The names travel unchanged, with the rung-1 token [`site_operand`]
/// mints: a pair boolean's operand tables are the operands' own.
fn side_by_operand<'n>(
    pairs: &'n [DeclaredPair],
    (a, a_table): (RecipeNodeId, &NameTable),
    (b, b_table): (RecipeNodeId, &NameTable),
    doc: &crate::doc::Doc<ProfileProgram>,
) -> Result<Vec<SidedPair<'n>>, NodeErrorKind> {
    let side = |r: &'n SitedRef, reference: usize| -> Result<Side<'n>, NodeErrorKind> {
        let tables = [a_table, b_table];
        let holds = |i: usize, name: &names::StableName| tables[i].lookup(name).is_some();
        let (i, live) = site_operand(r, reference, &[a, b], holds, doc, |_| {
            NodeErrorKind::DeclareSiteNotAnOperand { at: r.at }
        })?;
        let op = if i == 0 {
            topo::Operand::A
        } else {
            topo::Operand::B
        };
        Ok((op, SidedName::Live(live), reference))
    };
    pairs
        .iter()
        .enumerate()
        .map(|(k, ((r1, r2), class))| Ok((side(r1, 2 * k)?, side(r2, 2 * k + 1)?, *class)))
        .collect()
}

/// **Routing a union's declared pairs to their fold steps** (DM4, the
/// declaration channel): one bucket per step, filled from the two
/// SITES the pair names and from nothing else.
///
/// Member `i` is the joining operand at step `i - 1` and inside the
/// accumulation at every step after, so the step that has both sites
/// is the LATER member's: bucket `max(i, j) - 1`, the joining member
/// operand B and the accumulation operand A. A pair whose two sites are
/// ONE member is that member's carried contact at its own step (member
/// 0's saturates to step 0).
///
/// **Every sited pair has a step**, given the node's arity contract of
/// two members or more, which [`wire_union`] refuses before calling
/// this; a one-member list would index past the end.
///
/// Each pair is rewritten into the node's member space
/// ([`names::member_name`]), so the pair boolean's own resolver runs on
/// it. A site the member list does not hold — what `SetMembers` leaves
/// by removing a declared member — refuses as a vanished name (N5).
fn route_declarations(
    id: RecipeNodeId,
    members: &[RecipeNodeId],
    pairs: &[DeclaredPair],
    doc: &crate::doc::Doc<ProfileProgram>,
) -> Result<Vec<Vec<SidedPair<'static>>>, NodeErrorKind> {
    let steps = members.len().saturating_sub(1);
    let mut buckets: Vec<Vec<SidedPair<'static>>> = vec![Vec::new(); steps];
    for (k, ((r1, r2), class)) in pairs.iter().enumerate() {
        let ((i, n1), (j, n2)) = (
            member_site(id, members, r1, 2 * k, doc)?,
            member_site(id, members, r2, 2 * k + 1, doc)?,
        );
        let bucket = i.max(j).saturating_sub(1);
        let joining = bucket + 1;
        let op = |index: usize| {
            if index == joining {
                topo::Operand::B
            } else {
                topo::Operand::A
            }
        };
        buckets[bucket].push(((op(i), n1, 2 * k), (op(j), n2, 2 * k + 1), *class));
    }
    Ok(buckets)
}

/// **A union's declared side, sited at its member**: the member's
/// position in the list, and the name rewritten into the node's member
/// space ([`names::member_name`]), for both of a union's readers of its
/// declarations ([`route_declarations`], [`judge_pairwise_contact`]).
///
/// Rung 1 first ([`site_operand`]). A site the member list does not
/// hold refuses as a vanished name (N5). The rung-1 token is not
/// carried past here: the minted name pays its own.
fn member_site(
    id: RecipeNodeId,
    members: &[RecipeNodeId],
    r: &SitedRef,
    reference: usize,
    doc: &crate::doc::Doc<ProfileProgram>,
) -> Result<(usize, SidedName<'static>), NodeErrorKind> {
    // A union's members read distinct operations
    // ([`NodeErrorKind::MembersShareAnOperation`]), so no two share a
    // site and none is sided by its table.
    let (i, _) = site_operand(
        r,
        reference,
        members,
        |_, _| false,
        doc,
        |live| NodeErrorKind::DeclareResolve {
            error: ladder::vanished(live),
            reference,
        },
    )?;
    Ok((
        i,
        SidedName::Rewritten(names::member_name(id, r.at, &r.name)),
    ))
}

/// **A member-space name read through the fold's compositions**, one
/// step's bucket against the accumulation's table, before the pair
/// boolean's own door ([`resolve_declarations`]) sees the pair: a name
/// the fold MERGED away is rewritten to the merged row that holds it,
/// and one the fold left only in PIECES — split, or inside a merge it
/// later fragmented — refuses, naming the composition.
///
/// A face merged at an earlier step is in exactly one `[Merged(set)]`
/// row (`names::merged::covers`): the one consumption with a unique
/// successor, so the one looked through. A face one step cut and
/// partly merged is in that row and in a fragment of its own beside
/// it, so it is a split.
///
/// A split, and a merge a later step fragmented, have no unique
/// successor, so the pair refuses `Vanished` with
/// [`crate::resolve::Diagnosis::ConsumedByFold`], the composition read
/// off the rows that DESCEND from the name ([`fold_descent`]), never
/// by measuring the face again (DM4).
///
/// A name NOTHING in the accumulation descends from is handed on as
/// written: either another member consumed it whole
/// ([`drop_consumed`]), or it is in no table and the door refuses it as
/// vanished. A face split and then consumed whole in every piece is
/// the first case.
///
/// Only the ACCUMULATION side looks through: no composition could have
/// consumed a face of a member that has not joined yet.
///
/// Two refusals are emission bugs the flat mint cannot produce: a face
/// in the set of TWO bare merged rows, and descendant ROWS that
/// disagree about which composition consumed it (the first
/// composition retires the bare name). Disagreement is read across
/// rows only; within a row [`fold_descent`]'s first reading decides,
/// so a merged row whose constituents disagree goes unseen: a shape
/// the mint cannot produce either.
fn look_through_fold<'n>(
    bucket: &[SidedPair<'n>],
    acc_table: &NameTable,
) -> Result<Vec<SidedPair<'n>>, NodeErrorKind> {
    use crate::names::RoleSeg;
    let merged_row_of =
        |(op, sided, reference): &Side<'n>| -> Result<Option<names::StableName>, NodeErrorKind> {
            let name = sided.name();
            if *op == topo::Operand::B || acc_table.lookup(name).is_some() {
                return Ok(None);
            }
            let split = acc_table
                .iter()
                .any(|(row, _)| fold_descent(row, name) == Some(FoldConsumption::Split));
            let mut rows = acc_table
                .iter()
                .filter_map(|(row, _)| match row.path.as_slice() {
                    [RoleSeg::Merged(set)] if names::merged::covers(set, name) => Some(row),
                    _ => None,
                });
            match (rows.next(), rows.next()) {
                (Some(row), None) if !split => return Ok(Some(row.clone())),
                (Some(_), Some(_)) => {
                    return Err(NodeErrorKind::Naming(names::NamingError::Emission {
                        what: MEMBER_FACE_IN_TWO_MERGES,
                    }));
                }
                _ => {}
            }
            let mut ways = acc_table
                .iter()
                .filter_map(|(row, _)| fold_descent(row, name));
            let by = match ways.next() {
                Some(first) if ways.all(|w| w == first) => first,
                Some(_) => {
                    return Err(NodeErrorKind::Naming(names::NamingError::Emission {
                        what: MEMBER_FACE_CONSUMED_TWO_WAYS,
                    }));
                }
                None => return Ok(None),
            };
            Err(NodeErrorKind::DeclareResolve {
                error: Box::new(crate::resolve::ResolveError::Vanished {
                    name: name.clone(),
                    diagnosis: crate::resolve::Diagnosis::ConsumedByFold { by },
                    last_good: None,
                }),
                reference: *reference,
            })
        };
    bucket
        .iter()
        .map(|(s1, s2, class)| {
            let rewritten = |s: &Side<'n>| {
                Ok(match merged_row_of(s)? {
                    Some(row) => (s.0, SidedName::Rewritten(row), s.2),
                    None => s.clone(),
                })
            };
            Ok((rewritten(s1)?, rewritten(s2)?, *class))
        })
        .collect()
}

/// **How `row` descends from the member-space `name`**, when it does:
/// the composition that consumed the name, read off the row's SHAPE.
///
/// Only a row [`names::face_descends_from`] accepts is classified, so
/// a face refused here never reads as consumed whole in
/// [`drop_consumed`]. That predicate asks WHETHER anything of the face
/// survives; this asks which composition left it in PIECES, and a
/// piece is a `Fragment` tail, so a descendant without one is `None`.
///
/// A fragmented head equal to the name is a
/// [split](FoldConsumption::Split). A fragmented `Merged` head covering
/// the name is a [fragmented merge](FoldConsumption::FragmentedMerge).
/// A `Merged` head that does not cover the name says what its first
/// descending constituent says: a fragment merged later was still
/// consumed by the split that made it a fragment. A BARE merged row
/// covering the name is the look-through's, and `None` here.
fn fold_descent(row: &names::StableName, name: &names::StableName) -> Option<FoldConsumption> {
    use crate::names::RoleSeg;
    // A worklist in the order the first answer is looked for: a row's
    // constituents in set order, each's own before the next's.
    let mut rows = vec![row];
    while let Some(row) = rows.pop() {
        if !names::face_descends_from(row, name) {
            continue;
        }
        let tail = row
            .path
            .iter()
            .rev()
            .take_while(|seg| matches!(seg, RoleSeg::Fragment(_)))
            .count();
        let head = &row.path[..row.path.len() - tail];
        let fragmented = tail > 0;
        // No node or kind test needed: the guard's other routes descend
        // into names nested inside the path, and no name descends from a
        // name that contains it, so a head equal to the name's path is the
        // name's own node and kind.
        if fragmented && head == name.path.as_slice() {
            return Some(FoldConsumption::Split);
        }
        let [RoleSeg::Merged(set)] = head else {
            continue;
        };
        if !names::merged::covers(set, name) {
            rows.extend(set.iter().rev());
        } else if fragmented {
            return Some(FoldConsumption::FragmentedMerge);
        }
    }
    None
}

/// A union's accumulation lists one member face in the constituent
/// sets of two merged rows, which the flat mint cannot produce.
const MEMBER_FACE_IN_TWO_MERGES: &str =
    "a union's accumulation holds one member face in two merged rows' constituent sets";

/// A union's accumulation holds rows descending from one member face
/// by two different compositions, which the retiring mint cannot
/// produce: the first composition to consume a face retires its name.
const MEMBER_FACE_CONSUMED_TWO_WAYS: &str =
    "a union's accumulation holds rows descending from one member face by two compositions";

/// A union's pairwise judgement returned the typed empty from two real
/// bodies.
const UNION_PAIR_EMPTY: &str =
    "a union's pairwise judgement returned empty from two non-empty members";

/// A union fold step returned the typed empty from two real bodies.
const UNION_STEP_EMPTY: &str = "a union fold step returned empty from two non-empty operands";

/// Resolves one node's declared pairs against the two operand
/// tables into the kernel's [`BooleanDeclarations`] (F5).
///
/// [`wire_boolean`] calls it with the two operands' tables;
/// [`wire_union`] once per fold step, with the accumulation (in this
/// node's published space) and the joining member's `member_view`.
///
/// The pair vocabulary is [`DeclaredStep`]. The resolver is
/// carrier-agnostic: it pushes a `FacePairDeclaration` whatever the
/// faces' surface kinds, and the kernel's ladder verifies it.
/// Resolution scope is the OPERANDS' tables (spec D4), so a name minted
/// elsewhere is Vanished HERE even if another node still carries it.
/// Each name is read in exactly ONE table, the one its SITE picked, so
/// two placements of one prototype are told apart.
fn resolve_declarations<'n>(
    pairs: &'n [SidedPair<'n>],
    doc: &crate::doc::Doc<ProfileProgram>,
    (a_table, a_records): (&NameTable, Option<&topo::ContactRecords>),
    (b_table, b_records): (&NameTable, Option<&topo::ContactRecords>),
) -> Result<BooleanDeclarations, NodeErrorKind> {
    let mut out = BooleanDeclarations::none();
    for ((o1, n1, r1), (o2, n2, r2), class) in pairs {
        let (o1, o2, class) = (*o1, *o2, *class);
        let refused = |reference| move |error| NodeErrorKind::DeclareResolve { error, reference };
        // Rungs 1 and 3 for both names, the kind question, then rung 2
        // (the order is `ladder`'s doc).
        let table_of = |op| match op {
            topo::Operand::A => a_table,
            topo::Operand::B => b_table,
        };
        let (live1, l1) = declare_landing(n1, *r1, doc, table_of(o1))?;
        let (live2, l2) = declare_landing(n2, *r2, doc, table_of(o2))?;
        let (n1, n2) = (n1.name(), n2.name());
        // Asked of the NAMES' kinds, which every `NameTable` door that
        // seats a row makes every candidate's kind, so a tie answers it
        // too.
        let unsupported = |kinds| NodeErrorKind::DeclareUnsupportedPair {
            kinds,
            cross_operand: o1 != o2,
        };
        let Some(step) = declared_step((o1, n1.kind), (o2, n2.kind)) else {
            return Err(unsupported((n1.kind, n2.kind)));
        };
        let k1 = ladder::resolve(live1, l1).map_err(refused(*r1))?.key;
        let k2 = ladder::resolve(live2, l2).map_err(refused(*r2))?.key;
        // The arms below PROJECT the keys of the step, reading its
        // ORIENTATION off the step's [`sides`] tokens. A projection that
        // fails means a table holds a key of another kind than its
        // name's: asserted in debug, answered off the KEYS in release.
        let broke = |shape: &'static str| {
            debug_assert!(
                false,
                "a declared {shape} pair projected a key of another kind than its name's: \
                 every `NameTable` door that seats a row admits it only at its \
                 name's kind"
            );
            unsupported((k1.kind(), k2.kind()))
        };
        // A carried row is a CONTACT; a continuation is a relation
        // between two faces and has no vertex reading, so a vertex step
        // declared as one is an unsupported pair (the one check, read by
        // both vertex arms).
        let vertex_class = class.contact();
        // A same-operand pair is a contact its operand carries in: it
        // cites the operand's record of the pair, and a pair the operand
        // records no contact for, or one at a union step, backs nothing.
        let backing_record = |op: topo::Operand, pair| {
            match op {
                topo::Operand::A => a_records,
                topo::Operand::B => b_records,
            }
            .and_then(|records| records.position(pair))
            .ok_or(NodeErrorKind::DeclaredContactUnbacked {
                reference: *r1,
                at_union_step: a_records.is_none(),
            })
        };
        match step {
            DeclaredStep::CrossFaces(sides) => {
                let (a, b) = sides.a_then_b(k1, k2);
                let (Some(fa), Some(fb)) = (a.face(), b.face()) else {
                    return Err(broke("cross-operand face"));
                };
                out.coincident_faces
                    .push(FacePairDeclaration::new(fa, fb, class));
            }
            DeclaredStep::SameVv(side) => {
                let Some(class) = vertex_class else {
                    return Err(unsupported((n1.kind, n2.kind)));
                };
                let (Some(va), Some(vb)) = (k1.vertex(), k2.vertex()) else {
                    return Err(broke("same-operand vertex-vertex"));
                };
                let record = backing_record(
                    side.operand(),
                    (topo::Cell::Vertex(va), topo::Cell::Vertex(vb)),
                )?;
                // The AUTHORED class, carried, not re-defaulted.
                carried(&mut out, side.operand()).vv.push(CarriedVv {
                    pair: VvContact { a: va, b: vb },
                    class,
                    record,
                });
            }
            DeclaredStep::SameVf(side, roles) => {
                let Some(class) = vertex_class else {
                    return Err(unsupported((n1.kind, n2.kind)));
                };
                let (v, f) = roles.vertex_then_face(k1, k2);
                let (Some(vertex), Some(face)) = (v.vertex(), f.face()) else {
                    return Err(broke("same-operand vertex-face"));
                };
                let record = backing_record(
                    side.operand(),
                    (topo::Cell::Vertex(vertex), topo::Cell::Face(face)),
                )?;
                carried(&mut out, side.operand()).vf.push(CarriedVf {
                    rest: VfContact { vertex, face },
                    class,
                    record,
                });
            }
        }
    }
    Ok(out)
}

/// The carried-contact sink a SAME-operand declaration lands in.
fn carried(out: &mut BooleanDeclarations, op: topo::Operand) -> &mut CarriedContacts {
    match op {
        topo::Operand::A => &mut out.carried_a,
        topo::Operand::B => &mut out.carried_b,
    }
}

/// **The orientation facts a declared pair's step rests on, as tokens
/// only a COMPARISON of the two sides can mint** (the device
/// [`ladder::Live`] uses).
///
/// [`DeclaredStep`] carries these rather than a bare `Operand` or a
/// `bool`, so [`resolve_declarations`] reads the orientation
/// [`declared_step`] decided instead of re-deriving it. The fields are
/// private and the `of` constructors the only way in, so an arm cannot
/// fabricate an orientation. What remains spellable is calling a
/// comparison with ONE side twice (`SameOperand::of(oa, oa)`).
mod sides {
    use super::names::EntityKind;
    use topo::Operand;

    /// Proof that two declared names landed in the SAME operand, and
    /// which one.
    #[derive(Clone, Copy)]
    pub(super) struct SameOperand(Operand);

    impl SameOperand {
        /// `None` unless the two names landed in one operand.
        pub(super) fn of(a: Operand, b: Operand) -> Option<Self> {
            (a == b).then_some(Self(a))
        }

        /// The operand both names landed in.
        pub(super) fn operand(self) -> Operand {
            self.0
        }
    }

    /// Proof that two declared names landed in DIFFERENT operands,
    /// and which of the two is operand A's.
    #[derive(Clone, Copy)]
    pub(super) struct CrossOperand {
        a_is_first: bool,
    }

    impl CrossOperand {
        /// `None` unless the two names landed in different operands.
        pub(super) fn of(a: Operand, b: Operand) -> Option<Self> {
            (a != b).then_some(Self {
                a_is_first: a == Operand::A,
            })
        }

        /// The pair in OPERAND order, A's first.
        pub(super) fn a_then_b<T>(self, first: T, second: T) -> (T, T) {
            if self.a_is_first {
                (first, second)
            } else {
                (second, first)
            }
        }
    }

    /// Proof that of two declared kinds exactly one is a VERTEX and
    /// the other a FACE, and which is which.
    #[derive(Clone, Copy)]
    pub(super) struct VertexAndFace {
        vertex_is_first: bool,
    }

    impl VertexAndFace {
        /// `None` unless the two kinds are one vertex and one face.
        pub(super) fn of(a: EntityKind, b: EntityKind) -> Option<Self> {
            match (a, b) {
                (EntityKind::Vertex, EntityKind::Face) => Some(Self {
                    vertex_is_first: true,
                }),
                (EntityKind::Face, EntityKind::Vertex) => Some(Self {
                    vertex_is_first: false,
                }),
                _ => None,
            }
        }

        /// The pair in ROLE order, the vertex's first.
        pub(super) fn vertex_then_face<T>(self, first: T, second: T) -> (T, T) {
            if self.vertex_is_first {
                (first, second)
            } else {
                (second, first)
            }
        }
    }
}

/// **The step a declared pair has in the v1 threading vocabulary** —
/// the ONE enumeration of that vocabulary in this crate
/// ([`NodeErrorKind::DeclareUnsupportedPair`] points here).
///
/// Each variant carries its ORIENTATION as a [`sides`] token. A fourth
/// VARIANT fails to compile until the projection covers it (`E0004`);
/// a fourth pair shape reusing a variant must mint its witness through
/// a comparison. Not caught: an arm pairing the wrong KINDS with a
/// variant, which reaches `broke`, fail-loud.
///
/// Asked of the two names' KINDS and operands only, none of which needs
/// a name resolved to one entity, so the question can precede the tie.
#[derive(Clone, Copy)]
enum DeclaredStep {
    /// Cross-operand Face-Face: the cosurface glue intent, on
    /// whatever carrier the two faces share.
    CrossFaces(sides::CrossOperand),
    /// Same-operand Vertex-Vertex: a carried 3' contact.
    SameVv(sides::SameOperand),
    /// Same-operand Vertex-Face, either way round in the authored
    /// pair: a carried 3' contact.
    SameVf(sides::SameOperand, sides::VertexAndFace),
}

/// The vocabulary itself; see [`DeclaredStep`]. The KINDS pick the
/// shape and the SIDES have to witness it, so a pair its operands
/// cannot support is `None`: [`NodeErrorKind::DeclareUnsupportedPair`].
fn declared_step(
    a: (topo::Operand, names::EntityKind),
    b: (topo::Operand, names::EntityKind),
) -> Option<DeclaredStep> {
    use names::EntityKind::{Face, Vertex};
    let ((oa, ka), (ob, kb)) = (a, b);
    match (ka, kb) {
        (Face, Face) => Some(DeclaredStep::CrossFaces(sides::CrossOperand::of(oa, ob)?)),
        (Vertex, Vertex) => Some(DeclaredStep::SameVv(sides::SameOperand::of(oa, ob)?)),
        (Vertex, Face) | (Face, Vertex) => Some(DeclaredStep::SameVf(
            sides::SameOperand::of(oa, ob)?,
            sides::VertexAndFace::of(ka, kb)?,
        )),
        _ => None,
    }
}

/// **Where a declared name lands in the ONE table its site picked** —
/// rungs 1 and 3 of the declare door's walk, stopped short of rung 2
/// (the order and its reasons: [`ladder`]). The site is the side
/// (DM4), so there is no side to pick.
///
/// # Errors
///
/// Rung 1's `NodeGone` and rung 3's `Vanished`, both through
/// [`NodeErrorKind::DeclareResolve`].
fn declare_landing<'n>(
    sided: &'n SidedName<'n>,
    reference: usize,
    doc: &crate::doc::Doc<ProfileProgram>,
    table: &NameTable,
) -> Result<(ladder::Live<'n>, ladder::Landing), NodeErrorKind> {
    use ladder::Landing;
    let refused = |error| NodeErrorKind::DeclareResolve { error, reference };
    // Rung 1, paid ONCE per name (see [`SidedName`]).
    let live = match sided {
        SidedName::Live(live) => *live,
        SidedName::Rewritten(name) => ladder::live(name, doc).map_err(refused)?,
    };
    let landing = ladder::landing(&live, table);
    if matches!(landing, Landing::Absent) {
        return Err(refused(ladder::vanished(&live)));
    }
    Ok((live, landing))
}

/// The role word a transform's rotation axis is normalized under,
/// shared by the evaluation and the mate solve.
pub(crate) const TRANSFORM_AXIS_ROLE: &str = "transform rotation axis";

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

/// **The rigid map of a placement's rigid step** — the one home of
/// that construction, read only by [`crate::Placement::motion`], which
/// the evaluation and the mate solve's derived offset both read, so a
/// transform under a mate and a transform under the gather move a body
/// by the same arithmetic.
///
/// Rotate about the axis THROUGH THE WORLD ORIGIN by `angle`, then
/// translate. [`Mat3::rotation_about`] re-normalizes the already-unit
/// axis, which on a unit input changes no bit the format holds exactly.
pub(crate) fn transform_map<T: Decide>(
    translation: Vec3<T>,
    axis: UnitVec3<T>,
    angle: T,
) -> Affine3<T> {
    Affine3::from_parts(Mat3::rotation_about(axis.get(), angle), translation)
}

/// **The transform node**: ONE rigid map, its placement's motion
/// ([`crate::Placement::motion`]), shape-preserving over its input's
/// value ([`Placeable`]), so body `i` of a transform of instances is
/// bit for bit what the same map does to that body alone.
///
/// Identity-preserving pass-through (spec D2): the transform
/// contributes NO `RolePath` segment. `transform_rigid` is key-stable,
/// so the input's table holds verbatim — a name still points at the
/// MINTING node (N1), and output-body indices are instance indices.
fn wire_transform<T: Decide + topo::AtRestPolicy>(
    input: RecipeNodeId,
    placement: &crate::placement::Placement,
    results: &Results<T>,
    vals: &SlotValues<T>,
    tol: Tol,
) -> OpResult<T> {
    let value = value_of(results, input)?;
    let placeable = placeable_operand(value, input)?;
    let map = placement.motion(vals, band(tol)?)?;
    let payload = placeable.map(|body, _| place(body, Some(&map), tol))?;
    Ok(OpOut::plain(payload, Arc::clone(&value.name_table)).carrying(value.parts))
}

/// **A world placement** (A10): the body read, at port `port` of the
/// operation `of`, placed at `pose` as one copy named under this node
/// ([`names::name_placed`]). A split's half is its port, as a read of
/// any port is that output. A boolean's empty result places as itself:
/// an empty copy, a typed success (F8) the gather finds no solid in.
///
/// The copy carries the body's declared contact records and the
/// declaration rows keyed with them (ASM-R2b D-1) with their cells
/// verbatim: a rigid placement keeps every arena key
/// (`transform_rigid`), so they key the copy as they keyed the body.
/// Each record cites the body's record it is, through the `body` read.
/// A split's half carries none, as its value carries none.
fn wire_place_in_world<T: Decide + topo::AtRestPolicy>(
    id: RecipeNodeId,
    (of, read): (RecipeNodeId, crate::VarId),
    port: u8,
    pose: &crate::placement::Placement,
    results: &Results<T>,
    vals: &SlotValues<T>,
    tol: Tol,
) -> OpResult<T> {
    let value = value_of(results, of)?;
    let mut contacts = Arc::clone(&value.contacts);
    let mut carried = Arc::clone(&value.carried);
    let (body, index) = match &value.payload {
        ValuePayload::Boolean(BooleanValue::Empty) => {
            return Ok(OpOut::plain(
                ValuePayload::Boolean(BooleanValue::Empty),
                names::empty(),
            ));
        }
        ValuePayload::Boolean(BooleanValue::Body {
            body,
            contacts: records,
            ..
        }) => {
            contacts = Arc::clone(records);
            (Arc::clone(body), 0)
        }
        ValuePayload::Split { above, below } => {
            let half = if port == 0 {
                SplitHalf::Above
            } else {
                SplitHalf::Below
            };
            contacts = Arc::default();
            carried = Arc::default();
            match if port == 0 { above } else { below } {
                SplitSide::Body(b) => (Arc::clone(b), half.output_body()),
                SplitSide::Empty => return Err(NodeErrorKind::EmptyHalf { input: of, half }),
            }
        }
        _ => (read_body(results, of)?, 0),
    };
    let table = value
        .name_table
        .project(index)
        .map_err(|dup| NodeErrorKind::Naming(names::NamingError::from(dup)))?;
    let map = pose.motion_kept(vals, band(tol)?)?.non_identity();
    let placed = place(&body, map.as_ref(), tol)?;
    let table = names::name_placed(id, &table, &placed).map_err(NodeErrorKind::Naming)?;
    Ok(OpOut {
        contacts: Arc::new(contacts.carried_from(0)),
        carried,
        ..OpOut::plain(ValuePayload::Body(Arc::new(placed)), table)
            .carrying(value.parts)
            .citing(vec![crate::coincide::CitedInput::Read(read)])
    })
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
    predicate: &'static str,
    angle: T,
    whole: T,
    band: Band,
) -> Result<Sign, NodeErrorKind> {
    geom_core::k_stats::decide_flagged(predicate, angle.max(-angle) - whole, band, "F14")
        .map_err(escalated(predicate))
}

/// A decision under `predicate` that could not be called, as the
/// node's refusal.
fn escalated(predicate: &'static str) -> impl FnOnce(geom_core::Indeterminate) -> NodeErrorKind {
    move |source| NodeErrorKind::Escalated { predicate, source }
}

/// [`stepped_rule_map`] behind the evaluation's slot reads, which stay
/// INSIDE so a rule's operands are demanded only when a step uses
/// them: placement 0 is the identity and reads none. A listed rule
/// refuses as `listed`, the mismatch it is on the caller's node.
#[allow(clippy::too_many_arguments)] // `doc` resolves a circular rule's axis read
fn stepped_map<T: Decide>(
    kind: &PatternKind,
    doc: &crate::doc::Doc<ProfileProgram>,
    written: &dyn Fn(SlotId) -> crate::Formula,
    listed: crate::node::CountMismatch,
    i: i64,
    results: &Results<T>,
    vals: &SlotValues<T>,
    tol: Tol,
) -> Result<Affine3<T>, NodeErrorKind> {
    let ops = match kind {
        // An explicit rule's frames ARE the maps.
        PatternKind::Explicit(_) => {
            return Err(NodeErrorKind::PlacementRule(
                crate::node::PlacementRuleFault::CountSpelling { shape: listed },
            ));
        }
        _ if i == 0 => return Ok(Affine3::identity()),
        PatternKind::Linear { .. } => SteppedOperands::linear(
            need_vec3(vals, SlotId::Direction)?,
            need_scalar(vals, SlotId::Spacing)?,
            &crate::node::Axis3::ALL.map(|axis| written(SlotId::Direction(axis))),
            band(tol)?,
        )?,
        PatternKind::Circular { axis, .. } => {
            let axis = super::read_at(doc, crate::OperandSlot::Axis, *axis)?;
            let (origin, dir) = operand(results, axis, super::phrase::DATUM_AXIS, |v| {
                match &v.payload {
                    ValuePayload::Datum(DatumValue::Axis { origin, dir }) => Some((origin, dir)),
                    _ => None,
                }
            })?;
            SteppedOperands::circular(
                *origin,
                *dir,
                need_scalar(vals, SlotId::Step)?,
                &written(SlotId::Step),
                band(tol)?,
            )?
        }
    };
    Ok(stepped_rule_map(&ops, i))
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
    doc: &crate::doc::Doc<ProfileProgram>,
    written: &dyn Fn(SlotId) -> crate::Formula,
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
        return Err(NodeErrorKind::NonPositiveCount { count: n });
    }
    let naming = NodeErrorKind::Naming;
    let mut instances: Vec<Arc<Body<T>>> = master.to_vec();
    for j in 1..n {
        let map = stepped_map(
            kind,
            doc,
            written,
            crate::node::CountMismatch::ListedOnPattern,
            j,
            results,
            vals,
            tol,
        )?;
        instances.extend(place_each(master, &map, tol)?);
    }
    let table =
        names::name_pattern(id, &value.name_table, n, master.len(), &instances).map_err(naming)?;
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
    doc: &crate::doc::Doc<ProfileProgram>,
    written: &dyn Fn(SlotId) -> crate::Formula,
    results: &Results<T>,
    vals: &SlotValues<T>,
    tol: Tol,
) -> OpResult<T> {
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
                .map(|i| {
                    stepped_map(
                        kind,
                        doc,
                        written,
                        crate::node::CountMismatch::ListedWithCount,
                        i,
                        results,
                        vals,
                        tol,
                    )
                })
                .collect::<Result<_, _>>()?
        }
    };
    topo::Separation::of(body.as_ref(), tol)
        .map_err(NodeErrorKind::Boolean)?
        .certify(&maps)
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
    let master = Arc::clone(&value_of(results, input)?.name_table);
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

/// A loft section of loops the path lattice constructed (the replay's).
type ConstructedSection = sweep::Section<profile::ConstructedLoop<f64>>;

/// One section of a loft, taken from the RECIPE's own `f64`
/// description rather than from the evaluated `T` payload.
///
/// Structure selection is `f64` (C6/D9): the skinned surface's knots,
/// degrees and control bits must be identical in every scalar lane, so
/// taking the `f64` description is the only way the Interval lane
/// encloses the SAME surface the `f64` lane defines.
fn section_of<T: Decide + geom_core::Bounds + super::SectionScalar>(
    doc: &crate::doc::Doc<ProfileProgram>,
    results: &Results<T>,
    id: RecipeNodeId,
    lane: LaneEnv<'_, T>,
    tol: Tol,
) -> Result<(ConstructedSection, Affine3<f64>, super::ProfilePieces), NodeErrorKind> {
    let program = node_operand(doc, id, super::family::PROFILE, |n| match n {
        Node::Profile(program) => Some(program),
        _ => None,
    })?;
    // THE SEED STOPS HERE, TYPED. The section stays `f64`, so a seed on
    // a parameter this program reads would arrive at the skinned
    // surface as a constant — a finite, wrong zero tangent.
    // Keyed by the variable, which is what the program's readers read.
    if let Some(param) = lane.seed
        && program.reads(param)
    {
        return Err(NodeErrorKind::SeedPinnedSection { section: id, param });
    }
    // LIB-SWITCH §4b: the section is the node's program RESOLVED at
    // `LaneEnv::nominal` and REPLAYED through `prepare_profile`, the
    // profile node's own pipeline, so a bad section reads as a profile
    // error at the NODE (the §2 compatibility contract).
    let resolved = program
        .resolve(lane.nominal)
        .map_err(|(slot, source)| NodeErrorKind::Expr { slot, source })?;
    // DM1c: a section on a DERIVED frame takes the by-value placement,
    // which crosses to `f64` only where the scalar IS `f64`
    // (`SectionScalar`); anywhere else it refuses typed rather than
    // placing on a fabricated point of the frame's bracket.
    let frame = super::read_at(doc, crate::OperandSlot::Frame, program.frame)?;
    let plane = match profile_plane_f64(results, id, frame)? {
        Some(authored) => authored,
        None => {
            let lane_plane = frame_plane_lane(results, frame)?;
            pinned_plane(&lane_plane)
                .ok_or(NodeErrorKind::DerivedFrameSection { profile: id, frame })?
        }
    };
    let pre = prepare_profile(Some(plane), &resolved, &program.ids, tol)?;
    // The lift's second pass runs here too, as a GATE only: the
    // section stays f64, but the certify-or-abort answer must not
    // depend on which node consumes the profile.
    if lane.lift == super::ProfileLift::Guided {
        lane_profile::<T>(program, frame_plane_lane(results, frame)?, lane, &pre, tol)?;
    }
    // `Some` by construction: both arms above passed a placement.
    let place = pre
        .placement_f64
        .ok_or(NodeErrorKind::DerivedFrameSection { profile: id, frame })?
        .placement;
    // The REPLAYED loops in program order (LIB-U3), and the canonical
    // positions' names the skin's walls and seams are named by.
    Ok((pre.profile_f64.into_parts().1, place, pre.pieces))
}

/// A structural (Count) slot, refused typed when absent or unusable.
fn need_count(vals: &SlotValues<impl Decide>, slot: SlotId) -> Result<usize, NodeErrorKind> {
    let n = slots::count(vals, slot).ok_or(NodeErrorKind::MissingSlot { slot })?;
    usize::try_from(n).map_err(|_| NodeErrorKind::NonPositiveCount { count: n })
}

/// The Loft node: the §10.3 walls and their solid assembly.
fn wire_loft<T: Decide + topo::AtRestPolicy + geom_core::Bounds + super::SectionScalar>(
    id: RecipeNodeId,
    profiles: &[RecipeNodeId],
    doc: &crate::doc::Doc<ProfileProgram>,
    results: &Results<T>,
    vals: &SlotValues<T>,
    lane: LaneEnv<'_, T>,
    tol: Tol,
) -> OpResult<T> {
    let v_degree = need_count(vals, SlotId::VDegree)?;
    let mut sections = Vec::with_capacity(profiles.len());
    let mut places = Vec::with_capacity(profiles.len());
    let mut pieces = Vec::with_capacity(profiles.len());
    for pid in profiles {
        let (chain, place, named) = section_of::<T>(doc, results, *pid, lane, tol)?;
        sections.push(chain);
        places.push(place);
        pieces.push(named);
    }
    // Skin refusals keep their own node-error shape (the §2
    // compatibility contract); assembly refusals arrive as `Loft`.
    let built = sweep::loft_body::<T>(&sections, &places, v_degree, tol).map_err(|e| match e {
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

/// The Sweep node: ONE honest arm.
///
/// Every RECIPE door still runs first — the structural slots, and both
/// operands through [`section_of`] — because a Sweep on a datum, or
/// with a bad Count, is a recipe error and must read as one. The
/// geometry is out of reach: see [`SWEEP_FRONTIER`].
fn wire_sweep<T: Decide + geom_core::Bounds + super::SectionScalar>(
    profile: RecipeNodeId,
    path: RecipeNodeId,
    doc: &crate::doc::Doc<ProfileProgram>,
    results: &Results<T>,
    vals: &SlotValues<T>,
    lane: LaneEnv<'_, T>,
    tol: Tol,
) -> OpResult<T> {
    // Each door runs for its refusal alone; what it answers has no
    // reader, because the geometry that would read it does not exist.
    need_count(vals, SlotId::Stations)?;
    need_count(vals, SlotId::VDegree)?;
    section_of::<T>(doc, results, profile, lane, tol)?;
    section_of::<T>(doc, results, path, lane, tol)?;
    Err(NodeErrorKind::CurvedSolidFrontier {
        what: SWEEP_FRONTIER,
    })
}

/// **The lane → `f64` crossing, read directly.** What
/// [`pinned_plane`] carries across where the lane IS `f64`, and what
/// it refuses everywhere else. An evaluation shows only the typed
/// refusal a consumer eventually reports; the crossing's own answer —
/// which twelve components came back, in which places — is readable
/// only here.
#[cfg(test)]
mod pinned_plane_tests {
    #![allow(clippy::unwrap_used, clippy::expect_used, clippy::panic)]

    use super::pinned_plane;
    use geom_core::{Affine3, Dual64, Mat3, Real, Vec3};
    use profile::SketchPlane;

    /// Twelve distinct components and no symmetry, so a crossing that
    /// transposed a column or dropped the translation could not hide
    /// behind an axis coincidence.
    fn distinct() -> SketchPlane<f64> {
        SketchPlane::new(Affine3::from_parts(
            Mat3::from_cols(
                Vec3::new(1.0, 2.0, 3.0),
                Vec3::new(4.0, 5.5, -6.0),
                Vec3::new(-7.25, 0.5, 8.0),
            ),
            Vec3::new(10.0, 11.0, 12.0),
        ))
    }

    /// Where the lane IS `f64`, the crossing is exact and structural:
    /// all twelve components come back in their places, bit for bit.
    ///
    /// The comparison is [`SketchPlane::bit_eq`], which IS that
    /// twelve-component reading — by bits, in their places, off an
    /// irrefutable destructuring of the placement rather than through
    /// any walk. A readout spelled again here would be a second copy of
    /// it, and the transposed-column mutation reds this row through
    /// `bit_eq` exactly as it would through one.
    #[test]
    fn a_f64_lane_placement_crosses_bit_for_bit_in_its_places() {
        let plane = distinct();
        let crossed = pinned_plane(&plane).expect("f64 is the pinned lane");
        assert!(
            plane.bit_eq(&crossed),
            "the crossing keeps the columns and the translation in place"
        );
    }

    /// An analysis scalar refuses, and the refusal is the TYPE's, not
    /// a number's: the placement here is the very one the row above
    /// crossed successfully, lifted component for component, so every
    /// value that pinned at `f64` is present and still unpinnable. No
    /// number is inspected anywhere on the path, and the first
    /// component refuses, so nothing downstream sees a partly-crossed
    /// placement.
    #[test]
    fn an_analysis_scalar_refuses_the_very_placement_f64_crossed() {
        let lane: SketchPlane<Dual64> = distinct().map(Dual64::from_f64);
        assert!(pinned_plane(&lane).is_none());
    }
}

/// **The union's declaration routing, read directly.**
///
/// The buckets [`route_declarations`] fills are not visible in a
/// finished evaluation — a document only shows which pairs RESOLVED —
/// so the rule is read here, where the answer is the bucket list
/// itself. What a document row cannot pin and this can: that a step
/// with no declared pair receives NOTHING, that a pair is fed at
/// exactly one step rather than tried at several, and which SIDE of
/// that step each of its two sites takes.
#[cfg(test)]
mod route_tests {
    #![allow(clippy::unwrap_used, clippy::expect_used, clippy::panic)]

    use super::{
        NodeErrorKind, RecipeNodeId, SidedName, SidedPair, SitedRef, look_through_fold,
        resolve_declarations, route_declarations,
    };
    use crate::names::{
        CapEnd, EntityKey, EntityKind, EntityRef, NameTable, Qualifier, RoleSeg, StableName,
    };
    use crate::node::Node;
    use crate::resolve::{Diagnosis, FoldConsumption, ResolveError};
    use crate::{DocEdit, ProfileDoc};
    use geom_core::Tol;
    use topo::{BooleanCoincidence, Operand};

    /// A live document and `n` live node ids standing in for a union's
    /// members, plus one more for the union itself.
    ///
    /// The members have to be LIVE: a declared entity is named in its
    /// member's own table, so rung 1 — is the minting node still in
    /// the document — is asked of each member before anything is said
    /// about which step the pair belongs to.
    fn doc_with_members(n: usize) -> (ProfileDoc, RecipeNodeId, Vec<RecipeNodeId>) {
        let mut doc = ProfileDoc::empty_derived("union declare routing", Tol::witness());
        let mut ids = Vec::new();
        for _ in 0..=n {
            let applied = doc
                .apply(
                    &DocEdit::InsertNode {
                        node: Box::new(Node::Datum(crate::node::Datum::Plane {
                            origin: [0.0; 3].map(crate::test_support::len),
                            normal: [0.0, 0.0, 1.0].map(crate::test_support::scl),
                        })),
                        fresh: Vec::new(),
                    },
                    Tol::witness(),
                    &crate::mate::RefusingReach,
                )
                .expect("a datum plane inserts");
            ids.push(applied.record.minted.expect("the insert minted an id"));
            doc = applied.doc;
        }
        let union = ids.remove(0);
        (doc, union, ids)
    }

    /// One entity of one member, in the MEMBER's own name space — what
    /// a declaration names, with the member beside it.
    fn at(member: RecipeNodeId, end: CapEnd) -> SitedRef {
        SitedRef::new(
            member,
            StableName {
                kind: EntityKind::Face,
                node: member,
                path: vec![RoleSeg::Cap(end)],
            },
        )
    }

    /// The same entity as the UNION spells it — the row `member_view`
    /// puts into that member's operand table, which is what the
    /// routing rewrites a sited pair into.
    fn member_face(union: RecipeNodeId, member: RecipeNodeId, end: CapEnd) -> StableName {
        crate::names::member_name(union, member, &at(member, end).name)
    }

    /// The same entity, but MINTED somewhere other than the member it
    /// is read at — the pass-through case (N1: a transform adds no
    /// segment, so its table is its input's verbatim). It is the one
    /// shape that tells a routing by SITE from a routing by the name's
    /// minting node.
    fn at_minted_at(member: RecipeNodeId, mint: RecipeNodeId, end: CapEnd) -> SitedRef {
        SitedRef::new(
            member,
            StableName {
                kind: EntityKind::Face,
                node: mint,
                path: vec![RoleSeg::Cap(end)],
            },
        )
    }

    fn pair(a: SitedRef, b: SitedRef) -> ((SitedRef, SitedRef), BooleanCoincidence) {
        ((a, b), BooleanCoincidence::REST)
    }

    /// **The routing reads the SITE, not the name's minting node.**
    /// Both sides here name entities minted at a node that is in no
    /// member list; their sites are members 1 and 3, so the pair lands
    /// in step 2 and each side is rewritten under its own member.
    #[test]
    fn a_pair_routes_by_its_site_and_not_by_the_names_minting_node() {
        let (doc, union, ms) = doc_with_members(5);
        let (members, proto) = (&ms[..4], ms[4]);
        let p = pair(
            at_minted_at(members[1], proto, CapEnd::Start),
            at_minted_at(members[3], proto, CapEnd::End),
        );
        let buckets = route_declarations(union, members, std::slice::from_ref(&p), &doc)
            .expect("the sites are both members");
        let filled: Vec<usize> = buckets
            .iter()
            .enumerate()
            .filter(|(_, b)| !b.is_empty())
            .map(|(k, _)| k)
            .collect();
        assert_eq!(filled, vec![2]);
        let ((o1, n1, _), (o2, n2, _), _) = &buckets[2][0];
        assert_eq!((*o1, *o2), (Operand::A, Operand::B));
        assert_eq!(
            *n1.name(),
            crate::names::member_name(
                union,
                members[1],
                &at_minted_at(members[1], proto, CapEnd::Start).name
            )
        );
        assert_eq!(
            *n2.name(),
            crate::names::member_name(
                union,
                members[3],
                &at_minted_at(members[3], proto, CapEnd::End).name
            )
        );
    }

    /// The bucket of each pair on a FOUR-member document, read out of
    /// the routing itself.
    #[test]
    fn each_pair_is_routed_to_the_one_step_that_joins_its_two_sites() {
        let (doc, union, ms) = doc_with_members(4);
        let (m0, m1, m2, m3) = (ms[0], ms[1], ms[2], ms[3]);
        let face = |m| at(m, CapEnd::Start);
        let other = |m| at(m, CapEnd::End);

        let cases = vec![
            // Two members meet at the LATER one's step.
            ("members 0 and 1", pair(face(m0), face(m1)), 0),
            ("members 1 and 2", pair(face(m1), face(m2)), 1),
            // Order within the pair is not a fact about the step.
            ("members 2 and 1", pair(face(m2), face(m1)), 1),
            // The list's own gap: (1, 3) is fed at step 3 and step 2
            // receives nothing, which is the half a resolved-or-not
            // document cannot show.
            ("members 1 and 3", pair(face(m1), face(m3)), 2),
            // Two entities of ONE member are that member's carried
            // contact, at its own step — member 0's at step 0, where it
            // is the accumulation, exactly as in the pair chain.
            ("member 2 with itself", pair(face(m2), other(m2)), 1),
            ("member 0 with itself", pair(face(m0), other(m0)), 0),
        ];

        for (what, p, want) in cases {
            let buckets = route_declarations(union, &ms, std::slice::from_ref(&p), &doc)
                .unwrap_or_else(|e| panic!("{what} routed nowhere: {e:?}"));
            let filled: Vec<usize> = buckets
                .iter()
                .enumerate()
                .filter(|(_, b)| !b.is_empty())
                .map(|(i, _)| i)
                .collect();
            assert_eq!(filled, vec![want], "{what}");
            assert_eq!(buckets[want].len(), 1, "{what}");
        }

        // All of them at once: three steps, and the pair that belongs
        // to none of the first two leaves step 2 empty.
        let all = vec![
            pair(face(m0), face(m1)),
            pair(face(m1), face(m3)),
            pair(face(m2), other(m2)),
        ];
        let buckets = route_declarations(union, &ms, &all, &doc).expect("all three route");
        let sizes: Vec<usize> = buckets.iter().map(Vec::len).collect();
        assert_eq!(sizes, vec![1, 1, 1]);
    }

    /// **Every sited pair has a step**, which is what the sited payload
    /// bought: a declaration can only name entities that exist BEFORE
    /// the union, so there is no pair the routing accepts and then has
    /// nowhere to feed. The bucket is `max(i, j) - 1`, and `max(i, j)`
    /// is at most the last member's index, so the bucket is always one
    /// the fold runs.
    #[test]
    fn every_pair_of_member_sites_lands_in_a_step_of_the_fold() {
        let (doc, union, ms) = doc_with_members(5);
        let steps = ms.len() - 1;
        for i in 0..ms.len() {
            for j in 0..ms.len() {
                let p = pair(at(ms[i], CapEnd::Start), at(ms[j], CapEnd::End));
                let buckets = route_declarations(union, &ms, std::slice::from_ref(&p), &doc)
                    .unwrap_or_else(|e| panic!("({i}, {j}) routed nowhere: {e:?}"));
                let filled: Vec<usize> = buckets
                    .iter()
                    .enumerate()
                    .filter(|(_, b)| !b.is_empty())
                    .map(|(k, _)| k)
                    .collect();
                assert_eq!(filled, vec![i.max(j).saturating_sub(1)], "({i}, {j})");
                assert_eq!(buckets.len(), steps);
            }
        }
    }

    /// **The site is the side.** The member joining at a step is that
    /// step's operand B; every member the fold has already accumulated
    /// is operand A. Read off the routed bucket, because nothing
    /// downstream re-derives it — the resolver reads the side the
    /// routing decided.
    #[test]
    fn the_joining_member_is_operand_b_and_the_accumulation_is_operand_a() {
        let (doc, union, ms) = doc_with_members(4);
        let sided = |p| {
            let buckets =
                route_declarations(union, &ms, std::slice::from_ref(&p), &doc).expect("routes");
            buckets.into_iter().flatten().next().expect("one bucket")
        };
        // An earlier member against a later one: A then B, and each
        // name is rewritten into the union's member space.
        let ((o1, n1, _), (o2, n2, _), _) =
            sided(pair(at(ms[1], CapEnd::Start), at(ms[3], CapEnd::End)));
        assert_eq!(o1, Operand::A);
        assert_eq!(o2, Operand::B);
        assert_eq!(*n1.name(), member_face(union, ms[1], CapEnd::Start));
        assert_eq!(*n2.name(), member_face(union, ms[3], CapEnd::End));
        // The same pair written the other way round: the SITES decide,
        // not the order the author wrote them in.
        let ((o1, ..), (o2, ..), _) = sided(pair(at(ms[3], CapEnd::End), at(ms[1], CapEnd::Start)));
        assert_eq!((o1, o2), (Operand::B, Operand::A));
        // One member with itself is that member's CARRIED contact, on
        // the side it enters the step as: operand B at its own step,
        // and operand A for member 0, which is where the fold starts.
        let ((o1, ..), (o2, ..), _) = sided(pair(at(ms[2], CapEnd::Start), at(ms[2], CapEnd::End)));
        assert_eq!((o1, o2), (Operand::B, Operand::B));
        let ((o1, ..), (o2, ..), _) = sided(pair(at(ms[0], CapEnd::Start), at(ms[0], CapEnd::End)));
        assert_eq!((o1, o2), (Operand::A, Operand::A));
    }

    /// **A site the member list does not hold refuses through the N5
    /// ladder**, as a vanished name does — the state `SetMembers`
    /// creates by dropping a declared member, and never a silent drop.
    #[test]
    fn a_site_outside_the_member_list_refuses_as_a_vanished_name() {
        let (doc, union, ms) = doc_with_members(4);
        // A live node that is simply not in the list.
        let outsider = ms[3];
        let members = &ms[..3];
        let p = pair(at(ms[0], CapEnd::Start), at(outsider, CapEnd::End));
        let refused = route_declarations(union, members, std::slice::from_ref(&p), &doc);
        assert!(
            matches!(
                refused,
                Err(NodeErrorKind::DeclareResolve { ref error, .. })
                    if matches!(**error, crate::resolve::ResolveError::Vanished { .. })
            ),
            "{refused:?}",
        );
    }

    /// **Rung 1 outranks the site question**: a name whose minting node
    /// the document no longer holds says `NodeGone`, before anything is
    /// said about which step it would have belonged to.
    #[test]
    fn a_dead_minting_node_outranks_a_site_outside_the_list() {
        let (doc, union, ms) = doc_with_members(4);
        let gone = ms[3];
        let doc = doc
            .apply(
                &DocEdit::DeleteNode { id: gone },
                Tol::witness(),
                &crate::mate::RefusingReach,
            )
            .expect("the datum plane deletes")
            .doc;
        let p = pair(at(ms[0], CapEnd::Start), at(gone, CapEnd::End));
        let refused = route_declarations(union, &ms[..3], std::slice::from_ref(&p), &doc);
        assert!(
            matches!(
                refused,
                Err(NodeErrorKind::DeclareResolve { ref error, .. })
                    if matches!(**error, crate::resolve::ResolveError::NodeGone { .. })
            ),
            "{refused:?}",
        );
    }

    /// A real face key for a hand-built table — a unit cube's top cap.
    /// Nothing below reads the body; the door reads tables.
    fn a_face_key() -> topo::FaceKey {
        use profile::RawLoop;
        let plane = profile::SketchPlane::from_frame(geom_core::OrthoFrame::axes_xy(
            geom_core::Point3::new(0.0, 0.0, 0.0),
        ));
        let square = profile::ProfileLoop::polygon(
            [(0.0, 0.0), (1.0, 0.0), (1.0, 1.0), (0.0, 1.0)]
                .into_iter()
                .map(|(x, y)| geom_core::Point2::new(x, y)),
        );
        let profile = profile::Profile::new(plane, vec![square])
            .validate(Tol::witness())
            .unwrap();
        sweep::extrude(
            &profile,
            sweep::Extrusion::Distance {
                depth: 1.0_f64,
                side: crate::ExtrudeSide::Along,
            },
            Tol::witness(),
        )
        .unwrap()
        .top
    }

    fn face_ref(key: topo::FaceKey) -> EntityRef {
        EntityRef {
            body: 0,
            key: EntityKey::Face(key),
        }
    }

    /// A routed pair, as the bucket carries it: the two names already
    /// in the union's member space, each with the side its site took.
    fn routed(a: (Operand, StableName), b: (Operand, StableName)) -> SidedPair<'static> {
        (
            (a.0, SidedName::Rewritten(a.1), 0),
            (b.0, SidedName::Rewritten(b.1), 1),
            BooleanCoincidence::REST,
        )
    }

    /// A merged row over member faces, as a fold step mints it: flat,
    /// sorted, in the union's own space.
    fn merged(set: Vec<StableName>) -> StableName {
        let mut set = set;
        set.sort();
        StableName {
            kind: EntityKind::Face,
            node: RecipeNodeId::new(0, 0),
            path: vec![RoleSeg::Merged(set)],
        }
    }

    /// `name`, minted by `node` instead.
    fn renode(node: RecipeNodeId, mut name: StableName) -> StableName {
        name.node = node;
        name
    }

    /// The rewrite touches ONE shape: an ACCUMULATION-side member face
    /// that is no row and sits in a merged row's set goes to that row.
    /// A face the accumulation still holds, and a face no row holds or
    /// descends from, are handed on as written — and the JOINING
    /// member's side never looks through, because no merge the fold has
    /// performed could have consumed a face of a member that has not
    /// joined yet.
    #[test]
    fn look_through_rewrites_only_an_accumulated_member_face_inside_a_merged_row() {
        let (_doc, union, ms) = doc_with_members(4);
        let key = a_face_key();
        let f = |m, e| member_face(union, m, e);
        // Step 2's row: the merge of step 1's `{m0, m1}` with `m2`,
        // flat, minted in the union's space.
        let wide = renode(
            union,
            merged(vec![
                f(ms[0], CapEnd::Start),
                f(ms[1], CapEnd::Start),
                f(ms[2], CapEnd::Start),
            ]),
        );
        let mut acc = NameTable::new();
        acc.insert(wide.clone(), face_ref(key)).unwrap();
        acc.insert(
            f(ms[1], CapEnd::End),
            EntityRef {
                body: 1,
                key: EntityKey::Face(key),
            },
        )
        .unwrap();
        let mut member = NameTable::new();
        member
            .insert(f(ms[3], CapEnd::Start), face_ref(key))
            .unwrap();
        let bucket = vec![
            // Merged away at an earlier step: rewritten.
            routed(
                (Operand::A, f(ms[0], CapEnd::Start)),
                (Operand::B, f(ms[3], CapEnd::Start)),
            ),
            // Still a row of the accumulation: untouched.
            routed(
                (Operand::A, f(ms[1], CapEnd::End)),
                (Operand::B, f(ms[3], CapEnd::Start)),
            ),
            // In no table and in no set, and nothing descends from it:
            // untouched, and the door below refuses it as a vanished
            // name.
            routed(
                (Operand::A, f(ms[2], CapEnd::End)),
                (Operand::B, f(ms[3], CapEnd::Start)),
            ),
        ];
        let out = look_through_fold(&bucket, &acc).unwrap();
        assert_eq!(out[0].0, (Operand::A, SidedName::Rewritten(wide), 0));
        assert_eq!(out[1], bucket[1]);
        assert_eq!(out[2], bucket[2]);
    }

    /// The joining member's side is never looked through: its face is
    /// in the accumulation's merged row's set only in a table no fold
    /// can produce, and the door must not rewrite it there.
    #[test]
    fn the_joining_members_side_never_looks_through() {
        let (_doc, union, ms) = doc_with_members(4);
        let f = |m, e| member_face(union, m, e);
        let row = renode(
            union,
            merged(vec![f(ms[0], CapEnd::Start), f(ms[3], CapEnd::Start)]),
        );
        let mut acc = NameTable::new();
        acc.insert(row, face_ref(a_face_key())).unwrap();
        let p = routed(
            (Operand::A, f(ms[1], CapEnd::End)),
            (Operand::B, f(ms[3], CapEnd::Start)),
        );
        let out = look_through_fold(std::slice::from_ref(&p), &acc).unwrap();
        assert_eq!(out[0], p);
    }

    /// One member face in TWO merged rows' sets — a table the flat
    /// mint cannot produce — is refused as the emission bug it is,
    /// never resolved to whichever row came first.
    #[test]
    fn a_member_face_in_two_merged_rows_refuses_as_an_emission_bug() {
        let (_doc, union, ms) = doc_with_members(4);
        let key = a_face_key();
        let f = |m, e| member_face(union, m, e);
        let mut acc = NameTable::new();
        acc.insert(
            renode(
                union,
                merged(vec![f(ms[0], CapEnd::Start), f(ms[1], CapEnd::Start)]),
            ),
            face_ref(key),
        )
        .unwrap();
        // A second entity for the second row: the table refuses two
        // names on one entity, and the shape under test is two rows.
        acc.insert(
            renode(
                union,
                merged(vec![f(ms[0], CapEnd::Start), f(ms[2], CapEnd::Start)]),
            ),
            EntityRef {
                body: 1,
                key: EntityKey::Face(key),
            },
        )
        .unwrap();
        let p = routed(
            (Operand::A, f(ms[0], CapEnd::Start)),
            (Operand::B, f(ms[3], CapEnd::Start)),
        );
        let refused = look_through_fold(std::slice::from_ref(&p), &acc);
        assert!(
            matches!(
                refused,
                Err(NodeErrorKind::Naming(crate::names::NamingError::Emission { what }))
                    if what == super::MEMBER_FACE_IN_TWO_MERGES
            ),
            "{refused:?}"
        );
    }

    /// The `rank`-th fragment of `name`, as a fold step mints one.
    fn fragment(name: StableName, rank: u32) -> StableName {
        let mut name = name;
        name.path
            .push(RoleSeg::Fragment(Qualifier::OrderAlong { rank, of: 2 }));
        name
    }

    /// A table holding `rows`, one entity each.
    fn table_of(rows: Vec<StableName>) -> NameTable {
        let key = a_face_key();
        let mut t = NameTable::new();
        for (body, row) in (0u32..).zip(rows) {
            t.insert(
                row,
                EntityRef {
                    body,
                    key: EntityKey::Face(key),
                },
            )
            .unwrap();
        }
        t
    }

    /// The composition a refused A-side name reports, and the name it
    /// reports it for.
    fn consumed_by(
        refused: Result<Vec<SidedPair<'_>>, NodeErrorKind>,
    ) -> (StableName, FoldConsumption) {
        match refused {
            Err(NodeErrorKind::DeclareResolve { error, .. }) => match *error {
                ResolveError::Vanished {
                    name,
                    diagnosis: Diagnosis::ConsumedByFold { by },
                    last_good: None,
                } => (name, by),
                other => panic!("not a fold consumption: {other:?}"),
            },
            other => panic!("not a declare refusal: {other:?}"),
        }
    }

    /// **Each composition the fold consumes a member face by refuses,
    /// naming that composition** — read off the rows that descend from
    /// the name, and nothing re-pointed.
    ///
    /// Split: the name's own fragments, bare or merged later.
    /// Fragmented merge: fragments of a merged row whose set covers it
    /// — the several fragments of ONE merge, which is not the two-merges
    /// emission bug.
    #[test]
    fn a_member_face_consumed_other_than_by_a_merge_refuses_naming_the_composition() {
        let (_doc, union, ms) = doc_with_members(4);
        let f = |m, e| member_face(union, m, e);
        let named = f(ms[0], CapEnd::End);
        let pair = routed(
            (Operand::A, named.clone()),
            (Operand::B, f(ms[3], CapEnd::End)),
        );
        let merged_u = |set| renode(union, merged(set));
        let cases = [
            (
                "split, both fragments rows",
                table_of(vec![fragment(named.clone(), 0), fragment(named.clone(), 1)]),
                FoldConsumption::Split,
            ),
            (
                "split, one fragment merged later",
                table_of(vec![
                    fragment(named.clone(), 0),
                    merged_u(vec![fragment(named.clone(), 1), f(ms[1], CapEnd::End)]),
                ]),
                FoldConsumption::Split,
            ),
            (
                "split and partly merged in one step",
                table_of(vec![
                    fragment(named.clone(), 0),
                    merged_u(vec![named.clone(), f(ms[1], CapEnd::End)]),
                ]),
                FoldConsumption::Split,
            ),
            (
                "a merge fragmented after it",
                table_of(vec![
                    fragment(merged_u(vec![named.clone(), f(ms[1], CapEnd::End)]), 0),
                    fragment(merged_u(vec![named.clone(), f(ms[1], CapEnd::End)]), 1),
                ]),
                FoldConsumption::FragmentedMerge,
            ),
            (
                "a fragment of that merge merged again",
                table_of(vec![merged_u(vec![
                    fragment(merged_u(vec![named.clone(), f(ms[1], CapEnd::End)]), 0),
                    f(ms[2], CapEnd::End),
                ])]),
                FoldConsumption::FragmentedMerge,
            ),
        ];
        for (label, acc, want) in cases {
            let refused = look_through_fold(std::slice::from_ref(&pair), &acc);
            assert_eq!(consumed_by(refused), (named.clone(), want), "{label}");
        }
    }

    /// **Every piece merged again still names the composition that made
    /// the pieces.** With no bare fragment left, only the descent
    /// through a merged row's constituents reaches the name, and it
    /// reports what the constituent says — the split, or the fragmented
    /// merge — never the later merge it was read through.
    ///
    /// No real document has been built that reaches this shape: the
    /// step that merges a fragment again is fed a declared pair naming
    /// a face that fragment descends from, and that pair meets the bare
    /// fragment and refuses first.
    #[test]
    fn every_piece_merged_again_still_names_the_composition_that_made_it() {
        let (_doc, union, ms) = doc_with_members(4);
        let f = |m, e| member_face(union, m, e);
        let named = f(ms[0], CapEnd::End);
        let pair = routed(
            (Operand::A, named.clone()),
            (Operand::B, f(ms[3], CapEnd::End)),
        );
        let merged_u = |set| renode(union, merged(set));
        let inner = || merged_u(vec![named.clone(), f(ms[1], CapEnd::End)]);
        let cases = [
            (
                "every fragment of the name merged later",
                table_of(vec![
                    merged_u(vec![fragment(named.clone(), 0), f(ms[1], CapEnd::End)]),
                    merged_u(vec![fragment(named.clone(), 1), f(ms[2], CapEnd::End)]),
                ]),
                FoldConsumption::Split,
            ),
            (
                "every fragment of a merge over it merged again",
                table_of(vec![
                    merged_u(vec![fragment(inner(), 0), f(ms[2], CapEnd::End)]),
                    merged_u(vec![fragment(inner(), 1), f(ms[2], CapEnd::Start)]),
                ]),
                FoldConsumption::FragmentedMerge,
            ),
        ];
        for (label, acc, want) in cases {
            let refused = look_through_fold(std::slice::from_ref(&pair), &acc);
            assert_eq!(consumed_by(refused), (named.clone(), want), "{label}");
        }
    }

    /// **A fragment of the name under another node is not a piece of
    /// it.** A row carrying the name's path under ANOTHER node — a
    /// different union's member-space name for the same member face —
    /// fragmented, does not descend from the name: nothing consumed the
    /// name, so the pair is handed on as written, and the door refuses
    /// it as a vanished name.
    #[test]
    fn a_fragment_of_another_nodes_name_is_not_a_piece_of_it() {
        let (_doc, union, ms) = doc_with_members(4);
        let f = |m, e| member_face(union, m, e);
        let named = f(ms[0], CapEnd::End);
        let pair = routed(
            (Operand::A, named.clone()),
            (Operand::B, f(ms[3], CapEnd::End)),
        );
        let elsewhere = renode(ms[1], named);
        let acc = table_of(vec![fragment(elsewhere.clone(), 0), fragment(elsewhere, 1)]);
        let out = look_through_fold(std::slice::from_ref(&pair), &acc);
        assert_eq!(out.unwrap(), vec![pair]);
    }

    /// Rows that descend from one member face by two compositions — a
    /// table the retiring mint cannot produce — refuse as the emission
    /// bug they are, never as whichever of the two rows was read first.
    #[test]
    fn a_member_face_consumed_two_ways_refuses_as_an_emission_bug() {
        let (_doc, union, ms) = doc_with_members(4);
        let f = |m, e| member_face(union, m, e);
        let named = f(ms[0], CapEnd::End);
        let acc = table_of(vec![
            fragment(named.clone(), 0),
            fragment(
                renode(union, merged(vec![named.clone(), f(ms[1], CapEnd::End)])),
                0,
            ),
        ]);
        let pair = routed((Operand::A, named), (Operand::B, f(ms[3], CapEnd::End)));
        let refused = look_through_fold(std::slice::from_ref(&pair), &acc);
        assert!(
            matches!(
                refused,
                Err(NodeErrorKind::Naming(crate::names::NamingError::Emission { what }))
                    if what == super::MEMBER_FACE_CONSUMED_TWO_WAYS
            ),
            "{refused:?}"
        );
    }

    /// Two FACES of one operand are outside the v1 threading
    /// vocabulary, and the refusal says so with `cross_operand` false —
    /// the fact the two sites decide. A same-operand pair reaches this
    /// door whenever both sites name the one member the step joins.
    #[test]
    fn two_faces_of_one_operand_refuse_as_an_unsupported_pair() {
        let (doc, union, ms) = doc_with_members(4);
        let f = |m, e| member_face(union, m, e);
        let key = a_face_key();
        let mut member = NameTable::new();
        member
            .insert(f(ms[1], CapEnd::Start), face_ref(key))
            .unwrap();
        member
            .insert(
                f(ms[1], CapEnd::End),
                EntityRef {
                    body: 1,
                    key: EntityKey::Face(key),
                },
            )
            .unwrap();
        let acc = NameTable::new();
        let p = routed(
            (Operand::B, f(ms[1], CapEnd::Start)),
            (Operand::B, f(ms[1], CapEnd::End)),
        );
        let refused = resolve_declarations(
            std::slice::from_ref(&p),
            &doc,
            (&acc, None),
            (&member, None),
        );
        assert!(
            matches!(
                refused,
                Err(NodeErrorKind::DeclareUnsupportedPair {
                    kinds: (EntityKind::Face, EntityKind::Face),
                    cross_operand: false,
                })
            ),
            "{refused:?}",
        );
    }
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
                    bits(stepped_rule_map(&followed, i).components()),
                    "copy {i} of {reversed:?} lands where the negative spacing put it"
                );
            }
        }
    }
}
