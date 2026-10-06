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
//! vertex the seam meets twice (a welded pinch, with one correspondent
//! per meeting) is told apart by the ring run's other end. A ring that
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
/// correspondents. One each, except a welded pinch, which a seam meets
/// once per pierce it fused.
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

/// **A pinch is one vertex per cone** (Ev, PR 4057): before any zip,
/// each operand vertex the seams meet is split so that it holds one cone
/// of the result's boundary.
///
/// The zips fuse each seam's vertex pairs, seam after seam, and a
/// result vertex is one cycle of the corners they join round a point.
/// At an operand vertex `x`, each of its section corners on a seam
/// starts a *run*: `x`'s corners from that section half-edge round to
/// the next one. Where the seams are removed, an A run continues into
/// the B run whose ring half-edge the zip glues to the A section corner
/// it ends at, and that B run into the next A run the same way. So the
/// result's cones are the cycles of `σ_B ∘ σ_A` over the pairs, `σ`
/// stepping from a run to the next section corner round its vertex. A
/// vertex whose runs lie in several cones is a pinch: `mev_null`, which
/// keeps its new vertex on the old one's point key, moves each cone's
/// runs but the first to a vertex of their own. The null edge it leaves
/// lies between two section corners, on the section faces the zips
/// consume, and is killed there: by `kef` between two section faces
/// (merging their seams into one), or by `kemr` within one section loop,
/// whose split-off loop `mfkrh` promotes to a section face of its own.
///
/// The seams are then re-paired: an A section edge is the B ring edge
/// it zips with (`align`: `ob[j]` with the ring half-edge after `rs[j]`
/// in its seam), so each A section face pairs with the B section face
/// holding those edges, and `vmap` is rebuilt from the same edges.
/// Returns the seams to zip.
///
/// # Errors
///
/// [`BooleanError::ZipCorrespondence`] where a vertex's cones interleave
/// round it, where the split faces do not pair one to one, or where a
/// cone still fuses a vertex to itself (the seams meet one cone twice at
/// one vertex of each operand).
/// Each vertex's section corners, as pair indices in orbit order.
type RunsAt = BTreeMap<VertexKey, Vec<usize>>;

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
    let n = pairs.len();
    // σ for one side: each pair's next section corner round its vertex,
    // and each vertex's runs in orbit order.
    let sigma = |body: &Body<T>,
                 he_of: &dyn Fn(usize) -> HalfEdgeKey|
     -> Result<(Vec<usize>, RunsAt), BooleanError> {
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
            return Err(corr(
                "a section half-edge is missing from its vertex's orbit",
            ));
        }
        Ok((step, runs))
    };
    let (sigma_a, runs_a) = sigma(body, &|k| pairs[k].0)?;
    let (sigma_b, runs_b) = sigma(body, &|k| pairs[k].1)?;
    // The cones: cycles of σ_B ∘ σ_A over the pairs, by A run; a B run
    // is in the cone of the A run that ends at its pair's corner.
    let mut cone = vec![usize::MAX; n];
    for k0 in 0..n {
        let mut k = k0;
        while cone[k] == usize::MAX {
            cone[k] = k0;
            k = sigma_b[sigma_a[k]];
        }
    }
    let mut cone_b = vec![usize::MAX; n];
    for k in 0..n {
        cone_b[sigma_a[k]] = cone[k];
    }
    // Split each vertex per cone, its first cone's runs staying.
    let mut a_sections: Vec<FaceKey> = seams.iter().map(|&(a, _)| a).collect();
    let mut b_sections: Vec<FaceKey> = seams.iter().map(|&(_, b)| b).collect();
    let mut moved = false;
    for (runs, cone_of, he_of, sections) in [
        (
            &runs_a,
            &cone,
            &(|k: usize| pairs[k].0) as &dyn Fn(usize) -> HalfEdgeKey,
            &mut a_sections,
        ),
        (
            &runs_b,
            &cone_b,
            &(|k: usize| pairs[k].1) as &dyn Fn(usize) -> HalfEdgeKey,
            &mut b_sections,
        ),
    ] {
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
                return Err(corr("a vertex's cones interleave round it"));
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
    }
    // Re-pair: `ob[k]` zips with `rs[next[k]]`.
    let face_of = |body: &Body<T>, he: HalfEdgeKey| {
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
    for &fa in a_sections.iter().filter(|_| moved) {
        let partners: BTreeSet<FaceKey> = (0..n)
            .filter(|&k| face_of(body, pairs[k].0) == fa)
            .map(|k| face_of(body, pairs[next[k]].1))
            .collect();
        match (partners.first(), partners.len()) {
            (Some(&fb), 1) if b_sections.contains(&fb) => paired.push((fa, fb)),
            _ => return Err(corr("a split section face does not pair one to one")),
        }
    }
    let partnered: BTreeSet<FaceKey> = paired.iter().map(|&(_, b)| b).collect();
    if moved && (partnered.len() != paired.len() || partnered.len() != b_sections.len()) {
        return Err(corr("a split section face does not pair one to one"));
    }
    // The correspondence, from the edges the zips join; and no cone may
    // fuse a vertex to itself.
    let end_of = |body: &Body<T>, he: HalfEdgeKey| {
        body.half_edge_end(he)
            .ok_or_else(|| corr("a seam half-edge has no end"))
    };
    let mut root: BTreeMap<VertexKey, VertexKey> = BTreeMap::new();
    let find = |root: &BTreeMap<VertexKey, VertexKey>, mut v: VertexKey| {
        while let Some(&up) = root.get(&v) {
            v = up;
        }
        v
    };
    let mut rebuilt = SeamCorrespondence::new();
    for k in 0..n {
        let (a, b) = (start_of(body, pairs[k].0)?, end_of(body, pairs[next[k]].1)?);
        rebuilt.entry(a).or_default().insert(b);
        let (ra, rb) = (find(&root, a), find(&root, b));
        if ra == rb {
            return Err(corr("a cone the seams meet twice fuses a vertex to itself"));
        }
        root.insert(rb, ra);
    }
    if !moved {
        return Ok(seams.to_vec());
    }
    *vmap = rebuilt;
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
