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
//! The last rows are the boundary, measured on BOTH kinds — and the
//! scope is narrower than "curved walls". It is walls whose PROFILE
//! CONSTRAINT IS A CIRCLE. A partial revolve whose wall is a CYLINDER
//! hollows today: `sf2b_axial`'s quarter-turn wedge is exactly that
//! body, and its rim vertex carries TWO profile constraints (the
//! cylinder's line and the cap's), with the meridian plane supplying
//! only the azimuth. A sphere or torus wall whose profile is the whole
//! closed circle carries ONE — and the two fixtures below then refuse
//! at two DIFFERENT doors:
//!
//! - the klein elbow (torus wall, no second profile surface at the rim
//!   vertex at all) refuses `TogetherAxialCorner`, "one profile
//!   constraint meets here…", and the section that rim would need is a
//!   quartic (spiric) curve;
//! - the sphere lune's CORNERS do solve, through the axis-pole arm
//!   where `ρ = 0` is a geometric fact rather than a carried datum;
//!   what refuses is the rim EDGE — `TogetherEdgeDisagreement`, whose
//!   gap is exactly the wall thickness — because the moved rim circle
//!   is centred off the axis and the mint has no arm for one.
//!
//! So the partial revolve's rim is not a torus gap, and it is not
//! every curved wall's gap either: it is a circle-profile wall's, at
//! two doors.

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

/// **How far the transcript's refused-form gaps and this file's
/// re-derivations may differ, and why one bound covers both.**
///
/// The two quoted numbers are the DOOR's own — the `gap` payload the
/// per-chart re-anchor produced through its whole evaluation chain,
/// copied out of the C5ARMS STOP transcript. The rows re-derive them
/// from the closed form as `5/64 − ‖·‖`, which is a shorter chain, so
/// they agree as REAL NUMBERS and not bit-for-bit, and the residue is
/// per-operand. MEASURED at this head:
///
/// | fixture | re-derived | transcript | ulps | relative |
/// |---|---|---|---|---|
/// | barrel | `6.09730892739932e-3` | `6.0973089273993215e-3` | 2 | `−2.85e-16` |
/// | teapot | `4.422022405807788e-3` | `4.422022405807807e-3` | 22 | `−4.32e-15` |
///
/// **`hypot` is not the cause**, which an earlier note here claimed: a
/// bare `√(x² + y²)` on these two operand pairs returns the SAME BITS
/// as `hypot` — checked, both fixtures, `3f78f97de6fd1660` and
/// `3f721cd399d78bf0`. What differs is the length of the chain the
/// door took to the same real number. One bound, set a factor of two
/// above the larger measured residue, therefore replaces the two
/// unexplained per-row tolerances this file used to carry.
const GAP_REL: f64 = 1e-14;

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
    assert!(
        (gap - 6.0973089273993215e-3).abs() <= GAP_REL * gap,
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
        (gap - 4.422022405807807e-3).abs() <= GAP_REL * gap,
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
/// **The pose carries a TRANSLATION, and that is the point of it.** The
/// door canonicalizes its axis point to the axis line's own foot at the
/// world origin (`offset_axial`'s `axial_frame`), and a rotation ABOUT
/// THE ORIGIN leaves that foot at the origin — every station is
/// unchanged and the canonicalization is never asked to do anything. So
/// the anchor here is `(1/4, −1/2, 1/8)`, dyadic and off the axis: the
/// posed body's axis line misses the world origin, its foot is a
/// computed point, and every station is a different number from the
/// unposed body's.
///
/// The comparison is a BIJECTION, not a subset with a matching length —
/// a solve that collapsed two corners onto one point and invented a
/// third elsewhere would pass a subset check with the right count. Each
/// wanted point consumes its nearest unused match, and the pool must
/// empty. MEASURED at this head: worst matched distance `2.9e-17` m on
/// the barrel, `2.1e-17` m on the belly — sub-ulp at these coordinates,
/// so the bound is the file's own tightest (`1e-15`) rather than the
/// `1e-12` this row was first written with.
#[test]
fn torax_the_torus_corners_survive_a_rigid_re_pose() {
    let map = Affine3::rotation_about_axis(
        Point3::new(0.25, -0.5, 0.125),
        Vec3::new(1.0, 0.0, 0.0),
        0.7,
    );
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
        let mut pool: Vec<Point3<f64>> = hollow_after
            .vertices()
            .map(|(_, v)| *hollow_after.get_point(v.point).expect("point"))
            .collect();
        assert_eq!(want.len(), pool.len(), "{what}: vertex count under re-pose");
        for w in &want {
            let (i, d) = pool
                .iter()
                .enumerate()
                .map(|(i, g)| (i, (*g - *w).norm()))
                .min_by(|a, b| a.1.total_cmp(&b.1))
                .expect("the pool is non-empty while wanted points remain");
            assert!(
                d <= 1e-15,
                "{what}: hollow-then-pose has {w:?}; the nearest unmatched \
                 pose-then-hollow point is {d} m away"
            );
            pool.remove(i);
        }
        assert!(
            pool.is_empty(),
            "{what}: the match is not a bijection — {} pose-then-hollow points \
             are unclaimed",
            pool.len()
        );
    }
}

/// **The operand gate, named — and named as NOT this arm's floor.** A
/// wall thicker than the tube is refused before any torus arithmetic
/// happens: `shell`'s `wall_clearance` sees the two planar caps facing
/// each other across `1/8` m of material while two walls need `3/16`,
/// and says so. That gate is KIND-AGNOSTIC — a box of the same height
/// refuses identically — so this row is a check that the operand door
/// still fires and still names both numbers, and it is deliberately
/// NOT evidence about the torus arm. The arm's own floor is the row
/// below.
#[test]
fn torax_a_wall_thicker_than_the_tube_refuses_typed() {
    let e = topo::shell(&torus_barrel(), 6.0 / 64.0, FIT_TOL, tol())
        .expect_err("a wall thicker than the tube has no cavity");
    println!("[torax] the over-thick wall refuses: {e}");
    let ShellError::WallClearance {
        gap,
        needed,
        face,
        other,
    } = e
    else {
        panic!("the over-thick wall is the operand clearance gate's, got {e}");
    };
    assert_ne!(face, other, "the gate names two distinct faces");
    assert!(
        (gap - 0.125).abs() <= 1e-15 && (needed - 0.1875).abs() <= 1e-15,
        "the caps stand 1/8 m apart and two walls of 6/64 need 3/16, got \
         gap {gap} needed {needed}"
    );
}

/// **The torus arm's OWN floor: the ring closing.** An inward wall on
/// these waisted fixtures moves the minor radius OUT, so the ring
/// convention `R > r > 0` is what eventually stops them — not the
/// operand's clearance gate, which has decades of room left. The mint
/// refuses [`geom_brep::OffsetError::TorusRing`], and this row brackets
/// the floor rather than asserting one side of it: `1/128` hollows,
/// `2/128` does not, and the realized minor radius the refusal echoes
/// is the tube's own major radius `R = 12/128`.
///
/// **This row is the one that is sensitive to the arm.** The refusal is
/// only reachable BECAUSE the axial door now takes a torus body: with
/// the `(Torus, Torus)` arm removed from `classify`, `is_axial` answers
/// `false` and `shell` falls to the per-chart loop — which, at the time
/// this was measured, refused at the C5 table's
/// `NeighborPairUnroutable(Plane, Torus)`. VERBS-C5ARMS has since
/// routed the pair, so the same counterfactual would now proceed one
/// door deeper and refuse at the per-chart reanchor gate — the
/// corner-accumulation family `offd2_r1_probes` pins at `8.331e-4` m
/// on the elbow cap — never through the mint. Nothing else in this
/// suite refuses through the mint.
#[test]
fn torax_the_torus_arms_floor_is_the_ring_closing() {
    // Below the floor: the same fixture the closed-form rows use.
    hollowed("the torus barrel at t = 1/128", &torus_barrel());

    // At it. `r + t` reaches `R`, so the offset tube would swallow its
    // own hole.
    let e = topo::shell(&torus_barrel(), 2.0 / 128.0, FIT_TOL, tol())
        .expect_err("an offset tube that reaches the major radius has no ring left");
    println!("[torax] the closed ring refuses: {e:?}");
    let ShellError::Face { error, .. } = e else {
        panic!("not the offset door's refusal: {e}");
    };
    let topo::ReplaceFaceError::Offset { error, .. } = *error else {
        panic!("the ring floor is the surface mint's, not a corner's: {error}");
    };
    let geom_brep::OffsetError::TorusRing { realized_minor } = error else {
        panic!("the mint must name the ring convention, got {error}");
    };
    assert!(
        (realized_minor - 12.0 / 128.0).abs() <= 1e-15,
        "the realized minor radius reaches R = 12/128 exactly, got {realized_minor}"
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

/// **The SPHERE lune refuses the same RIM at a DIFFERENT door**, so the
/// gap is a circle-profile wall's rather than the torus kind's.
///
/// A quarter revolve of a half-disc whose diameter lies on the axis: a
/// sphere wall, two meridian caps, poles on the axis — the elbow's
/// shape with the profile circle slid onto it.
///
/// **Its CORNERS solve.** The rim arc ends on the axis poles, where
/// `ρ = 0` is a geometric fact and the pole arm answers without any
/// carried datum, so the elbow's `TogetherAxialCorner` never fires
/// here. What refuses is the rim EDGE: the moved rim section IS a
/// circle, but its centre is off the axis and the latitude mint has no
/// arm for one, so the edge's two ends are solved a wall thickness
/// apart and `TogetherEdgeDisagreement` says so — this row asserts that
/// gap is the thickness itself and not some other number.
///
/// Measured here so that "the partial revolve's rim" is documented as a
/// capability with TWO unbuilt doors, one per kind, rather than as
/// something the torus arm failed to reach — and so that the scope is
/// not read as "every curved wall", which `sf2b_axial`'s
/// cylinder-walled quarter-turn wedge disproves by hollowing.
#[test]
fn torax_the_sphere_lune_refuses_the_rim_at_the_other_door() {
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
    let topo::ReplaceFaceError::TogetherEdgeDisagreement { gap, .. } = *error else {
        panic!(
            "the sphere lune's rim refuses at the EDGE its two moved surfaces disagree \
             about — its corners solve at the poles — got {error}"
        );
    };
    // And the disagreement is exactly the wall, not some other length:
    // the moved rim circle is the unmoved one translated inward by `t`,
    // so the far end misses the carrier by the whole thickness.
    assert!(
        (gap - 0.05).abs() <= 1e-15,
        "the gap is the wall thickness itself, got {gap}"
    );
}
