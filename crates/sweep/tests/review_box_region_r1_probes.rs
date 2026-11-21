//! Review probes for PR 7225 (`oneround/box-region`), reviewer r1.
//! Each prints what it measures; the assertions are the reviewer's
//! expectations of the README contract, not pins the author owes.
#![allow(clippy::unwrap_used, clippy::expect_used, clippy::panic, clippy::print_stdout)]

use core::f64::consts::PI;
use geom_core::{Band, Point2, RigidMotion, Tol, UnitVec3, Vec3};
use sweep::Revolution;
use sweep::test_support::revolved_about_y;

fn class(errors: &[topo::ValidationError]) -> String {
    match errors {
        [topo::ValidationError::OutsideSessionBox { margin }] => {
            format!("Outside({})", margin.readable_below_zero())
        }
        [topo::ValidationError::SessionBoxUnshown { margin }] => {
            format!("Unshown({})", margin.readable_below_zero())
        }
        [topo::ValidationError::SessionBoxUncovered { .. }] => "Uncovered".into(),
        other => format!("Other({:?})", other.iter().map(|e| format!("{e}")).collect::<Vec<_>>().join(" | ").chars().take(120).collect::<String>()),
    }
}

fn gate(body: &topo::Body<f64>, tol: Tol) -> String {
    match topo::validate_geometric(body, tol) {
        Ok(_) => "Ok".into(),
        Err(e) => class(&e),
    }
}

fn door(home: &topo::Body<f64>, at: &RigidMotion<f64>, tol: Tol) -> String {
    match topo::session_box_at(home, at, tol) {
        Ok(()) => "Ok".into(),
        Err(topo::MoveError::NotFinished(found)) => class(found.as_slice()),
        Err(other) => format!("Map({other:?})"),
    }
}

fn unit(v: Vec3<f64>, tol: Tol) -> UnitVec3<f64> {
    UnitVec3::new(v, "probe axis", Band::linear(tol).unwrap()).unwrap()
}

fn torus_y(big: f64, r: f64, tol: Tol) -> topo::Body<f64> {
    // Seams at the inner and outer equators: the top circle is interior.
    revolved_about_y(
        vec![(Point2::new(big - r, 0.0), 1.0), (Point2::new(big + r, 0.0), 1.0)],
        Revolution::Full,
        tol,
    )
}

fn torus_row(tol: Tol) -> topo::Body<f64> {
    revolved_about_y(
        vec![(Point2::new(450.0, -400.0), 1.0), (Point2::new(450.0, 400.0), 1.0)],
        Revolution::Full,
        tol,
    )
}

fn cone(tol: Tol) -> topo::Body<f64> {
    revolved_about_y(
        vec![
            (Point2::new(0.0, 0.0), 0.0),
            (Point2::new(200.0, 0.0), 0.0),
            (Point2::new(0.0, 300.0), 0.0),
        ],
        Revolution::Full,
        tol,
    )
}

/// The conical-ended football: spindle sheet (R = 30, r = 50) between two
/// cones; its equator (radius 80) is interior to the spindle face except
/// at the revolve seam (azimuth +x).
fn football(tol: Tol) -> topo::Body<f64> {
    let (big, r, half, tip) = (30.0_f64, 50.0_f64, 1.2_f64, 80.0_f64);
    let (rho, z) = (big + r * half.cos(), r * half.sin());
    revolved_about_y(
        vec![
            (Point2::new(0.0, -tip), 0.0),
            (Point2::new(rho, -z), (half / 2.0).tan()),
            (Point2::new(rho, z), 0.0),
            (Point2::new(0.0, tip), 0.0),
        ],
        Revolution::Full,
        tol,
    )
}

/// C4: the placing door against the gate on the moved image, over
/// rotations and translations sweeping each body through the box's face.
#[test]
fn probe_c4_door_equals_gate_on_image() {
    let tol = Tol::witness();
    let bodies: Vec<(&str, topo::Body<f64>)> = vec![
        ("torus_row", torus_row(tol)),
        ("torus_y", torus_y(300.0, 100.0, tol)),
        ("ball_y", sweep::test_support::ball_poled_y::<f64>(50.0, Vec3::new(0.0, 0.0, 0.0), tol)),
        ("ball_z", sweep::test_support::ball_poled_z(50.0, Vec3::new(0.0, 0.0, 0.0), tol)),
        ("cone", cone(tol)),
        ("loft", sweep::test_support::loft_prism(tol)),
        ("football", football(tol)),
    ];
    let turns = [
        (Vec3::new(0.0, 1.0, 0.0), 0.0),
        (Vec3::new(1.0, 2.0_f64.sqrt(), 3.0_f64.sqrt()), 0.7),
        (Vec3::new(1.0, 0.0, 0.0), PI / 2.0),
        (Vec3::new(0.0, 0.0, 1.0), PI / 4.0),
        (Vec3::new(1.0, 1.0, 1.0), 2.1),
    ];
    let mut disagree = 0;
    let mut tally = std::collections::BTreeMap::<String, usize>::new();
    for (what, home) in &bodies {
        // A body's scale: the loft is 2 m, the rest hundreds.
        let (lo, hi, step) = if *what == "loft" { (990.0, 1002.0, 0.125) } else { (700.0, 1100.0, 2.5) };
        for (axis, angle) in turns {
            let turn = RigidMotion::rotation(unit(axis, tol), angle).unwrap();
            for dir in [Vec3::new(1.0, 0.0, 0.0), Vec3::new(0.0, 1.0, 0.0), Vec3::new(0.0, 0.0, -1.0), Vec3::new(1.0, 1.0, 1.0) * (1.0 / 3.0_f64.sqrt())] {
                let mut t = lo;
                while t <= hi {
                    let at = RigidMotion::translation(dir * t).unwrap().compose(turn).unwrap();
                    let image = topo::transform_rigid(home, &at).unwrap();
                    let (g, d) = (gate(&image, tol), door(home, &at, tol));
                    let gk = g.split('(').next().unwrap().to_string();
                    let dk = d.split('(').next().unwrap().to_string();
                    *tally.entry(format!("{what} gate={gk}")).or_default() += 1;
                    if gk != dk && !(gk == "Other" && dk == "Ok") {
                        disagree += 1;
                        println!("DISAGREE {what} turn={axis:?}/{angle} dir={dir:?} t={t}: gate {g} door {d}");
                    } else if gk == "Other" {
                        println!("GATE-OTHER {what} turn={axis:?}/{angle} dir={dir:?} t={t}: gate {g} door {d}");
                    }
                    t += step;
                }
            }
        }
    }
    println!("tally {tally:#?}");
    println!("disagreements: {disagree}");
}

/// C2: a torus whose axis is tilted by a small angle off `+y`, its top
/// placed so the true largest `y` is past the box by half of `R·sinθ`
/// and every vertex and edge is inside it.
#[test]
fn probe_c2_nearly_axial_torus() {
    let tol = Tol::witness();
    let (big, r) = (300.0_f64, 100.0_f64);
    let home = torus_y(big, r, tol);
    for theta in [0.3, 1e-1, 1e-2, 1e-3, 1e-4, 1e-5, 1e-6, 1e-7] {
        let turn = RigidMotion::rotation(unit(Vec3::new(1.0, 0.0, 0.0), tol), theta).unwrap();
        let top = r + big * theta.sin(); // the true extreme along y above the centre
        let seam = r; // the meridian seam's top, its tilt aside (r cos θ)
        let past = 0.5 * (top - seam);
        let cy = 1000.0 + past - top;
        let at = RigidMotion::translation(Vec3::new(0.0, cy, 0.0)).unwrap().compose(turn).unwrap();
        let image = topo::transform_rigid(&home, &at).unwrap();
        println!(
            "theta {theta:e}: true top {:.9} (past by {past:.3e}); gate {} ; door {}",
            cy + top,
            gate(&image, tol),
            door(&home, &at, tol)
        );
    }
}

/// C1 / README: a body with a point past the box whose bulge no edge and
/// no inside candidate reaches (the football's spindle equator along z).
#[test]
fn probe_spindle_point_past_reads_as_limit() {
    let tol = Tol::witness();
    let home = football(tol);
    for cz in [900.0, 930.0, 950.0, 960.0] {
        let at = RigidMotion::translation(Vec3::new(0.0, 0.0, cz)).unwrap();
        let image = topo::transform_rigid(&home, &at).unwrap();
        println!(
            "football at z={cz}: equator reaches {}; gate {} ; door {}",
            cz + 80.0,
            gate(&image, tol),
            door(&home, &at, tol)
        );
    }
}

/// C1 oracle: full tori and balls under rotations off every axis, placed
/// so their analytic largest `|coordinate|` sweeps across 1000 m; the
/// gate's verdict against that true reach. A true reach past the box by
/// more than 1e-3 m must not be admitted; one inside by more than 1e-3 m
/// must not be refused as past.
#[test]
fn probe_c1_analytic_reach_oracle() {
    let tol = Tol::witness();
    // (name, body, R, r) with the torus axis +y at home; R = 0 for a ball.
    let bodies: Vec<(&str, topo::Body<f64>, f64, f64)> = vec![
        ("torus_row", torus_row(tol), 450.0, 400.0),
        ("torus_y", torus_y(300.0, 100.0, tol), 300.0, 100.0),
        ("thin_torus", torus_y(600.0, 20.0, tol), 600.0, 20.0),
        ("ball_y", sweep::test_support::ball_poled_y::<f64>(50.0, Vec3::new(0.0, 0.0, 0.0), tol), 0.0, 50.0),
    ];
    let mut seed = 0x2545F4914F6CDD1Du64;
    let mut rnd = move || {
        seed ^= seed << 13;
        seed ^= seed >> 7;
        seed ^= seed << 17;
        (seed >> 11) as f64 / (1u64 << 53) as f64
    };
    let (mut admitted_past, mut refused_inside, mut limit_past, mut n) = (0, 0, 0, 0);
    for (what, home, big, r) in &bodies {
        for _ in 0..60 {
            let axis = Vec3::new(rnd() - 0.5, rnd() - 0.5, rnd() - 0.5);
            let angle = rnd() * PI;
            let turn = RigidMotion::rotation(unit(axis, tol), angle).unwrap();
            let a = turn.to_affine().transform_vec(Vec3::new(0.0, 1.0, 0.0));
            let a = a * (1.0 / a.norm());
            // Pick the axis and sign to push along, then the centre so the
            // true reach there is 1000 + d.
            let k = (rnd() * 3.0) as usize % 3;
            let e = [Vec3::new(1.0, 0.0, 0.0), Vec3::new(0.0, 1.0, 0.0), Vec3::new(0.0, 0.0, 1.0)][k];
            let s: f64 = (1.0_f64 - a.dot(e).powi(2)).max(0.0).sqrt();
            let half = big * s + r;
            let d = [-0.5, -0.01, 0.01, 0.5, 3.0][(rnd() * 5.0) as usize % 5];
            let c = e * (1000.0 + d - half);
            let at = RigidMotion::translation(c).unwrap().compose(turn).unwrap();
            let image = topo::transform_rigid(home, &at).unwrap();
            let g = gate(&image, tol);
            let pd = door(home, &at, tol);
            n += 1;
            let kind = g.split('(').next().unwrap();
            if d > 1e-3 && kind == "Ok" {
                admitted_past += 1;
                println!("ADMITTED-PAST {what} axis {a:?} e{k} d {d}: gate {g} door {pd}");
            }
            if d < -1e-3 && kind == "Outside" {
                refused_inside += 1;
                println!("REFUSED-INSIDE {what} axis {a:?} e{k} d {d}: gate {g} door {pd}");
            }
            if d > 1e-3 && kind != "Outside" {
                limit_past += 1;
                println!("NOT-OUTSIDE-PAST {what} axis {a:?} e{k} d {d}: gate {g} door {pd}");
            }
            if g.split('(').next() != pd.split('(').next() {
                println!("DOOR!=GATE {what} axis {a:?} e{k} d {d}: gate {g} door {pd}");
            }
        }
    }
    println!("n {n}: admitted past {admitted_past}, refused inside {refused_inside}, past but not Outside {limit_past}");
}

/// A barrel: the zone `|y| ≤ 30` of a radius-50 sphere about the origin,
/// capped by two plane discs. Its sphere's poles (`y = ±50`) are not on
/// its zone face.
fn barrel(tol: Tol) -> topo::Body<f64> {
    revolved_about_y(
        vec![
            (Point2::new(0.0, -30.0), 0.0),
            (Point2::new(40.0, -30.0), 1.0 / 3.0),
            (Point2::new(40.0, 30.0), 0.0),
            (Point2::new(0.0, 30.0), 0.0),
        ],
        Revolution::Full,
        tol,
    )
}

/// **A sphere zone whose carrier's pole is past the box and whose face is
/// inside it is admitted** (reviewer r1). The barrel centred at
/// `y = 965` reaches `y = 995` (its top disc); its sphere's ball reaches
/// 1015 m, past the box, and its `+y` pole is not on the face. So stage 1
/// (the ball) does not admit it, stage 2 (the zone's own cover) does,
/// and the inside read must drop the pole. At `y = 975` its disc is past
/// by 5 m and it is refused as past the box. *Breaks if* stage 2 never
/// admits, or a candidate the face does not hold is kept.
#[test]
fn a_zone_whose_pole_alone_is_past_the_box_is_admitted() {
    let tol = Tol::witness();
    let home = barrel(tol);
    for (f, _) in home.faces() {
        println!("face surface {:?}", home.face(f).map(|v| v.surface().clone()));
    }
    assert_eq!(gate(&home, tol), "Ok", "the barrel at the origin");
    let at = RigidMotion::translation(Vec3::new(0.0, 965.0, 0.0)).unwrap();
    let image = topo::transform_rigid(&home, &at).unwrap();
    assert_eq!(gate(&image, tol), "Ok", "the barrel at 965 m: its face is in the box");
    assert_eq!(door(&home, &at, tol), "Ok", "placed at 965 m");
    let far = RigidMotion::translation(Vec3::new(0.0, 975.0, 0.0)).unwrap();
    let image = topo::transform_rigid(&home, &far).unwrap();
    assert!(gate(&image, tol).starts_with("Outside(5"), "{}", gate(&image, tol));
    assert!(door(&home, &far, tol).starts_with("Outside(5"), "{}", door(&home, &far, tol));
}

/// **An arc edge whose interior alone is past the box refuses past it**
/// (reviewer r1). A cylinder of radius 100 about `(cx, 0)`, its circle
/// four quarter arcs whose ends sit on the diagonals, extruded 1 m: at
/// `cx = 905` its `+x` arc reaches 1005 m at its middle, on no vertex
/// (the vertices reach 975.7 m) and on no face extreme (a cylinder has
/// none). Refused past the box by 5 m; at `cx = 895` it is admitted.
/// *Breaks if* stage 3 drops the edges' own points.
#[test]
fn an_arc_whose_middle_alone_is_past_the_box_refuses_past_it() {
    let tol = Tol::witness();
    let b = (PI / 8.0).tan();
    let at = |cx: f64| {
        let d = 100.0 / 2.0_f64.sqrt();
        let lp = profile::test_support::bulge_loop(vec![
            (Point2::new(cx + d, -d), b),
            (Point2::new(cx + d, d), b),
            (Point2::new(cx - d, d), b),
            (Point2::new(cx - d, -d), b),
        ]);
        let pf = profile::Profile::new(profile::SketchPlane::xy(), vec![lp])
            .validate(tol)
            .expect("a valid circle");
        sweep::extrude(
            &pf,
            sweep::Extrusion::Distance { depth: 1.0, side: sweep::ExtrudeSide::Along },
            tol,
        )
    };
    match at(905.0) {
        Err(e) => {
            let said = format!("{e:?}");
            assert!(said.contains("OutsideSessionBox { margin: MarginDiag(Value(-5"), "{said}");
        }
        Ok(_) => panic!("the arc's middle is 5 m past the box"),
    }
    assert!(at(895.0).is_ok(), "inside the box");
}
