//! BLEND unit 4 review probes (r4): the two tour-side selection windows
//! the unit moved, exercised on the SAME geometry inside the kernel's
//! own test binary, and the hazards the unit's PR names, pinned.
//!
//! The tour is a detached cargo root that reaches the kernel through
//! the `pncad` façade, so its rows cannot be run from here; but its two
//! fixtures — the lily lantern of `blend1_r1_wall6_probes.rs` and the
//! teapot's lid of `demos/tour/src/teapot.rs` — are solids of revolution
//! of stated meridians, and `revolve` is the same door on both sides of
//! the façade. Each is rebuilt here from its own constants and the old
//! and new windows are compared on it.
//!
//! **The rebuild is a COPY of the tour's meridian constants**, and a
//! copy across a workspace boundary is exactly the thing this unit's
//! own subject warns about: if a scene re-authors its lantern or its
//! lid, these rows keep measuring the old shape and stay green while
//! saying nothing about the tour. What they pin is the SELECTION
//! question — that a window admits the same arcs on this geometry —
//! which is a property of the numbers, so the copy is the evidence and
//! not an accident of it. The tour's own suites are what pin the tour's
//! fixtures; these rows say what the kernel's binary can say about the
//! same meridians.

#![allow(clippy::unwrap_used, clippy::expect_used, clippy::panic)]

use geom::Curve3;
use geom_core::{Point2, Tol};
use profile::ProfileVertex;
use sweep::Revolution;
use sweep::test_support::{arcs_at, dome, one_edge_rim_at, revolved_about_y, rim_arcs_at};
use topo::{Body, EdgeKey};

fn tol() -> Tol {
    Tol::witness()
}

fn v(x: f64, y: f64, bulge: f64) -> ProfileVertex<f64> {
    ProfileVertex::new(Point2::new(x, y), bulge)
}

/// The bulge of a CCW arc about `c` from `a` to `b`: `tan(θ/4)`.
fn ccw_bulge(c: (f64, f64), a: (f64, f64), b: (f64, f64)) -> f64 {
    let ta = (a.1 - c.1).atan2(a.0 - c.0);
    let tb = (b.1 - c.1).atan2(b.0 - c.0);
    let theta = tb - ta;
    assert!(theta > 0.0, "a CCW arc sweeps forward");
    (theta / 4.0).tan()
}

/// Every circle edge of `body` with `|radius − r| < win`, with no
/// station read and only the two-sided support filter — the merge
/// base's `rims_of_radius` seed scan (`5e-4`, radius only).
fn seeds_radius_only(body: &Body<f64>, r: f64, win: f64) -> Vec<EdgeKey> {
    let face_of = |he| {
        body.get_loop(body.get_half_edge(he).unwrap().parent_loop)
            .unwrap()
            .face
    };
    body.edges()
        .filter_map(|(k, e)| {
            let c = body.get_curve_geom(e.curve)?.certified()?;
            match *c.carrier() {
                Curve3::Circle { radius, .. } if (radius - r).abs() < win => Some(k),
                _ => None,
            }
        })
        .filter(|k| {
            let e = body.get_edge(*k).unwrap();
            face_of(e.he_plus) != face_of(e.he_minus)
        })
        .collect()
}

/// Every circle edge at station `y` and radius `r` to `win`, no support
/// filter at all — the teapot scene's `rim_at` scan.
fn hits_station_radius(body: &Body<f64>, y: f64, r: f64, win: f64) -> Vec<EdgeKey> {
    body.edges()
        .filter_map(|(k, e)| {
            let c = body.get_curve_geom(e.curve)?.certified()?;
            match *c.carrier() {
                Curve3::Circle { center, radius, .. }
                    if (center.y - y).abs() < win && (radius - r).abs() < win =>
                {
                    Some(k)
                }
                _ => None,
            }
        })
        .collect()
}

// --- the lily lantern, from blend1_r1_wall6_probes.rs's constants -----

const GLOBE: f64 = 0.44;
const TOP: f64 = 0.40;
const MOUTH: f64 = 0.36;
const LIP_R: f64 = 0.09;
const LIP_DROP: f64 = 0.16;
const NECK_R: f64 = 0.052;
const NECK_HALF_ANGLE: f64 = 70.0 * core::f64::consts::PI / 180.0;

fn lily_rims() -> [(f64, f64); 4] {
    let r_top = (GLOBE.powi(2) - TOP.powi(2)).sqrt();
    let r_mouth = (GLOBE.powi(2) - MOUTH.powi(2)).sqrt();
    let shoulder = (r_top - NECK_R) / NECK_HALF_ANGLE.tan();
    let t_mouth = shoulder + TOP + MOUTH;
    [
        (NECK_R, 0.0),
        (r_top, shoulder),
        (r_mouth, t_mouth),
        (LIP_R, t_mouth + LIP_DROP),
    ]
}

fn lily_lantern() -> Body<f64> {
    let [(_, _), (r_top, shoulder), (r_mouth, t_mouth), (_, t_end)] = lily_rims();
    let c = (0.0, shoulder + TOP);
    let bulge = ccw_bulge(c, (r_top, shoulder), (r_mouth, t_mouth));
    revolved_about_y(
        vec![
            v(0.0, 0.0, 0.0),
            v(NECK_R, 0.0, 0.0),
            v(r_top, shoulder, bulge),
            v(r_mouth, t_mouth, 0.0),
            v(LIP_R, t_end, 0.0),
            v(0.0, t_end, 0.0),
        ],
        Revolution::Full,
        tol(),
    )
}

/// The lily's four transverse rims are each two arcs at their derived
/// `(radius, station)`, and the merge base's radius-only `5e-4` scan
/// seeds the SAME rim for every one of them: the slack admitted nothing,
/// and the station changed no row's subject.
#[test]
fn the_lily_lanterns_rims_select_the_same_arcs_under_both_windows() {
    let lant = lily_lantern();
    for (name, (r, y)) in ["throat", "shoulder", "mouth", "lip"]
        .into_iter()
        .zip(lily_rims())
    {
        let new_seeds = arcs_at(&lant, r, y);
        let old_seeds = seeds_radius_only(&lant, r, 5e-4);
        assert_eq!(old_seeds, new_seeds, "{name}: the seed set moved");
        assert_eq!(new_seeds.len(), 2, "{name}: seam-split into two arcs");
        let rim = rim_arcs_at(&lant, r, y);
        let mut want = new_seeds.clone();
        want.sort_unstable();
        let mut got = rim.clone();
        got.sort_unstable();
        assert_eq!(got, want, "{name}: the rim is exactly its two seeds");
    }
}

/// The `5e-4` was slack and not a property of the rims: every lily rim
/// still selects at `1e-9`, and NO rim needs more than `1e-9` — the
/// loosest window that still selects each is at the analytic value.
#[test]
fn the_lily_lanterns_rims_are_at_their_analytic_radii_to_1e_9() {
    let lant = lily_lantern();
    for (r, _) in lily_rims() {
        assert_eq!(
            seeds_radius_only(&lant, r, 1e-9).len(),
            2,
            "radius {r} selects at 1e-9"
        );
    }
}

// --- the teapot's lid, from demos/tour/src/teapot.rs's constants -----

const Y_MOUTH: f64 = 8.0 / 64.0;
const LIFT: f64 = 1.0 / 32.0;
const LID_BASE: f64 = Y_MOUTH + LIFT;
const R_FLANGE: f64 = 14.0 / 256.0;
const Y_FLANGE: f64 = LID_BASE + 6.0 / 256.0;
const R_NECK: f64 = 3.0 / 64.0;
const DOME_C: f64 = LID_BASE + 1.0 / 256.0;
const R_KNOB: f64 = 5.0 / 256.0;
const Y_KNOB: f64 = LID_BASE + 13.0 / 256.0;
const Y_TOP: f64 = LID_BASE + 18.0 / 256.0;
const R_VENT: f64 = 1.0 / 256.0;

fn teapot_lid() -> Body<f64> {
    let bulge = ccw_bulge((0.0, DOME_C), (R_NECK, Y_FLANGE), (R_KNOB, Y_KNOB));
    revolved_about_y(
        vec![
            v(R_VENT, LID_BASE, 0.0),
            v(R_FLANGE, LID_BASE, 0.0),
            v(R_NECK, Y_FLANGE, bulge),
            v(R_KNOB, Y_KNOB, 0.0),
            v(R_KNOB, Y_TOP, 0.0),
            v(R_VENT, Y_TOP, 0.0),
        ],
        Revolution::Full,
        tol(),
    )
}

/// The lid's three rims the scene fillets — flange, dome foot, knob top
/// — are ONE closed edge each at `1e-12` and at `1e-9` alike, and the
/// same edge: widening the scene's window moved no selection. The lid's
/// census is the scene's own `(6, 12, 6)`.
#[test]
fn the_teapot_lids_rims_select_the_same_edge_at_1e_12_and_1e_9() {
    let lid = teapot_lid();
    assert_eq!(
        (
            lid.vertices().count(),
            lid.edges().count(),
            lid.faces().count()
        ),
        (6, 12, 6),
        "an annular profile mints one full wall per segment"
    );
    for (name, y, r) in [
        ("flange", LID_BASE, R_FLANGE),
        ("dome foot", Y_FLANGE, R_NECK),
        ("knob top", Y_TOP, R_KNOB),
    ] {
        let tight = hits_station_radius(&lid, y, r, 1e-12);
        let wide = hits_station_radius(&lid, y, r, 1e-9);
        assert_eq!(tight, wide, "{name}: the window moved the selection");
        assert_eq!(wide.len(), 1, "{name}: one rim");
        assert_eq!(
            one_edge_rim_at(&lid, r, y),
            wide[0],
            "{name}: the home selects the scene's edge"
        );
    }
}

// --- the hazards the unit names, pinned on the kernel's own dome ------

/// The dome carries TWO rims of radius `0.5r` — its bore's, at stations
/// `0` and `r·√2/2` — which a radius-only scan cannot tell apart and the
/// homed selector does.
#[test]
fn the_domes_bore_rims_are_two_stations_of_one_radius() {
    let body = dome(1.0, tol());
    let lo = one_edge_rim_at(&body, 0.5, 0.0);
    let hi = one_edge_rim_at(&body, 0.5, core::f64::consts::FRAC_1_SQRT_2);
    assert_ne!(lo, hi, "two rims, two edges");
    assert_eq!(
        seeds_radius_only(&body, 0.5, 1e-9).len(),
        2,
        "radius alone names both"
    );
}

/// A caller holding a key has no way to say "no rim": an empty answer
/// panics rather than returning something.
#[test]
#[should_panic(expected = "is one closed edge")]
fn one_edge_rim_at_panics_on_an_empty_answer() {
    let body = dome(1.0, tol());
    let _ = one_edge_rim_at(&body, 0.5, 0.3);
}

/// The scan reads the sketch's `y` station only: a body whose axis is
/// not `y` cannot be selected through it. Said as a row so the axis
/// premise is pinned where the door is.
#[test]
fn arcs_at_reads_the_y_station_and_nothing_else() {
    let body = dome(1.0, tol());
    let posed = topo::transform_rigid(
        &body,
        &geom_core::Affine3::rotation_about_axis(
            geom_core::Point3::new(0.0, 0.0, 0.0),
            geom_core::Vec3::new(1.0, 0.0, 0.0),
            core::f64::consts::FRAC_PI_2,
        ),
        tol(),
    )
    .unwrap();
    assert_eq!(arcs_at(&body, 1.0, 0.0).len(), 1, "the equator at y = 0");
    assert!(
        arcs_at(&posed, 1.0, 0.0).len() == 1,
        "the equator's centre stays at the origin under a rotation about x"
    );
    assert!(
        arcs_at(&posed, 0.5, core::f64::consts::FRAC_1_SQRT_2).is_empty(),
        "the bore's top rim now sits on the z axis and the y-station scan cannot name it"
    );
}
