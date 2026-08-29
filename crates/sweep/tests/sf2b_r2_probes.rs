//! R2 review probes for VERBS-SHELLFIX PR-2b (#1180). Print-first,
//! assert where the answer is already known. Lane-private.

#![allow(clippy::unwrap_used, clippy::expect_used, clippy::panic, dead_code)]

use core::f64::consts::PI;

use geom::Surface;
use geom_core::{Band, Point2, Tol, Vec2};
use profile::{Profile, ProfileLoop, ProfileVertex, RawLoop, SketchPlane};
use sweep::{Revolution, RevolveAxis, revolve};
use topo::Body;

fn p2(x: f64, y: f64) -> Point2<f64> {
    Point2::new(x, y)
}

fn band() -> Band {
    Band::linear(Tol::witness()).unwrap()
}

const FIT_TOL: f64 = 1e-6;
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

fn frustum(r0: f64, r1: f64, h: f64) -> Body<f64> {
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

fn frustum_volume(r0: f64, r1: f64, h: f64) -> f64 {
    PI * h * (r0 * r0 + r0 * r1 + r1 * r1) / 3.0
}

/// The wall's closed form for a frustum hollowed by `T`: the cavity's
/// cone is the operand's offset perpendicular by the wall.
fn frustum_wall(r0: f64, r1: f64, h: f64) -> f64 {
    let tan_a = (r0 - r1).abs() / h;
    let alpha = tan_a.atan();
    // Distance from the axis at each moved cap station, on the inward
    // offset cone: measure the operand's radius at that station and
    // pull in by t / cos alpha.
    let at = |y: f64| r0 + (r1 - r0) * y / h;
    let c0 = at(T) - T / alpha.cos();
    let c1 = at(h - T) - T / alpha.cos();
    frustum_volume(r0, r1, h) - frustum_volume(c0, c1, h - 2.0 * T)
}

fn cone_faces(body: &Body<f64>) -> Vec<topo::FaceKey> {
    body.faces()
        .filter(|(_, f)| matches!(body.get_surface(f.surface), Some(Surface::Cone { .. })))
        .map(|(k, _)| k)
        .collect()
}

fn describe_cones(what: &str, body: &Body<f64>) {
    for (k, f) in body.faces() {
        if let Some(Surface::Cone {
            apex,
            axis,
            half_angle,
            ..
        }) = body.get_surface(f.surface)
        {
            // Which nappe: the sum of the face's own corner stations.
            println!(
                "[r2] {what}: {k:?} sense={} apex={:?} axis={:?} alpha={half_angle}",
                f.sense, apex, axis
            );
        }
    }
}

/// **BOTH cone nappes, through `shell`.** `sf2b_axial.rs` pins ONE
/// orientation (a frustum NARROWING upward, whose wall sits below its
/// apex). This row adds the mirror: a frustum WIDENING upward, whose
/// wall sits above its apex. If `nappe_signed`'s sign were resolved the
/// other way round, exactly one of these two would be wrong and the
/// shipped acceptance row would not notice.
#[test]
fn r2_both_cone_nappes_hollow_to_their_closed_forms() {
    let tol = Tol::witness();
    let h = 8.0 / 64.0;
    for (what, r0, r1) in [
        (
            "narrowing upward (wall BELOW its apex)",
            4.0 / 64.0,
            2.0 / 64.0,
        ),
        (
            "widening upward (wall ABOVE its apex)",
            2.0 / 64.0,
            4.0 / 64.0,
        ),
    ] {
        let body = frustum(r0, r1, h);
        describe_cones(what, &body);
        let v_out = topo::mass_properties(&body, tol).expect("props").volume;
        match topo::shell(&body, T, FIT_TOL, band(), tol) {
            Ok(hollow) => {
                assert_eq!(
                    topo::validate_geometric(&hollow, tol),
                    Ok(()),
                    "{what}: tier 3"
                );
                let v = topo::mass_properties(&hollow, tol).expect("props").volume;
                let want = frustum_wall(r0, r1, h);
                println!(
                    "[r2] {what}: operand {v_out}, wall {v} want {want} delta {}",
                    v - want
                );
                assert!(
                    v < v_out,
                    "{what}: a WALL cannot exceed the operand it was cut from ({v} vs {v_out})"
                );
                assert!(
                    (v - want).abs() <= 1e-12,
                    "{what}: the wall's closed form is {want}, got {v}"
                );
            }
            Err(e) => println!("[r2] {what}: REFUSED {e}"),
        }
    }
}

/// **The unfixed sibling: the per-chart door on a mirror-nappe cone.**
/// `offset_axial` resolves the nappe at its own door; `replace_face`'s
/// `mint_offset` does not, and `shell`'s `inward()` derives the sign
/// from the face's SENSE. This row asks the public single-chart verb to
/// pull the cone chart INWARD and reports which way it actually went.
#[test]
fn r2_per_chart_door_on_a_mirror_nappe_cone() {
    let tol = Tol::witness();
    let h = 8.0 / 64.0;
    for (what, r0, r1) in [
        ("narrowing upward", 4.0 / 64.0, 2.0 / 64.0),
        ("widening upward", 2.0 / 64.0, 4.0 / 64.0),
    ] {
        let body = frustum(r0, r1, h);
        let faces = cone_faces(&body);
        let v0 = topo::mass_properties(&body, tol).expect("props").volume;
        for signed in [-T, T] {
            let mut work = body.clone();
            match topo::replace_faces_offset(&mut work, &faces, signed, FIT_TOL, band(), tol) {
                Ok(()) => {
                    let v = topo::mass_properties(&work, tol).expect("props").volume;
                    let valid = topo::validate_geometric(&work, tol);
                    println!(
                        "[r2] per-chart {what} d={signed}: volume {v0} -> {v} ({}), tier3 {:?}",
                        if v > v0 { "GREW" } else { "shrank" },
                        valid.is_ok()
                    );
                }
                Err(e) => println!("[r2] per-chart {what} d={signed}: REFUSED {e}"),
            }
        }
    }
}

/// **A CONICAL wedge: `meridian plane ∩ cone`.** The PR body's edge
/// table claims that posture is an EXACT translation ("direction kept
/// (axis-parallel or generator), re-anchored from the corner — yes").
/// A plane parallel to a cone's axis cuts it in a HYPERBOLA, not a
/// line, so the offset edge is not the old line moved. This row asks
/// what actually happens: a refusal (the midpoint/endpoint meters doing
/// their job) or a body.
#[test]
fn r2_a_conical_wedge_meridian_edge() {
    let tol = Tol::witness();
    let (r0, r1, h) = (4.0 / 64.0, 2.0 / 64.0, 8.0 / 64.0);
    for (what, turn) in [("a quarter turn", PI / 2.0), ("a 1/12 turn", PI / 6.0)] {
        let body = revolved(
            ProfileLoop::new(vec![
                ProfileVertex::new(p2(0.0, 0.0), 0.0),
                ProfileVertex::new(p2(r0, 0.0), 0.0),
                ProfileVertex::new(p2(r1, h), 0.0),
                ProfileVertex::new(p2(0.0, h), 0.0),
            ]),
            Revolution::Partial(turn),
        );
        let v0 = topo::mass_properties(&body, tol).expect("props").volume;
        match topo::shell(&body, T, FIT_TOL, band(), tol) {
            Ok(hollow) => {
                let v = topo::mass_properties(&hollow, tol).expect("props").volume;
                println!(
                    "[r2] conical wedge {what}: HOLLOWS operand {v0} wall {v} tier3 {:?} shells {}",
                    topo::validate_geometric(&hollow, tol).is_ok(),
                    hollow.shells().count()
                );
                assert!(
                    v < v0,
                    "conical wedge {what}: a wall cannot exceed its operand"
                );
            }
            Err(e) => println!("[r2] conical wedge {what}: REFUSED {e}"),
        }
    }
}

/// **The wedge at degenerate turns.** The azimuth solve's two roots are
/// separated by `2 rho sin d`; a wedge whose two meridian caps are
/// nearly parallel (a half turn) or very sharp should refuse rather
/// than build. Print-first: a hollow is checked against the closed form
/// where the geometry has one.
#[test]
fn r2_wedge_at_degenerate_turns() {
    let tol = Tol::witness();
    let (r, h) = (3.0 / 64.0, 8.0 / 64.0);
    let two_chord = |rr: f64, d: f64| {
        let top = (rr * rr - d * d).sqrt();
        let f = |x: f64| x * (rr * rr - x * x).sqrt() / 2.0 + rr * rr * (x / rr).asin() / 2.0;
        f(top) - f(d) - d * (top - d)
    };
    for (what, turn) in [
        ("a 1/64 turn", PI / 32.0),
        ("a 1/12 turn", PI / 6.0),
        ("a quarter turn", PI / 2.0),
        ("a HALF turn (coplanar caps)", PI),
        ("a 3/4 turn (reflex)", 3.0 * PI / 2.0),
    ] {
        let body = revolved(
            ProfileLoop::new(vec![
                ProfileVertex::new(p2(0.0, 0.0), 0.0),
                ProfileVertex::new(p2(r, 0.0), 0.0),
                ProfileVertex::new(p2(r, h), 0.0),
                ProfileVertex::new(p2(0.0, h), 0.0),
            ]),
            Revolution::Partial(turn),
        );
        let v0 = topo::mass_properties(&body, tol).expect("props").volume;
        match topo::shell(&body, T, FIT_TOL, band(), tol) {
            Ok(hollow) => {
                let v = topo::mass_properties(&hollow, tol).expect("props").volume;
                let valid = topo::validate_geometric(&hollow, tol);
                println!(
                    "[r2] wedge {what}: operand {v0}, wall {v}, tier3 {:?}, shells {}",
                    valid.is_ok(),
                    hollow.shells().count()
                );
                assert!(v < v0, "wedge {what}: a wall cannot exceed its operand");
                assert!(valid.is_ok(), "wedge {what}: tier 3");
                if (turn - PI / 2.0).abs() < 1e-12 {
                    let want = PI * r * r * h / 4.0 - two_chord(r - T, T) * (h - 2.0 * T);
                    assert!((v - want).abs() <= 1e-15, "quarter turn: {want} vs {v}");
                }
            }
            Err(e) => println!("[r2] wedge {what}: REFUSED {e}"),
        }
    }
}

/// **The carried azimuth, checked against the moved chart's own seam.**
/// The door pins a full revolve's rim vertex by carrying `e_old`. That
/// is only right if the MOVED chart's seam still stands at that
/// azimuth. Measured: every moved vertex's azimuth against its
/// operand's, on the bellied pot (plane/cylinder/sphere in one body).
#[test]
fn r2_the_carried_azimuth_survives_both_surfaces_moving() {
    let tol = Tol::witness();
    let (r_foot, y_foot, r_neck, y_mouth, y_c, r_belly) = (
        4.0 / 64.0,
        1.0 / 64.0,
        3.0 / 64.0,
        8.0 / 64.0,
        4.0 / 64.0,
        5.0 / 64.0,
    );
    let c = p2(0.0, y_c);
    let (u, v) = (p2(r_foot, y_foot) - c, p2(r_neck, y_mouth) - c);
    let sweep = u.perp_dot(v).atan2(u.dot(v));
    let pot = revolved(
        RawLoop::new(vec![
            ProfileVertex::new(p2(0.0, 0.0), 0.0),
            ProfileVertex::new(p2(r_foot, 0.0), 0.0),
            ProfileVertex::new(p2(r_foot, y_foot), (sweep / 4.0).tan()),
            ProfileVertex::new(p2(r_neck, y_mouth), 0.0),
            ProfileVertex::new(p2(0.0, y_mouth), 0.0),
        ]),
        Revolution::Full,
    );
    println!(
        "[r2] pot residual foot {}",
        r_foot * r_foot + (y_foot - y_c) * (y_foot - y_c) - r_belly * r_belly
    );
    let before: Vec<f64> = pot
        .vertices()
        .filter_map(|(_, vd)| pot.get_point(vd.point).copied())
        .map(|p| p.z.atan2(p.x))
        .collect();
    let hollow = topo::shell(&pot, T, FIT_TOL, band(), tol).expect("the bellied pot hollows");
    let after: Vec<f64> = hollow
        .vertices()
        .filter_map(|(_, vd)| hollow.get_point(vd.point).copied())
        .filter(|p| (p.x * p.x + p.z * p.z).sqrt() > 1e-12)
        .map(|p| p.z.atan2(p.x))
        .collect();
    println!("[r2] azimuths before {before:?}");
    println!("[r2] azimuths after  {after:?}");
    for a in &after {
        assert!(
            a.abs() < 1e-15 || (a.abs() - PI).abs() < 1e-15,
            "a moved rim vertex left its seam meridian: {a}"
        );
    }
    // The mouth opens, and the cup's plug is a zone slab.
    let mouth: Vec<topo::FaceKey> = pot
        .faces()
        .filter(|(_, f)| {
            matches!(pot.get_surface(f.surface),
                Some(Surface::Plane { origin, .. }) if (origin.y - y_mouth).abs() < 1e-15)
        })
        .map(|(k, _)| k)
        .collect();
    match topo::shell_open(&pot, T, &mouth, FIT_TOL, band(), tol) {
        Ok(cup) => {
            let props = topo::mass_properties(&cup, tol).expect("props");
            println!(
                "[r2] the opened cup: V {} shells {} tier3 {:?}",
                props.volume,
                cup.shells().count(),
                topo::validate_geometric(&cup, tol).is_ok()
            );
        }
        Err(e) => println!("[r2] the opened cup REFUSED: {e}"),
    }
}

/// **The stepped-meridian vase's OPENED arm** — the branch the shipped
/// `verbs_shell_r2_probes` row still lets return early. Which is it?
#[test]
fn r2_stepped_vase_lift_branch() {
    let tol = Tol::witness();
    let t = 1.0 / 128.0;
    let body = revolved(
        ProfileLoop::new(vec![
            ProfileVertex::new(p2(0.0, 0.0), 0.0),
            ProfileVertex::new(p2(6.0 / 64.0, 0.0), 0.0),
            ProfileVertex::new(p2(5.0 / 64.0, 4.0 / 64.0), 0.0),
            ProfileVertex::new(p2(3.0 / 64.0, 8.0 / 64.0), 0.0),
            ProfileVertex::new(p2(0.0, 8.0 / 64.0), 0.0),
        ]),
        Revolution::Full,
    );
    match topo::shell(&body, t, FIT_TOL, band(), tol) {
        Ok(_) => println!("[r2] stepped vase SEALED: ok"),
        Err(e) => println!("[r2] stepped vase SEALED: REFUSED {e}"),
    }
    let mouth: Vec<topo::FaceKey> = body
        .faces()
        .filter(|(_, f)| {
            matches!(body.get_surface(f.surface),
                Some(Surface::Plane { origin, .. }) if (origin.y - 8.0 / 64.0).abs() < 1e-15)
        })
        .map(|(k, _)| k)
        .collect();
    match topo::shell_open(&body, t, &mouth, FIT_TOL, band(), tol) {
        Ok(cup) => println!(
            "[r2] stepped vase OPENED: ok, shells {} tier3 {:?}",
            cup.shells().count(),
            topo::validate_geometric(&cup, tol).is_ok()
        ),
        Err(e) => println!("[r2] stepped vase OPENED: REFUSED {e}"),
    }
}

/// **Which BRANCH does each fixture take?** `shell` picks the axial
/// door on `topo::is_axial`, and that predicate swallows an escalated
/// margin as `false` — so a body inside the axial class can be routed
/// to the per-chart door without anything saying so. This row prints
/// the branch for every fixture in the corpus, including the ones that
/// refuse.
#[test]
fn r2_which_branch_each_fixture_takes() {
    let (r, h) = (3.0 / 64.0, 8.0 / 64.0);
    let square = |turn| {
        revolved(
            ProfileLoop::new(vec![
                ProfileVertex::new(p2(0.0, 0.0), 0.0),
                ProfileVertex::new(p2(r, 0.0), 0.0),
                ProfileVertex::new(p2(r, h), 0.0),
                ProfileVertex::new(p2(0.0, h), 0.0),
            ]),
            turn,
        )
    };
    for (what, body) in [
        ("drum (full)", square(Revolution::Full)),
        ("wedge 1/64 turn", square(Revolution::Partial(PI / 32.0))),
        ("wedge 1/12 turn", square(Revolution::Partial(PI / 6.0))),
        ("wedge quarter", square(Revolution::Partial(PI / 2.0))),
        ("wedge half", square(Revolution::Partial(PI))),
        ("frustum narrowing", frustum(4.0 / 64.0, 2.0 / 64.0, h)),
        ("frustum widening", frustum(2.0 / 64.0, 4.0 / 64.0, h)),
    ] {
        println!(
            "[r2] branch {what}: is_axial = {}  ({} faces)",
            topo::is_axial(&body, band()),
            body.faces().count()
        );
    }
}
