//! The seam zip (ch. 15 §15.8 step 5, per polygon pair):
//! `kfmrh(a_face, b_face)` — the cross-shell fusion (or same-shell
//! genus form once a previous polygon already fused the shells) that
//! ch. 12's `loopglue` was promised as a second consumer for — then
//! the loopglue zip itself: per coincident vertex pair one scaffolding
//! `mekr`/`mef` + `kev`, per doubled seam edge a `kef`; the section
//! faces die and the seam becomes ordinary edges of the result.
//!
//! **Correspondence is record data** (F9): the ring half-edge matched
//! to each outer half-edge comes from the null-pair vertex map built
//! by `setopfinish` — never from geometric point matching. The two
//! cycles must be **antiparallel** (A's kept loop and B's kept loop
//! run in opposite senses — the book's crossover carried through):
//! the outer half-edge leaving `a` (for `a → a⁺`, after `a⁻ → a`) is
//! paired with the ring half-edge leaving a correspondent of `a`, which
//! runs to a correspondent of `a⁻`; the zip joins the two at `a`. So a
//! vertex with several correspondents is told apart by the ring run's
//! other end: a ring vertex, with one per run it holds of the pierces
//! of its face at its point (`vtxfac::mint_ring`), and after a split
//! ([`split_cones`]) a vertex
//! holding several corners of one cone, with one per corner. A ring that
//! leaves the right vertex but runs the same sense is refused before
//! any surgery
//! ([`BooleanError::SeamOrientation`]) rather than zipped.
//!
//! Scaffolding carriers use the canonical full-period self-loop spec
//! ([`EdgeCurveSpec::self_loop_circle_at`]), whose endpoint-pin
//! certification *requires* the zipped vertex pairs to be bitwise
//! coincident — the pipeline's coincidences are (crossing points are
//! computed once and inserted into both bodies; ring vertices copy the
//! pierce point bitwise); anything less refuses loudly at
//! certification, never zips approximately.
//!
//! **A pinch is one vertex per cone** ([`split_cones`]): before any
//! zip, each operand vertex whose section corners lead into several
//! cones of the result is split per cone, each new vertex on the split
//! vertex's point key. The transient edge each split leaves lies on the
//! section faces the zips consume, so no kept face's topology changes.
//! The split and the zip read one alignment ([`align`]); where the
//! two operands' splits leave their section faces paired several to one
//! (or several), the faces holding paired edges zip as one split seam
//! ([`zip_split`]). After the zips,
//! [`share_points`] puts the cones of a pinch that still sit on several
//! keys onto one: the keys the correspondence ties ([`point_classes`]),
//! read before the split.
//! Where the insertion hung runs at a turned run's copy, a point whose
//! cones still sit on several keys after that refuses
//! ([`refuse_split_hung_points`]).

use std::collections::{BTreeMap, BTreeSet};

use geom_core::Decide;

use super::BooleanError;
use crate::body::Body;
use crate::entity::{EntityId, FaceKey, HalfEdgeKey, LoopBoundary, LoopKey, VertexKey};
use crate::euler::{FaceSurface, MefSite, MevSite};
use crate::euler_ring::MekrSite;
use crate::geometry::PointKey;
use crate::live::proven;
use geom_brep::EdgeCurveSpec;
use geom_core::Tol;

/// The seam vertex correspondence: each A-side vertex → its B-side
/// correspondents. One each, except where the seams meet a vertex at
/// several corners: a ring vertex holds one per run of the pierces it
/// rings, and once [`split_cones`] rebuilds the map from the edges the zips
/// join, a vertex holds one per corner.
pub(super) type SeamCorrespondence = BTreeMap<VertexKey, BTreeSet<VertexKey>>;

pub use crate::lineage::Fusions;

/// Where a zero-length joint runs between two vertices of one face.
pub(super) enum Joint {
    /// Across two of its loops (`mekr`, which joins them): `target`'s
    /// loop absorbs `ring`'s.
    Loops {
        target: HalfEdgeKey,
        ring: HalfEdgeKey,
    },
    /// Across one loop (`mef`, which divides the face).
    Chord { he1: HalfEdgeKey, he2: HalfEdgeKey },
}

/// **Fuses two coincident vertices**: a zero-length edge between them at
/// `p` (the canonical self-loop carrier, whose certification requires
/// the pair bitwise coincident), collapsed onto `keep`, one of its ends,
/// by a `kev` that keeps the merged fan's carriers, each re-certified at
/// the kept vertex under the run's band. Returns the fusion
/// `(dead, kept)`; `desync` names a joint that no longer resolves.
pub(super) fn fuse_by_joint<T: Decide + crate::props::AtRestPolicy>(
    body: &mut Body<T>,
    joint: Joint,
    (p, keep): (geom_core::Point3<T>, VertexKey),
    desync: fn(&'static str) -> BooleanError,
    tol: Tol,
) -> Result<(VertexKey, VertexKey), BooleanError> {
    let carrier = EdgeCurveSpec::self_loop_circle_at(p);
    let (plus, minus) = match joint {
        Joint::Loops { target, ring } => {
            let made = body.mekr(MekrSite::Cycles { target, ring }, carrier, tol)?;
            (made.he_plus, made.he_minus)
        }
        Joint::Chord { he1, he2 } => {
            let made = body.mef(
                MefSite::Chords { he1, he2 },
                carrier,
                FaceSurface::Inherit,
                tol,
            )?;
            (made.he_plus, made.he_minus)
        }
    };
    let start = |he: HalfEdgeKey| {
        body.get_half_edge(he)
            .map(|h| h.start)
            .ok_or_else(|| desync("a joint half-edge no longer resolves"))
    };
    let he = if start(plus)? == keep { plus } else { minus };
    let dead = body
        .half_edge_end(he)
        .ok_or_else(|| desync("a joint half-edge has no end"))?;
    body.kev_describing(he, &[], tol)?;
    Ok((dead, keep))
}

/// What one seam zip did to the arena — the F9-style record the op
/// stage consumes (M3 PR 6a): every vertex fusion (dead key → kept
/// key, the D5 descendant map's zip rows) and the surviving seam
/// edges (the D6 description pass's worklist — tracked lineage, never
/// a post-hoc scan).
#[derive(Debug, Default)]
pub(super) struct ZipReport {
    /// Vertex fusions in zip order: `(dead, kept)` per zipped pair.
    pub vertex_merges: Fusions,
    /// The seam edges surviving the zip: each A section half-edge's
    /// edge, in the order the zip was given them.
    pub seam_edges: Vec<crate::entity::EdgeKey>,
    /// Edge fusions, `(dead, kept)`: each B section edge the zip kills
    /// and the A seam edge on its segment, which survives in its place.
    pub edge_merges: Vec<(crate::entity::EdgeKey, crate::entity::EdgeKey)>,
}

/// The order the loopglue zip fuses a seam's `n` vertex pairs in:
/// pair 0 first (its `mekr` joins the two loops), then `n − 1` down to
/// 1 (each a `mef` and the `kef` of the strip behind it).
pub(crate) fn fusion_order(n: usize) -> impl Iterator<Item = usize> {
    core::iter::once(0).chain((1..n).rev())
}

/// A union-find over keys, as a parent map: a key with no entry is its
/// own root, and a class is rooted at its smallest key.
pub(super) struct Roots<K>(BTreeMap<K, K>);

impl<K: Ord + Copy> Roots<K> {
    pub(super) fn new() -> Self {
        Self(BTreeMap::new())
    }

    pub(super) fn find(&self, mut k: K) -> K {
        while let Some(&up) = self.0.get(&k) {
            k = up;
        }
        k
    }

    /// Joins the classes of `a` and `b`; `false` where they were one.
    pub(super) fn union(&mut self, a: K, b: K) -> bool {
        let (ra, rb) = (self.find(a), self.find(b));
        if ra != rb {
            self.0.insert(ra.max(rb), ra.min(rb));
        }
        ra != rb
    }
}

/// **The point keys the seams tie are one point**: a pinch is several
/// vertices on one point key (Ev, PR 4057).
///
/// Each seam pair's two vertices are one point, which the zips need
/// (their scaffolding certifies only coincident pairs), and an op's
/// copies of one vertex share its key (`Body::mev_null`). So every key
/// the correspondence `vmap` reaches, through its pairs and through the
/// keys its vertices share, names one point. Returns those classes of
/// two or more keys, read before any zip; [`share_points`] reads them
/// after. Decided by the records alone: no position is read.
///
/// A ring serving several pierces ties their keys too: the ring joined
/// those pierces by a value decision, which the ledger records beside
/// the result's records (`ops::ledger`).
///
/// # Errors
///
/// [`BooleanError::ZipCorrespondence`] where a correspondent no longer
/// resolves.
pub(super) fn point_classes<T: geom_core::Real>(
    body: &Body<T>,
    vmap: &SeamCorrespondence,
) -> Result<Vec<BTreeSet<PointKey>>, BooleanError> {
    let corr = || BooleanError::ZipCorrespondence {
        what: "a seam correspondent no longer resolves",
    };
    let key = |v: VertexKey| body.get_vertex(v).map(|d| d.point).ok_or_else(corr);
    let mut roots = Roots::new();
    for (&a, bs) in vmap {
        let ka = key(a)?;
        for &b in bs {
            roots.union(ka, key(b)?);
        }
    }
    let mut classes: BTreeMap<PointKey, BTreeSet<PointKey>> = BTreeMap::new();
    for &k in roots.0.keys() {
        let r = roots.find(k);
        classes
            .entry(r)
            .or_insert_with(|| BTreeSet::from([r]))
            .insert(k);
    }
    Ok(classes.into_values().collect())
}

/// **A pinch's cones sit on one point key** ([`point_classes`]). After
/// the zips, the vertices of one point fused where they lie in one cone,
/// and a fused vertex keeps one key; the vertices of a pinch's other
/// cones keep theirs. So where a class's live vertices still sit on
/// several keys (an operand's own pinch, which an earlier op left on
/// several keys, tied through the other operand's copy of the point),
/// they move onto the class's smallest live key
/// ([`Body::share_point`]). Elsewhere the zips left one vertex, and
/// nothing moves. The census's same-point rung and the output stage's
/// join then read the pinch from its keys.
///
/// # Errors
///
/// [`BooleanError::ZipCorrespondence`] where a vertex no longer
/// resolves, which the index built just before rules out.
pub(super) fn share_points<T: geom_core::Real>(
    body: &mut Body<T>,
    classes: &[BTreeSet<PointKey>],
) -> Result<(), BooleanError> {
    // The classes are disjoint (`point_classes` builds them as a
    // partition), so one index serves them all.
    let mut on_key: BTreeMap<PointKey, Vec<VertexKey>> = BTreeMap::new();
    for (v, d) in body.vertices() {
        on_key.entry(d.point).or_default().push(v);
    }
    for class in classes {
        let keys: Vec<PointKey> = class
            .iter()
            .copied()
            .filter(|k| on_key.contains_key(k))
            .collect();
        let [onto, _, ..] = keys[..] else {
            continue;
        };
        let vertices: Vec<VertexKey> = keys.iter().flat_map(|k| on_key[k].clone()).collect();
        #[cfg(feature = "sweep-testing")]
        {
            let at: Vec<[String; 3]> = keys
                .iter()
                .filter_map(|&k| body.points.get(k))
                .map(|p| [p.x, p.y, p.z].map(|c| format!("{c:?}")))
                .collect();
            SHARED.with(|s| s.borrow_mut().push(at));
        }
        // Unreachable: `on_key` holds live vertices and live keys only.
        body.share_point(&vertices, onto)
            .ok_or(BooleanError::ZipCorrespondence {
                what: "a pinch vertex no longer resolves",
            })?;
    }
    Ok(())
}

/// **A hung point left on several keys refuses.** Where the insertion
/// hung runs at a copy of their own pair's (`insert::hang_at_shared`), `hung`
/// holds the point's keys: those of every vertex the vertex-vertex
/// contacts tie to the hung one, in result keys. Those are all of the
/// point's keys. Every vertex there either has a contact, or is a copy
/// that keeps its original's key (a null edge's, a cone split's), and
/// the seams' classes ([`point_classes`]) tie only keys of vertices the
/// seams pair there, which the contacts already tie. So after
/// [`share_points`], live vertices on more than one of them are a pinch
/// whose cones the seams do not link: an operand's own pinch, left on
/// several keys by an earlier op. The census cannot read it, and the op
/// refuses [`BooleanError::PinchConesOnSeparateKeys`] rather than ship
/// it. Only keys are read, no position.
///
/// # Errors
///
/// [`BooleanError::PinchConesOnSeparateKeys`] at such a point.
pub(super) fn refuse_split_hung_points<T: geom_core::Real>(
    body: &Body<T>,
    hung: &[(super::insert::Hang, BTreeSet<PointKey>)],
) -> Result<(), BooleanError> {
    for (hang, keys) in hung {
        let live: BTreeSet<PointKey> = body
            .vertices()
            .map(|(_, d)| d.point)
            .filter(|k| keys.contains(k))
            .collect();
        if live.len() > 1 {
            return Err(BooleanError::PinchConesOnSeparateKeys {
                operand: hang.operand,
                vertex: hang.vertex,
            });
        }
    }
    Ok(())
}

#[cfg(feature = "sweep-testing")]
thread_local! {
    /// Whether [`zip_split`] plays the two operands' roles the other way
    /// round ([`swap_split_roles`]).
    static SWAP_ROLES: core::cell::Cell<bool> = const { core::cell::Cell::new(false) };
}

/// Makes every [`zip_split`] on this thread glue with the operands' roles
/// swapped, B's faces hosting and B's half-edges leading, until called
/// with `false`: the same split seam read from the other side, which a
/// glue that only handles one side of it fails. `sweep-testing` only.
#[cfg(feature = "sweep-testing")]
pub fn swap_split_roles(on: bool) {
    SWAP_ROLES.with(|s| s.set(on));
}

#[cfg(feature = "sweep-testing")]
thread_local! {
    /// The stored points of the keys each [`share_points`] class rebinds,
    /// in call order ([`take_shared_points`]).
    static SHARED: core::cell::RefCell<Vec<Vec<[String; 3]>>> =
        const { core::cell::RefCell::new(Vec::new()) };
}

/// Drains the stored points of every class [`share_points`] rebound on
/// this thread since the last drain: one list per class, each key's
/// point before the rebind, each coordinate as its `{:?}` rendering.
/// `share_points` reads no position, so its premise (the seams tie only
/// keys that hold one point) is pinned here, in test builds: a row
/// parses the coordinates back and asserts each list is one point, bit
/// for bit. At `f64` the rendering is the shortest string that parses
/// back to the same value, so the parse is exact. Rendered, not typed,
/// because `T` reaches its bits only through the fenced bit-identity
/// seam (`geom_core::bit_identity`), which a test witness has no claim
/// on. `sweep-testing` only.
#[cfg(feature = "sweep-testing")]
pub fn take_shared_points() -> Vec<Vec<[String; 3]>> {
    SHARED.with(|s| core::mem::take(&mut *s.borrow_mut()))
}

/// Each vertex's section corners, as pair indices in orbit order.
type RunsAt = BTreeMap<VertexKey, Vec<usize>>;

/// **A pinch is one vertex per cone** (Ev, PR 4057): before any zip,
/// each operand vertex the seams meet is split so that it holds one cone
/// of the result's boundary.
///
/// The zips fuse each seam's vertex pairs, seam after seam, and a
/// result vertex is one cycle of the corners they join round a point.
/// At an operand vertex `x`, each of its section corners on a seam
/// starts a *run*: `x`'s corners from that section half-edge round to
/// the next one. The cones are read off the runs ([`cones`]). A vertex
/// whose runs lie in several cones is a pinch: `mev_null`, which keeps
/// its new vertex on the old one's point key, moves each cone's runs but
/// the first to a vertex of their own ([`split_side`]). The seams are
/// then re-paired ([`repair`]), a split seam where the faces holding
/// paired edges are several of either operand's, and `vmap` is rebuilt
/// from the edges the zips join. Returns the seams to zip, and the proof
/// that they were split ([`ConesSplit`]).
///
/// The split writes `body` before the re-pair can refuse: `body` is the
/// boolean's working copy, which a refusal drops whole.
///
/// # Errors
///
/// [`BooleanError::ZipCorrespondence`] where a vertex's cones interleave
/// round it, where the split section faces do not pair ([`repair`]), or
/// where a cone still fuses a vertex to itself (the seams meet one cone twice at
/// one vertex of each operand); [`BooleanError::Euler`] where a split's
/// operator refuses.
pub(super) fn split_cones<T: Decide + crate::props::AtRestPolicy>(
    body: &mut Body<T>,
    seams: &[(FaceKey, FaceKey)],
    vmap: &mut SeamCorrespondence,
    tol: Tol,
) -> Result<(Vec<Seam>, ConesSplit), BooleanError> {
    let corr = |what| BooleanError::ZipCorrespondence { what };
    let outer = |body: &Body<T>, f: FaceKey| -> Result<LoopKey, BooleanError> {
        Ok(body
            .get_face(f)
            .ok_or_else(|| corr("section face no longer resolves"))?
            .outer)
    };
    // Every seam's pairs `(ob, rs)`, and each pair's successor in its seam.
    let mut pairs: Vec<(HalfEdgeKey, HalfEdgeKey)> = Vec::new();
    let mut next: Vec<usize> = Vec::new();
    for &(a_face, b_face) in seams {
        let ob = section_cycle(body, outer(body, a_face)?)?;
        let rs = align(
            body,
            (a_face, b_face),
            &ob,
            &section_cycle(body, outer(body, b_face)?)?,
            vmap,
        )?;
        let base = pairs.len();
        for (j, (&o, &r)) in ob.iter().zip(&rs).enumerate() {
            one_vertex_sense(body, (a_face, b_face), (o, r), tol)?;
            pairs.push((o, r));
            next.push(base + (j + 1) % ob.len());
        }
    }
    let he_a = |k: usize| pairs[k].0;
    let he_b = |k: usize| pairs[k].1;
    let (sigma_a, runs_a) = sigma(body, pairs.len(), &he_a)?;
    let (sigma_b, runs_b) = sigma(body, pairs.len(), &he_b)?;
    let (cone_a, cone_b) = cones(&sigma_a, &sigma_b);
    let mut a_sections: Vec<FaceKey> = seams.iter().map(|&(a, _)| a).collect();
    let mut b_sections: Vec<FaceKey> = seams.iter().map(|&(_, b)| b).collect();
    let mut splits = Vec::new();
    split_side(body, &runs_a, &cone_a, &he_a, &mut a_sections, &mut splits, tol)?;
    split_side(body, &runs_b, &cone_b, &he_b, &mut b_sections, &mut splits, tol)?;
    let moved = !splits.is_empty();
    let paired = if moved {
        repair(body, &pairs, &next, &a_sections, &b_sections)?
    } else {
        seams.iter().map(|&(a, b)| Seam::Faces(a, b)).collect()
    };
    // No cone may fuse a vertex to itself. It needs one cone holding two
    // runs on one vertex of each operand; no battery line reaches it once
    // the split runs. Without the split it fires (the review's "no split"
    // mutant, 450 lines), so it stays typed.
    let end_of = |body: &Body<T>, he: HalfEdgeKey| {
        body.half_edge_end(he)
            .ok_or_else(|| corr("a seam half-edge has no end"))
    };
    let fused: Vec<(VertexKey, VertexKey)> = (0..pairs.len())
        .map(|k| Ok((start_of(body, pairs[k].0)?, end_of(body, pairs[next[k]].1)?)))
        .collect::<Result<_, BooleanError>>()?;
    let mut roots = Roots::new();
    for &(a, b) in &fused {
        if !roots.union(a, b) {
            return Err(corr("a cone the seams meet twice fuses a vertex to itself"));
        }
    }
    if moved {
        let mut rebuilt = SeamCorrespondence::new();
        for (a, b) in fused {
            rebuilt.entry(a).or_default().insert(b);
        }
        *vmap = rebuilt;
    }
    Ok((paired, ConesSplit { splits }))
}

/// **The seams were split per cone** ([`split_cones`]): a result built
/// past it is one whose pinches the boolean's carry reads
/// (`ops::Zipped`). Only [`split_cones`] builds one.
pub(super) struct ConesSplit {
    /// Each split `(split, new)` in the order made, keys of the body
    /// split: the vertex split and the vertex its moved cone now holds,
    /// on the split vertex's point key
    /// (`BooleanNaming::cone_splits`).
    splits: Vec<(VertexKey, VertexKey)>,
}

impl ConesSplit {
    /// The splits made, `(split, new)` in the order made.
    pub(super) fn splits(&self) -> &[(VertexKey, VertexKey)] {
        &self.splits
    }
}

/// One side's `σ`: each pair's next section corner round its vertex, as
/// a pair index, and each vertex's runs in orbit order. `he_of` reads a
/// pair's half-edge on that side.
fn sigma<T: Decide>(
    body: &Body<T>,
    n: usize,
    he_of: &dyn Fn(usize) -> HalfEdgeKey,
) -> Result<(Vec<usize>, RunsAt), BooleanError> {
    let pair_of: BTreeMap<HalfEdgeKey, usize> = (0..n).map(|k| (he_of(k), k)).collect();
    let mut runs = RunsAt::new();
    let mut step = vec![usize::MAX; n];
    for k in 0..n {
        let v = start_of(body, he_of(k))?;
        if runs.contains_key(&v) {
            continue;
        }
        let at: Vec<usize> = body
            .vertex_orbit_linked(v)
            .iter()
            .filter_map(|h| pair_of.get(h).copied())
            .collect();
        for (i, &k) in at.iter().enumerate() {
            step[k] = at[(i + 1) % at.len()];
        }
        runs.insert(v, at);
    }
    if step.contains(&usize::MAX) {
        return Err(BooleanError::ZipCorrespondence {
            what: "a section half-edge is missing from its vertex's orbit",
        });
    }
    Ok((step, runs))
}

/// The cone each run lies in, by pair index: `(A runs, B runs)`, a cone
/// named by one of its A runs.
///
/// An A run ends at the section corner `σ_A` steps it to. The zip glues
/// that corner's A section edge to the B ring edge the B run starting
/// there leaves along, so the boundary continues into that B run, and
/// from its end corner into the next A run the same way. So the cones
/// are the cycles of `σ_B ∘ σ_A`, A run to A run, and the B run starting
/// at corner `σ_A(k)` lies in A run `k`'s cone. Read the other way round
/// (`σ_A ∘ σ_B`), or each B run with the A run starting at its own
/// corner, a vertex's grouping turns by one run: invisible at two runs,
/// a different split where three or more runs hold two cones.
fn cones(sigma_a: &[usize], sigma_b: &[usize]) -> (Vec<usize>, Vec<usize>) {
    let n = sigma_a.len();
    let mut cone_a = vec![usize::MAX; n];
    for k0 in 0..n {
        let mut k = k0;
        while cone_a[k] == usize::MAX {
            cone_a[k] = k0;
            k = sigma_b[sigma_a[k]];
        }
    }
    let mut cone_b = vec![usize::MAX; n];
    for k in 0..n {
        cone_b[sigma_a[k]] = cone_a[k];
    }
    (cone_a, cone_b)
}

/// Splits each of one side's vertices per cone, its first cone's runs
/// staying, one cone at a time: a cone whose runs lie together round the
/// vertex, first in orbit order. Each split is pushed to `splits` as
/// `(split, new)`.
///
/// The null edge each `mev_null` leaves lies between two section
/// corners, on the section faces the zips consume, and is killed there:
/// by `kef` between two section faces (merging their seams into one),
/// or by `kemr` within one section loop, whose split-off loop `mfkrh`
/// promotes to a section face of its own. `sections` follows both.
///
/// # Errors
///
/// [`BooleanError::ZipCorrespondence`] where a vertex's cones interleave
/// round it. Cones nest round a vertex, one's runs between two of
/// another's, where a pierce ring is a tree deeper than a path; each
/// split takes a cone whose runs lie together, which nesting always
/// leaves, so only two cones' runs alternating, their boundary cycles
/// crossing there, refuse. No battery line reaches that.
/// The Euler operators' refusals, typed.
fn split_side<T: Decide + crate::props::AtRestPolicy>(
    body: &mut Body<T>,
    runs: &RunsAt,
    cone_of: &[usize],
    he_of: &dyn Fn(usize) -> HalfEdgeKey,
    sections: &mut Vec<FaceKey>,
    splits: &mut Vec<(VertexKey, VertexKey)>,
    tol: Tol,
) -> Result<(), BooleanError> {
    for at in runs.values() {
        let plan = peel(at, cone_of).ok_or(BooleanError::ZipCorrespondence {
            what: "a vertex's cones interleave round it",
        })?;
        for (from, to) in plan {
            let made = body.mev_null(
                MevSite::Fan {
                    he1: he_of(from),
                    he2: he_of(to),
                },
                crate::NewVertexSide::Above,
            )?;
            let split = proven(&body.half_edges, made.he_plus, EntityId::HalfEdge).start;
            splits.push((split, made.vertex));
            let loop_of =
                |he: HalfEdgeKey| proven(&body.half_edges, he, EntityId::HalfEdge).parent_loop;
            let (lp, lm) = (loop_of(made.he_plus), loop_of(made.he_minus));
            if lp == lm {
                let ring = body.kemr_minting(made.he_plus, made.he_minus, tol)?.ring;
                sections.push(body.mfkrh_minting(ring, FaceSurface::Inherit, tol)?.face);
            } else {
                let killed = body.kef_minting(made.he_plus, tol)?.killed_face;
                sections.retain(|&f| f != killed);
            }
        }
    }
    Ok(())
}

/// One vertex's splits, read before any is written: each the first and
/// the next pair index of a group of runs one cone holds alone, the
/// group moved off the vertex (the `mev_null` fan between them), until
/// one cone is left. Groups are maximal runs of one cone round the
/// vertex; the first stays, and the split takes the first later group
/// whose cone has no other. `None` where no such group is left while
/// two cones are: their runs alternate round the vertex.
fn peel(at: &[usize], cone_of: &[usize]) -> Option<Vec<(usize, usize)>> {
    let mut at = at.to_vec();
    let mut plan = Vec::new();
    loop {
        let r = at.len();
        let cones_here: BTreeSet<usize> = at.iter().map(|&k| cone_of[k]).collect();
        if cones_here.len() < 2 {
            return Some(plan);
        }
        let starts: Vec<usize> = (0..r)
            .filter(|&i| cone_of[at[i]] != cone_of[at[(i + r - 1) % r]])
            .collect();
        let whole = |g: usize| {
            let c = cone_of[at[starts[g]]];
            (0..starts.len()).all(|h| h == g || cone_of[at[starts[h]]] != c)
        };
        let g = (1..starts.len()).find(|&g| whole(g))?;
        let (s, end) = (starts[g], starts[(g + 1) % starts.len()]);
        plan.push((at[s], at[end]));
        if end > s {
            at.drain(s..end);
        } else {
            at.drain(s..);
            at.drain(..end);
        }
    }
}

/// What one zip consumes ([`split_cones`]).
pub(super) enum Seam {
    /// One section face of each operand whose cycles pair
    /// ([`zip_seam`]).
    Faces(FaceKey, FaceKey),
    /// Split section faces whose edges pair across several faces of
    /// either operand ([`zip_split`]).
    Split {
        /// The A section faces.
        a_faces: Vec<FaceKey>,
        /// The B section faces.
        b_faces: Vec<FaceKey>,
        /// `(A half-edge, B half-edge)`, one per segment.
        edges: Vec<(HalfEdgeKey, HalfEdgeKey)>,
    },
}

/// The seams after a split: an A section edge is the B ring edge it zips
/// with (`align`: `ob[k]` with `rs[next[k]]`), and the faces holding
/// paired edges are one seam. A seam of one face of each is a face pair.
/// A split can leave more: a vertex of each operand holding corners of
/// two cones, the two operands' vertices holding different pairs of
/// those corners (each operand's split merging or dividing its section
/// faces at its own vertex's corners), so one face's edges pair with
/// several of the other operand's faces.
///
/// # Errors
///
/// [`BooleanError::ZipCorrespondence`] where a split section edge's
/// partner lies on no section face of the other operand, or a section
/// face pairs with none.
fn repair<T: Decide>(
    body: &Body<T>,
    pairs: &[(HalfEdgeKey, HalfEdgeKey)],
    next: &[usize],
    a_sections: &[FaceKey],
    b_sections: &[FaceKey],
) -> Result<Vec<Seam>, BooleanError> {
    let refuse = |what| BooleanError::ZipCorrespondence { what };
    let face_of = |he: HalfEdgeKey| {
        body.face_of_half_edge(he)
            .ok_or_else(|| refuse("a split section half-edge has no face"))
    };
    let edges: Vec<(HalfEdgeKey, HalfEdgeKey)> = (0..pairs.len())
        .map(|k| (pairs[k].0, pairs[next[k]].1))
        .collect();
    let mut roots = Roots::new();
    let mut a_face_of = Vec::with_capacity(edges.len());
    for &(ob, r) in &edges {
        let (fa, fb) = (face_of(ob)?, face_of(r)?);
        if !a_sections.contains(&fa) || !b_sections.contains(&fb) {
            return Err(refuse(
                "a split section edge pairs with one off the section faces",
            ));
        }
        roots.union(fa, fb);
        a_face_of.push(fa);
    }
    let mut seams = Vec::new();
    let mut done = BTreeSet::new();
    for &fa in a_sections {
        let root = roots.find(fa);
        if !done.insert(root) {
            continue;
        }
        let of = |fs: &[FaceKey]| -> Vec<FaceKey> {
            fs.iter()
                .copied()
                .filter(|&f| roots.find(f) == root)
                .collect()
        };
        let (a_faces, b_faces) = (of(a_sections), of(b_sections));
        seams.push(match (&a_faces[..], &b_faces[..]) {
            (_, []) => return Err(refuse("a split section face pairs with none")),
            (&[a], &[b]) => Seam::Faces(a, b),
            _ => Seam::Split {
                edges: edges
                    .iter()
                    .zip(&a_face_of)
                    .filter(|&(_, &f)| roots.find(f) == root)
                    .map(|(&e, _)| e)
                    .collect(),
                a_faces,
                b_faces,
            },
        });
    }
    if b_sections.iter().any(|&f| !done.contains(&roots.find(f))) {
        return Err(refuse("a split section face pairs with none"));
    }
    Ok(seams)
}

/// Zips one section-face pair (module docs).
pub(super) fn zip_seam<T: Decide + crate::props::AtRestPolicy>(
    body: &mut Body<T>,
    a_face: FaceKey,
    b_face: FaceKey,
    vmap: &SeamCorrespondence,
    tol: Tol,
) -> Result<ZipReport, BooleanError> {
    let corr = |what| BooleanError::ZipCorrespondence { what };
    // ---- Fuse: B's section face becomes a ring of A's. ----
    let ring = body.kfmrh_minting(a_face, b_face, tol)?.ring;
    let outer = body
        .get_face(a_face)
        .ok_or_else(|| corr("A section face no longer resolves"))?
        .outer;
    let ob = section_cycle(body, outer)?;
    let rs = align(
        body,
        (a_face, b_face),
        &ob,
        &section_cycle(body, ring)?,
        vmap,
    )?;
    // `rs[j]` runs between the correspondents of `ob[j]`'s start and
    // `ob[j - 1]`'s (`align`), so it lies on `ob[j - 1]`'s segment.
    let n = ob.len();
    let edges: Vec<_> = (0..n).map(|j| (ob[j], rs[(j + 1) % n])).collect();
    glue(body, &edges, fusion_order(n), tol)
}

/// Zips a split seam ([`repair`]): its faces become loops of its first A
/// face, and [`glue`] zips them; `edges` pairs each A section half-edge
/// with the B one on its segment.
///
/// # Errors
///
/// As [`glue`], and [`BooleanError::Euler`] where a face does not
/// become a ring of the first (`kfmrh`).
pub(super) fn zip_split<T: Decide + crate::props::AtRestPolicy>(
    body: &mut Body<T>,
    (a_faces, b_faces): (&[FaceKey], &[FaceKey]),
    edges: &[(HalfEdgeKey, HalfEdgeKey)],
    tol: Tol,
) -> Result<ZipReport, BooleanError> {
    #[cfg(feature = "sweep-testing")]
    if SWAP_ROLES.with(core::cell::Cell::get) {
        let swapped: Vec<_> = edges.iter().map(|&(ob, r)| (r, ob)).collect();
        return zip_split_as_given(body, (b_faces, a_faces), &swapped, tol);
    }
    zip_split_as_given(body, (a_faces, b_faces), edges, tol)
}

/// [`zip_split`] with its roles as given.
fn zip_split_as_given<T: Decide + crate::props::AtRestPolicy>(
    body: &mut Body<T>,
    (a_faces, b_faces): (&[FaceKey], &[FaceKey]),
    edges: &[(HalfEdgeKey, HalfEdgeKey)],
    tol: Tol,
) -> Result<ZipReport, BooleanError> {
    let mut faces = a_faces.iter().chain(b_faces);
    let host = *faces.next().ok_or(BooleanError::ZipCorrespondence {
        what: "a split seam holds no face",
    })?;
    for &f in faces {
        body.kfmrh_minting(host, f, tol)?;
    }
    glue(body, edges, 0..edges.len(), tol)
}

/// Where a seam pair's two half-edges stand ([`glue`]).
#[derive(Clone, Copy, PartialEq, Eq)]
enum Pair {
    /// Not yet a loop of their own.
    Open,
    /// A loop of their own, alone in its face.
    Alone,
    /// A loop of their own, sharing its face with another loop.
    Shared,
}

/// **The loopglue zip, driven by records**: glues each A section
/// half-edge `ob` of `edges` to its B partner `r` on the same segment,
/// all on loops of the section faces the zip consumes.
///
/// A joint fusing two vertices, collapsed by its `kev`
/// ([`fuse_by_joint`]), hands the two joined half-edges' predecessors
/// each other as successor. So one joint per corner makes `r` precede
/// `ob` (in `order`) and `ob` precede `r` (after), unless the fusions
/// already did; then `ob` and `r` are a loop, which `kef` kills once it
/// is alone in its face. A face pair's cycle in `fusion_order` is the
/// reassembly oracle's sequence.
///
/// # Errors
///
/// [`BooleanError::ZipCorrespondence`] where a corner is wrong at a
/// vertex the zip already fused, where a pair does not close into a
/// face of its own, or where a joint's two faces both hold rings;
/// [`BooleanError::Euler`] where an operator refuses.
fn glue<T: Decide + crate::props::AtRestPolicy>(
    body: &mut Body<T>,
    edges: &[(HalfEdgeKey, HalfEdgeKey)],
    order: impl Iterator<Item = usize>,
    tol: Tol,
) -> Result<ZipReport, BooleanError> {
    let corr = |what| BooleanError::ZipCorrespondence { what };
    let n = edges.len();
    let edge_of = |body: &Body<T>, he: HalfEdgeKey| {
        body.get_half_edge(he)
            .map(|h| h.edge)
            .ok_or_else(|| corr("seam half-edge no longer resolves"))
    };
    let mut report = ZipReport::default();
    for &(ob, r) in edges {
        report.seam_edges.push(edge_of(body, ob)?);
        report
            .edge_merges
            .push((edge_of(body, r)?, edge_of(body, ob)?));
    }
    let pair_of: BTreeMap<HalfEdgeKey, usize> = edges
        .iter()
        .enumerate()
        .flat_map(|(k, &(ob, r))| [(ob, k), (r, k)])
        .collect();
    let he =
        |body: &Body<T>, he: HalfEdgeKey| proven(&body.half_edges, he, EntityId::HalfEdge).clone();
    let face = |body: &Body<T>, h: HalfEdgeKey| -> Result<(FaceKey, LoopKey, bool), BooleanError> {
        let f = body
            .face_of_half_edge(h)
            .ok_or_else(|| corr("a seam half-edge has no face"))?;
        let fd = proven(&body.faces, f, EntityId::Face);
        let l = he(body, h).parent_loop;
        Ok((f, fd.outer, fd.outer == l && fd.rings.is_empty()))
    };
    let pair = |body: &Body<T>, k: usize| -> Result<Pair, BooleanError> {
        let (ob, r) = edges[k];
        if he(body, ob).next != r || he(body, r).next != ob {
            return Ok(Pair::Open);
        }
        Ok(if face(body, ob)?.2 {
            Pair::Alone
        } else {
            Pair::Shared
        })
    };
    let mut killed = vec![false; n];
    let starts = order.map(|k| (k, true));
    for (k, start) in starts.chain((0..n).map(|k| (k, false))).collect::<Vec<_>>() {
        if killed[k] {
            continue;
        }
        let (ob, r) = edges[k];
        let (h, o) = if start { (r, ob) } else { (ob, r) };
        let x = he(body, h).next;
        if x == o {
            continue;
        }
        // Each joint fuses two vertices, so a corner wrong at one vertex
        // has no joint left to set it. The greedy order never meets one
        // where every vertex holds one corner of the pairs (each fix
        // takes the next corner of one cycle); a vertex holding several
        // has no such proof, and refuses here.
        if he(body, x).start == he(body, o).start {
            return Err(corr(if start {
                "a seam pair's start corner is wrong at a vertex the zip fused"
            } else {
                "a seam pair's end corner is wrong at a vertex the zip fused"
            }));
        }
        let prev_o = he(body, o).prev;
        // The kept vertex is `ob`'s at its start, its successor's at its
        // end: A's.
        let (he1, he2) = if start { (o, x) } else { (x, o) };
        let (l1, l2) = (he(body, he1).parent_loop, he(body, he2).parent_loop);
        let joint = if l1 == l2 {
            Joint::Chord { he1, he2 }
        } else {
            let (f1, f2) = (face(body, he1)?.0, face(body, he2)?.0);
            if f1 != f2 {
                let ringless = |f: FaceKey| proven(&body.faces, f, EntityId::Face).rings.is_empty();
                match (ringless(f2), ringless(f1)) {
                    (true, _) => body.kfmrh_minting(f1, f2, tol)?,
                    (_, true) => body.kfmrh_minting(f2, f1, tol)?,
                    _ => return Err(corr("a seam joint's two faces both hold rings")),
                };
            }
            // `mekr` absorbs a ring into the other loop; the joint reads
            // the same either way round.
            if face(body, he2)?.1 == l2 {
                Joint::Loops {
                    target: he2,
                    ring: he1,
                }
            } else {
                Joint::Loops {
                    target: he1,
                    ring: he2,
                }
            }
        };
        let keep = he(body, he1).start;
        let p = body
            .get_vertex(keep)
            .and_then(|vd| body.get_point(vd.point).copied())
            .ok_or_else(|| corr("seam vertex has no point"))?;
        let merge = fuse_by_joint(body, joint, (p, keep), corr, tol)?;
        report.vertex_merges.push(merge)?;
        // The pairs this joint can have closed: `o`'s, then those of the
        // two half-edges it handed new successors.
        for touched in [o, x, prev_o] {
            if let Some(&k) = pair_of.get(&touched)
                && !killed[k]
                && pair(body, k)? == Pair::Alone
            {
                body.kef_minting(edges[k].1, tol)?;
                killed[k] = true;
            }
        }
    }
    // A closed pair sharing its face: the rings take faces of their own
    // (`mfkrh`) first, then each kills as above.
    for rings_first in [true, false] {
        for k in 0..n {
            if killed[k] {
                continue;
            }
            match pair(body, k)? {
                Pair::Open => return Err(corr("a seam pair's loop does not close")),
                Pair::Alone => {}
                Pair::Shared => {
                    let (_, outer, _) = face(body, edges[k].0)?;
                    let l = he(body, edges[k].0).parent_loop;
                    if outer == l {
                        if rings_first {
                            continue;
                        }
                        return Err(corr("a seam pair's loop does not close"));
                    }
                    body.mfkrh_minting(l, FaceSurface::Inherit, tol)?;
                }
            }
            body.kef_minting(edges[k].1, tol)?;
            killed[k] = true;
        }
    }
    Ok(report)
}

/// The half-edge cycle of a section face's loop `l`.
fn section_cycle<T: Decide>(body: &Body<T>, l: LoopKey) -> Result<Vec<HalfEdgeKey>, BooleanError> {
    let corr = |what| BooleanError::ZipCorrespondence { what };
    let LoopBoundary::Cycle { first } = body
        .get_loop(l)
        .ok_or_else(|| corr("section loop no longer resolves"))?
        .boundary
    else {
        return Err(corr("section loop is empty"));
    };
    body.loop_cycle(first)
        .ok_or_else(|| corr("section loop not walkable"))
}

/// The record-keyed alignment, antiparallel: `ob[j]` runs `a_j →
/// a_{j+1}`, so the ring half-edge paired with it runs from a
/// correspondent of `a_j` to one of `a_{j−1}`.
fn align<T: Decide>(
    body: &Body<T>,
    (a_face, b_face): (FaceKey, FaceKey),
    ob: &[HalfEdgeKey],
    ring_cycle: &[HalfEdgeKey],
    vmap: &SeamCorrespondence,
) -> Result<Vec<HalfEdgeKey>, BooleanError> {
    let corr = |what| BooleanError::ZipCorrespondence { what };
    let n = ob.len();
    if ring_cycle.len() != n {
        return Err(corr("seam cycles differ in length"));
    }
    let correspondents = |v: VertexKey| {
        vmap.get(&v)
            .ok_or_else(|| corr("outer seam vertex has no recorded B correspondent"))
    };
    let mut rs: Vec<HalfEdgeKey> = Vec::with_capacity(n);
    for j in 0..n {
        let from = correspondents(start_of(body, ob[j])?)?;
        let to = correspondents(start_of(body, ob[(j + n - 1) % n])?)?;
        let mut leaves = false;
        let mut matched = None;
        for &rhe in ring_cycle {
            if !from.contains(&start_of(body, rhe)?) {
                continue;
            }
            leaves = true;
            let end = body
                .half_edge_end(rhe)
                .ok_or_else(|| corr("ring half-edge has no end"))?;
            if to.contains(&end) && matched.replace(rhe).is_some() {
                return Err(corr("a seam half-edge has two ring matches"));
            }
        }
        match matched {
            Some(rhe) if rs.contains(&rhe) => {
                return Err(corr("a ring half-edge matches two seam half-edges"));
            }
            Some(rhe) => rs.push(rhe),
            None if leaves => return Err(BooleanError::SeamOrientation { a_face, b_face }),
            None => return Err(corr("corresponding ring half-edge missing")),
        }
    }
    Ok(rs)
}

/// **A one-vertex seam edge's sense**, which [`align`]'s vertex test
/// cannot read: a whole section conic through one site starts and ends
/// at that site, so a co-wound ring half there leaves the right vertex
/// and arrives at the right one too. The paired halves must run against
/// each other along their carriers there, read off their tangents at the
/// site (`bool_zip_one_vertex_sense`); a half that runs with the outer
/// one is [`BooleanError::SeamOrientation`]. A half between two vertices
/// is [`align`]'s and passes.
fn one_vertex_sense<T: Decide>(
    body: &Body<T>,
    (a_face, b_face): (FaceKey, FaceKey),
    (outer, ring): (HalfEdgeKey, HalfEdgeKey),
    tol: Tol,
) -> Result<(), BooleanError> {
    let corr = |what| BooleanError::ZipCorrespondence { what };
    let end = body
        .half_edge_end(outer)
        .ok_or_else(|| corr("seam half-edge has no end"))?;
    if start_of(body, outer)? != end {
        return Ok(());
    }
    // The tangent a half leaves its start along: its carrier's at the
    // start parameter, reversed on the minus half.
    let leaving = |he: HalfEdgeKey| -> Result<geom_core::Vec3<T>, BooleanError> {
        let edge = body
            .get_half_edge(he)
            .ok_or_else(|| corr("seam half-edge no longer resolves"))?
            .edge;
        let data = body
            .get_edge(edge)
            .ok_or_else(|| corr("seam edge no longer resolves"))?;
        let curve = body
            .edge_curve_linked(edge, data)
            .certified()
            .ok_or_else(|| corr("a one-vertex seam edge has no certified curve"))?;
        let (t0, t1) = curve.params();
        Ok(if data.he_plus == he {
            curve.carrier().deriv(t0)
        } else {
            -curve.carrier().deriv(t1)
        })
    };
    let band = geom_core::Band::linear(tol).map_err(|_| corr("the zip's band does not read"))?;
    let margin = geom_core::Margin::of(leaving(outer)?.dot(leaving(ring)?));
    match crate::validate::decide("bool_zip_one_vertex_sense", margin, band) {
        Ok(geom_core::Sign::Negative) => Ok(()),
        Ok(_) => Err(BooleanError::SeamOrientation { a_face, b_face }),
        Err(diag) => Err(BooleanError::Escalated {
            decision: super::BooleanDecision::SelfCheck(super::SelfCheck::SeamSense),
            diag,
        }),
    }
}

/// The vertex a half-edge starts at.
fn start_of<T: Decide>(body: &Body<T>, he: HalfEdgeKey) -> Result<VertexKey, BooleanError> {
    Ok(body
        .get_half_edge(he)
        .ok_or(BooleanError::ZipCorrespondence {
            what: "seam half-edge no longer resolves",
        })?
        .start)
}

#[cfg(test)]
mod cone_rows {
    //! **The cone reading** ([`cones`]) on a vertex holding three runs,
    //! two in one cone: the shape where reading the cycles the other way
    //! round, or a B run with its own corner's A run, splits the vertex
    //! wrongly. No prism pose the batteries build reaches it (a pierce
    //! leaves a copy per run), so it is pinned here.
    //!
    //! The union of a cube and two reflex corners `P`, `Q` touching only
    //! at `v`, the cube's face through `v`: `P` crosses the face in two
    //! sectors, `Q` in one between them, corners `0` (`P`), `1` (`Q`),
    //! `2` (`P`) round the cube's vertex. The cube's runs follow them:
    //! run `0` from corner 0 to 1, run `1` from 1 to 2, run `2` from 2 to
    //! 0. Below the face `P` is one band joining its two sectors, which
    //! parts the cube's lower side: runs 0 and 1 (with `Q` between them)
    //! bound one cone, run 2 the other. `P` holds corners 0 and 2, `Q`
    //! corner 1.

    use super::cones;
    use std::collections::BTreeSet;

    /// The cube vertex's step: corner 0 → 1 → 2 → 0.
    const CUBE: [usize; 3] = [1, 2, 0];
    /// The corners' other side: `P` steps 0 ↔ 2, `Q` holds 1 alone.
    const PINCHED: [usize; 3] = [2, 1, 0];

    /// The runs at a vertex grouped by cone.
    fn grouping(labels: &[usize], runs: &[usize]) -> BTreeSet<BTreeSet<usize>> {
        let cones: BTreeSet<usize> = runs.iter().map(|&k| labels[k]).collect();
        cones
            .into_iter()
            .map(|c| runs.iter().copied().filter(|&k| labels[k] == c).collect())
            .collect()
    }

    fn want() -> BTreeSet<BTreeSet<usize>> {
        BTreeSet::from([BTreeSet::from([0, 1]), BTreeSet::from([2])])
    }

    /// The cube as A: its vertex's runs 0 and 1 are one cone, run 2
    /// another. Red if the cycles are read as `σ_A ∘ σ_B`.
    #[test]
    fn the_cube_as_a_holds_two_runs_of_one_cone() {
        let (cone_a, _) = cones(&CUBE, &PINCHED);
        assert_eq!(grouping(&cone_a, &[0, 1, 2]), want());
    }

    /// The cube as B: the same grouping on its B runs. Red if a B run
    /// takes the cone of the A run starting at its own corner.
    #[test]
    fn the_cube_as_b_holds_two_runs_of_one_cone() {
        let (_, cone_b) = cones(&PINCHED, &CUBE);
        assert_eq!(grouping(&cone_b, &[0, 1, 2]), want());
    }

    /// `P`'s two runs lie one in each cone and `Q`'s in the cone of the
    /// cube's runs 0 and 1, in either order.
    #[test]
    fn the_pinched_operand_parts_its_runs() {
        let (cube, pinched) = cones(&CUBE, &PINCHED);
        assert_ne!(pinched[0], pinched[2], "cube as A: P's runs");
        assert_eq!(pinched[1], cube[0], "cube as A: Q's run");
        let (pinched, cube) = cones(&PINCHED, &CUBE);
        assert_ne!(pinched[0], pinched[2], "cube as B: P's runs");
        assert_eq!(pinched[1], cube[0], "cube as B: Q's run");
    }
}

#[cfg(test)]
mod tests {
    #![allow(clippy::unwrap_used, clippy::panic)]

    use super::{Fusions, peel};
    use crate::entity::VertexKey;
    use slotmap::SlotMap;

    /// **A vertex's cones peel one group at a time, read before any is
    /// written** ([`peel`]), runs as pair indices round the vertex, each
    /// run's cone given:
    /// - cones apart, each together: each later group in turn, the first
    ///   staying (the one-pass split's order);
    /// - a cone nested between two runs of another, itself between two
    ///   runs of a third: the innermost first, which leaves the next one
    ///   together, and then the group after the first;
    /// - two cones alternating: none.
    #[test]
    fn a_vertexs_cones_peel_innermost_first_and_refuse_alternating() {
        type Row = (&'static str, Vec<usize>, Option<Vec<(usize, usize)>>);
        let rows: [Row; 3] = [
            ("apart", vec![0, 0, 1, 2, 2], Some(vec![(2, 3), (3, 0)])),
            (
                "nested twice",
                vec![0, 1, 2, 1, 0],
                Some(vec![(2, 3), (4, 1)]),
            ),
            ("alternating", vec![0, 1, 0, 1], None),
        ];
        for (what, cone_of, want) in rows {
            let at: Vec<usize> = (0..cone_of.len()).collect();
            assert_eq!(peel(&at, &cone_of), want, "{what}");
        }
    }

    /// **A fusion list refuses every row that would fold a key onto a
    /// dead one, and folds a well-ordered chain through every hop.** A
    /// row that keeps a key an earlier row killed, kills one twice, or
    /// fuses a key into itself refuses `LineageRow` as it is pushed or
    /// extended, in every build, and leaves the list as it was. Red if
    /// the check moves to a reader or behind a `debug_assert!`: the
    /// corrupt rows would push, and `survivor` would answer the dead
    /// `a` for `c`.
    #[test]
    fn a_fusion_list_refuses_a_row_naming_a_killed_key() {
        let mut arena: SlotMap<VertexKey, ()> = SlotMap::with_key();
        let [a, b, c, d] = [(); 4].map(|()| arena.insert(()));
        use crate::lineage::{LineageRows, RowDefect};
        let refusal = |r: Result<(), crate::lineage::RowRefusal>| match r {
            Err(crate::lineage::RowRefusal {
                rows: LineageRows::Fusions,
                defect,
                key,
            }) => (defect, key),
            other => panic!("a corrupt row must refuse LineageRow, got {other:?}"),
        };
        let killed = |v| (RowDefect::NamesRetired, super::super::Cell::Vertex(v));

        let mut chain = Fusions::default();
        chain.push((a, b)).unwrap();
        chain.push((b, c)).unwrap();
        assert_eq!(
            [a, b, c, d].map(|v| chain.survivor(v)),
            [c, c, c, d],
            "a chain folds through every hop, and an unfused key is its own"
        );

        let mut fusions = Fusions::default();
        fusions.push((a, b)).unwrap();
        for (row, what, case) in [
            ((c, a), killed(a), "keeps a killed key"),
            ((a, c), killed(a), "kills a key twice"),
            (
                (d, d),
                (RowDefect::SelfRow, super::super::Cell::Vertex(d)),
                "fuses a key into itself",
            ),
        ] {
            assert_eq!(refusal(fusions.push(row)), what, "pushed: {case}");
            let mut later = Fusions::default();
            if row.0 != row.1 {
                later.push(row).unwrap();
                assert_eq!(refusal(fusions.extend(&later)), what, "extended: {case}");
            }
            assert_eq!(fusions.rows(), [(a, b)], "{case}: the list is as it was");
            assert_eq!(fusions.survivor(c), c, "{case}: c folds onto no dead key");
        }
    }

    /// **An extend that refuses at a later row appends none of its
    /// rows.** `later`'s first row is sound against `fusions` and its
    /// second keeps the key `fusions` killed. Red if `extend` pushes
    /// row by row into the list it extends: the sound first row would
    /// stay, and `d` would fold onto `e`.
    #[test]
    fn an_extend_that_refuses_partway_leaves_the_list_as_it_was() {
        let mut arena: SlotMap<VertexKey, ()> = SlotMap::with_key();
        let [a, b, c, d, e] = [(); 5].map(|()| arena.insert(()));
        let mut fusions = Fusions::default();
        fusions.push((a, b)).unwrap();
        let mut later = Fusions::default();
        later.push((d, e)).unwrap();
        later.push((c, a)).unwrap();
        assert!(
            matches!(
                fusions.extend(&later),
                Err(crate::lineage::RowRefusal {
                    rows: crate::lineage::LineageRows::Fusions,
                    defect: crate::lineage::RowDefect::NamesRetired,
                    ..
                })
            ),
            "the second row keeps the killed a"
        );
        assert_eq!(fusions.rows(), [(a, b)], "no row of a refused extend stays");
        assert_eq!(fusions.survivor(d), d, "d is not fused");
        fusions.push((d, e)).unwrap();
        assert_eq!(
            fusions.rows(),
            [(a, b), (d, e)],
            "the list still takes rows"
        );
    }
}
