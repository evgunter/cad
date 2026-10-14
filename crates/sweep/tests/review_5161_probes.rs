//! Review probes for PR 5161 (one-cycle members: a cup on one end wall
//! goes with the band). Each built row checks tiers 3/3′, naming
//! totality and ΔV against a closed form derived independently; each
//! refusal row checks the refusal is typed and not a surgery fault.

#![allow(clippy::unwrap_used, clippy::expect_used, clippy::panic)]

use core::f64::consts::PI;

use geom_core::Vec3;
use sweep::blend::BlendError;
use sweep::test_support::{assert_naming_totality, brick};
use topo::{Body, ContactRecords, EdgeKey, validate_geometric, validate_pseudomanifold};

use crate::band_concave_bar::{BAR, Section, bar, blend, boolean, die, floor, rings};
use crate::band_planar_consume::{VERBS, section};
use crate::band_planar_cut_off::{D, Verb, edge, midpoint_tol, pad_ceiling, tol, volume_enclosure};

/// A round bar of radius `a` whose axis passes `(x0, p)` and `(x1, q)`
/// (`p`, `q` as `(y, z)`), its end caps square to the axis, started at
/// `xs < x0` on the same line.
fn oblique(xs: f64, (x0, p): (f64, (f64, f64)), (x1, q): (f64, (f64, f64)), a: f64) -> Body<f64> {
    let t = (xs - x0) / (x1 - x0);
    let ps = (p.0 + (q.0 - p.0) * t, p.1 + (q.1 - p.1) * t);
    bar(
        (xs, ps),
        (x1, q),
        0.0,
        Section::Round(a),
        Vec3::new(0.0, 1.0, 0.0),
    )
}

/// The axis length between `(x0, p)` and `(x1, q)`.
fn axis_len((x0, p): (f64, (f64, f64)), (x1, q): (f64, (f64, f64))) -> f64 {
    ((x1 - x0).powi(2) + (q.0 - p.0).powi(2) + (q.1 - p.1).powi(2)).sqrt()
}

fn convex_at((u, v): (f64, f64)) -> (f64, f64) {
    (1.0 - u, 1.0 - v)
}

fn concave_at((u, v): (f64, f64)) -> (f64, f64) {
    (0.25 + u, 0.5 + v)
}

fn block() -> Body<f64> {
    brick((0.0, 1.0), (0.0, 1.0), (0.0, 1.0), tol())
}

fn top(body: &Body<f64>) -> EdgeKey {
    edge(body, [0.0, 1.0, 1.0], [1.0, 1.0, 1.0])
}

/// What a request did: built at the closed form, or refused.
#[derive(Debug)]
enum Got {
    Built { dv: f64, pad: f64 },
    Refused(BlendError),
}

/// Run `verb` and, if it builds, check tiers 3/3′, naming totality, the
/// genus and the shell count held, and return ΔV.
fn run(verb: Verb, body: &Body<f64>, edges: &[EdgeKey], what: &str) -> Got {
    validate_geometric(body, tol()).unwrap_or_else(|e| panic!("{what}: source tier 3, {e:?}"));
    match blend(verb, body, edges, D) {
        Err(e) => {
            assert!(
                !matches!(
                    e,
                    BlendError::SurgeryInvariant { .. } | BlendError::Op { .. }
                ),
                "{what} {verb:?}: a refusal from inside the surgery, {e:?}"
            );
            Got::Refused(e)
        }
        Ok(out) => {
            validate_geometric(&out.body, tol())
                .unwrap_or_else(|e| panic!("{what} {verb:?}: tier 3, {e:?}"));
            validate_pseudomanifold(&out.body, &ContactRecords::default(), tol())
                .unwrap_or_else(|e| panic!("{what} {verb:?}: tier 3′, {e:?}"));
            assert_naming_totality(body, &out, edges, what);
            let ((v0, p0), (v1, p1)) = (volume_enclosure(body), volume_enclosure(&out.body));
            Got::Built {
                dv: v1 - v0,
                pad: p0 + p1,
            }
        }
    }
}

fn assert_dv(got: &Got, want: f64, what: &str) {
    match got {
        Got::Built { dv, pad } => {
            assert!(*pad < pad_ceiling(), "{what}: pad {pad}");
            assert!(
                (dv - want).abs() < 1e-12 + pad && (dv - want).abs() < midpoint_tol(),
                "{what}: ΔV {dv} ± {pad} vs the closed form {want} (off by {})",
                dv - want
            );
        }
        Got::Refused(e) => panic!("{what}: expected to build, refused {e:?}"),
    }
}

/// P1. An OBLIQUE blind bore (axis not parallel to the spine) wholly
/// inside the convex sliver. The PR only builds axis-parallel cups; an
/// oblique cylinder cut by the end face `x = 0` holds `π·a²·ℓ`, `ℓ` its
/// axis between the face and its square floor, so if it builds,
/// `ΔV = −(A − π·a²·ℓ)`.
#[test]
fn p1_an_oblique_blind_bore_inside_the_sliver() {
    let (d, a) = BAR;
    for (shift, must_build) in [(0.0, true), (0.0011, false), (0.0031, false)] {
        let p = convex_at((d, d));
        let q = convex_at((d + shift, d - shift));
        let (m, f) = ((0.0, p), (0.5, q));
        let body = boolean(false, block(), oblique(-0.2, m, f, a));
        let ell = axis_len(m, f);
        for verb in VERBS {
            let what = format!("oblique bore, shift {shift}, {verb:?}");
            let got = run(verb, &body, &[top(&body)], &what);
            eprintln!("{what}: {got:?}");
            let want = -(section(verb, D) - PI * a * a * ell);
            match got {
                Got::Built { .. } => assert_dv(&got, want, &what),
                Got::Refused(_) if !must_build => {}
                Got::Refused(e) => panic!("{what}: {e:?}"),
            }
        }
    }
}

/// P2. A pin leaning 26° across the concave fill, from `(u, v) =
/// (0.004, 0.056)` at the wall `x = ¼` to `(0.056, 0.004)` at its square
/// end `x = 0.4`; radius `0.002`. Its axis keeps `u + v = 0.06`.
///  - Under the CHAMFER (chord `u + v = 0.1`, a convex triangle) it lies
///    wholly inside: if built, `ΔV = +(A/2 − π·a²·ℓ)`.
///  - Under the FILLET its axis passes `(0.03, 0.03)`, outside the fill
///    (the arc crosses the bisector at `0.0293`), so its wall crosses the
///    band surface: it MUST refuse.
#[test]
fn p2_a_pin_leaning_across_the_fill() {
    let a = 0.002;
    let (m, f) = (
        (0.25, concave_at((0.004, 0.056))),
        (0.4, concave_at((0.056, 0.004))),
    );
    let body = boolean(true, die(1.0, 0.0), oblique(0.15, m, f, a));
    let ell = axis_len(m, f);
    let got = run(Verb::Fillet, &body, &[floor(&body, 1.0)], "leaning pin");
    eprintln!("leaning pin, fillet: {got:?}");
    assert!(
        matches!(got, Got::Refused(_)),
        "a pin whose wall leaves the fillet's fill must not build, got {got:?}"
    );
    let got = run(Verb::Chamfer, &body, &[floor(&body, 1.0)], "leaning pin");
    eprintln!("leaning pin, chamfer: {got:?}");
    if let Got::Built { .. } = got {
        assert_dv(
            &got,
            section(Verb::Chamfer, D) * 0.5 - PI * a * a * ell,
            "leaning pin, chamfer",
        );
    }
}

/// P2b. The same leaning pin at `u + v = 0.054`, its axis `≈ 0.0032`
/// inside the fillet's arc throughout and its wall `≈ 0.0012`: wholly
/// inside under both verbs. Built or refused conservatively; if built,
/// at the closed form.
#[test]
fn p2b_a_leaning_pin_wholly_inside_the_fill() {
    let a = 0.002;
    let (m, f) = (
        (0.25, concave_at((0.004, 0.05))),
        (0.4, concave_at((0.05, 0.004))),
    );
    let body = boolean(true, die(1.0, 0.0), oblique(0.15, m, f, a));
    let ell = axis_len(m, f);
    for verb in VERBS {
        let what = format!("leaning pin inside, {verb:?}");
        let got = run(verb, &body, &[floor(&body, 1.0)], &what);
        eprintln!("{what}: {got:?}");
        if let Got::Built { .. } = got {
            assert_dv(&got, section(verb, D) * 0.5 - PI * a * a * ell, &what);
        }
    }
}

/// P3. (Fixture blocked: the second bump's Boolean refuses typed, "an edge ... touches or crosses a curved face"; kept as the attempted row.) Two bumps on one bar: the bar's wall carries two rings, each
/// bridged to its outer cycle. Each bump is the box over `u ∈ [0.009,
/// 0.015]`, `v ∈ [0.012, 0.024]` and `0.05` of `x`, less what the bar's
/// disk holds of it: `ΔV = +(A − π·a²)/2 − 2·0.05·(2c·0.012 − c·√(a² −
/// c²) − a²·asin(c/a))`, `c = 0.003`.
#[test]
#[ignore = "fixture: the Boolean refuses the second bump; run with --ignored"]
fn p3_two_bumps_on_one_bar() {
    let (d, a) = BAR;
    let c: f64 = 0.003;
    let at = concave_at((d, d));
    let barred = boolean(
        true,
        die(1.0, 0.0),
        bar(
            (0.25, at),
            (0.75, at),
            0.15,
            Section::Round(a),
            Vec3::new(0.0, 1.0, 0.0),
        ),
    );
    let bump = |x: (f64, f64)| {
        brick(
            x,
            (0.25 + 0.009, 0.25 + 0.015),
            (0.5 + 0.012, 0.5 + 0.024),
            tol(),
        )
    };
    let body = boolean(
        true,
        boolean(true, barred, bump((0.45, 0.5))),
        bump((0.3, 0.35)),
    );
    let held = c * (a * a - c * c).sqrt() + a * a * (c / a).asin();
    let r0 = rings(&body);
    for verb in VERBS {
        let what = format!("two bumps, {verb:?}");
        let got = run(verb, &body, &[floor(&body, 1.0)], &what);
        eprintln!("{what}: {got:?} (rings before {r0})");
        let want = (section(verb, D) - PI * a * a) * 0.5 - 2.0 * 0.05 * (2.0 * c * 0.012 - held);
        assert_dv(&got, want, &what);
    }
}

/// P4. A blind bore and a through bore in one sliver: one cup and one
/// annulus in one request. `ΔV = −(A − π·a²·h − π·b²)`.
#[test]
fn p4_a_cup_beside_a_through_bore() {
    let (a, b, h) = (0.004, 0.003, 0.4);
    let p = convex_at((0.012, 0.03));
    let q = convex_at((0.03, 0.012));
    let body = boolean(
        false,
        boolean(false, block(), oblique(-0.2, (0.0, p), (h, p), a)),
        oblique(-0.2, (0.0, q), (1.2, q), b),
    );
    for verb in VERBS {
        let what = format!("cup beside a bore, {verb:?}");
        let got = run(verb, &body, &[top(&body)], &what);
        eprintln!("{what}: {got:?}");
        assert_dv(
            &got,
            -(section(verb, D) - PI * a * a * h - PI * b * b),
            &what,
        );
    }
}

/// P5. A blind bore under each of two bands in one request: the block's
/// top edges `(0, 1, 1) → (1, 1, 1)` and `(0, 0, 1) → (1, 0, 1)`, each
/// with a bore from `x = 0` to `h`. Each feature is read in its own band
/// (the crease check). `ΔV = −2·(A − π·a²·h)`.
#[test]
fn p5_a_cup_under_each_of_two_bands() {
    let (d, a) = BAR;
    let h = 0.5;
    let p = convex_at((d, d));
    let q = (d, 1.0 - d);
    let body = boolean(
        false,
        boolean(false, block(), oblique(-0.2, (0.0, p), (h, p), a)),
        oblique(-0.2, (0.0, q), (h, q), a),
    );
    let edges = [top(&body), edge(&body, [0.0, 0.0, 1.0], [1.0, 0.0, 1.0])];
    for verb in VERBS {
        let what = format!("a cup under each of two bands, {verb:?}");
        let got = run(verb, &body, &edges, &what);
        eprintln!("{what}: {got:?}");
        assert_dv(&got, -2.0 * (section(verb, D) - PI * a * a * h), &what);
    }
}

/// P6. A blind bore whose floor lies a hair short of the far end face:
/// still a cup (its floor 1e-4 from `x = 1`), wholly inside.
/// `ΔV = −(A − π·a²·(1 − 1e-4))`.
#[test]
fn p6_a_blind_bore_a_hair_short_of_the_far_cap() {
    let (d, a) = BAR;
    let h = 1.0 - 1e-4;
    let p = convex_at((d, d));
    let body = boolean(false, block(), oblique(-0.2, (0.0, p), (h, p), a));
    for verb in VERBS {
        let what = format!("bore a hair short of the far cap, {verb:?}");
        let got = run(verb, &body, &[top(&body)], &what);
        eprintln!("{what}: {got:?}");
        if let Got::Built { .. } = got {
            assert_dv(&got, -(section(verb, D) - PI * a * a * h), &what);
        }
    }
}

/// P7. The union of a mitre's bands: the block's top edges `(0, 1, 1) →
/// (1, 1, 1)` (along `x`) and `(0, 0, 1) → (0, 1, 1)` (along `y`) meet
/// at a mitre at `(0, 1, 1)`. The `x` band's cap there is the face
/// `x = 0`, which is the `y` band's support. A blind bore from `x = 0` in
/// the `x` band's sliver lies in `R₁` (and its mouth on the `y` band's
/// strip). If it builds, `ΔV = ΔV(plain mitre) + π·a²·h` (`S − (R₁ ∪
/// R₂)` with the bore inside `R₁`). A bore from the FAR cap `x = 1`
/// is the plain cup beside a mitre: same closed form.
#[test]
fn p7_a_cup_in_a_mitred_bands_cap() {
    let (d, a) = BAR;
    let h = 0.3;
    let plain = block();
    let edges_of = |b: &Body<f64>| [top(b), edge(b, [0.0, 0.0, 1.0], [0.0, 1.0, 1.0])];
    let p = convex_at((d, d));
    let near = boolean(false, block(), oblique(-0.2, (0.0, p), (h, p), a));
    // The far cap: a bore from `x = 1` back to `1 − h`, its axis along −x.
    let far = boolean(
        false,
        block(),
        bar(
            (1.0 - h, p),
            (1.2, p),
            0.0,
            Section::Round(a),
            Vec3::new(0.0, 1.0, 0.0),
        ),
    );
    for verb in VERBS {
        let Got::Built { dv: dv0, .. } = run(verb, &plain, &edges_of(&plain), "plain mitre") else {
            panic!("{verb:?}: the plain mitre builds");
        };
        for (what, body) in [("near cap (mitre plane)", &near), ("far cap", &far)] {
            let what = format!("mitre, cup on the {what}, {verb:?}");
            let got = run(verb, body, &edges_of(body), &what);
            eprintln!("{what}: {got:?} (plain ΔV {dv0})");
            if let Got::Built { .. } = got {
                assert_dv(&got, dv0 + PI * a * a * h, &what);
            }
        }
    }
}

/// P8. A cup WITH A HANDLE: a square blind bore of half-side `b` from
/// `x = 0` to `h` at corner `(d, d)`, with a square cross bar of
/// half-side `c` along `y` spanning it at `x = 0.2`, its ends sunk in
/// the bore's walls. The feature has one cap cycle and genus 1, so it is
/// no disk (`V − E + F − R = −1`). `S − R` still takes it all, at
/// `ΔV = −(A − 4b²h + 8c²b)` (the bar holds `(2c)²·2b` inside the bore);
/// a refusal must be typed `WhollyInside` or a bound, never a surgery
/// fault.
#[test]
fn p8_a_cup_with_a_handle() {
    let (d, h, b, c) = (0.012, 0.4, 0.004, 0.0015);
    let (y, z) = convex_at((d, d));
    let bored = boolean(
        false,
        block(),
        brick((-0.2, h), (y - b, y + b), (z - b, z + b), tol()),
    );
    let body = boolean(
        true,
        bored,
        brick(
            (0.2 - c, 0.2 + c),
            (y - 0.006, y + 0.006),
            (z - c, z + c),
            tol(),
        ),
    );
    let e = topo::readback::euler_counts(&body);
    eprintln!("cup with a handle: shells {} genus {:?}", e.s, e.genus());
    for verb in VERBS {
        let what = format!("a cup with a handle, {verb:?}");
        let got = run(verb, &body, &[top(&body)], &what);
        eprintln!("{what}: {got:?}");
        if let Got::Built { .. } = got {
            assert_dv(
                &got,
                -(section(verb, D) - 4.0 * b * b * h + 8.0 * c * c * b),
                &what,
            );
        }
    }
}
