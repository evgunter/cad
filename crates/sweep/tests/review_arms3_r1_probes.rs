//! **VERBS-ARMS-3 r1 review probes** — adversarial rows authored
//! independently of the unit's own suite.
//!
//! What each row would catch:
//!
//! - the general (unequal-radii) sphere×sphere reduction agreeing with
//!   the radical-plane closed form on a body the unit never built;
//! - the arm surviving a rigid re-pose (nothing in the closed form may
//!   read a global axis);
//! - the opposite-sense fold on a bitten ball (a cavity wall's sense
//!   bit is the other one), and the HONEST refusal when the offset
//!   circles stop crossing (containment / near-tangency poses);
//! - a genuine valence-4 vertex (a chamfered cube patch corner)
//!   keeping its `NEdgeVertex` refusal — the seam recognition must not
//!   swallow it;
//! - a torus-walled rim still refusing `SpineUnsupported` through its
//!   own gate, with the roster now advertising sphere–sphere;
//! - a second pole-touching revolve (not the unit's lantern)
//!   reproducing both halves of the seam-vertex story.

#![allow(clippy::unwrap_used, clippy::expect_used, clippy::panic)]

use geom::{Curve3, Surface};
use geom_core::{Affine3, Band, Point2, Point3, Tol, Vec3};
use profile::ProfileVertex;
use sweep::Revolution;
use sweep::blend::build::fillet_edges;
use sweep::blend::{BlendError, CornerConfig, RunOutPolicy};
use sweep::test_support::revolved_about_y;
use topo::{Body, EdgeKey, SurfaceKey, transform_rigid, validate_geometric};

fn tol() -> Tol {
    Tol::witness()
}

fn band() -> Band {
    Band::new(tol().eps(), tol().k() * tol().eps()).unwrap()
}

fn v(x: f64, y: f64, bulge: f64) -> ProfileVertex<f64> {
    ProfileVertex::new(Point2::new(x, y), bulge)
}

/// The bulge of an arc from `p1` to `p2` on the circle centred `c`
/// (radius `r`), taking the SHORT way round: `tan(sweep / 4)`, with the
/// sweep SIGNED by the turn direction (CCW positive).
fn bulge(c: Point2<f64>, r: f64, p1: Point2<f64>, p2: Point2<f64>) -> f64 {
    let u1 = ((p1.x - c.x) / r, (p1.y - c.y) / r);
    let u2 = ((p2.x - c.x) / r, (p2.y - c.y) / r);
    let cos = u1.0 * u2.0 + u1.1 * u2.1;
    let sin = u1.0 * u2.1 - u1.1 * u2.0;
    (sin.atan2(cos) / 4.0).tan()
}

/// The unordered pair of surfaces an edge's two supports carry.
fn supports(body: &Body<f64>, edge: EdgeKey) -> (SurfaceKey, SurfaceKey) {
    let e = body.get_edge(edge).unwrap();
    let face = |he| {
        body.get_face(
            body.get_loop(body.get_half_edge(he).unwrap().parent_loop)
                .unwrap()
                .face,
        )
        .unwrap()
        .surface
    };
    let (a, b) = (face(e.he_plus), face(e.he_minus));
    if a <= b { (a, b) } else { (b, a) }
}

/// Every circular edge with carrier radius `r` whose two supports are
/// DIFFERENT surfaces (excludes chart seams), anywhere in space.
fn rims_of_radius(body: &Body<f64>, r: f64) -> Vec<EdgeKey> {
    body.edges()
        .filter_map(|(k, e)| {
            let c = body.get_curve_geom(e.curve)?.certified()?;
            match *c.carrier() {
                Curve3::Circle { radius, .. } if (radius - r).abs() < 1e-9 => Some(k),
                _ => None,
            }
        })
        .filter(|k| {
            let (a, b) = supports(body, *k);
            a != b
        })
        .collect()
}

/// The band face's torus datum.
fn band_torus(body: &Body<f64>, face: topo::FaceKey) -> (Point3<f64>, Vec3<f64>, f64, f64) {
    let s = body
        .get_surface(body.get_face(face).unwrap().surface)
        .unwrap();
    match s {
        &Surface::Torus {
            center,
            axis,
            major_radius,
            minor_radius,
            ..
        } => (center, axis, major_radius, minor_radius),
        other => panic!("the band face is a torus, got {other:?}"),
    }
}

// ------------------------------------------------------------------
// 1. The general pair: unequal radii, unequal offsets.
// ------------------------------------------------------------------

/// Sphere A: centre (0,0), R = 1. Sphere B: centre (0, 2.1), R = 1.7.
/// They cross at the 3-4-5 rim (0.8, 0.6). The lens between them,
/// bored on-axis at 0.6 so the profile stays annular.
fn unequal_lentil() -> Body<f64> {
    let ca = Point2::new(0.0, 0.0);
    let cb = Point2::new(0.0, 2.1);
    let rim = Point2::new(0.8, 0.6);
    // Bore crossings: on A exact (0.6, 0.8); on B (0.6, 2.1 − √2.53).
    let a_lo = Point2::new(0.6, 2.1 - 2.53f64.sqrt());
    let a_hi = Point2::new(0.6, 0.8);
    revolved_about_y(
        vec![
            v(a_lo.x, a_lo.y, bulge(cb, 1.7, a_lo, rim)),
            v(rim.x, rim.y, bulge(ca, 1.0, rim, a_hi)),
            v(a_hi.x, a_hi.y, 0.0),
        ],
        Revolution::Full,
        tol(),
    )
}

/// **The general reduction against the radical form.** The ball centre
/// solves |c − ca| = R₁ − r, |c − cb| = R₂ − r with R₁ ≠ R₂, so the
/// spine is NOT level with the rim — the radical station and the
/// Pythagorean radius below are derived here from nothing but the two
/// sphere data.
#[test]
fn an_unequal_lentil_fillets_to_the_radical_closed_form() {
    let r = 0.05;
    let source = unequal_lentil();
    // Guard the fixture itself: both walls are the intended spheres.
    let mut spheres: Vec<(f64, f64)> = source
        .faces()
        .filter_map(|(_, f)| match source.get_surface(f.surface) {
            Some(&Surface::Sphere { center, radius, .. }) => Some((center.y, radius)),
            _ => None,
        })
        .collect();
    spheres.sort_by(|a, b| a.partial_cmp(b).unwrap());
    assert_eq!(spheres.len(), 2, "two sphere walls");
    assert!((spheres[0].0).abs() < 1e-9 && (spheres[0].1 - 1.0).abs() < 1e-9);
    assert!((spheres[1].0 - 2.1).abs() < 1e-9 && (spheres[1].1 - 1.7).abs() < 1e-9);

    let arcs = rims_of_radius(&source, 0.8);
    assert_eq!(arcs.len(), 1, "one closed sphere-sphere rim");
    let out = fillet_edges(&source, &arcs, r, band(), tol())
        .unwrap_or_else(|e| panic!("the unequal equator fillets, got {e:?}"));
    let (center, _, major, minor) = band_torus(&out.body, out.band_faces[0]);
    // The radical form: offsets o₁ = 1 − r, o₂ = 1.7 − r, centres 2.1
    // apart; station y* from A's centre, radius x* by Pythagoras.
    let (o1, o2, d) = (1.0 - r, 1.7 - r, 2.1);
    let y_star = (d * d + o1 * o1 - o2 * o2) / (2.0 * d);
    let x_star = (o1 * o1 - y_star * y_star).sqrt();
    assert!(
        (major - x_star).abs() < 1e-12,
        "spine radius {x_star}, got {major}"
    );
    assert!((minor - r).abs() < 1e-12, "tube {r}, got {minor}");
    assert!(
        (center.y - y_star).abs() < 1e-12,
        "spine station {y_star}, got {}",
        center.y
    );
}

// ------------------------------------------------------------------
// 2. The arm survives a rigid re-pose.
// ------------------------------------------------------------------

/// The unit's own symmetric lentil (spheres R = 1 at (0, ∓0.6), bored
/// at 0.6), rebuilt here so this file stands alone.
fn symmetric_lentil() -> Body<f64> {
    let b = (0.96f64.acos() / 4.0).tan();
    revolved_about_y(
        vec![v(0.6, -0.2, b), v(0.8, 0.0, b), v(0.6, 0.2, 0.0)],
        Revolution::Full,
        tol(),
    )
}

/// **Nothing in the closed form reads a global axis.** The same lentil
/// rotated off every coordinate plane and translated off the origin
/// fillets to the same torus, carried by the body's own stored frames.
#[test]
fn a_reposed_lentil_fillets_in_its_own_frame() {
    let r = 0.05;
    let map =
        Affine3::rotation_about_axis(Point3::new(0.0, 0.0, 0.0), Vec3::new(1.0, 0.0, 0.0), 0.7);
    let posed = transform_rigid(&symmetric_lentil(), &map, tol()).unwrap();
    let posed = transform_rigid(
        &posed,
        &Affine3::translation(Vec3::new(0.3, -0.2, 0.5)),
        tol(),
    )
    .unwrap();
    let arcs = rims_of_radius(&posed, 0.8);
    assert_eq!(arcs.len(), 1, "the equator rim survives the re-pose");
    let out = fillet_edges(&posed, &arcs, r, band(), tol())
        .unwrap_or_else(|e| panic!("the re-posed equator fillets, got {e:?}"));
    let (center, axis, major, minor) = band_torus(&out.body, out.band_faces[0]);
    let want = ((1.0 - r).powi(2) - 0.36).sqrt();
    assert!(
        (major - want).abs() < 1e-9,
        "pose-invariant spine radius {want}, got {major}"
    );
    assert!((minor - r).abs() < 1e-9, "tube {r}, got {minor}");
    // The spine's centre and axis are the POSED frame's.
    let want_c = Point3::new(0.3, -0.2, 0.5);
    assert!(
        (center - want_c).norm() < 1e-9,
        "the spine centre rides the pose, got {center:?}"
    );
    let posed_axis = Vec3::new(0.0, 0.7f64.cos(), 0.7f64.sin());
    assert!(
        axis.dot(posed_axis).abs() > 1.0 - 1e-9,
        "the spine axis rides the pose, got {axis:?}"
    );
}

// ------------------------------------------------------------------
// 3. Opposite senses: a bitten ball, and the poses that refuse.
// ------------------------------------------------------------------

/// A unit ball with a spherical bite: material inside A (centre origin,
/// R = 1), outside B (centre (0, 0.15), R = 0.9). Bored at 0.7 so the
/// profile stays annular; the crater rim is the sphere×sphere edge with
/// OPPOSITE stored senses.
fn bitten_ball() -> Body<f64> {
    let ca = Point2::new(0.0, 0.0);
    let cb = Point2::new(0.0, 0.15);
    // The rim: y = (d² + 1 − 0.81) / (2d) off A's centre, d = 0.15.
    let ry: f64 = 0.2125 / 0.3;
    let rim = Point2::new((1.0 - ry * ry).sqrt(), ry);
    let a_lo = Point2::new(0.7, -(1.0f64 - 0.49).sqrt());
    let b_lo = Point2::new(0.7, 0.15 - (0.81f64 - 0.49).sqrt());
    revolved_about_y(
        vec![
            v(a_lo.x, a_lo.y, bulge(ca, 1.0, a_lo, rim)),
            v(rim.x, rim.y, bulge(cb, 0.9, rim, b_lo)),
            v(b_lo.x, b_lo.y, 0.0),
        ],
        Revolution::Full,
        tol(),
    )
}

/// **The crater rim folds one sense each way**: the ball rests at
/// R₁ − r from A's centre and R₂ + r from B's — with r = 0.05 the two
/// offsets are EQUAL (0.95), so the spine sits exactly midway between
/// the centres. Derived here, independent of the arm.
#[test]
fn a_bitten_ball_crater_rim_folds_opposite_senses() {
    let r = 0.05;
    let source = bitten_ball();
    let ry: f64 = 0.2125 / 0.3;
    let rim_r = (1.0 - ry * ry).sqrt();
    let arcs = rims_of_radius(&source, rim_r);
    assert_eq!(arcs.len(), 1, "one closed crater rim");
    let out = fillet_edges(&source, &arcs, r, band(), tol())
        .unwrap_or_else(|e| panic!("the crater rim fillets, got {e:?}"));
    let (center, _, major, minor) = band_torus(&out.body, out.band_faces[0]);
    let y_star = 0.15 / 2.0;
    let x_star = (0.95f64 * 0.95 - y_star * y_star).sqrt();
    assert!(
        (major - x_star).abs() < 1e-12,
        "spine radius {x_star}, got {major}"
    );
    assert!((minor - r).abs() < 1e-12, "tube {r}, got {minor}");
    assert!(
        (center.y - y_star).abs() < 1e-12,
        "the spine sits midway at {y_star}, got {}",
        center.y
    );
}

/// **When the offset circles stop crossing, the refusal is typed.** At
/// r = 0.13 the shrunken A offset (0.87) fits INSIDE the grown B offset
/// (1.03) — centres 0.15 apart, 1.03 − 0.87 > 0.15 — so no ball rests
/// anywhere: the closed form goes NaN and the battery must escalate,
/// not build, not panic, not misdescribe a corner.
#[test]
fn a_contained_offset_pose_refuses_typed() {
    let source = bitten_ball();
    let ry: f64 = 0.2125 / 0.3;
    let arcs = rims_of_radius(&source, (1.0 - ry * ry).sqrt());
    match fillet_edges(&source, &arcs, 0.13, band(), tol()).map_err(|r| r.error) {
        Ok(_) => panic!("no ball rests on both supports at r = 0.13"),
        Err(BlendError::UnsupportedCorner { corner, .. }) => {
            panic!("a closed rim registers no corner, got {corner}")
        }
        Err(_typed) => {}
    }
}

/// The same honesty on the CONVEX pair: the symmetric lentil's offsets
/// (1 − r) stop reaching each other past r = 0.4 (they would have to
/// span the centre half-distance 0.6). r = 0.45 must refuse typed.
#[test]
fn an_oversized_ball_on_the_lentil_refuses_typed() {
    let source = symmetric_lentil();
    let arcs = rims_of_radius(&source, 0.8);
    match fillet_edges(&source, &arcs, 0.45, band(), tol()).map_err(|r| r.error) {
        Ok(_) => panic!("no ball of r = 0.45 rests on both lentil walls"),
        Err(BlendError::UnsupportedCorner { corner, .. }) => {
            panic!("a closed rim registers no corner, got {corner}")
        }
        Err(_typed) => {}
    }
}

/// **The closed form's own degeneracies, at the unit level**: at the
/// exact tangency radius the crossing lands ON the axis (spine radius
/// zero — downstream poison via 1/s); past it the half-chord square
/// goes negative and the centre is NaN, never a fabricated point.
#[test]
fn sheet_center_degrades_to_axis_then_nan_past_tangency() {
    use sweep::blend::arms::{Meridian, SupportTrace, sheet_center};
    let sheet = Meridian {
        origin: Point3::new(0.0, 0.0, 0.0),
        axis: Vec3::new(0.0, 1.0, 0.0),
        rim: Point3::new(0.8, 0.0, 0.0),
    };
    let trace = |y: f64, side: f64| SupportTrace::Round {
        center: Point3::new(0.0, y, 0.0),
        radius: 1.0,
        side,
    };
    // r = 0.4: offsets 0.6 exactly reach the far centre — the crossing
    // is the axis point midway.
    let at_tangency = sheet_center(
        sheet.rim,
        sheet.sheet_normal(),
        trace(-0.6, 1.0),
        trace(0.6, 1.0),
        0.4,
    );
    // The half-chord square is an EXACT zero only in real arithmetic;
    // in floats it is a ± ulp residue, so the honest outcomes are "on
    // the axis to √ulp" or NaN (poison) — both feed predicate 3 a
    // degenerate spine. A finite centre OFF the axis is the failure.
    assert!(
        (at_tangency.x.abs() < 1e-6 && at_tangency.z.abs() < 1e-6) || at_tangency.x.is_nan(),
        "at tangency the centre degenerates (axis or poison), got {at_tangency:?}"
    );
    // r = 0.45: no crossing exists; the form must say NaN.
    let past = sheet_center(
        sheet.rim,
        sheet.sheet_normal(),
        trace(-0.6, 1.0),
        trace(0.6, 1.0),
        0.45,
    );
    assert!(
        past.x.is_nan() || past.y.is_nan(),
        "past tangency the centre is poison, got {past:?}"
    );
    // The internal pose (opposite senses): at r = 0.125 the offsets
    // 0.875 / 1.025 are internally tangent (difference = the centre
    // distance 0.15) — again the crossing degenerates onto the axis.
    let internal = sheet_center(
        sheet.rim,
        sheet.sheet_normal(),
        SupportTrace::Round {
            center: Point3::new(0.0, 0.0, 0.0),
            radius: 1.0,
            side: 1.0,
        },
        SupportTrace::Round {
            center: Point3::new(0.0, 0.15, 0.0),
            radius: 0.9,
            side: -1.0,
        },
        0.125,
    );
    assert!(
        (internal.x.abs() < 1e-6 && internal.z.abs() < 1e-6) || internal.x.is_nan(),
        "internal tangency degenerates (axis or poison), got {internal:?}"
    );
}

// ------------------------------------------------------------------
// 4. A genuine valence-4 vertex keeps NEdgeVertex.
// ------------------------------------------------------------------

/// **A fully chamfered cube** — twelve edges at equal setback. Each
/// corner-patch triangle vertex is a GENUINE valence-4 vertex: patch,
/// two strips, and a shrunk box face, all four distinct planes, no
/// co-surface edge anywhere. (An earlier draft of this probe tried the
/// intersection of two crossing wedge prisms — the boolean refuses the
/// ridge-through-ridge contact typed, `ZipCorrespondence`, so the
/// chamfer is the machinery that actually mints valence-4 today.)
fn chamfered_cube() -> Body<f64> {
    let body = sweep::test_support::cube(1.0, tol());
    let edges: Vec<EdgeKey> = body.edges().map(|(k, _)| k).collect();
    sweep::chamfer::chamfer_edges(&body, &edges, 0.1, band(), tol())
        .expect("a cube's twelve edges chamfer")
        .body
}

/// **The seam recognition must not swallow a real N-edge vertex.** A
/// fillet chain stopping at a chamfered cube's patch vertex — valence
/// four, four distinct plane supports, zero co-surface edges — keeps
/// the `NEdgeVertex` refusal and its stop-at-vertex policy exactly as
/// before this unit.
#[test]
fn a_chamfer_patch_vertex_keeps_its_n_edge_vertex_refusal() {
    let body = chamfered_cube();
    // Find a valence-4 vertex and check its structure is the genuine
    // kind: four edges, four DISTINCT support pairs, none co-surface.
    let mut found = None;
    for (vk, vt) in body.vertices() {
        let Some(he) = vt.emanating else { continue };
        let Some(orbit) = body.vertex_orbit(he) else {
            continue;
        };
        let mut edges: Vec<EdgeKey> = orbit
            .iter()
            .map(|h| body.get_half_edge(*h).unwrap().edge)
            .collect();
        edges.sort_unstable();
        edges.dedup();
        if edges.len() == 4 {
            found = Some((vk, edges));
            break;
        }
    }
    let (_, edges) = found.expect("a chamfered cube has valence-4 patch vertices");
    for e in &edges {
        let (a, b) = supports(&body, *e);
        assert_ne!(a, b, "no edge at a patch vertex is co-surface");
    }
    // A chain of one incident edge terminates there; the refusal is
    // the N-edge one, not the seam one.
    match fillet_edges(&body, &edges[..1], 0.02, band(), tol()).map_err(|r| r.error) {
        Err(BlendError::UnsupportedCorner {
            corner: CornerConfig::NEdgeVertex { valence: 4 },
            policy,
            ..
        }) => assert_eq!(policy, Some(RunOutPolicy::RunOutStopAtVertex)),
        other => panic!("a genuine 4-edge vertex keeps NEdgeVertex, got {other:?}"),
    }
}

// ------------------------------------------------------------------
// 5. The differential: a pair outside the family still refuses.
// ------------------------------------------------------------------

/// A barrel whose wall is a TORUS (an off-axis profile arc revolved),
/// capped by plane annuli. Its rims are torus×plane — no arm.
fn torus_barrel() -> Body<f64> {
    let c = Point2::new(0.9 - 0.0325f64.sqrt(), 0.3);
    let lo = Point2::new(0.9, 0.0);
    let hi = Point2::new(0.9, 0.6);
    revolved_about_y(
        vec![
            v(0.6, 0.0, 0.0),
            v(lo.x, lo.y, bulge(c, 0.35, lo, hi)),
            v(hi.x, hi.y, 0.0),
            v(0.6, 0.6, 0.0),
        ],
        Revolution::Full,
        tol(),
    )
}

/// **A torus support refuses through its own gate**, and the roster it
/// names now advertises sphere–sphere: growing the table must not have
/// widened it past what the table holds.
#[test]
fn a_torus_walled_rim_refuses_spine_unsupported_naming_the_grown_roster() {
    let source = torus_barrel();
    // The fixture really has a torus wall.
    assert!(
        source
            .faces()
            .any(|(_, f)| matches!(source.get_surface(f.surface), Some(&Surface::Torus { .. }))),
        "the barrel wall is a torus"
    );
    let arcs = rims_of_radius(&source, 0.9);
    assert_eq!(arcs.len(), 2, "two torus-plane rims");
    match fillet_edges(&source, &arcs[..1], 0.03, band(), tol()).map_err(|r| r.error) {
        Err(BlendError::SpineUnsupported { supports, .. }) => {
            assert!(
                supports.contains("sphere–sphere"),
                "the roster advertises the ninth arm: {supports}"
            );
        }
        other => panic!("a torus pair refuses SpineUnsupported, got {other:?}"),
    }
}

// ------------------------------------------------------------------
// 6. The seam story on a second pole-touching body.
// ------------------------------------------------------------------

/// A spinning top: a cone from the bottom pole to (0.6, 0.45), closed
/// by a sphere cap through the top pole (centre (0, 0.275), R = 0.625).
/// Pole-touching, so the full revolve seam-splits every wall.
fn spinning_top() -> Body<f64> {
    let cs = Point2::new(0.0, 0.275);
    let p1 = Point2::new(0.6, 0.45);
    let p2 = Point2::new(0.0, 0.9);
    revolved_about_y(
        vec![
            v(0.0, 0.0, 0.0),
            v(p1.x, p1.y, bulge(cs, 0.625, p1, p2)),
            v(p2.x, p2.y, 0.0),
        ],
        Revolution::Full,
        tol(),
    )
}

/// **The seam-vertex story, reproduced on a body the unit never
/// built.** One arc of the cone×sphere rim refuses `SeamVertex` with no
/// policy; the rim requested WHOLE gets past every corner door and
/// CARVES — the recourse's promise, taken literally on a second
/// pole-touching body.
#[test]
fn a_spinning_top_seam_vertex_refuses_and_the_whole_rim_carves() {
    let body = spinning_top();
    let arcs = rims_of_radius(&body, 0.6);
    assert_eq!(arcs.len(), 2, "the seam splits the rim into two arcs");
    match fillet_edges(&body, &arcs[..1], 0.03, band(), tol()).map_err(|r| r.error) {
        Err(
            e @ BlendError::UnsupportedCorner {
                corner: CornerConfig::SeamVertex,
                policy: None,
                ..
            },
        ) => {
            let text = e.to_string();
            assert!(
                text.contains("request the rim whole"),
                "the recourse names the request: {text}"
            );
        }
        other => panic!("one arc refuses SeamVertex, got {other:?}"),
    }
    // The recourse's request, taken literally: past the seam and
    // through the closed-rim door, as one annulus over both arcs.
    let out = fillet_edges(&body, &arcs, 0.03, band(), tol())
        .unwrap_or_else(|e| panic!("the whole rim carves, got {e:?}"));
    validate_geometric(&out.body, tol())
        .unwrap_or_else(|e| panic!("the carved top must be tier-3 valid, got {e:?}"));
    assert_eq!(out.band_faces.len(), 1, "one annulus band over both arcs");
}
