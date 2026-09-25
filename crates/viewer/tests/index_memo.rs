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
use editor_core::resolve::{TSpan, answer_of, crossing, ray_triangle};
use editor_core::{
    Dimension, DocEdit, Evaluation, Expr, HitTestError, NodePick, PickHit, ProfileDoc,
    RecipeNodeId, SlotId, StableName, unparse,
};
use pncad::geom_core::{Point3, Tol, Vec3};
use pncad::mesh::Mesh;
use viewer::evalseam::{IndexDone, IndexRequest, IndexService, InlineIndexer, MemoReport};
use viewer::pickindex::{PickIndex, PictureKey};
use viewer::scene::DisplayTolerance;
use viewer::session::{DocSession, SessionOp};

use crate::common;
use crate::corpus;

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
        key: PictureKey::of(
            session
                .landed_generation()
                .expect("a landed evaluation has a generation"),
            at,
        ),
        doc: doc.clone(),
        evaluation: Arc::clone(session.evaluation_arc().expect("a landed run")),
        tol: session.tol(),
    }
}

/// A seam's answer for a request, waited for: the inline seam answers
/// inside `poll`, the threaded one when its worker is done.
fn answer(seam: &mut impl IndexService, request: IndexRequest) -> IndexDone {
    let key = request.key;
    seam.submit(request);
    for _ in 0..100_000 {
        if let Some(done) = seam.poll() {
            assert_eq!(done.key, key);
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
    seam_index_at(seam, session, common::corpus_delta())
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
    tables: usize,
    table_hits: usize,
    table_misses: usize,
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
        tables: memo.table_len(),
        table_hits: memo.table_hits(),
        table_misses: memo.table_misses(),
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
         faces {} (hits {} misses {}), tables {} (hits {} misses {})",
        r.nodes,
        r.node_hits,
        r.node_misses,
        r.faces,
        r.face_hits,
        r.face_misses,
        r.tables,
        r.table_hits,
        r.table_misses
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
    // The per-patch pick tables are keyed BY the patch memo's own
    // entries and looked up exactly where the patches are, so they hit
    // and miss where the patches do, and are evicted with them. This
    // is the row that says the table level reuses neither more nor
    // less than the byte-compared patch key licenses.
    //
    // WHAT IT DOES NOT COVER, because the two levels are stamped in
    // different places: a patch entry is stamped in `PatchMemo::record`
    // and a table in `MeshPick::build_with`, which never runs when the
    // tessellation it would follow refuses. `PickIndex::build_with`
    // closes the picture either way, so a root that refuses partway
    // leaves its counted patch hits alive with no tables beside them,
    // and the NEXT picture can report `face_hits > table_hits`
    // legitimately. Every document this differential drives lands (the
    // refusal arm is asserted in `assert_same_answer` and never
    // reaches here), so the equality holds over every row it is
    // asserted on — it is not a claim about pictures after a refusal.
    assert_eq!(
        (r.table_hits, r.table_misses),
        (r.face_hits, r.face_misses),
        "{name} after {step}: the pick tables hit and miss where the patches do"
    );
    assert!(
        r.tables <= faces,
        "{name} after {step}: the memo holds {} pick tables for a picture of {faces}",
        r.tables
    );
    assert!(r.tables >= 1 || faces == 0);
}

/// The plain door's answer for the same run: the definition of the
/// picture.
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
/// the exact test on every candidate the tree hands back — no
/// early-out — nearest `t` with ties to the lower `(part, flat
/// triangle)` position. Test-only. Whatever shape the production
/// index takes — one tree per mesh, a tree per patch under a tree
/// over the patches — its answer to every ray is this one's, hit for
/// hit: the minimum over the per-triangle tests, each of which reads
/// the ray and the triangle alone. The service's early-out on the
/// conservative entry is a cost measure, and [`Walk::Pruned`] is how
/// `reference_answers` says on every ray that it did not change the
/// minimum: what it can change in principle is a near-tie decided by
/// the rounding of `t` (`pick_face`'s docs), and the row is where
/// that would show.
struct FlatPart {
    tree: Bvh,
    corners: Vec<[Point3<f64>; 3]>,
    /// Flat triangle position → (patch position, triangle position).
    owner: Vec<(usize, usize)>,
}

struct FlatReference {
    parts: Vec<FlatPart>,
}

/// One reference hit: which part, patch and triangle, over what `t`
/// interval.
#[derive(Clone, Copy, Debug)]
struct FlatHit {
    part: usize,
    /// Flat triangle position within the part.
    item: usize,
    patch: usize,
    span: TSpan,
}

impl FlatHit {
    /// The rounded parameter — what the service reports.
    fn t(&self) -> f64 {
        self.span.t
    }
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

    /// **What the door answers for**: one hit per FACE of the certified
    /// tie, in the order the door lists them, and how many candidates
    /// the tie holds — which is what says a ray of the tie-break row
    /// actually tied.
    ///
    /// EVERY candidate box the ray meets is tested: this is the
    /// reference, and the service's early-out is what it is a
    /// reference for. Neither half of the RULE is restated — the
    /// candidates are offered to [`answer_of`] in `(part, flat
    /// position)` order, and that callable is the door's own, order
    /// and face grouping together. What is this file's own is the
    /// enumeration.
    fn pick(&self, ray: &Ray) -> (Vec<FlatHit>, usize) {
        let mut hits: Vec<FlatHit> = Vec::new();
        for (part, flat) in self.parts.iter().enumerate() {
            let mut items: Vec<usize> = flat.tree.ray(ray).into_iter().map(|c| c.item).collect();
            items.sort_unstable();
            for item in items {
                let Some(span) = ray_triangle(ray, &flat.corners[item]) else {
                    continue;
                };
                let (patch, _) = flat.owner[item];
                hits.push(FlatHit {
                    part,
                    item,
                    patch,
                    span,
                });
            }
        }
        let candidates: Vec<(TSpan, (usize, usize))> = hits
            .iter()
            .map(|hit| (hit.span, (hit.part, hit.patch)))
            .collect();
        let faces = answer_of(&candidates).faces();
        // How many candidates the tie holds — the sum of the groups'
        // memberships is the survivor count, which is what says a ray
        // of the tie-break row actually tied.
        let tied = faces.iter().map(|face| face.members).sum();
        let per_face = faces
            .into_iter()
            .map(|face| FlatHit {
                span: face.span,
                ..hits[face.member]
            })
            .collect();
        (per_face, tied)
    }
}

/// **The tie-break row**: rays aimed exactly at the points two or more
/// patches share — a boundary polyline's first point (a vertex or
/// chord point every incident face's triangles have as a corner) and
/// the midpoint of its first segment (a point on the shared edge) —
/// along the six axis directions from outside the picture. A hit
/// there is a hit for every incident triangle at one `t`, across
/// patches and, where bodies touch, across parts: the case the door's
/// set rule is about, and the one it refuses on.
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

/// The reference's answer to every ray, walked once, and one claim
/// about every answer it gives.
///
/// `Pruned == Every` is NOT here any more: it is a claim about the
/// service's early-out, and it belongs where the service is called.
/// `pick3_acceptance` pins it through `PickIndex::pick` against this
/// same exhaustive walk, over the same landings.
///
/// **No winner's barycentric carries a bound of `1` or more.** A
/// value inside `[0, 1]` whose interval is that wide covers the range
/// and is refused at the exact test (`ray_triangle`'s INFORM half), so
/// zero is the only count this can have; it is asserted here, over
/// every ray of every landing, rather than pinned as a number, because
/// the count is derivable and a pinned `0` would read as a baseline.
/// Re-derive the whole picture with
/// `cargo test -p viewer --test all -- index_memo`.
fn reference_answers(
    name: &str,
    step: &str,
    reference: &FlatReference,
    rays: &[Ray],
) -> Vec<(Vec<FlatHit>, usize)> {
    rays.iter()
        .enumerate()
        .map(|(i, ray)| {
            let (expected, tied) = reference.pick(ray);
            for hit in &expected {
                let tri = &reference.parts[hit.part].corners[hit.item];
                let bounds = crossing(ray, tri)
                    .expect("an answered candidate's determinant is certified")
                    .barycentrics
                    .map(|(_, err)| err);
                // A NaN bound is not a bound and must red this row,
                // not slip through a `b >= 1.0` that a NaN fails.
                assert!(
                    !bounds.iter().any(|&b| b.is_nan() || b >= 1.0),
                    "{name} after {step}: ray {i} ({ray:?}) is answered by {hit:?} whose widest \
                     barycentric bounds are {bounds:?} — an interval that wide covers [0, 1] and \
                     the exact test refuses it"
                );
            }
            (expected, tied)
        })
        .collect()
}

/// **Every pick answer is the single-level reference's, hit for hit**:
/// the same faces (through their patches' names), the same `t` bits,
/// the same point bits, the same miss — and, where the door refuses,
/// the same LIST of tied faces in the same order. Answers how many of
/// `rays` tied at the reference's nearest `t`.
fn assert_flat_reference(
    name: &str,
    step: &str,
    index: &PickIndex,
    answers: &[(Vec<FlatHit>, usize)],
    session: &DocSession,
    rays: &[Ray],
) -> usize {
    let (_, eval) = session.landed_pair().expect("a landed pair");
    let names: Vec<Vec<Result<StableName, HitTestError>>> = index
        .parts()
        .iter()
        .map(|part: &NodePick| {
            part.patch_names(eval)
                .expect("the parts are of this evaluation")
        })
        .collect();
    let mut ties = 0;
    for (i, ray) in rays.iter().enumerate() {
        let (expected, tied) = &answers[i];
        if *tied >= 2 {
            ties += 1;
        }
        let expected = if expected.is_empty() {
            "miss".to_owned()
        } else {
            expected
                .iter()
                .map(|hit| {
                    let part = &index.parts()[hit.part];
                    let point = ray.origin + ray.dir * hit.t();
                    match &names[hit.part][hit.patch] {
                        Ok(name) => descriptor(part.node(), part.body(), name, hit.t(), point),
                        Err(e) => format!("refused: {e}"),
                    }
                })
                .collect::<Vec<_>>()
                .join(" | ")
        };
        let actual = match index.pick(eval, ray) {
            Ok(Some(hit)) => descriptor(hit.node, hit.body, &hit.name, hit.t, hit.point),
            Ok(None) => "miss".to_owned(),
            Err(HitTestError::Ambiguous { hits }) => hits
                .iter()
                .map(|hit| descriptor(hit.node, hit.body, &hit.name, hit.t, hit.point))
                .collect::<Vec<_>>()
                .join(" | "),
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

/// **The service's whole answer**, as the list the door's two shapes
/// share: the one hit, the empty miss, or the tied faces of the
/// refusal. Every probe below reads the door through this, because a
/// certified tie between faces is an ANSWER about the ray and not an
/// error to unwrap past.
fn service_answers(index: &PickIndex, eval: &Evaluation<f64>, ray: &Ray) -> Vec<PickHit> {
    match index.pick(eval, ray) {
        Ok(hit) => hit.into_iter().collect(),
        Err(HitTestError::Ambiguous { hits }) => hits,
        Err(other) => panic!("the pick resolves: {other}"),
    }
}

/// One answered face as comparable bits: which body, which name,
/// where. The two sides of every differential here render through
/// this one function, so a drift is a drift in the answer and never in
/// the spelling.
fn descriptor(
    node: RecipeNodeId,
    body: u32,
    name: &StableName,
    t: f64,
    point: Point3<f64>,
) -> String {
    format!(
        "{node:?}/{body}/{name}/{:016x}/{:016x}/{:016x}/{:016x}",
        t.to_bits(),
        point.x.to_bits(),
        point.y.to_bits(),
        point.z.to_bits()
    )
}

/// A pick's answer as comparable bits.
fn hits(index: &PickIndex, session: &DocSession, rays: &[Ray]) -> Vec<String> {
    let (_, eval) = session.landed_pair().expect("a landed pair");
    rays.iter()
        .map(|ray| match index.pick(eval, ray) {
            Ok(Some(hit)) => descriptor(hit.node, hit.body, &hit.name, hit.t, hit.point),
            Ok(None) => "miss".to_owned(),
            Err(HitTestError::Ambiguous { hits }) => hits
                .iter()
                .map(|hit| descriptor(hit.node, hit.body, &hit.name, hit.t, hit.point))
                .collect::<Vec<_>>()
                .join(" | "),
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
        // The index itself, TABLE FOR TABLE and tree for tree: every
        // patch the memoised build served — its triangle corners, the
        // boxes the tree was built over, the tree's nodes and leaf
        // permutation, its hull — plus the top-level tree, are the
        // fresh build's. The whole `MeshPick` `Debug` form is the
        // comparison, and it is a comparison BY BITS: Rust prints an
        // `f64` at shortest round-trip precision, so two distinct
        // finite corners cannot print alike (a NaN payload is the one
        // thing it cannot separate, and a poisoned box is poisoned in
        // both). Direct, where the hit-for-hit rows are only implied
        // by it — and the row that catches a served table whose
        // corners are no longer the mesh's.
        assert_eq!(
            format!("{:?}", a.target()),
            format!("{:?}", b.target()),
            "{name} after {step}: node {:?} body {} — the seam's index is not the fresh one, table for table and tree for tree",
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
    let answers = reference_answers(name, step, &reference, &rays);
    let fresh_ties = assert_flat_reference(name, step, fresh, &answers, session, &rays);
    let ties = assert_flat_reference(name, step, seam, &answers, session, &rays);
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
/// **And a memo that hit EVERYTHING would pass them**, which is the
/// other half of the same defect: the edited face's key must change,
/// so every one of these steps also owes at least one face miss. The
/// floor is 1 rather than the measured count (4, 5, 18 and 4) because
/// what it guards is the existence of the miss, not the corpus's
/// current shape — and the row that makes it sharp is the
/// table-for-table comparison in `assert_same_picture`, which is what
/// says the missed face's table is the FRESH build's and not a served
/// one.
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
                report.table_hits >= faces,
                "{name} after {step}: the memo answered {} pick tables; the floor is {faces}",
                report.table_hits
            );
            assert!(
                report.face_misses >= 1 && report.table_misses >= 1,
                "{name} after {step}: the edit changed a face, so the memo owes a miss; it \
                 reported {} face misses and {} pick-table misses",
                report.face_misses,
                report.table_misses
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
    let fresh = common::index_at(&session, common::corpus_delta());
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
        let fresh = common::index_at(&session, common::corpus_delta());
        let landed = assert_same_answer(name, step, &index, &fresh, &session);
        if let Ok(index) = &index {
            assert_memo_is_one_picture(name, step, &seam, index);
        }
        assert_hit_floor(name, step, &MemoReport::of(seam.memo()));
        steps.push(landed);
    }
    // A δ change misses everything, at all three levels: the same
    // run, indexed finer, reuses no part, no face and no table. The
    // `tol` half of the key has no in-process row: `Tol` is a witness
    // of the one tolerance a process commits, so there is no second
    // value to change to here — the (ε, k) axis is exercised as CI's
    // per-process eps rows, each of which opens its memo cold.
    let finer =
        DisplayTolerance::new(common::corpus_delta().get() / 2.0).expect("a positive delta");
    let index = seam_index_at(&mut seam, &session, finer);
    if let Ok(index) = &index {
        let r = reading(&seam);
        assert_eq!(
            (r.node_hits, r.face_hits, r.table_hits),
            (0, self_hits, self_hits),
            "{name}: a δ change misses everything the previous picture held"
        );
        assert_eq!(r.node_misses, index.parts().len());
        assert_eq!(r.face_misses + self_hits, faces_of(index));
        assert_eq!(r.table_misses + self_hits, faces_of(index));
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
            let done = answer(&mut worker, request_at(&session, common::corpus_delta()));
            let fresh = common::index_at(&session, common::corpus_delta());
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
                assert_eq!((done.memo.face_misses, done.memo.table_misses), (0, 0));
                assert!(done.memo.faces <= faces && done.memo.tables <= faces);
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

/// **A grazing ray answers the corner it grazes, not a noise `t`.**
/// After the gallery ring's bump, the tie-break row aims a ray along
/// +y at a chord point of the tube. The ray lies in the plane of a
/// triangle of the face it does NOT cross there: Möller–Trumbore's
/// determinant for that triangle is rounding noise, and its quotient
/// `t = e2·q / det` cancels to `1.5`, 0.02 BEYOND the corner, while its `u` and `v` are exactly `0`: in exact arithmetic
/// over the mesh's rounded corners the ray passes through that
/// triangle's own corner `a` — the chord point — and its true `t` is
/// `1.480`. The exact test now takes `t` from the hit point
/// `a + u·e1 + v·e2` projected onto the ray, so the answer is the corner,
/// `t = 1.480` to the bit, from the reference and the service alike.
/// Two premise rows keep the probe honest against a retessellation:
/// the chord point is still a mesh vertex (so the ray is still a
/// graze), and the candidate set still holds a triangle with the
/// chord point as a corner whose Möller–Trumbore quotient is off the
/// corner by more than 1e-6 (so the class the projection exists for
/// is still there). Parked here rather than beside `pick.rs`'s own
/// rows because it needs the gallery ring, which only the viewer's
/// test corpus loads.
#[test]
fn the_ring_grazing_ray_answers_the_corner_it_grazes() {
    let tol = Tol::witness();
    let text = common::gallery_ring_at(tol);
    let loaded = pncad::document::load(&text, tol).expect("the gallery ring loads");
    let doc = loaded.snapshot;
    let (node, slot, expr) = first_length_slot(&doc);
    let bump = Edit {
        node,
        slot,
        text: unparse(&expr),
    };
    let mut session = DocSession::inline(doc, tol);
    session.pump();
    let outcome = session.perform(bump.op());
    assert!(
        outcome.refusal.is_none(),
        "the bump lands: {:?}",
        outcome.refusal
    );
    session.pump();
    let index =
        common::index_at(&session, common::corpus_delta()).expect("the bumped ring indexes");
    let corner = Point3::new(0.22558061449274294, 0.0, 0.0448707740637096);
    let reach = REACH;
    let ray = aimed_along_y(corner, 1.0);
    let same = |p: &Point3<f64>, q: &Point3<f64>| {
        (p.x.to_bits(), p.y.to_bits(), p.z.to_bits())
            == (q.x.to_bits(), q.y.to_bits(), q.z.to_bits())
    };
    assert!(
        index
            .parts()
            .iter()
            .any(|part| part.mesh().positions.iter().any(|p| same(p, &corner))),
        "the probe's premise: the chord point is a vertex of the bumped ring's mesh, so the ray \
         through it is a graze"
    );
    let reference = FlatReference::of(&index);
    // Möller–Trumbore's own `t` quotient for each candidate that has
    // the chord point as its first corner (so `s = origin − a` is
    // exactly `−reach · dir` and `u = v = 0` exactly): the quotient
    // the old exact test answered, and the number the projection
    // replaces.
    let quotients: Vec<f64> = reference
        .parts
        .iter()
        .flat_map(|flat| {
            flat.tree.ray(&ray).into_iter().filter_map(move |cand| {
                let tri = &flat.corners[cand.item];
                if !same(&tri[0], &corner) {
                    return None;
                }
                let det = crossing(&ray, tri)?.det;
                let e1: Vec3<f64> = tri[1] - tri[0];
                let e2: Vec3<f64> = tri[2] - tri[0];
                let q = (ray.origin - tri[0]).cross(e1);
                Some(e2.dot(q) / det)
            })
        })
        .collect();
    assert!(
        quotients.iter().any(|t| (t - reach).abs() > 1e-6),
        "the probe's premise: a candidate with the chord point as its corner whose \
         Möller–Trumbore quotient is off the corner is in the ray's candidate set: {quotients:?}"
    );
    let (answers, _) = reference.pick(&ray);
    assert!(!answers.is_empty(), "the ray meets the ring");
    for hit in &answers {
        assert_eq!(
            hit.t().to_bits(),
            RING_CORNER_T.to_bits(),
            "the reference answers the corner at t = {RING_CORNER_T} (reach {reach}), not a \
             noise t of a triangle the ray only lies in the plane of: {hit:?}"
        );
    }
    let (_, eval) = session.landed_pair().expect("a landed pair");
    let picked = service_answers(&index, eval, &ray);
    assert_eq!(
        picked.iter().map(|h| h.t.to_bits()).collect::<Vec<_>>(),
        answers.iter().map(|h| h.t().to_bits()).collect::<Vec<_>>(),
        "the service answers the reference's faces at the reference's t: {picked:?} against \
         {answers:?}"
    );
    let hit = answers[0];
    let picked = &picked[0];
    let expected_point = ray.origin + ray.dir * hit.t();
    assert_eq!(
        [picked.point.x, picked.point.y, picked.point.z].map(f64::to_bits),
        [expected_point.x, expected_point.y, expected_point.z].map(f64::to_bits),
        "the hit point is the chord point as the ray reaches it: {:?}",
        picked.point
    );
}

/// **A wide but informative candidate loses to the transversal
/// neighbour that answers the aimed vertex.** After the gallery ring's
/// bump, the `−y` ray through the tube vertex `(0.2452, 0, 0.0488)` at
/// `reach = 1.48` meets a flat face of the ring so nearly edge-on that
/// its determinant is `1.66e-19` and its conditioning `7.19e-16` — the
/// ray lies in that triangle's plane to within a rounding, and `|det|`
/// is a handful of times its own certification bound. Its barycentrics
/// land inside `[0, 1]` with intervals of `±0.47`, `±0.22` and `±0.68`
/// — wide, but not wide enough to COVER the admissible range, so
/// INFORM admits it and it answers `0.031` SHORT of the vertex. Two
/// triangles that cross the ray transversally (`|det| ≈ 2.8e-5`) answer
/// AT the vertex, `t = 1.48` to the bit.
///
/// **The door answers a `t` INTERVAL, and on this ray the interval
/// ORDERS them**: the wide candidate's own width is
/// `err_u·|e1| + err_v·|e2|` projected on the ray, and the ring's
/// triangles are `0.016` on a side, so an interval that says almost
/// nothing about WHERE on the triangle the ray crossed still says the
/// crossing is within `0.015` of `1.4488` — wholly before the vertex
/// `0.031` further on. The wide candidate PRECEDES the narrow one, so
/// they are not tied at all, and the answer is `1.4488` as before.
///
/// **That is what this row now records, and it is not what the `t`
/// ruling predicted for it.** The certified width is relative to the
/// TRIANGLE, not to the scene: a candidate at the certification's
/// noise floor over a small triangle is still certified to a small
/// piece of the ray. So the class the row above names — a
/// near-coplanar candidate beating the transversal neighbour that
/// answers the aimed vertex — survives the interval order wherever the
/// two crossings are further apart than the near-coplanar triangle is
/// large. `a_wide_candidates_interval_reaches_the_aimed_vertex_and_the_tie_break_takes_it`
/// is the other side of the same line, where they are not.
#[test]
fn a_wide_but_informative_candidate_answers_before_the_rings_aimed_vertex() {
    let tol = Tol::witness();
    let text = common::gallery_ring_at(tol);
    let loaded = pncad::document::load(&text, tol).expect("the gallery ring loads");
    let doc = loaded.snapshot;
    let (node, slot, expr) = first_length_slot(&doc);
    let bump = Edit {
        node,
        slot,
        text: unparse(&expr),
    };
    let mut session = DocSession::inline(doc, tol);
    session.pump();
    let outcome = session.perform(bump.op());
    assert!(
        outcome.refusal.is_none(),
        "the bump lands: {:?}",
        outcome.refusal
    );
    session.pump();
    let index =
        common::index_at(&session, common::corpus_delta()).expect("the bumped ring indexes");
    let vertex = Point3::new(0.245_196_320_100_807_58, 0.0, 0.048_772_580_504_032_18);
    let reach = REACH;
    let ray = aimed_along_y(vertex, -1.0);
    assert!(
        index.parts().iter().any(|part| {
            part.mesh().positions.iter().any(|p| {
                (p.x.to_bits(), p.y.to_bits(), p.z.to_bits())
                    == (vertex.x.to_bits(), vertex.y.to_bits(), vertex.z.to_bits())
            })
        }),
        "the probe's premise: the aimed point is a vertex of the bumped ring's mesh, so the ray \
         through it is a graze"
    );
    let reference = FlatReference::of(&index);
    // Every admitted candidate on this ray, with the triangle that
    // answered it: the wide one and its transversal neighbours are
    // both in here, which is what makes the row about the ORDER.
    let admitted: Vec<([Point3<f64>; 3], TSpan)> = reference
        .parts
        .iter()
        .flat_map(|flat| {
            flat.tree.ray(&ray).into_iter().filter_map(move |cand| {
                let tri = flat.corners[cand.item];
                ray_triangle(&ray, &tri).map(|span| (tri, span))
            })
        })
        .collect();
    // The wide candidate's own numbers, which are why the acceptance
    // takes it at all — the premise the row's name is about.
    let (wide_tri, wide) = admitted
        .iter()
        .find(|(_, span)| span.t.to_bits() == RING_WIDE_CANDIDATE_T.to_bits())
        .copied()
        .expect("the wide candidate still answers 0.031 short of the vertex");
    let c = crossing(&ray, &wide_tri).expect("the wide candidate's determinant is certified");
    let conditioning = c.conditioning(&ray, &wide_tri);
    let margin = c.det.abs() / c.bound_det;
    println!(
        "# the ring's wide candidate: det {:e}, conditioning {conditioning:e}, \
         |det|/bound_det {margin}, barycentrics {:?}, t interval {wide:?}",
        c.det, c.barycentrics
    );
    assert!(
        (conditioning - RING_WIDE_CANDIDATE_CONDITIONING).abs()
            < 0.01 * RING_WIDE_CANDIDATE_CONDITIONING,
        "the wide candidate's conditioning moved from {RING_WIDE_CANDIDATE_CONDITIONING:e}: \
         {conditioning:e}"
    );
    assert!(
        (1.0..10.0).contains(&margin),
        "the wide candidate sits at the certification's own floor: |det| is {margin} times its \
         bound"
    );
    for (what, (x, err)) in ["u", "v", "u + v"].into_iter().zip(c.barycentrics) {
        assert!(
            (0.0..=1.0).contains(&x),
            "{what} = {x} is inside the closed range"
        );
        assert!(
            x - err > 0.0 || x + err < 1.0,
            "{what}: the interval {x} ± {err} does not cover the admissible range, so INFORM \
             admits it"
        );
        assert!(
            err > 0.1,
            "{what}: the interval {x} ± {err} is wide all the same — which is what the `t` \
             interval inherits"
        );
    }
    assert!(
        wide.t < reach,
        "the wide candidate's own t {} is SHORT of the aimed vertex at {reach}",
        wide.t
    );
    // The transversal neighbours that answer at the vertex, and that
    // the wide candidate's interval nonetheless precedes.
    let at_vertex: Vec<TSpan> = admitted
        .iter()
        .filter(|(tri, _)| crossing(&ray, tri).is_some_and(|c| c.det.abs() > 1e-6))
        .map(|&(_, span)| span)
        .collect();
    let narrow = at_vertex
        .iter()
        .find(|span| span.t.to_bits() == reach.to_bits())
        .copied()
        .expect("a transversally crossing candidate answers the aimed vertex at t = reach exactly");
    assert!(
        narrow.width() < wide.width(),
        "the transversal neighbour is the better-certified claim: {} against {}",
        narrow.width(),
        wide.width()
    );
    assert!(
        wide.precedes(&narrow),
        "and the wide candidate's whole interval is still in front of it, so the geometry \
         ORDERS them and nothing is tied: wide {wide:?}, narrow {narrow:?}"
    );
    assert!(
        wide.width() < reach - wide.t,
        "the number that makes that true: the wide candidate's interval is {} across while the \
         aimed vertex is {} further on",
        wide.width(),
        reach - wide.t
    );
    // The row: the certified order takes the nearer claim, so both the
    // reference and the service answer the wide candidate — one face,
    // not a refusal, because `precedes` decided it.
    let (answers, _) = reference.pick(&ray);
    let [hit] = &answers[..] else {
        panic!("the geometry orders these candidates, so one face is answered: {answers:?}");
    };
    assert_eq!(
        hit.t().to_bits(),
        RING_WIDE_CANDIDATE_T.to_bits(),
        "the certified order answers the wide candidate at t = {RING_WIDE_CANDIDATE_T}: {hit:?}"
    );
    let (_, eval) = session.landed_pair().expect("a landed pair");
    let picked = index
        .pick(eval, &ray)
        .expect("the pick resolves, and does not refuse: the geometry ordered these")
        .expect("the service meets the ring");
    assert_eq!(
        picked.t.to_bits(),
        hit.t().to_bits(),
        "the service answers the reference's t: {picked:?} against {hit:?}"
    );
    assert_eq!(
        (picked.t_lo.to_bits(), picked.t_hi.to_bits()),
        (hit.span.t_lo.to_bits(), hit.span.t_hi.to_bits()),
        "and the interval it was chosen on rides out on the hit: {picked:?}"
    );
}

/// The wide candidate's own rounded answer, `0.031` short of the aimed
/// vertex, and the door's answer on this ray. Re-derive from the
/// probe's failure message; a move here is a change in the class the
/// row above carries, not a baseline to restore.
const RING_WIDE_CANDIDATE_T: f64 = 1.448_765_272_489_762_4;

/// **The other side of the line: a wide candidate whose interval DOES
/// reach the aimed vertex, and the door refuses with both.** The
/// `tube_arc` corpus document at open, a `+y` ray through the mesh
/// vertex `(1.2534, 0.3843, −1.9521)`. `main` answered
/// `1.475_904_852_772_309_5` — `0.0041` short of the vertex, a
/// near-coplanar candidate whose rounded `t` is the smallest on the
/// ray. Its `t` interval is wider than that gap, so it does not
/// precede the transversal candidate that answers the vertex; the two
/// are a CERTIFIED TIE between two FACES, and nothing breaks it: the
/// door names both and chooses neither.
///
/// This is the class `work/edit/pick-a-wide-but-informative-barycentric-wins-over-the-transversal-neighbour`
/// names. It is not resolved by preferring the narrow claim any more —
/// the width was a rule the user did not ask for — it is DISCLOSED:
/// the aimed vertex is among the answers, and so is the edge-on face
/// in front of it. A door comparing rounded `t` reds here (it answers
/// the wide candidate alone); a door that kept the width key reds here
/// (it answers the vertex alone).
#[test]
fn a_wide_candidates_interval_reaching_the_aimed_vertex_refuses_with_both() {
    let tol = Tol::witness();
    let c = corpus::documents()
        .into_iter()
        .find(|c| c.name == "tube_arc")
        .expect("tube_arc is a corpus document");
    let mut session = DocSession::inline(c.doc.clone(), tol);
    session.pump();
    let index = common::index_at(&session, common::corpus_delta()).expect("tube_arc indexes");
    let vertex = Point3::new(
        1.253_413_016_011_234,
        0.384_323_569_889_266_14,
        -1.952_075_113_318_894_5,
    );
    let reach = REACH;
    let ray = aimed_along_y(vertex, 1.0);
    assert!(
        index.parts().iter().any(|part| {
            part.mesh().positions.iter().any(|p| {
                (p.x.to_bits(), p.y.to_bits(), p.z.to_bits())
                    == (vertex.x.to_bits(), vertex.y.to_bits(), vertex.z.to_bits())
            })
        }),
        "the probe's premise: the aimed point is a vertex of tube_arc's mesh"
    );
    let reference = FlatReference::of(&index);
    let admitted: Vec<TSpan> = reference
        .parts
        .iter()
        .flat_map(|flat| {
            flat.tree
                .ray(&ray)
                .into_iter()
                .filter_map(move |cand| ray_triangle(&ray, &flat.corners[cand.item]))
        })
        .collect();
    let wide = admitted
        .iter()
        .find(|span| span.t.to_bits() == TUBE_ARC_WIDE_CANDIDATE_T.to_bits())
        .copied()
        .expect("the near-coplanar candidate still answers short of the vertex");
    let narrow = admitted
        .iter()
        .find(|span| span.t.to_bits() == reach.to_bits())
        .copied()
        .expect("a transversal candidate answers the aimed vertex at t = reach exactly");
    assert!(
        wide.t < narrow.t,
        "the premise a rounded order acts on: {} is before {}",
        wide.t,
        narrow.t
    );
    assert!(
        !wide.precedes(&narrow) && !narrow.precedes(&wide),
        "the two intervals overlap, so the geometry does not order them: wide {wide:?}, narrow \
         {narrow:?}"
    );
    assert!(
        narrow.width() < wide.width(),
        "the vertex is the better-certified claim, which decides nothing: {} against {}",
        narrow.width(),
        wide.width()
    );
    let (answers, tied) = reference.pick(&ray);
    assert!(
        tied >= 2,
        "the certified tie has both candidates in it: {tied}"
    );
    assert!(
        answers.len() >= 2,
        "and they are different faces, so the door refuses rather than answering: {answers:?}"
    );
    let ts: Vec<u64> = answers.iter().map(|h| h.t().to_bits()).collect();
    assert!(
        ts.contains(&reach.to_bits()),
        "the aimed vertex at t = {reach} is one of the tied answers: {answers:?}"
    );
    assert!(
        ts.contains(&wide.t.to_bits()),
        "and so is the edge-on candidate short of it: {answers:?}"
    );
    let (_, eval) = session.landed_pair().expect("a landed pair");
    let picked = service_answers(&index, eval, &ray);
    assert_eq!(
        picked.iter().map(|h| h.t.to_bits()).collect::<Vec<_>>(),
        ts,
        "the service refuses with the reference's faces, in the reference's order: {picked:?}"
    );
    assert_eq!(
        picked
            .iter()
            .map(|h| (h.t_lo.to_bits(), h.t_hi.to_bits()))
            .collect::<Vec<_>>(),
        answers
            .iter()
            .map(|h| (h.span.t_lo.to_bits(), h.span.t_hi.to_bits()))
            .collect::<Vec<_>>(),
        "and each tied face's own interval rides out on its own hit: {picked:?}"
    );
}

/// What `main`'s rounded-`t` order answers on `tube_arc`'s ray:
/// `0.0041` short of the aimed vertex. Re-derive from the probe's
/// failure message.
const TUBE_ARC_WIDE_CANDIDATE_T: f64 = 1.475_904_852_772_309_5;

/// The wide candidate's conditioning `|det| / (|e1|·|e2|·|d|)`, as
/// [`Crossing::conditioning`] computes it. The number the row is named
/// for: it is the certification's own noise floor, not a decade above
/// it. Re-derive with
/// `cargo test -p viewer --test all -- index_memo::a_wide_but --nocapture`.
const RING_WIDE_CANDIDATE_CONDITIONING: f64 = 7.19e-16;

/// The standoff the aimed-point rows fire from. It is an INPUT, and
/// [`RING_CORNER_T`] below is an ANSWER a row expects back; they are
/// kept apart so that a row comparing the two is still comparing
/// something.
const REACH: f64 = 1.48;

/// A ray along `sense` y (`1.0` or `-1.0`) whose target `p` lies
/// [`REACH`] along it.
fn aimed_along_y(p: Point3<f64>, sense: f64) -> Ray {
    Ray {
        origin: Point3::new(p.x, p.y - sense * REACH, p.z),
        dir: Vec3::new(0.0, sense, 0.0),
    }
}

/// The ring probe's answer: the chord point's parameter as the
/// winning triangle's exact test rounds it. Re-derive from the
/// probe's failure message if the ring's tessellation changes.
const RING_CORNER_T: f64 = 1.48;

/// The last extrude distance or revolve angle in the document, scaled
/// — the gallery ring's own bump.
fn first_length_slot(doc: &ProfileDoc) -> (RecipeNodeId, SlotId, Expr) {
    let env = doc.param_env::<f64>();
    for &node in doc.order().iter().rev() {
        match doc.node(node).expect("a node") {
            editor_core::Node::Extrude { distance, .. } => {
                let value = editor_core::eval(distance, &env).expect("a literal distance");
                let expr = common::len(value * 1.03125);
                return (node, SlotId::Distance, expr);
            }
            editor_core::Node::Revolve { angle, .. } => {
                let value = editor_core::eval(angle, &env).expect("a literal angle");
                let expr = common::ang(value * 0.96875);
                return (node, SlotId::RevolveAngle, expr);
            }
            _ => {}
        }
    }
    panic!("no extrude or revolve in the document")
}
