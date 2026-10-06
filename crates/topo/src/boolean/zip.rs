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
//! other end: a pinch weld's vertex (`finish::weld_pinches`), with one
//! per pierce it fused, and after a split ([`split_cones`]) a vertex
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
//! cones of the result is split per cone, on its own point key. The
//! transient edge each split leaves lies on the section faces the zips
//! consume, so no kept face's topology changes. The split and the zip
//! read one alignment ([`align`]).

use std::collections::{BTreeMap, BTreeSet};

use geom_core::Decide;

use super::BooleanError;
use crate::body::Body;
use crate::entity::{EntityId, FaceKey, HalfEdgeKey, LoopBoundary, LoopKey, VertexKey};
use crate::euler::{FaceSurface, MefSite, MevSite};
use crate::euler_ring::MekrSite;
use crate::live::{linked, proven};
use geom_brep::EdgeCurveSpec;
use geom_core::Tol;

/// The seam vertex correspondence: each A-side vertex → its B-side
/// correspondents. One each, except where the seams meet a vertex at
/// several corners: a pinch weld's vertex holds one per pierce it fused,
/// and once [`split_cones`] rebuilds the map from the edges the zips
/// join, a vertex holds one per corner.
pub(super) type SeamCorrespondence = BTreeMap<VertexKey, BTreeSet<VertexKey>>;

/// The vertex `v` survives as through the fusions `(dead, kept)`, in
/// the order they were made: the one reading of a fusion list. Every
/// writer appends a row as its kev runs, so a key is dead from its row
/// on and no later row names it.
pub(super) fn survivor(merges: &[(VertexKey, VertexKey)], v: VertexKey) -> VertexKey {
    debug_assert!(
        fusions_well_ordered(merges),
        "a fusion row names a key an earlier row killed: {merges:?}"
    );
    merges
        .iter()
        .fold(v, |at, &(dead, kept)| if at == dead { kept } else { at })
}

/// [`survivor`], refusing a corrupt fusion list in every build: a row
/// that keeps a key an earlier row killed, kills one twice, or fuses a
/// key into itself would fold `v` onto a dead key.
///
/// # Errors
///
/// [`BooleanError::JoinDesync`] on such a list.
pub(super) fn survivor_checked(
    merges: &[(VertexKey, VertexKey)],
    v: VertexKey,
) -> Result<VertexKey, BooleanError> {
    if !fusions_well_ordered(merges) {
        return Err(BooleanError::JoinDesync {
            what: "a fusion row names a key an earlier row killed",
        });
    }
    Ok(survivor(merges, v))
}

/// Whether every fusion row's keys are live when it is made: no row
/// keeps or kills a key an earlier row killed, and none fuses a key
/// into itself.
fn fusions_well_ordered(merges: &[(VertexKey, VertexKey)]) -> bool {
    let mut dead_so_far = BTreeSet::new();
    merges.iter().all(|&(dead, kept)| {
        !dead_so_far.contains(&kept) && dead_so_far.insert(dead) && dead != kept
    })
}

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
    /// Across one ring of `face`: the `mef` divides the hole, and the
    /// face it divides off is a hole too, so `kfmrh` returns it to
    /// `face` as a ring: two holes meeting at the vertex.
    Hole {
        face: FaceKey,
        he1: HalfEdgeKey,
        he2: HalfEdgeKey,
    },
}

/// **Fuses two coincident vertices**: a zero-length edge between them at
/// `p` (the canonical self-loop carrier, whose certification requires
/// the pair bitwise coincident), collapsed by a `kev` that keeps the
/// merged fan's carriers, each re-certified at the kept vertex under the
/// run's band. Returns the fusion `(dead, kept)` and, for a chord, the
/// face it divided off (a hole's goes back to its face as a ring);
/// `desync` names a joint that no longer resolves.
pub(super) fn fuse_by_joint<T: Decide + crate::props::AtRestPolicy>(
    body: &mut Body<T>,
    joint: Joint,
    p: geom_core::Point3<T>,
    desync: fn(&'static str) -> BooleanError,
    tol: Tol,
) -> Result<((VertexKey, VertexKey), Option<FaceKey>), BooleanError> {
    let carrier = EdgeCurveSpec::self_loop_circle_at(p);
    let hole_face = match joint {
        Joint::Hole { face, .. } => Some(face),
        _ => None,
    };
    let (he, made) = match joint {
        Joint::Loops { target, ring } => (
            body.mekr(MekrSite::Cycles { target, ring }, carrier, tol)?
                .he_plus,
            None,
        ),
        Joint::Chord { he1, he2 } | Joint::Hole { he1, he2, .. } => {
            let made = body.mef(
                MefSite::Chords { he1, he2 },
                carrier,
                FaceSurface::Inherit,
                tol,
            )?;
            (made.he_plus, Some(made.face))
        }
    };
    let hole = hole_face.zip(made);
    let kept = body
        .get_half_edge(he)
        .ok_or_else(|| desync("a joint half-edge no longer resolves"))?
        .start;
    let dead = body
        .half_edge_end(he)
        .ok_or_else(|| desync("a joint half-edge has no end"))?;
    body.kev_describing(he, &[], tol)?;
    if let Some((face, divided)) = hole {
        body.kfmrh_minting(face, divided, tol)?;
        return Ok(((dead, kept), None));
    }
    Ok(((dead, kept), made))
}

/// What one seam zip did to the arena — the F9-style record the op
/// stage consumes (M3 PR 6a): every vertex fusion (dead key → kept
/// key, the D5 descendant map's zip rows) and the surviving seam
/// edges (the D6 description pass's worklist — tracked lineage, never
/// a post-hoc scan).
#[derive(Debug, Default)]
pub(super) struct ZipReport {
    /// Vertex fusions in zip order: `(dead, kept)` per zipped pair.
    pub vertex_merges: Vec<(VertexKey, VertexKey)>,
    /// The seam edges surviving the zip (the outer cycle's edges), in
    /// cycle order.
    pub seam_edges: Vec<crate::entity::EdgeKey>,
    /// Seam edges KILLED by this zip as R-interior structure (the
    /// already-fused runs a slit zip consumes — e.g. the meridian
    /// seams of a closed cosurface band, which are segments AND
    /// interior to the contact region). Empty for a plain
    /// [`zip_seam`].
    pub interior_edges: Vec<crate::entity::EdgeKey>,
    /// Edge fusions, `(dead, kept)`: each ring edge the zip kills and
    /// the seam edge it lay on, which keeps its key.
    pub edge_merges: Vec<(crate::entity::EdgeKey, crate::entity::EdgeKey)>,
}

/// The order the loopglue zip fuses a seam's `n` vertex pairs in:
/// pair 0 first (its `mekr` joins the two loops), then `n − 1` down to
/// 1 (each a `mef` and the `kef` of the strip behind it).
pub(crate) fn fusion_order(n: usize) -> impl Iterator<Item = usize> {
    core::iter::once(0).chain((1..n).rev())
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
/// then re-paired ([`repair`]) and `vmap` is rebuilt from the edges the
/// zips join. Returns the seams to zip.
///
/// The split writes `body` before the re-pair can refuse: `body` is the
/// boolean's working copy, which a refusal drops whole.
///
/// # Errors
///
/// [`BooleanError::ZipCorrespondence`] where a vertex's cones interleave
/// round it, where the split faces do not pair one to one, or where a
/// cone still fuses a vertex to itself (the seams meet one cone twice at
/// one vertex of each operand).
pub(super) fn split_cones<T: Decide + crate::props::AtRestPolicy>(
    body: &mut Body<T>,
    seams: &[(FaceKey, FaceKey)],
    vmap: &mut SeamCorrespondence,
    tol: Tol,
) -> Result<Vec<(FaceKey, FaceKey)>, BooleanError> {
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
    let moved_a = split_side(body, &runs_a, &cone_a, &he_a, &mut a_sections, tol)?;
    let moved_b = split_side(body, &runs_b, &cone_b, &he_b, &mut b_sections, tol)?;
    let moved = moved_a || moved_b;
    let paired = if moved {
        repair(body, &pairs, &next, &a_sections, &b_sections)?
    } else {
        seams.to_vec()
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
    let mut root: BTreeMap<VertexKey, VertexKey> = BTreeMap::new();
    let find = |root: &BTreeMap<VertexKey, VertexKey>, mut v: VertexKey| {
        while let Some(&up) = root.get(&v) {
            v = up;
        }
        v
    };
    for &(a, b) in &fused {
        let (ra, rb) = (find(&root, a), find(&root, b));
        if ra == rb {
            return Err(corr("a cone the seams meet twice fuses a vertex to itself"));
        }
        root.insert(rb, ra);
    }
    if !moved {
        return Ok(paired);
    }
    let mut rebuilt = SeamCorrespondence::new();
    for (a, b) in fused {
        rebuilt.entry(a).or_default().insert(b);
    }
    *vmap = rebuilt;
    Ok(paired)
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
/// staying; whether any vertex was split.
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
/// round it. No battery line reaches it: two cones' runs alternating
/// round one vertex would need their boundary cycles to cross there.
/// The Euler operators' refusals, typed.
fn split_side<T: Decide + crate::props::AtRestPolicy>(
    body: &mut Body<T>,
    runs: &RunsAt,
    cone_of: &[usize],
    he_of: &dyn Fn(usize) -> HalfEdgeKey,
    sections: &mut Vec<FaceKey>,
    tol: Tol,
) -> Result<bool, BooleanError> {
    let mut moved = false;
    for at in runs.values() {
        let r = at.len();
        let cones_here: BTreeSet<usize> = at.iter().map(|&k| cone_of[k]).collect();
        if cones_here.len() < 2 {
            continue;
        }
        let edges = (0..r)
            .filter(|&i| cone_of[at[i]] != cone_of[at[(i + 1) % r]])
            .count();
        if edges != cones_here.len() {
            return Err(BooleanError::ZipCorrespondence {
                what: "a vertex's cones interleave round it",
            });
        }
        // Group starts: the runs whose cone differs from the run before.
        let starts: Vec<usize> = (0..r)
            .filter(|&i| cone_of[at[i]] != cone_of[at[(i + r - 1) % r]])
            .collect();
        for (g, &s) in starts.iter().enumerate().skip(1) {
            let end = starts[(g + 1) % starts.len()];
            let made = body.mev_null(
                MevSite::Fan {
                    he1: he_of(at[s]),
                    he2: he_of(at[end]),
                },
                crate::NewVertexSide::Above,
            )?;
            moved = true;
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
    Ok(moved)
}

/// The seams after a split: an A section edge is the B ring edge it zips
/// with (`align`: `ob[k]` with `rs[next[k]]`), so each A section face
/// pairs with the B section face holding those edges.
///
/// # Errors
///
/// [`BooleanError::ZipCorrespondence`] where the split faces do not pair
/// one to one.
fn repair<T: Decide>(
    body: &Body<T>,
    pairs: &[(HalfEdgeKey, HalfEdgeKey)],
    next: &[usize],
    a_sections: &[FaceKey],
    b_sections: &[FaceKey],
) -> Result<Vec<(FaceKey, FaceKey)>, BooleanError> {
    let refuse = || BooleanError::ZipCorrespondence {
        what: "a split section face does not pair one to one",
    };
    let face_of = |he: HalfEdgeKey| {
        let l = proven(&body.half_edges, he, EntityId::HalfEdge).parent_loop;
        linked(
            &body.loops,
            l,
            EntityId::Loop,
            EntityId::HalfEdge(he),
            "parent_loop",
        )
        .face
    };
    let mut paired: Vec<(FaceKey, FaceKey)> = Vec::new();
    for &fa in a_sections {
        let partners: BTreeSet<FaceKey> = (0..pairs.len())
            .filter(|&k| face_of(pairs[k].0) == fa)
            .map(|k| face_of(pairs[next[k]].1))
            .collect();
        match (partners.first(), partners.len()) {
            (Some(&fb), 1) if b_sections.contains(&fb) => paired.push((fa, fb)),
            _ => return Err(refuse()),
        }
    }
    let partnered: BTreeSet<FaceKey> = paired.iter().map(|&(_, b)| b).collect();
    if partnered.len() != paired.len() || partnered.len() != b_sections.len() {
        return Err(refuse());
    }
    Ok(paired)
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
    let mut report = ZipReport::default();

    // ---- Fuse: B's section face becomes a ring of A's. ----
    let fused = body.kfmrh_minting(a_face, b_face, tol)?;
    let ring = fused.ring;
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
    let n = ob.len();
    let ring_edges = rs
        .iter()
        .map(|&r| {
            body.get_half_edge(r)
                .map(|h| h.edge)
                .ok_or_else(|| corr("ring half-edge no longer resolves"))
        })
        .collect::<Result<Vec<_>, _>>()?;

    // ---- The loopglue zip (the reassembly-oracle sequence, driven by
    // records): pair 0 via mekr (kills the ring loop) + kev; pairs
    // n−1 … 1 via mef + kev + kef(rs[j+1 mod n]); final kef(rs[1]). ----
    let point_of =
        |body: &Body<T>, he: HalfEdgeKey| -> Result<geom_core::Point3<T>, BooleanError> {
            let v = start_of(body, he)?;
            body.get_vertex(v)
                .and_then(|vd| body.get_point(vd.point).copied())
                .ok_or_else(|| corr("seam vertex has no point"))
        };
    for j in fusion_order(n) {
        let pj = point_of(body, ob[j])?;
        let joint = if j == 0 {
            Joint::Loops {
                target: ob[0],
                ring: rs[0],
            }
        } else {
            Joint::Chord {
                he1: ob[j],
                he2: rs[j],
            }
        };
        let (merge, _) = fuse_by_joint(body, joint, pj, corr, tol)?;
        report.vertex_merges.push(merge);
        if j != 0 {
            body.kef_minting(rs[(j + 1) % n], tol)?;
        }
    }
    body.kef_minting(rs[1 % n], tol)?;
    for &he in &ob {
        let edge = body
            .get_half_edge(he)
            .ok_or_else(|| corr("surviving seam half-edge no longer resolves"))?
            .edge;
        report.seam_edges.push(edge);
    }
    // `rs[j]` runs between the correspondents of `ob[j]`'s start and
    // `ob[j - 1]`'s (`align`), so it lies on `ob[j - 1]`'s segment.
    report.edge_merges = ring_edges
        .into_iter()
        .enumerate()
        .map(|(j, dead)| (dead, report.seam_edges[(j + n - 1) % n]))
        .collect();
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
    //! leaves a copy per run, and a weld nests a pierce in one kept
    //! corner), so it is pinned here.
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
