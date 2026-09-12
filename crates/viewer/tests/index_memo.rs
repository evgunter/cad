//! **The index seam's picture is bit-for-bit the fresh one, edit after
//! edit** — the differential the pick-index memo is built under.
//!
//! The seam ([`InlineIndexer`]) is where the previous generation's
//! index lives, so it is where any reuse across edits happens. Whatever
//! it keeps between builds, the picture it answers has ONE definition:
//! the index the plain door ([`PickIndex::build`]) builds from the same
//! landed run, whose meshes are `mesh::tessellate` of each root body.
//! Every row here opens a document, indexes it through the seam, then
//! runs a sequence of edits — change a parameter, change another,
//! revert the first — and after every landing asserts that the seam's
//! meshes are byte-identical to the plain door's (the D9 goldens'
//! digest, over every position, patch and boundary) and that a fixed
//! set of rays picks the same faces on both.
//!
//! The corpus is `editor-core`'s (`crate::corpus`, every parametric
//! document through its own bump edit), plus the tour's gallery ring.

// Panicking is a test's failure mechanism (workspace lint note).
#![allow(clippy::expect_used, clippy::panic, clippy::unwrap_used)]

use std::collections::BTreeMap;
use std::sync::Arc;

use bvh::{Aabb, Bvh, Ray};
use editor_core::{
    Dimension, DocEdit, Expr, HitTestError, NodePick, ProfileDoc, RecipeNodeId, SlotId, StableName,
    unparse,
};
use pncad::geom_core::{Point3, Tol, Vec3};
use pncad::mesh::Mesh;
use viewer::evalseam::{IndexDone, IndexRequest, IndexService, InlineIndexer, MemoReport};
use viewer::pickindex::PickIndex;
use viewer::scene::DisplayTolerance;
use viewer::session::{DocSession, SessionOp};

use crate::common;
use crate::corpus;

/// Coarse on purpose: the rows are about reuse across edits, not about
/// mesh density, and the corpus has million-triangle documents at the
/// application's δ.
fn delta() -> DisplayTolerance {
    DisplayTolerance::new(2.0e-3).expect("a positive delta")
}

fn fnv(h: &mut u64, x: u64) {
    for b in x.to_le_bytes() {
        *h ^= u64::from(b);
        *h = h.wrapping_mul(0x0100_0000_01b3);
    }
}

fn fnv_str(h: &mut u64, s: &str) {
    fnv(h, s.len() as u64);
    for b in s.bytes() {
        fnv(h, u64::from(b));
    }
}

/// Every byte of a mesh value, the way `d9_mesh_goldens` digests one:
/// positions by bit pattern, patches (face key, triangles), boundaries
/// (edge key, polyline ids, endpoint vertex keys).
fn digest(m: &Mesh) -> u64 {
    let mut h: u64 = 0xcbf2_9ce4_8422_2325;
    fnv(&mut h, m.positions.len() as u64);
    for p in &m.positions {
        fnv(&mut h, p.x.to_bits());
        fnv(&mut h, p.y.to_bits());
        fnv(&mut h, p.z.to_bits());
    }
    fnv(&mut h, m.patches.len() as u64);
    for q in &m.patches {
        fnv_str(&mut h, &format!("{:?}", q.face));
        fnv(&mut h, q.triangles.len() as u64);
        for t in &q.triangles {
            fnv(&mut h, u64::from(t[0]));
            fnv(&mut h, u64::from(t[1]));
            fnv(&mut h, u64::from(t[2]));
        }
    }
    fnv(&mut h, m.boundaries.len() as u64);
    for b in &m.boundaries {
        fnv_str(&mut h, &format!("{:?}", b.edge));
        fnv(&mut h, b.points.len() as u64);
        for id in &b.points {
            fnv(&mut h, u64::from(*id));
        }
        fnv_str(&mut h, &format!("{:?}", b.start_vertex));
        fnv_str(&mut h, &format!("{:?}", b.end_vertex));
    }
    h
}

/// One edit as the session spells it.
#[derive(Clone, Debug)]
struct Edit {
    node: RecipeNodeId,
    slot: SlotId,
    text: String,
}

impl Edit {
    fn op(&self) -> SessionOp {
        SessionOp::SetSlotExpression {
            node: self.node,
            slot: self.slot,
            text: self.text.clone(),
        }
    }
}

/// The corpus document's own bump edit — every parametric corpus
/// document carries one — and the text that reverts it.
fn bump_of(c: &corpus::CorpusDoc) -> Option<(Edit, Edit)> {
    let DocEdit::SetParam { node, slot, expr } = c.bump.clone() else {
        return None;
    };
    let original = c.doc.node(node)?.expr(slot)?;
    Some((
        Edit {
            node,
            slot,
            text: unparse(&expr),
        },
        Edit {
            node,
            slot,
            text: unparse(original),
        },
    ))
}

/// A second parameter to change: the first literal length slot on a
/// node other than `not`, scaled — "change another", when the document
/// has another to change.
fn another_length_slot(doc: &ProfileDoc, not: RecipeNodeId) -> Option<Edit> {
    for &node in doc.order() {
        if node == not {
            continue;
        }
        let n = doc.node(node)?;
        for slot in n.slots() {
            let Some(expr) = n.expr(slot) else { continue };
            if expr.dim() != Dimension::Length {
                continue;
            }
            let Some(value) = expr.literal_value() else {
                continue;
            };
            if value == 0.0 {
                continue;
            }
            let scaled = Expr::literal(value * 1.015_625, Dimension::Length).ok()?;
            return Some(Edit {
                node,
                slot,
                text: unparse(&scaled),
            });
        }
    }
    None
}

/// The request for the session's landed run at `at`.
fn request_at(session: &DocSession, at: DisplayTolerance) -> IndexRequest {
    let (doc, _) = session.landed_pair().expect("a landed pair");
    IndexRequest {
        generation: session
            .landed_generation()
            .expect("a landed evaluation has a generation"),
        delta: at,
        doc: doc.clone(),
        evaluation: Arc::clone(session.evaluation_arc().expect("a landed run")),
        tol: session.tol(),
    }
}

/// A seam's answer for a request, waited for: the inline seam answers
/// inside `poll`, the threaded one when its worker is done.
fn answer(seam: &mut impl IndexService, request: IndexRequest) -> IndexDone {
    let generation = request.generation;
    seam.submit(request);
    for _ in 0..100_000 {
        if let Some(done) = seam.poll() {
            assert_eq!(done.generation, generation);
            return done;
        }
        std::thread::sleep(std::time::Duration::from_millis(1));
    }
    panic!("the seam never answered")
}

/// The seam's answer for the session's landed run at `at`: the index,
/// or the refusal (a failed or poisoned root is an ordinary editing
/// state).
fn seam_index_at(
    seam: &mut InlineIndexer,
    session: &DocSession,
    at: DisplayTolerance,
) -> Result<PickIndex, viewer::pickindex::PickIndexError> {
    answer(seam, request_at(session, at)).index
}

fn seam_index(
    seam: &mut InlineIndexer,
    session: &DocSession,
) -> Result<PickIndex, viewer::pickindex::PickIndexError> {
    seam_index_at(seam, session, delta())
}

/// What the seam's memo holds and did, after a build.
#[derive(Clone, Copy, Debug)]
struct MemoReading {
    nodes: usize,
    faces: usize,
    node_hits: usize,
    node_misses: usize,
    face_hits: usize,
    face_misses: usize,
    trees: usize,
    tree_hits: usize,
    tree_misses: usize,
}

fn reading(seam: &InlineIndexer) -> MemoReading {
    let memo = seam.memo();
    MemoReading {
        nodes: memo.len(),
        faces: memo.patches().len(),
        node_hits: memo.node_hits(),
        node_misses: memo.node_misses(),
        face_hits: memo.patches().hits(),
        face_misses: memo.patches().misses(),
        trees: memo.tree_len(),
        tree_hits: memo.tree_hits(),
        tree_misses: memo.tree_misses(),
    }
}

/// **The memo holds exactly the picture just built.** The counts the
/// seam's memo reports describe the picture just closed (the build
/// that just answered), and a memo that stopped evicting would hold
/// more faces than the picture has.
fn assert_memo_is_one_picture(name: &str, step: &str, seam: &InlineIndexer, index: &PickIndex) {
    let r = reading(seam);
    let faces = faces_of(index);
    let parts = index.parts().len();
    println!(
        "# {name} after {step}: {parts} parts / {faces} faces; memo nodes {} (hits {} misses {}), \
         faces {} (hits {} misses {}), trees {} (hits {} misses {})",
        r.nodes,
        r.node_hits,
        r.node_misses,
        r.faces,
        r.face_hits,
        r.face_misses,
        r.trees,
        r.tree_hits,
        r.tree_misses
    );
    assert_eq!(
        r.nodes, parts,
        "{name} after {step}: one memo entry per drawn (node, body)"
    );
    assert_eq!(
        r.node_hits + r.node_misses,
        parts,
        "{name} after {step}: every part was a node-level hit or miss"
    );
    // Faces the memo answered, plus faces it meshed, is the faces of
    // the recomputed parts; the reused parts' faces were kept, not
    // looked up. The memo then holds at most the picture's faces —
    // fewer only where two faces are bit-identical.
    assert!(
        r.faces <= faces,
        "{name} after {step}: the memo holds {} faces for a picture of {faces}",
        r.faces
    );
    assert!(r.faces >= 1 || faces == 0);
    // The per-patch pick trees are keyed like the patches and looked
    // up exactly where the patches are, so they hit and miss where
    // the patches do, and are evicted with them.
    assert_eq!(
        (r.tree_hits, r.tree_misses),
        (r.face_hits, r.face_misses),
        "{name} after {step}: the pick trees hit and miss where the patches do"
    );
    assert!(
        r.trees <= faces,
        "{name} after {step}: the memo holds {} pick trees for a picture of {faces}",
        r.trees
    );
    assert!(r.trees >= 1 || faces == 0);
}

/// The plain door's answer for the same run: the definition of the
/// picture.
fn fresh_index(session: &DocSession) -> Result<PickIndex, viewer::pickindex::PickIndexError> {
    let (doc, eval) = session.landed_pair().expect("a landed pair");
    let generation = session
        .landed_generation()
        .expect("a landed evaluation has a generation");
    PickIndex::build(doc, eval, generation, delta(), session.tol())
}

/// A fixed set of rays for the picture: the six axis rays through the
/// bounding box's centre and the eight corner-to-centre diagonals.
fn rays_for(index: &PickIndex) -> Vec<Ray> {
    let mut lo = Point3::new(f64::MAX, f64::MAX, f64::MAX);
    let mut hi = Point3::new(f64::MIN, f64::MIN, f64::MIN);
    for part in index.parts() {
        for p in &part.mesh().positions {
            lo = Point3::new(lo.x.min(p.x), lo.y.min(p.y), lo.z.min(p.z));
            hi = Point3::new(hi.x.max(p.x), hi.y.max(p.y), hi.z.max(p.z));
        }
    }
    let c = Point3::new(
        (lo.x + hi.x) * 0.5,
        (lo.y + hi.y) * 0.5,
        (lo.z + hi.z) * 0.5,
    );
    let ext = (hi - lo).norm().max(1e-3);
    let mut rays = Vec::new();
    for dir in [
        Vec3::new(1.0, 0.0, 0.0),
        Vec3::new(-1.0, 0.0, 0.0),
        Vec3::new(0.0, 1.0, 0.0),
        Vec3::new(0.0, -1.0, 0.0),
        Vec3::new(0.0, 0.0, 1.0),
        Vec3::new(0.0, 0.0, -1.0),
    ] {
        rays.push(Ray {
            origin: c - dir * (2.0 * ext),
            dir,
        });
    }
    let aabb = Aabb {
        min_x: lo.x,
        min_y: lo.y,
        min_z: lo.z,
        max_x: hi.x,
        max_y: hi.y,
        max_z: hi.z,
    };
    for corner in common::corners(&aabb) {
        let toward = c - corner;
        if toward.norm() > 0.0 {
            rays.push(Ray {
                origin: corner - toward,
                dir: toward,
            });
        }
    }
    rays
}

/// **The single-level reference.** One flat tree per part over EVERY
/// triangle of its mesh, patch-major in the mesh's patch order, and
/// the pick service's own loop over it verbatim: candidates in the
/// tree's order (ascending conservative entry, then flat position),
/// the early-out once the best hit is strictly below a candidate's
/// entry, the exact test, nearest `t` with ties to the lower
/// `(part, flat triangle)` position. Test-only. Whatever shape the
/// production index takes — one tree per mesh, a tree per patch under
/// a tree over the patches — its answer to every ray is this one's,
/// hit for hit, and the early-out is part of the definition: the
/// exact test can answer a `t` outside a grazed triangle's own box
/// (a ray in the triangle's plane), and which of those the loop
/// reaches before it breaks is decided by the candidate order.
struct FlatPart {
    tree: Bvh,
    corners: Vec<[Point3<f64>; 3]>,
    /// Flat triangle position → (patch position, triangle position).
    owner: Vec<(usize, usize)>,
}

struct FlatReference {
    parts: Vec<FlatPart>,
}

/// One reference hit: which part, patch and triangle, at what `t`.
#[derive(Clone, Copy, Debug)]
struct FlatHit {
    part: usize,
    /// Flat triangle position within the part.
    item: usize,
    patch: usize,
    t: f64,
}

impl FlatReference {
    fn of(index: &PickIndex) -> Self {
        let parts = index
            .parts()
            .iter()
            .map(|part| {
                let mesh = part.mesh();
                let mut corners = Vec::new();
                let mut owner = Vec::new();
                let mut boxes = Vec::new();
                for (pi, patch) in mesh.patches.iter().enumerate() {
                    for (ti, tri) in patch.triangles.iter().enumerate() {
                        let c = tri.map(|i| mesh.positions[i as usize]);
                        boxes.push(Aabb::from_points(c).expect("three points box"));
                        corners.push(c);
                        owner.push((pi, ti));
                    }
                }
                FlatPart {
                    tree: Bvh::build(&boxes),
                    corners,
                    owner,
                }
            })
            .collect();
        Self { parts }
    }

    /// The nearest hit and how many exact hits the loop saw at its
    /// `t` — the second is what says a ray of the tie-break row
    /// actually tied.
    fn pick(&self, ray: &Ray) -> (Option<FlatHit>, usize) {
        let mut best: Option<FlatHit> = None;
        let mut tied = 0;
        for (part, flat) in self.parts.iter().enumerate() {
            for cand in flat.tree.ray(ray) {
                if let Some(b) = &best
                    && b.t < cand.t_enter
                {
                    break;
                }
                let Some(t) = ray_triangle(ray, &flat.corners[cand.item]) else {
                    continue;
                };
                let (patch, _) = flat.owner[cand.item];
                let hit = FlatHit {
                    part,
                    item: cand.item,
                    patch,
                    t,
                };
                match &best {
                    Some(b) if t > b.t => {}
                    Some(b) if t == b.t => {
                        tied += 1;
                        if (part, cand.item) < (b.part, b.item) {
                            best = Some(hit);
                        }
                    }
                    _ => {
                        tied = 1;
                        best = Some(hit);
                    }
                }
            }
        }
        (best, tied)
    }
}

/// The exact ray/triangle test the pick service runs (Möller–Trumbore,
/// both-sided, closed boundaries, non-finite `t` refused), restated
/// here so the reference is a whole pick and not a call into the
/// service it checks. A change to the service's test — the guard
/// `work/docm/pick-grazing-ray-answer-depends-on-candidate-order.md`
/// asks for — must change both copies, or this pin reds on the rays
/// whose answer the guard moves.
fn ray_triangle(ray: &Ray, tri: &[Point3<f64>; 3]) -> Option<f64> {
    let e1: Vec3<f64> = tri[1] - tri[0];
    let e2: Vec3<f64> = tri[2] - tri[0];
    let p = ray.dir.cross(e2);
    let det = e1.dot(p);
    if det == 0.0 {
        return None;
    }
    let inv = 1.0 / det;
    let s = ray.origin - tri[0];
    let u = s.dot(p) * inv;
    if !(0.0..=1.0).contains(&u) {
        return None;
    }
    let q = s.cross(e1);
    let v = ray.dir.dot(q) * inv;
    if !(v >= 0.0 && u + v <= 1.0) {
        return None;
    }
    let t = e2.dot(q) * inv;
    (t >= 0.0 && t.is_finite()).then_some(t)
}

/// **The tie-break row**: rays aimed exactly at the points two or more
/// patches share — a boundary polyline's first point (a vertex or
/// chord point every incident face's triangles have as a corner) and
/// the midpoint of its first segment (a point on the shared edge) —
/// along the six axis directions from outside the picture. A hit
/// there is a hit for every incident triangle at one `t`, across
/// patches and, where bodies touch, across parts: the case only the
/// tie-break decides.
fn tie_rays_for(index: &PickIndex) -> Vec<Ray> {
    let mut ext = 0.0f64;
    let mut targets = Vec::new();
    for part in index.parts() {
        let mesh = part.mesh();
        for p in &mesh.positions {
            ext = ext.max(p.x.abs()).max(p.y.abs()).max(p.z.abs());
        }
        // At most ~40 boundaries per part, spread over the polyline list.
        let stride = mesh.boundaries.len().div_ceil(40).max(1);
        for boundary in mesh.boundaries.iter().step_by(stride) {
            let pts: Vec<Point3<f64>> = boundary
                .points
                .iter()
                .map(|&i| mesh.positions[i as usize])
                .collect();
            if let Some(&first) = pts.first() {
                targets.push(first);
            }
            if let [a, b, ..] = pts[..] {
                targets.push(Point3::new(
                    (a.x + b.x) * 0.5,
                    (a.y + b.y) * 0.5,
                    (a.z + b.z) * 0.5,
                ));
            }
        }
    }
    let reach = 4.0 * ext.max(1e-3);
    let mut rays = Vec::new();
    for at in targets {
        for dir in [
            Vec3::new(1.0, 0.0, 0.0),
            Vec3::new(-1.0, 0.0, 0.0),
            Vec3::new(0.0, 1.0, 0.0),
            Vec3::new(0.0, -1.0, 0.0),
            Vec3::new(0.0, 0.0, 1.0),
            Vec3::new(0.0, 0.0, -1.0),
        ] {
            rays.push(Ray {
                origin: at - dir * reach,
                dir,
            });
        }
    }
    rays
}

/// **Every pick answer is the single-level reference's, hit for hit**:
/// the same triangle (through its patch's name), the same `t` bits,
/// the same point bits, the same miss. Answers how many of `rays`
/// tied at the reference's nearest `t`.
fn assert_flat_reference(
    name: &str,
    step: &str,
    index: &PickIndex,
    reference: &FlatReference,
    session: &DocSession,
    rays: &[Ray],
) -> usize {
    let (_, eval) = session.landed_pair().expect("a landed pair");
    let names: Vec<Vec<Result<StableName, HitTestError>>> = index
        .parts()
        .iter()
        .map(|part: &NodePick| part.patch_names(eval))
        .collect();
    let mut ties = 0;
    for (i, ray) in rays.iter().enumerate() {
        let (expected, tied) = reference.pick(ray);
        if tied >= 2 {
            ties += 1;
        }
        let expected = match expected {
            None => "miss".to_owned(),
            Some(hit) => {
                let part = &index.parts()[hit.part];
                let point = ray.origin + ray.dir * hit.t;
                match &names[hit.part][hit.patch] {
                    Ok(name) => format!(
                        "{:?}/{}/{}/{:016x}/{:016x}/{:016x}/{:016x}",
                        part.node(),
                        part.body(),
                        name,
                        hit.t.to_bits(),
                        point.x.to_bits(),
                        point.y.to_bits(),
                        point.z.to_bits()
                    ),
                    Err(e) => format!("refused: {e}"),
                }
            }
        };
        let actual = match index.pick(eval, ray) {
            Ok(Some(hit)) => format!(
                "{:?}/{}/{}/{:016x}/{:016x}/{:016x}/{:016x}",
                hit.node,
                hit.body,
                hit.name,
                hit.t.to_bits(),
                hit.point.x.to_bits(),
                hit.point.y.to_bits(),
                hit.point.z.to_bits()
            ),
            Ok(None) => "miss".to_owned(),
            Err(e) => format!("refused: {e}"),
        };
        assert_eq!(
            actual, expected,
            "{name} after {step}: ray {i} ({tied} tied; {ray:?}) picks differently from the \
             single-level reference"
        );
    }
    ties
}

/// A pick's answer as comparable bits.
fn hits(index: &PickIndex, session: &DocSession, rays: &[Ray]) -> Vec<String> {
    let (_, eval) = session.landed_pair().expect("a landed pair");
    rays.iter()
        .map(|ray| match index.pick(eval, ray) {
            Ok(Some(hit)) => format!(
                "{:?}/{}/{}/{:016x}/{:016x}/{:016x}/{:016x}",
                hit.node,
                hit.body,
                hit.name,
                hit.t.to_bits(),
                hit.point.x.to_bits(),
                hit.point.y.to_bits(),
                hit.point.z.to_bits()
            ),
            Ok(None) => "miss".to_owned(),
            Err(e) => format!("refused: {e}"),
        })
        .collect()
}

/// One landing's reading: the picture's face count, and how many rays
/// of the pick rows tied at their nearest hit.
#[derive(Clone, Debug)]
struct Step {
    name: String,
    faces: usize,
    ties: usize,
}

/// The seam's answer against the plain door's: the same refusal, or
/// the same picture part by part — and both against the single-level
/// reference, hit for hit. Answers the picture's face count and the
/// tie count.
fn assert_same_answer(
    name: &str,
    step: &str,
    seam: &Result<PickIndex, viewer::pickindex::PickIndexError>,
    fresh: &Result<PickIndex, viewer::pickindex::PickIndexError>,
    session: &DocSession,
) -> Step {
    match (seam, fresh) {
        (Ok(seam), Ok(fresh)) => {
            let ties = assert_same_picture(name, step, seam, fresh, session);
            Step {
                name: step.to_owned(),
                faces: faces_of(seam),
                ties,
            }
        }
        (Err(a), Err(b)) => {
            assert_eq!(
                format!("{a:?}"),
                format!("{b:?}"),
                "{name} after {step}: the seam refuses differently from the plain door"
            );
            Step {
                name: step.to_owned(),
                faces: 0,
                ties: 0,
            }
        }
        (Ok(_), Err(e)) => {
            panic!("{name} after {step}: the plain door refuses ({e:?}) and the seam does not")
        }
        (Err(e), Ok(_)) => {
            panic!("{name} after {step}: the seam refuses ({e:?}) and the plain door does not")
        }
    }
}

/// The seam's picture against the plain door's, part by part; then
/// both indexes against the single-level reference over the fixed
/// rays and the tie-break row. Answers the tie count.
fn assert_same_picture(
    name: &str,
    step: &str,
    seam: &PickIndex,
    fresh: &PickIndex,
    session: &DocSession,
) -> usize {
    assert_eq!(
        seam.parts().len(),
        fresh.parts().len(),
        "{name} after {step}: the seam draws a different number of bodies"
    );
    for (a, b) in seam.parts().iter().zip(fresh.parts()) {
        assert_eq!(
            (a.node(), a.body()),
            (b.node(), b.body()),
            "{name} after {step}: part order"
        );
        assert_eq!(
            digest(a.mesh()),
            digest(b.mesh()),
            "{name} after {step}: node {:?} body {} — the seam's mesh is not the fresh tessellation",
            a.node(),
            a.body()
        );
        // The index itself, tree for tree: the memoised build's
        // per-patch trees (nodes, leaf permutation, boxes — the
        // tree's whole `Debug` form), triangle tables and top-level
        // tree are the fresh build's. Direct, where the hit-for-hit
        // rows are only implied by it.
        assert_eq!(
            format!("{:?}", a.target().pick),
            format!("{:?}", b.target().pick),
            "{name} after {step}: node {:?} body {} — the seam's index is not the fresh one, tree for tree",
            a.node(),
            a.body()
        );
    }
    let mut rays = rays_for(fresh);
    assert_eq!(
        hits(seam, session, &rays),
        hits(fresh, session, &rays),
        "{name} after {step}: the seam's index answers different picks"
    );
    // The seam's meshes are the fresh ones (asserted above), so one
    // reference over the fresh meshes is the reference for both.
    rays.extend(tie_rays_for(fresh));
    let reference = FlatReference::of(fresh);
    let fresh_ties = assert_flat_reference(name, step, fresh, &reference, session, &rays);
    let ties = assert_flat_reference(name, step, seam, &reference, session, &rays);
    assert_eq!(ties, fresh_ties);
    println!(
        "# {name} after {step}: {} rays against the single-level reference, {ties} tied",
        rays.len()
    );
    ties
}

/// **A memo that never hits would pass the differential**, so the
/// documents whose edits leave something reusable pin a floor on what
/// the memo answered. The measured counts (this row under
/// `--nocapture`) with a little slack, and the reason each is what it
/// is:
/// - `die_composed_tour`, the first edit (one pip moved): one root,
///   recomputed, 85 of 89 faces bit-identical — level 2's case.
/// - `die`, the first edit: 106 of 111 faces bit-identical.
/// - `kitchen_sink`, the first edit (8 roots, the bump feeds 3): five
///   roots reused whole — level 1's case.
/// - `heat_sink`, the second edit (6 roots, the second slot feeds 1):
///   five roots reused whole.
///
/// `(document, step, node-hit floor, face-hit floor)`.
const HIT_FLOORS: &[(&str, &str, usize, usize)] = &[
    ("die_composed_tour", "the first edit", 0, 80),
    ("die", "the first edit", 0, 100),
    ("kitchen_sink", "the first edit", 5, 0),
    ("heat_sink", "the second edit", 5, 20),
];

fn assert_hit_floor(name: &str, step: &str, report: &MemoReport) {
    for &(doc, at, nodes, faces) in HIT_FLOORS {
        if doc == name && at == step {
            assert!(
                report.node_hits >= nodes && report.face_hits >= faces,
                "{name} after {step}: the memo answered {} parts and {} faces; the floor is {nodes} and {faces}",
                report.node_hits,
                report.face_hits
            );
            assert!(
                report.tree_hits >= faces,
                "{name} after {step}: the memo answered {} pick trees; the floor is {faces}",
                report.tree_hits
            );
        }
    }
}

/// Open → index; then each edit → land → index, asserting the seam's
/// picture is the plain door's after every landing. Answers the
/// per-step face counts of the picture, for the memo rows to read.
fn drive(name: &str, doc: ProfileDoc, edits: &[(&str, Edit)], tol: Tol) -> Vec<Step> {
    let mut session = DocSession::inline(doc, tol);
    session.pump();
    assert!(session.evaluation().is_some(), "{name}: the document lands");
    let mut seam = InlineIndexer::new();
    let mut steps = Vec::new();
    let index = seam_index(&mut seam, &session);
    let fresh = fresh_index(&session);
    assert!(
        index.is_ok(),
        "{name}: the document indexes as opened: {index:?}"
    );
    let opened = assert_same_answer(name, "open", &index, &fresh, &session);
    // Faces answered at open are hits WITHIN the picture: two roots
    // drawing one bit-identical face (the heat sink's fins) share an
    // entry. That count is the document's, not δ's, and the δ row
    // below expects exactly it again.
    let self_hits = reading(&seam).face_hits;
    if let Ok(index) = &index {
        assert_memo_is_one_picture(name, "open", &seam, index);
        assert_eq!(
            reading(&seam).node_hits,
            0,
            "{name}: nothing to reuse at open"
        );
    }
    steps.push(opened);
    for (step, edit) in edits {
        let outcome = session.perform(edit.op());
        assert!(
            outcome.refusal.is_none(),
            "{name}: edit {step} refused: {:?}",
            outcome.refusal
        );
        session.pump();
        let index = seam_index(&mut seam, &session);
        let fresh = fresh_index(&session);
        let landed = assert_same_answer(name, step, &index, &fresh, &session);
        if let Ok(index) = &index {
            assert_memo_is_one_picture(name, step, &seam, index);
        }
        assert_hit_floor(name, step, &MemoReport::of(seam.memo()));
        steps.push(landed);
    }
    // A δ change misses everything, at all three levels: the same
    // run, indexed finer, reuses no part, no face and no tree. The
    // `tol` half of the key has no in-process row: `Tol` is a witness
    // of the one tolerance a process commits, so there is no second
    // value to change to here — the (ε, k) axis is exercised as CI's
    // per-process eps rows, each of which opens its memo cold.
    let finer = DisplayTolerance::new(delta().get() / 2.0).expect("a positive delta");
    let index = seam_index_at(&mut seam, &session, finer);
    if let Ok(index) = &index {
        let r = reading(&seam);
        assert_eq!(
            (r.node_hits, r.face_hits, r.tree_hits),
            (0, self_hits, self_hits),
            "{name}: a δ change misses everything the previous picture held"
        );
        assert_eq!(r.node_misses, index.parts().len());
        assert_eq!(r.face_misses + self_hits, faces_of(index));
        assert_eq!(r.tree_misses + self_hits, faces_of(index));
        assert_memo_is_one_picture(name, "the δ change", &seam, index);
    }
    steps
}

fn faces_of(index: &PickIndex) -> usize {
    index.parts().iter().map(|p| p.mesh().patches.len()).sum()
}

/// The edit sequence for a document with a bump: the bump, another
/// length slot where one exists, then the bump reverted.
fn sequence(doc: &ProfileDoc, bump: Edit, revert: Edit) -> Vec<(&'static str, Edit)> {
    let mut edits = vec![("the first edit", bump.clone())];
    if let Some(other) = another_length_slot(doc, bump.node) {
        edits.push(("the second edit", other));
    }
    edits.push(("the revert", revert));
    edits
}

#[test]
fn every_parametric_corpus_document_indexes_the_same_through_the_seam_across_edits() {
    let tol = Tol::witness();
    let mut seen = BTreeMap::new();
    for c in corpus::documents() {
        let Some((bump, revert)) = bump_of(&c) else {
            continue;
        };
        let edits = sequence(&c.doc, bump, revert);
        let steps = drive(c.name, c.doc.clone(), &edits, tol);
        seen.insert(c.name, steps);
    }
    assert!(
        seen.len() >= 8,
        "the corpus carries at least eight parametric documents; saw {:?}",
        seen.keys().collect::<Vec<_>>()
    );
    for &(doc, step, _, _) in HIT_FLOORS {
        let steps = seen
            .get(doc)
            .unwrap_or_else(|| panic!("{doc} is a corpus document"));
        assert!(
            steps.iter().any(|s| s.name == step),
            "{doc} reaches {step}, so its floor was checked"
        );
    }
    // The tie-break row is a row only if its rays tie: across the
    // corpus, a floor on the landings whose nearest hit was shared by
    // two or more triangles (the measured count, with slack).
    let tied_landings: usize = seen.values().flatten().filter(|s| s.ties > 0).count();
    let landings: usize = seen.values().map(Vec::len).sum();
    println!("# tie-break row: {tied_landings} of {landings} landings had a tied nearest hit");
    assert!(
        tied_landings >= TIED_LANDINGS_FLOOR,
        "the tie-break row tied on {tied_landings} landings; the floor is {TIED_LANDINGS_FLOOR}"
    );
}

/// How many corpus landings the tie-break row must tie on (measured
/// under `--nocapture`, with slack).
const TIED_LANDINGS_FLOOR: usize = 80;

/// **The production seam keeps its memo across pictures too.** The
/// threaded worker owns a memo for its thread's life; this drives it
/// over four landings of two documents, asking it for every picture
/// but one, and asserts that each answer is the plain door's and that
/// the picture after the skipped one is served from the memo at node
/// level: the revert returns the document to the state the worker
/// last indexed, so every root's content and naming keys match and
/// nothing is tessellated.
#[cfg(not(target_family = "wasm"))]
#[test]
fn the_worker_threads_memo_answers_across_landings_and_a_skipped_generation() {
    let tol = Tol::witness();
    for name in ["die", "kitchen_sink"] {
        let c = corpus::documents()
            .into_iter()
            .find(|c| c.name == name)
            .expect("a corpus document");
        let (bump, revert) = bump_of(&c).expect("a parametric document");
        let mut session = DocSession::inline(c.doc.clone(), tol);
        session.pump();
        let mut worker = viewer::evalseam::ThreadIndexer::spawn().expect("the worker starts");
        // Landing 0: open, indexed. Landing 1: the bump, NOT indexed by
        // the worker (a generation it never saw). Landing 2: the
        // revert, indexed — the open picture again. Landing 3: the
        // bump again, indexed.
        let ops = [Some(bump.clone()), Some(revert), Some(bump)];
        let asked = [true, false, true, true];
        let mut parts = 0;
        for (landing, ask) in asked.iter().enumerate() {
            if landing > 0 {
                let op = ops[landing - 1].clone().expect("an edit");
                let outcome = session.perform(op.op());
                assert!(
                    outcome.refusal.is_none(),
                    "{name}: landing {landing} refused"
                );
                session.pump();
            }
            if !ask {
                continue;
            }
            let done = answer(&mut worker, request_at(&session, delta()));
            let fresh = fresh_index(&session);
            let faces = assert_same_answer(
                name,
                &format!("landing {landing}"),
                &done.index,
                &fresh,
                &session,
            )
            .faces;
            if landing == 0 {
                parts = done
                    .index
                    .as_ref()
                    .map(|i| i.parts().len())
                    .expect("the open picture indexes");
                assert_eq!(done.memo.node_hits, 0, "{name}: nothing to reuse at open");
            }
            if landing == 2 {
                assert_eq!(
                    (done.memo.node_hits, done.memo.node_misses),
                    (parts, 0),
                    "{name}: the revert after a skipped generation is the open picture, served whole"
                );
                assert_eq!((done.memo.face_misses, done.memo.tree_misses), (0, 0));
                assert!(done.memo.faces <= faces && done.memo.trees <= faces);
            }
            if landing == 3 {
                assert_eq!(done.memo.node_hits + done.memo.node_misses, parts);
            }
        }
    }
}

#[test]
fn the_gallery_ring_indexes_the_same_through_the_seam_across_edits() {
    let tol = Tol::witness();
    let text = common::gallery_ring_at(tol);
    let loaded = pncad::document::load(&text, tol).expect("the gallery ring loads");
    let doc = loaded.snapshot;
    let (node, slot, expr) = first_length_slot(&doc);
    let original = doc
        .node(node)
        .expect("a node")
        .expr(slot)
        .expect("its slot");
    let bump = Edit {
        node,
        slot,
        text: unparse(&expr),
    };
    let revert = Edit {
        node,
        slot,
        text: unparse(original),
    };
    let edits = sequence(&doc, bump, revert);
    drive("gallery_ring", doc, &edits, tol);
}

/// The last extrude distance or revolve angle in the document, scaled
/// — the gallery ring's own bump.
fn first_length_slot(doc: &ProfileDoc) -> (RecipeNodeId, SlotId, Expr) {
    let env = doc.param_env::<f64>();
    for &node in doc.order().iter().rev() {
        match doc.node(node).expect("a node") {
            editor_core::Node::Extrude { distance, .. } => {
                let value = editor_core::eval(distance, &env).expect("a literal distance");
                let expr =
                    Expr::literal(value * 1.03125, Dimension::Length).expect("a length literal");
                return (node, SlotId::Distance, expr);
            }
            editor_core::Node::Revolve { angle, .. } => {
                let value = editor_core::eval(angle, &env).expect("a literal angle");
                let expr =
                    Expr::literal(value * 0.96875, Dimension::Angle).expect("an angle literal");
                return (node, SlotId::RevolveAngle, expr);
            }
            _ => {}
        }
    }
    panic!("no extrude or revolve in the document")
}
