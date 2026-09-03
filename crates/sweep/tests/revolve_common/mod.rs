//! Shared helpers for the revolve acceptance suites (M2 PR 5).
//! Each `revolve_*.rs` suite includes this via `mod revolve_common;`,
//! so it is loaded once per suite inside the crate's one aggregated
//! test binary (`tests/all.rs`).
#![allow(dead_code)]
// one instance per binary; no single consumer uses all of it
// These trees AUTHOR test documents, and a document that will not build is
// a test failure, not a value to hand back: the builders panic on a
// malformed fixture rather than thread a `Result` out to a caller whose
// only recourse is to unwrap it. Named here, where that code lives, rather
// than left to the crate-root allow of whatever module loads it.
#![allow(clippy::unwrap_used, clippy::panic)]
#![allow(unreachable_pub)] // why: root Cargo.toml, the `unreachable_pub` stanza

use geom_brep::{EdgeAuthority, EdgeDescription};
use geom_core::Tol;
use geom_core::{Point2, Point3, Vec2};
use profile::{Profile, ProfileLoop, SketchPlane, ValidatedProfile};
use sweep::RevolveAxis;
use topo::{Body, EdgeKey, LoopBoundary, LoopKey, validate, validate_closed, validate_geometric};

pub fn eps() -> f64 {
    Tol::witness().get().eps
}

pub fn p2(x: f64, y: f64) -> Point2<f64> {
    Point2::new(x, y)
}

pub fn validated(loops: Vec<ProfileLoop<f64>>) -> ValidatedProfile<f64> {
    Profile::new(SketchPlane::xy(), loops)
        .validate(Tol::witness())
        .unwrap()
}

/// The y-axis of the sketch plane (the canonical test axis: profiles
/// live in x ≥ 0).
pub fn axis_y() -> RevolveAxis<f64> {
    RevolveAxis {
        origin: p2(0.0, 0.0),
        dir: Vec2::new(0.0, 1.0),
    }
}

pub fn assert_all_tiers(body: &Body<f64>) {
    assert_eq!(validate(body), Ok(()));
    assert_eq!(validate_closed(body), Ok(()));
    assert_eq!(validate_geometric(body, Tol::witness()), Ok(()));
}

/// (v, e, f, r) of a body.
pub fn counts(body: &Body<f64>) -> (usize, usize, usize, usize) {
    let rings: usize = body.faces().map(|(_, f)| f.rings.len()).sum();
    (
        body.vertices().count(),
        body.edges().count(),
        body.faces().count(),
        rings,
    )
}

/// The edge's stored description.
pub fn description(body: &Body<f64>, edge: EdgeKey) -> EdgeDescription<f64> {
    let curve = body.get_edge(edge).unwrap().curve;
    body.get_curve_geom(curve)
        .unwrap()
        .certified()
        .unwrap()
        .description()
        .clone()
}

/// The edge's **authority record** (U2 Q3): who determined its locus.
///
/// This is the datum that survived the `IsoCurve` / `MappedCurve`
/// collapse. Before U2 a revolve suite read the description's VARIANT
/// to tell a natively-derived chart curve from a profile entity's
/// pushforward; both are chart images now, and the difference — who
/// determined the locus — is exactly what this record says.
pub fn authority(body: &Body<f64>, edge: EdgeKey) -> EdgeAuthority<f64> {
    let curve = body.get_edge(edge).unwrap().curve;
    body.get_curve_geom(curve)
        .unwrap()
        .certified()
        .unwrap()
        .authority()
}

/// The chart image an edge's conventional description draws, or a
/// panic naming what it found instead. Read `.surface` for the chart,
/// `.seam` for D1's seam obligation.
pub fn chart_image(body: &Body<f64>, edge: EdgeKey) -> geom_brep::ChartCurve<f64> {
    match description(body, edge) {
        EdgeDescription::Chart(c) => c,
        other => panic!("expected a conventional chart image, got {other:?}"),
    }
}

/// **A meridian the revolve DECLARED**: an image in the wall's own
/// chart that is not the chart's parameterization seam, carrying the
/// profile entity's pushforward as its authority. The post-collapse
/// spelling of "this edge keeps the conventional `MappedCurve`" — and
/// a stricter one, since it pins the chart as well as the class.
pub fn assert_declared_image_in(body: &Body<f64>, edge: EdgeKey, chart: topo::SurfaceKey) {
    let c = chart_image(body, edge);
    assert_eq!(c.surface, chart, "the image must be drawn in {chart:?}");
    assert!(!c.seam, "a declared image is not the chart's seam");
    assert!(
        authority(body, edge).is_declared(),
        "a sketch entity under the sweep map determined this locus"
    );
}

/// **The chart's own parameterization seam**: derived by the kernel
/// (no declaring sketch entity) and carrying D1's seam obligation.
/// The post-collapse spelling of "this edge re-describes as `Seam`".
pub fn assert_seam_of(body: &Body<f64>, edge: EdgeKey, chart: topo::SurfaceKey) {
    let c = chart_image(body, edge);
    assert_eq!(c.surface, chart, "the seam must be that of {chart:?}");
    assert!(c.seam, "the seam obligation must be carried");
    assert!(
        !authority(body, edge).is_declared(),
        "a seam is derived by the kernel, not declared"
    );
}

/// Probe points of a loop in `next` order: each start vertex plus
/// interior carrier samples **in the half-edge's traversal direction**
/// (the mate half runs its carrier backward). Dense enough (7 interior
/// samples) that full-period rim circles polygonalize honestly — the
/// PR 4 review's oracle refined for revolve's closed carriers.
pub fn loop_probe_points(body: &Body<f64>, r#loop: LoopKey) -> Vec<Point3<f64>> {
    let LoopBoundary::Cycle { first } = body.get_loop(r#loop).unwrap().boundary else {
        panic!("loop has no cycle");
    };
    let mut pts = Vec::new();
    for he in body.loop_cycle(first).unwrap() {
        let he_data = body.get_half_edge(he).unwrap();
        pts.push(
            *body
                .get_point(body.get_vertex(he_data.start).unwrap().point)
                .unwrap(),
        );
        let edge = body.get_edge(he_data.edge).unwrap();
        let forward = edge.he_plus == he;
        let ec = body
            .get_curve_geom(edge.curve)
            .unwrap()
            .certified()
            .unwrap();
        let (t0, t1) = ec.params();
        for i in 1..8 {
            let s = f64::from(i) / 8.0;
            let t = if forward {
                t0 + (t1 - t0) * s
            } else {
                t1 + (t0 - t1) * s
            };
            pts.push(ec.carrier().eval(t));
        }
    }
    pts
}

/// The anchor both divergence folds below measure their tetrahedra
/// from: the bounding-box centre of every probe point on the body,
/// overflow-robust midpoint. Over ℝ a closed surface's divergence sum
/// is the same from any single anchor, so this is a conditioning
/// choice — ONE anchor, shared by every face, sitting at the body's
/// own scale rather than at whatever distance the body was placed from
/// the world origin.
pub fn probe_anchor(body: &Body<f64>) -> Point3<f64> {
    let mut bbox: Option<(Point3<f64>, Point3<f64>)> = None;
    for (_, face) in body.faces() {
        for lk in core::iter::once(face.outer).chain(face.rings.iter().copied()) {
            for p in loop_probe_points(body, lk) {
                bbox = Some(match bbox {
                    None => (p, p),
                    Some((lo, hi)) => (lo.min(p), hi.max(p)),
                });
            }
        }
    }
    match bbox {
        Some((lo, hi)) => lo + (hi - lo) * 0.5,
        None => Point3::origin(),
    }
}

/// Independent orientation oracle: signed volume by the divergence
/// fan over every loop of every face (sign is the oracle — chordal on
/// curved faces; the PR 4 review's `signed_volume`).
pub fn signed_volume(body: &Body<f64>) -> f64 {
    let o = probe_anchor(body);
    let mut six_v = 0.0;
    for (_, face) in body.faces() {
        for lk in core::iter::once(face.outer).chain(face.rings.iter().copied()) {
            let pts = loop_probe_points(body, lk);
            let p1 = pts[0];
            for i in 1..pts.len() - 1 {
                let a = p1 - o;
                let b = pts[i] - o;
                let c = pts[i + 1] - o;
                six_v += a.dot(b.cross(c));
            }
        }
    }
    six_v / 6.0
}

/// [`signed_volume`] with per-face **lift points**: a face listed in
/// `lifts` is fanned from its lift point over the full cyclic boundary
/// (wrap included) instead of from a boundary vertex.
///
/// Needed wherever a fan from a boundary vertex spans no volume, which
/// happens by two DISTINCT mechanisms — the second one cost a row its
/// meaning before it was named:
///
/// * **Coplanar boundary.** The two-band sphere/cone patches, whose two
///   meridians lie in one plane, so every fan triangle is degenerate.
/// * **Mirror-paired faces.** The donut's two half-tori are bounded by
///   the same pair of full-period rims plus the two halves of one seam
///   meridian, so their fans are mirror images and cancel each other
///   exactly. Each face's fan is perfectly healthy on its own; it is
///   the SUM over the body that is identically zero. A row asserting
///   `> 0.0` on such a body is reading float noise, not a volume.
///
/// **What the lift may be, stated honestly.** One face's fan is
/// `(q − o)·A_L`, where `A_L` is that loop's own vector area — it is
/// LINEAR in the lift `q`, with no interior-ness anywhere in it. So the
/// old "must be an interior surface point" was an overclaim: what a
/// lift must do is give the pair of mirror faces a non-zero separation
/// along `A_L`, since for the donut the total collapses to
/// `(q₀ − q₁)·A / 6` and depends on the lift DIFFERENCE alone —
/// translating both lifts together changes nothing at all.
///
/// The contract is therefore narrower than it looks, and it is the only
/// thing callers should lean on: **the SIGN tracks the winding.** The
/// magnitude is a chordal artefact of the sampling, and agreement
/// between two different lift pairs is a consequence of that linearity,
/// not independent evidence that the oracle is well conditioned.
pub fn signed_volume_lifted(body: &Body<f64>, lifts: &[(topo::FaceKey, Point3<f64>)]) -> f64 {
    let o = probe_anchor(body);
    let mut six_v = 0.0;
    for (fk, face) in body.faces() {
        let lift = lifts.iter().find(|(k, _)| *k == fk).map(|(_, q)| *q);
        for lk in core::iter::once(face.outer).chain(face.rings.iter().copied()) {
            let pts = loop_probe_points(body, lk);
            match lift {
                Some(q) => {
                    let n = pts.len();
                    for i in 0..n {
                        let a = q - o;
                        let b = pts[i] - o;
                        let c = pts[(i + 1) % n] - o;
                        six_v += a.dot(b.cross(c));
                    }
                }
                None => {
                    let p1 = pts[0];
                    for i in 1..pts.len() - 1 {
                        let a = p1 - o;
                        let b = pts[i] - o;
                        let c = pts[i + 1] - o;
                        six_v += a.dot(b.cross(c));
                    }
                }
            }
        }
    }
    six_v / 6.0
}

/// A byte-comparable dump of the body's arenas plus the key bundle
/// (the D9 rebuild oracle; the PR 4 review's `dump` shape).
pub fn dump(t: &sweep::Revolved<f64>) -> String {
    let mut s = String::new();
    for (k, p) in t.body.points() {
        s.push_str(&format!("{k:?} {p:?}\n"));
    }
    for (k, c) in t.body.curves() {
        // Sweep bodies carry certified carriers only (no M3 null
        // scaffolding); dump the whole entry so a scaffolding entry
        // would still show loudly rather than being skipped.
        match c.certified() {
            Some(c) => s.push_str(&format!(
                "{k:?} {:?} {:?} {:?}\n",
                c.description(),
                c.params(),
                c.certificate()
            )),
            None => s.push_str(&format!("{k:?} {c:?}\n")),
        }
    }
    for (k, srf) in t.body.surfaces() {
        s.push_str(&format!("{k:?} {srf:?}\n"));
    }
    for (k, f) in t.body.faces() {
        s.push_str(&format!("{k:?} {f:?}\n"));
    }
    for (k, l) in t.body.loops() {
        s.push_str(&format!("{k:?} {l:?}\n"));
    }
    for (k, h) in t.body.half_edges() {
        s.push_str(&format!("{k:?} {h:?}\n"));
    }
    s.push_str(&format!("{:?} {:?} {:?}\n", t.walls, t.rims, t.kind));
    s
}

/// The kernel-coupled Pappus MAGNITUDE oracle, promoted from the PR 5
/// review suite (M2 PR 7): `V = |θ|/2 · |∮ r² dz|` with r/z measured
/// against the placed axis, the meridian line integral sampled densely
/// (256 chords per edge) along each meridian edge's stored carrier in
/// `he_plus` order. Independent of faces, loops, and the closed-form
/// divergence formulation — the sanity cross-check for revolved
/// bodies' exact mass properties (never the source of truth: it is a
/// converging quadrature, not a closed form).
pub fn meridian_pappus_volume(
    body: &Body<f64>,
    meridians: &[EdgeKey],
    angle: f64,
    axis_o: Point3<f64>,
    axis_d: geom_core::Vec3<f64>,
) -> f64 {
    let d = axis_d.normalize();
    let n = 256;
    let mut integral = 0.0;
    for &ek in meridians {
        let e = body.get_edge(ek).unwrap();
        let c = body.get_curve_geom(e.curve).unwrap().certified().unwrap();
        let (t0, t1) = c.params();
        let mut prev = c.carrier().eval(t0);
        for i in 1..=n {
            let t = t0 + (t1 - t0) * (f64::from(i) / f64::from(n));
            let p = c.carrier().eval(t);
            let r_mid = {
                let m = prev.lerp(p, 0.5);
                let w = m - axis_o;
                let w_perp = w - d * w.dot(d);
                w_perp.norm()
            };
            let dz = (p - axis_o).dot(d) - (prev - axis_o).dot(d);
            integral += r_mid * r_mid * dz;
            prev = p;
        }
    }
    (angle.abs() / 2.0) * integral.abs()
}

/// [`meridian_pappus_volume`] for the canonical y-axis full revolves,
/// taking the meridian chain out of the key bundle (omitted on-axis
/// segments contribute nothing).
pub fn full_pappus_y(t: &sweep::Revolved<f64>) -> f64 {
    let sweep::RevolvedKind::Full { meridians, .. } = &t.kind else {
        panic!("full revolve expected")
    };
    let meridians = &meridians[0];
    let chain: Vec<EdgeKey> = meridians.iter().filter_map(|m| *m).collect();
    meridian_pappus_volume(
        &t.body,
        &chain,
        core::f64::consts::TAU,
        Point3::origin(),
        geom_core::Vec3::new(0.0, 1.0, 0.0),
    )
}

/// **The containment suites' probe offset**, in one home.
///
/// A probe has two jobs — definitely outside the ambiguity band, and
/// definitely inside (or outside) the body it means — and every
/// containment door suite has needed the same expression for the first
/// of them. It had four copies before this was written; what varies
/// between suites is not the expression but the SHELL each one has to
/// clear, which stays with the suite that measures it.
///
/// The shared part of the argument:
///
/// * it scales with ε, because a probe that does not is a probe that
///   means something different in each tolerance row;
/// * the FLOOR keeps it clear of the escalation shells, which are wider
///   than `K·ε` wherever a discriminant decays to a multiple root (the
///   cone's apex shell goes as `√ε`; the torus's tangency shell as the
///   cube root);
/// * the CEILING keeps it inside a unit-sized body: an offset scaling
///   freely with ε reaches 1.0 at the 1e-6 row, and a probe "just inside
///   the wall" at that distance is out the other side.
///
/// **The clamp saturates at every shipped ε row** — `1e6·ε` lands inside
/// `[1e-3, 0.1]` only for `ε ∈ (1e-9, 1e-7)` and no row of the matrix
/// draws one — so what governs the offsets the gated rows actually use
/// is the clamp, not the ε-scaling. The scaling is kept because it is
/// the right law between the bounds, not because a gated run exercises
/// it.
///
/// Two suites deliberately do NOT call this and should not be pointed at
/// it: `bool2_r2_probes` re-derives the expression twice because it is a
/// reviewer probe CHECKING it (one that called the code under test would
/// check nothing), and `bool2_r1_probes` uses a `√`-scaled variant with
/// its own bounds, sized to the cone's apex shell rather than to the
/// band. `m5_pr9c_sphere_doors` uses a different expression entirely.
pub fn probe_offset() -> f64 {
    (1e6 * Tol::witness().get().eps).clamp(1e-3, 0.1)
}
