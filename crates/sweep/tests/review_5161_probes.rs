//! Review probes for PR 5161 (a cup on one end wall goes with the band),
//! on head 98f0e25cc. Results: P1/P2 fillet refuse `FaceClearance`
//! measured, chamfer builds on the oracle; P3/P5 chamfer refuse
//! `WhollyInside`; P4 refuses at the screen; P6/P7 build on the oracle.
//!
//! The oracle is the ruling's set algebra, read off the result alone: a
//! convex band's body is `S − R`, so whatever cup lay inside `R` the
//! block `[0, 1]³` leaves `1 − A·1`; a concave band's is `S ∪ F`, so the
//! die leaves its own volume plus `A·½`. Any request that builds must
//! land there, valid at tiers 3/3′, with naming totality; any that
//! refuses must refuse typed and not as a surgery invariant.

#![allow(clippy::unwrap_used, clippy::expect_used, clippy::panic)]

use core::f64::consts::PI;

use geom_core::Vec3;
use sweep::blend::BlendError;
use sweep::test_support::{assert_naming_totality, brick};
use topo::{Body, ContactRecords, EdgeKey, validate_geometric, validate_pseudomanifold};

use crate::band_concave_bar::{Section, bar, blend, boolean, die, floor};
use crate::band_planar_consume::{VERBS, section};
use crate::band_planar_cut_off::{D, Verb, edge, tol, volume_enclosure};

#[derive(Debug, PartialEq)]
enum Got {
    Built,
    Refused(String),
}

/// Runs `verb`, and checks a build against the set-algebra volume
/// `want` and the genus `genus`; a refusal is returned typed.
fn run(verb: Verb, body: &Body<f64>, edges: &[EdgeKey], want: f64, genus: i64, what: &str) -> Got {
    validate_geometric(body, tol()).unwrap_or_else(|e| panic!("{what}: source tier 3, {e:?}"));
    match blend(verb, body, edges, D) {
        Ok(out) => {
            validate_geometric(&out.body, tol())
                .unwrap_or_else(|e| panic!("{what} {verb:?}: tier 3, {e:?}"));
            validate_pseudomanifold(&out.body, &ContactRecords::default(), tol())
                .unwrap_or_else(|e| panic!("{what} {verb:?}: tier 3′, {e:?}"));
            assert_naming_totality(body, &out, edges, what);
            let c = topo::readback::euler_counts(&out.body);
            assert_eq!(
                (c.s, c.genus().expect("genus")),
                (1, genus),
                "{what} {verb:?}: one shell, genus"
            );
            let (v, pad) = volume_enclosure(&out.body);
            assert!(
                (v - want).abs() < 1e-9 + pad,
                "{what} {verb:?}: volume {v} ± {pad} vs the set algebra's {want}"
            );
            Got::Built
        }
        Err(e) => {
            assert!(
                !matches!(e, BlendError::SurgeryInvariant { .. }),
                "{what} {verb:?}: refused as a broken invariant: {e:?}"
            );
            Got::Refused(format!("{e:?}"))
        }
    }
}

fn block() -> Body<f64> {
    brick((0.0, 1.0), (0.0, 1.0), (0.0, 1.0), tol())
}

fn top(body: &Body<f64>) -> EdgeKey {
    edge(body, [0.0, 1.0, 1.0], [1.0, 1.0, 1.0])
}

/// A round rod of radius `a` whose axis runs linearly in corner
/// coordinates from `p` at `x = xa` to `q` at `x = xb`, mapped by `at`
/// to `(y, z)`, its ends square to the axis.
fn tilted(
    (xa, p): (f64, (f64, f64)),
    (xb, q): (f64, (f64, f64)),
    a: f64,
    at: impl Fn((f64, f64)) -> (f64, f64),
) -> Body<f64> {
    bar(
        (xa, at(p)),
        (xb, at(q)),
        0.0,
        Section::Round(a),
        Vec3::new(0.0, 1.0, 0.0),
    )
}

fn convex_at((u, v): (f64, f64)) -> (f64, f64) {
    (1.0 - u, 1.0 - v)
}

fn concave_at((u, v): (f64, f64)) -> (f64, f64) {
    (0.25 + u, 0.5 + v)
}

/// The corner coordinates at `x` on the line through `p` at `x0` and
/// `q` at `x1`.
fn lerp(x: f64, (x0, p): (f64, (f64, f64)), (x1, q): (f64, (f64, f64))) -> (f64, f64) {
    let t = (x - x0) / (x1 - x0);
    (p.0 + t * (q.0 - p.0), p.1 + t * (q.1 - p.1))
}

/// **P1. A tilted blind bore whose mouth and floor lie deep in the
/// fillet's sliver but whose axis dips through the arc's disk midway**
/// (`(0.005, 0.055) → (0.055, 0.005)`, the sliver non-convex): the
/// fillet must not build it; the chamfer's triangle holds it whole
/// (`u + v = 0.06`), so a build there lands on `1 − A`.
#[test]
fn p1_a_tilted_bore_through_the_arcs_disk_midway() {
    let a = 0.003;
    let (m, f) = ((0.0, (0.005, 0.055)), (0.5, (0.055, 0.005)));
    let start = (-0.2, lerp(-0.2, m, f));
    let body = boolean(false, block(), tilted(start, f, a, convex_at));
    for verb in VERBS {
        let want = 1.0 - section(verb, D);
        let got = run(verb, &body, &[top(&body)], want, 0, "P1 tilted bore");
        eprintln!("P1 {verb:?}: {got:?}");
        if matches!(verb, Verb::Fillet) {
            assert_ne!(got, Got::Built, "P1: the bore crosses the arc");
        }
    }
}

/// **P2. The concave twin**: a tilted pin from the die's end wall whose
/// axis dips through the fill's arc midway; the fillet must not build.
#[test]
fn p2_a_tilted_pin_through_the_arc_midway() {
    let a = 0.003;
    let (m, f) = ((0.25, (0.005, 0.055)), (0.5, (0.055, 0.005)));
    let start = (0.1, lerp(0.1, m, f));
    let d = die::<f64>(1.0, 0.0);
    let (v_die, _) = volume_enclosure(&d);
    let body = boolean(true, d, tilted(start, f, a, concave_at));
    for verb in VERBS {
        let want = v_die + section(verb, D) * 0.5;
        let got = run(verb, &body, &[floor(&body, 1.0)], want, 0, "P2 tilted pin");
        eprintln!("P2 {verb:?}: {got:?}");
        if matches!(verb, Verb::Fillet) {
            assert_ne!(got, Got::Built, "P2: the pin crosses the arc");
        }
    }
}

/// **P3. A pin with a cross hole** (a handle, genus 1, wholly inside the
/// fill): the Euler count is `−1`, so it refuses; a build would have to
/// seal it (`S ∪ F`, genus 0).
#[test]
fn p3_a_pin_with_a_cross_hole() {
    let (c, a) = ((0.025, 0.025), 0.006);
    let d = die::<f64>(1.0, 0.0);
    let (v_die, _) = volume_enclosure(&d);
    let pin = boolean(
        true,
        d,
        bar(
            (0.1, concave_at(c)),
            (0.5, concave_at(c)),
            0.0,
            Section::Square(a),
            Vec3::new(0.0, 1.0, 0.0),
        ),
    );
    // A rod along `y` through the pin at `x = 0.4`, its ends in the air.
    let (y, z) = concave_at(c);
    // `bar` runs along `x`; build the cross rod square to it instead.
    let cross = sweep_rod_y((0.4, z), (y - 0.012, y + 0.012), 0.002);
    let body = boolean(false, pin, cross);
    assert_eq!(
        topo::readback::euler_counts(&body).genus().expect("genus"),
        1,
        "P3: the source pin carries a handle"
    );
    for verb in VERBS {
        let want = v_die + section(verb, D) * 0.5;
        let got = run(verb, &body, &[floor(&body, 1.0)], want, 0, "P3 holed pin");
        eprintln!("P3 {verb:?}: {got:?}");
    }
}

/// A round rod along `y` over `[y0, y1]`, its axis at `(x, z)`.
fn sweep_rod_y((x, z): (f64, f64), (y0, y1): (f64, f64), a: f64) -> Body<f64> {
    use geom_core::{Point2, Point3};
    use sweep::test_support::{prism_on, sketch_from_axes};
    prism_on(
        sketch_from_axes(
            Point3::new(x, y0, z),
            Vec3::new(1.0, 0.0, 0.0),
            Vec3::new(0.0, 0.0, -1.0),
            tol(),
        ),
        [(-a, -a), (a, -a), (a, a), (-a, a)]
            .into_iter()
            .map(|(u, v)| (Point2::new(u, v), 0.0))
            .collect(),
        y1 - y0,
        tol(),
    )
}

/// **P4. A bore that breaks through a support** (`u = 0.004 < a`): a
/// slot, not a cup. Whatever lies in the block inside `R` goes, so a
/// build lands on `1 − A`; a refusal is fine.
#[test]
fn p4_a_bore_breaking_through_a_support() {
    let (c, a) = ((0.004, 0.02), 0.006);
    let body = boolean(false, block(), tilted((-0.2, c), (0.5, c), a, convex_at));
    for verb in VERBS {
        let want = 1.0 - section(verb, D);
        let got = run(verb, &body, &[top(&body)], want, 0, "P4 slot");
        eprintln!("P4 {verb:?}: {got:?}");
    }
}

/// **P5. Two bores from the same end wall that meet inside** (one
/// feature, two mouths on one cap, a V): `plan` admits neither an
/// annulus between two caps nor a disk, so it refuses; a build would
/// land on `1 − A`.
#[test]
fn p5_two_bores_from_one_wall_meeting_inside() {
    let a = 0.003;
    let meet = (0.4, (0.03, 0.03));
    let tilted = |p, q, a, at| {
        let at: fn((f64, f64)) -> (f64, f64) = at;
        let ((xa, pa), (xb, qb)): ((f64, (f64, f64)), (f64, (f64, f64))) = (p, q);
        bar(
            (xa, at(pa)),
            (xb, at(qb)),
            0.0,
            Section::Square(a),
            Vec3::new(0.0, 1.0, 0.0),
        )
    };
    let b1 = tilted(
        (-0.2, lerp(-0.2, (0.0, (0.015, 0.045)), meet)),
        meet,
        a,
        convex_at,
    );
    let b2 = tilted(
        (-0.2, lerp(-0.2, (0.0, (0.045, 0.015)), meet)),
        meet,
        a,
        convex_at,
    );
    let body = boolean(false, boolean(false, block(), b1), b2);
    for verb in VERBS {
        let want = 1.0 - section(verb, D);
        let got = run(verb, &body, &[top(&body)], want, 0, "P5 V bores");
        eprintln!("P5 {verb:?}: {got:?}");
    }
}

/// **P6. A blind bore whose floor stops a hair before the far end wall**
/// (`x = 1 − 1e-4`): inside, so a build lands on `1 − A`; and one whose
/// floor sits `1e-4` past it is a through-bore, which also lands there.
#[test]
fn p6_a_floor_a_hair_from_the_far_wall() {
    let (c, a) = ((0.012, 0.012), 0.006);
    for end in [1.0 - 1e-4, 1.0 + 1e-4] {
        let body = boolean(false, block(), tilted((-0.2, c), (end, c), a, convex_at));
        for verb in VERBS {
            let want = 1.0 - section(verb, D);
            let got = run(
                verb,
                &body,
                &[top(&body)],
                want,
                0,
                "P6 floor near the far wall",
            );
            eprintln!("P6 end {end} {verb:?}: {got:?}");
        }
    }
}

/// **P7. The tilted blind bore under the chamfer** goes, its volume `π·a²·ℓ` with `ℓ` the axis.
#[test]
fn p7_the_tilted_chamfer_bore_closed_form() {
    let a = 0.003;
    let (m, f) = ((0.0, (0.005, 0.055)), (0.5, (0.055, 0.005)));
    let start = (-0.2, lerp(-0.2, m, f));
    let body = boolean(false, block(), tilted(start, f, a, convex_at));
    let (v0, _) = volume_enclosure(&body);
    let ell = 0.5 * (1.0f64 + 0.01 + 0.01).sqrt();
    let bore = PI * a * a * ell;
    assert!(
        (v0 - (1.0 - bore)).abs() < 1e-9,
        "P7: the source holds the bore's π·a²·ℓ: {v0} vs {}",
        1.0 - bore
    );
    let got = run(
        Verb::Chamfer,
        &body,
        &[top(&body)],
        1.0 - section(Verb::Chamfer, D),
        0,
        "P7",
    );
    assert_eq!(
        got,
        Got::Built,
        "P7: the chamfer's triangle holds the tilted bore"
    );
}
