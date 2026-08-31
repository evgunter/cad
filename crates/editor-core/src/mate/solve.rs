//! **The mate solve** — reading edges, partitions, clusters, and the
//! constructive placement (ASM-R2a D-2/D-3/D-4/D-5; A9/A10/A11/A12).
//!
//! Everything here is recipe data plus decided predicates: no geometry
//! is inspected, nothing derived is stored beside the DAG. The
//! entry points, in the order the layers use them:
//!
//! - [`reading_edges`] — A12's second sort of edge, RECOMPUTED from
//!   the name heads every time it is wanted.
//! - [`relative_freedom_components`] — A9's partition, over consuming
//!   ∪ reading edges (so mates couple components).
//! - [`clusters`] — A11's placement clusters, the finer partition over
//!   instances alone, each with its document-order-first GAUGE.
//! - [`solve_document`] — the per-pair coset fold along a deterministic
//!   spanning tree, yielding every instance's pose relative to its
//!   gauge, and every mate's role.
//! - [`reconcile`] — the cluster-record keying maintenance the edit
//!   door runs after any edit that can move the mate graph.
//!
//! # Why the failure surface is per node, not per document
//!
//! A refusing cluster must not fail an unrelated one: GQ2/W5 make a
//! node's failure poison its descendants and nothing else, and a
//! second cluster has no dependence on the first. So [`solve_document`]
//! is TOTAL — it returns per-node faults rather than one document-wide
//! `Err`, and every node the fault actually reaches (the refusing mate
//! and its cluster's instances, which now have no pose) carries it.

use std::collections::{BTreeMap, BTreeSet, VecDeque};

use geom_core::Tol;
use geom_core::linalg::{Affine3, Mat3, Point3, Vec3};
use geom_core::predicate::Band;

use super::coset::{Coset, FoldStop, Subgroup};
use super::{Alignment, AxisSense, MateFault, MatePrimitive, MateSide};
use crate::doc::Doc;
use crate::names::RoleSeg;
use crate::node::{Datum, Node, PatternKind, RecipeNodeId};
use crate::placement::Frame;

/// What a mate did in the solve (A11 rule 4).
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum MateRole {
    /// A tree mate: it placed its child. Its pair's fold was DETERMINED.
    Determining,
    /// A non-tree mate: it solved nothing and is carried to evaluation
    /// as a pure contact declaration. R2-b verifies it against the
    /// solved geometry; this unit records only that it declares.
    Declaring,
    /// The mate refused; see the fault recorded against it.
    Refused,
}

/// The document's solved poses (D-5's compose-outward input).
#[derive(Debug, Clone, Default)]
pub struct SolvedPoses {
    /// Each live instance's pose RELATIVE TO ITS CLUSTER GAUGE. The
    /// gauge's own entry is the identity, bit-exactly.
    relative: BTreeMap<RecipeNodeId, Frame>,
    /// Each live instance's cluster gauge.
    gauge: BTreeMap<RecipeNodeId, RecipeNodeId>,
    /// Each live mate's role.
    roles: BTreeMap<RecipeNodeId, MateRole>,
    /// Per-node refusals: the refusing mate, and every node in its
    /// cluster that consequently has no pose.
    faults: BTreeMap<RecipeNodeId, MateFault>,
}

impl SolvedPoses {
    /// A node's recorded fault, if the solve refused for it.
    pub fn fault(&self, node: RecipeNodeId) -> Option<&MateFault> {
        self.faults.get(&node)
    }

    /// A mate's role, `None` if the node is not a live mate.
    pub fn role(&self, mate: RecipeNodeId) -> Option<MateRole> {
        self.roles.get(&mate).copied()
    }

    /// An instance's cluster gauge, `None` if the node is not a live
    /// instance.
    pub fn gauge(&self, instance: RecipeNodeId) -> Option<RecipeNodeId> {
        self.gauge.get(&instance).copied()
    }

    /// An instance's pose relative to its cluster gauge.
    pub fn relative(&self, instance: RecipeNodeId) -> Option<Frame> {
        self.relative.get(&instance).copied()
    }

    /// **The instance's world placement** (D-5): the cluster's recorded
    /// frame composed onto the solved relative pose. The gauge's
    /// relative pose is the bit-exact identity, so a singleton cluster
    /// returns its recorded frame VERBATIM — the mate-less document's
    /// evaluation is bit-for-bit what it was before mates existed.
    ///
    /// # Errors
    ///
    /// The cluster's own refusal, when it did not solve.
    pub fn placement<P>(
        &self,
        doc: &Doc<P>,
        instance: RecipeNodeId,
    ) -> Result<Frame, Box<MateFault>> {
        if let Some(fault) = self.faults.get(&instance) {
            return Err(Box::new(fault.clone()));
        }
        let gauge = self.gauge.get(&instance).copied().unwrap_or(instance);
        let relative = self.relative.get(&instance).copied().unwrap_or_default();
        Ok(doc
            .placements()
            .get(&gauge)
            .copied()
            .unwrap_or_default()
            .compose(&relative))
    }
}

// ---- A12: reading edges, recomputed ----

/// **The member a mate reference's head names** (A11's member
/// vocabulary): a live `InstantiatePart`, or a pattern-placed instance
/// — the `Pattern` node with its `Instance(i)` qualifier.
///
/// A member is more than its cluster-graph vertex: mates to SIBLING
/// copies of one pattern relate the same pair of instances through
/// different static offsets, so the copy is part of the member's
/// identity — it is what makes a second sibling mate close a LOOP
/// (non-tree, declaring) instead of folding into the first mate's
/// pair.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub struct Member {
    /// The cluster-graph vertex this member stands on: the head
    /// instance itself, or — for a pattern-placed member — the
    /// pattern's INPUT instance, whose pose the pattern's copies are
    /// all derived from. This is the edge end A9/A11's partitions see,
    /// and why a mate to `Instance(i)` joins the other member into the
    /// pattern's cluster.
    pub instance: RecipeNodeId,
    /// Which pattern copy this member is — `(pattern node, structural
    /// index)` — `None` for a plain instance head.
    pub copy: Option<(RecipeNodeId, u32)>,
}

/// The member a mate reference's HEAD names, or the typed
/// dangling-head refusal (N5).
///
/// Structural only — no expression is evaluated here, so the cluster
/// partition never depends on a slot value. A head outside the member
/// vocabulary (a non-instance node; a pattern whose name carries no
/// `Instance(i)` qualifier; a pattern whose input is not itself a live
/// instance — a patterned boolean, a nested pattern) resolves to no
/// member and refuses.
fn head_of<P>(
    doc: &Doc<P>,
    mate: RecipeNodeId,
    side: MateSide,
    name: &crate::names::StableName,
) -> Result<Member, MateFault> {
    let head = name.node;
    match doc.node(head) {
        Some(Node::InstantiatePart { .. }) => Ok(Member {
            instance: head,
            copy: None,
        }),
        Some(Node::Pattern { input, .. }) => match name.path.first() {
            Some(RoleSeg::Instance { i, .. })
                if matches!(doc.node(*input), Some(Node::InstantiatePart { .. })) =>
            {
                Ok(Member {
                    instance: *input,
                    copy: Some((head, *i)),
                })
            }
            _ => Err(MateFault::DanglingHead { mate, side, head }),
        },
        _ => Err(MateFault::DanglingHead { mate, side, head }),
    }
}

/// **The pattern-derived offset** of a pattern-placed member: the
/// rigid map the pattern's evaluation composes onto its input's
/// placement for structural index `i`, derived from the pattern's own
/// authored slots (document coordinates, LEFT-composed — the same
/// derivation the evaluation's stepped rule reads, at the document's
/// own parameter bindings). `None` is the identity: a plain member, or
/// copy 0, whose map is the identity by the rule's own construction —
/// kept as absence so the no-pattern solve composes nothing and stays
/// bit-for-bit what it was.
///
/// The offset is STATIC: nothing here depends on any solved pose,
/// which is how a mate to `Instance(i)` can never create per-instance
/// freedom — the copy rides its master wherever the solve puts it.
///
/// # Errors
///
/// A head whose derived pose does not exist resolves to no member of
/// the vocabulary and refuses [`MateFault::DanglingHead`] — an index
/// at or beyond the count, a rule whose slots do not evaluate, a
/// degenerate direction, an explicit-rule pattern (whose count
/// spelling the pattern node itself refuses). The pattern node's own
/// evaluation names the underlying cause in its own voice; this door's
/// job is only to refuse rather than guess a pose. An in-band
/// direction-norm decision escalates [`MateFault::Indeterminate`], as
/// every decided predicate here does.
fn derived_offset<P>(
    doc: &Doc<P>,
    mate: RecipeNodeId,
    side: MateSide,
    member: Member,
    band: Band,
) -> Result<Option<Affine3<f64>>, Box<MateFault>> {
    let Some((pattern, i)) = member.copy else {
        return Ok(None);
    };
    let dangling = || {
        Box::new(MateFault::DanglingHead {
            mate,
            side,
            head: pattern,
        })
    };
    let Some(Node::Pattern { count, kind, .. }) = doc.node(pattern) else {
        return Err(dangling());
    };
    let env = doc.param_env::<f64>();
    let n = crate::expr::eval_count(count, &env).map_err(|_| dangling())?;
    if i64::from(i) >= n {
        return Err(dangling());
    }
    if i == 0 {
        return Ok(None);
    }
    let scalar = |e: &crate::expr::Expr| crate::expr::eval(e, &env).map_err(|_| dangling());
    let triple = |es: &[crate::expr::Expr; 3]| -> Result<Vec3<f64>, Box<MateFault>> {
        Ok(Vec3::new(scalar(&es[0])?, scalar(&es[1])?, scalar(&es[2])?))
    };
    // The one normalization the stepped rules share, decided — never a
    // raw comparison, never a silent zero direction.
    let unit = |v: Vec3<f64>| -> Result<Vec3<f64>, Box<MateFault>> {
        match geom_core::k_stats::decide(
            "mate_pattern_direction_norm",
            geom_core::predicate::Margin::norm3(v),
            band,
        ) {
            Ok(geom_core::predicate::Sign::Positive) => Ok(v.normalize()),
            Ok(_) => Err(dangling()),
            Err(diag) => Err(Box::new(MateFault::Indeterminate {
                mate,
                diag: Box::new(diag),
            })),
        }
    };
    let step = f64::from(i);
    match kind {
        PatternKind::Linear { direction, spacing } => {
            let dir = unit(triple(direction)?)?;
            let spacing = scalar(spacing)?;
            Ok(Some(Affine3::translation(dir * (spacing * step))))
        }
        PatternKind::Circular { axis, step: angle } => {
            let Some(Node::Datum(Datum::Axis { origin, direction })) = doc.node(*axis) else {
                return Err(dangling());
            };
            let o = triple(origin)?;
            let dir = unit(triple(direction)?)?;
            let angle = scalar(angle)?;
            Ok(Some(Affine3::rotation_about_axis(
                Point3::origin() + o,
                dir,
                angle * step,
            )))
        }
        // The list-rule pattern's count has two spellings, which the
        // pattern node itself refuses; no copy of it has a derived
        // pose to stand a member on.
        PatternKind::Explicit(_) => Err(dangling()),
    }
}

/// **A12's reading edges**, recomputed from the recipe: `(mate, head)`
/// for every live mate reference whose head is a live instance.
///
/// Never stored — the DAG stays the single structure, and a dangling
/// head simply contributes no edge until `Rebind` repairs it (N5).
/// Deterministic order: document order of the mate, then `a` before
/// `b`.
pub fn reading_edges<P>(doc: &Doc<P>) -> Vec<(RecipeNodeId, RecipeNodeId)> {
    let mut out = Vec::new();
    for &id in doc.order() {
        let Some(Node::Mate { a, b, .. }) = doc.node(id) else {
            continue;
        };
        for (side, name) in [(MateSide::A, a), (MateSide::B, b)] {
            if let Ok(member) = head_of(doc, id, side, name) {
                out.push((id, member.instance));
            }
        }
    }
    out
}

/// **A9's relative-freedom partition**: the connected components of the
/// recipe DAG over CONSUMING ∪ READING edges, each component's nodes in
/// document order, the components themselves ordered by their first
/// node.
///
/// Two instances are relatively unconstrained exactly when they land in
/// different components — decidable from recipe structure alone, which
/// is the whole content of A9. Mates couple components precisely
/// because their reading edges count here (A12) even though A10's
/// invariants never see them.
pub fn relative_freedom_components<P>(doc: &Doc<P>) -> Vec<Vec<RecipeNodeId>> {
    let mut adjacency: BTreeMap<RecipeNodeId, BTreeSet<RecipeNodeId>> = BTreeMap::new();
    let mut edges: Vec<(RecipeNodeId, RecipeNodeId)> = Vec::new();
    for &id in doc.order() {
        adjacency.entry(id).or_default();
        if let Some(node) = doc.node(id) {
            edges.extend(node.inputs().into_iter().map(|input| (id, input)));
        }
    }
    edges.extend(reading_edges(doc));
    for (x, y) in edges {
        adjacency.entry(x).or_default().insert(y);
        adjacency.entry(y).or_default().insert(x);
    }
    components(doc.order(), &adjacency)
}

/// Connected components over `adjacency`, seeded in `order` so both the
/// components and their contents are document-ordered.
fn components(
    order: &[RecipeNodeId],
    adjacency: &BTreeMap<RecipeNodeId, BTreeSet<RecipeNodeId>>,
) -> Vec<Vec<RecipeNodeId>> {
    let position: BTreeMap<RecipeNodeId, usize> =
        order.iter().enumerate().map(|(i, &id)| (id, i)).collect();
    let mut seen: BTreeSet<RecipeNodeId> = BTreeSet::new();
    let mut out = Vec::new();
    for &seed in order {
        if !seen.insert(seed) {
            continue;
        }
        let mut members = vec![seed];
        let mut stack = vec![seed];
        while let Some(id) = stack.pop() {
            for &next in adjacency.get(&id).into_iter().flatten() {
                if position.contains_key(&next) && seen.insert(next) {
                    members.push(next);
                    stack.push(next);
                }
            }
        }
        members.sort_by_key(|id| position[id]);
        out.push(members);
    }
    out
}

/// Whether the document contains any live mate — the cheap
/// precondition for every cluster question, since without one the
/// answers are all the singleton ones.
fn has_mates<P>(doc: &Doc<P>) -> bool {
    doc.order()
        .iter()
        .any(|&id| matches!(doc.node(id), Some(Node::Mate { .. })))
}

// ---- A11: placement clusters ----

/// **A11's placement clusters**: the connected components of the
/// instance–mate graph, each in document order, the clusters ordered by
/// their gauge.
///
/// A cluster's GAUGE is its document-order-first instance — a
/// convention, never stored data, which is what makes zero-anchor and
/// multi-anchor states unrepresentable rather than merely refused. A
/// lone instance is the singleton case of the same field, and that is
/// why a mate-less document's registry is bit-identical to the
/// per-instance keying this generalizes.
pub fn clusters<P>(doc: &Doc<P>) -> Vec<Vec<RecipeNodeId>> {
    let instances: Vec<RecipeNodeId> = doc
        .order()
        .iter()
        .copied()
        .filter(|&id| matches!(doc.node(id), Some(Node::InstantiatePart { .. })))
        .collect();
    let mut adjacency: BTreeMap<RecipeNodeId, BTreeSet<RecipeNodeId>> = BTreeMap::new();
    for &id in &instances {
        adjacency.entry(id).or_default();
    }
    for &id in doc.order() {
        if let Some(Node::Mate { a, b, .. }) = doc.node(id)
            && let (Ok(ha), Ok(hb)) = (
                head_of(doc, id, MateSide::A, a),
                head_of(doc, id, MateSide::B, b),
            )
            && ha.instance != hb.instance
        {
            adjacency.entry(ha.instance).or_default().insert(hb.instance);
            adjacency.entry(hb.instance).or_default().insert(ha.instance);
        }
    }
    components(&instances, &adjacency)
}

/// The cluster representative (gauge) that keys `instance`'s placement
/// record, or `instance` itself when it is not a live instance (the
/// total reading a registry lookup wants).
pub fn gauge_of<P>(doc: &Doc<P>, instance: RecipeNodeId) -> RecipeNodeId {
    // A document with no mates has only singleton clusters, so every
    // instance IS its own gauge — stated as a fast path because this
    // is the door every placement lookup goes through, and the walk
    // below would otherwise cost a pass over the recipe for an answer
    // that is structurally fixed.
    if !has_mates(doc) {
        return instance;
    }
    clusters(doc)
        .into_iter()
        .find(|c| c.contains(&instance))
        .and_then(|c| c.first().copied())
        .unwrap_or(instance)
}

// ---- D-4: the per-pair coset solve ----

/// The frame flip that applies an OPPOSED axis sense: the half turn
/// about the mate frame's own local X, which reverses the axis and the
/// handedness of the cross axis while staying proper (det = +1).
fn opposed() -> Affine3<f64> {
    Affine3::from_parts(
        Mat3::from_cols(
            Vec3::new(1.0, 0.0, 0.0),
            Vec3::new(0.0, -1.0, 0.0),
            Vec3::new(0.0, 0.0, -1.0),
        ),
        Vec3::new(0.0, 0.0, 0.0),
    )
}

/// One mate's coset: the relative poses (b's part coordinates into a's)
/// its primitive admits.
///
/// The construction is the same three lines every time — build both
/// mate frames, apply the sense, ride the primitive's own displacement
/// onto the `a` side, and read off the subgroup the primitive leaves.
/// The clocking RIDER is applied here too, because it never stands
/// alone: it modifies its carrier's target frame and cuts its residual.
fn mate_coset(
    mate: RecipeNodeId,
    alignment: &Alignment,
    band: Band,
    tol: Tol,
) -> Result<Coset, Box<MateFault>> {
    let arm = alignment.lever_arm();
    let frame = |side: MateSide, f: &super::MateFrame| {
        f.placement(tol)
            .map_err(|error| Box::new(MateFault::Frame { mate, side, error }))
    };
    let fa = frame(MateSide::A, &alignment.a)?;
    let fb = frame(MateSide::B, &alignment.b)?;
    let fa = match alignment.sense {
        AxisSense::Aligned => fa,
        AxisSense::Opposed => fa * opposed(),
    };
    let local_z = Vec3::new(0.0, 0.0, 1.0);
    let spin = |theta: f64| {
        Affine3::from_parts(
            Mat3::rotation_about(local_z, theta),
            Vec3::new(0.0, 0.0, 0.0),
        )
    };
    let (target, subgroup) = match alignment.primitive {
        MatePrimitive::FrameCoincidence => {
            // The table: on frame-coincidence the clocking rider is
            // redundant-or-contradictory, DECIDED. A coincidence has
            // already pinned the roll, so the only clocking it can
            // agree with is zero.
            if let Some(theta) = alignment.clocking {
                let margin = geom_core::predicate::Margin::levered(theta, arm);
                let sign = geom_core::k_stats::decide("mate_clocking_redundant", margin, band)
                    .map_err(|diag| {
                        Box::new(MateFault::Indeterminate {
                            mate,
                            diag: Box::new(diag),
                        })
                    })?;
                if sign != geom_core::predicate::Sign::Zero {
                    return Err(Box::new(MateFault::Contradictory {
                        held: mate,
                        added: mate,
                        predicate: "mate_clocking_redundant",
                        clash: theta * arm,
                    }));
                }
            }
            (fa, Subgroup::Trivial)
        }
        MatePrimitive::Coaxial => match alignment.clocking {
            // The rider cuts the cylindrical residual to translation
            // along the axis — the table's coaxial+clocking row.
            Some(theta) => {
                let target = fa * spin(theta);
                let direction = target.linear.c2;
                (target, Subgroup::Prismatic { direction })
            }
            None => {
                let point = Point3::origin() + fa.translation;
                let direction = fa.linear.c2;
                (fa, Subgroup::Cylindrical { point, direction })
            }
        },
        MatePrimitive::PlanarRest { offset } => {
            if alignment.clocking.is_some() {
                return Err(Box::new(MateFault::TableLacks {
                    mate,
                    what: "a clocking rider on a planar rest",
                }));
            }
            let target = fa * Affine3::translation(local_z * offset);
            let normal = target.linear.c2;
            (target, Subgroup::Planar { normal })
        }
        MatePrimitive::Clocking => {
            return Err(Box::new(MateFault::TableLacks {
                mate,
                what: "a standalone clocking with no carrying mate",
            }));
        }
    };
    Ok(Coset {
        subgroup,
        representative: target * fb.inverse(),
    })
}

/// The coset of the INVERSE relation: `{ x⁻¹ : x ∈ c }`, written as a
/// left coset again by conjugating the subgroup.
///
/// A mate is authored on an ordered pair; the spanning tree may want it
/// the other way round. Inverting the relation rather than re-reading
/// the alignment keeps ONE construction of a mate's meaning.
fn invert(c: Coset) -> Coset {
    let r = c.representative.inverse();
    let dir = |v: Vec3<f64>| r.linear * v;
    let pt = |p: Point3<f64>| r.transform_point(p);
    let subgroup = match c.subgroup {
        Subgroup::Se3 => Subgroup::Se3,
        Subgroup::Trivial => Subgroup::Trivial,
        Subgroup::Empty => Subgroup::Empty,
        Subgroup::Planar { normal } => Subgroup::Planar {
            normal: dir(normal),
        },
        Subgroup::Prismatic { direction } => Subgroup::Prismatic {
            direction: dir(direction),
        },
        Subgroup::Cylindrical { point, direction } => Subgroup::Cylindrical {
            point: pt(point),
            direction: dir(direction),
        },
        Subgroup::Revolute { point, direction } => Subgroup::Revolute {
            point: pt(point),
            direction: dir(direction),
        },
    };
    Coset {
        subgroup,
        representative: r,
    }
}

/// **The per-pair fold** (A11 rule 1): every mate on the ordered
/// MEMBER pair `(parent, child)`, intersected. The result's
/// representative maps the child MEMBER's part coordinates into the
/// parent MEMBER's — for a pattern-placed member, the coordinates its
/// mate frames are authored in (the master's, which the copy shares
/// key-for-key). The pair's derived offsets are NOT in this coset:
/// they are the pair's static left factor
/// ([`pair_left_factor`]), composed outside the fold, which is what
/// keeps the coset algebra itself unchanged (the rider's rule-1
/// clause).
///
/// # Errors
///
/// The first refusal: a malformed alignment, a table gap, an
/// Indeterminate case split, or the CONTRADICTORY empty intersection —
/// which names both mates, the predicate, and the measured clash.
pub fn fold_pair<P>(
    doc: &Doc<P>,
    parent: Member,
    child: Member,
    mates: &[RecipeNodeId],
    band: Band,
    tol: Tol,
) -> Result<Coset, Box<MateFault>> {
    let mut held = Coset::unconstrained();
    let mut held_mate = None;
    let mut arm = 1.0_f64;
    for &mate in mates {
        let Some(Node::Mate {
            a,
            b,
            class,
            alignment,
        }) = doc.node(mate)
        else {
            continue;
        };
        // The class table is the policy (`super::class_admission`);
        // this door enforces its own half of it and nothing more.
        if super::class_admission(*class) == super::ClassAdmission::NotAdmitted {
            return Err(Box::new(MateFault::ClassNotAdmitted { mate }));
        }
        let ha = head_of(doc, mate, MateSide::A, a).map_err(Box::new)?;
        let hb = head_of(doc, mate, MateSide::B, b).map_err(Box::new)?;
        if ha == hb {
            return Err(Box::new(MateFault::SelfMate {
                mate,
                instance: ha.instance,
            }));
        }
        arm = arm.max(alignment.lever_arm());
        let mut coset = mate_coset(mate, alignment, band, tol)?;
        // The authored order is `a`'s coordinates from `b`'s; the tree
        // may need the other direction.
        if (ha, hb) != (parent, child) {
            coset = invert(coset);
        }
        held = match super::coset::intersect(held, coset, band, arm) {
            Ok(next) => next,
            Err(FoldStop::Indeterminate(diag)) => {
                return Err(Box::new(MateFault::Indeterminate { mate, diag }));
            }
            Err(FoldStop::Clash { predicate, margin }) => {
                return Err(Box::new(MateFault::Contradictory {
                    held: held_mate.unwrap_or(mate),
                    added: mate,
                    predicate,
                    clash: margin,
                }));
            }
        };
        if matches!(held.subgroup, Subgroup::Empty) {
            return Err(Box::new(MateFault::Contradictory {
                held: held_mate.unwrap_or(mate),
                added: mate,
                predicate: "mate_member_empty",
                clash: f64::INFINITY,
            }));
        }
        held_mate.get_or_insert(mate);
    }
    Ok(held)
}

/// **The pair's static left factor**: what conjugating the members'
/// pattern-derived offsets through the cluster's recorded frame
/// contributes to the child's gauge-relative pose,
///
/// ```text
/// rel_child = (F⁻¹ ∘ O_child⁻¹ ∘ O_parent ∘ F) ∘ rel_parent ∘ rep
/// ```
///
/// where `F` is the cluster's recorded frame (the offsets are document
/// -coordinate maps the evaluation composes OUTSIDE the placement, so
/// relative poses must un-wind `F` around them), and `O` is each
/// member's derived offset. `None` when both members are plain — the
/// factor is then the identity BY CONSTRUCTION, not numerically, so a
/// document without pattern members composes nothing and its solve
/// stays bit-for-bit what it was.
///
/// The faults a member's offset can raise are attributed through
/// `mate` — the pair's first mate, whose sides name these members.
fn pair_left_factor<P>(
    doc: &Doc<P>,
    gauge: RecipeNodeId,
    parent: Member,
    child: Member,
    mate: RecipeNodeId,
    band: Band,
) -> Result<Option<Affine3<f64>>, Box<MateFault>> {
    // The authored sides for attribution: whichever of the pair the
    // parent member is, the other is the child.
    let (parent_side, child_side) = match doc.node(mate) {
        Some(Node::Mate { a, .. }) if head_of(doc, mate, MateSide::A, a) == Ok(parent) => {
            (MateSide::A, MateSide::B)
        }
        _ => (MateSide::B, MateSide::A),
    };
    let op = derived_offset(doc, mate, parent_side, parent, band)?;
    let oc = derived_offset(doc, mate, child_side, child, band)?;
    let middle = match (oc, op) {
        (None, None) => return Ok(None),
        (Some(oc), Some(op)) => oc.inverse() * op,
        (Some(oc), None) => oc.inverse(),
        (None, Some(op)) => op,
    };
    let f = doc
        .placements()
        .get(&gauge)
        .copied()
        .unwrap_or_default()
        .affine::<f64>();
    Ok(Some(f.inverse() * middle * f))
}

/// **The document's solve** (D-4 + D-5): every cluster's spanning tree
/// from its gauge, every tree pair folded and required DETERMINED,
/// every other mate recorded DECLARING, and every instance's pose
/// composed outward from the gauge.
///
/// Total by construction — a refusing cluster records its fault against
/// its own mates and instances and leaves every other cluster solved.
pub fn solve_document<P>(doc: &Doc<P>, tol: Tol) -> SolvedPoses {
    let mut out = SolvedPoses::default();
    let band = match Band::linear(tol) {
        Ok(band) => band,
        Err(error) => {
            // No band, no decisions: every mate and instance refuses
            // with the same typed cause rather than any of them
            // guessing.
            let fault = MateFault::Band { error };
            for &id in doc.order() {
                if matches!(
                    doc.node(id),
                    Some(Node::Mate { .. } | Node::InstantiatePart { .. })
                ) {
                    out.faults.insert(id, fault.clone());
                }
            }
            return out;
        }
    };
    // Mates by the unordered MEMBER pair they relate, document order
    // within a pair. The member — not just its instance — is the key:
    // mates to sibling copies of one pattern are DIFFERENT pairs, so
    // the second of them is a non-tree edge (declaring) rather than a
    // fold-mate of the first, which is the rider's loop clause. A
    // self-mate is one MEMBER named on both sides; two distinct copies
    // of one pattern are a pair like any other (their edge just joins
    // no clusters, both ends standing on the same instance).
    let mut by_pair: BTreeMap<(Member, Member), Vec<RecipeNodeId>> = BTreeMap::new();
    let mut broken: Vec<(RecipeNodeId, MateFault)> = Vec::new();
    for &id in doc.order() {
        let Some(Node::Mate { a, b, .. }) = doc.node(id) else {
            continue;
        };
        out.roles.insert(id, MateRole::Declaring);
        match (
            head_of(doc, id, MateSide::A, a),
            head_of(doc, id, MateSide::B, b),
        ) {
            (Ok(ha), Ok(hb)) if ha != hb => {
                by_pair.entry(unordered(ha, hb)).or_default().push(id);
            }
            (Ok(ha), Ok(_)) => broken.push((
                id,
                MateFault::SelfMate {
                    mate: id,
                    instance: ha.instance,
                },
            )),
            (Err(fault), _) | (_, Err(fault)) => broken.push((id, fault)),
        }
    }
    for (mate, fault) in broken {
        out.roles.insert(mate, MateRole::Refused);
        out.faults.insert(mate, fault);
    }
    for cluster in clusters(doc) {
        let Some(&gauge) = cluster.first() else {
            continue;
        };
        match solve_cluster(doc, &cluster, gauge, &by_pair, band, tol) {
            Ok(solved) => {
                for (instance, frame) in solved.relative {
                    out.relative.insert(instance, frame);
                    out.gauge.insert(instance, gauge);
                }
                for (mate, role) in solved.roles {
                    out.roles.insert(mate, role);
                }
            }
            Err(fault) => {
                // The refusal reaches exactly what it stops: every
                // instance in the cluster (which now has no pose) and
                // every mate holding it together.
                for &instance in &cluster {
                    out.gauge.insert(instance, gauge);
                    out.faults
                        .entry(instance)
                        .or_insert_with(|| (*fault).clone());
                }
                for (pair, mates) in &by_pair {
                    if cluster.contains(&pair.0.instance) {
                        for &mate in mates {
                            out.roles.insert(mate, MateRole::Refused);
                            out.faults.entry(mate).or_insert_with(|| (*fault).clone());
                        }
                    }
                }
            }
        }
    }
    out
}

/// One cluster's solved relative poses and mate roles.
struct ClusterSolve {
    relative: BTreeMap<RecipeNodeId, Frame>,
    roles: BTreeMap<RecipeNodeId, MateRole>,
}

fn unordered<T: Ord>(x: T, y: T) -> (T, T) {
    if x <= y { (x, y) } else { (y, x) }
}

/// One cluster: the deterministic spanning tree from the gauge, each
/// tree pair folded and required DETERMINED (A11 rule 4), the poses
/// composed outward (rule 5).
///
/// The tree spans INSTANCES, but its edges are member pairs: between
/// two instances the tree takes the first member pair in key order and
/// every other pair between them — a sibling copy's mate, say — is a
/// non-tree edge and stays DECLARING. A pair standing on one instance
/// twice (two copies of the same pattern mated to each other) can
/// never be a tree edge at all — the pattern already determined both
/// ends — so it stays declaring the same way.
fn solve_cluster<P>(
    doc: &Doc<P>,
    cluster: &[RecipeNodeId],
    gauge: RecipeNodeId,
    by_pair: &BTreeMap<(Member, Member), Vec<RecipeNodeId>>,
    band: Band,
    tol: Tol,
) -> Result<ClusterSolve, Box<MateFault>> {
    let position: BTreeMap<RecipeNodeId, usize> =
        cluster.iter().enumerate().map(|(i, &id)| (id, i)).collect();
    let mut neighbours: BTreeMap<RecipeNodeId, Vec<RecipeNodeId>> = BTreeMap::new();
    // The tree edge between two instances: the FIRST member pair
    // relating them, in pair-key order (deterministic). Every other
    // pair between the same two is a non-tree edge and stays
    // declaring.
    let mut edge_of: BTreeMap<(RecipeNodeId, RecipeNodeId), (Member, Member)> = BTreeMap::new();
    for (&(x, y), _) in by_pair
        .iter()
        .filter(|((x, _), _)| position.contains_key(&x.instance))
    {
        if x.instance == y.instance {
            continue;
        }
        neighbours.entry(x.instance).or_default().push(y.instance);
        neighbours.entry(y.instance).or_default().push(x.instance);
        edge_of
            .entry(unordered(x.instance, y.instance))
            .or_insert((x, y));
    }
    for list in neighbours.values_mut() {
        list.sort_by_key(|id| position.get(id).copied().unwrap_or(usize::MAX));
        list.dedup();
    }
    let mut relative: BTreeMap<RecipeNodeId, Frame> = BTreeMap::new();
    relative.insert(gauge, Frame::IDENTITY);
    let mut poses: BTreeMap<RecipeNodeId, Affine3<f64>> = BTreeMap::new();
    poses.insert(gauge, Affine3::identity());
    // Only THIS cluster's mates get a role here: a role written for
    // another cluster's mate would race that cluster's own answer,
    // and which one won would depend on document order.
    let mut roles: BTreeMap<RecipeNodeId, MateRole> = BTreeMap::new();
    for (pair, mates) in by_pair {
        if position.contains_key(&pair.0.instance) {
            for &mate in mates {
                roles.insert(mate, MateRole::Declaring);
            }
        }
    }
    let mut queue = VecDeque::from([gauge]);
    let mut visited: BTreeSet<RecipeNodeId> = BTreeSet::from([gauge]);
    while let Some(parent) = queue.pop_front() {
        for &child in neighbours.get(&parent).into_iter().flatten() {
            if !visited.insert(child) {
                continue;
            }
            let (x, y) = edge_of[&unordered(parent, child)];
            let (pm, cm) = if x.instance == parent { (x, y) } else { (y, x) };
            let mates = &by_pair[&(x, y)];
            let coset = fold_pair(doc, pm, cm, mates, band, tol)?;
            if !coset.subgroup.is_determined() {
                // A11 rule 4: a tree edge that does not determine
                // refuses, naming the residual and its parameters.
                return Err(Box::new(MateFault::Under {
                    mate: mates[0],
                    parent,
                    child,
                    residual: coset.subgroup,
                }));
            }
            let mut pose = poses[&parent] * coset.representative;
            if let Some(left) = pair_left_factor(doc, gauge, pm, cm, mates[0], band)? {
                pose = left * pose;
            }
            poses.insert(child, pose);
            relative.insert(child, Frame::from_affine(pose));
            for &mate in mates {
                roles.insert(mate, MateRole::Determining);
            }
            queue.push_back(child);
        }
    }
    Ok(ClusterSolve { relative, roles })
}

// ---- D-3: the cluster-record keying maintenance ----

/// One recorded act of cluster-record maintenance (D-3). Each is a
/// consequence of an ordinary recorded edit, carried on that edit's
/// [`crate::EditRecord`]; undo is keeping the prior document value, so
/// every one of them restores exactly.
#[derive(Debug, Clone, PartialEq)]
pub enum ClusterMaintenance {
    /// Two clusters became one. The surviving gauge keeps its frame;
    /// the absorbed cluster's frame is CONSUMED into this record (it
    /// no longer keys anything).
    Join {
        /// The gauge that survived: the earlier of the two.
        survived: RecipeNodeId,
        /// The absorbed cluster's former gauge.
        absorbed: RecipeNodeId,
        /// Its frame, as recorded — `None` when the row was absent
        /// (the identity).
        absorbed_frame: Option<Frame>,
    },
    /// A cluster split off and its new gauge's frame was RE-MINTED
    /// from the solved pose.
    ///
    /// **The claim is GAUGE-exact** (review MINOR-1): the new gauge's
    /// world pose is preserved BIT for bit, and every other member's
    /// is preserved as a value, not as bits. Members are placed by
    /// composition from the gauge, and the split re-associates that
    /// composition — `(F ∘ rel_gauge) ∘ rel_member` where it used to
    /// be `F ∘ (rel_gauge ∘ rel_member)` — so a deep member can move
    /// by a rounding step. Stating this exactly is what makes rows 4b
    /// and 4c assertable: they pin the gauge's bits, which is the
    /// claim, rather than a bit-identity the arithmetic cannot offer.
    Split {
        /// The gauge of the cluster it separated from.
        from: RecipeNodeId,
        /// The new cluster's gauge.
        to: RecipeNodeId,
        /// The minted frame, `None` for the identity.
        frame: Option<Frame>,
    },
    /// The gauge instance died and the key moved to the next
    /// representative, composing with the already-solved relative
    /// pose. GAUGE-exact, on [`ClusterMaintenance::Split`]'s terms and
    /// for its reason.
    GaugeRewrite {
        /// The dead gauge.
        from: RecipeNodeId,
        /// Its successor.
        to: RecipeNodeId,
        /// The rewritten frame, `None` for the identity.
        frame: Option<Frame>,
    },
    /// A cluster's last instance died; its record goes with it.
    Drop {
        /// The dead gauge.
        gauge: RecipeNodeId,
        /// The frame that was recorded, `None` when the row was absent.
        frame: Option<Frame>,
    },
}

/// **The keying maintenance** (D-3): re-key `after`'s placement
/// registry onto its cluster representatives, preserving every
/// surviving cluster's GAUGE world pose BIT for bit (and every other
/// member's as a value — see [`ClusterMaintenance::Split`]), and
/// report what it did.
///
/// The one invariant, from which all four acts follow: *a cluster's
/// frame is the frame that leaves its gauge's world pose where the
/// prior document had it.* When the gauge did not change, that is the
/// prior row VERBATIM — bit-identical, which is what makes a mate-less
/// document's registry unchanged by this machinery existing.
pub(crate) fn reconcile<P>(
    before: &Doc<P>,
    after: &mut Doc<P>,
    tol: Tol,
) -> Vec<ClusterMaintenance> {
    // Neither side has a mate: every cluster is a singleton on both,
    // so the registry is already keyed by its own gauges and the
    // maintenance has nothing to do. The invariant below would compute
    // exactly this, at the cost of a recipe pass per edit.
    if !has_mates(before) && !has_mates(after) {
        return Vec::new();
    }
    let before_clusters = clusters(before);
    let before_gauge: BTreeMap<RecipeNodeId, RecipeNodeId> = before_clusters
        .iter()
        .flat_map(|c| c.iter().map(|&id| (id, c[0])))
        .collect();
    let before_poses = solve_document(before, tol);
    let after_clusters = clusters(after);

    let mut rows: BTreeMap<RecipeNodeId, Frame> = BTreeMap::new();
    let mut acts: Vec<ClusterMaintenance> = Vec::new();
    let mut carried: BTreeSet<RecipeNodeId> = BTreeSet::new();
    for cluster in &after_clusters {
        let gauge = cluster[0];
        // Which prior clusters this one is made of, gauge-ordered.
        let mut sources: BTreeSet<RecipeNodeId> = cluster
            .iter()
            .filter_map(|id| before_gauge.get(id).copied())
            .collect();
        let Some(&old_gauge) = before_gauge.get(&gauge) else {
            // A freshly inserted instance has no prior world pose: the
            // identity is its frame, and an absent row IS the identity.
            continue;
        };
        carried.insert(old_gauge);
        sources.remove(&old_gauge);
        let prior = before.placements().get(&old_gauge).copied();
        // The relative pose of this cluster's gauge under the PRIOR
        // mate graph — the identity when the gauge did not move.
        let relative = before_poses.relative(gauge).unwrap_or_default();
        let frame = match (prior, relative.is_identity_bits()) {
            (row, true) => row,
            (Some(f), false) => Some(f.compose(&relative)),
            (None, false) => Some(relative),
        };
        if let Some(f) = frame {
            rows.insert(gauge, f);
        }
        if old_gauge != gauge {
            let act = if after.node(old_gauge).is_some() {
                ClusterMaintenance::Split {
                    from: old_gauge,
                    to: gauge,
                    frame,
                }
            } else {
                ClusterMaintenance::GaugeRewrite {
                    from: old_gauge,
                    to: gauge,
                    frame,
                }
            };
            acts.push(act);
        }
        for absorbed in sources {
            carried.insert(absorbed);
            acts.push(ClusterMaintenance::Join {
                survived: gauge,
                absorbed,
                absorbed_frame: before.placements().get(&absorbed).copied(),
            });
        }
    }
    for (&gauge, &frame) in before.placements() {
        if !carried.contains(&gauge) && !rows.contains_key(&gauge) {
            acts.push(ClusterMaintenance::Drop {
                gauge,
                frame: Some(frame),
            });
        }
    }
    after.set_placements(rows);
    acts
}
