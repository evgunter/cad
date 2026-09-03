//! **VERBS-CHAMFER**: the plane–plane chamfer over the fillet's own
//! infrastructure — the chamfered cube, the refusals a consumer
//! actually reaches, and the two claims the geometry rests on (the
//! strip's outward chart normal, and the foot the two verbs share on a
//! right corner).

#![allow(clippy::unwrap_used, clippy::expect_used, clippy::panic)]

use geom::Surface;
use geom_core::{Point2, Point3, Tol, Vec3};
use profile::{Profile, ProfileLoop, ProfileVertex, RawLoop, SketchPlane};
use sweep::blend::arms::chamfer_strip;
use sweep::blend::build::fillet_edges;
use sweep::blend::{BlendError, CornerConfig, RunOutPolicy};
use sweep::chamfer::chamfer_edges;
use sweep::test_support::cube;
use sweep::{Extrusion, extrude};
use topo::query;
use topo::{Body, EdgeKey};

/// The cube side, meters.
const L: f64 = 1.0;
/// The chamfer setback, meters.
const D: f64 = 0.1;

/// **The chamfered cube's volume in closed form.**
///
/// The solid is the cube intersected with the twelve strip planes and
/// the eight corner planes, so the removed material is the union of
/// twelve triangular prisms (leg `d`, cross-section `d²/2`, the full
/// edge length) and eight corner tetrahedra `{x + y + z < 2d}`
/// (volume `4d³/3`), and inclusion–exclusion over the four sets that
/// meet at each corner over-counts by exactly `2d³` there.
///
/// `a³ − 6·a·d² + (16/3)·d³`, which at `d = a/2` gives `a³/6` — the
/// octahedron on the cube's face centres, the degenerate end of the
/// family.
fn chamfered_cube_volume(a: f64, d: f64) -> f64 {
    a.powi(3) - 6.0 * a * d * d + (16.0 / 3.0) * d.powi(3)
}

/// Every vertex point of a body, in a deterministic total order — so
/// two bodies' vertex SETS can be compared bit for bit.
fn sorted_points(body: &Body<f64>) -> Vec<(f64, f64, f64)> {
    let mut pts: Vec<(f64, f64, f64)> = body
        .vertices()
        .filter_map(|(k, _)| body.get_vertex(k))
        .filter_map(|v| body.get_point(v.point))
        .map(|p| (p.x, p.y, p.z))
        .collect();
    pts.sort_by(|a, b| a.partial_cmp(b).expect("finite coordinates"));
    pts
}

/// **THE CHAMFERED CUBE** — all twelve edges at equal setback: 26
/// faces (6 shrunk squares, 12 strips, 8 corner triangles), every one
/// of them a PLANE, tiers 1–3 green, the census and Euler relation
/// pinned, the certified volume against the closed form, and a
/// watertight mesh.
#[test]
fn the_chamfered_cube() {
    let body = cube(L, Tol::witness());
    let out = chamfer_edges(&body, &query::all_edges(&body), D, Tol::witness())
        .expect("a cube's twelve edges chamfer");
    let out_body = out.body;

    assert_eq!(topo::validate(&out_body), Ok(()), "tier 1");
    assert_eq!(topo::validate_closed(&out_body), Ok(()), "tier 2");
    assert_eq!(
        topo::validate_geometric(&out_body, Tol::witness()),
        Ok(()),
        "tier 3"
    );

    assert_eq!(out.blend_faces.len(), 12, "one strip per edge");
    assert_eq!(out.corner_faces.len(), 8, "one patch per corner");
    assert_eq!(
        out.band_faces.len(),
        0,
        "a chamfer has no closed-chain band"
    );

    let (v, e, f) = (
        out_body.vertices().count(),
        out_body.edges().count(),
        out_body.faces().count(),
    );
    assert_eq!((v, e, f), (24, 48, 26), "census");
    assert_eq!(v as i64 - e as i64 + f as i64, 2, "Euler–Poincaré");

    // Every face is a plane — the whole claim of the analytic case.
    for (k, _) in out_body.faces() {
        let fd = out_body.get_face(k).expect("a face");
        assert!(
            matches!(
                out_body.get_surface(fd.surface),
                Some(Surface::Plane { .. })
            ),
            "a chamfered cube carries planes only; face {k:?} does not"
        );
    }

    let want = chamfered_cube_volume(L, D);
    let props = topo::mass_properties(&out_body, Tol::witness()).expect("closed-form props");
    assert!(
        (props.volume - want).abs() <= 1e-12 * want,
        "volume {} vs closed form {want}",
        props.volume
    );
    assert_eq!(props.volume_pad, 0.0, "closed forms need no pad");

    let mesh = mesh::tessellate(&out_body, 5e-3, Tol::witness()).expect("tessellates");
    mesh::validate::check_mesh(&mesh).expect("watertight");
}

/// **The birth records close.** `output = (source − dead) ⊎ minted`,
/// in both directions, for the chamfer's own rows: twelve strips off
/// twelve edges, eight patches off eight corners, and every source
/// edge and corner vertex accounted for as retired.
#[test]
fn the_chamfer_records_every_birth_and_death() {
    let body = cube(L, Tol::witness());
    let source_edges = query::all_edges(&body);
    let out = chamfer_edges(&body, &source_edges, D, Tol::witness()).expect("chamfers");
    let rec = out.naming.expect("the surgery is the only producer");

    assert_eq!(rec.blends.len(), 12, "a strip per source edge");
    assert_eq!(rec.corners.len(), 8, "a patch per source vertex");
    assert!(rec.bands.is_empty(), "no rim phase ran");
    assert_eq!(rec.dead.edges.len(), 12, "every source edge is excised");
    assert_eq!(rec.dead.vertices.len(), 8, "every sharp corner is fused");

    // Every retired source key is gone from the output; every strip and
    // patch key is present in it and was not in the source.
    for e in &rec.dead.edges {
        assert!(out.body.get_edge(*e).is_none(), "a retired edge survives");
    }
    for v in &rec.dead.vertices {
        assert!(
            out.body.get_vertex(*v).is_none(),
            "a retired vertex survives"
        );
    }
    let minted = rec
        .blends
        .iter()
        .map(|(f, _)| *f)
        .chain(rec.corners.iter().map(|(f, _)| *f));
    for face in minted {
        assert!(
            out.body.get_face(face).is_some(),
            "a minted face is missing from the output"
        );
    }
    // Every source edge the request named is retired, and the rows
    // name the source they were minted for.
    let mut blended: Vec<EdgeKey> = rec.blends.iter().map(|(_, e)| *e).collect();
    blended.sort_unstable();
    let mut requested = source_edges;
    requested.sort_unstable();
    assert_eq!(blended, requested, "a strip per requested edge, no other");
}

/// **The two verbs put their feet in the same place on a RIGHT
/// corner**, and the fillet's are still the exact numbers.
///
/// The rolling ball at rest is at distance `r` inside all three
/// supports, so its foot on each is on both of that support's
/// trimlines; the chamfer crosses those trimlines directly. In exact
/// arithmetic the two derivations coincide on a cube.
///
/// **The fillet's half is asserted BIT-EXACTLY**, and that is the
/// point of the row: the feet moved from the support-admission door
/// into the plan, and a filleted cube whose 24 vertices are not
/// literally `{0, r, L − r, L}` is a fillet whose foot derivation
/// changed. The chamfer's half is asserted to within an ulp instead —
/// a line-crossing solve and a projection are two different exact
/// forms of one point, and rounding them to the same `f64` is not
/// something either form promises.
#[test]
fn fillet_and_chamfer_agree_on_a_right_corner() {
    let body = cube(L, Tol::witness());
    let edges = query::all_edges(&body);
    let filleted = fillet_edges(&body, &edges, D, Tol::witness()).expect("fillets");
    let chamfered = chamfer_edges(&body, &edges, D, Tol::witness()).expect("chamfers");

    let want: Vec<(f64, f64, f64)> = {
        // Each of the 24 is a foot: one coordinate on a face of the
        // cube, the other two set back by exactly D.
        let mut v = Vec::with_capacity(24);
        for face in [0.0, L] {
            for a in [D, L - D] {
                for b in [D, L - D] {
                    v.push((face, a, b));
                    v.push((a, face, b));
                    v.push((a, b, face));
                }
            }
        }
        v.sort_by(|x, y| x.partial_cmp(y).expect("finite"));
        v
    };
    assert_eq!(sorted_points(&filleted.body), want, "the fillet's feet");
    let got = sorted_points(&chamfered.body);
    assert_eq!(got.len(), want.len(), "the chamfer's vertex count");
    // Matched by proximity, not by order: an ulp of difference moves a
    // coordinate across the sort key, and the claim is about the
    // POINTS, not about how they sort.
    for w in &want {
        let near = got
            .iter()
            .filter(|g| {
                (g.0 - w.0)
                    .abs()
                    .max((g.1 - w.1).abs())
                    .max((g.2 - w.2).abs())
                    <= 1e-15
            })
            .count();
        assert_eq!(
            near, 1,
            "exactly one chamfer foot sits at {w:?}, found {near}"
        );
    }
}

/// **A partial request refuses typed as a RUN-OUT** — the first thing
/// a consumer tries. One edge of a cube terminates at two trivalent
/// corners whose other four edges are not requested, which is a
/// property of the REQUEST, not of the corners' configuration.
#[test]
fn one_edge_of_a_cube_refuses_as_a_run_out() {
    let body = cube(L, Tol::witness());
    let edges = query::all_edges(&body);
    let err = chamfer_edges(&body, &edges[..1], D, Tol::witness())
        .expect_err("a partially-requested corner is a run-out");
    assert!(
        matches!(err.error, BlendError::UnsupportedRunOut { .. }),
        "the request's coverage is what ran out: {err:?}"
    );
    let text = format!("{err}");
    assert!(
        text.contains("not implemented"),
        "the refusal names the unbuilt door: {text}"
    );
}

/// **A curved support refuses through the chamfer's OWN arm table**,
/// with its own recourse: the missing door is the chamfer over a
/// curved support, not the fillet's canal surface, and the
/// plane–sphere pair the fillet offers is no alternative here.
#[test]
fn a_curved_support_refuses_with_the_chamfers_own_sentence() {
    let cyl = cylinder(0.5, 1.0);
    let edges = query::all_edges(&cyl);
    let err = chamfer_edges(&cyl, &edges, D, Tol::witness())
        .expect_err("a plane–cylinder rim has no ruled strip");
    assert!(
        matches!(err.error, BlendError::ChamferArmUnsupported { .. }),
        "the arm table is what refused: {err:?}"
    );
    let text = format!("{err}");
    assert!(
        text.starts_with("chamfer:") && text.contains("both planes"),
        "the refusal speaks as the chamfer: {text}"
    );
}

/// **A CONCAVE plane–plane edge gets a decided, correct verdict.**
///
/// An L-bracket's inner edge is concave and its two terminations are
/// trivalent vertices with two convex edges and one concave one — a
/// mixed-convexity corner, which is the OQ6 configuration tag for it,
/// and the policy that would handle it is the feathered run-out (a
/// corner patch cannot help where the band changes sides). v1 refuses;
/// the refusal names the configuration rather than a generic failure.
#[test]
fn an_l_brackets_inner_edge_refuses_on_its_corner_configuration() {
    let bracket = l_bracket();
    let inner = concave_edge(&bracket);
    let err = chamfer_edges(&bracket, &[inner], D, Tol::witness())
        .expect_err("v1 does not chamfer a concave edge");
    match err.error {
        BlendError::UnsupportedCorner {
            corner: CornerConfig::MixedConvexity { convex },
            policy,
            ..
        } => {
            assert_eq!(convex, 2, "two of the three incident edges are convex");
            assert_eq!(policy, Some(RunOutPolicy::RunOutFeather));
        }
        other => panic!("expected a mixed-convexity corner refusal, got {other:?}"),
    }
}

/// **The strip's chart normal is outward on a CONCAVE edge too** — the
/// claim that lets the chamfer mint every band with sense `true` and
/// carry no convexity parameter in its geometry.
///
/// The witness is the concave 90° edge along `+z` between the support
/// `{y = 0, x > 0}` (outward `+y`) and the support `{x = 0, y > 0}`
/// (outward `+x`): material fills everything but the first quadrant,
/// so the strip's outward normal is `(1, 1, 0)/√2` — which is what the
/// closed form returns, with no side chosen anywhere.
#[test]
fn the_strip_mints_an_outward_normal_on_a_concave_edge() {
    let p = Point3::new(0.0, 0.0, 0.0);
    let tau = Vec3::new(0.0, 0.0, 1.0);
    let n_a = Vec3::new(0.0, 1.0, 0.0);
    let n_b = Vec3::new(1.0, 0.0, 0.0);
    let strip = chamfer_strip(p, tau, n_a, n_b, D);
    let Surface::Plane { normal, origin, .. } = strip.surface else {
        panic!("a chamfer strip is a plane");
    };
    let want = (n_a + n_b).normalize();
    assert!(
        (normal - want).norm() < 1e-15,
        "the strip's chart normal must be the supports' outward bisector, got {normal:?}"
    );
    // Both trimlines sit inside their own supports: the first support
    // extends toward +x from the edge, the second toward +y.
    assert!(origin.x > 0.0, "the first trimline is inside its support");
    assert_eq!(strip.trim_a.1, D, "the setback is the request's, exactly");
    assert_eq!(strip.trim_b.1, D);
    assert_eq!(strip.spine_curvature, 0.0, "a ruled strip has no spine");
}

// ------------------------------------------------------------------
// Fixtures.
// ------------------------------------------------------------------

/// A circular prism: two half-arc profile segments extruded, so every
/// rim edge has a plane and a CYLINDER for supports.
fn cylinder(r: f64, h: f64) -> Body<f64> {
    let lp = ProfileLoop::new(vec![
        ProfileVertex::new(Point2::new(-r, 0.0), 1.0),
        ProfileVertex::new(Point2::new(r, 0.0), 1.0),
    ]);
    let profile = Profile::new(SketchPlane::xy(), vec![lp])
        .validate(Tol::witness())
        .expect("a circle is a valid profile");
    extrude(&profile, Extrusion::Distance(h), Tol::witness())
        .expect("a circular prism")
        .body
}

/// An L-bracket: the six-vertex L profile extruded by 1 m. Its one
/// reflex profile corner becomes the body's one concave edge.
fn l_bracket() -> Body<f64> {
    let lp = ProfileLoop::new(
        [
            (0.0, 0.0),
            (2.0, 0.0),
            (2.0, 1.0),
            (1.0, 1.0),
            (1.0, 2.0),
            (0.0, 2.0),
        ]
        .into_iter()
        .map(|(x, y)| ProfileVertex::new(Point2::new(x, y), 0.0))
        .collect(),
    );
    let profile = Profile::new(SketchPlane::xy(), vec![lp])
        .validate(Tol::witness())
        .expect("the L is a valid profile");
    extrude(&profile, Extrusion::Distance(1.0), Tol::witness())
        .expect("the bracket extrudes")
        .body
}

/// The bracket's one concave edge: the vertical wall–wall edge over the
/// reflex profile corner at `(1, 1)`, found by its endpoints rather
/// than by index.
fn concave_edge(body: &Body<f64>) -> EdgeKey {
    let at = |p: Point3<f64>| (p.x - 1.0).abs() < 1e-12 && (p.y - 1.0).abs() < 1e-12;
    body.edges()
        .find(|(k, _)| {
            let Some(e) = body.get_edge(*k) else {
                return false;
            };
            let Some(h) = body.get_half_edge(e.he_plus) else {
                return false;
            };
            let Some(end) = body.half_edge_end(e.he_plus) else {
                return false;
            };
            let pt = |v| {
                body.get_vertex(v)
                    .and_then(|x| body.get_point(x.point))
                    .copied()
            };
            match (pt(h.start), pt(end)) {
                (Some(a), Some(b)) => at(a) && at(b),
                _ => false,
            }
        })
        .map(|(k, _)| k)
        .expect("the bracket has a vertical edge over its reflex corner")
}
