//! **The axial door's TORUS arm**: the offset-axial reduction one kind
//! wider, and what it does not reach.
//!
//! The claim is not "it hollows". A corner solved to the wrong point
//! still produces a valid two-shell body, so every row here carries the
//! CLOSED FORM of the corner it moved — a circle–line intersection at
//! the offset radii, on dyadic inputs — and asserts the solved `(ρ, h)`
//! against it. The refused-form gap each fixture used to produce is
//! quoted beside it, so a row says what changed as well as what is.
//!
//! The last rows are the boundary, measured on BOTH kinds: a rim whose
//! two moved surfaces are a wall and a meridian plane that has just
//! stopped containing the axis is a section this door cannot carry —
//! for a torus it is a quartic (spiric) curve, and even for the SPHERE,
//! where it is still a circle, the mint has no arm for one centred off
//! the axis. So the partial revolve's rim is not a torus gap; it is a
//! rim capability that does not exist yet for any curved wall.

#![allow(clippy::unwrap_used, clippy::expect_used, clippy::panic)]

use geom::Surface;
use geom_core::{Affine3, Point2, Point3, Tol, Vec2, Vec3};
use profile::{Profile, ProfileLoop, ProfileVertex, RawLoop, SketchPlane};
use sweep::{Revolution, RevolveAxis, revolve};
use topo::{Body, FaceKey, LoopBoundary, ShellError, VertexKey, transform_rigid};

fn p2(x: f64, y: f64) -> Point2<f64> {
    Point2::new(x, y)
}

fn tol() -> Tol {
    Tol::witness()
}

const FIT_TOL: f64 = 1e-6;

/// The wall thickness every accepting row here hollows by — the tour's
/// own.
const T: f64 = 1.0 / 128.0;

/// Revolved about the `y` axis through the origin, so a vertex's axial
/// coordinates are `(hypot(x, z), y)`.
fn revolved(lp: ProfileLoop<f64>, turn: Revolution<f64>) -> Body<f64> {
    let profile = Profile::new(SketchPlane::xy(), vec![lp])
        .validate(tol())
        .expect("the meridian validates");
    revolve(
        &profile,
        RevolveAxis {
            origin: p2(0.0, 0.0),
            dir: Vec2::new(0.0, 1.0),
        },
        turn,
        tol(),
    )
    .expect("the meridian revolves")
    .body
}

/// The bulge (`tan(θ/4)`) of the arc from `a` to `b` about `c`.
fn bulge(a: Point2<f64>, b: Point2<f64>, c: Point2<f64>) -> f64 {
    let (u, v) = (a - c, b - c);
    (u.perp_dot(v).atan2(u.dot(v)) / 4.0).tan()
}

/// `p` in the `(ρ, h)` half-plane of the `y` axis.
fn axial(p: Point3<f64>) -> (f64, f64) {
    ((p.x * p.x + p.z * p.z).sqrt(), p.y)
}

/// `body`'s distinct corners in that half-plane.
fn corners(body: &Body<f64>) -> Vec<(f64, f64)> {
    let mut out: Vec<(f64, f64)> = Vec::new();
    for (_, v) in body.vertices() {
        let c = axial(*body.get_point(v.point).expect("a vertex carries a point"));
        if !out
            .iter()
            .any(|q| (q.0 - c.0).abs() < 1e-12 && (q.1 - c.1).abs() < 1e-12)
        {
            out.push(c);
        }
    }
    out
}

/// `(ρ, h)` is among `body`'s corners to `1e-14` m — an ABSOLUTE bound
/// on bodies whose radii are ~5e-2 m, so a relative agreement of about
/// `2e-13`, stated both ways.
fn has_corner(body: &Body<f64>, rho: f64, h: f64, what: &str) {
    let all = corners(body);
    assert!(
        all.iter()
            .any(|q| (q.0 - rho).abs() <= 1e-14 && (q.1 - h).abs() <= 1e-14),
        "{what}: no corner at the closed form (ρ, h) = ({rho}, {h}); the body has {all:?}"
    );
}

/// The sealed hollow, with tier 3 and the two-shell shape first.
fn hollowed(what: &str, body: &Body<f64>) -> Body<f64> {
    let out = topo::shell(body, T, FIT_TOL, tol())
        .unwrap_or_else(|e| panic!("{what}: the axial door must hollow this, got {e}"));
    assert_eq!(
        topo::validate_geometric(&out, tol()),
        Ok(()),
        "{what}: tier 3"
    );
    assert_eq!(out.shells().count(), 2, "{what}: outer + cavity");
    out
}

/// Every vertex on `face`'s own boundary.
fn face_vertices(body: &Body<f64>, face: FaceKey) -> Vec<VertexKey> {
    let data = body.get_face(face).expect("face");
    let mut out = Vec::new();
    for lk in core::iter::once(data.outer).chain(data.rings.iter().copied()) {
        let LoopBoundary::Cycle { first } = body.get_loop(lk).expect("loop").boundary else {
            continue;
        };
        for he in body.loop_cycle(first).expect("cycle") {
            out.push(body.get_half_edge(he).expect("half edge").start);
        }
    }
    out
}

/// A point's signed distance from an analytic surface, in the `y`-axis
/// frame these fixtures share.
fn residual(s: &Surface<f64>, p: Point3<f64>) -> f64 {
    let (rho, h) = axial(p);
    match s {
        Surface::Plane { origin, normal, .. } => normal.dot(p - *origin),
        Surface::Cylinder { radius, .. } => rho - *radius,
        Surface::Sphere { center, radius, .. } => p.distance(*center) - *radius,
        Surface::Torus {
            center,
            major_radius,
            minor_radius,
            ..
        } => (rho - *major_radius).hypot(h - center.y) - *minor_radius,
        other => panic!("no fixture here carries a {other:?}"),
    }
}

// ---------------------------------------------------------------------
// The two full-revolve consumers
// ---------------------------------------------------------------------

/// **The barrel bulged about a centre OFF the axis.** The same two
/// junction stations and the same `5/64` meridian radius as the tour's
/// sphere-zone barrel, about the OTHER centre on their perpendicular
/// bisector — so the wall is a TORUS: `R = 6/64`, `r = 5/64`,
/// `h_c = 4/64`, a 3-4-5 at each junction with both residuals exactly
/// zero.
fn torus_barrel() -> Body<f64> {
    let c = p2(6.0 / 64.0, 1.0 / 16.0);
    let (lo, hi) = (p2(3.0 / 64.0, 0.0), p2(3.0 / 64.0, 8.0 / 64.0));
    revolved(
        RawLoop::new(vec![
            ProfileVertex::new(p2(0.0, 0.0), 0.0),
            ProfileVertex::new(lo, bulge(lo, hi, c)),
            ProfileVertex::new(hi, 0.0),
            ProfileVertex::new(p2(0.0, 8.0 / 64.0), 0.0),
        ]),
        Revolution::Full,
    )
}

/// **The teapot's wall-1 belly.** The pot's own foot and mouth, its
/// belly bulged about `(7/64, 5/64)` — off the axis, so a TORUS with
/// `R = 7/64`, `r = 5/64`, `h_c = 5/64`.
fn torus_belly() -> Body<f64> {
    let c = p2(7.0 / 64.0, 5.0 / 64.0);
    let (lo, hi) = (p2(4.0 / 64.0, 1.0 / 64.0), p2(3.0 / 64.0, 8.0 / 64.0));
    revolved(
        RawLoop::new(vec![
            ProfileVertex::new(p2(0.0, 0.0), 0.0),
            ProfileVertex::new(p2(4.0 / 64.0, 0.0), 0.0),
            ProfileVertex::new(lo, bulge(lo, hi, c)),
            ProfileVertex::new(hi, 0.0),
            ProfileVertex::new(p2(0.0, 8.0 / 64.0), 0.0),
        ]),
        Revolution::Full,
    )
}

/// **The barrel's cap × wall corners solve as Station × torus-circle
/// roots, in closed form.**
///
/// Both caps move inward by `T` to the stations `1/128` and `15/128`.
/// The wall's arc runs CW about a centre OUTSIDE it, so the tube's
/// material side faces the axis and an inward wall moves the minor
/// radius OUT, to `r + T = 11/128`, about the UNMOVED tube centre
/// `(R, h_c) = (12/128, 8/128)`. Each station stands `7/128` from
/// `h_c`, so the circle's half-chord there is
/// `√(11² − 7²)/128 = √72/128 = 6√2/128`, and the root a small offset
/// keeps is the INNER one:
///
/// ```text
/// ρ = (12 − 6√2)/128 = 3(2 − √2)/64     h ∈ {1/128, 15/128}
/// ```
///
/// The refused form this replaces left the corner on the UNMOVED
/// meridian circle, `5/64 − hypot(3/64 − 6/64, 1/128 − 1/16)` =
/// `6.0973089273993215e-3` m off its own carrier — nine orders above ε,
/// and the number the per-chart loop refused on.
#[test]
fn torax_the_torus_barrel_corners_solve_in_closed_form() {
    let out = hollowed("the torus barrel", &torus_barrel());
    let rho = 3.0 * (2.0 - 2.0f64.sqrt()) / 64.0;
    has_corner(&out, rho, 1.0 / 128.0, "the barrel's base junction");
    has_corner(&out, rho, 15.0 / 128.0, "the barrel's mouth junction");

    let gap = 5.0 / 64.0 - (3.0f64 / 64.0 - 6.0 / 64.0).hypot(1.0 / 128.0 - 1.0 / 16.0);
    // The transcript's number, quoted rather than re-derived. The
    // closed form and the door's own evaluation order agree to `2e-16`
    // RELATIVE and not bit-for-bit: `hypot` and the door's 2-D norm
    // associate their three operations differently, which is a fact
    // about the two spellings and not about the geometry.
    assert!(
        (gap - 6.0973089273993215e-3).abs() <= 1e-15 * gap,
        "the C5ARMS STOP transcript's barrel gap, got {gap}"
    );
}

/// **The teapot's wall-1 corners, one of each shape.**
///
/// Same waisted sense, so the minor radius again moves out to
/// `r + T = 11/128` about `(R, h_c) = (14/128, 10/128)`. The foot
/// junction is a CYLINDER × torus-circle root: the foot wall moves to
/// `ρ = 7/128`, which stands `7/128` from `R`, so
/// `h = (10 − √72)/128 = (10 − 6√2)/128`. The mouth junction is a
/// Station × torus-circle root at `h = 15/128`, `5/128` from `h_c`, so
/// `ρ = (14 − √96)/128 = (7 − 2√6)/64`.
///
/// The refused form, quoted:
/// `5/64 − hypot(3/64 − 7/64, 8/64 − 1/128 − 5/64)` =
/// `4.422022405807807e-3` m.
#[test]
fn torax_the_teapot_belly_corners_solve_in_closed_form() {
    let out = hollowed("the teapot's torus belly", &torus_belly());
    has_corner(
        &out,
        7.0 / 128.0,
        (10.0 - 6.0 * 2.0f64.sqrt()) / 128.0,
        "the belly's foot junction",
    );
    has_corner(
        &out,
        (7.0 - 2.0 * 6.0f64.sqrt()) / 64.0,
        15.0 / 128.0,
        "the belly's mouth junction",
    );

    let gap =
        5.0 / 64.0 - (3.0f64 / 64.0 - 7.0 / 64.0).hypot(8.0 / 64.0 - 1.0 / 128.0 - 5.0 / 64.0);
    assert!(
        (gap - 4.422022405807807e-3).abs() <= 1e-14 * gap,
        "the C5ARMS STOP transcript's teapot gap, got {gap}"
    );
}

/// **Every corner lands on BOTH its own moved surfaces.**
///
/// The closed forms say where a corner is; this says it is ON the
/// geometry it belongs to, which is the claim a wrong corner breaks
/// without any tier noticing. Read from the outside: each face's own
/// boundary vertices are metered against that face's surface.
#[test]
fn torax_every_torus_corner_lies_on_its_own_moved_surfaces() {
    for (what, body) in [
        ("the torus barrel", torus_barrel()),
        ("the teapot's torus belly", torus_belly()),
    ] {
        let out = hollowed(what, &body);
        let mut metered = 0usize;
        for (face, f) in out.faces() {
            let surface = out
                .get_surface(f.surface)
                .expect("a face carries a surface");
            for v in face_vertices(&out, face) {
                let p = *out
                    .get_point(out.get_vertex(v).expect("vertex").point)
                    .expect("point");
                let d = residual(surface, p);
                assert!(
                    d.abs() <= 1e-15,
                    "{what}: {v:?} stands {d} m off {face:?}'s own surface"
                );
                metered += 1;
            }
        }
        assert!(metered >= 16, "{what}: only {metered} incidences metered");
    }
}

/// **The corner solve reads no global axis.** Hollowing and re-posing
/// commute: the same body posed by a rigid motion hollows to the same
/// points, compared in the posed frame so no inverse is invented.
///
/// The door canonicalizes its axis point to the world origin's foot, so
/// a re-pose is exactly the case that would catch a coordinate leaking
/// into the arithmetic.
#[test]
fn torax_the_torus_corners_survive_a_rigid_re_pose() {
    let map =
        Affine3::rotation_about_axis(Point3::new(0.0, 0.0, 0.0), Vec3::new(1.0, 0.0, 0.0), 0.7);
    for (what, body) in [
        ("the torus barrel", torus_barrel()),
        ("the teapot's torus belly", torus_belly()),
    ] {
        let posed_after = transform_rigid(&hollowed(what, &body), &map, tol())
            .unwrap_or_else(|e| panic!("{what}: the hollow re-poses, got {e}"));
        let posed_first = transform_rigid(&body, &map, tol())
            .unwrap_or_else(|e| panic!("{what}: the operand re-poses, got {e}"));
        let hollow_after = topo::shell(&posed_first, T, FIT_TOL, tol())
            .unwrap_or_else(|e| panic!("{what}, re-posed: {e}"));
        let want: Vec<Point3<f64>> = posed_after
            .vertices()
            .map(|(_, v)| *posed_after.get_point(v.point).expect("point"))
            .collect();
        let got: Vec<Point3<f64>> = hollow_after
            .vertices()
            .map(|(_, v)| *hollow_after.get_point(v.point).expect("point"))
            .collect();
        assert_eq!(want.len(), got.len(), "{what}: vertex count under re-pose");
        for w in &want {
            assert!(
                got.iter().any(|g| (*g - *w).norm() <= 1e-12),
                "{what}: hollow-then-pose has {w:?}, pose-then-hollow does not"
            );
        }
    }
}

/// **The planted red: a request the torus arm must refuse rather than
/// build.** A wall thicker than the tube leaves no minor radius at all,
/// and the arm this unit added must not carry that past the mint's own
/// floor into a corner solve on a surface that does not exist.
#[test]
fn torax_a_wall_thicker_than_the_tube_refuses_typed() {
    let e = topo::shell(&torus_barrel(), 6.0 / 64.0, FIT_TOL, tol())
        .expect_err("a wall thicker than the tube has no cavity");
    println!("[torax] the over-thick wall refuses: {e}");
    assert!(
        !matches!(e, ShellError::Corrupt { .. }),
        "the refusal must be typed and about the geometry, got {e}"
    );
}

// ---------------------------------------------------------------------
// The boundary: the partial revolve's rim, on both kinds
// ---------------------------------------------------------------------

/// **The klein elbow's rim is this unit's measured blocker, and it is
/// not a torus one.**
///
/// A partial revolve's meridian cap CONTAINS the axis at rest and stops
/// containing it the moment it is offset inward — by exactly the wall
/// thickness. The rim vertex is then two distinct surfaces with only
/// ONE profile constraint between them (the wall's meridian circle; the
/// cap fixes an AZIMUTH, not a `(ρ, h)`), and the door says so.
///
/// What the missing coordinate would have to be is the wall chart's own
/// `v`-seam, a second carried datum in the profile half-plane — and
/// there is no precedent for it, because on a SPHERE that same seam
/// lands on the axis poles, where `ρ = 0` is a geometric fact and no
/// datum is carried at all. The row below measures that.
#[test]
fn torax_the_partial_revolve_rim_has_one_profile_constraint() {
    let r = 0.275_f64;
    let elbow = {
        let profile = Profile::new(
            SketchPlane::xy(),
            vec![ProfileLoop::new(vec![
                ProfileVertex::new(p2(-r, 0.0), 1.0),
                ProfileVertex::new(p2(r, 0.0), 1.0),
            ])],
        )
        .validate(tol())
        .expect("the elbow's cross-section validates");
        revolve(
            &profile,
            RevolveAxis {
                origin: p2(1.2, 0.0),
                dir: Vec2::new(0.0, -1.0),
            },
            Revolution::Partial(-core::f64::consts::FRAC_PI_2),
            tol(),
        )
        .expect("the elbow revolves")
        .body
    };
    let e = topo::shell(&elbow, 0.05, FIT_TOL, tol())
        .expect_err("the elbow's rim has no second profile constraint");
    let ShellError::Face { error, .. } = e else {
        panic!("not the offset door's refusal: {e}");
    };
    let topo::ReplaceFaceError::TogetherAxialCorner { what, surfaces, .. } = *error else {
        panic!("the rim must refuse at the corner it is about: {error}");
    };
    assert_eq!(surfaces, 2, "wall + meridian cap");
    assert!(
        what.contains("one profile constraint"),
        "the refusal must say what is missing, got {what}"
    );
    println!("[torax] the elbow rim: {surfaces} surfaces — {what}");

    // And the section that rim edge would need is not a circle. The
    // moved cap stands `t` from the axis and stays parallel to it, so
    // it cuts the moved torus in a SPIRIC section: sampled here as the
    // oval's own half-width against its half-height, which a circle
    // would make equal.
    let (big, small) = (1.2_f64, r - 0.05);
    let d = 0.05_f64;
    let out = ((big + small).powi(2) - d * d).sqrt();
    let inn = ((big - small).powi(2) - d * d).sqrt();
    let half_width = (out - inn) / 2.0;
    let half_height = small;
    assert!(
        (half_width - half_height).abs() > 1e-6,
        "a plane parallel to the axis at {d} m cuts this torus in a quartic, not a circle: \
         {half_width} vs {half_height}"
    );
    println!(
        "[torax] the moved rim section is spiric: half-width {half_width}, half-height \
         {half_height}, difference {}",
        half_width - half_height
    );
}

/// **The SPHERE lune refuses the same rim, so the gap is not the torus
/// kind's.**
///
/// A quarter revolve of a half-disc whose diameter lies on the axis:
/// a sphere wall, two meridian caps, poles on the axis — the elbow's
/// shape with the profile circle slid onto it. Its rim section after
/// the offset IS a circle, and the door still cannot carry it, because
/// that circle's centre is off the axis and the latitude mint has no
/// arm for one. The refusal is measured here so that "the partial
/// revolve's rim" is documented as a rim capability nobody has built
/// rather than as something the torus arm failed to reach.
#[test]
fn torax_the_sphere_lune_refuses_the_same_rim() {
    let r = 0.3_f64;
    let lune = {
        let profile = Profile::new(
            SketchPlane::xy(),
            vec![ProfileLoop::new(vec![
                ProfileVertex::new(p2(0.0, -r), 0.0),
                ProfileVertex::new(p2(0.0, r), -1.0),
            ])],
        )
        .validate(tol())
        .expect("the lune's cross-section validates");
        revolve(
            &profile,
            RevolveAxis {
                origin: p2(0.0, 0.0),
                dir: Vec2::new(0.0, 1.0),
            },
            Revolution::Partial(core::f64::consts::FRAC_PI_2),
            tol(),
        )
        .expect("the lune revolves")
        .body
    };
    assert!(
        lune.surfaces()
            .any(|(_, s)| matches!(s, Surface::Sphere { .. })),
        "the lune's wall is a sphere"
    );
    let e = topo::shell(&lune, 0.05, FIT_TOL, tol())
        .expect_err("the sphere's partial-revolve rim does not carry either");
    println!("[torax] the sphere lune's rim: {e}");
    let ShellError::Face { error, .. } = e else {
        panic!("not the offset door's refusal: {e}");
    };
    assert!(
        matches!(
            *error,
            topo::ReplaceFaceError::TogetherEdgeDisagreement { .. }
        ),
        "the sphere lune's rim refuses at the edge its two moved surfaces disagree about, \
         got {error}"
    );
}
