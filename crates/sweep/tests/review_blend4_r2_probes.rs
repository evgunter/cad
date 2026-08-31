//! **BLEND-4 review probes (R2).**
//!
//! Six rows, each pinning something the unit's own suites leave
//! unpinned or state more strongly than the code holds. They are
//! written to be adoptable as they stand.
//!
//! The subject is the convexity-parametric fillet corner (issue 644):
//! `corner_ball`'s side, the feet's sign, `octant_chart`'s fold, and
//! the corner arcs. Everything here goes through the public carve
//! (`fillet_edges`) or through the one public arm (`corner_ball`) —
//! no private plan is reached, so the rows survive refactors of the
//! surgery's internals.

#![allow(clippy::unwrap_used, clippy::expect_used, clippy::panic)]

use geom::{Curve3, Surface};
use geom_core::{Affine3, Band, Mat3, Point2, Point3, Tol, Vec3};
use profile::{Profile, ProfileLoop, ProfileVertex, RawLoop, SketchPlane};
use sweep::blend::arms::corner_ball;
use sweep::blend::build::fillet_edges;
use sweep::blend::{BlendError, CornerConfig};
use sweep::test_support::cube;
use sweep::{Extrusion, extrude};
use topo::{Body, EdgeKey, FaceKey, LoopBoundary, subtract};

/// The fillet radius the concave-fillet suite carves at.
const R: f64 = 0.25;

fn p(x: f64, y: f64, z: f64) -> Point3<f64> {
    Point3::new(x, y, z)
}
fn v(x: f64, y: f64, z: f64) -> Vec3<f64> {
    Vec3::new(x, y, z)
}

fn band() -> Band {
    let tol = Tol::witness().get();
    Band::new(tol.eps, tol.k * tol.eps).unwrap()
}

// ------------------------------------------------------------------
// Fixtures — the concave-fillet suite's vented cavity, restated (the
// suite-tree fixture-copy class that suite already declares).
// ------------------------------------------------------------------

fn brick(lo: Point3<f64>, hi: Point3<f64>) -> Body<f64> {
    let lp = ProfileLoop::polygon([
        Point2::new(lo.x, lo.y),
        Point2::new(hi.x, lo.y),
        Point2::new(hi.x, hi.y),
        Point2::new(lo.x, hi.y),
    ]);
    let plane = SketchPlane::new(Affine3::from_parts(
        Mat3::from_cols(Vec3::unit_x(), Vec3::unit_y(), Vec3::unit_z()),
        Point3::new(0.0, 0.0, lo.z) - Point3::origin(),
    ));
    let profile = Profile::new(plane, vec![lp])
        .validate(Tol::witness())
        .expect("a rectangle is a valid profile");
    extrude(&profile, Extrusion::Distance(hi.z - lo.z), Tol::witness())
        .expect("a brick extrudes")
        .body
}

fn rod(center: Point2<f64>, r: f64, z0: f64, z1: f64) -> Body<f64> {
    let lp = ProfileLoop::new(vec![
        ProfileVertex::new(Point2::new(center.x - r, center.y), 1.0),
        ProfileVertex::new(Point2::new(center.x + r, center.y), 1.0),
    ]);
    let plane = SketchPlane::new(Affine3::from_parts(
        Mat3::from_cols(Vec3::unit_x(), Vec3::unit_y(), Vec3::unit_z()),
        Point3::new(0.0, 0.0, z0) - Point3::origin(),
    ));
    let profile = Profile::new(plane, vec![lp])
        .validate(Tol::witness())
        .expect("a circle is a valid profile");
    extrude(&profile, Extrusion::Distance(z1 - z0), Tol::witness())
        .expect("a rod extrudes")
        .body
}

fn vented_cavity() -> Body<f64> {
    let block = brick(p(0.0, 0.0, 0.0), p(4.0, 4.0, 4.0));
    let vent = rod(Point2::new(2.0, 2.0), 0.5, 2.5, 5.0);
    let cavity = brick(p(1.0, 1.0, 1.0), p(3.0, 3.0, 3.0));
    let vented = subtract(&block, &vent, Tol::witness())
        .expect("the vent cut succeeds")
        .body()
        .expect("the vent cut leaves material")
        .body
        .clone();
    subtract(&vented, &cavity, Tol::witness())
        .expect("the cavity cut succeeds")
        .body()
        .expect("the cavity cut leaves material")
        .body
        .clone()
}

fn cavity_edges(body: &Body<f64>) -> Vec<EdgeKey> {
    let corner = |q: Point3<f64>| {
        [q.x, q.y, q.z]
            .iter()
            .all(|c| (c - 1.0).abs() < 1e-12 || (c - 3.0).abs() < 1e-12)
    };
    let mut found: Vec<EdgeKey> = body
        .edges()
        .filter(|(k, _)| {
            let Some(e) = body.get_edge(*k) else {
                return false;
            };
            let Some(h) = body.get_half_edge(e.he_plus) else {
                return false;
            };
            let Some(end) = body.half_edge_end(e.he_plus) else {
                return false;
            };
            let pt = |vk| {
                body.get_vertex(vk)
                    .and_then(|x| body.get_point(x.point))
                    .copied()
            };
            match (pt(h.start), pt(end)) {
                (Some(a), Some(b)) => corner(a) && corner(b),
                _ => false,
            }
        })
        .map(|(k, _)| k)
        .collect();
    found.sort_unstable();
    found
}

/// The all-convex carve: a cube's twelve edges.
fn convex_carve() -> (Body<f64>, Vec<FaceKey>) {
    let body = cube(2.0, Tol::witness());
    let edges: Vec<EdgeKey> = body.edges().map(|(k, _)| k).collect();
    let out = fillet_edges(&body, &edges, R, band(), Tol::witness())
        .expect("a cube's twelve convex edges fillet");
    (out.body, out.corner_faces)
}

/// The all-concave carve: the vented cavity's twelve edges.
fn concave_carve() -> (Body<f64>, Vec<FaceKey>) {
    let body = vented_cavity();
    let out = fillet_edges(&body, &cavity_edges(&body), R, band(), Tol::witness())
        .expect("the cavity's twelve concave edges fillet");
    (out.body, out.corner_faces)
}

/// One octant patch's stored chart and the unit directions of its
/// three boundary vertices from the sphere centre — the patch's FEET,
/// which for a sphere octant are exactly its corner points.
fn chart_and_feet(body: &Body<f64>, face: FaceKey) -> (Vec3<f64>, Vec3<f64>, Vec<Vec3<f64>>) {
    let f = body.get_face(face).expect("a minted corner face");
    let Surface::Sphere {
        center,
        axis,
        u_ref,
        ..
    } = body.get_surface(f.surface).expect("its surface")
    else {
        panic!("a fillet corner patch is a sphere octant");
    };
    let LoopBoundary::Cycle { first } = body.get_loop(f.outer).expect("an outer loop").boundary
    else {
        panic!("a minted face's boundary is a cycle");
    };
    let feet = body
        .loop_cycle(first)
        .expect("the cycle walks")
        .into_iter()
        .map(|he| {
            let vk = body.get_half_edge(he).expect("a cycle half-edge").start;
            let q = *body
                .get_vertex(vk)
                .and_then(|x| body.get_point(x.point))
                .expect("a boundary vertex's point");
            (q - *center).normalize()
        })
        .collect();
    (*axis, *u_ref, feet)
}

// ------------------------------------------------------------------
// The chart fold.
// ------------------------------------------------------------------

/// **The octant chart's SEAM and its quarter-turn are both feet — on
/// either material side.**
///
/// This is the pin the chart fold does not otherwise have. The unit's
/// own coverage of `octant_chart`'s concave arm is a single
/// plan-level assertion that the two sides' chart POLES are
/// antipodal, which is a statement about `axis` alone: it is silent
/// on `u_ref`, and the concave arm's `u_ref = −n_b` — the half the
/// mirror-substitution derivation is actually about — can be reverted
/// to the convex `n_a` with the whole battery still green.
///
/// What is pinned here is the property that makes the patch an
/// iso-parameter rectangle of its chart at all, read off the CARVED
/// body rather than off the plan: the seam meridian passes exactly
/// through one foot, and `axis × u_ref` — the quarter-turn along the
/// equator — passes exactly through a second. Both hold on both
/// sides, and either half of the concave fold reverted breaks the
/// first.
#[test]
fn r2_the_octant_charts_seam_and_quarter_turn_are_feet_on_both_sides() {
    for (tag, (body, corners)) in [("convex", convex_carve()), ("concave", concave_carve())] {
        assert_eq!(corners.len(), 8, "{tag}: eight octants");
        for face in corners {
            let (axis, u_ref, feet) = chart_and_feet(&body, face);
            assert_eq!(feet.len(), 3, "{tag}: an octant has three feet");
            let quarter = axis.cross(u_ref);
            let hits = |d: Vec3<f64>| feet.iter().any(|f| (*f - d).norm() < 1e-12);
            assert!(
                hits(u_ref),
                "{tag}: the seam meridian must pass through a foot — u_ref {u_ref:?} \
                 is not among the feet {feet:?}"
            );
            assert!(
                hits(quarter),
                "{tag}: the quarter-turn along the equator must pass through a foot — \
                 axis x u_ref {quarter:?} is not among the feet {feet:?}"
            );
            // The chart is orthonormal wherever it is aimed.
            assert!(
                axis.dot(u_ref).abs() < 1e-12,
                "{tag}: the seam is perpendicular to the pole"
            );
        }
    }
}

/// **The chart POLE is parallel to the third foot, but its SIGN is
/// not invariant — on either side.**
///
/// `octant_chart` scores its candidates by `|n_c × axis|`, which is
/// even in `axis`, so the pick constrains the pole only up to sign;
/// which sign a given corner gets falls out of the stored face order
/// of whichever link won. Measured on the two shipped fixtures: the
/// cube's eight convex octants split 7 pole-at-the-foot to 1
/// pole-at-its-antipode, and the cavity's eight concave octants split
/// 6 to 2.
///
/// The row pins the property that DOES hold (parallel up to sign) and
/// records the split, so that a later unit that wants the stronger
/// invariant — "the pole is the third foot", which `octant_chart`'s
/// prose currently asserts of both charts — has the counterexamples
/// in an executing row rather than in a review.
#[test]
fn r2_the_octant_chart_pole_is_the_third_foot_only_up_to_sign() {
    for (tag, (body, corners)) in [("convex", convex_carve()), ("concave", concave_carve())] {
        let mut at_foot = 0usize;
        let mut at_antipode = 0usize;
        for face in corners {
            let (axis, u_ref, feet) = chart_and_feet(&body, face);
            let quarter = axis.cross(u_ref);
            // The third foot: the one that is neither the seam nor the
            // quarter-turn.
            let third = *feet
                .iter()
                .find(|f| (**f - u_ref).norm() > 1e-9 && (**f - quarter).norm() > 1e-9)
                .expect("a third foot distinct from the two on the equator");
            assert!(
                (third - axis).norm() < 1e-12 || (third + axis).norm() < 1e-12,
                "{tag}: the pole is parallel to the third foot, got axis {axis:?} \
                 against foot {third:?}"
            );
            if (third - axis).norm() < 1e-12 {
                at_foot += 1;
            } else {
                at_antipode += 1;
            }
        }
        assert_eq!(at_foot + at_antipode, 8, "{tag}: every octant classified");
        assert!(
            at_antipode > 0,
            "{tag}: the measured split has at least one pole at the ANTIPODE of its \
             third foot — if this row goes green-with-zero, the sign became invariant \
             and the chart prose can be strengthened"
        );
    }
}

// ------------------------------------------------------------------
// The corner arcs.
// ------------------------------------------------------------------

/// **Every corner-patch boundary arc is a sub-π turn about the
/// patch's own stored centre, at the fillet radius — on either
/// side.**
///
/// The unit claims arc traversals needed no edit because
/// `attach_contact` derives each corner arc from its endpoints as the
/// short way round, which is side-blind. That claim is argued in the
/// PR and pinned nowhere: no shipped row reads a corner arc's stored
/// carrier. This row does, on both material sides — a wrong-way arc
/// on some concave pose is the failure mode the claim is exposed to,
/// and it would show up here as a turn at or beyond π, or as an axis
/// that does not agree with the traversal.
#[test]
fn r2_corner_arcs_are_sub_pi_turns_about_the_stored_centre_on_both_sides() {
    for (tag, (body, corners)) in [("convex", convex_carve()), ("concave", concave_carve())] {
        let mut seen = 0usize;
        for face in corners {
            let f = body.get_face(face).expect("a minted corner face");
            let Surface::Sphere { center, radius, .. } =
                body.get_surface(f.surface).expect("its surface")
            else {
                panic!("a fillet corner patch is a sphere octant");
            };
            let LoopBoundary::Cycle { first } =
                body.get_loop(f.outer).expect("an outer loop").boundary
            else {
                panic!("a minted face's boundary is a cycle");
            };
            for he in body.loop_cycle(first).expect("the cycle walks") {
                let h = body.get_half_edge(he).expect("a cycle half-edge");
                let e = body.get_edge(h.edge).expect("its edge");
                let c = body
                    .get_curve_geom(e.curve)
                    .and_then(|g| g.certified())
                    .expect("a corner arc is described and certified");
                let Curve3::Circle {
                    center: ac,
                    radius: ar,
                    axis: aa,
                    u_ref: au,
                } = *c.carrier()
                else {
                    // A corner patch's cycle also meets the cove
                    // bands' straight trimlines at some fixtures;
                    // only the arcs are this row's subject.
                    continue;
                };
                seen += 1;
                assert!(
                    (ac - *center).norm() < 1e-12,
                    "{tag}: a corner arc turns about the patch's own ball centre, \
                     got {ac:?} against {center:?}"
                );
                assert!(
                    (ar - *radius).abs() < 1e-12,
                    "{tag}: a corner arc's radius is the fillet radius, got {ar}"
                );
                let (t0, t1) = c.params();
                assert!(
                    t0 == 0.0 && t1 > 0.0 && t1 < core::f64::consts::PI,
                    "{tag}: a corner arc is the SHORT way round, got ({t0}, {t1})"
                );
                // The stored frame is orthonormal and right-handed
                // about the traversal, so the arc leaves its start
                // toward `axis x u_ref` rather than away from it.
                assert!(
                    aa.dot(au).abs() < 1e-12,
                    "{tag}: an arc's axis and seam are perpendicular"
                );
                assert!(
                    (aa.norm() - 1.0).abs() < 1e-12 && (au.norm() - 1.0).abs() < 1e-12,
                    "{tag}: an arc's stored frame is unit"
                );
            }
        }
        assert!(
            seen >= 8,
            "{tag}: corner arcs were actually read, got {seen}"
        );
    }
}

// ------------------------------------------------------------------
// The refusal payload, and the sibling-hazard claim.
// ------------------------------------------------------------------

/// **The mixed-corner refusal's COUNT is pinned, not elided.**
///
/// The row this unit retired (`the_concave_fillet_refuses_exactly_as
/// _it_did_before`) asserted its payload's `convex` count rather than
/// the variant, and said in its own prose why: the variant alone
/// stays green under a relabelling. Its replacement at the L-bracket
/// matches `MixedConvexity { .. }` and drops the count, so the
/// discipline the retired row articulated left the tree with it.
/// This restores it at the surviving fixture: the bracket's reflex
/// edge ends at corners where exactly two of three edges are convex.
#[test]
fn r2_the_mixed_corner_refusals_count_is_two_of_three() {
    let lp = ProfileLoop::polygon([
        Point2::new(0.0, 0.0),
        Point2::new(2.0, 0.0),
        Point2::new(2.0, 1.0),
        Point2::new(1.0, 1.0),
        Point2::new(1.0, 2.0),
        Point2::new(0.0, 2.0),
    ]);
    let profile = Profile::new(SketchPlane::xy(), vec![lp])
        .validate(Tol::witness())
        .expect("the L is a valid profile");
    let bracket = extrude(&profile, Extrusion::Distance(1.0), Tol::witness())
        .expect("the bracket extrudes")
        .body;
    let on_reflex = |q: Point3<f64>| (q.x - 1.0).abs() < 1e-12 && (q.y - 1.0).abs() < 1e-12;
    let reflex: Vec<EdgeKey> = bracket
        .edges()
        .filter(|(k, _)| {
            let Some(e) = bracket.get_edge(*k) else {
                return false;
            };
            let pt = |vk| {
                bracket
                    .get_vertex(vk)
                    .and_then(|x| bracket.get_point(x.point))
                    .copied()
            };
            let (Some(a), Some(b)) = (
                bracket
                    .get_half_edge(e.he_plus)
                    .map(|h| h.start)
                    .and_then(pt),
                bracket.half_edge_end(e.he_plus).and_then(pt),
            ) else {
                return false;
            };
            on_reflex(a) && on_reflex(b)
        })
        .map(|(k, _)| k)
        .collect();
    assert_eq!(reflex.len(), 1, "the bracket's one reflex vertical edge");
    let refused = fillet_edges(&bracket, &reflex, 0.1, band(), Tol::witness())
        .expect_err("a mixed corner refuses");
    match refused.error {
        BlendError::UnsupportedCorner {
            corner: CornerConfig::MixedConvexity { convex },
            ..
        } => assert_eq!(
            convex, 2,
            "the reflex edge's ends have two convex edges of three"
        ),
        ref other => panic!("expected the corner-configuration refusal, got {other:?}"),
    }
}

/// **No sliver-wedge pose on the fillet corner path comes back
/// silently wrong: every one either carves a valid body or refuses
/// with a finite, typed diagnostic.**
///
/// Every division and normalization on the fillet corner path
/// degenerates only where `det(n₁, n₂, n₃) = 0` or the dihedral
/// decides Zero — `1 ± n_a·n_b`, `n_a × n_b`, `Σn` and the Cramer
/// `det` all vanish only there, and the exact singularities are what
/// `fillet3_corner_independence` and the dihedral sign refuse. But
/// the net that actually catches NEAR-singular poses is elsewhere,
/// and this row is where that is measured rather than argued: every
/// refusing pose of this grid refuses at the battery's clearance
/// screen (`FaceClearanceUncertified` — the sliver's setbacks explode
/// faster than its wedge closes), and the pose family that slips past
/// screening refuses MID-assembly at the description-attachment
/// certification gate (`Op { site: "surgery contact edge", .. }`
/// carrying a `TangentSecondOrder` escalation — the R1 probes' 0.2°
/// skew). The independence and dihedral gates are the guards of the
/// exact zero, not the working frontier.
///
/// A triangular prism with a closing apex drives the apex dihedral to
/// zero, which drives the plane–plane spine's `1 + d`, the chart's
/// `n_a × n_b` and the corner's `det` down together. Across a grid of
/// apex thicknesses and radii — spanning the carving, the screened
/// and the degenerate regimes — the verb must never hand back a body
/// it cannot stand behind: an `Ok` must validate at tiers 1 and 2, and
/// an `Err` must render a finite sentence naming a door. Neither a
/// panic nor a `NaN` in a margin is an acceptable answer, and this is
/// the shape a missed sibling-hazard guard would take.
#[test]
fn r2_no_sliver_wedge_pose_is_silently_wrong_on_the_corner_path() {
    let mut carved = 0usize;
    let mut refused = 0usize;
    for thickness in [1.0_f64, 0.25, 0.05, 0.004, 0.0002] {
        for radius in [0.2_f64, 0.02, 0.0005] {
            let lp = ProfileLoop::polygon([
                Point2::new(0.0, 0.0),
                Point2::new(2.0, 0.0),
                Point2::new(2.0, thickness),
            ]);
            let profile = Profile::new(SketchPlane::xy(), vec![lp])
                .validate(Tol::witness())
                .expect("a triangle is a valid profile");
            let prism = extrude(&profile, Extrusion::Distance(1.0), Tol::witness())
                .expect("the wedge prism extrudes")
                .body;
            let edges: Vec<EdgeKey> = prism.edges().map(|(k, _)| k).collect();
            let pose = format!("thickness {thickness}, radius {radius}");
            match fillet_edges(&prism, &edges, radius, band(), Tol::witness()) {
                Ok(out) => {
                    carved += 1;
                    assert_eq!(topo::validate(&out.body), Ok(()), "{pose}: tier 1");
                    assert_eq!(topo::validate_closed(&out.body), Ok(()), "{pose}: tier 2");
                    let props = topo::mass_properties(&out.body, Tol::witness())
                        .expect("a carved wedge has closed-form props");
                    assert!(
                        props.volume.is_finite() && props.volume > 0.0,
                        "{pose}: a carved body has a finite positive volume, got {}",
                        props.volume
                    );
                }
                Err(err) => {
                    refused += 1;
                    let text = err.error.to_string();
                    assert!(
                        !text.is_empty(),
                        "{pose}: a refusal names its door in words"
                    );
                    assert!(
                        !text.contains("NaN") && !text.contains("inf"),
                        "{pose}: a refusal's diagnostic is finite, got {text}"
                    );
                }
            }
        }
    }
    // The grid must actually straddle the frontier — a grid that only
    // carves, or only refuses, would pass this row while testing
    // nothing about the singularity it is aimed at.
    assert!(
        carved > 0 && refused > 0,
        "the pose grid straddles the frontier: {carved} carved, {refused} refused"
    );
}

// ------------------------------------------------------------------
// The concave arm, off the orthonormal special case.
// ------------------------------------------------------------------

/// **The concave FEET land on their walls at an oblique trihedron.**
///
/// The unit measures the concave ball's REST off the orthonormal case
/// (`the_concave_rest_holds_at_an_oblique_trihedron`, `det = 0.8`)
/// and measures the FEET only at the orthonormal mirror corner, where
/// the three walls are mutually perpendicular and the two candidate
/// sign conventions are farthest apart for the easiest possible
/// reason. This row carries the feet measurement onto the oblique
/// trihedron: `centre − n·r` is on every wall and `centre + n·r` is
/// `2r` off it, at walls that are neither orthogonal nor through one
/// common point.
#[test]
fn r2_the_concave_feet_land_on_their_walls_at_an_oblique_trihedron() {
    let r = 0.2;
    let normals = [v(1.0, 0.0, 0.0), v(0.0, 1.0, 0.0), v(0.6, 0.0, 0.8)];
    // Three DISTINCT wall points, so the solve is not handed a common
    // origin that would hide a per-wall offset error.
    let verts = [p(0.0, 0.5, 0.25), p(-0.3, 0.0, 0.75), p(0.0, 1.5, 0.0)];
    let ball = corner_ball(verts, normals, r, false);
    for (i, n) in normals.iter().enumerate() {
        let rest = (ball.center - verts[i]).dot(*n);
        assert!(
            (rest - r).abs() < 1e-14,
            "wall {i}: the concave rest is +r in the void, got {rest}"
        );
        let foot = ball.center - *n * r;
        let on = (foot - verts[i]).dot(*n);
        assert!(
            on.abs() < 1e-14,
            "wall {i}: `centre - n*r` is the tangency point, got {on}"
        );
        let convex_formula = ball.center + *n * r;
        let off = (convex_formula - verts[i]).dot(*n);
        assert!(
            (off - 2.0 * r).abs() < 1e-14,
            "wall {i}: the convex-signed foot floats 2r into the void, got {off}"
        );
    }
    assert!(
        (ball.independence - 0.8).abs() < 1e-14,
        "independence is the walls' |det|, side-blind"
    );
}
