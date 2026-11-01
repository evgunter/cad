//! **Review probe (PR 6221, second pass, frozen at `af002cec1`)**: the
//! join's seam fallback (`boolean::join`'s `seam_germ_side`) at curved
//! chords, against closed-form volume and a `point_in_solid` lattice.
//! Every op that builds is held to its closed form and lattice; a
//! refusal is recorded, never asserted. The rows fail only on a body
//! built wrong.
//!
//! Measured at ε 1e-6, 1e-9 and 1e-12 on the PR head: 254 ops, none
//! built wrong. Against main `c0faf181` (ε 1e-9): none built wrong
//! either; 59 ops refused on main build right on the head (the centred
//! pip, four wells, the tipped ball's quarters and caps), and none
//! moves the other way. Of these poses, only the centred pip and the
//! wells (as the PR's, wide in `y`, narrow, lower, `x`-poled) reach the
//! curved branch (instrumented); the tori, the overhanging pips and the
//! grazes are decided, or refused (`CurvedPierce*`, the interior-loop
//! guard), before it.
#![allow(clippy::unwrap_used, clippy::expect_used, clippy::panic)]

use crate::common::membership::{self, Region, ball, cuboid};
use core::f64::consts::PI;
use geom_core::{Point2, Point3, Tol, Vec2};
use sweep::Revolution;
use sweep::test_support::{finished, revolved_about_y};
use topo::{AtRestBody, BooleanError, BooleanResult, intersect, subtract, union};

fn tol() -> Tol {
    Tol::witness()
}

const BIG: f64 = 1.0;
const SMALL: f64 = 0.3;

fn torus() -> AtRestBody<f64> {
    let profile = profile::Profile::new(
        profile::SketchPlane::<f64>::xy(),
        vec![profile::test_support::bulge_loop(vec![
            (Point2::new(BIG - SMALL, 0.0), 1.0),
            (Point2::new(BIG + SMALL, 0.0), 1.0),
        ])],
    )
    .validate(tol())
    .unwrap();
    let axis = sweep::RevolveAxis {
        origin: Point2::new(0.0, 0.0),
        dir: Vec2::new(0.0, 1.0),
    };
    let body = sweep::revolve(&profile, axis, Revolution::Full, tol())
        .unwrap()
        .body;
    finished("the torus", body, tol())
}

fn the_torus() -> Region {
    Box::new(|p: Point3<f64>| SMALL - ((p.x.hypot(p.z) - BIG).hypot(p.y)))
}

fn plain_ball() -> AtRestBody<f64> {
    let t = (PI / 8.0).tan();
    finished(
        "the ball",
        revolved_about_y(
            vec![
                (Point2::new(0.0, -1.0), t),
                (Point2::new(1.0, 0.0), t),
                (Point2::new(0.0, 1.0), 0.0),
            ],
            Revolution::Full,
            tol(),
        ),
        tol(),
    )
}

/// Outcome of one op: Ok(()) when it built right, Err(text) otherwise.
fn run(
    what: &str,
    r: Result<BooleanResult<f64>, BooleanError>,
    want: f64,
    region: &dyn Fn(Point3<f64>) -> f64,
    lattice: (Point3<f64>, Point3<f64>),
) -> String {
    match r {
        Ok(BooleanResult::Body(b)) => {
            let what2 = what.to_string();
            let body = b.body;
            let res = std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| {
                membership::holds_short_of_tier_3_prime(&what2, &body, want, region, lattice, 9)
            }));
            match res {
                Ok(()) => format!("{what}: BUILT OK"),
                Err(e) => {
                    let msg = e
                        .downcast_ref::<String>()
                        .cloned()
                        .or_else(|| e.downcast_ref::<&str>().map(|s| s.to_string()))
                        .unwrap_or_default();
                    format!("{what}: BUILT WRONG {msg}")
                }
            }
        }
        Ok(BooleanResult::Empty) => {
            if want.abs() < 1e-9 {
                format!("{what}: EMPTY OK")
            } else {
                format!("{what}: EMPTY WRONG (want {want})")
            }
        }
        other => format!("{what}: REFUSED {other:?}"),
    }
}

/// All six ops of `a` against a box, reporting each.
fn six(
    name: &str,
    a: &AtRestBody<f64>,
    va: f64,
    ra: fn() -> Region,
    bx: ((f64, f64), (f64, f64), (f64, f64)),
    share: f64,
    lattice: (Point3<f64>, Point3<f64>),
) -> Vec<String> {
    let (x, y, z) = bx;
    let tool = finished("a box", topo::test_support::brick(x, y, z, tol()), tol());
    let vb = (x.1 - x.0) * (y.1 - y.0) * (z.1 - z.0);
    let rb = move || cuboid(x, y, z);
    let mut out = Vec::new();
    let both = |p: Point3<f64>| ra()(p).min(rb()(p));
    let uni = |p: Point3<f64>| ra()(p).max(rb()(p));
    let a_less = |p: Point3<f64>| ra()(p).min(-rb()(p));
    let b_less = |p: Point3<f64>| rb()(p).min(-ra()(p));
    out.push(run(
        &format!("{name} a∩b"),
        intersect(a, &tool, tol()),
        share,
        &both,
        lattice,
    ));
    out.push(run(
        &format!("{name} b∩a"),
        intersect(&tool, a, tol()),
        share,
        &both,
        lattice,
    ));
    out.push(run(
        &format!("{name} a∪b"),
        union(a, &tool, tol()),
        va + vb - share,
        &uni,
        lattice,
    ));
    out.push(run(
        &format!("{name} b∪a"),
        union(&tool, a, tol()),
        va + vb - share,
        &uni,
        lattice,
    ));
    out.push(run(
        &format!("{name} a∖b"),
        subtract(a, &tool, tol()),
        va - share,
        &a_less,
        lattice,
    ));
    out.push(run(
        &format!("{name} b∖a"),
        subtract(&tool, a, tol()),
        vb - share,
        &b_less,
        lattice,
    ));
    out
}

fn report(rows: Vec<String>) {
    let mut wrong = Vec::new();
    for r in &rows {
        println!("RV2 {r}");
        if r.contains("WRONG") {
            wrong.push(r.clone());
        }
    }
    assert!(wrong.is_empty(), "wrong builds: {wrong:#?}");
}

#[test]
fn rv2_torus_cut_through_its_seam_planes() {
    let t = torus();
    let vt = 2.0 * PI * PI * BIG * SMALL * SMALL;
    let lat = (Point3::new(-1.4, -0.4, -1.4), Point3::new(1.4, 0.4, 1.4));
    let mut rows = Vec::new();
    for (name, bx, share) in [
        ("torus y>0", ((-2.0, 2.0), (0.0, 2.0), (-2.0, 2.0)), 0.5),
        ("torus y<0", ((-2.0, 2.0), (-2.0, 0.0), (-2.0, 2.0)), 0.5),
        ("torus z>0", ((-2.0, 2.0), (-2.0, 2.0), (0.0, 2.0)), 0.5),
        ("torus z<0", ((-2.0, 2.0), (-2.0, 2.0), (-2.0, 0.0)), 0.5),
        ("torus y>0 z>0", ((-2.0, 2.0), (0.0, 2.0), (0.0, 2.0)), 0.25),
        (
            "torus y<0 z<0",
            ((-2.0, 2.0), (-2.0, 0.0), (-2.0, 0.0)),
            0.25,
        ),
        ("torus x>0 z>0", ((0.0, 2.0), (-2.0, 2.0), (0.0, 2.0)), 0.25),
    ] {
        rows.extend(six(name, &t, vt, the_torus, bx, share * vt, lat));
    }
    report(rows);
}

fn cap(a: f64) -> f64 {
    PI * (1.0 - a) * (1.0 - a) * (2.0 + a) / 3.0
}

fn the_ball() -> Region {
    ball(Point3::origin(), 1.0)
}

#[test]
fn rv2_ball_cut_by_boxes_holding_part_of_a_meridian() {
    let b = plain_ball();
    let vb = 4.0 * PI / 3.0;
    let lat = (
        Point3::new(-1.15, -1.15, -1.15),
        Point3::new(1.15, 1.15, 1.15),
    );
    let mut rows = Vec::new();
    for (name, bx, share) in [
        (
            "ball y>0 z>0",
            ((-2.0, 2.0), (0.0, 2.0), (0.0, 2.0)),
            vb / 4.0,
        ),
        (
            "ball y>-0.6 z>0",
            ((-2.0, 2.0), (-0.6, 2.0), (0.0, 2.0)),
            cap(-0.6) / 2.0,
        ),
        (
            "ball y>0.4 z>0",
            ((-2.0, 2.0), (0.4, 2.0), (0.0, 2.0)),
            cap(0.4) / 2.0,
        ),
        (
            "ball y>0.4 z<0",
            ((-2.0, 2.0), (0.4, 2.0), (-2.0, 0.0)),
            cap(0.4) / 2.0,
        ),
        (
            "ball z>0 (graze S on edge)",
            ((-2.0, 2.0), (-1.0, 2.0), (0.0, 2.0)),
            vb / 2.0,
        ),
        (
            "ball x>0 (graze S on edge)",
            ((0.0, 2.0), (-1.0, 2.0), (-2.0, 2.0)),
            vb / 2.0,
        ),
    ] {
        rows.extend(six(name, &b, vb, the_ball, bx, share, lat));
    }
    report(rows);
}

/// The tipped ball (one wrap edge pole to pole in `z = 0`), as the PR's
/// row builds it.
fn tipped_ball() -> AtRestBody<f64> {
    use geom_brep::EdgeDescriptionSpec;
    use topo::construction::Construct;
    let t = (PI / 8.0).tan();
    let mut body = revolved_about_y(
        vec![
            (Point2::new(0.0, -1.0), t),
            (Point2::new(1.0, 0.0), t),
            (Point2::new(0.0, 1.0), 0.0),
        ],
        Revolution::Full,
        tol(),
    );
    let north = crate::common::wrap_tip::vertex_at(&body, crate::common::wrap_tip::NORTH);
    let out = crate::common::wrap_tip::leaving(&body, north);
    let wraps = body.get_half_edge(out[0]).unwrap().edge;
    let surface = body
        .get_face(body.face_of_half_edge(out[0]).unwrap())
        .unwrap()
        .surface;
    body.kef_describing(
        out[1],
        &[(wraps, EdgeDescriptionSpec::wrap(surface))],
        tol(),
    )
    .unwrap();
    finished("the tipped ball", body, tol())
}

#[test]
fn rv2_tipped_ball_cut_by_boxes_holding_part_of_its_wrap_edge() {
    let b = tipped_ball();
    let vb = 4.0 * PI / 3.0;
    let lat = (
        Point3::new(-1.15, -1.15, -1.15),
        Point3::new(1.15, 1.15, 1.15),
    );
    let mut rows = Vec::new();
    for (name, bx, share) in [
        (
            "tball y>0 z>0",
            ((-2.0, 2.0), (0.0, 2.0), (0.0, 2.0)),
            vb / 4.0,
        ),
        (
            "tball y<0 z<0",
            ((-2.0, 2.0), (-2.0, 0.0), (-2.0, 0.0)),
            vb / 4.0,
        ),
        (
            "tball y>-0.6 z>0",
            ((-2.0, 2.0), (-0.6, 2.0), (0.0, 2.0)),
            cap(-0.6) / 2.0,
        ),
        (
            "tball y>0.4 z>0",
            ((-2.0, 2.0), (0.4, 2.0), (0.0, 2.0)),
            cap(0.4) / 2.0,
        ),
        (
            "tball z>0 (graze tip on edge)",
            ((-2.0, 2.0), (-1.0, 2.0), (0.0, 2.0)),
            vb / 2.0,
        ),
        (
            "tball z>0 both tips grazed",
            ((-2.0, 2.0), (-1.0, 1.0), (0.0, 2.0)),
            vb / 2.0,
        ),
    ] {
        rows.extend(six(name, &b, vb, the_ball, bx, share, lat));
    }
    report(rows);
}

fn cylinder() -> AtRestBody<f64> {
    finished(
        "the cylinder",
        revolved_about_y(
            vec![
                (Point2::new(0.0, 0.0), 0.0),
                (Point2::new(0.5, 0.0), 0.0),
                (Point2::new(0.5, 1.0), 0.0),
                (Point2::new(0.0, 1.0), 0.0),
            ],
            Revolution::Full,
            tol(),
        ),
        tol(),
    )
}

fn the_cylinder() -> Region {
    Box::new(|p: Point3<f64>| (0.5 - p.x.hypot(p.z)).min(p.y).min(1.0 - p.y))
}

#[test]
fn rv2_cylinder_grazed_along_its_seam_ruling() {
    let c = cylinder();
    let vc = PI / 4.0;
    let lat = (
        Point3::new(-0.55, -0.55, -1.05),
        Point3::new(1.55, 1.55, 1.05),
    );
    let mut rows = Vec::new();
    for (name, bx) in [
        (
            "cyl seam graze past rims",
            ((0.5, 1.5), (-0.5, 1.5), (-1.0, 1.0)),
        ),
        (
            "cyl seam graze ends inside",
            ((0.5, 1.5), (0.25, 0.75), (-1.0, 1.0)),
        ),
        (
            "cyl seam graze box edge on seam",
            ((0.5, 1.5), (-0.5, 1.5), (0.0, 1.0)),
        ),
        (
            "cyl seam graze flush caps",
            ((0.5, 1.5), (0.0, 1.0), (-1.0, 1.0)),
        ),
    ] {
        rows.extend(six(name, &c, vc, the_cylinder, bx, 0.0, lat));
    }
    report(rows);
}

/// The `y`-poled ball of radius `r` about `c`, its one seam meridian in
/// the plane `z = c.z`, on `x > c.x` (`tilted_sphere_pair`'s `ball`).
fn pip(r: f64, c: geom_core::Vec3<f64>) -> AtRestBody<f64> {
    let b = revolved_about_y(
        vec![(Point2::new(0.0, -r), 1.0), (Point2::new(0.0, r), 0.0)],
        Revolution::Full,
        tol(),
    );
    let b =
        topo::transform_rigid(&b, &geom_core::RigidMotion::translation(c).unwrap(), tol()).unwrap();
    finished("the pip", b, tol())
}

fn cap_of(r: f64, h: f64) -> f64 {
    PI * h * h * (3.0 * r - h) / 3.0
}

/// Pips whose seam great circle lies in the cube's top plane, at poses
/// off the PR's centred one: overhanging an edge either side, on an
/// edge, at a corner.
#[test]
fn rv2_pips_with_their_seam_in_the_top_plane() {
    use geom_core::Vec3;
    let lat = (
        Point3::new(-0.45, -0.45, 0.45),
        Point3::new(1.45, 1.45, 1.45),
    );
    let mut rows = Vec::new();
    let r = 0.3;
    let vball = 4.0 * PI * r * r * r / 3.0;
    for (name, c, share) in [
        ("pip centred r.3", Vec3::new(0.5, 0.5, 1.0), vball / 2.0),
        (
            "pip over x=1 (seam side)",
            Vec3::new(0.8, 0.5, 1.0),
            vball / 2.0 - cap_of(r, 0.1) / 2.0,
        ),
        (
            "pip over x=0 (section side)",
            Vec3::new(0.2, 0.5, 1.0),
            vball / 2.0 - cap_of(r, 0.1) / 2.0,
        ),
        (
            "pip over y=1",
            Vec3::new(0.5, 0.8, 1.0),
            vball / 2.0 - cap_of(r, 0.1) / 2.0,
        ),
        ("pip on edge x=1", Vec3::new(1.0, 0.5, 1.0), vball / 4.0),
        ("pip on edge x=0", Vec3::new(0.0, 0.5, 1.0), vball / 4.0),
        ("pip at corner", Vec3::new(1.0, 1.0, 1.0), vball / 8.0),
    ] {
        eprintln!("RVPOSE {name}");
        let p = pip(r, c);
        let cube = finished(
            "the cube",
            sweep::test_support::cube::<f64>(1.0, tol()),
            tol(),
        );
        let rp = move || ball(Point3::new(c.x, c.y, c.z), r);
        let rc = || cuboid((0.0, 1.0), (0.0, 1.0), (0.0, 1.0));
        let both = |q: Point3<f64>| rp()(q).min(rc()(q));
        let uni = |q: Point3<f64>| rp()(q).max(rc()(q));
        let c_less = |q: Point3<f64>| rc()(q).min(-rp()(q));
        let p_less = |q: Point3<f64>| rp()(q).min(-rc()(q));
        rows.push(run(
            &format!("{name} c∩p"),
            intersect(&cube, &p, tol()),
            share,
            &both,
            lat,
        ));
        rows.push(run(
            &format!("{name} p∩c"),
            intersect(&p, &cube, tol()),
            share,
            &both,
            lat,
        ));
        rows.push(run(
            &format!("{name} c∪p"),
            union(&cube, &p, tol()),
            1.0 + vball - share,
            &uni,
            lat,
        ));
        rows.push(run(
            &format!("{name} c∖p"),
            subtract(&cube, &p, tol()),
            1.0 - share,
            &c_less,
            lat,
        ));
        rows.push(run(
            &format!("{name} p∖c"),
            subtract(&p, &cube, tol()),
            vball - share,
            &p_less,
            lat,
        ));
    }
    report(rows);
}

fn cyl_y(r: f64, y: (f64, f64)) -> AtRestBody<f64> {
    finished(
        "the cylinder",
        revolved_about_y(
            vec![
                (Point2::new(0.0, y.0), 0.0),
                (Point2::new(r, y.0), 0.0),
                (Point2::new(r, y.1), 0.0),
                (Point2::new(0.0, y.1), 0.0),
            ],
            Revolution::Full,
            tol(),
        ),
        tol(),
    )
}

/// The unit ball inside the coaxial unit cylinder: the wall grazes the
/// ball along its equator edge.
#[test]
fn rv2_ball_grazed_by_a_coaxial_cylinder_along_its_equator() {
    let b = plain_ball();
    let vb = 4.0 * PI / 3.0;
    let lat = (
        Point3::new(-1.15, -2.15, -1.15),
        Point3::new(1.15, 2.15, 1.15),
    );
    let mut rows = Vec::new();
    for (name, y, share) in [
        ("cyl past both poles", (-2.0, 2.0), vb),
        ("cyl cap through the equator", (0.0, 2.0), vb / 2.0),
        ("cyl cap at y=-0.5", (-0.5, 2.0), vb - cap_of(1.0, 0.5)),
    ] {
        let c = cyl_y(1.0, y);
        let vc = PI * (y.1 - y.0);
        let rc = move || -> Region {
            Box::new(move |p: Point3<f64>| (1.0 - p.x.hypot(p.z)).min(p.y - y.0).min(y.1 - p.y))
        };
        let rb = the_ball;
        let both = |q: Point3<f64>| rb()(q).min(rc()(q));
        let uni = |q: Point3<f64>| rb()(q).max(rc()(q));
        let b_less = |q: Point3<f64>| rb()(q).min(-rc()(q));
        let c_less = |q: Point3<f64>| rc()(q).min(-rb()(q));
        rows.push(run(
            &format!("{name} b∩c"),
            intersect(&b, &c, tol()),
            share,
            &both,
            lat,
        ));
        rows.push(run(
            &format!("{name} c∩b"),
            intersect(&c, &b, tol()),
            share,
            &both,
            lat,
        ));
        rows.push(run(
            &format!("{name} b∪c"),
            union(&b, &c, tol()),
            vb + vc - share,
            &uni,
            lat,
        ));
        rows.push(run(
            &format!("{name} b∖c"),
            subtract(&b, &c, tol()),
            vb - share,
            &b_less,
            lat,
        ));
        rows.push(run(
            &format!("{name} c∖b"),
            subtract(&c, &b, tol()),
            vc - share,
            &c_less,
            lat,
        ));
    }
    report(rows);
}

type Bounds = [(f64, f64); 3];

fn bvol(b: Bounds) -> f64 {
    b.iter().map(|(lo, hi)| hi - lo).product()
}

/// The PR's ring-in-a-sphere-island well (`a_ring_on_a_sphere_face`),
/// at poses off its one: walls near grazing either side, a narrow well,
/// a lower ball, and the ball poled along `z` and `x`.
#[test]
fn rv2_wells_under_a_ball_dipping_into_a_slab() {
    use geom_core::Vec3;
    let slab_b: Bounds = [(0.0, 4.0), (0.0, 4.0), (0.0, 1.0)];
    let mut rows = Vec::new();
    for (name, wx, wy, c, pole) in [
        (
            "well as PR",
            (0.1, 0.4),
            (1.8, 2.2),
            Vec3::new(0.3, 2.0, 1.2),
            'y',
        ),
        (
            "well wall -x near graze",
            (-0.15, 0.4),
            (1.8, 2.2),
            Vec3::new(0.3, 2.0, 1.2),
            'y',
        ),
        (
            "well wall +x near graze",
            (0.1, 0.72),
            (1.8, 2.2),
            Vec3::new(0.3, 2.0, 1.2),
            'y',
        ),
        (
            "well wide in y",
            (0.1, 0.4),
            (1.6, 2.4),
            Vec3::new(0.3, 2.0, 1.2),
            'y',
        ),
        (
            "narrow well",
            (0.25, 0.35),
            (1.95, 2.05),
            Vec3::new(0.3, 2.0, 1.2),
            'y',
        ),
        (
            "lower ball",
            (0.1, 0.4),
            (1.8, 2.2),
            Vec3::new(0.3, 2.0, 1.05),
            'y',
        ),
        (
            "z-poled",
            (0.1, 0.4),
            (1.8, 2.2),
            Vec3::new(0.3, 2.0, 1.2),
            'z',
        ),
        (
            "x-poled",
            (0.1, 0.4),
            (1.8, 2.2),
            Vec3::new(0.3, 2.0, 1.2),
            'x',
        ),
        (
            "z-poled off centre",
            (0.0, 0.45),
            (1.75, 2.15),
            Vec3::new(0.3, 2.0, 1.2),
            'z',
        ),
    ] {
        let r = 0.5;
        let well_b: Bounds = [wx, wy, (0.5, 2.0)];
        // The well's part inside the slab (a well may open through a side).
        let in_slab: Bounds = [(wx.0.max(0.0), wx.1), wy, (0.5, 1.0)];
        let tolw = tol();
        let slab = match topo::subtract(
            &finished(
                "slab",
                topo::test_support::brick(slab_b[0], slab_b[1], slab_b[2], tolw),
                tolw,
            ),
            &finished(
                "well",
                topo::test_support::brick(wx, wy, (0.5, 2.0), tolw),
                tolw,
            ),
            tolw,
        ) {
            Ok(BooleanResult::Body(b)) => b.body,
            other => {
                rows.push(format!("{name}: slab REFUSED {other:?}"));
                continue;
            }
        };
        let raw = match pole {
            'y' => sweep::test_support::ball_poled_y(r, Vec3::new(c.x, c.y, c.z), tolw),
            'z' => sweep::test_support::ball_poled_z(r, Vec3::new(c.x, c.y, c.z), tolw),
            _ => sweep::test_support::ball_poled(
                r,
                Vec3::new(c.x, c.y, c.z),
                Vec3::new(1.0, 0.0, 0.0),
                tolw,
            ),
        };
        let b = finished("the ball", raw, tolw);
        let va = bvol(slab_b) - bvol(in_slab);
        let vb = 4.0 * PI * r * r * r / 3.0;
        let cc = (c.x, c.y, c.z);
        let shared = crate::common::oracles::ball_in_box(r, cc, slab_b)
            - crate::common::oracles::ball_in_box(r, cc, in_slab);
        let solid = move || -> Region {
            let (s, w) = (
                cuboid(slab_b[0], slab_b[1], slab_b[2]),
                cuboid(well_b[0], well_b[1], well_b[2]),
            );
            Box::new(move |p| s(p).min(-w(p)))
        };
        let sph = move || ball(Point3::new(c.x, c.y, c.z), r);
        let lat = (
            Point3::new(c.x - 0.6, c.y - 0.6, c.z - 0.6),
            Point3::new(c.x + 0.6, c.y + 0.6, c.z + 0.6),
        );
        let both = |p: Point3<f64>| solid()(p).min(sph()(p));
        let uni = |p: Point3<f64>| solid()(p).max(sph()(p));
        let s_less = |p: Point3<f64>| solid()(p).min(-sph()(p));
        let b_less = |p: Point3<f64>| sph()(p).min(-solid()(p));
        eprintln!("RVPOSE {name}");
        rows.push(run(
            &format!("{name} s∩b"),
            intersect(&slab, &b, tol()),
            shared,
            &both,
            lat,
        ));
        rows.push(run(
            &format!("{name} b∩s"),
            intersect(&b, &slab, tol()),
            shared,
            &both,
            lat,
        ));
        rows.push(run(
            &format!("{name} s∪b"),
            union(&slab, &b, tol()),
            va + vb - shared,
            &uni,
            lat,
        ));
        rows.push(run(
            &format!("{name} b∪s"),
            union(&b, &slab, tol()),
            va + vb - shared,
            &uni,
            lat,
        ));
        rows.push(run(
            &format!("{name} s∖b"),
            subtract(&slab, &b, tol()),
            va - shared,
            &s_less,
            lat,
        ));
        rows.push(run(
            &format!("{name} b∖s"),
            subtract(&b, &slab, tol()),
            vb - shared,
            &b_less,
            lat,
        ));
    }
    report(rows);
}

/// The die pip's shape (`m5_pr9c`): the slab `z ∈ [0, h]` against the
/// unit ball whose seam great circle lies in `z = 0`, at `h = 1` (the
/// top tangent at a point, the pinned refusal) and off it.
#[test]
fn rv2_die_pip_slab_heights() {
    let b = plain_ball();
    let vb = 4.0 * PI / 3.0;
    let mut rows = Vec::new();
    for h in [1.0, 1.3, 0.7] {
        let name = format!("die slab h={h}");
        let sl = finished(
            "slab",
            topo::test_support::brick((-2.0, 2.0), (-2.0, 2.0), (0.0, h), tol()),
            tol(),
        );
        let vs = 16.0 * h;
        let hh = h.min(1.0);
        let share = PI * (hh - hh * hh * hh / 3.0);
        let lat = (Point3::new(-1.3, -1.3, -1.3), Point3::new(1.3, 1.3, 1.3));
        let rs = move || cuboid((-2.0, 2.0), (-2.0, 2.0), (0.0, h));
        let both = |q: Point3<f64>| the_ball()(q).min(rs()(q));
        let uni = |q: Point3<f64>| the_ball()(q).max(rs()(q));
        let s_less = |q: Point3<f64>| rs()(q).min(-the_ball()(q));
        let b_less = |q: Point3<f64>| the_ball()(q).min(-rs()(q));
        rows.push(run(
            &format!("{name} s∖b"),
            subtract(&sl, &b, tol()),
            vs - share,
            &s_less,
            lat,
        ));
        rows.push(run(
            &format!("{name} b∖s"),
            subtract(&b, &sl, tol()),
            vb - share,
            &b_less,
            lat,
        ));
        rows.push(run(
            &format!("{name} s∩b"),
            intersect(&sl, &b, tol()),
            share,
            &both,
            lat,
        ));
        rows.push(run(
            &format!("{name} s∪b"),
            union(&sl, &b, tol()),
            vs + vb - share,
            &uni,
            lat,
        ));
    }
    report(rows);
}
