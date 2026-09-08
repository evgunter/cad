//! Node-to-kernel wiring (spec D3: wire, don't invent): each F4 node
//! maps to an EXISTING public kernel op; every editor-side geometric
//! judgment (direction normalization, the revolve axis's in-plane
//! projection, full-vs-partial classification) goes through the
//! kernel's decided-predicate door, never a raw comparison.

use std::collections::BTreeMap;
use std::sync::Arc;

use geom_core::{Affine3, Band, Decide, Mat3, Point2, Point3, Sign, Tol, Vec2, Vec3};
use sweep::blend::BlendKind;
use sweep::{Revolution, RevolveAxis};
use topo::query;
use topo::splitting::SplitPart;
use topo::transform::transform_rigid;
use topo::{
    Body, BooleanDeclarations, CarriedContacts, CarriedVf, CarriedVv, ContactClass,
    DATUM_UNIT_NORM, FacePairDeclaration, GeomSource, UnitVec3, UnitVec3Error, VfContact,
    VvContact,
};

use super::anchor::{self, ProfileNaming, ProfilePre, ProfileValue};
use super::slots::{self, SlotValues};
use super::{BooleanValue, DatumValue, NodeErrorKind, NodeResult, SplitSide, ValuePayload};
use crate::names::{self, NameTable, SplitHalf};
use crate::node::{Axis3, BooleanOp, Datum, Node, PartSelect, PatternKind, RecipeNodeId, SlotId};
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
            contacts: Arc::new(topo::ContactRecords::default()),
            carried: Arc::new(crate::assembly::CarriedDeclarations::default()),
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
        + crate::verbs::shell::ShellLane,
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
        + crate::verbs::shell::ShellLane,
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
fn place<T: Decide + geom_brep::PcurveFittedLane>(
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
    match placeable_operand(value_of(results, input)?, input) {
        Ok(Placeable::Body(b)) => Ok(b),
        Ok(Placeable::Instances(_)) => Err(NodeErrorKind::WrongOperand {
            input,
            expected: "body",
            found: "instances",
        }),
        Err(NodeErrorKind::WrongOperand { input, found, .. }) => Err(NodeErrorKind::WrongOperand {
            input,
            expected: "body",
            found,
        }),
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
        other => Err(NodeErrorKind::WrongOperand {
            input,
            expected: "body or instances",
            found: other.kind_name(),
        }),
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
/// [`topo::query::decide_unit_direction`] rather than as a literal at the
/// `decide` call, so it is a roster carrier (`docs/K-REPORT.md`, "The
/// inventory method, restated"), and it is a constant so that the
/// name the telemetry records and the name an escalation reports
/// cannot drift apart.
pub(crate) const EVAL_DIRECTION_NORM: &str = "eval_direction_norm";

/// Normalizes a direction-valued vector; a non-finite length refuses,
/// decided-zero length refuses, in-band indeterminacy escalates.
///
/// **The decision is the kernel's one body**
/// ([`topo::query::decide_unit_direction`]): finiteness asked first through
/// the value channel every scalar has, then which side of zero the
/// length lies on, then normalize or refuse. This function is that
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
/// ([`topo::UnitVec3::new`]), because `DatumValue` has no
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
) -> Result<Vec3<T>, NodeErrorKind> {
    query::decide_unit_direction(v, EVAL_DIRECTION_NORM, band)
        .map_err(|e| refusal(e, role, EVAL_DIRECTION_NORM))
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

/// A slot's vector as a datum direction, through the kernel type's own
/// constructor: the decision and its three refusals live there, this
/// layer names the ROLE, and the refusal reaches the node error
/// through the same [`refusal`] map the evaluation layer's own
/// direction door uses — under [`DATUM_UNIT_NORM`], because on this
/// road the kernel type owns the value.
fn datum_unit<T: Decide>(
    v: Vec3<T>,
    role: &'static str,
    band: Band,
) -> Result<UnitVec3<T>, NodeErrorKind> {
    UnitVec3::new(v, band).map_err(|e| refusal(e, role, DATUM_UNIT_NORM))
}

/// **A profile's `f64` placement, where the document HOLDS one** — an
/// authored frame's nine expressions, resolved and orthonormalized;
/// `None` for a frame derived from a face, which the document does not
/// elaborate at any scalar.
///
/// # The two frame kinds (DM1c), and why the authored one is read here
///
/// This is the site that reads the plane, and it forks ONCE on the
/// frame node's kind, because the two kinds differ in where their
/// numbers come from.
///
/// An AUTHORED frame ([`Datum::Frame`]) is document expressions, and
/// they are read at `f64` rather than at the lane scalar for the C6
/// reason: the placement feeds STRUCTURE selection, which must be
/// lane-identical — the same document has to select the same structure
/// at `f64` and at the interval scalar, or the lift's two passes are
/// deciding different questions. That is the rule the profile's own
/// program already follows (`eval_node` resolves it "at f64 because it
/// feeds C6 structure selection"), and the frame's LANDED value is the
/// right answer to a different question (what a reader sees, what a
/// measure measures). So an authored frame is read twice, at two
/// scalars, for two purposes — `Pinned` places with THIS read,
/// `Guided` with [`frame_plane_lane`]'s.
///
/// A DERIVED frame ([`Datum::FaceFrame`]) has no document elaboration
/// at all: its placement is read off an upstream body's value, at
/// whatever scalar that body was evaluated at, so there is nothing
/// here to read and the answer is `None`. The profile's placement into
/// 3-D then comes from the lane under every lift
/// ([`frame_plane_lane`]), and its 2-D structure record is assembled
/// in the conventional `SketchPlane::xy()` ([`prepare_profile`]),
/// which no decision reads. `ProfilePre::placement_f64` carries the
/// same `Option`, so a consumer that places with a derived frame's
/// record has nothing to mistake for a placement.
///
/// # Errors
///
/// [`NodeErrorKind::WrongOperand`] when the reference does not name a
/// frame — the door every operand's kind is checked at — and the
/// frame's own two direction refusals through [`frame_axes`].
pub(crate) fn profile_plane_f64(
    doc: &crate::doc::Doc<ProfileProgram>,
    plane: RecipeNodeId,
    tol: Tol,
) -> Result<Option<profile::SketchPlane<f64>>, NodeErrorKind> {
    match frame_kind(doc, plane)? {
        FrameKind::Authored => {}
        FrameKind::Derived => return Ok(None),
    }
    let node = doc
        .node(plane)
        .ok_or(NodeErrorKind::MissingInput { input: plane })?;
    let env = doc.param_env::<f64>();
    let read = |family: fn(Axis3) -> SlotId| -> Result<Vec3<f64>, NodeErrorKind> {
        let mut out = [0.0_f64; 3];
        for axis in Axis3::ALL {
            let slot = family(axis);
            let expr = node.expr(slot).ok_or(NodeErrorKind::MissingSlot { slot })?;
            out[axis.index()] = crate::expr::eval(expr, &env)
                .map_err(|source| NodeErrorKind::Expr { slot, source })?;
        }
        Ok(Vec3::new(out[0], out[1], out[2]))
    };
    let origin = read(SlotId::Origin)?;
    let (u, v) = frame_axes(read(SlotId::U)?, read(SlotId::V)?, band(tol)?)?;
    Ok(Some(profile::SketchPlane::from_frame(
        Point3::new(origin.x, origin.y, origin.z),
        u.get(),
        v.get(),
    )))
}

/// Which kind of frame node a profile's `plane` names — the ONE fork
/// DM1c adds, keyed by node kind and never by a number.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub(crate) enum FrameKind {
    /// A [`Datum::Frame`]: nine document expressions, placed from the
    /// document at `f64` under `Pinned`.
    Authored,
    /// A [`Datum::FaceFrame`]: placed from the frame's landed value at
    /// the lane scalar under every lift.
    Derived,
}

/// Classifies the frame node a profile's `plane` names — the variant
/// read every by-value reader of a frame shares, so "is this a frame"
/// is answered once with one refusal vocabulary.
///
/// # Errors
///
/// [`NodeErrorKind::MissingInput`] for an absent node;
/// [`NodeErrorKind::WrongOperand`] when the node is not a frame.
pub(crate) fn frame_kind(
    doc: &crate::doc::Doc<ProfileProgram>,
    plane: RecipeNodeId,
) -> Result<FrameKind, NodeErrorKind> {
    match doc.node(plane) {
        None => Err(NodeErrorKind::MissingInput { input: plane }),
        Some(Node::Datum(Datum::Frame { .. })) => Ok(FrameKind::Authored),
        Some(Node::Datum(Datum::FaceFrame { .. })) => Ok(FrameKind::Derived),
        Some(_) => Err(NodeErrorKind::WrongOperand {
            input: plane,
            expected: "datum frame",
            found: "not a datum frame",
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
/// program has had since M10-P, applied to the input it just gained.
///
/// A DERIVED frame is read here under EVERY lift (DM1c): it has no
/// document elaboration, so this by-value read is the only placement
/// it has, and the profile on it is placed at the lane scalar with
/// its 2-D structure record still `f64`-pinned.
///
/// # Errors
///
/// [`NodeErrorKind::WrongOperand`] when the landed value is not a
/// frame — the kind door, at the lane where the value is read.
pub(crate) fn frame_plane_lane<T: Decide>(
    results: &Results<T>,
    plane: RecipeNodeId,
) -> Result<profile::SketchPlane<T>, NodeErrorKind> {
    let v = value_of(results, plane)?;
    let ValuePayload::Datum(DatumValue::Frame { origin, u, v: y }) = &v.payload else {
        return Err(NodeErrorKind::WrongOperand {
            input: plane,
            expected: "datum frame",
            found: v.payload.kind_name(),
        });
    };
    // Unit and perpendicular by the datum's own construction, which is
    // `SketchPlane::from_frame`'s stated obligation on its caller.
    Ok(profile::SketchPlane::from_frame(*origin, u.get(), y.get()))
}

/// A lane-scalar placement carried across to `f64`, exactly, where the
/// scalar is the `f64` lane — every component through
/// [`super::SectionScalar::pinned_f64`] — and `None` on any analysis
/// scalar. No component is inspected: the answer is the type's. The
/// walk is [`anchor::map_affine`], the fallible direction of the walk
/// whose infallible direction is `Affine3::map`.
pub(crate) fn pinned_plane<T: super::SectionScalar>(
    plane: &profile::SketchPlane<T>,
) -> Option<profile::SketchPlane<f64>> {
    anchor::map_affine(&plane.placement, |x| x.pinned_f64().ok_or(()))
        .ok()
        .map(profile::SketchPlane::new)
}

/// **A frame's authored pair, made orthonormal** — the one spelling of
/// it, and the one door its two refusals come out of.
///
/// Two callers need this and they must agree: the datum's own
/// evaluation, which produces the [`DatumValue::Frame`] a reader sees,
/// and the profile's f64 read of the frame it is drawn on. A second
/// spelling would be two frames for one node, free to disagree about
/// where a sketch's +x points.
///
/// `u` is normalized and KEPT; `v` yields its component along `u`.
/// Gram-Schmidt states "these two span no plane" as a length, so a
/// parallel pair refuses at the same decided door every other
/// direction does, under the y axis's role, rather than under a
/// predicate invented here.
pub(crate) fn frame_axes<T: Decide>(
    u_raw: Vec3<T>,
    v_raw: Vec3<T>,
    band: Band,
) -> Result<(UnitVec3<T>, UnitVec3<T>), NodeErrorKind> {
    let u = datum_unit(u_raw, "datum frame x axis", band)?;
    let v_perp = v_raw - u.get() * v_raw.dot(u.get());
    Ok((u, datum_unit(v_perp, "datum frame y axis", band)?))
}

/// Reads the frame an in-plane axis lives in, as the orthonormal pair
/// its coordinates are written against.
///
/// Same door as [`frame_plane_lane`]'s and same refusal: an axis whose
/// `plane` does not name a frame is a kind mismatch at the input, not
/// a geometry problem.
fn axis_frame<T: Decide>(
    results: &Results<T>,
    plane: RecipeNodeId,
) -> Result<AxisFrame<T>, NodeErrorKind> {
    let v = value_of(results, plane)?;
    let ValuePayload::Datum(DatumValue::Frame { origin, u, v: y }) = &v.payload else {
        return Err(NodeErrorKind::WrongOperand {
            input: plane,
            expected: "datum frame",
            found: v.payload.kind_name(),
        });
    };
    Ok(AxisFrame {
        origin: *origin,
        u: *u,
        v: *y,
    })
}

/// The frame an in-plane axis is written against, as the three vectors
/// the lift needs — a name rather than a bare triple, because a caller
/// that mixed up `u` and `v` would silently turn every such axis by a
/// right angle.
struct AxisFrame<T: Decide> {
    origin: Point3<T>,
    u: UnitVec3<T>,
    v: UnitVec3<T>,
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
                "datum plane normal",
                band(tol)?,
            )?,
        },
        Datum::Axis { .. } => DatumValue::Axis {
            origin: need_point3(vals, SlotId::Origin)?,
            dir: datum_unit(
                need_vec3(vals, SlotId::Direction)?,
                DATUM_AXIS_ROLE,
                band(tol)?,
            )?,
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
        //
        // u is normalized FIRST and kept: the frame's sketch +x is what
        // the author wrote, and v is the axis that yields. Choosing the
        // other order would silently rotate every profile drawn on the
        // frame when only v was edited.
        Datum::Frame { .. } => {
            let (u, v) = frame_axes(
                need_vec3(vals, SlotId::U)?,
                need_vec3(vals, SlotId::V)?,
                band(tol)?,
            )?;
            DatumValue::Frame {
                origin: need_point3(vals, SlotId::Origin)?,
                u,
                v,
            }
        }
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
            let f = axis_frame(results, *plane)?;
            let (frame_origin, u, v) = (f.origin, f.u, f.v);
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
                dir: datum_unit(lift(plane_dir), DATUM_AXIS_ROLE, band(tol)?)?,
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
            let ent = ladder::resolve_in(face, doc, table, |error| {
                NodeErrorKind::FaceFrameResolve { error }
            })?;
            let names::EntityKey::Face(key) = ent.key else {
                return Err(NodeErrorKind::FaceFrameKind {
                    name: Box::new(face.clone()),
                    found: ent.key.kind(),
                });
            };
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
            // DM1a: the outward normal is the sense beside the pose
            // times the chart axis, formed here, in the open. The
            // sense is a bool, so the sign is selected, never
            // computed.
            let n = if pose.sense { pose.axis } else { -pose.axis };
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
            let (u, v) = frame_axes(u_raw, v_raw, band(tol)?)?;
            DatumValue::Frame {
                origin: pose.origin,
                u,
                v,
            }
        }
    }))
}

/// The profile node's F64 PRECOMPUTE (LIB-SWITCH §4b): the resolved
/// program replays through `profile::replay` — the driver, the ONLY
/// path from steps to geometry — then the assembled `Profile<f64>`
/// validates at f64 (the C6 structure-selection gate, which also
/// yields the canonical form the program-anchor naming map is derived
/// from). Runs inside the node's verdict frame (`eval_node`) ahead of
/// the op: structure decisions, the successor of the stored f64 bits,
/// logged as the node's own. VQ6 is closed here and in the op below:
/// the replay-time junction checks and both validations run under the
/// SAME `Tolerance::get()` the evaluation pins.
pub(crate) fn prepare_profile(
    placement: Option<profile::SketchPlane<f64>>,
    resolved: &[Vec<profile::Step<f64>>],
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
    Ok(ProfilePre {
        profile_f64,
        placement_f64: placement,
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
    plane: profile::SketchPlane<T>,
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
        // The build path: the `f64` elaboration embedded bit for bit —
        // loops AND placement for an authored frame. A DERIVED frame
        // has no `f64` elaboration of its placement (DM1c: the
        // document holds a face name, not nine numbers), so its loops
        // embed exactly the same way and its placement is the lane's
        // own value, read where every by-value reader of a frame reads
        // it. The fork is by node kind; the numbers on both sides are
        // the ones already computed.
        super::ProfileLift::Pinned => {
            let mut embedded = anchor::embed_profile::<T>(&pre.profile_f64);
            embedded.plane = match &pre.placement_f64 {
                Some(placement) => placement.map(T::from_f64),
                None => frame_plane_lane(results, program.plane)?,
            };
            // Validated again at `T`: the validated form is minted by
            // `validate` at the scalar it is built at, and the pinned
            // lift embeds the f64 form whole rather than the
            // precompute's validated one, so at the build scalar this
            // repeats the precompute's validation decision for
            // decision and the node's log holds both (the doubled
            // populations `resolve::vdiff` describes).
            embedded.validate(tol).map_err(NodeErrorKind::Profile)?
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
/// record, then program-anchor rewritten — canonical → program indices,
/// LIB-SWITCH §6), the provenance stamp on everything the sweep minted,
/// and the per-edge parameter sources the verb's flow declares.
// The 8th argument is the verb's correspondence — the parameter that
// REMOVES duplication rather than adding a duty, exactly as
// `wire_blend`'s is; the 7th is the evaluation environment, read for
// the descent chain the attached tokens' scope is.
#[allow(clippy::too_many_arguments)]
fn wire_swept<
    T: Decide + geom_core::Bounds + geom_brep::PcurveFittedLane + crate::verbs::shell::ShellLane,
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
    let v = value_of(results, profile)?;
    let ValuePayload::Profile(vp) = &v.payload else {
        return Err(NodeErrorKind::WrongOperand {
            input: profile,
            expected: "profile",
            found: v.payload.kind_name(),
        });
    };
    let built = (verb.build)(args);
    // The verb's own declaration of where its parameters land, read off
    // the value the correspondence just built (VERB-SEAT-DESIGN V1).
    let flow = built.param_flow();
    let record = built
        .run_profile(&vp.validated, tol)
        .map_err(verb_refused)?;
    // Eager N4 emission from the emitter's own maps, inside the reader,
    // BEFORE the structural handoff is taken apart.
    let out = (verb.read)(id, record, verb.foreign_record)?;
    let table = anchored(out.table, &vp.naming)?;
    let mut body = out.body;
    // The sweep's own surfaces, curves and points are minted HERE
    // (D1/N6).
    stamp_minted(&mut body, id);
    // **Attach-at-mint for the lowered parameter-identity channel**
    // (VERB-SEAT-DESIGN P2), through the sweeps' per-EDGE flow source.
    // The token is not this node's: a swept wall's radius is the
    // PROFILE's, so what lowers is the operand profile's own carrier
    // radius, under this evaluation's scope, and the walls the record
    // exported are what it lands on. A profile with no carrier radius
    // (every polygon) yields no token and attaches nothing, which is
    // the declaration being obeyed rather than a case skipped.
    let scope = crate::param_source::ParamScope::of(doc.id(), env.parts.chain());
    let tokens = crate::param_source::profile_radius_tokens(doc, profile, &vp.naming, scope);
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
    T: Decide + geom_core::Bounds + geom_brep::PcurveFittedLane + crate::verbs::shell::ShellLane,
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
    T: Decide + geom_core::Bounds + geom_brep::PcurveFittedLane + crate::verbs::shell::ShellLane,
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
    let pv = value_of(results, profile)?;
    // `wire_swept` re-checks this and refuses identically, so the only
    // thing this pre-check decides is ORDER: a node whose profile input
    // is not a profile AND whose axis is not an in-plane axis must
    // refuse on the profile, because that is the operand the reader
    // named first and re-authoring a wrong axis for a document whose
    // profile was never one is a wasted edit.
    if !matches!(pv.payload, ValuePayload::Profile(_)) {
        return Err(NodeErrorKind::WrongOperand {
            input: profile,
            expected: "profile",
            found: pv.payload.kind_name(),
        });
    }
    let av = value_of(results, axis)?;
    let ValuePayload::Datum(DatumValue::AxisInPlane {
        plane_origin,
        plane_dir,
        ..
    }) = &av.payload
    else {
        return Err(NodeErrorKind::WrongOperand {
            input: axis,
            // A 3-D `Datum::Axis` lands here, and the sentence has to
            // say what to author instead: the seat is not "an axis",
            // it is "an axis written in the sketch the profile is
            // drawn on".
            expected: "an axis in a sketch frame (Datum::AxisInPlane)",
            found: av.payload.kind_name(),
        });
    };
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
/// Nothing here validates. The frame's unit-length and
/// perpendicularity conditions, the window's span and headroom, and
/// (for the hollow door) all three wall verdicts are the door's own,
/// decided against the run's band; a check here would be a second and
/// weaker opinion about a body this layer is not building.
struct TubeArgs<T: geom_core::Real> {
    center: Point3<T>,
    axis: Vec3<T>,
    u_ref: Vec3<T>,
    major_radius: T,
    window: sweep::TubeWindow<T>,
    minor_radius: T,
}

fn tube_args<T: Decide>(
    spine: RecipeNodeId,
    window: &crate::node::TubeWindow,
    results: &Results<T>,
    vals: &SlotValues<T>,
) -> Result<TubeArgs<T>, NodeErrorKind> {
    let sv = value_of(results, spine)?;
    let ValuePayload::Datum(DatumValue::Axis { origin, dir }) = &sv.payload else {
        return Err(NodeErrorKind::WrongOperand {
            input: spine,
            expected: "datum axis",
            found: sv.payload.kind_name(),
        });
    };
    // The datum is consumed WHOLE — origin as the spine centre, dir as
    // the spine axis — which is `Node::Revolve`'s precedent, and both
    // cross to the door verbatim: no re-origining, and nothing
    // normalized HERE.
    //
    // The axis arrives already unit-length, and that is the datum
    // node's doing rather than this arm's: `wire_datum` decides
    // `DATUM_UNIT_NORM` when it evaluates the axis, so a degenerate or
    // non-finite direction refuses there, one node upstream, and what
    // reaches the door is a `UnitVec3`. The door's own non-unit-axis
    // verdict is therefore unreachable along the recipe path — it
    // still guards the kernel-direct caller, which is who it was
    // written for. `u_ref` is a bare direction that passes through NO
    // datum, so its unit-length and perpendicularity verdicts are the
    // door's and stay reachable from a document.
    Ok(TubeArgs {
        center: *origin,
        axis: dir.get(),
        u_ref: need_vec3(vals, SlotId::Direction)?,
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
/// `Revolved<T>` maps it is handed — walls, rims, poles and the
/// partial/full kind — and never the profile that produced them; the
/// tube doors return a `Revolved<T>` built by the very same
/// `full`/`partial` machinery. So the revolve emitter names a tube
/// body with no translation and NO new `RoleSeg` variants: a tube's
/// bands, rims, meridians, caps and poles are those roles, in the
/// profile-loop/segment coordinates of the two-arc circle traversal
/// the door constructs (loop 0 the outer circle, loop 1 a hollow
/// tube's inner one; segments 0 and 1 its two half-circle arcs).
///
/// The one tube-specific step is a step NOT taken: there is no
/// `anchored` rewrite, because that rewrite maps a profile PROGRAM's
/// loop and step indices onto validate's canonical ones, and a tube
/// has no profile node. Its traversal is canonical by construction,
/// so the canonical indices ARE the final ones.
fn wire_tube<T: Decide + geom_brep::PcurveFittedLane>(
    id: RecipeNodeId,
    spine: RecipeNodeId,
    window: &crate::node::TubeWindow,
    results: &Results<T>,
    vals: &SlotValues<T>,
    tol: Tol,
) -> OpResult<T> {
    let a = tube_args(spine, window, results, vals)?;
    let mut built = sweep::tube_along_arc(
        a.center,
        a.axis,
        a.u_ref,
        a.major_radius,
        a.window,
        a.minor_radius,
        tol,
    )
    .map_err(|e| NodeErrorKind::Tube(Box::new(e)))?;
    let table = names::name_revolve(id, &built).map_err(NodeErrorKind::Naming)?;
    stamp_minted(&mut built.body, id);
    Ok(OpOut::plain(
        ValuePayload::Body(Arc::new(built.body)),
        table,
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
    let a = tube_args(spine, window, results, vals)?;
    let wall = need_scalar(vals, SlotId::TubeWall)?;
    let mut built = sweep::tube_along_arc_hollow(
        a.center,
        a.axis,
        a.u_ref,
        a.major_radius,
        a.window,
        a.minor_radius,
        wall,
        tol,
    )
    .map_err(|e| NodeErrorKind::Tube(Box::new(e)))?;
    let table = names::name_revolve(id, &built).map_err(NodeErrorKind::Naming)?;
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
fn verb_refused<T: crate::verbs::shell::ShellLane>(refusal: verbs::VerbError<T>) -> NodeErrorKind {
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
        // lane beside its rights (`ShellLane::witness`), never a
        // rendering or a drop.
        verbs::VerbError::Shell(error) => NodeErrorKind::Shell(Box::new(T::witness(*error))),
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
// duplication rather than adding a duty; the 9th is the evaluation
// environment, read for the descent chain the token's scope is.
#[allow(clippy::too_many_arguments)]
fn wire_blend<
    T: Decide + geom_core::Bounds + geom_brep::PcurveFittedLane + crate::verbs::shell::ShellLane,
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
/// seat's lane door, emit names from the birth record under THIS
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
fn wire_shell<T: Decide + crate::verbs::shell::ShellLane>(
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
    let out = T::run_shell(&built, &body, tol)
        .ok_or(NodeErrorKind::ShellLaneUnsupported {
            lane: <T as crate::lane::Lane>::NAME,
        })?
        .map_err(verb_refused)?;
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
/// [`ladder`], with two differences that are the door's own arity: an
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
    use crate::names::EntityKey;

    let mut keys = Vec::with_capacity(open.len());
    for name in open {
        let ent = ladder::resolve_in(name, doc, target, |error| NodeErrorKind::ShellOpenResolve {
            error,
        })?;
        let EntityKey::Face(k) = ent.key else {
            return Err(NodeErrorKind::ShellOpenKind {
                name: Box::new(name.clone()),
                found: ent.key.kind(),
            });
        };
        keys.push(k);
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
        let ent = ladder::resolve_in(name, doc, target, |error| {
            NodeErrorKind::BlendSelectionResolve { verb, error }
        })?;
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
        match self.key {
            crate::names::EntityKey::Body => Ok(self.body.faces().map(|(k, _)| k).collect()),
            crate::names::EntityKey::Face(k) => Ok(vec![k]),
            crate::names::EntityKey::Edge(_) => Err(NodeErrorKind::MeasureSelectionKind {
                verb: "min_clearance",
                found: "an edge",
            }),
            crate::names::EntityKey::Vertex(_) => Err(NodeErrorKind::MeasureSelectionKind {
                verb: "min_clearance",
                found: "a vertex",
            }),
        }
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
        let index = u32::try_from(index).unwrap_or(u32::MAX);
        if !read.contains(&index) {
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
        let operand =
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
        let (oa, ob) = (operand(a)?, operand(b)?);
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
    T: Decide + geom_core::Bounds + geom_brep::PcurveFittedLane + crate::verbs::shell::ShellLane,
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
    let wrong_tool = || NodeErrorKind::WrongOperand {
        input: tool,
        expected: verb.tool_expected,
        found: tv.payload.kind_name(),
    };
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
    let table = (verb.emitter)(
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
    Ok(OpOut::plain(ValuePayload::Split { above, below }, table))
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
        (PartSelect::SplitHalf(_), other) => {
            return Err(NodeErrorKind::WrongOperand {
                input: of,
                expected: "split",
                found: other.kind_name(),
            });
        }
        (PartSelect::Instance(_), other) => {
            return Err(NodeErrorKind::WrongOperand {
                input: of,
                expected: "instances",
                found: other.kind_name(),
            });
        }
    };
    let table = value
        .name_table
        .project(index)
        .map_err(|dup| NodeErrorKind::Naming(names::NamingError::from(dup)))?;
    names::check_total(&table, &body, 0).map_err(NodeErrorKind::Naming)?;
    Ok(OpOut::plain(ValuePayload::Body(body), Arc::new(table)))
}

// `Bounds` rides along for the boolean lane only (M5 PR 8): the sweep's
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
    T: Decide + geom_core::Bounds + geom_brep::PcurveFittedLane + crate::verbs::shell::ShellLane,
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
    // F5 threading (M4 PR 5): the Declare input's name pairs resolve
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
    let kernel_decls = match declare {
        None => BooleanDeclarations::none(),
        Some(d) => resolve_declarations(declared_pairs(results, d)?, doc, &a_table, &b_table)?,
    };
    let body_a = body_operand(results, a)?;
    let body_b = body_operand(results, b)?;
    match (verb.build)(op, kernel_decls)
        .run_pair(&body_a, &body_b, boolean_sweep, tol)
        .map_err(|err| refusal_menu(&a_table, &b_table, err))?
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
            let table = (verb.emitter)(
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
                table,
            ))
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
/// **Declarations are routed, not positioned** (DM4 as amended). The
/// node's optional `Declare` input names entities in this node's own
/// space, and [`route_declarations`] sends each pair to the one step
/// that joins the two things it names — derived from the member ids
/// the names carry. A step's bucket is resolved by the pair boolean's
/// own [`resolve_declarations`] against that step's two tables, so the
/// union adds the routing and reuses the door.
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
    T: Decide + geom_core::Bounds + geom_brep::PcurveFittedLane + crate::verbs::shell::ShellLane,
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
    let buckets: Vec<Vec<DeclaredPair>> = match declare {
        None => vec![Vec::new(); rest.len()],
        Some(d) => route_declarations(id, members, declared_pairs(results, d)?, doc)?,
    };
    let mut last: Option<(topo::BooleanResultKind, Arc<topo::ContactRecords>)> = None;
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
        // node's refusal named, and a refusal names published rows
        // (`union_refusal`). So the door reads the accumulation in the
        // one space a caller can write.
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
            let resolved = look_through_merges(id, &buckets[step], &acc_view, &member_table)?;
            resolve_declarations(&resolved, doc, &acc_view, &member_table)
                .map_err(|err| step_diagnosis(err, id, members, &buckets[step], &acc_view))?
        };
        match (verb.build)(BooleanOp::Union, decls)
            .run_pair(&acc_body, &member_body, boolean_sweep, tol)
            .map_err(|err| union_refusal(id, &acc_table, &member_table, err))?
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
                acc_table = (verb.emitter)(
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
                acc_body = Arc::new(out.body);
            }
        }
    }
    let table = names::name_union(id, &acc_body, &acc_table).map_err(NodeErrorKind::Naming)?;
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
    ))
}

/// One declared pair as the recipe carries it: the two names and the
/// contact class the author claimed for them.
type DeclaredPair = ((names::StableName, names::StableName), ContactClass);

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
    let value = value_of(results, declare)?;
    let ValuePayload::Declarations(pairs) = &value.payload else {
        return Err(NodeErrorKind::WrongOperand {
            input: declare,
            expected: "declarations",
            found: value.payload.kind_name(),
        });
    };
    Ok(pairs)
}

/// **Where one declared name sits in a union's own name space.**
///
/// The two cases are told apart by SHAPE, not by a stored position:
/// `member_view` mints exactly one segment, so a one-segment
/// `FromMember` row is a member's own entity and anything else in this
/// node's space is a row some fold step produced.
#[derive(Clone, Copy)]
enum DeclSite {
    /// A member's own entity, by its index in the member list.
    Member(usize),
    /// A row the fold minted (a seam, a merge, a fragment), by the
    /// index of the LATEST member it mentions — which bounds the step
    /// that could first have produced it.
    Accumulated(usize),
    /// A row of this node's space that names NO member: the
    /// accumulated body's own [`names::RoleSeg::OutputBody`] row, or a
    /// name discriminated only by an ordinal fragment rank.
    ///
    /// Its own case, and not folded into "denotes nothing here",
    /// because the two answers are different and a caller acts on
    /// them differently: such a row IS in this node's published space
    /// (the finished table carries it), it is simply a row no fold
    /// step has as one of its two OPERANDS — a step's output body is
    /// not one of that step's inputs.
    Unkeyed,
}

impl DeclSite {
    /// The first fold step (as a bucket index) at which this name is a
    /// row of one of the step's two operand tables, or `None` for a
    /// name that is a row of neither operand at any step.
    ///
    /// Member 0 is operand A of step 1 and member `i` is operand B of
    /// step `i`, so a member arrives at bucket `i - 1` and member 0 at
    /// bucket 0. A fold row is PRODUCED by a step, so the earliest it
    /// can be an operand is the step after — bucket 1 at the earliest,
    /// whichever member it mentions.
    fn arrival(self) -> Option<usize> {
        match self {
            DeclSite::Member(i) => Some(i.saturating_sub(1)),
            DeclSite::Accumulated(d) => Some(d.max(1)),
            DeclSite::Unkeyed => None,
        }
    }
}

/// **The step a declared pair is fed at**, as a bucket index — the
/// bucket rule, in one place.
///
/// The rule is the LATER of the two names' arrivals: both have to be
/// operand rows for the step to be able to join them, and each is one
/// from its arrival on. What arrival alone cannot say is the one
/// asymmetry between the two kinds of name — a member is the JOINING
/// operand at exactly its own step and is inside the ACCUMULATION at
/// every step after, so a member paired with a fold row minted later
/// than the member's own step is two rows of one operand there, which
/// is a different claim (a carried contact, not a contact between the
/// two things the step joins). That pair has no step, and says so
/// rather than being re-read as the other claim.
///
/// `None` means no step has both names as its two operands.
fn declared_bucket(s1: DeclSite, s2: DeclSite) -> Option<usize> {
    let bucket = s1.arrival()?.max(s2.arrival()?);
    for (site, other) in [(s1, s2), (s2, s1)] {
        if let (DeclSite::Member(n), DeclSite::Accumulated(_)) = (site, other) {
            // Member 0 is never the joining operand — it is where the
            // accumulation starts — so it is never the counterpart of
            // a fold row.
            if n == 0 || n - 1 != bucket {
                return None;
            }
        }
    }
    Some(bucket)
}

/// The index of the LATEST member of `members` this path mentions,
/// `Some(None)` for a path that mentions no member at all, and `None`
/// for a path that is not in this node's published space: one naming a
/// node the member list no longer holds — the state `SetMembers`
/// creates by removing a declared member — or one carrying a segment
/// the member-keying rule never mints.
fn latest_member(members: &[RecipeNodeId], path: &[names::RoleSeg]) -> Option<Option<usize>> {
    use crate::names::{Qualifier, RoleSeg};
    let mut best: Option<usize> = None;
    for seg in path {
        let here = match seg {
            // The member EDGE answers, and the name it wraps is NOT
            // walked: that name is in the member's own space, so a
            // member which is itself a union carries `FromMember`
            // segments naming ITS members, which are not this node's.
            RoleSeg::FromMember { member, .. } => Some(members.iter().position(|m| m == member)?),
            // The embedded names of a fold row are in this node's own
            // space — that is what the member-keying rule guarantees —
            // so they are walked by this same rule.
            RoleSeg::Seam { a, b } => {
                latest_member(members, &a.path)?.max(latest_member(members, &b.path)?)
            }
            RoleSeg::Merged(constituents) => {
                let mut acc = None;
                for c in constituents {
                    acc = acc.max(latest_member(members, &c.path)?);
                }
                acc
            }
            RoleSeg::Fragment(Qualifier::SideOf(partners)) => {
                let mut acc = None;
                for (n, _) in partners {
                    acc = acc.max(latest_member(members, &n.path)?);
                }
                acc
            }
            // The two segments this node's space carries that name no
            // member: the accumulated body's own row, and the ordinal
            // fragment discriminator, whose ranks are numbers.
            RoleSeg::OutputBody | RoleSeg::Fragment(Qualifier::OrderAlong { .. }) => None,
            // `FromA`/`FromB` are the fold's INTERNAL space — exactly
            // what `collapse` takes out — and everything after them is
            // a segment the boolean emitter never mints at all. A name
            // carrying either is not in this node's published space
            // and denotes nothing here. The long half is
            // [`names::never_in_a_boolean_table`], the one place a new
            // `RoleSeg` is classified; this match still stops the
            // compiler if one is added and not classified there.
            RoleSeg::FromA(_) | RoleSeg::FromB(_) | names::never_in_a_boolean_table!() => {
                return None;
            }
        };
        // `Option::max` IS "the later of two, either of which may be
        // absent": `None` orders below every `Some`.
        best = best.max(here);
    }
    Some(best)
}

/// Which of [`DeclSite`]'s three cases a declared name is, or `None`
/// if it is none of them — a name from another node's space, or one
/// naming a member the list no longer holds.
///
/// The `None` answer and [`DeclSite::Unkeyed`] are kept apart on
/// purpose: `None` is a name this node does not denote at all, which
/// is the vanished class, and `Unkeyed` is a name this node DOES
/// denote that no step has as an operand, which is the unroutable
/// class. Flattening the two would answer a caller's "why" with the
/// wrong sentence.
fn decl_site(
    id: RecipeNodeId,
    members: &[RecipeNodeId],
    name: &names::StableName,
) -> Option<DeclSite> {
    use crate::names::RoleSeg;
    // Every row of every operand table this node presents is minted
    // under this node (`member_view`, and the fold's own tables), so a
    // name minted elsewhere denotes nothing here.
    if name.node != id {
        return None;
    }
    if let [RoleSeg::FromMember { member, .. }] = name.path.as_slice() {
        return members
            .iter()
            .position(|m| m == member)
            .map(DeclSite::Member);
    }
    Some(match latest_member(members, &name.path)? {
        Some(d) => DeclSite::Accumulated(d),
        None => DeclSite::Unkeyed,
    })
}

/// **Routing a union's declared pairs to their fold steps** (DM4 as
/// amended): one bucket per step, filled from the MEMBER IDS the two
/// names carry and from nothing else.
///
/// No fold position is read and none is recorded. A pair says "this
/// entity meets that one"; which step joins the two is a consequence
/// of where their members sit in the list, and [`declared_bucket`] is
/// that consequence.
///
/// - Two names in ONE member are that member's CARRIED contact, at its
///   own step — operand B's carry there, or operand A's for the first
///   member, whose step is step 1.
/// - Two names in members `m` and `n` meet at the LATER one's step,
///   where the earlier is inside the accumulation and the later is the
///   joining member.
/// - A name the fold minted meets a member at that member's step,
///   provided the member joins after the row was minted; the same pair
///   one step earlier would be two rows of ONE operand, which is a
///   different claim, so it refuses rather than being re-read as one.
/// - Two fold rows are the accumulation's own carried contact, at the
///   first step that has both.
///
/// # A member-space declaration resolves through the fold's MERGES
///
/// A declaration is routed, so a pair of MEMBER names outlives any
/// reordering or dropping of the list: the two ids still say which
/// step joins them, wherever they sit. And it resolves at that step
/// through the merges the fold has performed by then: a declared
/// merge consumes the two faces it joins and publishes a `Merged` row
/// in their place, and a member-space name that is no longer an
/// operand row because of one is rewritten to the accumulation's
/// `Merged` row whose flat constituent set holds it
/// ([`look_through_merges`]) before the door runs. The set is flat
/// (N3), so the row is the same whatever order the merges happened
/// in: with `a` touching `c` and `c` touching `d`, every order of the
/// three fuses, and the cap row is `Merged({a, c, d})` in each.
///
/// That is the bound, and it is the merges alone. A member face the
/// fold consumed some other way is not looked through: one SPLIT by a
/// later member (its fragments are rows, the face is not), one
/// consumed by CONTAINMENT (inside another member, no row anywhere),
/// or one inside a merged row that was later FRAGMENTED (`[Merged(set),
/// Fragment(q)]` is a fragment, not a merge, and no set is searched
/// through it). A pair naming such a face resolves in the orders that
/// reach it while it is still a row and refuses `Vanished` in the
/// others — measured by `docm8_flat_merged`'s
/// `a_member_face_split_by_a_later_member_is_still_order_shaped`, filed
/// as
/// `work/docm/member-space-look-through-stops-at-splits-containment-and-fragmented-merges.md`.
///
/// Every name this node does not denote at all refuses through the N5
/// ladder as a vanished name does — a member the list no longer holds,
/// a name from another node's space — and nothing is dropped.
fn route_declarations(
    id: RecipeNodeId,
    members: &[RecipeNodeId],
    pairs: &[DeclaredPair],
    doc: &crate::doc::Doc<ProfileProgram>,
) -> Result<Vec<Vec<DeclaredPair>>, NodeErrorKind> {
    let steps = members.len().saturating_sub(1);
    let mut buckets: Vec<Vec<DeclaredPair>> = vec![Vec::new(); steps];
    for pair in pairs {
        let ((n1, n2), _) = pair;
        let site = |name: &names::StableName| -> Result<DeclSite, NodeErrorKind> {
            // Rung 1 first, as at every other door: a name whose
            // minting node is gone says THAT, before anything is said
            // about which step it would have belonged to.
            //
            // [`resolve_declarations`] pays rung 1 again, per bucket,
            // when the step it was routed to runs. That is two calls
            // for one name, and deliberately: this is the ROUTING
            // door, which must rank `NodeGone` above "no step joins
            // these", and that one is the pair boolean's own door,
            // which cannot assume a caller routed anything. Neither
            // can be dropped without one of them answering a name it
            // has not checked; the cost is a document lookup on a path
            // that already refuses.
            ladder::live(name, doc).map_err(|error| NodeErrorKind::DeclareResolve { error })?;
            decl_site(id, members, name).ok_or_else(|| NodeErrorKind::DeclareResolve {
                error: Box::new(crate::resolve::ResolveError::vanished_fallback(name)),
            })
        };
        let (s1, s2) = (site(n1)?, site(n2)?);
        let unroutable = || NodeErrorKind::UnionDeclareStep {
            pair: Box::new((n1.clone(), n2.clone())),
        };
        // One rule, one home ([`declared_bucket`]) — and a step past
        // the last one is no step at all: the fold is over before both
        // names are operands together.
        let Some(bucket) = declared_bucket(s1, s2).filter(|b| *b < steps) else {
            return Err(unroutable());
        };
        buckets[bucket].push(pair.clone());
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
/// Three things it does not do. Only a MEMBER-SPACE name looks
/// through; an accumulation-entity name (`Seam`, `Merged`, `Fragment`,
/// `OutputBody`) is its own row or nothing. A member-space name in no
/// table and in no merged row's set is left alone, and the door
/// refuses it as the vanished name it is. And a pair whose two names
/// land on ONE row is handed to the door as such, and refuses there
/// by the door's own same-operand rule.
///
/// A face in the set of TWO merged rows cannot happen under the flat
/// mint — a merged face's constituents retire, and a merge over it
/// lists them in the new row's set and drops the old row — so meeting
/// one is refused as the emission bug it would be.
fn look_through_merges(
    id: RecipeNodeId,
    bucket: &[DeclaredPair],
    acc_table: &NameTable,
    member_table: &NameTable,
) -> Result<Vec<DeclaredPair>, NodeErrorKind> {
    use crate::names::RoleSeg;
    let merged_row_of =
        |name: &names::StableName| -> Result<Option<names::StableName>, NodeErrorKind> {
            // Only a MEMBER's own face looks through. The guard is
            // not redundant: a fragment of a member face
            // (`[FromMember, Fragment]`) or of a merged face is a
            // legitimate constituent of a later merge, and a declared
            // name of that shape is an accumulation entity — its own
            // row or nothing.
            if name.node != id || !matches!(name.path.as_slice(), [RoleSeg::FromMember { .. }]) {
                return Ok(None);
            }
            if acc_table.lookup(name).is_some() || member_table.lookup(name).is_some() {
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
        .map(|((n1, n2), class)| {
            let r1 = merged_row_of(n1)?.unwrap_or_else(|| n1.clone());
            let r2 = merged_row_of(n2)?.unwrap_or_else(|| n2.clone());
            Ok(((r1, r2), *class))
        })
        .collect()
}

/// A union's accumulation lists one member face in the constituent
/// sets of two merged rows, which the flat mint cannot produce.
const MEMBER_FACE_IN_TWO_MERGES: &str =
    "a union's accumulation holds one member face in two merged rows' constituent sets";

/// **A routed FOLD ROW that is not an operand row at the step it was
/// sent to** — said as the routing failure it is.
///
/// [`resolve_declarations`] answers for the pair boolean too, where a
/// name absent from both operand tables has genuinely vanished: no
/// table in the evaluation derives it any more. Under a union that
/// reading survives for a MEMBER's own row — its absence is the
/// member's table not deriving it (a name written against an entity
/// the member does not have), or the fold having consumed it, which is
/// the vanished class either way and is diagnosed as one.
///
/// A row the FOLD minted is different. Its bucket is bounded by the
/// latest member it mentions, and that bound is not tight: a row can be
/// minted at a step LATER than the member index alone implies, in which
/// case the step it was routed to does not have it. Nothing vanished
/// there — the name is this node's, and the fold does mint it — the
/// pair simply has no step that holds both its names, which is
/// [`NodeErrorKind::UnionDeclareStep`] and is what the author acts on.
///
/// Only the vanished rung is re-said. `NodeGone` cannot occur here —
/// routing paid rung 1 already — and a tie
/// ([`crate::resolve::ResolveError::Ambiguous`]),
/// `DeclareBothOperands` and `DeclareUnsupportedPair` are answers about
/// the name itself that a step index would not improve.
///
/// A fold row that a LATER merge absorbed is not re-said either: it
/// is covered by the accumulation's merged row that lists its faces
/// (`names::merged::covers`), which is N3's offer for it — the name
/// vanished into a merge, and the vanished rung with that offer is
/// the true answer, not a routing failure.
fn step_diagnosis(
    err: NodeErrorKind,
    id: RecipeNodeId,
    members: &[RecipeNodeId],
    bucket: &[DeclaredPair],
    acc_table: &NameTable,
) -> NodeErrorKind {
    use crate::names::RoleSeg;
    let NodeErrorKind::DeclareResolve { error } = &err else {
        return err;
    };
    let crate::resolve::ResolveError::Vanished { name, .. } = &**error else {
        return err;
    };
    if !matches!(decl_site(id, members, name), Some(DeclSite::Accumulated(_))) {
        return err;
    }
    let absorbed = acc_table.iter().any(|(row, _)| match row.path.as_slice() {
        [RoleSeg::Merged(set)] => names::merged::covers(set, name),
        _ => false,
    });
    if absorbed {
        return err;
    }
    let Some(((n1, n2), _)) = bucket.iter().find(|((a, b), _)| a == name || b == name) else {
        return err;
    };
    NodeErrorKind::UnionDeclareStep {
        pair: Box::new((n1.clone(), n2.clone())),
    }
}

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
/// [`names::collapse_name`], the same rewrite the node's own table gets
/// from `name_union`, so a refusal denotes member-space entities and
/// nothing else.
///
/// A name that will not collapse is an emission bug in the fold's own
/// table, and it is raised as one rather than swallowed: the union was
/// going to fail naming for the same reason had the step succeeded, and
/// a bug reported as a contact refusal would send a caller to edit
/// their model over a defect in this crate.
///
/// The recourse a caller whose members touch has is the pair boolean's,
/// on this node: wire a `Declare` naming the two entities to the
/// union's own `declare` input. The names it carries are the ones this
/// refusal carries — member-space rows of this node — and the step the
/// pair is fed at is derived from them ([`route_declarations`]).
fn union_refusal<T: crate::verbs::shell::ShellLane>(
    id: RecipeNodeId,
    a_table: &crate::names::NameTable,
    b_table: &crate::names::NameTable,
    err: verbs::VerbError<T>,
) -> NodeErrorKind {
    let refused = refusal_menu(a_table, b_table, err);
    let NodeErrorKind::UndeclaredContact { finding, diag } = refused else {
        return refused;
    };
    let names::FlushFinding {
        pair: (a, b),
        class,
        evidence,
    } = *finding;
    let (Ok(a), Ok(b)) = (names::collapse_name(id, &a), names::collapse_name(id, &b)) else {
        return NodeErrorKind::Naming(names::NamingError::Emission {
            what: UNION_REFUSAL_FOREIGN,
        });
    };
    NodeErrorKind::UndeclaredContact {
        finding: Box::new(names::FlushFinding {
            pair: (a, b),
            class,
            evidence,
        }),
        diag,
    }
}

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
fn refusal_menu<T: crate::verbs::shell::ShellLane>(
    a_table: &crate::names::NameTable,
    b_table: &crate::names::NameTable,
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
    let name_of = |(operand, face): (topo::Operand, topo::FaceKey)| {
        let table = match operand {
            topo::Operand::A => a_table,
            topo::Operand::B => b_table,
        };
        face_name(table, face)
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
/// tables into the kernel's [`BooleanDeclarations`] (F5, M4 PR 5).
///
/// **One definition, two doors.** [`wire_boolean`] calls it with the
/// two operands' tables; [`wire_union`] calls it once per fold step
/// with that step's two — the accumulation in this node's published
/// space and the joining member's `member_view` — for the bucket
/// [`route_declarations`] sent there. The side-picking below is the
/// same for both, which is what makes "resolve a declared name against
/// two tables" one answer rather than two.
///
/// v1 vocabulary: cross-operand Face–Face pairs (cosurface glue
/// intents — the resolver is carrier-agnostic and always was: it
/// pushes a `FacePairDeclaration` whatever the two faces' surface
/// kinds are, and the kernel's ladder is what verifies it) and
/// same-operand Vertex–Vertex / Vertex–Face pairs (carried 3′
/// contacts). Everything else refuses typed. Resolution
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
        // `route_declarations` has already paid this for a union's
        // names, one bucket earlier; it stays here because this door
        // is also the pair boolean's, where nothing routed first.
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
            // Cross-operand face pair: the cosurface glue intent, on
            // whatever carrier the two faces share.
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
/// PR 1's die convention: rotate about the axis THROUGH THE WORLD
/// ORIGIN by `angle`, then translate. `axis` is already unit — the
/// callers normalize it through [`unit()`] under
/// [`TRANSFORM_AXIS_ROLE`], where the degenerate and non-finite cases
/// refuse.
pub(crate) fn transform_map<T: Decide>(
    translation: Vec3<T>,
    axis: Vec3<T>,
    angle: T,
) -> Affine3<T> {
    Affine3::from_parts(Mat3::rotation_about(axis, angle), translation)
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
fn wire_transform<T: Decide + geom_brep::PcurveFittedLane>(
    id: RecipeNodeId,
    input: RecipeNodeId,
    results: &Results<T>,
    vals: &SlotValues<T>,
    tol: Tol,
) -> OpResult<T> {
    let value = value_of(results, input)?;
    let operand = placeable_operand(value, input)?;
    let translation = need_vec3(vals, SlotId::Translation)?;
    let rot_axis = unit(
        need_vec3(vals, SlotId::RotationAxis)?,
        TRANSFORM_AXIS_ROLE,
        band(tol)?,
    )?;
    let angle = need_scalar(vals, SlotId::RotationAngle)?;
    let map = transform_map(translation, rot_axis, angle);
    let payload = operand.map(|body, i| {
        let ordinal = names::output_body(i).map_err(NodeErrorKind::Naming)?;
        place(body, Some(&map), id, ordinal, tol)
    })?;
    Ok(OpOut::plain(payload, Arc::clone(&value.name_table)))
}

/// The resolved operands of a stepped placement rule: what the rule's
/// math consumes once every slot or expression is evaluated and every
/// direction is unit. The two rules get there by different roads: a
/// LINEAR rule's direction is a slot this layer normalizes through
/// [`unit()`], while a CIRCULAR rule's axis arrives already unit out of
/// a datum's `UnitVec3` — the kernel type's constructor did it, and
/// `.get()` only reads it back.
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
        /// The axis direction, unit because it came out of the datum's
        /// `UnitVec3` — no door here re-decides it.
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
                PATTERN_DIRECTION_ROLE,
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
    let value = value_of(results, input)?;
    let operand = placeable_operand(value, input)?;
    let master = operand.bodies();
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
fn section_of<T: Decide + geom_core::Bounds + super::SectionScalar>(
    doc: &crate::doc::Doc<ProfileProgram>,
    results: &Results<T>,
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
    // DM1c: a section on a DERIVED frame has no `f64` placement of its
    // own — the frame's landed value is the lane's — and a section's
    // geometry stays `f64`. So the placement comes off the by-value
    // read, and crosses to `f64` only where the scalar IS `f64`
    // (`SectionScalar`, decided by the type); anywhere else the
    // section refuses typed, naming itself and the frame, rather than
    // placing on a fabricated point of the frame's bracket.
    let plane = match profile_plane_f64(doc, program.plane, tol)? {
        Some(authored) => authored,
        None => {
            let lane_plane = frame_plane_lane(results, program.plane)?;
            pinned_plane(&lane_plane).ok_or(NodeErrorKind::DerivedFrameSection {
                profile: id,
                frame: program.plane,
            })?
        }
    };
    let pre = prepare_profile(Some(plane), &resolved, tol)?;
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
    let mut first_naming = ProfileNaming::default();
    for (i, pid) in profiles.iter().enumerate() {
        let (chain, place, naming) = section_of::<T>(doc, results, *pid, lane, tol)?;
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

/// **The union's declaration routing, read directly.**
///
/// The buckets [`route_declarations`] fills are not visible in a
/// finished evaluation — a document only shows which pairs RESOLVED —
/// so the rule is read here, where the answer is the bucket list
/// itself. What a document row cannot pin and this can: that a step
/// with no declared pair receives NOTHING, and that a pair is fed at
/// exactly one step rather than tried at several.
#[cfg(test)]
mod route_tests {
    #![allow(clippy::unwrap_used, clippy::expect_used, clippy::panic)]

    use super::{
        DeclaredPair, NodeErrorKind, RecipeNodeId, look_through_merges, resolve_declarations,
        route_declarations,
    };
    use crate::names::{CapEnd, EntityKey, EntityKind, EntityRef, NameTable, RoleSeg, StableName};
    use crate::node::Node;
    use crate::{DocEdit, ProfileDoc};
    use geom_core::Tol;
    use topo::ContactClass;

    /// A live document holding one node, whose id stands in for the
    /// union's: routing reads the document for rung 1 only (is the
    /// minting node live), and every member-space name carries the
    /// union's own id, so one live node is the whole dependency.
    fn doc_with_one_node() -> (ProfileDoc, RecipeNodeId) {
        let doc = ProfileDoc::empty_derived("union declare routing", Tol::witness());
        let applied = doc
            .apply(
                &DocEdit::InsertNode {
                    node: Node::declare_rest(Vec::new()),
                },
                Tol::witness(),
            )
            .expect("an empty Declare inserts");
        let id = applied.record.minted.expect("the insert minted an id");
        (applied.doc, id)
    }

    /// One member's own face, as `member_view` spells it: a one-segment
    /// `FromMember` path under the union's id.
    fn member_face(union: RecipeNodeId, member: RecipeNodeId) -> StableName {
        StableName {
            kind: EntityKind::Face,
            node: union,
            path: vec![RoleSeg::FromMember {
                member,
                of: Box::new(StableName {
                    kind: EntityKind::Face,
                    node: member,
                    path: vec![RoleSeg::Cap(CapEnd::Start)],
                }),
            }],
        }
    }

    /// A row the fold mints: the merge of two members' faces.
    fn merged(union: RecipeNodeId, a: RecipeNodeId, b: RecipeNodeId) -> StableName {
        StableName {
            kind: EntityKind::Face,
            node: union,
            path: vec![RoleSeg::Merged(vec![
                member_face(union, a),
                member_face(union, b),
            ])],
        }
    }

    /// The union's own body row — this node's space, and no step's
    /// operand.
    fn output_body(union: RecipeNodeId) -> StableName {
        StableName {
            kind: EntityKind::Body,
            node: union,
            path: vec![RoleSeg::OutputBody],
        }
    }

    fn pair(a: StableName, b: StableName) -> DeclaredPair {
        ((a, b), ContactClass::Rest)
    }

    /// The bucket of each pair on a FOUR-member document, read out of
    /// the routing itself.
    #[test]
    fn each_pair_is_routed_to_the_one_step_that_joins_its_two_names() {
        let (doc, union) = doc_with_one_node();
        let members: Vec<RecipeNodeId> = (100..104).map(RecipeNodeId).collect();
        let (m0, m1, m2, m3) = (members[0], members[1], members[2], members[3]);
        let face = |m| member_face(union, m);

        let cases: Vec<(&str, DeclaredPair, usize)> = vec![
            // Two members meet at the LATER one's step.
            ("members 0 and 1", pair(face(m0), face(m1)), 0),
            ("members 1 and 2", pair(face(m1), face(m2)), 1),
            // The list's own gap: (1, 3) is fed at step 3 and step 2
            // receives nothing, which is the half a resolved-or-not
            // document cannot show.
            ("members 1 and 3", pair(face(m1), face(m3)), 2),
            // Two names in ONE member are that member's carried
            // contact, at its own step — member 0's at step 1, where it
            // is operand A, exactly as in the pair chain.
            ("member 2 with itself", pair(face(m2), face(m2)), 1),
            ("member 0 with itself", pair(face(m0), face(m0)), 0),
            // A fold row meets a member that joins after it was minted.
            (
                "the 0-1 merge and member 2",
                pair(merged(union, m0, m1), face(m2)),
                1,
            ),
            // Two fold rows are the accumulation's own carried contact,
            // at the first step that has both.
            (
                "the 0-1 merge and the 0-2 merge",
                pair(merged(union, m0, m1), merged(union, m0, m2)),
                2,
            ),
        ];

        for (what, p, want) in cases {
            let buckets = route_declarations(union, &members, std::slice::from_ref(&p), &doc)
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
            pair(face(m2), face(m2)),
        ];
        let buckets = route_declarations(union, &members, &all, &doc).expect("all three route");
        let sizes: Vec<usize> = buckets.iter().map(Vec::len).collect();
        assert_eq!(sizes, vec![1, 1, 1]);
    }

    /// The arrival clamp: a member is the JOINING operand at exactly
    /// its own step, so a fold row minted after that step is never that
    /// member's counterpart — the two would be rows of one operand
    /// there, a different claim.
    #[test]
    fn a_fold_row_paired_with_a_member_the_fold_already_swallowed_has_no_step() {
        let (doc, union) = doc_with_one_node();
        let members: Vec<RecipeNodeId> = (100..104).map(RecipeNodeId).collect();
        let (m0, m1, m2) = (members[0], members[1], members[2]);
        for (what, p) in [
            // The 1-2 merge is minted at step 2; member 1 joined at
            // step 1.
            (
                "a later merge with an earlier member",
                pair(merged(union, m1, m2), member_face(union, m1)),
            ),
            // Member 0 is where the accumulation starts and is never a
            // joining operand.
            (
                "any merge with member 0",
                pair(merged(union, m0, m1), member_face(union, m0)),
            ),
        ] {
            let refused = route_declarations(union, &members, std::slice::from_ref(&p), &doc);
            assert!(
                matches!(refused, Err(NodeErrorKind::UnionDeclareStep { .. })),
                "{what}: {refused:?}",
            );
        }
    }

    /// A row of this node's space that names no member — the union's
    /// own body — is a step's OUTPUT and never one of its two inputs.
    /// It refuses as unroutable, not as vanished: the union publishes
    /// that name, so "no table derives it any more" would be false.
    #[test]
    fn the_unions_own_body_row_is_no_steps_operand() {
        let (doc, union) = doc_with_one_node();
        let members: Vec<RecipeNodeId> = (100..103).map(RecipeNodeId).collect();
        let p = pair(output_body(union), member_face(union, members[1]));
        let refused = route_declarations(union, &members, std::slice::from_ref(&p), &doc);
        assert!(
            matches!(refused, Err(NodeErrorKind::UnionDeclareStep { .. })),
            "{refused:?}",
        );
    }

    /// A real face key for a hand-built table — a unit cube's top cap.
    /// Nothing below reads the body; the door reads tables.
    fn a_face_key() -> topo::FaceKey {
        use profile::RawLoop;
        let plane = profile::SketchPlane::from_frame(
            geom_core::Point3::new(0.0, 0.0, 0.0),
            geom_core::Vec3::new(1.0, 0.0, 0.0),
            geom_core::Vec3::new(0.0, 1.0, 0.0),
        );
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

    /// The rewrite touches ONE shape: a member face that is no row and
    /// sits in a merged row's set goes to that row. An accumulation
    /// name — even one whose whole set is inside a wider row's — and a
    /// member face in no set are handed on as written.
    #[test]
    fn look_through_rewrites_only_a_member_face_inside_a_merged_row() {
        let (_doc, union) = doc_with_one_node();
        let ms: Vec<RecipeNodeId> = (100..104).map(RecipeNodeId).collect();
        let key = a_face_key();
        // Step 2's row: the merge of step 1's `{m0, m1}` with `m2`,
        // flat.
        let mut set = vec![
            member_face(union, ms[0]),
            member_face(union, ms[1]),
            member_face(union, ms[2]),
        ];
        set.sort();
        let wide = StableName {
            kind: EntityKind::Face,
            node: union,
            path: vec![RoleSeg::Merged(set)],
        };
        let mut acc = NameTable::new();
        acc.insert(wide.clone(), face_ref(key)).unwrap();
        let mut member = NameTable::new();
        member
            .insert(member_face(union, ms[3]), face_ref(key))
            .unwrap();
        let bucket = vec![
            pair(member_face(union, ms[0]), member_face(union, ms[3])),
            pair(merged(union, ms[0], ms[1]), member_face(union, ms[3])),
            pair(
                member_face(union, RecipeNodeId(200)),
                member_face(union, ms[3]),
            ),
        ];
        let out = look_through_merges(union, &bucket, &acc, &member).unwrap();
        assert_eq!(out[0].0, (wide, member_face(union, ms[3])));
        assert_eq!(out[1], bucket[1]);
        assert_eq!(out[2], bucket[2]);
    }

    /// The member-space guard observed: a FRAGMENT of a member face
    /// (`[FromMember, Fragment]`) is a legitimate constituent of a
    /// later merge, and a declared name of that shape — an
    /// accumulation entity — is not rewritten to the row that lists
    /// it, though a membership test alone would find it.
    #[test]
    fn a_fragment_of_a_member_face_inside_a_merged_row_does_not_look_through() {
        let (_doc, union) = doc_with_one_node();
        let (m0, m1, m2) = (RecipeNodeId(100), RecipeNodeId(101), RecipeNodeId(102));
        let fragment = StableName {
            kind: EntityKind::Face,
            node: union,
            path: vec![
                RoleSeg::FromMember {
                    member: m0,
                    of: Box::new(StableName {
                        kind: EntityKind::Face,
                        node: m0,
                        path: vec![RoleSeg::Cap(CapEnd::Start)],
                    }),
                },
                RoleSeg::Fragment(crate::names::Qualifier::OrderAlong { rank: 0, of: 2 }),
            ],
        };
        let mut set = vec![fragment.clone(), member_face(union, m1)];
        set.sort();
        let row = StableName {
            kind: EntityKind::Face,
            node: union,
            path: vec![RoleSeg::Merged(set)],
        };
        let mut acc = NameTable::new();
        acc.insert(row, face_ref(a_face_key())).unwrap();
        let member = NameTable::new();
        let p = pair(fragment, member_face(union, m2));
        let out = look_through_merges(union, std::slice::from_ref(&p), &acc, &member).unwrap();
        assert_eq!(out[0], p);
    }

    /// One member face in TWO merged rows' sets — a table the flat
    /// mint cannot produce — is refused as the emission bug it is,
    /// never resolved to whichever row came first.
    #[test]
    fn a_member_face_in_two_merged_rows_refuses_as_an_emission_bug() {
        let (_doc, union) = doc_with_one_node();
        let (m0, m1, m2, m3) = (
            RecipeNodeId(100),
            RecipeNodeId(101),
            RecipeNodeId(102),
            RecipeNodeId(103),
        );
        let key = a_face_key();
        let mut acc = NameTable::new();
        acc.insert(merged(union, m0, m1), face_ref(key)).unwrap();
        // A second entity for the second row: the table refuses two
        // names on one entity, and the shape under test is two rows.
        acc.insert(
            merged(union, m0, m2),
            EntityRef {
                body: 1,
                key: EntityKey::Face(key),
            },
        )
        .unwrap();
        let member = NameTable::new();
        let p = pair(member_face(union, m0), member_face(union, m3));
        let refused = look_through_merges(union, std::slice::from_ref(&p), &acc, &member);
        assert!(
            matches!(
                refused,
                Err(NodeErrorKind::Naming(crate::names::NamingError::Emission { what }))
                    if what == super::MEMBER_FACE_IN_TWO_MERGES
            ),
            "{refused:?}"
        );
    }

    /// Both names of one pair inside ONE merged row: the rewrite hands
    /// the door that row twice, and the door refuses it by its own
    /// same-operand rule — a pair of two faces of one operand is not a
    /// contact the pair verb takes.
    #[test]
    fn a_pair_whose_two_names_rewrite_to_one_row_refuses_typed() {
        let (doc, union) = doc_with_one_node();
        let (m0, m1) = (RecipeNodeId(100), RecipeNodeId(101));
        let row = merged(union, m0, m1);
        let mut acc = NameTable::new();
        acc.insert(row.clone(), face_ref(a_face_key())).unwrap();
        let member = NameTable::new();
        let p = pair(member_face(union, m0), member_face(union, m1));
        let out = look_through_merges(union, std::slice::from_ref(&p), &acc, &member).unwrap();
        assert_eq!(out[0].0, (row.clone(), row));
        let refused = resolve_declarations(&out, &doc, &acc, &member);
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
