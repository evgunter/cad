//! R1 review probes for VERBS-PIERCE (#1068). Adjacent shapes to the
//! disc class: annular caps, concentric circles, ring-carrying planar
//! faces, and the STATED blind spot (loops that mix arcs and lines).
//!
//! Nothing here is a deliverable; every row prints what it measured so
//! the same file can be run at the base commit for a before column.

#![allow(clippy::unwrap_used, clippy::expect_used, clippy::panic)]

use core::f64::consts::PI;

use geom_core::{Affine3, Point2, Tol, Vec3};
use profile::{Profile, ProfileLoop, ProfileVertex, RawLoop, SketchPlane};
use sweep::{Extrusion, extrude};
use topo::{Body, BooleanError};

fn p2(x: f64, y: f64) -> Point2<f64> {
    Point2::new(x, y)
}

fn pv(x: f64, y: f64, bulge: f64) -> ProfileVertex<f64> {
    ProfileVertex::new(p2(x, y), bulge)
}

fn body_of(loops: Vec<ProfileLoop<f64>>, z0: f64, z1: f64) -> Body<f64> {
    let tol = Tol::witness();
    let plane = SketchPlane::new(Affine3::translation(Vec3::new(0.0, 0.0, z0)));
    let profile = Profile::new(plane, loops).validate(tol).unwrap();
    extrude(&profile, Extrusion::Distance(z1 - z0), tol)
        .unwrap()
        .body
}

fn cyl(cx: f64, cy: f64, r: f64, z0: f64, z1: f64) -> Body<f64> {
    let tol = Tol::witness();
    let lp = profile::circle(p2(cx, cy), r, tol).unwrap();
    body_of(vec![lp.into()], z0, z1)
}

/// An annular cap: outer circle `ro`, coaxial bore `ri`.
fn tube(ro: f64, ri: f64, z0: f64, z1: f64) -> Body<f64> {
    let tol = Tol::witness();
    let outer = profile::circle(p2(0.0, 0.0), ro, tol).unwrap();
    let bore = profile::circle(p2(0.0, 0.0), ri, tol).unwrap();
    body_of(vec![outer.into(), bore.into()], z0, z1)
}

fn boxx(x0: f64, x1: f64, y0: f64, y1: f64, z0: f64, z1: f64) -> Body<f64> {
    body_of(
        vec![RawLoop::polygon([
            p2(x0, y0),
            p2(x1, y0),
            p2(x1, y1),
            p2(x0, y1),
        ])],
        z0,
        z1,
    )
}

fn report(name: &str, a: &Body<f64>, b: &Body<f64>) -> Option<f64> {
    let tol = Tol::witness();
    match topo::union(a, b, tol) {
        Err(e) => {
            println!("R1[{name}] REFUSED {e:?}");
            None
        }
        Ok(topo::BooleanResult::Body(out)) => {
            let shells = out.body.shells().count();
            let tier3 = topo::validate_geometric(&out.body, tol);
            let v = topo::mass_properties(&out.body, tol).map(|p| p.volume);
            println!("R1[{name}] BODY shells={shells} tier3={tier3:?} volume={v:?}");
            v.ok()
        }
        Ok(other) => {
            println!("R1[{name}] OTHER {other:?}");
            None
        }
    }
}

// ---------------------------------------------------------------
// The disc class's adjacent shapes.
// ---------------------------------------------------------------

/// **Annular cap.** A tube's cap is bounded by TWO circle loops (outer
/// + bore), each all-arc, so `loop_disc` fires on both. A box driven
/// through the SOLID part of the annulus must not be silent.
#[test]
fn r1_a_box_through_an_annular_cap() {
    let a = tube(1.0, 0.4, 0.0, 2.0);
    let b = boxx(0.55, 0.85, -0.15, 0.15, 1.0, 3.0);
    report("annular-cap-through-wall", &a, &b);
}

/// **The bore is a hole.** A box driven down the BORE of a tube meets
/// no material at the cap at all: the point is inside the outer disc
/// AND inside the ring, so the face must answer `Out`.
#[test]
fn r1_a_box_down_the_bore_of_a_tube() {
    let a = tube(1.0, 0.4, 0.0, 2.0);
    let b = boxx(-0.2, 0.2, -0.2, 0.2, 1.0, 3.0);
    let v = report("annular-cap-down-the-bore", &a, &b);
    // Truth if it unions: tube volume + the box's part outside the tube
    // (z ∈ [2,3] slab) — the box in z ∈ [1,2] is entirely in the bore.
    let truth = PI * (1.0 - 0.16) * 2.0 + 0.4 * 0.4 * 1.0;
    println!("R1[annular-cap-down-the-bore] truth-if-union={truth} got={v:?}");
}

/// **Concentric circles, no ring.** Two coaxial cylinders of different
/// radii is the boss row's shape; here the SMALL one is fully buried
/// so the caps' discs decide by containment alone.
#[test]
fn r1_concentric_buried_cylinder() {
    let a = cyl(0.0, 0.0, 1.0, 0.0, 2.0);
    let b = cyl(0.0, 0.0, 0.4, 0.5, 1.5);
    let v = report("concentric-buried", &a, &b);
    println!("R1[concentric-buried] truth={} got={v:?}", PI * 2.0);
}

/// **A ring-carrying planar face in a boolean, the polygon kind.** A
/// square hole through a box: the ring is a polygon, so the disc class
/// must NOT fire and the ray-parity walk must keep its old answer.
#[test]
fn r1_square_hole_plate_unioned_with_a_boss() {
    let plate = body_of(
        vec![
            RawLoop::polygon([p2(-2.0, -2.0), p2(2.0, -2.0), p2(2.0, 2.0), p2(-2.0, 2.0)]),
            RawLoop::polygon([p2(-0.5, -0.5), p2(0.5, -0.5), p2(0.5, 0.5), p2(-0.5, 0.5)]),
        ],
        0.0,
        1.0,
    );
    let boss = boxx(-0.2, 0.2, -0.2, 0.2, 0.5, 2.0);
    report("square-hole-plate+boss", &plate, &boss);
}

// ---------------------------------------------------------------
// The STATED blind spot: loops that MIX arcs and lines.
// ---------------------------------------------------------------

/// **Half-disc** — one semicircular arc plus one straight chord, TWO
/// vertices, so `point_in_loop`'s polygon through them is the chord: a
/// segment of zero area, exactly the cap's defect. Both bulge senses
/// are run against the SAME box: one of the two genuinely contains it,
/// so if both answered "disjoint" the wrong answer was demonstrated
/// without having to know which sense is which.
///
/// **RED-BY-DESIGN, FLIPPED.** Both senses measured the silent wrong
/// body (3.321592653589793 against a truth of 3.231592653589793); the
/// loop-shape gate refuses both typed now.
#[test]
fn r1_a_box_through_a_half_disc_cap() {
    let tol = Tol::witness();
    let b = boxx(-0.15, 0.15, -0.6, -0.3, 1.0, 3.0);
    let hd = PI * 0.5 * 2.0;
    let disjoint_answer = hd + 0.3 * 0.3 * 2.0;
    let buried_truth = hd + 0.3 * 0.3 * 1.0;
    for bulge in [1.0, -1.0] {
        // bulge = tan(theta/4); a semicircle is theta = pi -> |1|.
        let half = ProfileLoop::new(vec![pv(-1.0, 0.0, bulge), pv(1.0, 0.0, 0.0)]);
        let a = body_of(vec![half], 0.0, 2.0);
        match topo::union(&a, &b, tol) {
            Err(e) => assert!(
                matches!(e, BooleanError::ArcLoopContainmentUnsupported { .. }),
                "bulge={bulge}: the half-disc cap has no walk; got {e:?}"
            ),
            Ok(topo::BooleanResult::Body(out)) => {
                let v = topo::mass_properties(&out.body, tol).unwrap().volume;
                panic!(
                    "bulge={bulge}: ARC-BEARING LOOP SILENT WRONG BODY {v} \
                     (disjoint-answer {disjoint_answer}, truth {buried_truth})"
                );
            }
            Ok(other) => panic!("bulge={bulge}: unexpected {other:?}"),
        }
    }
}

/// **Slot** — two straight flanks and two semicircular ends.
#[test]
fn r1_a_box_through_a_slot_cap() {
    let slot = ProfileLoop::new(vec![
        pv(-1.0, -0.5, 0.0),
        pv(1.0, -0.5, 1.0),
        pv(1.0, 0.5, 0.0),
        pv(-1.0, 0.5, 1.0),
    ])
    .with_tangent_joints(vec![0, 1, 2, 3]);
    let a = body_of(vec![slot], 0.0, 2.0);
    let b = boxx(-0.15, 0.15, -0.15, 0.15, 1.0, 3.0);
    let v = report("slot-cap", &a, &b);
    let area = 2.0 * 1.0 + PI * 0.25;
    println!(
        "R1[slot-cap] disjoint-wrong={} truth={} got={v:?}",
        area * 2.0 + 0.3 * 0.3 * 2.0,
        area * 2.0 + 0.3 * 0.3 * 1.0
    );
}

/// **Rounded rectangle** — four straight flanks and four quarter arcs.
#[test]
fn r1_a_box_through_a_rounded_rectangle_cap() {
    // quarter arc: bulge = tan(pi/8).
    let q = (PI / 8.0).tan();
    let (w, h, r) = (1.5f64, 1.0f64, 0.3f64);
    let rr = ProfileLoop::new(vec![
        pv(-w + r, -h, 0.0),
        pv(w - r, -h, q),
        pv(w, -h + r, 0.0),
        pv(w, h - r, q),
        pv(w - r, h, 0.0),
        pv(-w + r, h, q),
        pv(-w, h - r, 0.0),
        pv(-w, -h + r, q),
    ])
    .with_tangent_joints(vec![0, 1, 2, 3, 4, 5, 6, 7]);
    let a = body_of(vec![rr], 0.0, 2.0);
    let b = boxx(-0.15, 0.15, -0.15, 0.15, 1.0, 3.0);
    let v = report("rounded-rect-cap", &a, &b);
    let area = 2.0 * w * 2.0 * h - (4.0 - PI) * r * r;
    println!(
        "R1[rounded-rect-cap] disjoint-wrong={} truth={} got={v:?}",
        area * 2.0 + 0.3 * 0.3 * 2.0,
        area * 2.0 + 0.3 * 0.3 * 1.0
    );
}

// ---------------------------------------------------------------
// The re-derived acceptance numbers.
// ---------------------------------------------------------------

/// The boss row's closed form, re-derived here from the fixture's own
/// constants rather than copied from the PR: shaft r=1 over z∈[0,2],
/// boss r=0.5 over z∈[1,3]; the boss's buried half adds nothing, its
/// stub z∈[2,3] adds pi*0.5^2*1.
#[test]
fn r1_the_boss_closed_form_and_ulp_claim() {
    let tol = Tol::witness();
    let out = match topo::union(
        &cyl(0.0, 0.0, 1.0, 0.0, 2.0),
        &cyl(0.0, 0.0, 0.5, 1.0, 3.0),
        tol,
    ) {
        Ok(topo::BooleanResult::Body(o)) => o,
        other => panic!("R1[boss] {other:?}"),
    };
    let v = topo::mass_properties(&out.body, tol).unwrap().volume;
    let shaft = PI * 1.0f64.powi(2) * 2.0;
    let stub = PI * 0.5f64.powi(2) * (3.0 - 2.0);
    let truth = shaft + stub;
    let ulp = (truth - f64::from_bits(truth.to_bits() - 1)).abs();
    println!(
        "R1[boss] volume={v} truth={truth} err={} ulp={ulp} err_in_ulps={}",
        v - truth,
        (v - truth) / ulp
    );
    println!(
        "R1[boss] tier3={:?} shells={}",
        topo::validate_geometric(&out.body, tol),
        out.body.shells().count()
    );
}

/// The box-through-a-cap yardstick, re-derived from the fixture.
#[test]
fn r1_the_cap_yardstick_is_what_the_pr_says() {
    let wrong = PI * 1.0f64.powi(2) * 2.0 + 0.6 * 0.6 * 2.0;
    let truth = PI * 1.0f64.powi(2) * 2.0 + 0.6 * 0.6 * 1.0;
    println!("R1[yardstick] wrong={wrong:?} truth={truth:?}");
    println!(
        "R1[yardstick] pr_wrong=7.003185307179585 pr_truth=6.643185307179586 \
         naive_closed_form_differs={} {}",
        format!("{wrong}") != "7.003185307179585",
        format!("{truth}") != "6.643185307179586"
    );
}

/// **The RING half of the disc class, isolated.** The PR claims "a
/// circular hole was invisible too: the ring's two-vertex polygon
/// holds only its own diameter, and a point inside the hole was
/// reported inside the face". Nothing in the PR demonstrates it — the
/// tube rows do not, because a tube's OUTER loop is also a circle and
/// the two errors cancel. This isolates it: a SQUARE plate (outer loop
/// a polygon, already right) with a CIRCULAR hole, and a box driven
/// down the hole. The box meets no plate material at all.
#[test]
fn r1_a_box_down_a_circular_hole_in_a_square_plate() {
    let tol = Tol::witness();
    let hole = profile::circle(p2(0.0, 0.0), 0.5, tol).unwrap();
    let plate = body_of(
        vec![
            RawLoop::polygon([p2(-2.0, -2.0), p2(2.0, -2.0), p2(2.0, 2.0), p2(-2.0, 2.0)]),
            hole.into(),
        ],
        0.0,
        1.0,
    );
    let boss = boxx(-0.2, 0.2, -0.2, 0.2, 0.5, 2.0);
    let v = report("circular-hole-plate+box-down-the-hole", &plate, &boss);
    let truth = (16.0 - PI * 0.25) * 1.0 + 0.4 * 0.4 * 1.5;
    println!("R1[circular-hole-plate] truth-if-disjoint={truth} got={v:?}");
}
