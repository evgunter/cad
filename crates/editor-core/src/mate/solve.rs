//! **The mate solve** — reading edges, partitions, clusters, and the
//! constructive placement (ASM-R2a D-2/D-3/D-4/D-5; A9/A10/A11/A12).
//!
//! Everything here is recipe data plus decided predicates over the
//! sides' RESOLVED frames — the two reads `ASSEMBLY.md` A11 rule 5
//! states cross through one door, [`MateReach`]: each mated part's
//! extent, the lever, and a `FromFace` frame's pose, resolved before
//! the frame is read ([`resolve_side`]) — and nothing derived is
//! stored beside the DAG. The entry points, in the order the layers
//! use them:
//!
//! - [`reading_edges`] — A12's second sort of edge, RECOMPUTED by
//!   walking from each reference's OPERAND every time it is wanted.
//! - [`relative_freedom_components`] — A9's partition, over consuming
//!   ∪ reading edges (so mates couple components).
//! - [`clusters`] — A11's placement clusters, the finer partition over
//!   instances alone, each with its document-order-first GAUGE.
//! - [`solve_document`] — the per-pair coset fold along a deterministic
//!   spanning tree, yielding every instance's pose relative to its
//!   gauge, and every mate's role.
//! - [`admit_mate`] — one mate's own admission, the per-mate prefix
//!   of the solve asked by the edit door of a mate being inserted.
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
use geom_core::linalg::frame::{FrameError, FrameInput, FrameVector};
use geom_core::linalg::{Affine3, Mat3, Point3, UnitVec3, UnitVec3Error, Vec3};
use geom_core::predicate::Band;

use super::coset::{Coset, FoldStop, Measured, Subgroup};
use super::member::{Member, Walk, check_reference, derived_offset, walk_of};
use super::reach::MateReach;
use super::{
    Alignment, AuthoredFrame, AxisSense, Clash, FaceRefusal, Lever, MateFault, MateFrame,
    MatePrimitive, MateSide,
};
use crate::doc::Doc;
use crate::edit::EditError;
use crate::expr::ParamEnv;
use crate::node::{Node, RecipeNodeId};
use crate::placement::Frame;

/// What a mate did in the solve (A11 rule 4).
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
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
///
/// A solve is a solve OF a document, and it says so: `document` is the
/// id [`solve_document`] read, and [`SolvedPoses::placement`] — the one
/// door that takes a `Doc` back — refuses a mispairing with it (DI3).
/// No `Default`, for that reason: a poses value with no document is a
/// value that cannot answer which document it is about.
#[derive(Debug, Clone)]
pub struct SolvedPoses {
    /// **Which document this is a solve OF** (DI3), stamped by
    /// [`solve_document`].
    document: crate::ident::DocumentId,
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
    /// An empty solve OF `document`: no poses, no roles, no faults.
    fn empty(document: crate::ident::DocumentId) -> Self {
        Self {
            document,
            relative: BTreeMap::new(),
            gauge: BTreeMap::new(),
            roles: BTreeMap::new(),
            faults: BTreeMap::new(),
        }
    }

    /// **Which document this solve is of** (DI3). A caller holding a
    /// solve and a document can check the pairing itself; every door
    /// here that takes a `Doc` checks it already.
    pub fn document(&self) -> crate::ident::DocumentId {
        self.document
    }

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
    /// `doc` is read for its cluster frames, and it must be the
    /// document this solve is OF: composing this document's relative
    /// poses onto another one's recorded frames is a pose of neither.
    /// The pairing is CHECKED here (DI3) — the solve carries the id it
    /// was built from — rather than left to the caller.
    ///
    /// # Errors
    ///
    /// [`MateFault::PosesOfAnotherDocument`] when `doc` is not the
    /// document this solve is of, and the cluster's own refusal when it
    /// did not solve.
    pub fn placement<P>(
        &self,
        doc: &Doc<P>,
        instance: RecipeNodeId,
    ) -> Result<Frame, Box<MateFault>> {
        if let Some(m) = crate::ident::mispaired(doc.id(), self.document) {
            return Err(Box::new(m.into()));
        }
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

/// **A12's reading edges**, recomputed from the recipe:
/// `(mate, instance)` for every mate reference that resolves to a
/// member of the A11 vocabulary. The edge lands on the MEMBER's
/// instance — the one the walk from the operand ends on — which is the
/// vertex the A9/A11 partitions see.
///
/// Never stored — the DAG stays the single structure, and a reference
/// that resolves to no member simply contributes no edge (N5).
/// Deterministic order: document order of the mate, then `a` before
/// `b`.
pub fn reading_edges<P>(doc: &Doc<P>) -> Vec<(RecipeNodeId, RecipeNodeId)> {
    let mut out = Vec::new();
    for &id in doc.order() {
        let Some(Node::Mate { a, b, .. }) = doc.node(id) else {
            continue;
        };
        for (side, name) in [(MateSide::A, a), (MateSide::B, b)] {
            if let Ok(w) = walk_of(doc, id, side, name) {
                out.push((id, w.member.instance));
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
pub fn relative_freedom_components<P: crate::ProfilePayload>(
    doc: &Doc<P>,
) -> Vec<Vec<RecipeNodeId>> {
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
    clusters_welded_by(doc, &welds(&read_mates(doc)))
}

/// One mate's two references as [`read_mates`] read them: both walks,
/// or the first refusal that stops the mate being an edge at all.
type ReadMate = Result<(Walk, Walk), MateFault>;

/// **Which mates WELD, read once**: each live mate in document order
/// with both its references walked, or the first refusal that stops
/// it being an edge at all.
///
/// The one reading [`clusters`] and [`solve_document`] share. They
/// used to ask the same question through two loops written out
/// separately — the same predicate spelled twice, where a change to
/// either could leave the partition the registry is keyed by
/// disagreeing with the partition the solve folds over.
///
/// STRUCTURAL, and that is the point: it walks and nothing more, so
/// the partition never depends on a slot value. The solve's own
/// further checks ([`check_reference`]) can refuse a mate this admits
/// — such a mate welds its cluster and contributes no PAIR, so its
/// instances keep the cluster's frame and no pose is invented for
/// them.
fn read_mates<P>(doc: &Doc<P>) -> Vec<(RecipeNodeId, ReadMate)> {
    let mut out = Vec::new();
    for &id in doc.order() {
        let Some(Node::Mate { a, b, .. }) = doc.node(id) else {
            continue;
        };
        out.push((
            id,
            walk_of(doc, id, MateSide::A, a)
                .and_then(|wa| Ok((wa, walk_of(doc, id, MateSide::B, b)?))),
        ));
    }
    out
}

/// The instance pairs [`read_mates`] welds — its resolving mates,
/// projected onto the vertices A9/A11's partitions see.
fn welds(read: &[(RecipeNodeId, ReadMate)]) -> Vec<(RecipeNodeId, RecipeNodeId)> {
    read.iter()
        .filter_map(|(_, r)| r.as_ref().ok())
        .map(|(wa, wb)| (wa.member.instance, wb.member.instance))
        .collect()
}

/// The clusters a given set of WELDS produces.
///
/// A weld standing on ONE instance (two copies of a pattern mated to
/// each other) joins nothing and is dropped here rather than at each
/// caller, so no caller can forget it.
fn clusters_welded_by<P>(
    doc: &Doc<P>,
    welds: &[(RecipeNodeId, RecipeNodeId)],
) -> Vec<Vec<RecipeNodeId>> {
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
    for &(x, y) in welds {
        if x == y {
            continue;
        }
        adjacency.entry(x).or_default().insert(y);
        adjacency.entry(y).or_default().insert(x);
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

/// **One solve's inputs**, borrowed for its duration and read by every
/// step below `solve_document`: the document, its one nominal
/// environment, the reach the lever is asked through, and the
/// decision band with the tolerance it was derived from. Nothing here
/// outlives the solve and nothing is derived into it — a cache would
/// be a second answer to a question the document already answers.
struct Solve<'a, P> {
    doc: &'a Doc<P>,
    env: &'a ParamEnv<f64>,
    reach: &'a dyn MateReach,
    band: Band,
    tol: Tol,
}

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
///
/// Each side's frame arrives RESOLVED (`a`, `b`: [`resolve_side`] —
/// the authored vectors, or the face's pose read through the reach)
/// and is read ONCE ([`AuthoredFrame::frame`], the witness ladder
/// both arms meet): its affine is the placement and its `w` the axis
/// witness, negated exactly for an opposed sense. Every primitive's
/// target keeps that axis — a standoff translates along it and the
/// rider spins about it — so no direction here is read back off a
/// product of matrices, which would be unit only to rounding and a
/// witness to nothing.
///
/// `lever` forms this mate's lever — the two mated parts' reach
/// summed ([`pair_reach`]) plus the datum's own terms
/// ([`Alignment::lever_arm`]) — and is asked at exactly one site: the
/// rider on a coincidence, the one row of the table that levers a
/// decision. Every other row decides on the datum alone, so a caller
/// with no lever in hand (the edit door, [`admit_mate`]) forms none
/// for them, and a caller that holds one already (the fold, which
/// levers its intersections too) hands it in. Replay never reaches
/// the table: with no reach it declines at [`admit_mate`], on the
/// rule stated at [`Maintain::reach`].
fn mate_coset(
    mate: RecipeNodeId,
    alignment: &Alignment,
    a: &AuthoredFrame,
    b: &AuthoredFrame,
    lever: impl FnOnce() -> Result<f64, Box<MateFault>>,
    band: Band,
    tol: Tol,
) -> Result<Coset, Box<MateFault>> {
    let frame = |side: MateSide, f: &AuthoredFrame| {
        f.frame(tol)
            .map_err(|error| Box::new(MateFault::Frame { mate, side, error }))
    };
    let fa = frame(MateSide::A, a)?;
    let fb = frame(MateSide::B, b)?.to_affine();
    let (fa, axis) = match alignment.sense {
        AxisSense::Aligned => (fa.to_affine(), fa.w()),
        AxisSense::Opposed => (fa.to_affine() * opposed(), -fa.w()),
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
            // The lever is asked HERE and nowhere else in the table:
            // no rider, no ask.
            if let Some(theta) = alignment.clocking {
                let arm = lever()?;
                let roll = Measured::Lever(Lever::Roll {
                    radians: theta,
                    arm,
                });
                let sign =
                    geom_core::k_stats::decide("mate_clocking_redundant", roll.margin(), band)
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
                        clash: roll.clash(),
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
                (target, Subgroup::Prismatic { direction: axis })
            }
            None => {
                let point = Point3::origin() + fa.translation;
                (
                    fa,
                    Subgroup::Cylindrical {
                        point,
                        direction: axis,
                    },
                )
            }
        },
        MatePrimitive::PlanarRest { offset } => {
            // The table's static gap, read where the table keeps it
            // (`super::table_gap`), at this arm so a frame refusal
            // still precedes it.
            if let Some(what) = super::table_gap(alignment.primitive, alignment.clocking) {
                return Err(Box::new(MateFault::TableLacks { mate, what }));
            }
            let target = fa * Affine3::translation(local_z * offset);
            (target, Subgroup::Planar { normal: axis })
        }
        MatePrimitive::Clocking => {
            // The table's other static gap, from the same home
            // (`super::table_gap`), which gaps a standalone clocking
            // for every rider: an answer of `None` here is the table
            // contradicting itself, not a mate the table admits.
            let Some(what) = super::table_gap(alignment.primitive, alignment.clocking) else {
                unreachable!(
                    "table_gap admits a standalone clocking, which the table has no row for"
                )
            };
            return Err(Box::new(MateFault::TableLacks { mate, what }));
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
///
/// The subgroup's directions are transported by the representative's
/// rotation and re-minted under the band ([`derived_direction`]): a
/// proper rotation keeps a witness's length one within rounding, so
/// the mint decides a length within rounding of one and refuses on no
/// document the doors build.
///
/// # Errors
///
/// The frame ladder's own refusal for a transported direction whose
/// length could not be decided ([`derived_direction`]).
fn invert(c: Coset, band: Band) -> Result<Coset, FrameError> {
    let r = c.representative.inverse();
    let dir = |u: UnitVec3<f64>| derived_direction(r.linear * u.get(), "mate_coset_inverse", band);
    let pt = |p: Point3<f64>| r.transform_point(p);
    let subgroup = match c.subgroup {
        Subgroup::Se3 => Subgroup::Se3,
        Subgroup::Trivial => Subgroup::Trivial,
        Subgroup::Empty => Subgroup::Empty,
        Subgroup::Planar { normal } => Subgroup::Planar {
            normal: dir(normal)?,
        },
        Subgroup::Prismatic { direction } => Subgroup::Prismatic {
            direction: dir(direction)?,
        },
        Subgroup::Cylindrical { point, direction } => Subgroup::Cylindrical {
            point: pt(point),
            direction: dir(direction)?,
        },
        Subgroup::Revolute { point, direction } => Subgroup::Revolute {
            point: pt(point),
            direction: dir(direction)?,
        },
    };
    Ok(Coset {
        subgroup,
        representative: r,
    })
}

/// **A direction the fold derives from a witness, re-minted under the
/// run's band**, with the direction door's refusal in the frame
/// ladder's vocabulary — the refusal a mate frame's own axis gets —
/// so a decided ZERO stays a definite refusal and only an in-band
/// length is the escalation: `Degenerate` and an underflowed length
/// are the ladder's `Degenerate` and `UnderflowedLength` at the aim,
/// an in-band length carries its diagnostic, a length that is no
/// number is `NonFiniteLength`. The invariant a caller relies on is
/// that a proper rotation of a witness has length one within
/// rounding, so on a document the doors build the mint never refuses;
/// a refusal is the witness doing its job.
fn derived_direction(
    v: Vec3<f64>,
    site: &'static str,
    band: Band,
) -> Result<UnitVec3<f64>, FrameError> {
    UnitVec3::new(v, site, band).map_err(|error| match error {
        UnitVec3Error::Degenerate => FrameError::Degenerate {
            input: FrameInput::Aim,
            indeterminate: None,
        },
        UnitVec3Error::Escalated(diag) => FrameError::Degenerate {
            input: FrameInput::Aim,
            indeterminate: Some(diag),
        },
        UnitVec3Error::UnderflowedLength => FrameError::UnderflowedLength {
            input: FrameVector::Aim,
        },
        UnitVec3Error::NonFiniteLength => FrameError::NonFiniteLength {
            input: FrameVector::Aim,
        },
    })
}

/// **The part a member stands on**: its instance's reference, or the
/// node when it is not a live instantiate node — which the member walk
/// excludes and the two readers ([`pair_reach`], [`resolve_side`])
/// still name rather than assume.
fn part_of<P: crate::ProfilePayload>(
    doc: &Doc<P>,
    member: &Member,
) -> Result<crate::ident::DocRef, RecipeNodeId> {
    match doc.node(member.instance) {
        Some(Node::InstantiatePart { doc_ref, .. }) => Ok(*doc_ref),
        _ => Err(member.instance),
    }
}

/// **One side's frame as the solve reads it** — the two arms of
/// [`MateFrame`] meeting at one [`AuthoredFrame`], which the coset
/// table then reads through the frame witness ([`mate_coset`]).
///
/// An `Authored` frame is its own vectors. A `FromFace` frame is the
/// named face's canonical pose, asked of the member's part through
/// the reach ([`MateReach::face_pose`]) in the part's own coordinates
/// — the same coordinates the authored vectors are written in, so no
/// placement enters — with the pose's origin and CHART axis (the
/// orientation sense is not folded in; the mate's own
/// [`AxisSense`] says which way the sides point) and, for the roll,
/// the pose's own in-frame reference where the carrier fixes one,
/// else the frame's authored reference. One reference, from one
/// source: both present refuses [`FaceRefusal::ReferenceRefused`],
/// neither refuses [`FaceRefusal::NoReference`].
///
/// Asked before the coset table reads the frame and before the lever
/// is formed (the datum's `‖origin‖` terms are the RESOLVED origins),
/// and asked ONCE per side per read: the door asks it of a mate being
/// inserted, the fold of every mate it folds.
///
/// # Errors
///
/// [`MateFault::FaceUnresolved`] naming the mate, the side, the
/// instance and the part, carrying the reach's refusal in its own
/// voice.
fn resolve_side<P: crate::ProfilePayload>(
    doc: &Doc<P>,
    reach: &dyn MateReach,
    mate: RecipeNodeId,
    side: MateSide,
    member: &Member,
    frame: &MateFrame,
) -> Result<AuthoredFrame, Box<MateFault>> {
    let face = match frame {
        MateFrame::Authored(authored) => return Ok(*authored),
        MateFrame::FromFace(face) => face,
    };
    let unresolved = |refusal| Box::new(MateFault::FaceUnresolved {
        mate,
        side,
        refusal,
    });
    let part = part_of(doc, member)
        .map_err(|node| unresolved(FaceRefusal::NotAnInstance { node }))?;
    let named = |refusal| unresolved(FaceRefusal::of(refusal, member.instance, part, face.face.clone()));
    let pose = reach.face_pose(&part, &face.face).map_err(named)?;
    let reference = match (pose.u_ref, face.reference) {
        (Some(u_ref), None) => [u_ref.x, u_ref.y, u_ref.z],
        (None, Some(authored)) => authored,
        (Some(_), Some(_)) => return Err(named(super::FacePoseRefusal::ReferenceRefused)),
        (None, None) => return Err(named(super::FacePoseRefusal::NoReference)),
    };
    Ok(AuthoredFrame {
        origin: [pose.origin.x, pose.origin.y, pose.origin.z],
        axis: [pose.axis.x, pose.axis.y, pose.axis.z],
        reference,
    })
}

/// **The per-reference prefix of a mate's admission**, after its two
/// walks: the checks that need a number ([`check_reference`]) on both
/// sides, then the pair as two DISTINCT members
/// ([`MateFault::SelfMate`]) — the one function the solve's first
/// loop and [`admit_mate`] both call, so the two cannot meet these
/// refusals in different orders. The walks themselves stay
/// [`walk_of`]'s, made before this is asked: the loop needs them for
/// the welds whether or not these checks admit the mate. Two COPIES
/// of one pattern are two members and pass: what a mate cannot relate
/// is a member to itself.
fn check_references<P: crate::ProfilePayload>(
    doc: &Doc<P>,
    env: &ParamEnv<f64>,
    mate: RecipeNodeId,
    wa: &Walk,
    wb: &Walk,
) -> Result<(), MateFault> {
    check_reference(doc, env, mate, MateSide::A, wa)?;
    check_reference(doc, env, mate, MateSide::B, wb)?;
    if wa.member == wb.member {
        return Err(MateFault::SelfMate {
            mate,
            instance: wa.member.instance,
        });
    }
    Ok(())
}

/// **The class door of a mate's own admission**: the class inside the
/// vocabulary ([`MateFault::ClassNotAdmitted`]). The class table is
/// the policy (`super::class_admission`); this door enforces its own
/// half of it and nothing more.
fn admit_class(mate: RecipeNodeId, class: super::ContactClass) -> Result<(), Box<MateFault>> {
    if super::class_admission(class) == super::ClassAdmission::NotAdmitted {
        return Err(Box::new(MateFault::ClassNotAdmitted { mate }));
    }
    Ok(())
}

/// **One mate's own admission** — what the solve decides about a mate
/// from its own datum alone, asked of that one mate (`node`, the
/// `Node::Mate` the document holds or is about to hold at `mate`): the
/// two walks ([`walk_of`]), the per-reference prefix
/// ([`check_references`]), the class door ([`admit_class`]), each
/// side's frame resolved ([`resolve_side`]) and the coset table
/// ([`mate_coset`]), in the order the solve meets them, with the
/// reach asked exactly where the solve asks it: a `FromFace` side's
/// pose, once per such side, and the lever where the table levers a
/// decision (the rider on a coincidence) and nowhere else.
/// It is the per-mate prefix of the solve: [`solve_with_env`]'s first
/// loop makes the walks and asks [`check_references`], and
/// [`fold_pair`] asks [`admit_class`] and [`mate_coset`] of every
/// mate it folds, adding only what a PAIR needs — the lever formed
/// for every mate, because the fold levers its intersections too.
///
/// The edit door asks this of a mate being inserted, so a mate the
/// solve refuses on its own datum is refused at the insert door
/// (`ASSEMBLY.md` A11 rule 1). The solve records the same fault
/// against the mate whenever it reads the datum; a mate on a pair the
/// fold never reads — two members over one instance — is refused on
/// the datum alone all the same, since which pairs the fold reads is
/// a cluster fact this door does not decide. It folds nothing and
/// reads no other mate: the relational verdicts — UNDER, a
/// contradiction against ANOTHER mate, an escalation on a fold — are
/// the solve's, because they are facts about a pair, not about a
/// mate. A rider the band decides redundant is admitted, as the solve
/// admits it. What the door does not cover is a state: a mate that
/// COMES to carry one of these faults after insert — a head a rebind
/// or a shrunk pattern strands, a `Part` re-pointed, a snapshot loaded
/// from a doctored or older file — is the solve's at evaluation.
///
/// `env` is the document's own nominal environment, built by the
/// door that asks (the evaluation's arrangement at [`solve_with_env`]:
/// one build per entry, every reader handed it). `reach` absent is
/// replay's: a `FromFace` side is then not resolved and the rider not
/// re-decided — the rules and their reason are stated once, at
/// [`Maintain::reach`].
///
/// # Errors
///
/// The fault the solve records against the mate for its own datum,
/// unaltered: [`MateFault::Band`] when no band forms; the walk's
/// [`MateFault::DanglingHead`]; [`check_references`]'s; the class
/// door's; [`resolve_side`]'s [`MateFault::FaceUnresolved`]; and
/// [`mate_coset`]'s — `Frame`, `TableLacks`, the decided
/// contradictory rider or its escalation, and
/// [`MateFault::Unleverable`] where the rider needs a lever the reach
/// cannot form.
pub(crate) fn admit_mate<P: crate::ProfilePayload>(
    doc: &Doc<P>,
    mate: RecipeNodeId,
    node: &Node<P>,
    env: &ParamEnv<f64>,
    reach: Option<&dyn MateReach>,
    tol: Tol,
) -> Result<(), Box<MateFault>> {
    // The door asks this of the mate it is inserting, under the id it
    // minted for it; any other node here is the caller's mistake, not
    // a refusal the mate earned.
    let Node::Mate {
        a,
        b,
        class,
        alignment,
    } = node
    else {
        unreachable!("admit_mate is asked of a mate; node {} is not one", mate.0)
    };
    let band = Band::linear(tol).map_err(|error| Box::new(MateFault::Band { error }))?;
    let wa = walk_of(doc, mate, MateSide::A, a).map_err(Box::new)?;
    let wb = walk_of(doc, mate, MateSide::B, b).map_err(Box::new)?;
    check_references(doc, env, mate, &wa, &wb).map_err(Box::new)?;
    admit_class(mate, *class)?;
    // The two parts are asked in DOCUMENT order, which is the order
    // the fold asks a pair that is a cluster of its own: its gauge is
    // the earlier instance and the tree's parent, so a refusal that
    // names the first part not in hand names the same part here.
    let (first, second) = if wa.member.instance <= wb.member.instance {
        (&wa.member, &wb.member)
    } else {
        (&wb.member, &wa.member)
    };
    let Some(reach) = reach else {
        // Replay: a `FromFace` side is DECLINED, not resolved (the
        // rule at `Maintain::reach`), so the coset cannot be read.
        // What the datum alone decides is decided again — each
        // authored side's frame ladder, then the table's static gaps
        // in the order the table meets them (a frame refusal first).
        for (side, frame) in [(MateSide::A, &alignment.a), (MateSide::B, &alignment.b)] {
            if let Some(authored) = frame.authored_vectors() {
                authored
                    .frame(tol)
                    .map_err(|error| Box::new(MateFault::Frame { mate, side, error }))?;
            }
        }
        if let Some(what) = super::table_gap(alignment.primitive, alignment.clocking) {
            return Err(Box::new(MateFault::TableLacks { mate, what }));
        }
        return Ok(());
    };
    let a = resolve_side(doc, reach, mate, MateSide::A, &wa.member, &alignment.a)?;
    let b = resolve_side(doc, reach, mate, MateSide::B, &wb.member, &alignment.b)?;
    let lever = || {
        pair_reach(doc, reach, first, second)
            .map(|parts| parts + alignment.lever_arm(&a, &b))
            .map_err(|refusal| Box::new(MateFault::Unleverable { mate, refusal }))
    };
    mate_coset(mate, alignment, &a, &b, lever, band, tol).map(|_| ())
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
/// Each mate passes its own admission first — the class door, each
/// side's frame resolved, and the coset table, the doors
/// [`admit_mate`] asks of a mate alone — and only then meets the fold.
///
/// # Errors
///
/// The first refusal: a malformed alignment, a table gap, an
/// Indeterminate case split, or the CONTRADICTORY empty intersection —
/// which names both mates, the predicate, and the measured clash.
fn fold_pair<P: crate::ProfilePayload>(
    s: &Solve<'_, P>,
    parent: &Member,
    child: &Member,
    mates: &[PairMate],
) -> Result<Coset, Box<MateFault>> {
    let Solve {
        doc,
        reach,
        band,
        tol,
        ..
    } = *s;
    let mut held = Coset::unconstrained();
    let mut held_mate = None;
    // The fold's lever is the largest of the mates' own: each is the
    // pair's two part reaches plus that mate's datum terms, so it
    // starts at nothing and no constant seeds it. The reaches are
    // asked ONCE per pair, lazily, at the first mate that forms a
    // lever — after that mate's own class and self-mate checks, so a
    // part that does not resolve never pre-empts a refusal the mate
    // earns on its own.
    let mut arm = 0.0_f64;
    let mut parts_reach: Option<f64> = None;
    for pm in mates {
        let mate = pm.mate;
        let Some(Node::Mate {
            class, alignment, ..
        }) = doc.node(mate)
        else {
            continue;
        };
        // The members these two references resolved to, walked once
        // where the pair map was built and carried here.
        let (ha, hb) = (&pm.a.member, &pm.b.member);
        admit_class(mate, *class)?;
        // The sides' frames, resolved before the lever they enter.
        let a = resolve_side(doc, reach, mate, MateSide::A, ha, &alignment.a)?;
        let b = resolve_side(doc, reach, mate, MateSide::B, hb, &alignment.b)?;
        let parts = match parts_reach {
            Some(parts) => parts,
            None => {
                let parts = pair_reach(doc, reach, parent, child)
                    .map_err(|refusal| Box::new(MateFault::Unleverable { mate, refusal }))?;
                parts_reach = Some(parts);
                parts
            }
        };
        // This mate's lever, formed once: the pair's parts plus its
        // own datum terms. The fold's is the largest so far.
        let mate_arm = parts + alignment.lever_arm(&a, &b);
        arm = arm.max(mate_arm);
        let mut coset = mate_coset(
            mate,
            alignment,
            &a,
            &b,
            || Ok(mate_arm),
            band,
            tol,
        )?;
        // The authored order is `a`'s coordinates from `b`'s; the tree
        // may need the other direction.
        // The transported direction is `a`'s axis carried into `b`'s
        // coordinates, so a refusal is reported at side `a`.
        if (ha, hb) != (parent, child) {
            coset = invert(coset, band).map_err(|error| {
                Box::new(MateFault::Frame {
                    mate,
                    side: MateSide::A,
                    error,
                })
            })?;
        }
        held = match super::coset::intersect(held, coset, band, arm) {
            Ok(next) => next,
            Err(FoldStop::Indeterminate(diag)) => {
                return Err(Box::new(MateFault::Indeterminate { mate, diag }));
            }
            Err(FoldStop::Clash { predicate, clash }) => {
                return Err(Box::new(MateFault::Contradictory {
                    held: held_mate.unwrap_or(mate),
                    added: mate,
                    predicate,
                    clash,
                }));
            }
        };
        if matches!(held.subgroup, Subgroup::Empty) {
            return Err(Box::new(MateFault::Contradictory {
                held: held_mate.unwrap_or(mate),
                added: mate,
                predicate: super::MATE_MEMBER_EMPTY,
                clash: Clash::Structural,
            }));
        }
        held_mate.get_or_insert(mate);
    }
    Ok(held)
}

/// **The two mated parts' reach, summed** — the body terms of the
/// pair's lever ([`Alignment::lever_arm`] states the whole sum). Each
/// member's part is the one its instance stands on: a pattern copy or
/// a transform on the chain moves the part rigidly and changes no
/// reach, so the member's chain is not consulted.
///
/// # Errors
///
/// The first part whose reach is not in hand, in pair order — or a
/// member standing on a node that is not a live instantiate node,
/// which the member walk excludes and this door still names rather
/// than assumes.
fn pair_reach<P: crate::ProfilePayload>(
    doc: &Doc<P>,
    reach: &dyn MateReach,
    parent: &Member,
    child: &Member,
) -> Result<f64, super::LeverRefusal> {
    let of = |member: &Member| {
        let doc_ref =
            part_of(doc, member).map_err(|node| super::LeverRefusal::NotAnInstance { node })?;
        reach
            .reach(&doc_ref)
            .map_err(|refusal| super::LeverRefusal::of(refusal, member.instance, doc_ref))
    };
    Ok(of(parent)? + of(child)?)
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
/// reference's derived offset. `None` when neither reference passes a
/// placer — the factor is then the identity BY CONSTRUCTION, not
/// numerically, so a document with no transform and no pattern between
/// its mates and their instances composes nothing and its solve stays
/// bit-for-bit what it was.
///
/// The faults a reference's offset can raise are attributed through
/// `mate` — the pair's first mate, whose sides name these members.
fn pair_left_factor<P: crate::ProfilePayload>(
    s: &Solve<'_, P>,
    gauge: RecipeNodeId,
    parent: &Member,
    first: &PairMate,
) -> Result<Option<Affine3<f64>>, Box<MateFault>> {
    // The authored sides for attribution: whichever of the pair the
    // parent member is, the other is the child. The pair's mates all
    // relate these two members, so the FIRST mate's own two
    // references are the ones the offsets are derived from — and its
    // two walks are in hand, so neither is walked a second time.
    let mate = first.mate;
    let ((parent_side, parent_walk), (child_side, child_walk)) = if first.a.member == *parent {
        ((MateSide::A, &first.a), (MateSide::B, &first.b))
    } else {
        ((MateSide::B, &first.b), (MateSide::A, &first.a))
    };
    let Solve { doc, env, band, .. } = *s;
    let op = derived_offset(doc, env, mate, parent_side, parent_walk, band)?;
    let oc = derived_offset(doc, env, mate, child_side, child_walk, band)?;
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
///
/// `reach` is the door the solve's two geometric reads cross: each
/// mated part's own extent, asked lazily per pair and entering only as
/// the lever a parallelism verdict is decided over, and each
/// `FromFace` side's pose, asked once per side where the frame is
/// read. The evaluation hands its own part cache (`eval::mate_reach`
/// is the door every other caller builds one through), so a mated
/// part is evaluated exactly once and the instantiate node hits the
/// cache afterwards.
///
/// **One nominal environment per solve.** Every number the solve
/// reads out of the recipe — a pattern's count, a `Part`'s index, the
/// slots a derived offset is composed from — is evaluated at the
/// document's own parameter bindings, under no box and no seed: the
/// solve is a fact about the document, not about any run over it.
/// That environment is built here, once, and handed to every reader
/// (`check_reference`, `derived_offset` and what they call) as a
/// parameter, so "the document's own" is decided at one site and the
/// readers cannot be given different ones. An evaluation, which has
/// already built that same environment for its own f64-pinned
/// readers, hands it in through [`solve_with_env`] instead of paying
/// for a second.
pub fn solve_document<P: crate::ProfilePayload>(
    doc: &Doc<P>,
    reach: &dyn MateReach,
    tol: Tol,
) -> SolvedPoses {
    let env = doc.param_env::<f64>();
    solve_with_env(doc, &env, reach, tol)
}

/// [`solve_document`] over an environment the caller already holds —
/// the evaluation's `LaneEnv::nominal`, which is the document's own
/// by that field's contract. `env` must be that environment: the
/// solve answers about the document, and a boxed or seeded one would
/// make it answer about a run.
pub(crate) fn solve_with_env<P: crate::ProfilePayload>(
    doc: &Doc<P>,
    env: &ParamEnv<f64>,
    reach: &dyn MateReach,
    tol: Tol,
) -> SolvedPoses {
    let mut out = SolvedPoses::empty(doc.id());
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
    let s = Solve {
        doc,
        env,
        reach,
        band,
        tol,
    };
    // Mates by the unordered MEMBER pair they relate, document order
    // within a pair. The member — not just its instance — is the key:
    // mates to sibling copies of one pattern are DIFFERENT pairs, so
    // the second of them is a non-tree edge (declaring) rather than a
    // fold-mate of the first, which is the rider's loop clause. A
    // self-mate is one MEMBER named on both sides; two distinct copies
    // of one pattern are a pair like any other (their edge just joins
    // no clusters, both ends standing on the same instance).
    let mut by_pair: BTreeMap<(Member, Member), Vec<PairMate>> = BTreeMap::new();
    let mut broken: Vec<(RecipeNodeId, MateFault)> = Vec::new();
    // ONE walk per reference, here: the members below, the pair
    // keying, the cluster welds and every derived offset the fold
    // needs are all read off these two walks.
    let read = read_mates(doc);
    for (id, walked) in &read {
        let id = *id;
        out.roles.insert(id, MateRole::Declaring);
        let (wa, wb) = match walked {
            Ok(pair) => pair,
            Err(fault) => {
                broken.push((id, fault.clone()));
                continue;
            }
        };
        // **The checks that need a number, at the site the solve
        // reads the reference** — for EVERY reference of EVERY live
        // mate, not only the ones a tree edge's offset happens to
        // derive — and the pair as two distinct members: the same
        // prefix the edit door asks (`check_references`). The walk
        // itself evaluated nothing, so this is where the name meets a
        // count.
        if let Err(fault) = check_references(doc, env, id, wa, wb) {
            broken.push((id, fault));
            continue;
        }
        let (ha, hb) = (wa.member.clone(), wb.member.clone());
        by_pair
            .entry(unordered(ha, hb))
            .or_default()
            .push(PairMate {
                mate: id,
                a: wa.clone(),
                b: wb.clone(),
            });
    }
    for (mate, fault) in broken {
        out.roles.insert(mate, MateRole::Refused);
        out.faults.insert(mate, fault);
    }
    for cluster in clusters_welded_by(doc, &welds(&read)) {
        let Some(&gauge) = cluster.first() else {
            continue;
        };
        match solve_cluster(&s, &cluster, gauge, &by_pair) {
            Ok(solved) => {
                // The GAUGE is the cluster's, and every instance in it
                // is keyed by that gauge whether or not a pair placed
                // it. A mate this solve refused still WELDS its
                // cluster (the partition is structural), so an
                // instance the spanning tree could not reach keeps the
                // cluster's recorded frame instead of falling back to
                // an identity that would move it.
                for &instance in &cluster {
                    out.gauge.insert(instance, gauge);
                }
                for (instance, frame) in solved.relative {
                    out.relative.insert(instance, frame);
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
                        for pm in mates {
                            out.roles.insert(pm.mate, MateRole::Refused);
                            out.faults
                                .entry(pm.mate)
                                .or_insert_with(|| (*fault).clone());
                        }
                    }
                }
            }
        }
    }
    out
}

/// **One mate on a member pair, with both its references' walks.**
///
/// The walks are the solve's one reading of those two references: the
/// members they resolved to key the pair, and the chains they carry
/// are what [`pair_left_factor`] folds — so nothing below re-walks a
/// reference the pair map already resolved.
struct PairMate {
    /// The mate node.
    mate: RecipeNodeId,
    /// The `a` side's walk, as authored.
    a: Walk,
    /// The `b` side's walk, as authored.
    b: Walk,
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
fn solve_cluster<P: crate::ProfilePayload>(
    s: &Solve<'_, P>,
    cluster: &[RecipeNodeId],
    gauge: RecipeNodeId,
    by_pair: &BTreeMap<(Member, Member), Vec<PairMate>>,
) -> Result<ClusterSolve, Box<MateFault>> {
    let position: BTreeMap<RecipeNodeId, usize> =
        cluster.iter().enumerate().map(|(i, &id)| (id, i)).collect();
    let mut neighbours: BTreeMap<RecipeNodeId, Vec<RecipeNodeId>> = BTreeMap::new();
    // The tree edge between two instances: the FIRST member pair
    // relating them, in pair-key order (deterministic). Every other
    // pair between the same two is a non-tree edge and stays
    // declaring.
    let mut edge_of: BTreeMap<(RecipeNodeId, RecipeNodeId), (&Member, &Member)> = BTreeMap::new();
    for ((x, y), _) in by_pair
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
            for pm in mates {
                roles.insert(pm.mate, MateRole::Declaring);
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
            let mates = &by_pair[&(x.clone(), y.clone())];
            let coset = fold_pair(s, pm, cm, mates)?;
            if !coset.subgroup.is_determined() {
                // A11 rule 4: a tree edge that does not determine
                // refuses, naming the residual and its parameters.
                return Err(Box::new(MateFault::Under {
                    mate: mates[0].mate,
                    parent,
                    child,
                    residual: coset.subgroup,
                }));
            }
            let mut pose = poses[&parent] * coset.representative;
            if let Some(left) = pair_left_factor(s, gauge, pm, &mates[0])? {
                pose = left * pose;
            }
            poses.insert(child, pose);
            relative.insert(child, Frame::from_affine(pose));
            for pm in mates {
                roles.insert(pm.mate, MateRole::Determining);
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
///
/// On the wire, beside the edit that performed it (`edit::LoggedEdit`):
/// replay re-applies these rows to the registry rather than solving
/// again, so the log carries what the maintenance decided.
#[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize)]
#[serde(deny_unknown_fields)]
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

impl ClusterMaintenance {
    /// The frame the row carries, if any: an absorbed cluster's, a
    /// minted or rewritten gauge's, a dropped gauge's.
    pub fn frame(&self) -> Option<&Frame> {
        match self {
            Self::Join { absorbed_frame, .. } => absorbed_frame.as_ref(),
            Self::Split { frame, .. }
            | Self::GaugeRewrite { frame, .. }
            | Self::Drop { frame, .. } => frame.as_ref(),
        }
    }
}

impl core::fmt::Display for ClusterMaintenance {
    /// Each act in prose, F6-shaped. It lives beside the enum because
    /// the sentence is about the mate graph's motion, which is what
    /// this module knows; [`crate::Maintenance`] delegates here for
    /// its cluster arm rather than keeping a second copy of these
    /// four sentences in the edit vocabulary.
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        match self {
            Self::Join {
                survived, absorbed, ..
            } => write!(
                f,
                "the cluster gauged by node {} was absorbed into the one gauged by node {}",
                absorbed.0, survived.0
            ),
            Self::Split { from, to, .. } => write!(
                f,
                "a cluster separated from the one gauged by node {} and is now gauged by node {}",
                from.0, to.0
            ),
            Self::GaugeRewrite { from, to, .. } => write!(
                f,
                "the cluster gauged by node {} lost that instance and is now gauged by node {}",
                from.0, to.0
            ),
            Self::Drop { gauge, .. } => write!(
                f,
                "the cluster gauged by node {} lost its last instance, and its placement record \
                 went with it",
                gauge.0
            ),
        }
    }
}

/// **Where the maintenance's rows come from** for one edit: derived
/// from the edit's motion of the mate graph with a solved frame
/// obtained one of two ways when a row needs one (a cluster whose
/// gauge moved; every other row is structural), or replayed verbatim
/// from the log.
#[derive(Clone, Copy)]
pub(crate) enum Maintain<'a> {
    /// The live edit door: solve the PRIOR document through this
    /// reach, once, the first time a row needs it.
    Solve(&'a dyn MateReach),
    /// Replay of a logged edit that recorded no rows: a row that needs
    /// a solved frame refuses, because replay never solves.
    Never,
    /// Replay of a logged edit that recorded rows: they are what the
    /// maintenance decided, re-applied verbatim, and nothing is
    /// derived.
    Recorded(&'a [ClusterMaintenance]),
}

impl<'a> Maintain<'a> {
    /// The reach a decision at this door levers through: the live
    /// door's, and none under either replay arm.
    ///
    /// **The replay rules, and why they differ.** The per-mate
    /// admission ([`admit_mate`]) with no reach DECLINES what needs
    /// the parts — the rider on a coincidence, decided over a lever,
    /// and a `FromFace` side's frame, resolved from the part's own
    /// face ([`resolve_side`]) — under `Never` and `Recorded` alike,
    /// because the door that recorded the entry decided them over the
    /// parts it had in hand, and re-deciding here would need a store
    /// replay never holds; everything decided on the datum alone is
    /// decided again (each authored side's frame ladder, the table's
    /// static gaps). A `FromFace` side declined is a face not read:
    /// the name is the datum, and the next solve resolves it. The
    /// maintenance under `Never` REFUSES where a row needs a solved
    /// frame ([`EditError::MaintenanceUnrecorded`]), because a frame
    /// nothing decided cannot be recorded; under `Recorded` it
    /// re-applies the rows the recording door minted. A decision
    /// declined leaves nothing false in the document — the datum is
    /// what it was and the next solve decides it again; a frame
    /// invented would.
    pub(crate) fn reach(&self) -> Option<&'a dyn MateReach> {
        match self {
            Self::Solve(reach) => Some(*reach),
            Self::Never | Self::Recorded(_) => None,
        }
    }
}

/// **The maintenance for one accepted edit** — [`reconcile`] deriving
/// the rows, or the recorded rows re-applied — leaving `after`'s
/// registry keyed on its clusters and answering the rows that got it
/// there. The one dispatch over [`Maintain`].
///
/// # Errors
///
/// [`reconcile`]'s.
pub(crate) fn maintain<P: crate::ProfilePayload>(
    before: &Doc<P>,
    after: &mut Doc<P>,
    tol: Tol,
    how: Maintain<'_>,
) -> Result<Vec<ClusterMaintenance>, EditError> {
    match how {
        Maintain::Recorded(rows) => {
            after.set_placements(registry_after(before.placements(), rows));
            Ok(rows.to_vec())
        }
        Maintain::Solve(reach) => reconcile(before, after, tol, Some(reach)),
        Maintain::Never => reconcile(before, after, tol, None),
    }
}

/// **The registry after the maintenance's acts**: the prior registry
/// with every act applied in order — a surviving gauge keeps its row
/// verbatim, a `Join` drops the absorbed row, a `Split` writes the new
/// gauge's minted frame, a `GaugeRewrite` moves the key, a `Drop`
/// removes the row. The live door and every replay door build the
/// registry through this one fold, from the acts alone, so a replayed
/// log reproduces the live registry bit for bit (D9) without a solve.
pub(crate) fn registry_after(
    before: &BTreeMap<RecipeNodeId, Frame>,
    acts: &[ClusterMaintenance],
) -> BTreeMap<RecipeNodeId, Frame> {
    let mut rows = before.clone();
    let set =
        |rows: &mut BTreeMap<RecipeNodeId, Frame>, key: RecipeNodeId, frame: Option<Frame>| {
            match frame {
                Some(f) => {
                    rows.insert(key, f);
                }
                None => {
                    rows.remove(&key);
                }
            }
        };
    for act in acts {
        match act {
            ClusterMaintenance::Join { absorbed, .. } => {
                rows.remove(absorbed);
            }
            ClusterMaintenance::Split { to, frame, .. } => set(&mut rows, *to, *frame),
            ClusterMaintenance::GaugeRewrite { from, to, frame } => {
                rows.remove(from);
                set(&mut rows, *to, *frame);
            }
            ClusterMaintenance::Drop { gauge, .. } => {
                rows.remove(gauge);
            }
        }
    }
    rows
}

/// The fault the prior solve recorded that explains a gauge with no
/// pose: the gauge's OWN — a cluster's refusal reaches every member
/// the solve could not pose, so a gauge with no pose and no fault is
/// a state the solve's invariants exclude, and `None` reports it
/// rather than borrowing another cluster's fault (which could be a
/// decided one, and would let the door proceed to write a frame).
fn unsolved_because(poses: &SolvedPoses, gauge: RecipeNodeId) -> Option<Box<MateFault>> {
    poses.fault(gauge).cloned().map(Box::new)
}

/// **Whether a fault means the solve reached NO verdict** about the
/// cluster — as against deciding, from the document's own content,
/// that the cluster has no pose.
///
/// The maintenance's invariant is that a cluster's frame leaves its
/// gauge's world pose where the prior document had it. When the prior
/// solve DECIDED the cluster has no pose (a contradictory or
/// under-determined fold, a table gap, a self-mate, a dangling or
/// mis-selected head, a malformed frame, a class the solve does not
/// admit), there is no pose to preserve and the members' own frames
/// were consumed when they joined: the orphan keeps the cluster's
/// frame, which is what deleting the offending mate — the recourse
/// every such refusal names — has always done. When the solve could
/// NOT decide — no band, an in-band case split, a part whose extent
/// or resolution is not in hand, a placer whose pose could not be
/// derived, a read mispaired against another document's solve — a
/// pose may well exist and nothing here knows it, so the edit refuses
/// rather than record a frame nothing decided.
fn undecided(fault: &MateFault) -> bool {
    match fault {
        MateFault::Band { .. }
        | MateFault::Indeterminate { .. }
        | MateFault::Unleverable { .. }
        | MateFault::PlacerRefused { .. }
        // A caller's mispairing is no verdict about the document.
        | MateFault::PosesOfAnotherDocument { .. } => true,
        // A face frame that did not resolve: the part not in hand, a
        // product whose scalar pins nothing, or a body whose table
        // names a key it lacks — nothing here knows the pose; a name
        // the part's table has no row for or ties, a carrier with no
        // canonical frame, a reference missing or spelled twice, a
        // member on no instance — the document's own content decided
        // there is no frame, and re-authoring the mate is the recourse.
        MateFault::FaceUnresolved { refusal, .. } => match refusal {
            FaceRefusal::PartUnresolved { .. } | FaceRefusal::Unpinned { .. } => true,
            FaceRefusal::Readback { error, .. } => !matches!(
                error,
                topo::readback::ReadbackError::NoCanonicalFrame { .. }
            ),
            FaceRefusal::NoSuchName { .. }
            | FaceRefusal::Ambiguous { .. }
            | FaceRefusal::NotAFace { .. }
            | FaceRefusal::NoReference { .. }
            | FaceRefusal::ReferenceRefused { .. }
            | FaceRefusal::NotAnInstance { .. } => false,
        },
        // An unsupported mate (a class the solve does not admit, a
        // primitive the coset table lacks) has no pose, and deleting
        // it is its recourse.
        MateFault::ClassNotAdmitted { .. }
        | MateFault::TableLacks { .. }
        | MateFault::Frame { .. }
        | MateFault::Contradictory { .. }
        | MateFault::Under { .. }
        | MateFault::DanglingHead { .. }
        | MateFault::PartSelectsAnotherCopy { .. }
        | MateFault::SelfMate { .. } => false,
    }
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
///
/// The before/after clusters are computed first and the prior
/// document is SOLVED only when a row needs a solved frame — a cluster
/// whose gauge moved (`Split`, `GaugeRewrite`) — and then once, through
/// `how`. A `Join`, a drop, a gauge that stayed put, an edit on an
/// unmated document: none of these asks the reach, so the store is
/// consulted exactly when a frame is minted from a solve. The registry
/// itself is [`registry_after`] over the acts.
///
/// # Errors
///
/// [`EditError::MaintenanceRefused`] when the prior solve reached no
/// verdict for a gauge that moved (carrying the solve's fault;
/// [`undecided`] says which faults those are);
/// [`EditError::MaintenanceUnrecorded`] when `solve` is `None` (a
/// replay — [`Maintain::Never`]) and a row needed a solved frame.
pub(crate) fn reconcile<P: crate::ProfilePayload>(
    before: &Doc<P>,
    after: &mut Doc<P>,
    tol: Tol,
    solve: Option<&dyn MateReach>,
) -> Result<Vec<ClusterMaintenance>, EditError> {
    // Neither side has a mate: every cluster is a singleton on both,
    // so the registry is already keyed by its own gauges and the
    // maintenance has nothing to do. The invariant below would compute
    // exactly this, at the cost of a recipe pass per edit.
    if !has_mates(before) && !has_mates(after) {
        return Ok(Vec::new());
    }
    let before_clusters = clusters(before);
    let before_gauge: BTreeMap<RecipeNodeId, RecipeNodeId> = before_clusters
        .iter()
        .flat_map(|c| c.iter().map(|&id| (id, c[0])))
        .collect();
    let after_clusters = clusters(after);
    // The prior solve, made on the first row that needs a solved frame
    // and never otherwise.
    let mut before_poses: Option<SolvedPoses> = None;

    let mut acts: Vec<ClusterMaintenance> = Vec::new();
    let mut carried: BTreeSet<RecipeNodeId> = BTreeSet::new();
    let mut after_gauges: BTreeSet<RecipeNodeId> = BTreeSet::new();
    for cluster in &after_clusters {
        let gauge = cluster[0];
        after_gauges.insert(gauge);
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
        if old_gauge != gauge {
            // The relative pose of this cluster's NEW gauge under the
            // PRIOR mate graph — the one number the maintenance must
            // solve for; a gauge that stayed put is the identity by
            // construction and asks nothing.
            let relative = match solve {
                Some(reach) => {
                    let poses =
                        before_poses.get_or_insert_with(|| solve_document(before, reach, tol));
                    match poses.relative(gauge) {
                        Some(relative) => relative,
                        None => {
                            let fault = unsolved_because(poses, gauge);
                            if fault.as_deref().is_none_or(undecided) {
                                return Err(EditError::MaintenanceRefused { gauge, fault });
                            }
                            // Decided: no pose to preserve. The orphan
                            // keeps the cluster's frame ([`undecided`]).
                            Frame::IDENTITY
                        }
                    }
                }
                None => return Err(EditError::MaintenanceUnrecorded { gauge }),
            };
            let prior = before.placements().get(&old_gauge).copied();
            let frame = match (prior, relative.is_identity_bits()) {
                (row, true) => row,
                (Some(f), false) => Some(f.compose(&relative)),
                (None, false) => Some(relative),
            };
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
        if !carried.contains(&gauge) && !after_gauges.contains(&gauge) {
            acts.push(ClusterMaintenance::Drop {
                gauge,
                frame: Some(frame),
            });
        }
    }
    after.set_placements(registry_after(before.placements(), &acts));
    Ok(acts)
}

#[cfg(test)]
#[allow(clippy::unwrap_used, clippy::expect_used, clippy::panic)]
mod tests {
    use super::*;
    use geom_core::predicate::MarginDiag;

    const SITE: &str = "solve_test_direction";

    fn band() -> Band {
        Band::linear(Tol::witness()).unwrap()
    }

    /// **A derived direction refuses in the frame ladder's own
    /// vocabulary, and a decided zero is not an escalation.** A length
    /// inside `(ε, Kε)` is the in-band escalation carrying its
    /// diagnostic under the site's name; one the band calls zero is
    /// the definite `Degenerate` with no diagnostic; one that
    /// underflowed the format, or is no number, is that arm; a proper
    /// rotation of a witness mints.
    #[test]
    fn a_derived_direction_refuses_as_the_frame_ladder_does() {
        let eps = Tol::witness().eps();
        let in_band = derived_direction(Vec3::new(3.0 * eps, 0.0, 0.0), SITE, band()).unwrap_err();
        let FrameError::Degenerate {
            input: FrameInput::Aim,
            indeterminate: Some(diag),
        } = in_band
        else {
            panic!("an in-band length escalates with its diagnostic: {in_band:?}");
        };
        assert_eq!(diag.predicate, Some(SITE));
        assert!(matches!(diag.margin, MarginDiag::Value(m) if (m - 3.0 * eps).abs() <= eps * 1e-9));
        assert_eq!(
            derived_direction(Vec3::new(0.5 * eps, 0.0, 0.0), SITE, band()).unwrap_err(),
            FrameError::Degenerate {
                input: FrameInput::Aim,
                indeterminate: None,
            }
        );
        assert_eq!(
            derived_direction(Vec3::new(f64::NAN, 0.0, 0.0), SITE, band()).unwrap_err(),
            FrameError::NonFiniteLength {
                input: FrameVector::Aim,
            }
        );
        assert_eq!(
            derived_direction(Vec3::new(1e-200, 0.0, 0.0), SITE, band()).unwrap_err(),
            FrameError::UnderflowedLength {
                input: FrameVector::Aim,
            }
        );
        let axis = UnitVec3::new(Vec3::new(1.0, 2.0, -3.0), SITE, band()).unwrap();
        let turned = Mat3::rotation_about(Vec3::new(0.3, -0.7, 0.2), 1.234) * axis.get();
        let minted = derived_direction(turned, SITE, band()).unwrap().get();
        assert!((minted - turned).norm() <= 4.0 * f64::EPSILON);
    }
}
