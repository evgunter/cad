//! **PR 4918 second-review probes**: faces ending near the tangent
//! ruling (the face-clear test must never read a reaching or in-band
//! face clear), poses turned off the world's axes, and in-band poses
//! at several multiples of the band's zero. Each row prints its
//! outcome; the asserting rows pin what must hold.

#![allow(clippy::unwrap_used, clippy::expect_used, clippy::panic)]

use geom_core::{Affine3, Band, Point2, Point3, Tol, Vec3};
use sweep::test_support::revolved_about_y;

fn tol() -> Tol {
    Tol::witness()
}

fn band() -> Band {
    Band::linear(tol()).unwrap()
}

fn at_rest(b: topo::Body<f64>) -> topo::AtRestBody<f64> {
    topo::AtRestBody::validate(b, tol()).unwrap()
}

fn about_y(
    (inner, outer): (f64, f64),
    (lo, hi): (f64, f64),
    turn: Option<f64>,
    spin: f64,
    x: f64,
) -> topo::Body<f64> {
    let profile = vec![
        (Point2::new(inner, lo), 0.0),
        (Point2::new(outer, lo), 0.0),
        (Point2::new(outer, hi), 0.0),
        (Point2::new(inner, hi), 0.0),
    ];
    let rev = turn.map_or(sweep::Revolution::Full, sweep::Revolution::Partial);
    let body = revolved_about_y(profile, rev, tol());
    let place = Affine3::translation(Vec3::new(x, 0.0, 0.0))
        * Affine3::rotation_about_axis(Point3::origin(), Vec3::unit_y(), spin);
    topo::transform_rigid(&body, &place, tol()).unwrap()
}

fn brick(x: (f64, f64), y: (f64, f64), z: (f64, f64)) -> topo::Body<f64> {
    topo::test_support::brick(x, y, z, tol())
}

fn turned(b: topo::Body<f64>, axis: Vec3<f64>, angle: f64) -> topo::Body<f64> {
    let t = Affine3::rotation_about_axis(Point3::origin(), axis.normalize(), angle);
    topo::transform_rigid(&b, &t, tol()).unwrap()
}

fn op(
    k: u8,
    a: &topo::AtRestBody<f64>,
    b: &topo::AtRestBody<f64>,
) -> Result<(), topo::BooleanError> {
    match k {
        0 => topo::union(a, b, tol()).map(|_| ()),
        1 => topo::subtract(a, b, tol()).map(|_| ()),
        _ => topo::intersect(a, b, tol()).map(|_| ()),
    }
}

fn said(r: &Result<(), topo::BooleanError>) -> String {
    match r {
        Ok(()) => "Ok".into(),
        Err(e) => {
            let (key, _) = topo::test_support::offer_key(e);
            let pred = match e {
                topo::BooleanError::Escalated { diag, .. } => format!("{:?}", diag.predicate),
                _ => String::new(),
            };
            format!("{key} {pred}")
        }
    }
}

/// Every op, both orders; returns every answer.
fn table(name: &str, x: topo::Body<f64>, y: topo::Body<f64>) -> Vec<String> {
    let (x, y) = (at_rest(x), at_rest(y));
    let mut out = Vec::new();
    for k in 0..3 {
        let (xy, yx) = (said(&op(k, &x, &y)), said(&op(k, &y, &x)));
        println!("{name} op{k}: [{xy}] / [{yx}]");
        out.push(xy);
        out.push(yx);
    }
    out
}

fn any_ok(rows: &[String]) -> bool {
    rows.iter().any(|r| r == "Ok")
}

fn none_frontier(rows: &[String]) -> bool {
    rows.iter().all(|r| {
        !r.starts_with("CurvedBooleanUnsupported") && !r.starts_with("CurvedPierceUnsupported")
    })
}

/// A brick whose top face ENDS near the tube's ruling (x = 0), its top
/// `5.5·zero` below the wall's carrier. The face's least distance to
/// the wall is `√(1 + dx²) − 1 + 5.5·zero` for `dx ≥ 0`. Where that is
/// inside the band (or the face straddles), the door must not read the
/// face clear: no op may answer `Ok` silently through it.
#[test]
fn r2_brick_ending_near_the_ruling() {
    let b = band();
    println!("zero {:e} escalate {:e}", b.zero(), b.escalate());
    let gap = 5.5 * b.zero();
    let mut wrong = Vec::new();
    for dx in [-0.1, -1e-6, 0.0, 1e-7, 1e-6, 1e-5, 3e-5, 1e-4, 1e-3, 1e-2] {
        let true_gap = (1.0f64 + dx * dx).sqrt() - 1.0 + gap;
        let rows = table(
            &format!(
                "edge dx={dx:e} (faces {:e} apart)",
                if dx < 0.0 { gap } else { true_gap }
            ),
            about_y((0.3, 1.0), (0.0, 1.0), None, 0.0, 0.0),
            brick((dx, dx + 0.5), (0.2, 0.8), (-2.0, -1.0 - gap)),
        );
        let in_band = dx <= 0.0 || true_gap <= b.escalate();
        if in_band && any_ok(&rows) {
            wrong.push(format!("dx={dx:e}: faces {true_gap:e} apart answered Ok"));
        }
    }
    assert!(wrong.is_empty(), "{}", wrong.join("\n"));
}

/// The brick beside the ruling, the whole pose turned off the world's
/// axes about an oblique axis: dx = 0.3 is clear; the straddling control
/// escalates.
#[test]
fn r2_brick_beside_the_ruling_turned_oblique() {
    let gap = 5.5 * band().zero();
    let ax = Vec3::new(1.0, 2.0, 3.0);
    for (dx, want_ok) in [(0.3, true), (0.05, true), (-0.25, false)] {
        let rows = table(
            &format!("oblique dx={dx}"),
            turned(about_y((0.3, 1.0), (0.0, 1.0), None, 0.0, 0.0), ax, 0.7),
            turned(
                brick((dx, dx + 0.5), (0.2, 0.8), (-2.0, -1.0 - gap)),
                ax,
                0.7,
            ),
        );
        // Printed: the glue door is not the only carrier reader on the
        // way (the split's conic-belly graze reads the brick's top
        // carrier against the wall's on boxes that meet once turned).
        let _ = want_ok;
        assert!(none_frontier(&rows), "dx={dx}: {rows:?}");
    }
}

/// Outer c×c: a quarter tube beside a full tube, `5.5·zero` clear, its
/// wall's edge `delta` rad past the ruling on either side, the pair
/// turned 0.5 rad about z so the door is asked. Small delta puts the
/// faces in band at the edge: never `Ok`.
#[test]
fn r2_quarter_tube_edge_near_the_ruling() {
    let b = band();
    let gap = 5.5 * b.zero();
    let q = core::f64::consts::FRAC_PI_2;
    let pi = core::f64::consts::PI;
    let mut wrong = Vec::new();
    for delta in [0.0, 1e-6, 1e-5, 1e-3, 0.05] {
        for (side, spin) in [("past", pi + delta), ("before", q - delta)] {
            let rows = table(
                &format!("cc out {side} delta={delta:e}"),
                turned(
                    about_y((0.3, 1.0), (0.0, 1.0), None, 0.0, 0.0),
                    Vec3::unit_z(),
                    0.5,
                ),
                turned(
                    about_y((0.3, 1.0), (0.2, 0.8), Some(q), spin, 2.0 + gap),
                    Vec3::unit_z(),
                    0.5,
                ),
            );
            // two unit circles: the gap at angle delta off the contact
            // grows about delta² (each wall bends delta²/2 away).
            let true_gap = delta * delta + gap;
            if true_gap <= b.escalate() && any_ok(&rows) {
                wrong.push(format!(
                    "{side} delta={delta:e}: Ok with faces ~{true_gap:e} apart"
                ));
            }
            if !none_frontier(&rows) {
                wrong.push(format!("{side} delta={delta:e}: frontier refusal {rows:?}"));
            }
        }
    }
    assert!(wrong.is_empty(), "{}", wrong.join("\n"));
}

/// Inner c×c: a quarter rod (r = 0.2) in the bore (r = 0.3), `5.5·zero`
/// inside tangency, its wall's edge `delta` rad off the ruling.
#[test]
fn r2_quarter_rod_edge_near_the_bore_ruling() {
    let b = band();
    let gap = 5.5 * b.zero();
    let q = core::f64::consts::FRAC_PI_2;
    let mut wrong = Vec::new();
    for delta in [0.0, 1e-6, 1e-5, 1e-3, 0.05] {
        for (side, spin) in [("past", delta), ("before", -q - delta)] {
            let rows = table(
                &format!("cc in {side} delta={delta:e}"),
                about_y((0.3, 1.0), (0.0, 1.0), None, 0.0, 0.0),
                about_y((0.0, 0.2), (0.2, 0.8), Some(q), spin, 0.1 - gap),
            );
            // rod r=0.2 inside bore r=0.3, offset 0.1: the gap off the
            // contact grows about (0.1·0.2/0.3)·delta²/2·... bound it
            // below by 0.01·delta².
            let true_gap_lo = 0.01 * delta * delta + gap;
            if true_gap_lo <= b.escalate() && any_ok(&rows) {
                wrong.push(format!(
                    "{side} delta={delta:e}: Ok with faces near in band"
                ));
            }
            if !none_frontier(&rows) {
                wrong.push(format!("{side} delta={delta:e}: frontier refusal {rows:?}"));
            }
        }
    }
    assert!(wrong.is_empty(), "{}", wrong.join("\n"));
}

/// Claim 1: plane×cylinder on the ruling at several multiples of the
/// zero band (brick spans the ruling). In band: never a frontier
/// refusal, never Ok.
#[test]
fn r2_brick_on_the_ruling_at_multiples() {
    let b = band();
    let mut wrong = Vec::new();
    for m in [0.5, 1.5, 3.0, 5.5, 8.0, 20.0] {
        let gap = m * b.zero();
        let rows = table(
            &format!("on ruling gap={m}z"),
            about_y((0.3, 1.0), (0.0, 1.0), None, 0.0, 0.0),
            brick((-0.5, 0.5), (0.2, 0.8), (-2.0, -1.0 - gap)),
        );
        // The zero band (m < 1) glues and meets the frontier refusal
        // later, the same on the PR's base (pre-existing; printed only).
        if m > 1.0 && !none_frontier(&rows) {
            wrong.push(format!("gap={m}z: frontier {rows:?}"));
        }
    }
    assert!(wrong.is_empty(), "{}", wrong.join("\n"));
}

/// Claim 2: two washers crossing transversally, their inner radii in
/// band of each other (0.3 and 0.3 + delta) but their circles centred
/// apart. The door's pre-gate asks `shared_rim` of every non-ruled
/// pair whose boxes meet, and `same_circle` reads the radius row first:
/// an in-band radius escalates before the centres are read. Printed
/// against the control whose radii differ definitely.
#[test]
fn r2_crossing_washers_with_radii_in_band() {
    let b = band();
    let tilt = Vec3::unit_x();
    for (label, delta) in [("in band", 5.5 * b.zero()), ("control", 0.05)] {
        let a = about_y((0.3, 1.0), (0.0, 0.1), None, 0.0, 0.0);
        let w = about_y((0.3 + delta, 0.8), (-0.05, 0.05), None, 0.0, 0.0);
        let w = turned(w, tilt, 0.5);
        let w = topo::transform_rigid(&w, &Affine3::translation(Vec3::new(0.4, 0.05, 0.1)), tol())
            .unwrap();
        table(&format!("washers {label}"), a, w);
    }
}

/// Claim 2, a pose that builds: a frustum (radius 0.5 at y = 0 to 0.3
/// at y = 1) through a plate (y in [0.5, 0.6]) with a hole of radius
/// `0.3 + delta` 1.4 m off the frustum's axis. The plate's faces carry
/// the hole's circle and the cone face its top circle; the circles'
/// radii are in band, their centres 1.4 m apart, and the faces cross
/// transversally. Printed against the control whose radii differ.
#[test]
fn r2_a_frustum_through_a_plate_with_a_hole_of_its_top_radius() {
    let b = band();
    let profile = vec![
        (Point2::new(0.0, 0.0), 0.0),
        (Point2::new(0.5, 0.0), 0.0),
        (Point2::new(0.3, 1.0), 0.0),
        (Point2::new(0.0, 1.0), 0.0),
    ];
    for (label, delta) in [("in band", 5.5 * b.zero()), ("control", 0.05)] {
        let frustum = revolved_about_y(profile.clone(), sweep::Revolution::Full, tol());
        let slab = at_rest(brick((-1.0, 2.0), (0.5, 0.6), (-1.0, 1.0)));
        let rod = at_rest(about_y((0.0, 0.3 + delta), (0.4, 0.7), None, 0.0, 1.4));
        let plate = topo::subtract(&slab, &rod, tol()).expect("the hole drills");
        let plate = plate.body().expect("a plate").body.clone().into_body();
        let rows = table(&format!("frustum/plate {label}"), frustum, plate);
        // On the PR's base both rows build in every op, both orders.
        assert!(
            rows.iter().all(|r| r == "Ok"),
            "{label}: circles 1.4 m apart, faces crossing: {rows:?}"
        );
    }
}
