//! **Four jobs**: node-to-kernel wiring, the mid-evaluation name
//! ladder, the declaration routing that has no kernel op to wire to,
//! and the placement-rule arithmetic two modules share.
//!
//! **The wiring** (spec D3: wire, don't invent). Each F4 node that
//! names a geometric operation maps to an EXISTING public kernel op;
//! every editor-side geometric judgment (direction normalization, the
//! revolve axis's in-plane projection, full-vs-partial classification)
//! goes through the kernel's decided-predicate door, never a raw
//! comparison. Which value a node's operand is allowed to be, and what
//! it is told when it is not, is one door ([`operand`]) speaking one
//! vocabulary ([`super::family`], [`super::phrase`]).
//!
//! **The mid-evaluation name ladder** ([`ladder`]). Every door that
//! resolves an AUTHORED name against the tables THIS run has built so
//! far — the blend selection, a shell's open faces, a face frame, the
//! declare door, a measure's references — asks the same three
//! questions, and [`ladder::Live`] is the token that makes rung 1's
//! place a type rather than a convention. The one-table doors ask
//! them 1, 2, 3; the DECLARE door interleaves its own pair question
//! and asks 1, 3, kind, 2 ([`resolve_declarations`]). It maps to no
//! kernel op either: the kernel takes entity keys, and everything that
//! turns an authored name into one, or into an N5 refusal, is this
//! module's. What a resolved name DENOTES is the question after it,
//! and it has one door too: [`named_entity`] over
//! [`super::entity_door`]. A road supplies the projection and its own
//! refusal and CANNOT supply the word for the kind it found — that
//! word arrives as a token only the door can mint.
//!
//! **The declaration routing.** A union's declared face pairs are
//! SITED at its members and consumed by a fold of pairwise booleans,
//! so something must decide which step of the fold each pair belongs
//! to and which side of that step each of its two sites takes. That is
//! [`side_by_operand`] for the pair boolean, [`route_declarations`]
//! and [`look_through_merges`] for the union, and the refusal menu
//! beneath them, in a vocabulary of their own (buckets, sides,
//! look-through). No
//! kernel op is behind any of it: the kernel takes a
//! [`BooleanDeclarations`] already resolved to operands and entity
//! keys, and every decision about which authored pair resolves where
//! is made here. Its consumers are the boolean and union ops wired
//! above it, and its refusals are theirs.
//!
//! **The placement-rule arithmetic.** [`transform_map`],
//! [`SteppedOperands`], [`stepped_rule_map`] and the direction-role
//! words beside them ([`TRANSFORM_AXIS_ROLE`],
//! [`PATTERN_DIRECTION_ROLE`], [`DATUM_AXIS_ROLE`]) are the ONE
//! spelling of "where does a placer put instance `i`", and they are
//! `pub(crate)` because the second reader is not here: the mate
//! solve's derived offset (`crate::mate::member`) re-derives a
//! placer's map from the recipe and must get the same affine and the
//! same refusal words as the evaluation does. A second spelling would
//! be a body that a mate and a gather disagree about the position of.

use std::collections::BTreeMap;
use std::sync::Arc;

use geom_brep::OutwardNormal;
use geom_core::{
    Affine3, Band, Decide, Mat3, OrthoAxis, OrthoFrame, Point2, Point3, Sign, Tol, UnitVec3,
    UnitVec3Error, Vec2, Vec3,
};
use sweep::blend::BlendKind;
use sweep::{Revolution, RevolveAxis};
use topo::splitting::SplitPart;
use topo::transform::transform_rigid;
use topo::{
    Body, BooleanDeclarations, CarriedContacts, CarriedVf, CarriedVv, ContactClass,
    DATUM_UNIT_NORM, FacePairDeclaration, GeomSource, VfContact, VvContact,
};

use super::anchor::{self, ProfilePre, ProfileValue};
use super::slots::{self, SlotValues};
use super::{BooleanValue, DatumValue, NodeErrorKind, NodeResult, SplitSide, ValuePayload};
use crate::names::{self, NameTable, SplitHalf};
use crate::node::{
    Axis3, BooleanOp, Datum, Node, PartSelect, PatternKind, RecipeNodeId, SitedRef, SlotId,
};
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
    /// The fragment groups the op's emitter formed
    /// (`names::FragmentGroups`); empty for an op that forms none.
    pub groups: Arc<names::FragmentGroups>,
    pub contacts: Arc<topo::ContactRecords>,
    /// Whose mate authored each of those records, and which mates of
    /// the documents below could not be minted at all — the same
    /// channel's bookkeeping half, in the same arena and filled at the
    /// same one op.
    pub carried: Arc<crate::assembly::CarriedDeclarations>,
}

impl<T: Decide> OpOut<T> {
    /// An op that declares no contact — every op but instantiate (see
    /// the type docs for why a boolean is not an exception).
    fn plain(payload: ValuePayload<T>, names: Arc<NameTable>) -> Self {
        Self {
            payload,
            names,
            groups: Arc::default(),
            contacts: Arc::new(topo::ContactRecords::default()),
            carried: Arc::new(crate::assembly::CarriedDeclarations::default()),
        }
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
/// elaborated over. They travel together because they are one
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
    /// The document's own f64 parameter environment, under no box and
    /// no seed, built once per evaluation beside `params`. THE CENSUS
    /// of its readers — every f64-pinned decision the evaluation makes:
    /// the nominal slot list `eval_node` evaluates for every node,
    /// which is what an authored frame's placement is minted from
    /// ([`mint_frame_placement`]) and what every profile drawn on that
    /// frame then READS ([`profile_plane_f64`]); the pre-pass's
    /// resolution of a profile node's program (`eval_node`); a loft or
    /// sweep section's resolution of its program ([`section_of`]); and
    /// the placement the pinned lift embeds. It is therefore what a
    /// content key owes ([`super::tag::slot`]). Nothing under
    /// evaluation builds a second one.
    pub nominal: &'a crate::expr::ParamEnv<f64>,
    /// The E4 seed this evaluation carries, by name (`None` on the
    /// build path). Consulted by the one place the lift cannot reach:
    /// a C6/D9-pinned section refuses a seed it would otherwise embed
    /// as a constant ([`section_of`]).
    pub seed: Option<&'a crate::doc::ParamName>,
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
/// inside the node's verdict frame and ahead of this op).
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
        + crate::analysis::AxisScalar
        + crate::analysis::SeedScalar
        + crate::measure::MinClearanceLane
        + super::SectionScalar
        + crate::lane::Lane,
{
    match node {
        Node::Datum(d) => Ok(OpOut::plain(
            wire_datum(d, doc, results, vals, tol)?,
            names::empty(),
        )),
        Node::Profile(program) => Ok(OpOut::plain(
            wire_profile(program, results, profile_pre, env.lane, tol)?,
            names::empty(),
        )),
        Node::Extrude { profile, .. } => wire_extrude(id, *profile, doc, results, vals, env, tol),
        Node::Revolve { profile, axis, .. } => {
            wire_revolve(id, *profile, *axis, doc, results, vals, env, tol)
        }
        Node::Loft { profiles, .. } => wire_loft(id, profiles, doc, results, vals, env.lane, tol),
        Node::Sweep { profile, path, .. } => {
            wire_sweep(*profile, *path, doc, results, vals, env.lane, tol)
        }
        // Two arms, two doors, no flag between them: which node kind
        // this is decides which kernel door runs, and nothing else
        // does. That is the split vocabulary's whole content.
        Node::Tube { spine, window, .. } => wire_tube(id, *spine, window, results, vals, tol),
        Node::HollowTube { spine, window, .. } => {
            wire_hollow_tube(id, *spine, window, results, vals, tol)
        }
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
            env,
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
            env,
            tol,
        ),
        Node::Shell { target, open, .. } => wire_shell(
            &crate::verbs::shell::shell(),
            id,
            *target,
            open,
            doc,
            results,
            vals,
            env,
            tol,
        ),
        Node::Split { target, tool } => wire_split(
            &crate::verbs::split::split(),
            id,
            *target,
            *tool,
            results,
            tol,
        ),
        Node::Boolean { op, a, b, declare } => wire_boolean(
            &crate::verbs::boolean::boolean(),
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
        Node::Union { members, declare } => wire_union(
            &crate::verbs::boolean::boolean(),
            id,
            members,
            *declare,
            doc,
            results,
            env.boolean_sweep,
            tol,
        ),
        Node::Transform { input, .. } => wire_transform(id, *input, results, vals, tol),
        Node::Pattern { input, kind, .. } => wire_pattern(id, *input, kind, results, vals, tol),
        // No `id`: the projection mints no description and no name, so
        // nothing it produces is stamped or keyed by this node.
        Node::Part { of, select } => wire_part(*of, select, results, vals),
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
        } => {
            // **Which endpoints this assertion may read** (M10-6/R1).
            // Structural, off the referenced measure's own expression,
            // so it is the same answer at every scalar; a reference
            // that does not name a measure falls through to
            // `wire_assertion`'s existing typed `WrongOperand`.
            let certified = match doc.node(*measure) {
                Some(Node::Measure { expr, .. }) => expr.certified(),
                _ => crate::measure::Certified::Enclosure,
            };
            wire_assertion(
                *measure,
                bound,
                *dir,
                certified,
                payload_values,
                results,
                tol,
            )
        }
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
        + crate::analysis::AxisScalar
        + crate::analysis::SeedScalar
        + crate::measure::MinClearanceLane
        + super::SectionScalar
        + crate::lane::Lane,
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
        let crate::node::InterfaceCrossing::Mate { outer, inner, .. } = crossing;
        if part.names.lookup(inner).is_none() {
            return Err(NodeErrorKind::CrossingUnverified {
                instance: id,
                outer: Box::new(outer.clone()),
                name: Box::new((**inner).clone()),
            });
        }
    }
    // The identity fast-path is admitted only for a BIT-exact identity
    // frame: any other value could round, and `transform_rigid` is what
    // decides whether it stayed rigid.
    let map = (!placement.is_identity_bits()).then(|| placement.affine::<T>());
    let placed = place(&part.body, map.as_ref(), id, 0, tol)?;
    let table = names::name_in_part(id, &part.names, &placed).map_err(NodeErrorKind::Naming)?;
    // ASM-R2b D-1: the part's OWN declared contacts survive
    // instantiation. INVARIANT — the records ride the placement
    // UNCHANGED, because `transform_rigid` is key-stable (its own
    // contract, the same one `compose_placed` above depends on) and the
    // identity fast path clones keys verbatim. Re-deriving them from
    // the placed geometry is exactly the scan-to-bless move F1 bans;
    // the declaration is inherited, never rediscovered.
    // The bookkeeping half of the same channel. The face keys ride the
    // placement unchanged for the same reason the records do, so the
    // gather re-keys both alike; what is added here is the ROUTE, and
    // one rule builds every one of them: a row the pinned document
    // minted itself arrives through THIS node, and a row that already
    // came up from deeper keeps its own `of` with this node prepended
    // ([`Route::through_instance`]).
    let carried = crate::assembly::CarriedDeclarations {
        minted: carry_up(
            &part.minted,
            part.carried.iter().map(|r| (&r.route, &r.declaration)),
            id,
            doc_ref.id,
        )
        .map(|(route, declaration)| crate::assembly::CarriedDeclaration { route, declaration })
        .collect(),
        unminted: carry_up(
            &part.unminted,
            part.carried_unminted.iter().map(|r| (&r.route, &r.refusal)),
            id,
            doc_ref.id,
        )
        .map(|(route, refusal)| crate::assembly::CarriedRefusal { route, refusal })
        .collect(),
    };
    Ok(OpOut {
        payload: ValuePayload::Body(Arc::new(placed)),
        names: table,
        groups: Arc::default(),
        contacts: Arc::clone(&part.contacts),
        carried: Arc::new(carried),
    })
}

/// One instantiation's worth of routed rows, over one payload kind:
/// the pinned document's OWN rows first — reached through `node`, `of`
/// that document, nothing in between — then the rows it carried up
/// itself, each re-routed through `node`
/// ([`crate::assembly::Route::through_instance`]).
///
/// Generic over the payload because a declaration and a mint refusal
/// are the same act here — a row of another document reaching this one
/// — and one route rule written twice is one place for it to drift.
fn carry_up<'a, P: Clone + 'a>(
    own: &'a [P],
    below: impl Iterator<Item = (&'a crate::assembly::Route, &'a P)> + 'a,
    node: RecipeNodeId,
    of: crate::ident::DocumentId,
) -> impl Iterator<Item = (crate::assembly::Route, P)> + 'a {
    own.iter()
        .map(move |payload| {
            (
                crate::assembly::Route {
                    through: node,
                    of,
                    via: Vec::new(),
                },
                payload.clone(),
            )
        })
        .chain(below.map(move |(route, payload)| (route.through_instance(node), payload.clone())))
}

/// Stamps every UNSOURCED description of `body` with this node's
/// minted [`GeomSource`]s, one shared index space in deterministic
/// arena order (D1/N6: every description minted by evaluation carries
/// its recipe source; pass-through descriptions keep the source they
/// arrived with). Per-evaluation identity — exactly the scope N6's
/// binding caveat allows.
fn stamp_minted<T: Decide>(body: &mut Body<T>, node: RecipeNodeId) {
    let _ = stamp_minted_from(body, node, 0);
}

/// [`stamp_minted`] continuing an index space: stamps `body`'s
/// unsourced descriptions from `first` up and returns the next free
/// index. A node that mints SEVERAL bodies stamps them all through
/// this, threading the index, because the same-source theorem (N6:
/// same `GeomSource` ⇒ bit-identical descriptions) is stated per
/// NODE — two bodies of one node carrying `minted(node, 0)` on two
/// different descriptions would be one source over two geometries,
/// which the boolean's rung 1 reads as identity.
fn stamp_minted_from<T: Decide>(body: &mut Body<T>, node: RecipeNodeId, first: u32) -> u32 {
    let mut idx: u32 = first;
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
    idx
}

/// Re-stamps `placed`'s descriptions with `input`'s sources wrapped
/// by placing node `by` at `instance` (N6: the transform node
/// composes into `expr`). Keys are stable across `transform_rigid`,
/// so the input's rows map key-for-key. Unsourced input descriptions
/// stay unsourced — never invented.
///
/// **The ordinal.** `instance` is the body's OUTPUT index in the
/// placing node's value, and that is the whole rule: a transform of
/// one body stamps 0, a transform of instances stamps body `i` as
/// `i`, a pattern stamps every placed body with its flat index
/// `j·M + i` (the structural index `j` when the master is one body),
/// and an instantiated part is placement 0 of its document. A
/// pattern's placement 0 is the master's own bodies VERBATIM — their
/// `Arc`s, carrying the master's stamps, unstamped by the pattern —
/// so distinct bodies of one node never share a source: two placed
/// bodies differ in the ordinal, and placement 0 differs from every
/// placed body in the wrapping node.
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

/// **A rigid placement of `body` by node `by`, stamped** — the one site
/// that pairs `transform_rigid` with [`compose_placed`], so every
/// placing door (instantiate, transform, pattern, placed union) places
/// and stamps the same way. `None` is the bit-exact identity the
/// instantiate door admits (a clone: the arenas are untouched, so no
/// re-certification is owed), stamped like any other placement.
///
/// # Errors
///
/// The kernel's own [`topo::transform::TransformError`] as
/// [`NodeErrorKind::Transform`].
fn place<T: Decide + geom_brep::PcurveFittedLane + topo::AtRestPolicy>(
    body: &Body<T>,
    map: Option<&Affine3<T>>,
    by: RecipeNodeId,
    ordinal: u32,
    tol: Tol,
) -> Result<Body<T>, NodeErrorKind> {
    let mut placed = match map {
        None => body.clone(),
        Some(map) => transform_rigid(body, map, tol).map_err(NodeErrorKind::Transform)?,
    };
    compose_placed(body, &mut placed, by, ordinal);
    Ok(placed)
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

// OPERAND-DOOR BEGIN — the region the `wire_operand_door` suite's
// `source_rules` census reads. That row counts every `WrongOperand`
// CONSTRUCTION in this file and requires exactly one, inside here; a
// second construction reds it wherever it is written, including inside
// these sentinels, and widening the region does not buy one. The
// doors it censuses are DERIVED from what is declared between the
// sentinels, so a door added here is measured the moment it is typed.

/// **The operand refusal, constructed** — the one site in this file
/// that writes [`NodeErrorKind::WrongOperand`]'s three fields.
///
/// Private to the two doors below, and that is the whole point: a
/// caller supplies the read and the phrase, never the word for what
/// was found. The two doors are the two SOURCES of that word — a
/// value's payload, and a node's kind — and they are the only callers
/// this has.
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
/// `read` is the only thing a caller decides: the projection that
/// either finds on the value what this door's consumer needs, or says
/// it is not there. `expected` is the phrase to author, and it comes
/// from [`super::family`] or [`super::phrase`] — one home per phrase,
/// never a literal here.
///
/// **`found:` is not a caller's to write.** It is the family the value
/// actually carries, read off the payload HERE. A door that spelled it
/// itself could answer with the negation of its own `expected:` —
/// *"carries kind not a datum frame; the operand needs kind datum
/// frame"* — which tells a reader what the input is not, twice, and
/// what it is, never.
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
/// the roads that never hold a value: a reference read straight out of
/// the document.
///
/// It is a second door rather than a second copy of [`operand`]
/// because `found:` has a different SOURCE here, not a different
/// spelling: there is no payload to read the family off, so it comes
/// from [`super::node_value_kind`], which answers the same question
/// over node kinds and in the same words. What the two doors share is
/// the rule — the caller supplies the read and the phrase, never the
/// word for what it found.
///
/// A reference to no live node is not a kind mismatch and is not
/// spelled as one.
///
/// # The two halves answer about the same node, except across a placer
///
/// `read` tests the node the reference NAMES; `found` answers what
/// family that reference's value lands in, and
/// [`super::node_value_kind`] walks a [`Node::Transform`] chain to its
/// source to say so. The two coincide everywhere a document can
/// reach — but a `Transform` over a `Profile` would take the `None`
/// arm (it is not a profile NODE) and answer `found: "profile"`, a
/// refusal that states nothing.
///
/// **That is unreachable by mechanism rather than by luck**, and the
/// mechanism is a rung earlier: a transform of a profile never
/// evaluates. `wire_transform` reads its operand through
/// [`placeable_operand`], which admits only a body, a boolean's body
/// and instances, so the transform itself refuses `WrongOperand` and
/// POISONS every dependent — the loft or sweep that named it is never
/// run, and this door is never reached with such an id. The day a
/// placer becomes shape-preserving over profiles, the walk and the
/// read stop agreeing, and this paragraph is what to come back to.
///
/// # Errors
///
/// [`NodeErrorKind::MissingInput`] for a reference that names no live
/// node; otherwise [`NodeErrorKind::WrongOperand`] naming the family
/// the node lands in. The seat [`super::node_value_kind`] answers
/// beside a refusal of its own is dropped here, because no refusal of
/// its own can arrive: the reference this door reads is one of the
/// consuming node's INPUT edges (a loft's or sweep's section), which
/// the schedule evaluated `Ok` before the op ran — a transform in that
/// slot whose source is not placeable, or whose input is no live
/// node, fails on its own and poisons the consumer ahead of this door.
/// What is dropped is therefore a seat the evaluation has already
/// given, not a seat this door chooses.
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

/// [`operand`]'s refusal alone, for the three doors that already hold
/// the value and cannot go through the door itself: one with two
/// admitted shapes and a narrower word for each
/// ([`body_operand`] over [`placeable_operand`]), one whose read is a
/// `fn` pointer its correspondence supplies ([`wire_split`]), and one
/// that selects on the pairing of a selector with a payload
/// ([`wire_part`]).
fn wrong_operand<T: Decide>(
    v: &super::NodeValue<T>,
    input: RecipeNodeId,
    expected: &'static str,
) -> NodeErrorKind {
    operand_refusal(input, expected, v.payload.kind_name())
}

// OPERAND-DOOR END

/// A single-body operand: a Body value, or a boolean's non-empty
/// result — what every consumer that genuinely takes ONE body reads
/// through (a datum's face frame, a blend, a shell, a split's target,
/// a boolean's and a union's members, a placed union's prototype).
/// Written through [`placeable_operand`]: the admitted one-body set is
/// that door's `Body` arm, and this door's only addition is the
/// refusal of the other arm, in its own one-body word.
fn body_operand<T: Decide>(
    results: &Results<T>,
    input: RecipeNodeId,
) -> Result<Arc<Body<T>>, NodeErrorKind> {
    let v = value_of(results, input)?;
    match placeable_operand(v, input) {
        Ok(Placeable::Body(b)) => Ok(b),
        // The instances the wider door admits, and everything it
        // refused by kind, are one case here: both are a value this
        // door cannot take, and both name the family it carries.
        Ok(Placeable::Instances(_)) | Err(NodeErrorKind::WrongOperand { .. }) => {
            Err(wrong_operand(v, input, super::family::BODY))
        }
        Err(other) => Err(other),
    }
}

/// **What a placer places**: the two value shapes a rigid map is
/// defined over. The placers (`Transform`, `Pattern`) are
/// shape-preserving over their input's value — `Body → Body`,
/// `Instances → Instances` — so this is the operand they read, and
/// [`body_operand`] is its one-body restriction.
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
    /// index in the value, yielding the SAME shape as a payload —
    /// `Body → Body`, `Instances → Instances`. This is the ruling as
    /// a function: a placer decides only what `f` does to one body.
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
/// rule over NODE kinds, for the road that holds no value, is decided
/// in [`super::node_value_kind`]'s one match, beside the family word.
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
/// the mate solve's re-derivation of both from the recipe.
///
/// It reaches the funnel as an argument to
/// [`geom_core::decide_unit_direction`] rather than as a literal at the
/// `decide` call, so it is a roster carrier (`docs/K-REPORT.md`, "The
/// inventory method, restated"), and it is a constant so that the
/// name the telemetry records and the name an escalation reports
/// cannot drift apart.
pub(crate) const EVAL_DIRECTION_NORM: &str = "eval_direction_norm";

/// Normalizes a direction-valued vector; a non-finite length refuses,
/// an underflowed one refuses, a decided-zero length refuses, in-band
/// indeterminacy escalates.
///
/// **The decision is the kernel's one body**
/// ([`geom_core::decide_unit_direction`]): finiteness asked first through
/// the value channel every scalar has, then whether the length
/// underflowed out of the format through the same channel, then which
/// side of zero the length lies on, then normalize or refuse. This function is that
/// call plus the two things the evaluation layer owns — the funnel
/// name it is decided under ([`EVAL_DIRECTION_NORM`]) and the ROLE
/// word each refusal carries, so a user reads which vector of theirs
/// was refused.
///
/// **Two names, one body, and the split is RATIFIED** (Ev's ruling on
/// the direction-family home, executed by SEAT-DN): the layer that
/// OWNS a value is the layer whose telemetry names its length
/// decision. This door carries the directions this layer owns; a
/// datum's normal or axis direction is decided under
/// [`DATUM_UNIT_NORM`] inside the kernel type that holds it
/// ([`UnitVec3::new`]), because `DatumValue` has no
/// unnormalized spelling and there is nowhere for this door to stand
/// in that path. Collapsing the two names would erase which layer a
/// length decision came from; collapsing the two BODIES was the
/// remedy, and it is what the call below is.
///
/// MATE-1's collapse of `mate_pattern_direction_norm` into this door
/// HOLDS — the mate solve derives its offsets through this function,
/// so a direction this layer owns is decided under one predicate
/// wherever it is read. It re-reads a circular pattern's DATUM axis
/// from the recipe, so that one triple is decided under this name on
/// the solve road and under [`DATUM_UNIT_NORM`] on the evaluation
/// road: same arithmetic, same refusal shape, two names by road. That
/// is the ratified consequence, stated where the two roads meet
/// (`crate::mate::solve`) and in `docs/K-REPORT.md`, not a residue.
pub(crate) fn unit<T: Decide>(
    v: Vec3<T>,
    role: &'static str,
    band: Band,
) -> Result<UnitVec3<T>, NodeErrorKind> {
    UnitVec3::new(v, EVAL_DIRECTION_NORM, band).map_err(|e| refusal(e, role, EVAL_DIRECTION_NORM))
}

/// **The kernel refusal in this layer's vocabulary** — the ONE map,
/// for both roads.
///
/// The two doors above and below decide the same three things under
/// two funnel names, so the arms and the role word are one function
/// and the name is its parameter: a map per road is how the arms come
/// to disagree, which is the defect one body was collapsed to fix and
/// would be silly to re-introduce at the mapping.
///
/// `role` names the vector the CALLER passed, which is what a user
/// reads; `predicate` names the funnel site the length was decided
/// under, which is what an escalation is comparable by. They are
/// different words on purpose and both travel.
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
/// derived offset, which reads the same nodes' slots through the same
/// door and must not spell the read a second way.
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
/// The two are separated because a refusal is not always raised where
/// it is made: [`FramePlacement::Unreadable`] CARRIES one to the reader
/// that needed it. Both roads spell it through [`DirectionRefusal::node_error`],
/// which is the only place [`refusal`] is reached from either, so a
/// carried refusal and an on-the-spot one are one fact in one
/// vocabulary — by construction, not by two call sites agreeing.
#[derive(Debug, Clone, Copy)]
pub struct DirectionRefusal {
    /// Which vector refused, in the words a user reads — "datum frame
    /// x axis", "datum frame y axis". The whole user-facing content of
    /// the refusal, and the half a wrong carry would corrupt silently.
    pub role: &'static str,
    /// What the kernel's direction door said. Four facts, not one:
    /// [`UnitVec3Error`] enumerates them.
    pub error: UnitVec3Error,
}

impl DirectionRefusal {
    /// The node error this refusal spells, under [`DATUM_UNIT_NORM`],
    /// because on this road the kernel type owns the value. **The one
    /// spelling**: every road from a carried or raised refusal to a
    /// [`NodeErrorKind`] comes through here — including
    /// [`NodeErrorKind::FrameDirection`]'s `Display` and its tag,
    /// which is why this is `pub`: a carried refusal that named the
    /// frame would otherwise have to re-spell the fact in the crate
    /// that reads it.
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
/// `NoDirection` is not only "these two are parallel". It is whichever
/// of [`UnitVec3Error`]'s four facts the door reported: a decided-zero
/// length (the parallel case), a length that overflowed the norm, one
/// that underflowed out of the format — a vector with a perfectly good
/// direction and no representable length — or an undecided margin
/// inside the band, which `f64` reaches too, since `Decide for f64`
/// answers `Indeterminate` there.
///
/// The refusal is an ANSWER here rather than an `Err` because a
/// fallible read whose two callers dispose of the failure differently
/// would otherwise be a `Result<Result<_, _>, _>`: the slot faults are
/// the node's either way and stay in the `Err`, while this one is the
/// READ's and both dispositions are legitimate — [`wire_datum`] raises
/// it at once, the `f64` placement carries it
/// ([`FramePlacement::Unreadable`]). Nesting is what is avoided, not a
/// forced disposition; an `Err` would have forced nothing.
enum FrameRead<T: geom_core::Real> {
    /// The orthonormal frame.
    Frame(OrthoFrame<T>),
    /// The direction door refused `u`, or `v`'s residual — see the
    /// type's own docs for which four facts that covers.
    NoDirection(DirectionRefusal),
}

/// **An authored frame from its evaluated slots** — the one spelling
/// of the read, shared by the frame's own evaluation at the lane
/// scalar ([`wire_datum`]) and by its f64 placement
/// ([`mint_frame_placement`]), so the two cannot keep different axes or
/// refuse in different orders. The origin is read first; then `u` and
/// `v` are orthonormalized through [`frame_axes`] with `u` kept — the
/// frame's sketch +x is what the author wrote, and `v` is the axis
/// that yields, because keeping `v` would silently rotate every
/// profile drawn on the frame when only `v` was edited.
///
/// # Errors
///
/// [`NodeErrorKind::MissingSlot`] for a slot the values do not carry
/// (unreachable while `Node::slots` and the wire agree). A refused
/// DIRECTION is [`FrameRead::NoDirection`], not an error — see there.
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
/// fork, decided ONCE on the frame's own behalf and carried by name on
/// its value ([`super::NodeValue::placement`]), so a profile drawn on
/// the frame READS this instead of evaluating the frame's nine
/// expressions again.
///
/// The three arms are the three answers, and no reader has to infer
/// one from the absence of another.
#[derive(Debug, Clone, Copy)]
#[non_exhaustive]
pub enum FramePlacement {
    /// An AUTHORED frame ([`Datum::Frame`]): its nine expressions at
    /// the document's nominal, resolved and orthonormalized. Its
    /// profiles place with THIS.
    Authored(profile::SketchPlane<f64>),
    /// An AUTHORED frame whose nominal read REFUSED a direction — `u`,
    /// or `v`'s residual, on whichever of [`UnitVec3Error`]'s four
    /// facts the door reported THERE. Not only the parallel pair:
    /// [`FrameRead::NoDirection`] enumerates them.
    ///
    /// The refusal is held WHOLE, so the reader that raises it spells
    /// it through [`DirectionRefusal::node_error`] and cannot spell it
    /// a second way.
    ///
    /// Carried rather than raised, because the frame's own evaluation
    /// succeeded: it landed a value at the lane scalar, and every
    /// reader that wanted only that value ([`frame_plane_lane`],
    /// `Datum::AxisInPlane`, the mate solve, a measure, the viewer)
    /// reads it correctly. The refusal belongs to the reader that
    /// actually needed the nominal placement, which is where
    /// [`profile_plane_f64`] raises it. The decision itself was made
    /// in the frame's own verdict frame and is logged there.
    Unreadable(DirectionRefusal),
    /// A DERIVED frame ([`Datum::FaceFrame`], DM1c): no document
    /// elaboration at any scalar, so there is nothing to mint. Its
    /// profiles are placed at the lane under every lift
    /// ([`frame_plane_lane`]) and their 2-D structure record is
    /// assembled in the conventional `SketchPlane::xy()`
    /// ([`prepare_profile`]), which no decision reads.
    Derived,
}

/// **A frame node's placement, minted once** — [`FramePlacement`] for
/// a frame node, `None` for every node that is not one.
///
/// # The fork is here, and it is exhaustive
///
/// This is the site that MINTS the answer, and it forks on the RECIPE
/// node's kind over a closed match: a new [`Datum`] variant is a
/// compile error here rather than a node whose profiles silently place
/// at the lane. `None` means "not a frame at all" and nothing else —
/// [`profile_plane_f64`] turns it into the same loud
/// [`NodeErrorKind::WrongOperand`] every other by-value reader of a
/// frame raises.
///
/// # Why an authored frame is read at `f64`
///
/// An AUTHORED frame is document expressions, and they are read at
/// `f64` rather than at the lane scalar for the C6 reason: the
/// placement feeds STRUCTURE selection, which must be lane-identical —
/// the same document has to select the same structure at `f64` and at
/// the interval scalar, or the lift's two passes are deciding
/// different questions. That is the rule the profile's own program
/// already follows (`eval_node` resolves it "at f64 because it feeds
/// C6 structure selection"), and the frame's LANDED value is the right
/// answer to a different question (what a reader sees, what a measure
/// measures). So an authored frame is read twice, at two scalars, for
/// two purposes — `Pinned` places with THIS read, `Guided` with
/// [`frame_plane_lane`]'s — and the two rides of one value sit side by
/// side on one result.
///
/// `nominal` is the node's slots already evaluated at
/// [`LaneEnv::nominal`] — `eval_node`'s nominal list, the one the
/// content key is owed ([`super::tag::slot`]) — so this mints from the
/// values that are already in hand and evaluates no expression.
///
/// # Errors
///
/// [`NodeErrorKind::MissingSlot`] for a slot the values do not carry,
/// and the band's own refusal. Both are faults of the NODE rather than
/// of one reader's question — a node that does not carry the slots it
/// declares is unreadable at every environment — so they fail it,
/// where a direction refusal is carried as
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
        // Not a frame: no placement, and a profile that names one of
        // these gets the kind refusal, not a silent lane placement.
        Datum::Plane { .. }
        | Datum::Axis { .. }
        | Datum::Point { .. }
        | Datum::AxisInPlane { .. } => Ok(None),
    }
}

/// **A profile's `f64` placement — a READ of the frame's result**, not
/// a second evaluation of the frame's slots: the answer
/// [`mint_frame_placement`] minted on the frame node, which the frame's own
/// content key already fixes (its nominal slots and the tolerance are
/// exactly what the placement is a function of, so a memo hit carries
/// a placement equal bit for bit to the one a recompute would mint).
///
/// This function's `None` is a DERIVED frame and only that. It is a
/// different `None` from the field's, which means "not a frame" — a
/// derived frame says so BY NAME on the value, and the other two
/// answers there are a plane and a refusal.
///
/// This `None`, and the one [`ProfilePre::placement_f64`] carries on
/// from it, are the two `Option`s that survive on this road, and they
/// are both honest: by the time either is written the other two
/// answers have been discharged — "not a frame" into a
/// [`NodeErrorKind::WrongOperand`] and "unreadable" into its own
/// refusal — so "no `f64` placement" has exactly one cause left, and
/// a consumer that places with a derived frame's record still has
/// nothing to mistake for a placement.
///
/// The frame is a DAG input of the profile node ([`Node::inputs`]), so
/// it precedes every reader in the schedule and a failed frame poisons
/// them; a section's frame is its profile's input and so precedes the
/// loft or sweep that reads it.
///
/// # Errors
///
/// [`NodeErrorKind::WrongOperand`] when the reference does not name a
/// frame — through [`operand`], so this reader asks the question the
/// one way it is asked and names the frame with the one phrase —
/// [`NodeErrorKind::FrameDirection`] where the frame carried a
/// direction refusal ([`FramePlacement::Unreadable`], raised HERE
/// because this is the reader that needed it, and naming BOTH
/// `profile` and the frame because neither is reliably the node the
/// error ends up attached to — the section seam raises it on the loft),
/// and [`NodeErrorKind::MissingInput`] for a reference with no value.
pub(crate) fn profile_plane_f64<T: Decide>(
    results: &Results<T>,
    profile: RecipeNodeId,
    plane: RecipeNodeId,
) -> Result<Option<profile::SketchPlane<f64>>, NodeErrorKind> {
    // The carry's `None` IS the kind refusal — a node that is not a
    // frame mints no placement — so the door reads the carry and the
    // three answers a frame can give are what is left.
    match operand(results, plane, super::phrase::DATUM_FRAME, |v| v.placement)? {
        FramePlacement::Authored(placement) => Ok(Some(placement)),
        FramePlacement::Derived => Ok(None),
        // The refusal is the frame's; raising it HERE is right (a
        // frame nobody draws on must not poison the document) and is
        // exactly why it must name the frame: on this node the role
        // word alone says which AXIS refused and nothing says whose.
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
/// The f64 read above is for STRUCTURE, and it is the whole answer only
/// while a frame's components are literals. They are `Expr`s, so a
/// document parameter can drive a frame's origin — and under an
/// interval or dual run that parameter has a non-degenerate value.
/// Embedding the f64 placement into `T` (which is what this pass did
/// while the plane was inline literal floats, and was exact then)
/// would drop that parameter's width from the plane while carrying it
/// correctly through every other slot: an enclosure that does not
/// enclose.
///
/// So the lane pass reads what the frame's own evaluation landed, at
/// the lane's scalar. Structure stays f64-pinned and lane-identical;
/// magnitudes stay lane-live. That is the same split the profile's own
/// program follows, applied to the frame it is drawn on.
///
/// A DERIVED frame is read here under EVERY lift (DM1c): it has no
/// document elaboration, so this by-value read is the only placement
/// it has, and the profile on it is placed at the lane scalar with
/// its 2-D structure record still `f64`-pinned.
///
/// # Errors
///
/// [`NodeErrorKind::WrongOperand`] when the landed value is not a
/// frame, through [`frame_value`].
pub(crate) fn frame_plane_lane<T: Decide>(
    results: &Results<T>,
    plane: RecipeNodeId,
) -> Result<profile::SketchPlane<T>, NodeErrorKind> {
    // Orthonormal by the datum's own construction, and carried as the
    // frame witness `SketchPlane::from_frame` takes.
    Ok(profile::SketchPlane::from_frame(frame_value(
        results, plane,
    )?))
}

/// A lane-scalar placement carried across to `f64`, exactly, where the
/// scalar is the `f64` lane — every component through
/// [`super::SectionScalar::pinned_f64`] — and `None` on any analysis
/// scalar. No component is inspected: the answer is the type's. The
/// walk is the kernel's [`profile::SketchPlane::try_map`], the
/// fallible direction of [`profile::SketchPlane::map`].
pub(crate) fn pinned_plane<T: super::SectionScalar>(
    plane: &profile::SketchPlane<T>,
) -> Option<profile::SketchPlane<f64>> {
    plane.try_map(|x| x.pinned_f64().ok_or(())).ok()
}

/// **A frame's authored pair, made orthonormal** — the one spelling of
/// it, and the one door its two refusals come out of.
///
/// Two callers need this and they must agree: the datum's own
/// evaluation at the lane scalar, which produces the
/// [`DatumValue::Frame`] a reader sees, and the frame's own `f64`
/// placement ([`mint_frame_placement`]), which every profile drawn on it
/// then reads. A second spelling would be two frames for one node,
/// free to disagree about where a sketch's +x points.
///
/// `u` is normalized and KEPT; `v` yields its component along `u`.
/// Gram-Schmidt states "these two span no plane" as a length, so a
/// parallel pair refuses at the same decided door every other
/// direction does, under the y axis's role, rather than under a
/// predicate invented here.
///
/// The refusal comes out UNSPELLED ([`DirectionRefusal`]): one caller
/// raises it on the spot, the other carries it to the reader that
/// needed it, and both spell it through the one map.
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
/// whole, so neither reader can swap `u` for `v` and silently turn an
/// in-plane axis by a right angle.
///
/// The refusal is the operand door's: a reference whose value is not a
/// frame is a kind mismatch at the input, not a geometry problem.
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
        // **The frame is the one datum whose slots are not independent**:
        // u and v have to span a plane, and a pair that does not is the
        // authoring mistake this refuses. Gram-Schmidt states that
        // condition as a length, so it is decided at the SAME door as
        // every other direction rather than under a new predicate — v's
        // component perpendicular to û is decided-zero exactly when the
        // two are parallel, which is exactly when there is no plane.
        // Which axis is kept, and why, is stated at the one spelling of
        // the read, `frame_from_slots`.
        Datum::Frame { .. } => match frame_from_slots(vals, band(tol)?)? {
            FrameRead::Frame(frame) => DatumValue::Frame(frame),
            FrameRead::NoDirection(refusal) => return Err(refusal.node_error()),
        },
        // **The one datum that reads another node.** Its four numbers
        // are coordinates IN a frame, so the frame is what they mean,
        // and the lift happens once here rather than at each reader.
        //
        // No in-plane check appears anywhere in this arm, and that is
        // the point of the variant: a 2-D pair lifted through the
        // frame's own axes lies in that frame by construction, so
        // there is no residual to decide and no band to decide it
        // against.
        Datum::AxisInPlane { plane, .. } => {
            let f = frame_value(results, *plane)?;
            let (frame_origin, u, v) = (f.origin(), f.u(), f.v());
            let plane_origin = need_point2(vals, SlotId::Origin)?;
            let plane_dir = need_vec2(vals, SlotId::Direction)?;
            let lift = |d: Vec2<T>| u.get() * d.x + v.get() * d.y;
            DatumValue::AxisInPlane {
                plane_origin,
                plane_dir,
                origin: frame_origin + lift(Vec2::new(plane_origin.x, plane_origin.y)),
                // The frame's axes are orthonormal, so this lift
                // preserves length: it is decided-zero exactly when the
                // authored pair is, which is why the sketch direction
                // above can go to the kernel unnormalized and still get
                // the same refusal a 3-D axis would.
                dir: datum_unit(lift(plane_dir), DATUM_AXIS_ROLE, band(tol)?)
                    .map_err(DirectionRefusal::node_error)?,
            }
        }
        // **The frame read off a face** (DM1). Its value is exactly an
        // authored frame's, produced through the same `frame_axes`
        // door, so every reader of a frame takes it by value; what is
        // different is where the numbers come from, and every step of
        // that is a stored fact copied out or a resolution through
        // the N5 ladder — nothing here decides a number.
        Datum::FaceFrame { at, face, .. } => {
            let body = body_operand(results, *at)?;
            let table = &value_of(results, *at)?.name_table;
            // The fillet's ladder: rung 1 against the document, rungs
            // 2 and 3 against the body's own table.
            let key = named_entity(
                face,
                doc,
                table,
                |error| NodeErrorKind::FaceFrameResolve { error },
                names::EntityKey::face,
                |name, found| NodeErrorKind::FaceFrameKind { name, found },
            )?;
            // DM1b / DM2: the carrier's KIND is a stored tag, and a
            // sketch frame wants a plane. A comparison of tags, not a
            // predicate.
            let carrier = topo::readback::face_carrier_kind(&body, key)
                .map_err(|error| NodeErrorKind::FaceFrameReadback { error })?;
            if carrier != geom_brep::SurfaceKind::Plane {
                return Err(NodeErrorKind::FaceFrameNotPlanar { carrier });
            }
            let pose = topo::readback::face_pose(&body, key)
                .map_err(|error| NodeErrorKind::FaceFrameReadback { error })?;
            // DM1a: the outward normal is the chart axis folded through
            // the sense beside the pose — the bit selects, nothing is
            // computed.
            let n = OutwardNormal::from_chart(pose.axis, pose.sense).vec();
            // A plane carrier always fixes its u-reference (readback's
            // rule 3 leaves `None` only where the carrier fixes none,
            // which a plane never is); the kind check above is what
            // makes this arm unreachable for such a carrier.
            let u_ref = pose.u_ref.ok_or(NodeErrorKind::FaceFrameReadback {
                error: topo::readback::ReadbackError::NoCanonicalFrame {
                    carrier: "planar carrier without a u-reference",
                },
            })?;
            // The spin: sketch +x is the u-reference turned about the
            // outward normal — a rotation, not a predicate. `u_ref`
            // lies in the plane, so the rotation is the two-term
            // form, and `v` is the right-handed third leg.
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
/// program replays through `profile::replay` — the driver, the ONLY
/// path from steps to geometry — then the assembled `Profile<f64>`
/// validates at f64 (the C6 structure-selection gate, which also
/// yields the canonical form the naming anchor is derived from). Runs inside the node's verdict frame (`eval_node`) ahead of
/// the op: structure decisions, the successor of the stored f64 bits,
/// logged as the node's own. The validated form is kept: under the
/// pinned lift it IS the op's value, lifted (`wire_profile`), so the
/// node decides each structure question once. VQ6 is closed here and
/// in the guided op below: the replay-time junction checks and every
/// validation run under the SAME `Tolerance::get()` the evaluation
/// pins.
pub(crate) fn prepare_profile(
    placement: Option<profile::SketchPlane<f64>>,
    resolved: &[Vec<profile::Step<f64>>],
    ids: &[Vec<crate::node::StepId>],
    tol: Tol,
) -> Result<ProfilePre, NodeErrorKind> {
    // The 2-D record's assembly frame: the placement where there is
    // one, the conventional frame where there is not (a derived
    // frame's profile, DM1c). Validation is 2-D and the naming anchor
    // is loop-derived, so no decision below reads it; what places the
    // profile is `placement_f64`, carried through as the same Option.
    let plane = placement.unwrap_or_else(profile::SketchPlane::xy);
    let mut loops = Vec::with_capacity(resolved.len());
    let mut replay_records = Vec::with_capacity(resolved.len());
    for (li, steps) in loop_coordinates(resolved.len())?.zip(resolved) {
        let (lp, record) = profile::replay_recording(steps, tol)
            .map_err(|error| NodeErrorKind::ProfileReplay { loop_: li, error })?;
        loops.push(lp);
        replay_records.push(record);
    }
    let profile_f64 = profile::Profile::new(plane, loops);
    let (validated_f64, canonical) = profile_f64
        .validate_recording(tol)
        .map_err(NodeErrorKind::Profile)?;
    let naming = anchor::derive_naming(&validated_f64, &profile_f64.loops).ok_or({
        // A canonical loop failed to match any program loop — an
        // internal invariant break, typed (the loop coordinate is not
        // recoverable from the failed derivation; 0 names the walk).
        NodeErrorKind::ProfileAnchor { loop_: 0 }
    })?;
    // The piece each canonical position is: the records above and the
    // program's minted ids describe one program, so a disagreement is
    // the same internal break, surfaced the same way.
    let pieces = super::ProfilePieces::publish(&naming, &replay_records, ids)
        .ok_or(NodeErrorKind::ProfileAnchor { loop_: 0 })?;
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
    profile::Profile::new(plane, loops)
        .validate_guided(tol, &pre.structure.canonical)
        .map_err(NodeErrorKind::Profile)
}

fn wire_profile<T: Decide + geom_core::Bounds>(
    program: &ProfileProgram,
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
        // through `from_f64` (`ValidatedProfile::lift_onto`), every
        // decision carried as the f64 one and none remade. That is
        // this lift's design, not predicate agreement: structure is
        // selected once, at f64, identically for every lane
        // (`ProfileLift`); a margin an `Interval` validation would
        // escalate on is decided here by its f64 verdict, and the
        // guided lift is where that margin escalates. Placed on the
        // lane's plane: an authored frame at its `f64` elaboration
        // lifted; a DERIVED frame has no `f64` elaboration of its
        // placement (DM1c: the document holds a face name, not nine
        // numbers), so its placement is the lane's own value, read
        // where every by-value reader of a frame reads it. The fork is
        // by node kind; the numbers on both sides are the ones already
        // computed, and the op decides nothing: the node's log under
        // this lift is the precompute's.
        super::ProfileLift::Pinned => {
            let plane = match &pre.placement_f64 {
                Some(placement) => placement.map(T::from_f64),
                None => frame_plane_lane(results, program.plane)?,
            };
            pre.validated_f64.clone().lift_onto(plane)
        }
        super::ProfileLift::Guided => lane_profile::<T>(
            program,
            frame_plane_lane(results, program.plane)?,
            lane,
            pre,
            tol,
        )?,
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
/// The door already answers in canonical positions — the numbering a
/// sweep's walls are indexed by — so this is a lookup by position:
/// canonical loop `l`, segment `k` is the position `(l, k)`. Nothing here re-derives a
/// permutation; the door checked the evaluation's two records of it
/// against each other.
///
/// **A refusal here is the evaluation contradicting itself.** The
/// records were minted from this program by the same pre-pass, so
/// every refusal the door has is a statement about a record from
/// somewhere else: no loop is missing, no record is of the wrong
/// shape, no span or emission runs off its loop, and no emission
/// credits a radius role the step it names does not hold. That is the
/// class `ProfileProgram::profile_edges_of` asserts on rather than
/// refusing, and it is surfaced the same way here: a typed error
/// would be one no document can reach and no caller can repair, so it
/// is a kernel bug the code observes in a branch — `unreachable!`'s
/// own job (D9's D2 addendum).
fn edge_radii(program: &ProfileProgram, pre: &ProfilePre) -> Vec<Vec<Option<crate::expr::Expr>>> {
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
                    // and the address is recorded at that one moment.
                    // `no_two_emissions_of_one_loop_name_the_same_segment`
                    // (`crates/profile/tests/path_program.rs`) is that
                    // property, measured over the corpus.
                    by_program_segment
                        .iter()
                        .find(|(e, _)| *e == want)
                        .map(|(_, expr)| (*expr).clone())
                })
                .collect()
        })
        .collect()
}

/// **The profile-operand verbs' ONE lowering**, driven by the verb's
/// correspondence ([`crate::verbs::sweep`]) rather than written twice.
///
/// It is a THIRD lowering beside `wire_blend` and `wire_boolean`, and
/// stating that plainly is the honest reading: the three share a
/// SHAPE — build the verb, run it through its own door, read the birth
/// record, emit names, stamp provenance, attach the declared flow — and
/// share no code, because the operand differs at every step (a body
/// and its name table, two bodies and two tables, a validated profile
/// and its naming anchor). What this function removes is the SECOND
/// copy of that shape within the sweep family, which is what a verb's
/// migration is measured on.
///
/// The verb ARGUMENTS come in already resolved. That is the division
/// the boolean drew for `resolve_declarations` and the reason it holds
/// here too: an extrude's distance is a slot read, but a revolve's axis
/// is a node whose value must be an in-plane axis, written against the
/// same frame the profile is drawn on, with an angle classified full or
/// partial at a funnel site whose escalation is a document-layer
/// refusal. None of that is a verb parameter; all of it is what the
/// document MEANS. So each node's arm resolves its own semantics and
/// this body takes it from the built verb onward.
///
/// # What it writes
///
/// The body (moved out of the record), the name table (emitted from the
/// record, its profile refs in canonical numbering — the one numbering
/// every profile-consuming verb publishes), the provenance stamp on
/// everything the sweep minted,
/// and the per-edge parameter sources the verb's flow declares.
// The 8th argument is the verb's correspondence — the parameter that
// REMOVES duplication rather than adding a duty, exactly as
// `wire_blend`'s is; the 7th is the evaluation environment, read for
// the descent chain the attached tokens' scope is.
#[allow(clippy::too_many_arguments)]
fn wire_swept<
    T: Decide
        + geom_core::Bounds
        + geom_brep::PcurveFittedLane
        + crate::lane::Lane
        + topo::AtRestPolicy,
    A,
>(
    verb: &crate::verbs::sweep::ProfileVerb<T, A>,
    args: A,
    id: RecipeNodeId,
    profile: RecipeNodeId,
    doc: &crate::doc::Doc<ProfileProgram>,
    results: &Results<T>,
    env: &OpEnv<'_, T>,
    tol: Tol,
) -> OpResult<T> {
    let vp = operand(results, profile, super::family::PROFILE, |v| {
        match &v.payload {
            ValuePayload::Profile(vp) => Some(vp),
            _ => None,
        }
    })?;
    let built = (verb.build)(args);
    // The verb's own declaration of where its parameters land, read off
    // the value the correspondence just built (VERB-SEAT-DESIGN V1).
    let flow = built.param_flow();
    let record = built
        .run_profile(&vp.validated, tol)
        .map_err(verb_refused)?;
    // Eager N4 emission from the emitter's own maps, inside the reader,
    // BEFORE the structural handoff is taken apart.
    let out = (verb.read)(id, record, &vp.pieces, verb.foreign_record)?;
    let table = out.table;
    let mut body = out.body;
    // The sweep's own surfaces, curves and points are minted HERE
    // (D1/N6).
    stamp_minted(&mut body, id);
    // **Attach-at-mint for the lowered parameter-identity channel**
    // (VERB-SEAT-DESIGN P2), through the sweeps' per-EDGE flow source.
    // The token is not this node's: a swept wall's radius is the
    // PROFILE's, so what lowers is the radius the operand profile
    // draws that wall's own edge at, under this evaluation's scope, and
    // the walls the record exported are what it lands on. An edge with
    // no authored radius (every straight one) yields no token and
    // attaches nothing, which is the declaration being obeyed rather
    // than a case skipped.
    let scope = crate::param_source::ParamScope::of(doc.id(), env.parts.chain());
    let tokens = crate::param_source::profile_radius_tokens(vp, scope);
    crate::param_source::attach_swept(
        &mut body,
        flow,
        crate::verbs::sweep::PROFILE_RADIUS,
        &tokens,
        &out.walls,
    )
    .map_err(NodeErrorKind::ParamSourceAttach)?;
    Ok(OpOut::plain(ValuePayload::Body(Arc::new(body)), table))
}

/// **Extrudes a profile along its sketch normal** — the distance slot
/// read, and the generic lowering from there.
fn wire_extrude<
    T: Decide
        + geom_core::Bounds
        + geom_brep::PcurveFittedLane
        + crate::lane::Lane
        + topo::AtRestPolicy,
>(
    id: RecipeNodeId,
    profile: RecipeNodeId,
    doc: &crate::doc::Doc<ProfileProgram>,
    results: &Results<T>,
    vals: &SlotValues<T>,
    env: &OpEnv<'_, T>,
    tol: Tol,
) -> OpResult<T> {
    let distance = need_scalar(vals, SlotId::Distance)?;
    wire_swept(
        &crate::verbs::sweep::extrude(),
        distance,
        id,
        profile,
        doc,
        results,
        env,
        tol,
    )
}

/// The frame a node is written against, read from the RECIPE.
///
/// A profile and an in-plane axis both name one; "the same plane"
/// means the same node, and this is the only reading of it. It is
/// deliberately not a value: two frames that evaluate to the same
/// numbers are still two frames, and an evaluated comparison would
/// make a revolve's legality depend on a float coincidence.
fn written_against(
    doc: &crate::doc::Doc<ProfileProgram>,
    id: RecipeNodeId,
) -> Option<RecipeNodeId> {
    match doc.node(id)? {
        Node::Profile(p) => Some(p.plane),
        Node::Datum(Datum::AxisInPlane { plane, .. }) => Some(*plane),
        _ => None,
    }
}

/// **Revolves a profile about an axis written in its own sketch
/// plane** — the document semantics (the axis operand, the same-frame
/// rule, the full-vs-partial classification), and the generic lowering
/// from there.
// The 8th argument is the evaluation environment, read for the descent
// chain the attached tokens' scope is; the 7th is the document, read
// for the frame rule and the operand profile's own expressions.
#[allow(clippy::too_many_arguments)]
fn wire_revolve<
    T: Decide
        + geom_core::Bounds
        + geom_brep::PcurveFittedLane
        + crate::lane::Lane
        + topo::AtRestPolicy,
>(
    id: RecipeNodeId,
    profile: RecipeNodeId,
    axis: RecipeNodeId,
    doc: &crate::doc::Doc<ProfileProgram>,
    results: &Results<T>,
    vals: &SlotValues<T>,
    env: &OpEnv<'_, T>,
    tol: Tol,
) -> OpResult<T> {
    // `wire_swept` re-checks this and refuses identically, so the only
    // thing this pre-check decides is ORDER: a node whose profile input
    // is not a profile AND whose axis is not an in-plane axis must
    // refuse on the profile, because that is the operand the reader
    // named first and re-authoring a wrong axis for a document whose
    // profile was never one is a wasted edit.
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
    // **The kernel's `RevolveAxis` lives in SKETCH-PLANE coordinates,
    // and so does the axis now** — so the wiring is the identity, and
    // the only question left is whether the two nodes are written
    // against the SAME frame.
    //
    // That is an equality of node ids: no band, no residual, no scale.
    // What stood here was two decided predicates projecting a 3-D axis
    // onto the profile's normal, and the second of them was the
    // dimension audit's F15 — a bare sine `dir·n̂` classified against
    // the metre band, whose executed consequence (that row's pin, a
    // review probe) is a tilt that reads in-plane at every model scale
    // while the deviation it induces crosses the band between a
    // millimetre profile and a ten-metre one. F15's own note proposed
    // levering the sine at the profile's radial extent. This deletes
    // the sine instead: an axis authored in the frame cannot leave it,
    // so there is nothing to lever.
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
    wire_swept(
        &crate::verbs::sweep::revolve(),
        (axis2, revolution),
        id,
        profile,
        doc,
        results,
        env,
        tol,
    )
}

/// The spine frame and window a tube door takes, resolved from the
/// node's one datum edge and its slots.
///
/// Shared by the two tube arms and by nothing else. It is the
/// RESOLUTION that is shared, never the door: this returns the
/// argument list both doors begin with, and each arm then calls its
/// own public door with it. That is the same division the kernel
/// draws — two public doors over one private build — read at the
/// recipe layer.
///
/// The one thing this layer does decide is the FRAME: the tube door
/// takes a witness, so the reference direction the document authored
/// is minted here against the datum's axis, under this layer's own
/// funnel name and role word. The window's span and headroom and (for
/// the hollow door) all three wall verdicts stay the door's own,
/// decided against the run's band; a check here would be a second and
/// weaker opinion about a body this layer is not building.
struct TubeArgs<T: geom_core::Real> {
    frame: geom_core::OrthoFrame<T>,
    major_radius: T,
    window: sweep::TubeWindow<T>,
    minor_radius: T,
}

fn tube_args<T: Decide>(
    spine: RecipeNodeId,
    window: &crate::node::TubeWindow,
    results: &Results<T>,
    vals: &SlotValues<T>,
    tol: Tol,
) -> Result<TubeArgs<T>, NodeErrorKind> {
    let (origin, dir) = operand(results, spine, super::phrase::DATUM_AXIS, |v| {
        match &v.payload {
            ValuePayload::Datum(DatumValue::Axis { origin, dir }) => Some((origin, dir)),
            _ => None,
        }
    })?;
    // The datum is consumed WHOLE — origin as the spine centre, dir as
    // the spine axis — which is `Node::Revolve`'s precedent, and both
    // cross to the frame verbatim: no re-origining.
    //
    // The axis arrives already unit-length, and that is the datum
    // node's doing rather than this arm's: `wire_datum` decides
    // `DATUM_UNIT_NORM` when it evaluates the axis, so a degenerate or
    // non-finite direction refuses there, one node upstream, and what
    // reaches this arm is a `UnitVec3`. `u_ref` is a bare direction
    // that passes through NO datum, so it is the one the frame mint
    // decides here: its component along the axis is projected out and
    // what remains becomes the frame's `u`, normalized, with the axis
    // the frame's `w` VERBATIM (the mint does not re-decide a witness)
    // and `v = w × u`. So a `u_ref` off perpendicular is no longer a
    // refusal — it names a roll and the frame takes the part of it
    // that can; a `u_ref` ON the axis line refuses, under the
    // direction door's own vocabulary and this layer's role word.
    Ok(TubeArgs {
        frame: geom_core::OrthoFrame::from_aim_and_reference(
            *origin,
            *dir,
            need_vec3(vals, SlotId::Direction)?,
            EVAL_DIRECTION_NORM,
            band(tol)?,
        )
        // The aim mint decides the reference's residual and nothing
        // else — the axis is a witness before it arrives — so every
        // refusal here is [`geom_core::OrthoAxis::V`]'s and the role
        // is the reference's.
        .map_err(|e| refusal(e.error, TUBE_REFERENCE_ROLE, EVAL_DIRECTION_NORM))?,
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
/// Measured, not assumed. [`names::name_revolve`] reads only the
/// `Revolved<T>` maps it is handed and the pieces it names them by,
/// never the profile that produced them; the tube doors return a
/// `Revolved<T>` built by the very same `full`/`partial` machinery. So
/// the revolve emitter names a tube body with NO new `RoleSeg`
/// variants: a tube's bands, rims, meridians, caps and poles are those
/// roles, spelled with the section's structural locators
/// ([`tube_pieces`]): the outer circle's pieces 0 and 1 — its two
/// half-circle arcs — and a hollow tube's bore's.
fn wire_tube<T: Decide + geom_brep::PcurveFittedLane>(
    id: RecipeNodeId,
    spine: RecipeNodeId,
    window: &crate::node::TubeWindow,
    results: &Results<T>,
    vals: &SlotValues<T>,
    tol: Tol,
) -> OpResult<T> {
    let a = tube_args(spine, window, results, vals, tol)?;
    let mut built = sweep::tube_along_arc(a.frame, a.major_radius, a.window, a.minor_radius, tol)
        .map_err(|e| NodeErrorKind::Tube(Box::new(e)))?;
    let table =
        names::name_revolve(id, &built, &tube_pieces(&built)?).map_err(NodeErrorKind::Naming)?;
    stamp_minted(&mut built.body, id);
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
    let counts: Vec<usize> = built.walls.iter().map(Vec::len).collect();
    super::ProfilePieces::section(&counts).ok_or(NodeErrorKind::Naming(
        names::NamingError::Emission {
            what: "a tube door built a section of more than two circles",
        },
    ))
}

/// **A hollow tube** — `sweep::tube_along_arc_hollow`, the OTHER
/// public door.
///
/// The wall crosses to it untouched and unexamined. Its three
/// verdicts — the thickness is positive, `minor_radius − wall` is a
/// bore, and the gap between the two radii the body would STORE is
/// positive — are decided kernel-side before anything is minted, and
/// they are what the full ring's cavity insertion carries as its
/// containment evidence. Re-deriving any of them here would be a
/// second opinion that cannot see what the third one sees (the
/// realized gap is a fact about the stored numbers, not the supplied
/// ones).
///
/// Naming is [`wire_tube`]'s, for the reason given there: a hollow
/// tube's cavity shell is the revolve's own hole-loop vocabulary,
/// already emitted by `name_revolve`'s holed-full and windowed arms.
fn wire_hollow_tube<T: Decide + geom_brep::PcurveFittedLane>(
    id: RecipeNodeId,
    spine: RecipeNodeId,
    window: &crate::node::TubeWindow,
    results: &Results<T>,
    vals: &SlotValues<T>,
    tol: Tol,
) -> OpResult<T> {
    let a = tube_args(spine, window, results, vals, tol)?;
    let wall = need_scalar(vals, SlotId::TubeWall)?;
    let mut built =
        sweep::tube_along_arc_hollow(a.frame, a.major_radius, a.window, a.minor_radius, wall, tol)
            .map_err(|e| NodeErrorKind::Tube(Box::new(e)))?;
    let table =
        names::name_revolve(id, &built, &tube_pieces(&built)?).map_err(NodeErrorKind::Naming)?;
    stamp_minted(&mut built.body, id);
    Ok(OpOut::plain(
        ValuePayload::Body(Arc::new(built.body)),
        table,
    ))
}

/// **The verb dispatch's one refusal translation**: the kernel door
/// attached the verb, the `verbs` run door carried the refusal through
/// unaltered, and this layer READS the family off it rather than
/// re-deriving which door it called — one discrimination point per
/// layer, and one site for it here so no two doors can drift.
///
/// Exhaustive over [`verbs::VerbError`] with no wildcard arm, so a
/// verb family with a new refusal shape breaks here rather than
/// arriving as another's.
/// One boolean refusal does NOT come through this door: the
/// undeclared-coincidence menu lift needs the operands'
/// naming context, so [`refusal_menu`] intercepts it and delegates
/// everything else here.
fn verb_refused<T: crate::lane::Lane>(refusal: verbs::VerbError<T>) -> NodeErrorKind {
    match refusal {
        verbs::VerbError::Blend(sweep::blend::BlendRefusal { verb, error }) => {
            NodeErrorKind::Blend { verb, error }
        }
        verbs::VerbError::Boolean(error) => NodeErrorKind::Boolean(error),
        verbs::VerbError::Extrude(error) => NodeErrorKind::Extrude(error),
        verbs::VerbError::Revolve(error) => NodeErrorKind::Revolve(error),
        verbs::VerbError::Split(error) => NodeErrorKind::Split(error),
        verbs::VerbError::Arity { verb, given } => NodeErrorKind::VerbArity { verb, given },
        // **The shell's refusal crosses at the lane's `f64` witness.**
        // The kernel's error is generic over the lane scalar and this
        // enum is scalar-free, so the carriage is a TOTAL fold — every
        // arm, every nested payload, every number — declared by the
        // lane's own end reading (`crate::verbs::shell::
        // fold_shell_error_at`), never a rendering or a drop.
        verbs::VerbError::Shell(error) => {
            NodeErrorKind::Shell(Box::new(crate::verbs::shell::fold_shell_error_at(*error)))
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
/// target's edges**.
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
// duplication rather than adding a duty; the 9th is the evaluation
// environment, read for the descent chain the token's scope is.
#[allow(clippy::too_many_arguments)]
fn wire_blend<
    T: Decide
        + geom_core::Bounds
        + geom_brep::PcurveFittedLane
        + crate::lane::Lane
        + topo::AtRestPolicy,
>(
    verb: &crate::verbs::blend::BlendVerb<T>,
    id: RecipeNodeId,
    target: RecipeNodeId,
    selection: &[names::StableName],
    doc: &crate::doc::Doc<ProfileProgram>,
    results: &Results<T>,
    vals: &SlotValues<T>,
    env: &OpEnv<'_, T>,
    tol: Tol,
) -> OpResult<T> {
    let body = body_operand(results, target)?;
    let size = need_scalar(vals, verb.slots.size_slot)?;
    let target_table = Arc::clone(&value_of(results, target)?.name_table);
    let edges = resolve_selection(verb.selection_label, selection, doc, &target_table)?;
    let built = (verb.build)(edges, size);
    // The verb's own declaration of where its scalar lands, read off
    // the value the correspondence just built (VERB-SEAT-DESIGN V1).
    let flow = built.param_flow();
    let out = built.run(&body, tol).map_err(verb_refused)?;
    // The record channel is per-family; a blend's run produces the
    // blend variant by construction, so another family here is a
    // kernel bug — refused typed, exactly like the `None` record below.
    // The match is EXHAUSTIVE with no wildcard arm (D3): a record
    // family added to the channel breaks this consumer at compile time
    // and must be routed here deliberately, never silently refused.
    let naming = crate::verbs::read_record(out.record, verb.record, verb.foreign_record)?;
    let rec = naming.ok_or(NodeErrorKind::Naming(names::NamingError::Emission {
        what: verb.no_records,
    }))?;
    let table = (verb.emitter)(id, target, &target_table, &out.body, &rec)
        .map_err(NodeErrorKind::Naming)?;
    let mut body = out.body;
    // The blend's own surfaces, curves and points are minted HERE
    // (D1/N6); the supports' pass-through descriptions keep the source
    // they arrived with.
    stamp_minted(&mut body, id);
    // **Attach-at-mint for the lowered parameter-identity channel**
    // (VERB-SEAT-DESIGN P2). The size slot's expression lowers to an
    // opaque token under THIS evaluation's scope — the document's own
    // table, or the reference a part was reached through — and the
    // verb's DECLARED flow says which stored fields of which minted
    // carriers that scalar became; the two meet here and nowhere
    // else. A slot the document does not hold cannot have produced the
    // value above, so its absence is not a case — but it is a lookup,
    // so it degrades to attaching nothing rather than asserting.
    // Nothing downstream is entitled to a token: the channel is opt-in
    // and its absence refuses typed (P3). The attach's own refusals
    // cannot fire (its doc says why) and are surfaced typed if they
    // ever do, never discarded.
    if let Some(expr) = doc.node(id).and_then(|n| n.expr(verb.slots.size_slot)) {
        let scope = crate::param_source::ParamScope::of(doc.id(), env.parts.chain());
        crate::param_source::attach_blend(
            &mut body,
            flow,
            verb.slots.size_param,
            &crate::param_source::lower(scope, expr),
            &rec,
        )
        .map_err(NodeErrorKind::ParamSourceAttach)?;
    }
    Ok(OpOut::plain(ValuePayload::Body(Arc::new(body)), table))
}

/// **The shell's lowering**, driven by the verb's correspondence
/// ([`crate::verbs::shell`]): resolve the frozen, ORDERED list of open
/// faces through the target's name table into face keys, evaluate the
/// thickness slot to `T`, build the kernel verb, run it through the
/// seat's shell door, emit names from the birth record under THIS
/// node's id.
///
/// # Refusals
///
/// The open list resolves through the TARGET's name table into face
/// keys, through the same N5 [`ladder`] a blend's selection takes;
/// a name that stopped resolving is [`NodeErrorKind::ShellOpenResolve`]
/// and a name of another kind [`NodeErrorKind::ShellOpenKind`]. An
/// EMPTY list is not a refusal: it is the sealed hollow, the kernel
/// door's own contract.
///
/// Failure of the op itself is a TYPED refusal ([`NodeErrorKind::Shell`])
/// carrying the kernel's own error through the total fold
/// [`verb_refused`] applies. The input body is never passed through: a
/// hollow that did not happen must read as a failed node, not as a
/// silently solid one. A scalar that cannot form the door's call at
/// all — a dual — refuses [`NodeErrorKind::ShellLaneUnsupported`].
///
/// # Naming
///
/// The record is written by the doors themselves as they act, so it is
/// not an `Option` and there is no "no records" sentence: the emitter
/// translates every row, and the totality check closes the other
/// direction. Nothing here calls `topo::shell_open` — the seat is the
/// door, and the seat's record channel is read through the
/// correspondence's own projection.
// The 9 arguments are the blend lowering's: the correspondence, the
// node and its operand, the payload, and the evaluation environment.
#[allow(clippy::too_many_arguments)]
fn wire_shell<
    T: Decide
        + geom_core::Bounds
        + geom_brep::PcurveFittedLane
        + crate::lane::Lane
        + topo::AtRestPolicy,
>(
    verb: &crate::verbs::shell::ShellVerb<T>,
    id: RecipeNodeId,
    target: RecipeNodeId,
    open: &[names::StableName],
    doc: &crate::doc::Doc<ProfileProgram>,
    results: &Results<T>,
    vals: &SlotValues<T>,
    env: &OpEnv<'_, T>,
    tol: Tol,
) -> OpResult<T> {
    let body = body_operand(results, target)?;
    let thickness = need_scalar(vals, verb.slots.size_slot)?;
    let target_table = Arc::clone(&value_of(results, target)?.name_table);
    let faces = resolve_open_faces(open, doc, &target_table)?;
    let built = (verb.build)(faces, thickness);
    // The verb's own declaration of where its scalar lands, read off
    // the value the correspondence just built (VERB-SEAT-DESIGN V1).
    let flow = built.param_flow();
    // The door is the scalar's own answer, read at the ONE seam that
    // holds it; a scalar that may not certify has none and refuses
    // here rather than at an unvalidated hollow.
    let door =
        <T as topo::AtRestPolicy>::shell_door().ok_or(NodeErrorKind::ShellLaneUnsupported {
            lane: <T as crate::lane::Lane>::NAME,
        })?;
    let out = built.run_shell(&body, tol, door).map_err(verb_refused)?;
    let rec = crate::verbs::read_record(out.record, verb.record, verb.foreign_record)?;
    let table = (verb.emitter)(id, target, &target_table, &out.body, &rec)
        .map_err(NodeErrorKind::Naming)?;
    let mut body = out.body;
    // The cavity's surfaces, curves and points and the rims' rings are
    // minted HERE (D1/N6); the outer wall's pass-through descriptions
    // keep the source they arrived with.
    stamp_minted(&mut body, id);
    // **Attach-at-mint for the lowered parameter-identity channel**
    // (VERB-SEAT-DESIGN P2), through the shell's own attach door,
    // driven by the verb's DECLARED flow. The shell's row is declared
    // EMPTY today (its thickness becomes `r − t`, the identity of
    // neither), so there is nothing to lower and nothing to stamp:
    // the lowering is skipped, not performed onto nothing. The day the
    // seat's row names a field, the token is lowered here and
    // `attach_shell` is the placeholder that row will have to fill.
    if crate::param_source::flow_bearing(verb.slots.size_param)
        && let Some(expr) = doc.node(id).and_then(|n| n.expr(verb.slots.size_slot))
    {
        let scope = crate::param_source::ParamScope::of(doc.id(), env.parts.chain());
        crate::param_source::attach_shell(
            &mut body,
            flow,
            verb.slots.size_param,
            &crate::param_source::lower(scope, expr),
            &rec,
        )
        .map_err(NodeErrorKind::ParamSourceAttach)?;
    }
    Ok(OpOut::plain(ValuePayload::Body(Arc::new(body)), table))
}

/// Resolves a shell's open-face designation against the target's name
/// table — [`resolve_selection`]'s twin over FACES, through the same
/// [`ladder`] and the same [`named_entity`] door, with two differences
/// that are this door's own arity: an
/// empty list is legal (the sealed hollow), and the keys come back in
/// DESIGNATION ORDER rather than arena order. D9's arena-order rule is
/// for DERIVED lists; here the order is authored data the kernel reads
/// (the first designated face of a chart carries its rim), so
/// re-sorting it would silently move a rim.
///
/// A repeated designation cannot arrive here: the construction door
/// deduplicates, and the insert door and the load door both refuse a
/// repeat through `Node::input_fault`. If one did, the kernel would
/// refuse it itself (`OpenFaceRepeated`).
fn resolve_open_faces(
    open: &[names::StableName],
    doc: &crate::doc::Doc<ProfileProgram>,
    target: &NameTable,
) -> Result<Vec<topo::FaceKey>, NodeErrorKind> {
    let mut keys = Vec::with_capacity(open.len());
    for name in open {
        keys.push(named_entity(
            name,
            doc,
            target,
            |error| NodeErrorKind::ShellOpenResolve { error },
            names::EntityKey::face,
            |name, found| NodeErrorKind::ShellOpenKind { name, found },
        )?);
    }
    Ok(keys)
}

/// The mid-evaluation N5 refusal ladder, shared by every door that
/// resolves an AUTHORED name against the tables the run has built so
/// far ([`resolve_selection`], [`resolve_open_faces`],
/// [`resolve_declarations`]).
///
/// Mid-evaluation there is no prior run and no whole-evaluation
/// index, so [`mod@crate::resolve`]'s full ladder does not apply:
/// what is left is three rungs, numbered here in the order the
/// ONE-TABLE doors ask them ([`ladder::resolve_in`] walks exactly
/// this).
///
/// 1. [`ladder::live`] — the minting node must still be in the
///    document. Ids are never reused, so an id below the mint counter
///    was DELETED and one at/above it was never this document's
///    (`ForeignNode`). This rung outranks every later refusal,
///    including a door's own, and the [`ladder::Live`] token enforces
///    that rather than asking for it: reading a table needs the token,
///    so no refusal ABOUT the tables — the ladder's own rungs, or a
///    door's own — can be reached before this one has passed.
/// 2. [`ladder::Landing::Tied`] → `Ambiguous`. The tie row IS the
///    ambiguity (N5), so the tied set expressed in names is the name
///    itself, and the witness carries the multiplicity and the
///    minting site.
/// 3. [`ladder::Landing::Absent`] → `Vanished`, through
///    [`ladder::vanished`]: no prior run is consultable
///    mid-evaluation, so there is no evidence to weigh and nothing to
///    bank, which is exactly the payload that constructor names.
///
/// **Rung 2 and rung 3 are ordered by the DOOR, not by this list.**
/// [`resolve_declarations`] asks 1, 3, its pair's kind question, then
/// 2: a pair the vocabulary has no step for is unsupported however
/// many entities answer to either name, so the kind question outranks
/// the tie there, and the argument is written at that door. Rung 1
/// outranks both at every door, which is the part the [`ladder::Live`]
/// token enforces.
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
        Tied(usize),
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
    #[derive(Clone, Copy, Debug, PartialEq, Eq)]
    pub(super) struct Live<'n>(&'n StableName);

    impl<'n> Live<'n> {
        /// The name rung 1 was paid on. Copying the token copies the
        /// proof, which is sound because the proof is about a name
        /// that cannot change under it.
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
    /// through `refuse` in the caller's own vocabulary. The three
    /// one-table doors (a blend's selection, a measure's reference, a
    /// derived frame's face) are this function; the declare door
    /// walks the rungs itself because it reads TWO tables between
    /// rung 1 and rung 3 and picks a side in between.
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
            // The tie row is the name itself: a door resolves the
            // authored name against one table, so there is no widened
            // base to tie against here.
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
    /// Split out of [`resolve`] for the declare door, which asks its
    /// PAIR's kind question between rung 3 and rung 2 and so reaches
    /// this rung on its own — through this function rather than
    /// through a second spelling of the same refusal.
    pub(super) fn vanished(live: &Live<'_>) -> Box<ResolveError> {
        Box::new(ResolveError::vanished_fallback(live.0))
    }
}

/// **The entity-kind question asked of an authored NAME** — the
/// designation road, for every door that reads a name out of the
/// recipe: resolve it through the [`ladder`] first, then hand the key
/// to [`super::entity_door::entity`].
///
/// It is a door of its own rather than a second copy because the name
/// is a second thing the refusal CARRIES, not a second way of asking:
/// all three of these refusals name the offending designation so the
/// author knows which of a list failed, and the boxed clone that puts
/// it there is made here, once, rather than at each road.
///
/// `unresolved` is the road's N5 vocabulary and `refuse` its kind
/// refusal; the two are separate because they are separate answers — a
/// name that stopped resolving is not a name of the wrong kind, and
/// rung 1 outranks this door entirely ([`ladder::Live`]).
///
/// The one thing neither this door nor its callers can supply is the
/// KIND: [`super::entity_door::Found`] is mintable only inside that
/// module, so `refuse` receives it and passes it on. **That is why the
/// door is in two files and this half is here**: the token's field has
/// to be private to a module that is not an ancestor of these roads,
/// and the roads are in this one. What this door adds is the
/// resolution and the boxed name; what it cannot add, and does not
/// try to, is the word.
///
/// The KEY, though, is this door's own — it comes off
/// `ladder::resolve_in` two lines below and nowhere else, which is the
/// property `entity_door`'s module docs say the type system does not
/// carry.
///
/// # Errors
///
/// The [`ladder`]'s closed N5 trio through `unresolved`, and `refuse`'s
/// own refusal when `read` finds the name denotes another kind.
fn named_entity<R>(
    name: &names::StableName,
    doc: &crate::doc::Doc<ProfileProgram>,
    table: &NameTable,
    unresolved: impl Fn(Box<crate::resolve::ResolveError>) -> NodeErrorKind,
    read: fn(names::EntityKey) -> Option<R>,
    refuse: impl FnOnce(Box<names::StableName>, super::entity_door::Found) -> NodeErrorKind,
) -> Result<R, NodeErrorKind> {
    let ent = ladder::resolve_in(name, doc, table, unresolved)?;
    super::entity_door::entity(ent.key, read, |found| refuse(Box::new(name.clone()), found))
}

/// Resolves a fillet's edge selection against the target's name table
/// (M6-5). Single-operand, so simpler than
/// [`resolve_declarations`] — but the refusal vocabulary is the SAME
/// N5 trio, deliberately: the two sites answer the same question, and
/// they answer it through the same [`ladder`], which owns rung order
/// and payload shapes. What stays here is this door's arity — one
/// table — and which kind it reads for: a selection names EDGES, and
/// the test and its refusal go through [`named_entity`].
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
    if selection.is_empty() {
        return Err(NodeErrorKind::BlendSelectionEmpty { verb });
    }
    let mut keys = Vec::with_capacity(selection.len());
    for name in selection {
        keys.push(named_entity(
            name,
            doc,
            target,
            |error| NodeErrorKind::BlendSelectionResolve { verb, error },
            names::EntityKey::edge,
            |name, found| NodeErrorKind::BlendSelectionKind { verb, name, found },
        )?);
    }
    // D9 order; the kernel refuses a repeated edge itself, so a
    // duplicate that survived canonicalization still fails loudly.
    keys.sort_unstable();
    Ok(keys)
}

/// One resolved measure reference, as a SELECTION rather than as a
/// carrier: the body the name landed in, where it was read, and which
/// entity it is.
///
/// [`super::measure::Carrier`] is the closed forms' view of the same
/// resolution — a point, a plane, an axis. `min_clearance` needs the
/// other view (a body and a face scope), and both come off one ladder
/// walk in [`wire_measure`] rather than off two.
struct Selected<'v, T: Decide> {
    at: RecipeNodeId,
    index: u32,
    body: &'v topo::Body<T>,
    key: crate::names::EntityKey,
}

/// What a measure reference is allowed to scope over — the whole body,
/// or one face of it.
///
/// It exists so [`Selected::faces`]'s projection can be a `fn`: the
/// entity door takes a `fn` so that no `read` can answer from a key it
/// captured rather than the one the door holds, which means the body
/// work has to happen after the door rather than inside it. The two
/// arms are the two admitted kinds, so neither this enum nor the match
/// below has an unreachable case.
enum Scope {
    /// A body-kind reference: every face of it.
    WholeBody,
    /// A face-kind reference: that one face.
    One(topo::entity::FaceKey),
}

/// The scope a key denotes, or `None` for a kind that is neither — the
/// entity door's `read` for the measure road.
fn scope_of(key: names::EntityKey) -> Option<Scope> {
    match key {
        names::EntityKey::Body => Some(Scope::WholeBody),
        names::EntityKey::Face(k) => Some(Scope::One(k)),
        names::EntityKey::Edge(_) | names::EntityKey::Vertex(_) => None,
    }
}

impl<T: Decide> Selected<'_, T> {
    /// The faces this selection scopes over: every face of the body for
    /// a body-kind reference (arena order, which is the deterministic
    /// order every derived list in this kernel inherits), the one face
    /// for a face-kind reference, and a typed refusal for anything
    /// else.
    ///
    /// # Errors
    ///
    /// [`NodeErrorKind::MeasureSelectionKind`], naming what was
    /// selected instead.
    fn faces(&self) -> Result<Vec<topo::entity::FaceKey>, NodeErrorKind> {
        let scope = super::entity_door::entity(self.key, scope_of, |found| {
            NodeErrorKind::MeasureSelectionKind {
                verb: "min_clearance",
                found,
            }
        })?;
        Ok(match scope {
            Scope::WholeBody => self.body.faces().map(|(k, _)| k).collect(),
            Scope::One(k) => vec![k],
        })
    }
}

/// **A measurement sink** (E3): resolve the node's references, read
/// the carriers they sit on, run the closed form, hand back a typed F1
/// quantity. No body in, no body out.
///
/// # Where a reference resolves
///
/// At the node the reference NAMES AS ITS READING SITE
/// ([`crate::SitedRef::at`]), which is what makes the answer the
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
fn wire_measure<T: Decide + crate::measure::MinClearanceLane>(
    node: &Node<ProfileProgram>,
    expr: &crate::measure::MeasureExpr,
    refs: &[crate::node::SitedRef],
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
    for prim in &prims {
        read.extend(prim.refs());
    }
    let mut carriers = Vec::with_capacity(refs.len());
    // The SELECTION half of the same resolution, kept beside the
    // carrier half rather than resolved a second time: `min_clearance`
    // is about a body and a face scope where every other primitive is
    // about a carrier, and both are read off the one ladder walk below.
    let mut selections: Vec<Option<Selected<'_, T>>> = Vec::with_capacity(refs.len());
    for (index, r) in refs.iter().enumerate() {
        // A primitive names a reference by a `u32` index, so one at a
        // position past `u32::MAX` is one no primitive reads.
        if !u32::try_from(index).is_ok_and(|i| read.contains(&i)) {
            carriers.push(super::measure::Carrier::Unread);
            selections.push(None);
            continue;
        }
        let name = &r.name;
        let value = value_of(results, r.at)?;
        let ent = ladder::resolve_in(name, doc, &value.name_table, |error| {
            NodeErrorKind::MeasureRefResolve { error }
        })?;
        let body =
            crate::names::interrogate::output_body(&value.payload, ent.body).map_err(|error| {
                NodeErrorKind::MeasureRefUnreadable {
                    name: Box::new(name.clone()),
                    error,
                }
            })?;
        carriers.push(super::measure::carrier_of(body, ent));
        selections.push(Some(Selected {
            at: r.at,
            index: ent.body,
            body,
            key: ent.key,
        }));
    }
    // The `min_clearance` leaves, in the SAME pre-order the evaluation
    // walk reads them back in — one order, two consumers, exactly as
    // the value leaves are. Computed here because this is where the
    // bodies are: the engine wants the geometry this evaluation
    // already built, and re-entering `evaluate` to find it again would
    // be a second lane of the same document.
    let mut clearances = Vec::new();
    for prim in &prims {
        let crate::measure::MeasurePrimitive::MinClearance { a, b } = prim else {
            continue;
        };
        let clearance_side =
            |i: &u32| -> Result<crate::measure::MinClearanceOperand<'_, T>, NodeErrorKind> {
                // Bounds are the node door's and the load door's; a miss
                // here is the same kernel bug `eval_measure` announces.
                let Some(Some(sel)) = selections.get(*i as usize) else {
                    unreachable!(
                        "`min_clearance` reads reference {i} of {} resolved selections, yet \
                     `Node::measure_fault` bounds every index at both doors and the read set \
                     is computed from these very primitives",
                        selections.len()
                    )
                };
                Ok(crate::measure::MinClearanceOperand {
                    at: sel.at,
                    index: sel.index,
                    body: sel.body,
                    faces: sel.faces()?,
                })
            };
        let (oa, ob) = (clearance_side(a)?, clearance_side(b)?);
        match T::min_separation(&oa, &ob) {
            Some(Ok(v)) => clearances.push(v),
            Some(Err(refusal)) => return Err(NodeErrorKind::MeasureClearanceRefused(refusal)),
            // **The typed absence, and the whole node takes it.** A
            // measured expression is one number; when one of its leaves
            // has no value at this scalar, neither does the expression,
            // and saying so at the node is what lets an assertion over
            // it report `Unevaluated` instead of being poisoned.
            None => {
                return Ok(OpOut::plain(
                    ValuePayload::MeasureUnavailable {
                        reason: crate::measure::MeasureUnavailableAt::NeedsEnclosure {
                            verb: prim.verb(),
                            scalar: <T as crate::lane::Lane>::NAME,
                            door: "clearance::min_separation",
                        },
                        dim: expr.dim(),
                    },
                    names::empty(),
                ));
            }
        }
    }
    let mut cursor = 0usize;
    let mut clearance_cursor = 0usize;
    let value = super::measure::eval_measure(
        expr,
        &carriers,
        leaves.unwrap_or(&[]),
        &mut cursor,
        &clearances,
        &mut clearance_cursor,
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
    debug_assert_eq!(clearance_cursor, clearances.len());
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
    certified: crate::measure::Certified,
    payload_values: Option<&[T]>,
    results: &Results<T>,
    tol: Tol,
) -> OpResult<T> {
    let mv = value_of(results, measure)?;
    // **The typed absence, reported rather than propagated.** A measure
    // with no value at this scalar is not a failed node — it built, and
    // said what it could not say — so the assertion answers with E10's
    // third state carrying that reason, and the recorded requirement
    // stays visible in a build that cannot check it. The dimension
    // check below still has to run somewhere; it runs at the scalar
    // that HAS a value, which is where a comparison exists to be
    // ill-dimensioned.
    if let ValuePayload::MeasureUnavailable { reason, .. } = &mv.payload {
        return Ok(OpOut::plain(
            ValuePayload::Assertion(crate::measure::AssertionVerdict::Unevaluated {
                reason: crate::measure::UnevaluatedReason::MeasureUnavailable(*reason),
            }),
            names::empty(),
        ));
    }
    let ValuePayload::Measure { value, dim } = &mv.payload else {
        return Err(wrong_operand(mv, measure, super::family::MEASURE));
    };
    // The bound's DECLARED dimension is what must agree — read off the
    // expression, never inferred from the evaluated number, which has
    // no dimension left (units erase at the evaluation boundary).
    //
    // E10's agreement is `AssertionBoundFault::against`, the same rule
    // the two document doors ask through
    // `Node::assertion_bound_fault`: this seat reaches it by the
    // measured-dimension entry point because it has no document to
    // resolve the reference in, only the measure's evaluated payload —
    // which carries the dimension that node's own expression declared.
    // The fault's other arm cannot arise here: a reference that is not
    // a measure has already failed the operand-kind check above.
    if crate::node::AssertionBoundFault::against(measure, *dim, bound_expr.dim()).is_some() {
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
            certified,
        )),
        names::empty(),
    ))
}

/// **The split's lowering**, driven by its correspondence
/// ([`crate::verbs::split`]) — a fourth lowering body, beside the
/// three the other doors have, because the split matches none of
/// their shapes: one body and one DATUM operand in (no selection, no
/// slot), TWO sides out under one record, and a provenance stamp that
/// runs across both sides in one index space.
///
/// The shape: read the body operand, read the tool operand as a
/// datum and ask the correspondence for the plane it is, build the
/// kernel verb, run it through the split door, take the record out of
/// the closed channel, stamp both sides, emit names from the record
/// and the two sides under THIS node's id. What the correspondence
/// supplies is the datum reading and its refusal label, the verb
/// constructor, the emitter, and what to call a wrong-family record.
///
/// # Refusals
///
/// A tool that is not a plane datum is `WrongOperand` — the document's
/// own semantics, decided here before any verb exists, exactly as the
/// boolean's declarations resolve upstairs. Failure of the op itself is
/// a TYPED refusal ([`NodeErrorKind::Split`]) carrying the kernel's own
/// error unaltered, through [`verb_refused`]. The D7 pinch lane lives
/// inside the kernel door and is reached through the verb door
/// unchanged; nothing here re-derives the plane or its orientation.
fn wire_split<
    T: Decide
        + geom_core::Bounds
        + geom_brep::PcurveFittedLane
        + crate::lane::Lane
        + topo::AtRestPolicy,
>(
    verb: &crate::verbs::split::SplitVerb<T>,
    id: RecipeNodeId,
    target: RecipeNodeId,
    tool: RecipeNodeId,
    results: &Results<T>,
    tol: Tol,
) -> OpResult<T> {
    let body = body_operand(results, target)?;
    let tv = value_of(results, tool)?;
    let wrong_tool = || wrong_operand(tv, tool, verb.tool_expected);
    let ValuePayload::Datum(datum) = &tv.payload else {
        return Err(wrong_tool());
    };
    let plane = (verb.tool)(datum).ok_or_else(wrong_tool)?;
    let built = (verb.build)(plane);
    let out = built.run_split(&body, tol).map_err(verb_refused)?;
    let naming = crate::verbs::read_record(out.record, verb.record, verb.foreign_record)?;
    // Pass-through descriptions keep their sources (the clone carried
    // them); the split's fresh section planes get THIS node's (D1) —
    // in ONE index space across both halves. Each half's section
    // plane is its own description with its own outward normal, and
    // the two are the operands of any boolean that joins the halves
    // back together: a source shared between them would read as one
    // plane at that boolean's rung 1 while the bits say two. The
    // counter carried from the first side into the second is what
    // keeps the two spaces one; the split digest rows red if it is
    // dropped.
    let mut next = 0u32;
    let mut side = |part: SplitPart<T>| match part {
        SplitPart::Body(mut b) => {
            next = stamp_minted_from(&mut b, id, next);
            SplitSide::Body(Arc::new(b))
        }
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
        plane.normal,
        tol,
    )
    .map_err(NodeErrorKind::Naming)?;
    Ok(
        OpOut::plain(ValuePayload::Split { above, below }, emitted.table)
            .grouped(Arc::new(names::FragmentGroups::minted(&emitted.groups))),
    )
}

/// **The projection node** (DM3): ONE body out of a split's or a
/// pattern's value, as the `Body` value every consumer already takes.
///
/// The selector and the value must agree in kind — a half against a
/// `Split`, an index against `Instances` — and any other pairing
/// refuses `WrongOperand` through the same door `body_operand` uses.
/// A single body is NOT admitted as its own instance 0: nothing is
/// several bodies until a node says so ("wire, don't invent", D3).
///
/// The body handed on is the half's or the instance's own `Arc` — no
/// clone, no re-stamp, no transform — so every consumer sees exactly
/// the body the split or the pattern minted. The table is the input's
/// PROJECTED onto that body ([`NameTable::project`]): the selected
/// body's rows, re-keyed to body 0, names verbatim. The projection
/// mints nothing and adds no segment (`wire_transform`'s
/// identity-preserving rule), so a selector spelled against the
/// split's `SplitBody(half)` rows or the pattern's `Instance { i, .. }`
/// rows resolves here unchanged — and one spelled for another instance
/// finds no row and refuses through the N5 ladder as absent, never
/// re-anchored. Totality is re-checked against the projected body:
/// `check_total` stays the tripwire that the projection dropped
/// nothing the body still has.
///
/// No number is compared to decide anything here: the half is a tag,
/// and the index is a structural count checked against a length.
fn wire_part<T: Decide>(
    of: RecipeNodeId,
    select: &PartSelect,
    results: &Results<T>,
    vals: &SlotValues<T>,
) -> OpResult<T> {
    let value = value_of(results, of)?;
    let (body, index) = match (select, &value.payload) {
        (PartSelect::SplitHalf(half), ValuePayload::Split { above, below }) => {
            let side = match half {
                SplitHalf::Above => above,
                SplitHalf::Below => below,
            };
            match side {
                SplitSide::Body(b) => (Arc::clone(b), half.output_body()),
                SplitSide::Empty => {
                    return Err(NodeErrorKind::EmptyHalf {
                        input: of,
                        half: *half,
                    });
                }
            }
        }
        (PartSelect::Instance(_), ValuePayload::Instances(instances)) => {
            let index = slots::count(vals, SlotId::Instance).ok_or(NodeErrorKind::MissingSlot {
                slot: SlotId::Instance,
            })?;
            // The index is into the value's FLAT list: over a nested
            // pattern's value, body `j·M + i` (placement `j` of the
            // inner instance `i`, the layout `wire_pattern` fixes),
            // and the out-of-range refusal reads the flat count.
            // The count is a u32 quantity in every table row (a name's
            // output-body index), so a value past that is the
            // pattern's own emission bug, refused typed before any
            // index is judged against it.
            let count = names::output_body(instances.len()).map_err(NodeErrorKind::Naming)?;
            // ONE refusal, one fold: a negative index and one past the
            // end fail the same way, and an index the fold admits is
            // in range by construction.
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
    Ok(OpOut::plain(ValuePayload::Body(body), Arc::new(table)))
}

// `Bounds` rides along for the boolean lane only: the sweep's
// BVH candidate generation reads coordinate brackets — the L7 driver-code
// allowance, threaded from `run_op`'s service bound.
//
// The TWO-OPERAND generic lowering, beside `wire_blend`'s one-operand
// shape rather than folded into it: the boolean's document semantics
// have more upstairs than a blend's — two operand tables, the
// `declare` input's N5 resolution, the declared-contact carry into the
// boolean VALUE, and the typed empty success — and a lowering generic
// over operand COUNT would trade those typed shapes for runtime arity.
// The correspondence (`crate::verbs::boolean`) supplies what varies
// per pair verb: the verb constructor and the naming emitter.
#[allow(clippy::too_many_arguments)] // one parameter per named input; strategy is the §4.4 door
fn wire_boolean<
    T: Decide
        + geom_core::Bounds
        + geom_brep::PcurveFittedLane
        + crate::lane::Lane
        + topo::AtRestPolicy,
>(
    verb: &crate::verbs::boolean::PairVerb<T>,
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
    // F5 threading: the Declare input's name pairs resolve
    // through the OPERANDS' name tables into the kernel's declared
    // coincidence data. Resolution failures are the N5 typed errors —
    // no silent drop, no best-effort gluing. This stays upstairs: it
    // is the document's semantics (names, freezes, refusal payloads),
    // and the kernel verb receives only the lowered arena-key form.
    // Both operand tables are read three times downstairs — by the
    // declare resolution, by the refusal menu and by the emitter — so
    // they are taken once here rather than re-fetched per reader.
    let a_table = Arc::clone(&value_of(results, a)?.name_table);
    let b_table = Arc::clone(&value_of(results, b)?.name_table);
    // **The site is the side.** Each declared entity names the
    // operand it is read at, so each name is resolved in the ONE table
    // that site designates and a name carried by both operands is no
    // longer ambiguous — two placements of one prototype are
    // declarable through this door. A site that is neither operand
    // refuses typed.
    let kernel_decls = match declare {
        None => BooleanDeclarations::none(),
        Some(d) => {
            let sided = side_by_operand(declared_pairs(results, d)?, a, b, doc)?;
            resolve_declarations(&sided, doc, &a_table, &b_table)?
        }
    };
    let body_a = body_operand(results, a)?;
    let body_b = body_operand(results, b)?;
    match (verb.build)(op, kernel_decls)
        .run_pair(&body_a, &body_b, boolean_sweep, tol)
        .map_err(|err| refusal_menu((a, &a_table), (b, &b_table), err))?
    {
        verbs::PairOut::Empty => Ok(OpOut::plain(
            ValuePayload::Boolean(BooleanValue::Empty),
            names::empty(),
        )),
        verbs::PairOut::Out(out) => {
            // Per-family record channel; another family from a
            // boolean run is a kernel bug, refused typed
            // (`wire_blend`'s clause, mirrored). Exhaustive with no
            // wildcard arm (D3): a new record family breaks this
            // consumer at compile time rather than routing silently
            // to the refusal.
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
            let mut body = out.body;
            // Seam chords / minted descriptions get THIS node's
            // sources; everything carried keeps its own (D1).
            stamp_minted(&mut body, id);
            Ok(OpOut::plain(
                ValuePayload::Boolean(BooleanValue::Body {
                    body: Arc::new(body),
                    kind,
                    contacts: Arc::new(contacts),
                }),
                emitted.table,
            )
            .grouped(Arc::new(names::FragmentGroups::minted(&emitted.groups))))
        }
    }
}

/// **The n-ary union's lowering** (DM4): the SAME pair verb, folded
/// over the members in list order — `((m0 ∪ m1) ∪ m2) ∪ …`, through
/// the same `run_pair` door, the same refusal menu, and one body out
/// in the same `BooleanValue::Body` shape a pair union yields, so
/// every consumer of a union is unchanged.
///
/// No new numeric decision is taken anywhere here: the geometry is the
/// pair verb's at every step, which is what makes the fold and the
/// chain it replaces the same body. What the node adds is the NAMING
/// — the fold's own tables record join depth, and `names::name_union`
/// rewrites the last one into member-keyed names.
///
/// **Declarations are routed, not positioned** (DM4 as re-ruled). The
/// node's optional `Declare` input names SITED entities — a member
/// and that member's own name for the entity — and
/// [`route_declarations`] sends each pair to the one step that joins
/// its two sites, derived from where those members sit in the list. A
/// step's bucket is rewritten into this node's member space and
/// resolved by the pair boolean's own [`resolve_declarations`]
/// against that step's two tables, so the union adds the routing and
/// reuses the door.
///
/// **Nothing ∅-absorbing is invented** (D3, "wire, don't invent"). A
/// member that evaluates to an empty boolean refuses `EmptyOperand`
/// naming that member, exactly as `body_operand` refuses one for a
/// pair. An empty INTERMEDIATE — which two non-empty operands cannot
/// produce under union, so this is a kernel-bug path rather than an
/// authoring one — is the empty operand the next step would be handed,
/// and refuses the same way, naming the member the fold had reached;
/// at the LAST step it is the typed empty success a pair union already
/// has.
// The allow is `wire_boolean`'s, for its reason: one parameter per
// named input, and the declare edge is one of them.
#[allow(clippy::too_many_arguments)]
fn wire_union<
    T: Decide
        + geom_core::Bounds
        + geom_brep::PcurveFittedLane
        + crate::lane::Lane
        + topo::AtRestPolicy,
>(
    verb: &crate::verbs::boolean::PairVerb<T>,
    id: RecipeNodeId,
    members: &[RecipeNodeId],
    declare: Option<RecipeNodeId>,
    doc: &crate::doc::Doc<ProfileProgram>,
    results: &Results<T>,
    boolean_sweep: topo::SweepStrategy,
    tol: Tol,
) -> OpResult<T> {
    // Two or more is the node's contract, held at both edit doors
    // (`EditError::TooFewMembers`). Reaching here with fewer means the
    // fold has no pair to hand the verb, which is the arity class this
    // crate already refuses typed — never a panic, and never a
    // one-member "union" that silently denotes its own input.
    let Some((first, rest)) = members.split_first().filter(|(_, rest)| !rest.is_empty()) else {
        return Err(NodeErrorKind::VerbArity {
            verb: verbs::VerbKind::Boolean(BooleanOp::Union),
            given: verbs::Arity::One,
        });
    };
    let mut acc_body = body_operand(results, *first)?;
    // The FIRST member enters member-keyed too, so every operand of
    // every step is already in this node's name space and nothing
    // downstream has to recover a member from an inner name.
    let mut acc_table = Arc::new(
        names::member_view(id, *first, &value_of(results, *first)?.name_table)
            .map_err(NodeErrorKind::Naming)?,
    );
    // The declaration channel, routed BEFORE the fold: each pair goes
    // to the one step that joins the two things it names, derived from
    // the member ids its names carry (`route_declarations`). One bucket
    // per step, so a step with no declared pair runs exactly as it did
    // without the input.
    let buckets: Vec<Vec<SidedPair<'static>>> = match declare {
        None => vec![Vec::new(); rest.len()],
        Some(d) => route_declarations(id, members, declared_pairs(results, d)?, doc)?,
    };
    let mut last: Option<(topo::BooleanResultKind, Arc<topo::ContactRecords>)> = None;
    // Each step's fragment groups, in fold order (`FragmentGroups::folded`).
    let mut step_groups = Vec::with_capacity(rest.len());
    for (step, member) in rest.iter().enumerate() {
        let member_body = body_operand(results, *member)?;
        let member_table = Arc::new(
            names::member_view(id, *member, &value_of(results, *member)?.name_table)
                .map_err(NodeErrorKind::Naming)?,
        );
        // This step's declared pairs, resolved against the two tables
        // the step actually joins by the SAME door the pair boolean
        // resolves its own through — side-picking included, so a name
        // in neither table or in both refuses there and not here.
        //
        // The accumulation is presented COLLAPSED. Its own rows are
        // `FromA`/`FromB`-headed, which is the fold's internal space and
        // denotes nothing outside it; a declaration answers what this
        // node's refusal named, and a refusal names collapsed rows
        // (`union_refusal`). So the door reads the accumulation in the
        // one space a caller can write. The collapse is not the whole of
        // what the published table gets: `names::name_union` then
        // renumbers the pieces of member EDGES over the finished body,
        // which a step that has not finished does not have. A declared
        // pair cannot name an edge at all: `declared_step` admits face
        // and vertex pairs only, and refuses any other
        // (`DeclareUnsupportedPair`).
        //
        // A member-space name the fold has already merged away is
        // rewritten to the accumulation's `Merged` row that holds it
        // (`look_through_merges`) before the door runs, so the door
        // itself stays the pair boolean's. A refusal it raises is
        // diagnosed against the AUTHORED bucket: the name that fails is
        // one the rewrite left alone, and the pair a caller acts on is
        // the one they wrote.
        let decls = if buckets[step].is_empty() {
            BooleanDeclarations::none()
        } else {
            let acc_view = names::collapse_table(id, &acc_table).map_err(NodeErrorKind::Naming)?;
            let resolved = look_through_merges(&buckets[step], &acc_view)?;
            resolve_declarations(&resolved, doc, &acc_view, &member_table)?
        };
        match (verb.build)(BooleanOp::Union, decls)
            .run_pair(&acc_body, &member_body, boolean_sweep, tol)
            .map_err(|err| union_refusal(id, members, &acc_table, &member_table, err))?
        {
            // A union of two REAL bodies cannot be empty, and both
            // operands here are real: `body_operand` refuses a member
            // whose value is the typed empty before this line, and the
            // accumulation is a body the previous step returned. So
            // this arm is a kernel bug and is refused as one — typed,
            // in the same channel the foreign-record check below uses.
            // It is NOT attributed to `member`: blaming an operand that
            // is not empty names the wrong node and sends a caller to
            // edit a member that is fine.
            verbs::PairOut::Empty => {
                return Err(NodeErrorKind::Naming(names::NamingError::Emission {
                    what: UNION_STEP_EMPTY,
                }));
            }
            verbs::PairOut::Out(out) => {
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
                last = Some((kind, Arc::new(contacts)));
                // The fold's own table, minted by the PAIR emitter
                // under THIS node's id: that id is what tells an
                // intermediate row from a member's own name when the
                // chain is collapsed, and it is the id the node's
                // names carry in the end anyway. Both operand
                // CONTEXTS name this node for the same reason: their
                // tables are the member-keyed views, so an error this
                // step raises about an operand is about a row in this
                // node's space.
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
                acc_table = emitted.table;
                step_groups.push(emitted.groups);
                acc_body = Arc::new(out.body);
            }
        }
    }
    // The members' own bodies and tables: where each member edge the
    // published table ranks pieces of is defined (`name_union`).
    let member_bodies = members
        .iter()
        .map(|&m| {
            Ok((
                m,
                body_operand(results, m)?,
                &value_of(results, m)?.name_table,
            ))
        })
        .collect::<Result<Vec<_>, NodeErrorKind>>()?;
    let member_views: Vec<names::UnionMember<'_, T>> = member_bodies
        .iter()
        .map(|(node, body, table)| names::UnionMember {
            node: *node,
            body,
            table,
        })
        .collect();
    let table = names::name_union(id, &acc_body, &acc_table, &member_views, tol)
        .map_err(NodeErrorKind::Naming)?;
    let mut body = (*acc_body).clone();
    // ONCE, over the finished body, and not per fold step: the stamp
    // numbers a node's minted descriptions from zero, so a second pass
    // would hand a later step's geometry an index an earlier step
    // already used. Everything carried from a member keeps its own
    // source (D1); a seam chord minted at any step gets this node's.
    stamp_minted(&mut body, id);
    // The LAST step's record is the result's: the kind says how the
    // body that came out was produced, and the body that came out is
    // that step's. Its contacts are that step's too — the contacts a
    // step discovers or a declaration carried into it — and the absent
    // case is the arity refusal above.
    //
    // So a contact fed at a step BEFORE the last does not reach the
    // value: it is threaded into that step's verb and consumed there,
    // and the record the next step returns is its own. The pairwise
    // chain this node replaces loses it identically — `wire_boolean`
    // publishes the outer boolean's record and drops the inner one's —
    // so this is the pair verb's carry rule showing through a fold,
    // not a rule the fold adds.
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
    .grouped(Arc::new(names::FragmentGroups::folded(id, &step_groups))))
}

/// One declared pair as the recipe carries it: the two SITED
/// entities and the contact class the author claimed for them.
type DeclaredPair = ((SitedRef, SitedRef), ContactClass);

/// One declared pair as the shared resolver takes it: each side's
/// name in the table of the operand its SITE picked, and the class.
///
/// The two doors build it differently and that is the whole of the
/// difference between them. [`wire_boolean`] reads each name as
/// authored, in the operand's own table ([`side_by_operand`]);
/// [`wire_union`] rewrites each into the node's member space and
/// picks the side from the member's position in the list
/// ([`route_declarations`]). What arrives at [`resolve_declarations`]
/// is the same shape either way, so "resolve a declared name at its
/// site" has one definition.
type SidedPair<'n> = (
    (topo::Operand, SidedName<'n>),
    (topo::Operand, SidedName<'n>),
    ContactClass,
);

/// One side's name on its way to the shared resolver, and whether
/// rung 1 is already paid on it.
///
/// The two doors differ here and nowhere else. A pair boolean's
/// operand tables are the OPERANDS' own, so the authored name travels
/// unchanged and the rung-1 check [`site_operand`] paid to rank
/// `NodeGone` above the site question is the same check the landing
/// needs — carried, not paid twice. A union's operand tables are
/// member-keyed views, so [`route_declarations`] mints a NEW name
/// ([`names::member_name`]) and [`look_through_merges`] may mint
/// another; rung 1 on the AUTHORED name is paid at the routing door,
/// and the minted name pays its own.
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
/// The site question is a question about which TABLE a name is read
/// in, so it ranks below rung 1 exactly as a landing does: a name
/// whose minting node is gone says THAT, whatever its site, at both
/// doors. Writing the order once is what keeps the two doors from
/// disagreeing about it, which is what they did.
///
/// What the two doors do NOT share is the refusal noun, and that is
/// the caller's: a pair boolean's operands are two named nodes, so a
/// site that is neither is a site fault
/// ([`NodeErrorKind::DeclareSiteNotAnOperand`]); a union's operands
/// are its member LIST, which `SetMembers` rewrites, so a site that
/// left it is the vanished name DM4 says it is. `absent` is handed
/// the rung-1 token so the union can mint that payload.
fn site_operand<'n>(
    r: &'n SitedRef,
    operands: &[RecipeNodeId],
    doc: &crate::doc::Doc<ProfileProgram>,
    absent: impl FnOnce(&ladder::Live<'n>) -> NodeErrorKind,
) -> Result<(usize, ladder::Live<'n>), NodeErrorKind> {
    let live =
        ladder::live(&r.name, doc).map_err(|error| NodeErrorKind::DeclareResolve { error })?;
    match operands.iter().position(|m| *m == r.at) {
        Some(i) => Ok((i, live)),
        None => Err(absent(&live)),
    }
}

/// **A pair boolean's declared pairs, sided by their sites** — `at ==
/// a` is operand A, `at == b` is operand B, and anything else refuses
/// typed.
///
/// The names travel unchanged: a pair boolean's operand tables are the
/// operands' own, so a declared name is already spelled in the table
/// its site designates, and the rung-1 token [`site_operand`] mints
/// travels with it. The union's door has to rewrite, because its
/// operand tables are member-keyed views.
fn side_by_operand<'n>(
    pairs: &'n [DeclaredPair],
    a: RecipeNodeId,
    b: RecipeNodeId,
    doc: &crate::doc::Doc<ProfileProgram>,
) -> Result<Vec<SidedPair<'n>>, NodeErrorKind> {
    let side = |r: &'n SitedRef| -> Result<(topo::Operand, SidedName<'n>), NodeErrorKind> {
        let (i, live) = site_operand(r, &[a, b], doc, |_| {
            NodeErrorKind::DeclareSiteNotAnOperand { at: r.at }
        })?;
        let op = if i == 0 {
            topo::Operand::A
        } else {
            topo::Operand::B
        };
        Ok((op, SidedName::Live(live)))
    };
    pairs
        .iter()
        .map(|((r1, r2), class)| Ok((side(r1)?, side(r2)?, *class)))
        .collect()
}

/// The pairs a `Declare` input carries, or the typed refusal for a
/// node wired at a declare seat that is not a `Declare`.
///
/// ONE definition, two callers ([`wire_boolean`] and [`wire_union`]):
/// the seat is the same seat, so its refusal is the same refusal. Both
/// edit doors refuse this shape before a document can hold it
/// (`Node::declare_input`), which makes this the evaluation's
/// defensive answer rather than its first line of defence — and a
/// defence that is written once cannot drift between the two nodes
/// that have the seat.
fn declared_pairs<T: Decide>(
    results: &Results<T>,
    declare: RecipeNodeId,
) -> Result<&[DeclaredPair], NodeErrorKind> {
    operand(
        results,
        declare,
        super::family::DECLARATIONS,
        |v| match &v.payload {
            ValuePayload::Declarations(pairs) => Some(&pairs[..]),
            _ => None,
        },
    )
}

/// **Routing a union's declared pairs to their fold steps** (DM4, the
/// declaration channel): one bucket per step, filled from the two
/// SITES the pair names and from nothing else.
///
/// Member `i` is the joining operand at step `i - 1` and is inside the
/// accumulation at every step after, so the step that has both sites
/// is the LATER member's: bucket `max(i, j) - 1`, the joining member
/// operand B and everything already accumulated operand A. A pair
/// whose two sites are ONE member is that member's carried contact at
/// its own step; member 0 is where the accumulation starts, which is
/// why the subtraction saturates rather than underflowing.
///
/// **Every sited pair has a step.** `max(i, j)` is at most
/// `members.len() - 1`, so the bucket is at most `steps - 1` and the
/// only refusals here are about the sites themselves. That rests on
/// the node's arity contract — two members or more, which
/// [`wire_union`] refuses before it calls this — and a one-member list
/// would index past the end rather than routing anything.
///
/// Each bucket's pair is rewritten into the node's member space
/// ([`names::member_name`], `member_view`'s one segment) and sided, so
/// the pair boolean's own resolver runs on a union's pairs exactly as
/// it runs on its own. A member face the fold has MERGED away is
/// rewritten again, at the step it is fed to, by
/// [`look_through_merges`]; the bound on that — splits, containment
/// and fragmented merges are not looked through — is DM4's and is
/// stated there.
///
/// A site the member list does not hold — the state `SetMembers`
/// creates by removing a declared member — refuses through the N5
/// ladder as a vanished name does, and nothing is dropped.
fn route_declarations(
    id: RecipeNodeId,
    members: &[RecipeNodeId],
    pairs: &[DeclaredPair],
    doc: &crate::doc::Doc<ProfileProgram>,
) -> Result<Vec<Vec<SidedPair<'static>>>, NodeErrorKind> {
    let steps = members.len().saturating_sub(1);
    // Every side a union routes is a name this door MINTED, so no
    // bucket borrows the payload it was built from.
    let mut buckets: Vec<Vec<SidedPair<'static>>> = vec![Vec::new(); steps];
    for ((r1, r2), class) in pairs {
        // Rung 1 first, as at both doors ([`site_operand`] is where
        // that order is written): a name whose minting node is gone
        // says THAT, before anything is said about which step it
        // would have belonged to.
        let member_of = |r: &SitedRef| -> Result<usize, NodeErrorKind> {
            site_operand(r, members, doc, |live| NodeErrorKind::DeclareResolve {
                error: ladder::vanished(live),
            })
            .map(|(i, _)| i)
        };
        let (i, j) = (member_of(r1)?, member_of(r2)?);
        // The bucket, and the side each name takes in it: the joining
        // member is operand B, and everything the fold has already
        // accumulated is operand A.
        let bucket = i.max(j).saturating_sub(1);
        let joining = bucket + 1;
        let sided = |index: usize, r: &SitedRef| {
            let op = if index == joining {
                topo::Operand::B
            } else {
                topo::Operand::A
            };
            // The rung-1 token is not carried past here: the name
            // this mints is a NEW one, minted under the union, and
            // the landing pays rung 1 on the name it actually reads.
            (
                op,
                SidedName::Rewritten(names::member_name(id, r.at, &r.name)),
            )
        };
        buckets[bucket].push((sided(i, r1), sided(j, r2), *class));
    }
    Ok(buckets)
}

/// **A member-space name the fold has merged away, rewritten to the
/// merged row that holds it** — one step's bucket, read against the
/// two tables that step joins, before the pair boolean's own door
/// ([`resolve_declarations`]) sees the pair.
///
/// A member's face merged at an earlier step is no longer an operand
/// row; the pair naming it still says what it said, and the face is
/// exactly one of the accumulation's `[Merged(set)]` rows by
/// membership (`names::merged::covers`), one face being in one row's
/// flat set. The rewrite reads the step's two tables and nothing else.
///
/// Two things it does not do. A name in no table and in no merged
/// row's set is left alone, and the door refuses it as the vanished
/// name it is. And a pair whose two names land on ONE row is handed to
/// the door as such, and refuses there by the door's own rule.
///
/// Only the ACCUMULATION side looks through, which is why the joining
/// member's table is not a parameter: no merge the fold has performed
/// could have consumed a face of a member that has not joined yet, so
/// a B-side name absent from that table is the vanished name it looks
/// like and the door below says so.
///
/// A face in the set of TWO merged rows cannot happen under the flat
/// mint — a merged face's constituents retire, and a merge over it
/// lists them in the new row's set and drops the old row — so meeting
/// one is refused as the emission bug it would be.
fn look_through_merges<'n>(
    bucket: &[SidedPair<'n>],
    acc_table: &NameTable,
) -> Result<Vec<SidedPair<'n>>, NodeErrorKind> {
    use crate::names::RoleSeg;
    let merged_row_of = |(op, sided): &(topo::Operand, SidedName<'n>)| -> Result<
        Option<names::StableName>,
        NodeErrorKind,
    > {
        let name = sided.name();
        if *op == topo::Operand::B || acc_table.lookup(name).is_some() {
            return Ok(None);
        }
        let mut rows = acc_table
            .iter()
            .filter_map(|(row, _)| match row.path.as_slice() {
                [RoleSeg::Merged(set)] if names::merged::covers(set, name) => Some(row),
                _ => None,
            });
        match (rows.next(), rows.next()) {
            (None, _) => Ok(None),
            (Some(row), None) => Ok(Some(row.clone())),
            (Some(_), Some(_)) => Err(NodeErrorKind::Naming(names::NamingError::Emission {
                what: MEMBER_FACE_IN_TWO_MERGES,
            })),
        }
    };
    bucket
        .iter()
        .map(|(s1, s2, class)| {
            let rewritten = |s: &(topo::Operand, SidedName<'n>)| {
                Ok(match merged_row_of(s)? {
                    Some(row) => (s.0, SidedName::Rewritten(row)),
                    None => (s.0, s.1.clone()),
                })
            };
            Ok((rewritten(s1)?, rewritten(s2)?, *class))
        })
        .collect()
}

/// A union's accumulation lists one member face in the constituent
/// sets of two merged rows, which the flat mint cannot produce.
const MEMBER_FACE_IN_TWO_MERGES: &str =
    "a union's accumulation holds one member face in two merged rows' constituent sets";

/// A union fold step returned the typed empty from two real bodies.
/// Unreachable (see the arm that raises it); surfaced typed.
const UNION_STEP_EMPTY: &str = "a union fold step returned empty from two non-empty operands";

/// A union's refusal, with every name it carries in the node's own
/// published space.
///
/// [`refusal_menu`] resolves the raise site's face keys through the two
/// OPERAND tables it is handed. From the second fold step on the `a`
/// side is the ACCUMULATED table — the pair emitter's, whose rows are
/// `FromA`/`FromB`-headed — so the name it finds is in the fold's
/// internal space: no published table holds it, [`mod@crate::resolve`]
/// cannot look it up, and a selector written against it matches
/// nothing. Every name the refusal carries is therefore put through
/// [`names::collapse_name`], the collapse the node's own table gets from
/// `name_union`, so a refusal denotes member-space entities and nothing
/// else.
///
/// The collapse is not all `name_union` does: it then numbers the
/// pieces of each member EDGE by the cells the finished body cuts it
/// into, and has seam vertices cite member edges whole. A refusal is
/// raised before there is a finished body, so a member-edge piece it
/// carried would keep the fold's rank, which no published table holds.
/// So one refuses as an emission bug ([`UNION_REFUSAL_FOLD_RANKED_EDGE`])
/// rather than being handed out; today none reaches it, since
/// `refusal_menu` resolves face keys and a flush finding names faces.
///
/// A name that will not collapse is an emission bug in the fold's own
/// table, and it is raised as one rather than swallowed: the union was
/// going to fail naming for the same reason had the step succeeded, and
/// a bug reported as a contact refusal would send a caller to edit
/// their model over a defect in this crate.
///
/// The recourse a caller whose members touch has is the pair boolean's,
/// on this node: wire a `Declare` naming the two entities to the
/// union's own `declare` input, each side SITED at the member that
/// carries it ([`sited_member`] reads the site off the published row).
/// A member's own face is handed back verbatim; a face the fold MERGED
/// is handed back as a constituent of that merge, which declares the
/// same contact because a declaration resolves through the fold's
/// merges ([`look_through_merges`]); and a row the fold minted that no
/// member stands for is refused
/// [`NodeErrorKind::UndeclarableContact`] — typed, because a sited
/// declaration cannot name a row that does not exist before the union.
///
/// So this door never degrades a user's undeclared contact into an
/// emission bug. The emission arm below is reached only when the
/// COLLAPSE itself refuses, which is the fold's own table being
/// malformed.
fn union_refusal<T: crate::lane::Lane>(
    id: RecipeNodeId,
    members: &[RecipeNodeId],
    a_table: &crate::names::NameTable,
    b_table: &crate::names::NameTable,
    err: verbs::VerbError<T>,
) -> NodeErrorKind {
    let refused = refusal_menu((id, a_table), (id, b_table), err);
    let NodeErrorKind::UndeclaredContact {
        finding,
        merged: _,
        diag,
    } = refused
    else {
        return refused;
    };
    let names::FlushFinding {
        pair: (a, b),
        class,
        evidence,
    } = *finding;
    let (Ok(a), Ok(b)) = (
        sited_member(id, members, &a.name),
        sited_member(id, members, &b.name),
    ) else {
        return NodeErrorKind::Naming(names::NamingError::Emission {
            what: UNION_REFUSAL_FOREIGN,
        });
    };
    // A fold-minted row is answered for FIRST, and the A side before
    // the B side: a refusal that has one has no pair to offer at all,
    // so there is nothing for the sited arm below to carry.
    for subject in [&a, &b] {
        if let DeclarationSubject::FoldMinted(row) = subject {
            // A piece of a member edge here carries the FOLD's rank,
            // which the published table renumbers over the finished
            // body: handing it out would name nothing. No flush finding
            // names an edge today; if one ever does, it refuses loudly.
            if names::is_fold_ranked_member_edge(row) {
                return NodeErrorKind::Naming(names::NamingError::Emission {
                    what: UNION_REFUSAL_FOLD_RANKED_EDGE,
                });
            }
            return NodeErrorKind::UndeclarableContact {
                row: Box::new(row.clone()),
                diag,
            };
        }
    }
    let (sa, ca) = a.declarable();
    let (sb, cb) = b.declarable();
    let (Some(sa), Some(sb)) = (sa, sb) else {
        // Unreachable: the loop above returned for every `FoldMinted`,
        // and the other two arms both have a site. Raised rather than
        // asserted, in the one channel this door has.
        return NodeErrorKind::Naming(names::NamingError::Emission {
            what: UNION_REFUSAL_FOREIGN,
        });
    };
    NodeErrorKind::UndeclaredContact {
        finding: Box::new(names::FlushFinding {
            pair: (sa, sb),
            class,
            evidence,
        }),
        merged: Box::new((ca, cb)),
        diag,
    }
}

/// **What one row of a union's fold space is, as a DECLARATION
/// SUBJECT** — [`sited_member`]'s answer, total over the rows a
/// refusal can name.
#[derive(Debug, Clone, PartialEq, Eq)]
enum DeclarationSubject {
    /// A member's own entity: declarable verbatim, sited at that
    /// member.
    Member(SitedRef),
    /// A face the fold MERGED. The row itself has no site — the union
    /// minted it — but every CONSTITUENT of its flat set (N3) is a
    /// member's entity, and a declaration written at any one of them
    /// resolves back to this row through [`look_through_merges`]. The
    /// set is ordered by the constituent's member in the union's own
    /// MEMBER ORDER (D9), so taking the first is a deterministic
    /// choice rather than an arbitrary one.
    Merged(Vec<SitedRef>),
    /// A row the fold minted that no member's entity stands for: a
    /// fragment, the union's own body, a merge none of whose
    /// constituents is a member's row. Carries the row itself, in the
    /// node's published space, because that is all a refusal about it
    /// can say.
    FoldMinted(names::StableName),
}

impl DeclarationSubject {
    /// The side a refusal carries and the merged set it came from:
    /// `(None, _)` only for [`DeclarationSubject::FoldMinted`], which
    /// has no site.
    fn declarable(&self) -> (Option<SitedRef>, Vec<SitedRef>) {
        match self {
            Self::Member(r) => (Some(r.clone()), Vec::new()),
            Self::Merged(set) => (set.first().cloned(), set.clone()),
            Self::FoldMinted(_) => (None, Vec::new()),
        }
    }
}

/// **One row of a union's fold space as the MEMBER entity it stands
/// for** — the member-keyed collapse, then the member edge read back
/// off it.
///
/// This is [`names::member_name`]'s inverse where one exists, and it
/// is what lets a union's refusal hand back a pair a caller can
/// declare: the refusal's names are rows of the fold's internal
/// space, the collapse puts them in this node's published space, and
/// a published row of a MEMBER's own entity is exactly one
/// `FromMember` segment, which says the site and the name at once.
///
/// It is TOTAL over the rows that collapse: a row with no member
/// entity behind it is [`DeclarationSubject::FoldMinted`] and the
/// caller must answer for it, rather than a `None` that reads like an
/// invariant break. The error case is the COLLAPSE's alone.
fn sited_member(
    id: RecipeNodeId,
    members: &[RecipeNodeId],
    name: &names::StableName,
) -> Result<DeclarationSubject, names::NamingError> {
    use crate::names::RoleSeg;
    let collapsed = names::collapse_name(id, name)?;
    let member_of = |n: &names::StableName| match n.path.as_slice() {
        [RoleSeg::FromMember { member, of }] => Some(SitedRef::new(*member, of.name().clone())),
        _ => None,
    };
    if let Some(r) = member_of(&collapsed) {
        return Ok(DeclarationSubject::Member(r));
    }
    Ok(match collapsed.path.as_slice() {
        [RoleSeg::Merged(set)] => {
            let mut sited: Vec<(usize, SitedRef)> = set
                .iter()
                .filter_map(member_of)
                .map(|r| {
                    (
                        members
                            .iter()
                            .position(|m| *m == r.at)
                            .unwrap_or(usize::MAX),
                        r,
                    )
                })
                .collect();
            // Member order (D9) is the union's list order, which is
            // data the node carries; a constituent whose member has
            // left the list sorts last and still declares the row.
            sited.sort_by(|(i, x), (j, y)| i.cmp(j).then_with(|| x.name.cmp(&y.name)));
            if sited.is_empty() {
                DeclarationSubject::FoldMinted(collapsed)
            } else {
                DeclarationSubject::Merged(sited.into_iter().map(|(_, r)| r).collect())
            }
        }
        _ => DeclarationSubject::FoldMinted(collapsed),
    })
}

/// A union's refusal named a piece of a member edge by the fold's rank.
const UNION_REFUSAL_FOLD_RANKED_EDGE: &str = "a union fold's refusal names a piece of a member \
     edge by the fold's rank, which no published table holds";

/// A union's refusal named a row its own fold table cannot collapse.
const UNION_REFUSAL_FOREIGN: &str =
    "a union fold's refusal names a row the member-keying rule cannot collapse";

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
///
/// The menu is the ONE translation that needs the operands' naming
/// context, so it happens here, before the shared translation: every
/// other refusal a boolean run can carry falls through to
/// [`verb_refused`], the same door every verb's refusal goes through.
/// The two operands are given as their name TABLES rather than as node
/// ids: the n-ary union folds the same verb over an ACCUMULATION that
/// is no node's result, and the menu reads nothing else about an
/// operand.
fn refusal_menu<T: crate::lane::Lane>(
    a: (RecipeNodeId, &crate::names::NameTable),
    b: (RecipeNodeId, &crate::names::NameTable),
    err: verbs::VerbError<T>,
) -> NodeErrorKind {
    let verbs::VerbError::Boolean(topo::BooleanError::UndeclaredCoincidence {
        diag,
        pair,
        relation,
    }) = err
    else {
        return verb_refused(err);
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
    // A finding is SITED: the name says which entity, and the
    // operand it was raised on says where that name is read. The pair
    // a caller declares back is therefore buildable from the refusal
    // alone — including a SAME-operand pair, whose two names are both
    // that one operand's and which no downstream door could have
    // sided from the names.
    let name_of = |(operand, face): (topo::Operand, topo::FaceKey)| {
        let (at, table) = match operand {
            topo::Operand::A => a,
            topo::Operand::B => b,
        };
        face_name(table, face).map(|name| crate::node::SitedRef::new(at, name))
    };
    let (Some(na), Some(nb)) = (name_of(ordered[0]), name_of(ordered[1])) else {
        return NodeErrorKind::Boolean(topo::BooleanError::UndeclaredCoincidence {
            diag,
            pair,
            relation,
        });
    };
    NodeErrorKind::UndeclaredContact {
        // A pair boolean's operands are NODES, so both rows are their
        // own and neither is a merge this door minted. The union's
        // door ([`union_refusal`]) is the one that fills this.
        merged: Box::new((Vec::new(), Vec::new())),
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
fn face_name(
    table: &crate::names::NameTable,
    face: topo::FaceKey,
) -> Option<crate::names::StableName> {
    use crate::names::{EntityKey, EntityKind, EntityRef, Entry};
    let mut found: Option<&crate::names::StableName> = None;
    for (name, entry) in table.iter() {
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
/// tables into the kernel's [`BooleanDeclarations`] (F5).
///
/// **One definition, two doors.** [`wire_boolean`] calls it with the
/// two operands' tables; [`wire_union`] calls it once per fold step
/// with that step's two — the accumulation in this node's published
/// space and the joining member's `member_view` — for the bucket
/// [`route_declarations`] sent there. The side-picking below is the
/// same for both, which is what makes "resolve a declared name against
/// two tables" one answer rather than two.
///
/// The v1 pair vocabulary is [`DeclaredStep`] and is not re-listed
/// here; what this door adds to it is that the resolver is
/// carrier-agnostic and always was — it pushes a
/// `FacePairDeclaration` whatever the two faces' surface kinds are,
/// and the kernel's ladder is what verifies it. Everything outside
/// that vocabulary refuses typed. Resolution scope is deliberately
/// the OPERANDS' tables (spec D4: "resolve through the operands' name
/// tables") — a name minted elsewhere in the document is Vanished
/// HERE even if some other node still carries it.
///
/// **Twinned with [`resolve_selection`]** (M6-5): the fillet's
/// selection resolves through the same [`ladder`], which owns rung
/// order and payload shapes. What stays here is its pair vocabulary,
/// and the fact that each name is read in exactly ONE table — the one
/// its SITE picked, before this door ran. There is no side to guess
/// and no name that lands in both operands: two placements of one
/// prototype carry identical tables and are still told apart, because
/// the pair says which member it means.
fn resolve_declarations<'n>(
    pairs: &'n [SidedPair<'n>],
    doc: &crate::doc::Doc<ProfileProgram>,
    a_table: &NameTable,
    b_table: &NameTable,
) -> Result<BooleanDeclarations, NodeErrorKind> {
    let mut out = BooleanDeclarations::none();
    for ((o1, n1), (o2, n2), class) in pairs {
        let (o1, o2, class) = (*o1, *o2, *class);
        let refused = |error| NodeErrorKind::DeclareResolve { error };
        // BOTH names walk their own rungs before EITHER tie is
        // raised, which is the cross-name half of the order below and
        // is not something the ladder decides: the ladder ranks
        // within one name's walk and says nothing about one name's
        // rungs against the other's. This door's rule is that the
        // TIE is the one per-name refusal the PAIR question outranks,
        // and a pair question cannot be asked before both names have
        // landed. So every per-name fault that is not the tie —
        // `NodeGone` and `Vanished` — is raised for
        // whichever name carries it, and a tie on the first name
        // waits behind them: an author with a second name that does
        // not resolve at all has a repair to make either way, and
        // narrowing the first would not reach it.
        let table_of = |op| match op {
            topo::Operand::A => a_table,
            topo::Operand::B => b_table,
        };
        let (live1, l1) = declare_landing(n1, doc, table_of(o1))?;
        let (live2, l2) = declare_landing(n2, doc, table_of(o2))?;
        let (n1, n2) = (n1.name(), n2.name());
        // KIND BEFORE MULTIPLICITY, the order [`resolve_face`] asks
        // in: a pair the vocabulary has no step for is unsupported
        // however many entities answer to either name, so WHAT the
        // two names denote precedes how many do. Asked of the NAMES'
        // kinds, which the table makes every candidate's kind
        // (`NameTable::insert_ref`, `insert_tied_ref` admit a row
        // only at its name's kind, and they are the only two writers
        // of a row), so a tie answers this as readily as a unique row
        // does — and an unsupported pair reads the same whether or
        // not one of its names happens to be tied.
        let unsupported = |kinds| NodeErrorKind::DeclareUnsupportedPair {
            kinds,
            cross_operand: o1 != o2,
        };
        let Some(step) = declared_step((o1, n1.kind), (o2, n2.kind)) else {
            return Err(unsupported((n1.kind, n2.kind)));
        };
        let k1 = ladder::resolve(live1, l1).map_err(refused)?.key;
        let k2 = ladder::resolve(live2, l2).map_err(refused)?.key;
        // The arms below PROJECT the keys of the step named above and
        // add no shape of their own. They read the ORIENTATION off
        // the step too ([`sides`]) rather than re-deriving it from
        // `o1` and `n1.kind`: a projection that re-asks a question
        // the classifier already answered agrees with it only by
        // coincidence, and this door is the one that exists because
        // two sites agreed by coincidence.
        //
        // A projection that still fails means a table holds a key of
        // another kind than its name's — asserted in debug, naming
        // WHICH projection, and in release answered off the KEYS, the
        // one place the two can disagree.
        let broke = |shape: &'static str| {
            debug_assert!(
                false,
                "a declared {shape} pair projected a key of another kind than its name's: \
                 `NameTable::insert_ref` and `insert_tied_ref` admit a row only at its \
                 name's kind"
            );
            unsupported((k1.kind(), k2.kind()))
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
                let (Some(va), Some(vb)) = (k1.vertex(), k2.vertex()) else {
                    return Err(broke("same-operand vertex-vertex"));
                };
                // The AUTHORED class, carried — not re-defaulted. The
                // whole point of the payload change is that this door
                // no longer has to guess.
                carried(&mut out, side.operand()).vv.push(CarriedVv {
                    pair: VvContact { a: va, b: vb },
                    class,
                });
            }
            DeclaredStep::SameVf(side, roles) => {
                let (v, f) = roles.vertex_then_face(k1, k2);
                let (Some(vertex), Some(face)) = (v.vertex(), f.face()) else {
                    return Err(broke("same-operand vertex-face"));
                };
                carried(&mut out, side.operand()).vf.push(CarriedVf {
                    rest: VfContact { vertex, face },
                    class,
                });
            }
        }
    }
    Ok(out)
}

/// The carried-contact sink a SAME-operand declaration lands in — one
/// place where the operand decides which side's list a 3′ contact
/// joins, rather than the same two-arm match at each contact shape.
fn carried(out: &mut BooleanDeclarations, op: topo::Operand) -> &mut CarriedContacts {
    match op {
        topo::Operand::A => &mut out.carried_a,
        topo::Operand::B => &mut out.carried_b,
    }
}

/// **The orientation facts a declared pair's step rests on, as tokens
/// only a COMPARISON of the two sides can mint** — the device
/// [`ladder::Live`] uses one door over, for the reason this door
/// exists at all.
///
/// [`DeclaredStep`] carries these rather than a bare `Operand` or a
/// `bool`, so [`resolve_declarations`]'s projection reads the
/// orientation [`declared_step`] decided instead of re-deriving it
/// from `o1` and `n1.kind`. A projection that re-asks a question the
/// classifier already answered agrees with it only by coincidence,
/// and two sites agreeing by coincidence is the whole subject of this
/// door.
///
/// The fields are private to this module and the `of` constructors
/// are the only way in, so an arm of [`declared_step`] cannot
/// fabricate an orientation its own pattern does not support.
/// Reaching past a constructor is `E0603`; naming a variant without
/// its witness is `E0308`. What remains spellable is calling a
/// comparison with ONE side twice (`SameOperand::of(oa, oa)`), which
/// compiles — the residue, named here because the previous round of
/// this door shipped an unchecked "fails to compile" and this doc is
/// not going to ship a second one.
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

        /// The pair in OPERAND order, A's first — whatever the two
        /// carry, since the fact is about the sides and not about
        /// what is being ordered.
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
/// the ONE enumeration of that vocabulary in this crate. Every other
/// mention points here: [`resolve_declarations`] projects the keys of
/// whichever variant comes back and adds no shape of its own, and
/// [`NodeErrorKind::DeclareUnsupportedPair`]'s doc names this
/// function instead of re-listing the pairs.
///
/// Each variant carries the ORIENTATION its step needs, as a
/// [`sides`] token: which operand is A's for a cross pair, which
/// operand both names landed in for a same-operand pair, which
/// authored name is the vertex. **What that buys, at the resolution
/// it is true at** — a fourth VARIANT fails to compile until the
/// projection covers it (`E0004`; the `match` is exhaustive with no
/// wildcard), and a fourth PAIR SHAPE reusing a variant fails to
/// compile in the two spellings that assert a side (`E0308` without
/// the witness, `E0603` reaching past its constructor) while the
/// spelling that asks for one honestly returns `None` and refuses.
/// What is NOT caught: an arm that calls a comparison with one side
/// twice, and an arm that pairs the wrong KINDS with a variant — the
/// second projects nothing and reaches `broke`, which is fail-loud
/// and not a bijection.
///
/// The claim is written at that resolution on purpose. The round
/// before this one said "a fourth shape fails to compile" over PAIR
/// SHAPES when it was only true over VARIANTS, one paragraph below a
/// note telling future lanes that "once" is a claim to check. A
/// property worth a sentence is worth the experiment that the
/// sentence reports.
///
/// Asked of the two names' KINDS and the operands they landed in,
/// which is everything the question depends on — none of it needs a
/// name resolved to one entity, which is why the question can precede
/// the tie.
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
/// shape and the SIDES have to witness it, so a pair whose kinds name
/// a step its operands cannot support falls out as `None` rather than
/// needing a guard to remember. `None` is
/// [`NodeErrorKind::DeclareUnsupportedPair`]'s case.
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
/// so [`resolve_declarations`] can ask the PAIR's kind question in
/// between.
///
/// Rung 1 first, and not by convention: reading the table needs the
/// token [`ladder::live`] returns, so a dead minting node refuses
/// `NodeGone` before anything is said about where the name landed.
/// [`route_declarations`] has already paid this for a union's names,
/// one bucket earlier; it stays here because this door is also the
/// pair boolean's, where nothing routed first.
///
/// **There is no side to pick.** The site is the side (DM4): a
/// declared entity names the operand it is read at, so a name carried
/// by BOTH operands — two placements of one prototype, whose tables
/// are identical because a transform contributes no segment (N1) — is
/// resolved in the one the author named, and the pair boolean declares
/// between them like any other.
///
/// Rung 3 is here rather than with rung 2 because a name that names
/// nothing in the table its site picked says THAT: the pair's
/// vocabulary is not an answer about a name that is not there. Only
/// rung 2 — the tie — is left for the caller, which is the one
/// refusal the kind question outranks.
///
/// # Errors
///
/// Rung 1's `NodeGone` and rung 3's `Vanished`, both through
/// [`NodeErrorKind::DeclareResolve`].
fn declare_landing<'n>(
    sided: &'n SidedName<'n>,
    doc: &crate::doc::Doc<ProfileProgram>,
    table: &NameTable,
) -> Result<(ladder::Live<'n>, ladder::Landing), NodeErrorKind> {
    use ladder::Landing;
    let refused = |error| NodeErrorKind::DeclareResolve { error };
    // Rung 1, paid ONCE per name: the pair boolean's door paid it to
    // answer the site question and hands the token on; a union's
    // minted name has none, and pays here.
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

/// The role word a transform's rotation axis is normalized under —
/// one spelling, so the evaluation and the mate solve name the same
/// vector in the same refusal and the K census sees one predicate
/// role rather than two.
pub(crate) const TRANSFORM_AXIS_ROLE: &str = "transform rotation axis";

/// The role word a stepped rule's LINEAR direction is normalized
/// under, for the same reason and with the same two callers: the
/// evaluation's own rule ([`stepped_map`]) and the mate solve's
/// re-derivation of it from the recipe.
pub(crate) const PATTERN_DIRECTION_ROLE: &str = "pattern direction";

/// The role word a frame's authored +x direction is normalized under
/// ([`frame_axes`]).
pub(crate) const FRAME_X_ROLE: &str = "datum frame x axis";

/// The role word a frame's authored +y direction is normalized under,
/// after Gram-Schmidt ([`frame_axes`]).
pub(crate) const FRAME_Y_ROLE: &str = "datum frame y axis";

/// The role word a plane datum's normal is normalized under.
pub(crate) const PLANE_NORMAL_ROLE: &str = "datum plane normal";

/// The role word a tube's REFERENCE DIRECTION is normalized under —
/// the authored `u_ref` that fixes where the window's angles start.
/// It reaches the frame mint from a slot, not from a datum, so this
/// arm is where its refusal is spelled.
///
/// The role names the RESIDUAL and not the vector, because that is
/// the length the mint decides: `u_ref` yields its component along
/// the spine axis and what remains becomes the frame's `u`. A
/// reference five metres long that lies on the axis line refuses
/// here, and "the tube reference direction has zero length" would be
/// false of it.
pub(crate) const TUBE_REFERENCE_ROLE: &str =
    "tube reference direction's component perpendicular to the spine axis";

/// The role word a DATUM AXIS's direction is normalized under. Three
/// callers, and they do not all take the same road — the evaluation
/// decides it under [`DATUM_UNIT_NORM`], through the kernel type that
/// holds the datum, and the mate solve's re-derivation from the
/// recipe under [`EVAL_DIRECTION_NORM`], which is the ratified
/// two-name split. So the constant is what keeps the ROLE one word
/// wherever the refusal comes from, and it is the half of the
/// refusal a user actually reads.
pub(crate) const DATUM_AXIS_ROLE: &str = "datum axis direction";

/// **The rigid map a [`crate::node::Node::Transform`] applies** — the
/// one home of that construction, read by the evaluation and by the
/// mate solve's derived offset, so a transform under a mate and a
/// transform under the gather move a body by the same arithmetic.
///
/// The die convention: rotate about the axis THROUGH THE WORLD
/// ORIGIN by `angle`, then translate. `axis` is unit as a property of
/// its type — the callers mint it through [`unit()`] under
/// [`TRANSFORM_AXIS_ROLE`], where the degenerate and non-finite cases
/// refuse. [`Mat3::rotation_about`] takes the bare vector and divides
/// it by its own norm once more; on a unit input that divide changes
/// no bit the format holds exactly.
pub(crate) fn transform_map<T: Decide>(
    translation: Vec3<T>,
    axis: UnitVec3<T>,
    angle: T,
) -> Affine3<T> {
    Affine3::from_parts(Mat3::rotation_about(axis.get(), angle), translation)
}

/// **The transform node**: ONE rigid map, shape-preserving over its
/// input's value — `Body → Body`, `Instances → Instances` — so a
/// transform of N bodies is N transforms in the input's own order,
/// and body `i` of a transform of instances is bit for bit what the
/// same map does to that body selected alone through `Node::Part`.
///
/// Identity-preserving pass-through (spec D2): the transform
/// contributes NO `RolePath` segment. `transform_rigid` is key-stable
/// (arenas rewritten in place of a clone), so the input's table holds
/// verbatim — same names, same keys, same output-body indices, the N1
/// derivation-path semantics (a name still points at the MINTING node;
/// the placement is recipe context, not identity) — and over
/// `Instances` this holds body by body because a row's output-body
/// index is the instance index and the i-th output body is the i-th
/// input body placed. Stamps: [`compose_placed`].
fn wire_transform<T: Decide + geom_brep::PcurveFittedLane + topo::AtRestPolicy>(
    id: RecipeNodeId,
    input: RecipeNodeId,
    results: &Results<T>,
    vals: &SlotValues<T>,
    tol: Tol,
) -> OpResult<T> {
    let value = value_of(results, input)?;
    let placeable = placeable_operand(value, input)?;
    let translation = need_vec3(vals, SlotId::Translation)?;
    let rot_axis = unit(
        need_vec3(vals, SlotId::RotationAxis)?,
        TRANSFORM_AXIS_ROLE,
        band(tol)?,
    )?;
    let angle = need_scalar(vals, SlotId::RotationAngle)?;
    let map = transform_map(translation, rot_axis, angle);
    let payload = placeable.map(|body, i| {
        let ordinal = names::output_body(i).map_err(NodeErrorKind::Naming)?;
        place(body, Some(&map), id, ordinal, tol)
    })?;
    Ok(OpOut::plain(payload, Arc::clone(&value.name_table)))
}

/// The resolved operands of a stepped placement rule: what the rule's
/// math consumes once every slot or expression is evaluated and every
/// direction is unit as a property of its type. The two rules get there
/// by different roads: a LINEAR rule's direction is a slot this layer
/// mints through [`unit()`], while a CIRCULAR rule's axis arrives out
/// of a datum's `UnitVec3` — the kernel type's constructor did it, and
/// no door here re-decides it.
pub(crate) enum SteppedOperands<T: geom_core::Real> {
    /// A linear rule: unit direction, spacing per step.
    Linear {
        /// The stepping direction.
        direction: UnitVec3<T>,
        /// The per-step translation distance along it.
        spacing: T,
    },
    /// A circular rule: the datum axis and the angle per step.
    Circular {
        /// A point on the rotation axis.
        origin: Point3<T>,
        /// The axis direction, the datum's own witness.
        dir: UnitVec3<T>,
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
            Affine3::translation(direction.get() * (*spacing * step))
        }
        SteppedOperands::Circular {
            origin,
            dir,
            step: angle,
        } => Affine3::rotation_about_axis(*origin, dir.get(), *angle * step),
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
                PATTERN_DIRECTION_ROLE,
                band(tol)?,
            )?,
            spacing: need_scalar(vals, SlotId::Spacing)?,
        },
        PatternKind::Circular { axis, .. } => {
            let (origin, dir) = operand(results, *axis, super::phrase::DATUM_AXIS, |v| {
                match &v.payload {
                    ValuePayload::Datum(DatumValue::Axis { origin, dir }) => Some((origin, dir)),
                    _ => None,
                }
            })?;
            SteppedOperands::Circular {
                origin: *origin,
                dir: *dir,
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

/// **The pattern node**: a stepped rule over its input's value,
/// shape-preserving — a body or an `Instances` value is the MASTER,
/// placed whole — yielding `Instances`.
///
/// **The layout, placement-major** (D9's order, the one the rule is
/// walked in): output body `j·M + i` is placement `j` of the master's
/// body `i`, `M` the master's body count — the instance index itself
/// for a one-body master, and for a nested pattern the flat list
/// `Node::Part` selects from and the product gathers. Placement 0 is
/// the master's own bodies verbatim (the rule at 0 IS the identity);
/// every other placement goes through [`place`]. The one home of the
/// arithmetic is [`names::flat_body_index`], which the name table is
/// keyed by too: every master name wraps `Instance(j)` per placement
/// (A8/N1), and key-stability means instance keys equal master keys.
fn wire_pattern<T: Decide + geom_brep::PcurveFittedLane + topo::AtRestPolicy>(
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
    let per = names::output_body(master.len()).map_err(naming)?;
    let mut instances: Vec<Arc<Body<T>>> = master.to_vec();
    for j in 1..n {
        let map = stepped_map(kind, j, results, vals, tol)?;
        let placement =
            names::output_body(usize::try_from(j).unwrap_or(usize::MAX)).map_err(naming)?;
        for (i, body) in master.iter().enumerate() {
            let ordinal = names::output_body(i)
                .and_then(|i| names::flat_body_index(placement, per, i))
                .map_err(naming)?;
            instances.push(Arc::new(place(body, Some(&map), id, ordinal, tol)?));
        }
    }
    let table =
        names::name_pattern(id, &value.name_table, n, master.len(), &instances).map_err(naming)?;
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
///    this lowers through asserts nothing about its operands,
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
fn wire_placed_union<
    T: Decide + geom_core::Bounds + geom_brep::PcurveFittedLane + topo::AtRestPolicy,
>(
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
    // "failure" or a kernel rigidity refusal.
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
        // Stamped per structural instance — the pattern node's rule
        // verbatim: distinct instances are distinct sources.
        let ordinal = names::output_body(i).map_err(NodeErrorKind::Naming)?;
        let placed = place(&body, Some(map), id, ordinal, tol)?;
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
// The definitional §10.3/§10.4 nodes
// ---------------------------------------------------------------------

/// The Sweep node's frontier — the ONE
/// [`NodeErrorKind::CurvedSolidFrontier`] door. Kept as a constant so
/// the acceptance rows assert the SAME text the node produces.
///
/// §10.4's rigid-profile sweep needs the path as ONE curve. The recipe
/// layer cannot supply one: a `Node::Sweep`'s `path` operand is a
/// profile, a validated profile's loop is a CLOSED chain, and a closed
/// chain has two or more segments — even the minimal two-vertex loop is
/// two half-turn arcs. So there is no recipe-expressible path, and the
/// honest node-layer answer is a single refusal naming what is
/// missing: a joined-path composition lane.
///
/// `sweep::sweep_geometry` AND `sweep::sweep_body` are live and
/// exercised through the library API on a real curved-path caller —
/// `sweep/tests/m7_skin_integral.rs` builds, validates and measures a
/// quarter-torus elbow, and `step-export/tests/m7_swept_elbow.rs` puts
/// it on the wire — so it is only this NODE lane that is gated, at one
/// door, and the message cannot imply an expressible case that does
/// not exist.
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
fn section_of<T: Decide + geom_core::Bounds + super::SectionScalar>(
    doc: &crate::doc::Doc<ProfileProgram>,
    results: &Results<T>,
    id: RecipeNodeId,
    lane: LaneEnv<'_, T>,
    tol: Tol,
) -> Result<(sweep::Section, Affine3<f64>, super::ProfilePieces), NodeErrorKind> {
    let program = node_operand(doc, id, super::family::PROFILE, |n| match n {
        Node::Profile(program) => Some(program),
        _ => None,
    })?;
    // THE SEED STOPS HERE, TYPED. The section stays `f64` in every lane
    // (the C6/D9 argument below), so a seed on a parameter this program
    // reads has no channel to ride and would arrive at the skinned
    // surface as a constant — a finite, wrong zero. That is the state
    // the lift exists to end at the profile node, and cannot end here;
    // the answer is a refusal naming the section and the parameter,
    // never the zero.
    if let Some(param) = lane.seed
        && program.references(param)
    {
        return Err(NodeErrorKind::SeedPinnedSection {
            section: id,
            param: param.clone(),
        });
    }
    // LIB-SWITCH §4b at the loft/sweep seam: the section is the
    // node's program RESOLVED at `LaneEnv::nominal` and REPLAYED —
    // the same C6/D9 pipeline the profile node runs. The profile's own
    // validation door still runs first, so a bad section reads as a
    // profile error at the NODE (the §2 compatibility contract) before
    // the library door re-gates it.
    let resolved = program
        .resolve(lane.nominal)
        .map_err(|(slot, source)| NodeErrorKind::Expr { slot, source })?;
    // The f64 ladder is `prepare_profile` ITSELF, not a copy of it:
    // one pipeline shared by this seam and the profile node, so a gate
    // added to it gates both.
    // DM1c: a section on a DERIVED frame has no `f64` placement of its
    // own — the frame's landed value is the lane's — and a section's
    // geometry stays `f64`. So the placement comes off the by-value
    // read, and crosses to `f64` only where the scalar IS `f64`
    // (`SectionScalar`, decided by the type); anywhere else the
    // section refuses typed, naming itself and the frame, rather than
    // placing on a fabricated point of the frame's bracket.
    let plane = match profile_plane_f64(results, id, program.plane)? {
        Some(authored) => authored,
        None => {
            let lane_plane = frame_plane_lane(results, program.plane)?;
            pinned_plane(&lane_plane).ok_or(NodeErrorKind::DerivedFrameSection {
                profile: id,
                frame: program.plane,
            })?
        }
    };
    let pre = prepare_profile(Some(plane), &resolved, &program.ids, tol)?;
    // The lift's second pass runs HERE TOO, as a GATE. A loft's or a
    // sweep's section stays f64 — the skinned surface's knots, degrees
    // and control bits must be identical in every lane, which is the
    // C6/D9 argument above and is untouched — but the certify-or-abort
    // answer must not depend on which node consumes the profile. A
    // parameter box the extrude ladder refuses to certify is refused
    // here as well, and by the same predicate.
    if lane.lift == super::ProfileLift::Guided {
        lane_profile::<T>(
            program,
            frame_plane_lane(results, program.plane)?,
            lane,
            &pre,
            tol,
        )?;
    }
    // Both arms above handed `prepare_profile` a placement, so the
    // typed one is `Some` here by construction; a `None` would be an
    // internal break, refused typed rather than placed at a
    // convention.
    let place = pre
        .placement_f64
        .ok_or(NodeErrorKind::DerivedFrameSection {
            profile: id,
            frame: program.plane,
        })?
        .placement;
    // The sections are the REPLAYED loops (program order — exactly
    // the stored-loop handoff LIB-U3 established, one derivation
    // earlier): positions, bulges, declared joints verbatim. The
    // pieces are the canonical positions' names, which the skin's
    // canonical walls and seams are named by.
    Ok((pre.profile_f64.loops, place, pre.pieces))
}

/// A structural (Count) slot, refused typed when absent or unusable.
fn need_count(vals: &SlotValues<impl Decide>, slot: SlotId) -> Result<usize, NodeErrorKind> {
    let n = slots::count(vals, slot).ok_or(NodeErrorKind::MissingSlot { slot })?;
    usize::try_from(n).map_err(|_| NodeErrorKind::NonPositiveCount { count: n })
}

/// The Loft node (M6-3: the frontier flipped to the BUILDER — the
/// §10.3 walls plus the M5-LOG item-6 assembly, tiers 1–3 green at
/// rest).
fn wire_loft<T: Decide + geom_brep::PcurveFittedLane + geom_core::Bounds + super::SectionScalar>(
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
    // The geometry/profile doors keep their historical node-error
    // shapes (the §2 compatibility contract predates the builder);
    // assembly-proper refusals arrive as the M6-3 `Loft` kind.
    let mut built =
        sweep::loft_body::<T>(&sections, &places, v_degree, tol).map_err(|e| match e {
            sweep::LoftError::Skin(s) => NodeErrorKind::Skin(s),
            other => NodeErrorKind::Loft(other),
        })?;
    // Eager N4 emission from the builder's own maps, BEFORE the
    // structural handoff is dropped (the extrude idiom). The maps are
    // indexed by canonical (loop, segment), and the skin paired
    // canonical segment `k` of every section into wall `k`, so wall
    // `k` is named by the piece each section's canonical segment `k`
    // is — one locator per section.
    let table = names::name_loft(id, &built, &pieces).map_err(NodeErrorKind::Naming)?;
    stamp_minted(&mut built.body, id);
    Ok(OpOut::plain(
        ValuePayload::Body(Arc::new(built.body)),
        table,
    ))
}

/// The Sweep node: ONE honest arm.
///
/// Every RECIPE door still runs first — the structural slots, and both
/// operands through [`section_of`] — because a Sweep on a datum, or
/// with a bad Count, is a recipe error and must read as one. What the
/// node cannot do is reach the geometry: see [`SWEEP_FRONTIER`] for
/// why no recipe-expressible path exists to sweep.
fn wire_sweep<T: Decide + geom_core::Bounds + super::SectionScalar>(
    profile: RecipeNodeId,
    path: RecipeNodeId,
    doc: &crate::doc::Doc<ProfileProgram>,
    results: &Results<T>,
    vals: &SlotValues<T>,
    lane: LaneEnv<'_, T>,
    tol: Tol,
) -> OpResult<T> {
    let _stations = need_count(vals, SlotId::Stations)?;
    let _v_degree = need_count(vals, SlotId::VDegree)?;
    let _ = section_of::<T>(doc, results, profile, lane, tol)?;
    let _ = section_of::<T>(doc, results, path, lane, tol)?;
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
        NodeErrorKind, RecipeNodeId, SidedName, SidedPair, SitedRef, look_through_merges,
        resolve_declarations, route_declarations,
    };
    use crate::names::{CapEnd, EntityKey, EntityKind, EntityRef, NameTable, RoleSeg, StableName};
    use crate::node::Node;
    use crate::{DocEdit, ProfileDoc};
    use geom_core::Tol;
    use topo::{ContactClass, Operand};

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
                        node: Node::declare_rest(Vec::new()),
                    },
                    Tol::witness(),
                    &crate::mate::RefusingReach,
                )
                .expect("an empty Declare inserts");
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

    fn pair(a: SitedRef, b: SitedRef) -> ((SitedRef, SitedRef), ContactClass) {
        ((a, b), ContactClass::Rest)
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
        let ((o1, n1), (o2, n2), _) = &buckets[2][0];
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
        let ((o1, n1), (o2, n2), _) = sided(pair(at(ms[1], CapEnd::Start), at(ms[3], CapEnd::End)));
        assert_eq!(o1, Operand::A);
        assert_eq!(o2, Operand::B);
        assert_eq!(*n1.name(), member_face(union, ms[1], CapEnd::Start));
        assert_eq!(*n2.name(), member_face(union, ms[3], CapEnd::End));
        // The same pair written the other way round: the SITES decide,
        // not the order the author wrote them in.
        let ((o1, _), (o2, _), _) = sided(pair(at(ms[3], CapEnd::End), at(ms[1], CapEnd::Start)));
        assert_eq!((o1, o2), (Operand::B, Operand::A));
        // One member with itself is that member's CARRIED contact, on
        // the side it enters the step as: operand B at its own step,
        // and operand A for member 0, which is where the fold starts.
        let ((o1, _), (o2, _), _) = sided(pair(at(ms[2], CapEnd::Start), at(ms[2], CapEnd::End)));
        assert_eq!((o1, o2), (Operand::B, Operand::B));
        let ((o1, _), (o2, _), _) = sided(pair(at(ms[0], CapEnd::Start), at(ms[0], CapEnd::End)));
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
                Err(NodeErrorKind::DeclareResolve { ref error })
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
            .expect("the empty Declare deletes")
            .doc;
        let p = pair(at(ms[0], CapEnd::Start), at(gone, CapEnd::End));
        let refused = route_declarations(union, &ms[..3], std::slice::from_ref(&p), &doc);
        assert!(
            matches!(
                refused,
                Err(NodeErrorKind::DeclareResolve { ref error })
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
            sweep::Extrusion::Distance(1.0_f64),
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
            (a.0, SidedName::Rewritten(a.1)),
            (b.0, SidedName::Rewritten(b.1)),
            ContactClass::Rest,
        )
    }

    /// A merged row over member faces, as a fold step mints it: flat,
    /// sorted, in the union's own space.
    fn merged(set: Vec<StableName>) -> StableName {
        let mut set = set;
        set.sort();
        StableName {
            kind: EntityKind::Face,
            node: RecipeNodeId(0),
            path: vec![RoleSeg::Merged(set)],
        }
    }

    /// The rewrite touches ONE shape: an ACCUMULATION-side member face
    /// that is no row and sits in a merged row's set goes to that row.
    /// A face the accumulation still holds, and one in no set at all,
    /// are handed on as written — and the JOINING member's side never
    /// looks through, because no merge the fold has performed could
    /// have consumed a face of a member that has not joined yet.
    #[test]
    fn look_through_rewrites_only_an_accumulated_member_face_inside_a_merged_row() {
        let (_doc, union, ms) = doc_with_members(4);
        let key = a_face_key();
        let f = |m, e| member_face(union, m, e);
        // Step 2's row: the merge of step 1's `{m0, m1}` with `m2`,
        // flat, minted in the union's space.
        let wide = StableName {
            node: union,
            ..merged(vec![
                f(ms[0], CapEnd::Start),
                f(ms[1], CapEnd::Start),
                f(ms[2], CapEnd::Start),
            ])
        };
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
            // In no table and in no set: untouched, and the door below
            // refuses it as the vanished name it is.
            routed(
                (Operand::A, f(ms[2], CapEnd::End)),
                (Operand::B, f(ms[3], CapEnd::Start)),
            ),
        ];
        let out = look_through_merges(&bucket, &acc).unwrap();
        assert_eq!(out[0].0, (Operand::A, SidedName::Rewritten(wide)));
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
        let row = StableName {
            node: union,
            ..merged(vec![f(ms[0], CapEnd::Start), f(ms[3], CapEnd::Start)])
        };
        let mut acc = NameTable::new();
        acc.insert(row, face_ref(a_face_key())).unwrap();
        let p = routed(
            (Operand::A, f(ms[1], CapEnd::End)),
            (Operand::B, f(ms[3], CapEnd::Start)),
        );
        let out = look_through_merges(std::slice::from_ref(&p), &acc).unwrap();
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
            StableName {
                node: union,
                ..merged(vec![f(ms[0], CapEnd::Start), f(ms[1], CapEnd::Start)])
            },
            face_ref(key),
        )
        .unwrap();
        // A second entity for the second row: the table refuses two
        // names on one entity, and the shape under test is two rows.
        acc.insert(
            StableName {
                node: union,
                ..merged(vec![f(ms[0], CapEnd::Start), f(ms[2], CapEnd::Start)])
            },
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
        let refused = look_through_merges(std::slice::from_ref(&p), &acc);
        assert!(
            matches!(
                refused,
                Err(NodeErrorKind::Naming(crate::names::NamingError::Emission { what }))
                    if what == super::MEMBER_FACE_IN_TWO_MERGES
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
        let refused = resolve_declarations(std::slice::from_ref(&p), &doc, &acc, &member);
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

    /// **`sited_member` is `member_name`'s inverse** on a member's own
    /// row, round-tripped over rows of every entity kind and over a
    /// member whose names are minted elsewhere (a transform).
    #[test]
    fn sited_member_inverts_member_name_over_every_row_kind() {
        let (_doc, union, ms) = doc_with_members(3);
        let (member, proto) = (ms[1], ms[2]);
        let rows = vec![
            StableName {
                kind: EntityKind::Face,
                node: proto,
                path: vec![RoleSeg::Cap(CapEnd::Start)],
            },
            StableName {
                kind: EntityKind::Edge,
                node: member,
                path: vec![RoleSeg::Cap(CapEnd::End), RoleSeg::Cap(CapEnd::Start)],
            },
            StableName {
                kind: EntityKind::Vertex,
                node: proto,
                path: vec![RoleSeg::OutputBody],
            },
            StableName {
                kind: EntityKind::Body,
                node: member,
                path: vec![RoleSeg::OutputBody],
            },
        ];
        for row in rows {
            let keyed = crate::names::member_name(union, member, &row);
            let back = super::sited_member(union, &ms, &keyed).expect("the row collapses");
            assert_eq!(
                back,
                super::DeclarationSubject::Member(SitedRef::new(member, row.clone())),
                "{row}"
            );
        }
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
