//! **The axial door's acceptance**: the curved-corner fixtures hollow,
//! and their walls are pinned to closed forms.
//!
//! "It hollows" is not the claim. A corner solved to the WRONG point
//! still produces a valid two-shell body — that is the whole reason
//! `ReanchorOffCarrier` had to stay load-bearing through PR-2a — so
//! each fixture here carries the exact volume of its cavity, derived in
//! this file from the solid of revolution rather than read off the
//! door. A wrong corner moves that number and nothing else has to
//! notice.

#![allow(clippy::unwrap_used, clippy::expect_used, clippy::panic)]

use core::f64::consts::PI;

use geom_core::{Point2, Tol, Vec2};
use profile::{Profile, ProfileLoop, ProfileVertex, RawLoop, SketchPlane};
use sweep::{Revolution, RevolveAxis, revolve};
use topo::{Body, ShellError};

fn p2(x: f64, y: f64) -> Point2<f64> {
    Point2::new(x, y)
}

const FIT_TOL: f64 = 1e-6;

/// The wall thickness every row here uses.
const T: f64 = 1.0 / 128.0;

fn revolved(lp: ProfileLoop<f64>, turn: Revolution<f64>) -> Body<f64> {
    let profile = Profile::new(SketchPlane::xy(), vec![lp])
        .validate(Tol::witness())
        .expect("the meridian validates");
    revolve(
        &profile,
        RevolveAxis {
            origin: p2(0.0, 0.0),
            dir: Vec2::new(0.0, 1.0),
        },
        turn,
        Tol::witness(),
    )
    .expect("the meridian revolves")
    .body
}

/// The bulge (`tan(θ/4)`) of the arc from `a` to `b` about `c`.
fn bulge(a: Point2<f64>, b: Point2<f64>, c: Point2<f64>) -> f64 {
    let (u, v) = (a - c, b - c);
    (u.perp_dot(v).atan2(u.dot(v)) / 4.0).tan()
}

/// **The sphere-zone vase**: a belly on a sphere centred on the axis,
/// between two caps normal to it.
fn sphere_zone_vase(r: f64, h: f64) -> Body<f64> {
    let c = p2(0.0, h / 2.0);
    revolved(
        RawLoop::new(vec![
            ProfileVertex::new(p2(0.0, 0.0), 0.0),
            ProfileVertex::new(p2(r, 0.0), bulge(p2(r, 0.0), p2(r, h), c)),
            ProfileVertex::new(p2(r, h), 0.0),
            ProfileVertex::new(p2(0.0, h), 0.0),
        ]),
        Revolution::Full,
    )
}

/// **The cone frustum** between two caps normal to its axis.
fn cone_frustum(r0: f64, r1: f64, h: f64) -> Body<f64> {
    revolved(
        ProfileLoop::new(vec![
            ProfileVertex::new(p2(0.0, 0.0), 0.0),
            ProfileVertex::new(p2(r0, 0.0), 0.0),
            ProfileVertex::new(p2(r1, h), 0.0),
            ProfileVertex::new(p2(0.0, h), 0.0),
        ]),
        Revolution::Full,
    )
}

/// **The partial-revolve wedge**: a quarter turn, so its meridian caps
/// are planes CONTAINING the axis.
fn wedge(r: f64, h: f64) -> Body<f64> {
    revolved(
        ProfileLoop::new(vec![
            ProfileVertex::new(p2(0.0, 0.0), 0.0),
            ProfileVertex::new(p2(r, 0.0), 0.0),
            ProfileVertex::new(p2(r, h), 0.0),
            ProfileVertex::new(p2(0.0, h), 0.0),
        ]),
        Revolution::Partial(core::f64::consts::FRAC_PI_2),
    )
}

/// The volume of the solid of revolution of a sphere of radius `r`
/// between the two planes `±half` from its centre: `π(r²·2·half −
/// (2·half)³/12)`, which is `π∫(r² − y²)dy` written once.
fn zone_volume(r: f64, half: f64) -> f64 {
    PI * 2.0 * (r * r * half - half * half * half / 3.0)
}

/// A conical frustum's volume.
fn frustum_volume(r0: f64, r1: f64, h: f64) -> f64 {
    PI * h * (r0 * r0 + r0 * r1 + r1 * r1) / 3.0
}

/// The area of the disc of radius `r` cut by two PERPENDICULAR chords,
/// each at distance `t` from the centre, on the inner side of both —
/// the wedge cavity's own cross-section, since the two meridian caps
/// move inward by `t` and meet at a right angle.
fn two_chord_area(r: f64, t: f64) -> f64 {
    let top = (r * r - t * t).sqrt();
    let f = |u: f64| u * (r * r - u * u).sqrt() / 2.0 + r * r * (u / r).asin() / 2.0;
    f(top) - f(t) - t * (top - t)
}

/// The measured wall volume of `body` hollowed by `T`, with tier 3 and
/// the two-shell shape asserted first.
///
/// The `1e-15` each row compares against is an ABSOLUTE bound in m³,
/// and the walls here are ~1e-4 m³, so it is a relative agreement of
/// about `4e-12` — stated both ways because an absolute bound read as
/// a relative one flatters itself by eight orders of magnitude.
fn wall(what: &str, body: &Body<f64>) -> f64 {
    let tol = Tol::witness();
    let hollow = topo::shell(body, T, FIT_TOL, tol)
        .unwrap_or_else(|e| panic!("{what}: the axial door must hollow this, got {e}"));
    assert_eq!(
        topo::validate_geometric(&hollow, tol),
        Ok(()),
        "{what}: tier 3"
    );
    assert_eq!(hollow.shells().count(), 2, "{what}: outer + cavity");
    let props = topo::mass_properties(&hollow, tol).expect("props");
    println!("[axial] {what}: wall volume {}", props.volume);
    props.volume
}

/// **The sphere-zone vase hollows, and its wall is the difference of
/// two spherical zones.** The cavity's sphere is concentric and
/// smaller by the wall; its two caps have each come in by the wall; so
/// the closed form is the same integral at `(R − t, h/2 − t)`.
#[test]
fn the_sphere_zone_vase_hollows_to_its_closed_form() {
    let (r, h): (f64, f64) = (3.0 / 64.0, 8.0 / 64.0);
    let big = (r * r + h * h / 4.0).sqrt();
    let want = zone_volume(big, h / 2.0) - zone_volume(big - T, h / 2.0 - T);
    let got = wall("the sphere-zone vase", &sphere_zone_vase(r, h));
    assert!(
        (got - want).abs() <= 1e-15,
        "the wall's closed form is {want}, got {got}"
    );
}

/// **The cone frustum hollows, and its wall is the difference of two
/// frusta.** The cavity's cone is the operand's offset perpendicular by
/// the wall — its apex slides `t/sin α` along the axis — so the
/// cavity's radii are read off THAT cone at the two moved cap
/// stations, which is the whole content of the corner solve.
#[test]
fn the_cone_frustum_hollows_to_its_closed_form() {
    let (r0, r1, h): (f64, f64, f64) = (4.0 / 64.0, 2.0 / 64.0, 8.0 / 64.0);
    // The operand's cone: apex above, half-angle from the meridian.
    let tan_a = (r0 - r1) / h;
    let alpha = tan_a.atan();
    let apex = r0 / tan_a;
    let apex_in = apex - T / alpha.sin();
    let (c0, c1) = ((apex_in - T) * tan_a, (apex_in - (h - T)) * tan_a);
    let want = frustum_volume(r0, r1, h) - frustum_volume(c0, c1, h - 2.0 * T);
    let got = wall("the cone frustum", &cone_frustum(r0, r1, h));
    assert!(
        (got - want).abs() <= 1e-15,
        "the wall's closed form is {want}, got {got}"
    );
}

/// **The partial-revolve wedge hollows**, and its cavity is NOT a
/// wedge: the two meridian caps move inward along their own normals, so
/// the cross-section is the disc of radius `r − t` cut by two
/// perpendicular chords at distance `t`. That is the azimuth solve's
/// own answer written as an area, and it is what separates a correct
/// corner from one transported along the wall.
#[test]
fn the_partial_revolve_wedge_hollows_to_its_closed_form() {
    let (r, h) = (3.0 / 64.0, 8.0 / 64.0);
    let want = PI * r * r * h / 4.0 - two_chord_area(r - T, T) * (h - 2.0 * T);
    let got = wall("the partial-revolve wedge", &wedge(r, h));
    assert!(
        (got - want).abs() <= 1e-15,
        "the wall's closed form is {want}, got {got}"
    );
}

/// **A cylinder between two caps normal to its axis still hollows**,
/// and now on the AXIAL branch rather than the per-chart one. The
/// closed form is the row that says the branch change did not move the
/// answer: a square junction's simultaneous corner IS its transported
/// one.
#[test]
fn the_drum_still_hollows_on_the_new_branch() {
    let (r, h) = (3.0 / 64.0, 8.0 / 64.0);
    let want = PI * r * r * h - PI * (r - T) * (r - T) * (h - 2.0 * T);
    let got = wall(
        "the drum",
        &revolved(
            ProfileLoop::new(vec![
                ProfileVertex::new(p2(0.0, 0.0), 0.0),
                ProfileVertex::new(p2(r, 0.0), 0.0),
                ProfileVertex::new(p2(r, h), 0.0),
                ProfileVertex::new(p2(0.0, h), 0.0),
            ]),
            Revolution::Full,
        ),
    );
    assert!(
        (got - want).abs() <= 1e-15,
        "the wall's closed form is {want}, got {got}"
    );
}

/// **The door's own boundary, measured rather than presumed.**
///
/// One row is a refusal that stands; the other has been RETIRED, and
/// the retirement is asserted here rather than deleted:
///
/// - a **torus** wall is INSIDE the axial kinds. It is a surface of
///   revolution about the body's own axis and its meridian is a circle
///   centred `(R, h_c)`, which the reduction reads exactly as it reads
///   a sphere's circle centred `(0, h_c)`. What the C5 table says about
///   `plane × torus` (it now routes the pair's exact-degenerate closed
///   forms, since VERBS-C5ARMS) is beside the point at this door: it
///   does not call the table, and a full revolve's rim is a LATITUDE
///   circle whose position the corner solves give. `torax_axial`
///   carries the closed forms;
/// - a **tangent** junction has no transversal corner to solve, and the
///   conditioning meter says so in the geometry's own terms. This is
///   the tangent bullet's differential, and it now refuses at a door
///   NAMED for what is wrong with it rather than at the mapped-carrier
///   lane it used to reach first.
#[test]
fn the_axial_door_names_its_own_boundary() {
    let tol = Tol::witness();
    let (r, h) = (3.0 / 64.0, 8.0 / 64.0);

    // A belly bulged the other way puts the arc's centre OFF the axis,
    // and the revolve mints a torus.
    let c = p2(0.0, h / 2.0);
    let torus_vase = revolved(
        RawLoop::new(vec![
            ProfileVertex::new(p2(0.0, 0.0), 0.0),
            ProfileVertex::new(p2(r, 0.0), -bulge(p2(r, 0.0), p2(r, h), c)),
            ProfileVertex::new(p2(r, h), 0.0),
            ProfileVertex::new(p2(0.0, h), 0.0),
        ]),
        Revolution::Full,
    );
    let hollow =
        topo::shell(&torus_vase, T, FIT_TOL, tol).expect("a torus wall is inside the axial kinds");
    assert_eq!(
        topo::validate_geometric(&hollow, tol),
        Ok(()),
        "the torus vase's hollow: tier 3"
    );
    assert_eq!(hollow.shells().count(), 2, "outer + cavity");
    println!("[axial] the torus belly hollows through the axial door");

    // A hemisphere TANGENT to its cylinder: same pair as the bellied
    // pot's foot junction, differing only in the angle between them.
    let dome = revolved(
        <ProfileLoop<f64> as RawLoop<f64>>::new(vec![
            ProfileVertex::new(p2(0.0, 0.0), 0.0),
            ProfileVertex::new(p2(r, 0.0), 0.0),
            ProfileVertex::new(p2(r, h), (core::f64::consts::FRAC_PI_2 / 4.0).tan()),
            ProfileVertex::new(p2(0.0, h + r), 0.0),
        ])
        .with_tangent_joints(vec![2]),
        Revolution::Full,
    );
    let e = topo::shell(&dome, T, FIT_TOL, tol)
        .expect_err("a tangent junction has no transversal corner");
    let ShellError::Face { error, .. } = e else {
        panic!("not the offset door's refusal: {e}");
    };
    let topo::ReplaceFaceError::TogetherAxialCorner { what, surfaces, .. } = *error else {
        panic!("the tangent bullet must refuse at the corner it is about: {error}");
    };
    println!("[axial] the tangent bullet: {surfaces} surfaces — {what}");
    assert!(
        what.contains("tangent"),
        "the refusal must say what is wrong, got {what}"
    );
}
