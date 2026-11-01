//! **Sphere sections tilted against the chart**: a sphere pair whose
//! centre line is off a ball's polar axis, and a plane that is not
//! normal to it, under every boolean.
//!
//! Each ball is the canonical full revolve of a semicircle about `y`
//! (two half-bands on one sphere key), moved off the origin. The pair's
//! section is the radical-plane circle, handed to both sides' wall
//! lanes; offset along `x` (or anywhere in
//! the seam plane `z = c_z`) that circle is tilted against both charts'
//! polar axis, so its azimuth doubles back; the join takes each chord's
//! arc from the germs it pairs, which reads no chart. The faces it leaves are
//! bounded by circles that are neither rims nor meridians, and the
//! sphere flux arm measures them by Gauss–Bonnet
//! (`props::curved::sphere_face`). Every body is held to all
//! three validation tiers and to its volume against the lens the two
//! spheres share, computed here from the radii and the centre distance
//! alone.
//!
//! A pierce ring off the seam plane and a plane tilted against the ball's
//! chart build too; the rows a tilted section does not yet reach are
//! pinned at the door they stop at.

#![allow(clippy::unwrap_used, clippy::expect_used, clippy::panic)]

use geom_core::{Point2, Point3, Tol, Vec3};
use sweep::Revolution;
use sweep::test_support::{finished, revolved_about_y};
use topo::{AtRestBody, Body, BooleanOp};

use crate::common::oracles::{ball_volume, cap_volume, lens_volume};

/// A ball of radius `r` centred at `c`, poles on world `y`.
fn ball(r: f64, c: Vec3<f64>) -> AtRestBody<f64> {
    let b = revolved_about_y(
        vec![(Point2::new(0.0, -r), 1.0), (Point2::new(0.0, r), 0.0)],
        Revolution::Full,
        Tol::witness(),
    );
    let b = topo::transform_rigid(
        &b,
        &geom_core::RigidMotion::translation(c).unwrap(),
        Tol::witness(),
    )
    .unwrap();
    finished("the ball", b, Tol::witness())
}

/// [`assert_solid`], then a mesh at δ = 1e-2 that `check_mesh` passes:
/// the sphere faces a tilted circle bounds mesh through the trimmed
/// lane's sphere arm.
///
/// Meshed at δ 1e-2, and at 2e-2 and 4e-2 too: PR 5920's review found
/// triangles folded over the chord beside a tilted circle at the coarser
/// two (`sphere_pairs_tilted_against_both_charts…` at 2e-2,
/// `a_pierce_off_the_seam_plane…` at 4e-2), which the mesher now drops
/// and rebuilds, and every triangle is read for its winding.
fn assert_body(label: &str, body: &Body<f64>, expected: f64) {
    assert_solid(label, body, expected);
    for delta in [1e-2, 2e-2, 4e-2] {
        let mesh = mesh::tessellate(body, delta, Tol::witness())
            .unwrap_or_else(|e| panic!("{label}, δ {delta}: tessellate, got {e:?}"));
        mesh::validate::check_mesh(&mesh)
            .unwrap_or_else(|e| panic!("{label}, δ {delta}: the mesh: {e:?}"));
        crate::common::differential::assert_meshed_geometry(
            &format!("{label}, δ {delta}"),
            body,
            &mesh,
            delta,
        );
    }
}

/// Every tier of validation, then the volume against `expected` through
/// the kernel's mass properties — closed-form on every face here, so
/// the slack is rounding's and a wrong arc (the complement of a cap
/// selected) misses by the cap's own volume.
fn assert_solid(label: &str, body: &Body<f64>, expected: f64) {
    assert_eq!(topo::validate(body), Ok(()), "{label}: validate");
    assert_eq!(
        topo::validate_closed(body),
        Ok(()),
        "{label}: validate_closed"
    );
    assert_eq!(
        topo::validate_geometric(body, Tol::witness()),
        Ok(()),
        "{label}: validate_geometric"
    );
    let p = topo::mass_properties(
        &topo::test_support::finished("the measured body", topo::Body::clone(body), Tol::witness()),
        Tol::witness(),
        topo::test_support::pinned_resolution(Tol::witness()),
    )
    .unwrap_or_else(|e| panic!("{label}: mass properties, got {e:?}"));
    assert_eq!(
        p.enclosure.volume_pad, 0.0,
        "{label}: closed-form faces only"
    );
    assert!(
        (p.enclosure.volume - expected).abs() <= 1e-9 * expected.max(1.0),
        "{label}: volume {} against the lens closed form {expected}",
        p.enclosure.volume
    );
}

fn run(
    op: BooleanOp,
    a: &AtRestBody<f64>,
    b: &AtRestBody<f64>,
) -> Result<topo::BooleanResult<f64>, topo::BooleanError> {
    match op {
        BooleanOp::Union => topo::boolean::union(a, b, Tol::witness()),
        BooleanOp::Intersect => topo::boolean::intersect(a, b, Tol::witness()),
        BooleanOp::Subtract => topo::boolean::subtract(a, b, Tol::witness()),
    }
}

/// `a ∪ b`, `a ∩ b`, `a ∖ b` and `b ∖ a`, each a body against its
/// closed form from the operands' volumes and the shared volume.
fn assert_every_op(
    pose: &str,
    a: &AtRestBody<f64>,
    b: &AtRestBody<f64>,
    va: f64,
    vb: f64,
    shared: f64,
) {
    for (label, op, x, y, expected) in [
        ("A ∪ B", BooleanOp::Union, a, b, va + vb - shared),
        ("A ∩ B", BooleanOp::Intersect, a, b, shared),
        ("A ∖ B", BooleanOp::Subtract, a, b, va - shared),
        ("B ∖ A", BooleanOp::Subtract, b, a, vb - shared),
    ] {
        let label = format!("{pose}, {label}");
        let out = run(op, x, y).unwrap_or_else(|e| panic!("{label} refused: {e:?}"));
        let body = &out
            .body()
            .unwrap_or_else(|| panic!("{label} came back empty"))
            .body;
        assert_body(&label, body, expected);
    }
}

/// The unit ball every pose offsets from.
const BASE: Vec3<f64> = Vec3::new(2.0, 2.0, 0.5);

/// **Sphere pairs offset in the seam plane** build under every op: the
/// radical plane is tilted against both charts, from a pair of equal
/// balls whose section is a great-circle-sized cut to a small ball
/// whose section lies wholly inside one half-band. Each pose is the
/// `y`-poled unit ball at [`BASE`] against a ball of radius `r` at
/// `BASE + offset`.
#[test]
fn sphere_pairs_tilted_against_both_charts_build_under_every_boolean() {
    for (pose, r, offset) in [
        ("equal balls along x", 1.0, Vec3::new(1.4, 0.0, 0.0)),
        ("equal balls along x and y", 1.0, Vec3::new(1.2, 0.6, 0.0)),
        ("a smaller ball along x", 0.6, Vec3::new(0.9, 0.0, 0.0)),
        ("a small ball mostly inside", 0.3, Vec3::new(0.9, 0.0, 0.0)),
        (
            "a smaller ball along x and y",
            0.5,
            Vec3::new(0.8, 0.4, 0.0),
        ),
    ] {
        assert_every_op(
            pose,
            &ball(1.0, BASE),
            &ball(r, BASE + offset),
            ball_volume(1.0),
            ball_volume(r),
            lens_volume(1.0, r, offset.norm()),
        );
    }
}

/// **The equal pair at the `Interval` scalar**: enclosures throughout,
/// the chords' departure trileans and the Gauss–Bonnet turning angles on
/// enclosures, and every body certifies with a volume bracket around
/// the lens closed form, at every band the sweep runs.
#[test]
fn a_tilted_sphere_pair_builds_at_the_interval_scalar() {
    use crate::common::interval::iv;
    use geom_core::{Bounds, Interval};
    let ball_iv = |c: Vec3<f64>| -> AtRestBody<Interval> {
        let b = sweep::test_support::revolved_about_y_at::<Interval>(
            vec![
                (Point2::new(iv(0.0), iv(-1.0)), iv(1.0)),
                (Point2::new(iv(0.0), iv(1.0)), iv(0.0)),
            ],
            Revolution::Full,
            Tol::witness(),
        );
        let to = Vec3::new(iv(c.x), iv(c.y), iv(c.z));
        let b = topo::transform_rigid(
            &b,
            &geom_core::RigidMotion::translation(to).unwrap(),
            Tol::witness(),
        )
        .unwrap();
        finished("the ball", b, Tol::witness())
    };
    let (a, b) = (ball_iv(BASE), ball_iv(BASE + Vec3::new(1.4, 0.0, 0.0)));
    let lens = lens_volume(1.0, 1.0, 1.4);
    let v1 = ball_volume(1.0);
    for (op, expected) in [
        (BooleanOp::Union, 2.0 * v1 - lens),
        (BooleanOp::Intersect, lens),
        (BooleanOp::Subtract, v1 - lens),
    ] {
        let out = match op {
            BooleanOp::Union => topo::boolean::union(&a, &b, Tol::witness()),
            BooleanOp::Intersect => topo::boolean::intersect(&a, &b, Tol::witness()),
            BooleanOp::Subtract => topo::boolean::subtract(&a, &b, Tol::witness()),
        };
        let out = out.unwrap_or_else(|e| panic!("Interval {op:?} refused: {e:?}"));
        let body = &out
            .body()
            .unwrap_or_else(|| panic!("Interval {op:?} came back empty"))
            .body;
        assert_eq!(topo::validate(body), Ok(()), "Interval {op:?}: validate");
        assert_eq!(
            topo::validate_geometric(body, Tol::witness()),
            Ok(()),
            "Interval {op:?}: validate_geometric"
        );
        let v = topo::mass_properties(
            body,
            Tol::witness(),
            topo::test_support::pinned_resolution(Tol::witness()),
        )
        .expect("the measured body")
        .met()
        .expect("the measured body")
        .volume;
        let slack = 1e-9 * expected.max(1.0);
        assert!(
            v.lo() - slack <= expected && expected <= v.hi() + slack,
            "Interval {op:?}: volume [{}, {}] against the lens closed form {expected}",
            v.lo(),
            v.hi()
        );
    }
}

/// **A pierce ring builds.** An offset with a component off the seam
/// plane drives B's boundary through the interior of one of A's
/// half-bands, where the section lands as a ring of that face, joined
/// to its outer loop by `mekr`. Its chord co-bounds no run, and takes
/// the arc the pierce germs' directions name: an equal ball off the seam
/// plane, and a smaller one off every axis.
#[test]
fn a_pierce_off_the_seam_plane_builds_under_every_boolean() {
    for (pose, r, offset) in [
        ("off the seam plane", 1.0, Vec3::new(1.3, 0.0, 0.2)),
        (
            "a smaller ball off every axis",
            0.7,
            Vec3::new(0.9, 0.3, 0.6),
        ),
    ] {
        assert_every_op(
            pose,
            &ball(1.0, BASE),
            &ball(r, BASE + offset),
            ball_volume(1.0),
            ball_volume(r),
            lens_volume(1.0, r, offset.norm()),
        );
    }
}

/// **A plane tilted against the ball's chart builds.** A box face across
/// the `y`-poled unit ball at the origin, in the plane `x = 0.5`: the
/// boolean's planar side takes the same arc as the sphere side, from the
/// germs, so the box over `z ∈ (0, 2)`, its mirror over `z ∈ (−2, 0)`
/// and the box over both (whose face holds the whole section circle, a
/// pierce ring on the plane) each build under every op. The shared
/// volume is the cap beyond `x = 0.5`, `c = πh²(3 − h)/3` at `h = ½`,
/// halved where the box covers one side of `z = 0`.
#[test]
fn a_plane_tilted_against_the_balls_chart_builds_under_every_boolean() {
    let cap = cap_volume(1.0, 0.5);
    for (pose, z, shared) in [
        ("a box face across the ball", (0.0, 2.0), cap / 2.0),
        ("the box mirrored", (-2.0, 0.0), cap / 2.0),
        ("a box face holding the whole section", (-2.0, 2.0), cap),
    ] {
        let a = finished(
            "the box",
            sweep::test_support::brick((0.5, 3.0), (-2.0, 2.0), z, Tol::witness()),
            Tol::witness(),
        );
        assert_every_op(
            pose,
            &a,
            &ball(1.0, Vec3::new(0.0, 0.0, 0.0)),
            2.5 * 4.0 * (z.1 - z.0),
            ball_volume(1.0),
            shared,
        );
    }
}

/// **A pip whose poles lie in the cube's top face builds at its closed
/// form.** The `y`-poled ball(0.3) at `(0.5, 0.5, 1)` has its seam great
/// circle in the plane `z = 1`, so every section segment runs along a
/// seam edge of the ball, and both section loops' regions are the ball's
/// two hemispheres, every witness of which lies on the cube's top face
/// (`work/honesplit/the-uncut-shell-witness-reads-no-curved-face-interior.md`).
/// The join reads their sides at the seam instead, each hemisphere
/// leaving the top face to its own side at first order
/// (`boolean::join`'s `seam_germ_side`), so every op builds: `∪` the cube
/// and the upper half-ball, `∩` the lower half-ball, `∖` the cube less
/// it, each at its closed form and placing a lattice about the pip as
/// the analytic region does.
#[test]
fn a_pip_with_its_seam_in_the_cubes_top_builds_at_its_closed_form() {
    use crate::common::membership::{ball as round, cuboid, holds};
    let cube = finished(
        "the cube",
        sweep::test_support::cube::<f64>(1.0, Tol::witness()),
        Tol::witness(),
    );
    let pip = ball(0.3, Vec3::new(0.5, 0.5, 1.0));
    let half = ball_volume(0.3) / 2.0;
    let (c, b) = (
        || cuboid((0.0, 1.0), (0.0, 1.0), (0.0, 1.0)),
        || round(Point3::new(0.5, 0.5, 1.0), 0.3),
    );
    let about = (Point3::new(0.1, 0.1, 0.6), Point3::new(0.9, 0.9, 1.4));
    for (op, want, region) in [
        (
            BooleanOp::Union,
            1.0 + half,
            Box::new(move |p| c()(p).max(b()(p))) as crate::common::membership::Region,
        ),
        (
            BooleanOp::Intersect,
            half,
            Box::new(move |p| c()(p).min(b()(p))),
        ),
        (
            BooleanOp::Subtract,
            1.0 - half,
            Box::new(move |p| c()(p).min(-b()(p))),
        ),
    ] {
        let what = format!("the cube {op:?} the pip");
        let r = run(op, &cube, &pip).unwrap_or_else(|e| panic!("{what}: {e:?}"));
        let body = r
            .body()
            .unwrap_or_else(|| panic!("{what}: empty"))
            .body
            .clone();
        holds(&what, &body, want, &region, about, 9);
    }
}

/// **A flipped sense bit on a tilted face is named, not measured.** The
/// faces a tilted cut leaves that hold neither pole — both faces of the
/// equal pair's lens, and the bitten cap `A ∖ B` keeps on B's sphere —
/// encode their side by their loop, so a lone flip of the bit reads as
/// `CurvedSenseInverted` in tier 3 and as `SenseContradicted` in the
/// mass properties, rather than as the complement's volume. The
/// complement the flipped bit claims holds both poles, which tier 3
/// names too (`ChartSingularityNotAVertex`).
#[test]
fn a_flipped_tilted_face_is_refused_by_name() {
    let a = ball(1.0, BASE);
    let b = ball(1.0, BASE + Vec3::new(1.4, 0.0, 0.0));
    let b_centre = BASE + Vec3::new(1.4, 0.0, 0.0);
    for (label, op, on_b_only) in [
        ("A ∩ B", BooleanOp::Intersect, false),
        ("A ∖ B", BooleanOp::Subtract, true),
    ] {
        let body = run(op, &a, &b).unwrap().body().unwrap().body.clone();
        let mut flipped = 0;
        for (k, f) in body.faces() {
            let on_b = matches!(
                body.get_surface(f.surface),
                Some(geom::Surface::Sphere { center, .. })
                    if (*center - geom_core::Point3::new(b_centre.x, b_centre.y, b_centre.z)).norm() < 1e-12
            );
            if on_b_only && !on_b {
                continue;
            }
            let inverted = body.flipped_face_sense_for_tests(k).expect("the face");
            assert_eq!(
                topo::validate_geometric(&inverted, Tol::witness()),
                Err(vec![
                    topo::ValidationError::CurvedSenseInverted { face: k },
                    topo::ValidationError::ChartSingularityNotAVertex {
                        face: k,
                        point: geom_brep::chart_singularity::Singularity::North,
                        lies: topo::SingularityLies::Interior,
                    },
                    topo::ValidationError::ChartSingularityNotAVertex {
                        face: k,
                        point: geom_brep::chart_singularity::Singularity::South,
                        lies: topo::SingularityLies::Interior,
                    },
                ]),
                "{label}: face {k:?} flipped"
            );
            assert!(
                matches!(
                    // Raw door: `inverted` is a body with a curved face flipped, which tier 3 refuses (CurvedSenseInverted).
                    topo::test_support::measure_unvalidated(&inverted, Tol::witness(), topo::test_support::pinned_resolution(Tol::witness())),
                    Err(topo::MassPropsError::Face {
                        face,
                        source: geom_brep::props::PropsError::SenseContradicted,
                    }) if face == k
                ),
                "{label}: face {k:?} flipped is refused, not measured"
            );
            flipped += 1;
        }
        assert!(flipped > 0, "{label}: a tilted face to flip");
    }
}

/// **A remnant face that keeps one pole and loses the other encodes its
/// side.** With B offset `(0.6, 1.2)` from A, B swallows A's north pole,
/// so A's remnant faces in `A ∪ B` keep the south pole on their loop
/// and have the north one off it, inside B; the side of the loop that
/// holds no pole is the face's. A flipped bit on any such face is named
/// by tier 3's check 6 and refused by the mass properties, where the
/// bit alone used to measure the complement. Faces with both poles on
/// their loop still encode no side (`MaterialSign::Unencoded`), the
/// residual check 4 reads as a lamina; this row asserts only on faces
/// whose side is encoded, and that there are some.
#[test]
fn a_flipped_remnant_with_a_pole_off_its_loop_is_refused_by_name() {
    let tol = Tol::witness();
    let band = geom_core::Band::linear(tol).unwrap();
    let a = ball(1.0, BASE);
    let b = ball(1.0, BASE + Vec3::new(0.6, 1.2, 0.0));
    let body = run(BooleanOp::Union, &a, &b)
        .unwrap_or_else(|e| panic!("A ∪ B refused: {e:?}"))
        .body()
        .expect("a body")
        .body
        .clone();
    let shared = lens_volume(1.0, 1.0, (0.6_f64.powi(2) + 1.2_f64.powi(2)).sqrt());
    assert_body("A ∪ B", &body, 2.0 * ball_volume(1.0) - shared);
    let mut encoded = 0;
    for (k, f) in body.faces() {
        let surface = body.get_surface(f.surface).expect("the surface");
        if !matches!(surface, geom::Surface::Sphere { .. }) {
            continue;
        }
        let (outer, _) = topo::props::loop_edges(&body, f.outer).expect("the loop");
        if geom_brep::props::boundary_material_sign(surface, &outer, band)
            == Ok(geom_brep::props::MaterialSign::Unencoded)
        {
            continue;
        }
        encoded += 1;
        let inverted = body.flipped_face_sense_for_tests(k).expect("the face");
        let errors = topo::validate_geometric(&inverted, tol).expect_err("tier 3 refuses");
        assert!(
            errors.contains(&topo::ValidationError::CurvedSenseInverted { face: k }),
            "face {k:?} flipped: {errors:?}"
        );
        assert!(
            matches!(
                // Raw door: `inverted` is a lamina (a flipped band), which tier 3 refuses (LaminaWedge).
                topo::test_support::measure_unvalidated(&inverted, tol, topo::test_support::pinned_resolution(tol)),
                Err(topo::MassPropsError::Face {
                    face,
                    source: geom_brep::props::PropsError::SenseContradicted,
                }) if face == k
            ),
            "face {k:?} flipped is refused, not measured"
        );
    }
    assert!(encoded > 0, "a remnant whose side its loop encodes");
}

/// **A tilted split of a sphere body refuses at the reduce.** The split
/// lane refuses a sphere face before its join, where CLEAVE's conic
/// pairing (`topo::splitting::join`'s `conic_pairs`) would walk the
/// section and hand each chord the arc it walked. A ball, the tilted
/// pair's union and its lens, each split by planes tilted against their
/// charts, all refuse there. When the split lane admits sphere faces this
/// goes red, and the chord takes the walk's arc with nothing more to
/// compose.
#[test]
fn a_tilted_split_of_a_sphere_body_refuses_at_the_reduce() {
    let tol = Tol::witness();
    let a = ball(1.0, BASE);
    let b = ball(1.0, BASE + Vec3::new(1.4, 0.0, 0.0));
    let body_of = |out: Result<topo::BooleanResult<f64>, topo::BooleanError>| match out {
        Ok(topo::BooleanResult::Body(bb)) => bb.body,
        other => panic!("the tilted pair builds: {other:?}"),
    };
    let union = body_of(topo::boolean::union(&a, &b, tol));
    let lens = body_of(topo::boolean::intersect(&a, &b, tol));
    for (name, body) in [("ball", &a), ("union", &union), ("lens", &lens)] {
        for (origin, normal) in [
            (Point3::new(2.0, 2.0, 0.5), Vec3::new(0.3, 1.0, 0.4)),
            (Point3::new(2.7, 2.3, 0.5), Vec3::new(0.2, 1.0, 0.7)),
            (Point3::new(2.7, 2.0, 0.5), Vec3::new(1.0, 0.3, 0.2)),
        ] {
            let plane = topo::test_support::split_plane(origin, normal, tol);
            match topo::split(body, &plane, tol) {
                Err(topo::SplitError::Reduce(
                    topo::SplitReduceError::CurvedBooleanUnsupported {
                        kind: geom::SurfaceKind::Sphere,
                        ..
                    },
                )) => {}
                other => panic!(
                    "{name} split through {origin:?} along {normal:?}: expected the split's \
                     reduce to refuse the sphere face, got {other:?}"
                ),
            }
        }
    }
}

/// **A section passing near a chart pole images at every band.** Two
/// unit balls whose centre line leans `s = 0.7 + δ` toward `y`, so the
/// radical circle passes within about `δ` of the first ball's north
/// pole. Its rows are projected images whose pieces refine toward the
/// pole until each piece's azimuth sector holds, so δ down to 1e-5
/// builds and certifies whole. Through the pole (δ = 0) azimuth names
/// no point, and the row refuses "not yet" on the azimuth sector (the
/// projected lane's own gap where the lane choice sent the circle,
/// `NoGeneralLane { gap: LaneGap::Pose }`); splitting
/// such an arc at the pole is the join's
/// (`work/reach/tilted-section-through-a-chart-pole-is-not-split-at-the-pole.md`).
///
/// **What the mesher answers** (measured in release, PR 5920's second
/// fix pass; `work/tess/a-tilted-circle-near-a-sphere-pole-refuses-at-the-sphere-arm.md`):
/// at 1e-4 and 1e-5 the circle's chord schedule near the pole asks
/// more than the sphere arm's per-edge budget at δ = 1e-2, and refuses
/// `ChordBudget` at once; at 3e-4 it is under the budget and folds a
/// triangle beside the pole (`FoldedTriangle`, 0.2 s in release), which
/// is not meshed here.
#[test]
fn a_section_passing_near_a_pole_builds_and_one_through_it_refuses_typed() {
    let c = Vec3::new(2.0, 2.0, 0.5);
    let pair = |d: f64| {
        let s = 0.7 + d;
        let lean = Vec3::new(1.4 * (1.0 - s * s).sqrt(), 1.4 * s, 0.0);
        (ball(1.0, c), ball(1.0, c + lean))
    };
    let expected = 2.0 * ball_volume(1.0) - lens_volume(1.0, 1.0, 1.4);
    for d in [1e-5, 1e-4, 3e-4] {
        let (a, b) = pair(d);
        let out = run(BooleanOp::Union, &a, &b)
            .unwrap_or_else(|e| panic!("δ {d}: the near-pole section refused: {e:?}"));
        let body = &out.body().unwrap_or_else(|| panic!("δ {d}: empty")).body;
        assert_solid(&format!("δ {d}"), body, expected);
        if d < 2e-4 {
            let drawn = mesh::tessellate(body, 1e-2, Tol::witness());
            assert!(
                matches!(
                    &drawn,
                    Err(mesh::TessellateError::SphereTrimShape {
                        cause: mesh::types::SphereTrimCause::ChordBudget,
                        ..
                    })
                ),
                "δ {d}: the near-pole schedule is past the arm's chord budget, got {:?}",
                drawn.map(|m| m.positions.len())
            );
        }
    }
    let (a, b) = pair(0.0);
    let out = run(BooleanOp::Union, &a, &b);
    assert!(
        matches!(
            &out,
            Err(topo::BooleanError::Pcurves {
                source: topo::PcurveMintError::Certify {
                    error: geom_brep::PcurveCertifyError::NoGeneralLane {
                        gap: geom_brep::LaneGap::Pose {
                            channel: geom_brep::SectorChannel::Azimuth,
                            ..
                        },
                        ..
                    },
                    ..
                },
            })
        ),
        "δ 0: the arc through the pole refuses not yet on its azimuth sector (the general \
         lane's own gap, PCERT E1's P3), got {out:?}"
    );
}

/// **A section near a pole meshes at every δ, coarse ones included.**
/// Unit balls at the origin and at `(1, 0.8, 0)`: the radical circle
/// passes the first ball's pole close enough that its chord schedule is
/// fine there, and the sphere arm meshes it from δ = 0.1 down to 1e-3,
/// each mesh enclosing the body. PR 5920's review measured the first
/// cut's near-pole share refusing at δ = 0.1 and meshing finer: a cutoff
/// that grows with δ. The schedule is held to `sizing::MAX_COUNT` alone
/// now, so a coarser δ asks fewer chords, never more.
#[test]
fn a_section_near_a_pole_meshes_at_every_delta() {
    let (a, b) = (
        ball(1.0, Vec3::new(0.0, 0.0, 0.0)),
        ball(1.0, Vec3::new(1.0, 0.8, 0.0)),
    );
    let out = run(BooleanOp::Union, &a, &b).expect("the pair unions");
    let body = &out.body().expect("a body").body;
    let expected = 2.0 * ball_volume(1.0) - lens_volume(1.0, 1.0, 1.0f64.hypot(0.8));
    assert_solid("the (1, 0.8, 0) pair", body, expected);
    for delta in [1e-1, 5e-2, 2e-2, 1e-2, 5e-3, 1e-3] {
        let m = mesh::tessellate(body, delta, Tol::witness())
            .unwrap_or_else(|e| panic!("δ {delta}: the pair meshes, got {e:?}"));
        mesh::validate::check_mesh(&m).unwrap_or_else(|e| panic!("δ {delta}: {e:?}"));
        crate::common::differential::assert_meshed_geometry(&format!("δ {delta}"), body, &m, delta);
    }
}

/// **A circle 1e-3 from the pole meshes at δ 1e-2 and folds at 4e-2.**
/// The near-pole pair of
/// [`a_section_passing_near_a_pole_builds_and_one_through_it_refuses_typed`]
/// at `d = 1e-3`: at δ 1e-2 the sphere arm meshes it, enclosing the body
/// and wound outward; at 4e-2 a triangle beside the pole folds over the
/// chord with no grid corner to drop (its three corners are boundary
/// points), and it refuses `FoldedTriangle`
/// (`work/tess/a-tilted-circle-near-a-sphere-pole-refuses-at-the-sphere-arm.md`).
#[test]
fn a_circle_near_the_pole_folds_at_a_coarse_delta() {
    let c = Vec3::new(2.0, 2.0, 0.5);
    let s = 0.7f64 + 1e-3;
    let lean = Vec3::new(1.4 * (1.0 - s * s).sqrt(), 1.4 * s, 0.0);
    let out = run(BooleanOp::Union, &ball(1.0, c), &ball(1.0, c + lean)).expect("the pair unions");
    let body = &out.body().expect("a body").body;
    let expected = 2.0 * ball_volume(1.0) - lens_volume(1.0, 1.0, 1.4);
    let m = mesh::tessellate(body, 1e-2, Tol::witness()).expect("δ 1e-2 meshes");
    mesh::validate::check_mesh(&m).expect("and checks");
    crate::common::differential::assert_meshed_geometry("d 1e-3, δ 1e-2", body, &m, 1e-2);
    assert!((expected - crate::common::differential::body_volume(body)).abs() < 1e-9 * expected);
    let coarse = mesh::tessellate(body, 4e-2, Tol::witness());
    assert!(
        matches!(&coarse, Err(mesh::TessellateError::FoldedTriangle { .. })),
        "δ 4e-2: the boundary-only fold refuses typed, got {:?}",
        coarse.map(|m| m.positions.len())
    );
}
