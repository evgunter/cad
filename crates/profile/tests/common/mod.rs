//! Shared fixtures and helpers for the profile integration suites.
//!
//! Fixtures are built at `f64` and lifted to other scalars via
//! [`lift`]; geometry that must sit at an ε-relative margin takes the
//! run's ε explicitly so the multi-ε CI rows genuinely re-exercise the
//! bands. The run tolerance comes from `Tol::witness().get()` — one read
//! per test process, and the crate's suites all run inside the one
//! aggregated `all` binary, so the geom-core global-state discipline is
//! satisfied by a single process-wide read.
#![allow(dead_code)] // loaded once per consumer; each uses a subset
#![allow(unreachable_pub)] // why: root Cargo.toml, the `unreachable_pub` stanza
#![allow(clippy::unwrap_used, clippy::expect_used, clippy::panic)]

use geom_core::Tol;
use geom_core::{Point2, Real};
use profile::RawLoop;
use profile::{ClosedLoop, Open, Profile, ProfileLoop, ProfileVertex, SketchPlane, Start};

/// The run's tolerance (env-driven; the multi-ε matrix parameterizes
/// it).
pub fn tol() -> Tol {
    Tol::witness()
}

/// tan(π/8) = √2 − 1: the bulge of a counterclockwise quarter-circle
/// arc.
pub fn quarter_bulge() -> f64 {
    std::f64::consts::SQRT_2 - 1.0
}

/// Lifts an `f64` profile to any scalar (exact embedding per
/// `Real::from_f64`).
pub fn lift<T: Real>(p: &Profile<f64>) -> Profile<T> {
    Profile::new(
        SketchPlane::xy(),
        p.loops
            .iter()
            .map(|lp| {
                ProfileLoop::new(
                    lp.vertices()
                        .iter()
                        .map(|v| {
                            ProfileVertex::new(
                                Point2::new(T::from_f64(v.pos().x), T::from_f64(v.pos().y)),
                                T::from_f64(v.bulge()),
                            )
                        })
                        .collect(),
                )
                .with_tangent_joints(lp.tangent_joints().to_vec())
            })
            .collect(),
    )
}

/// A loop from `(x, y, bulge)` triples.
pub fn chain(vs: &[(f64, f64, f64)]) -> ProfileLoop<f64> {
    ProfileLoop::new(
        vs.iter()
            .map(|&(x, y, bulge)| ProfileVertex::new(Point2::new(x, y), bulge))
            .collect(),
    )
}

/// A single-loop profile on the world xy-plane.
pub fn profile(loops: Vec<ProfileLoop<f64>>) -> Profile<f64> {
    Profile::new(SketchPlane::xy(), loops)
}

/// An axis-aligned rectangle, counterclockwise from `(x0, y0)`.
pub fn rect(x0: f64, y0: f64, w: f64, h: f64) -> ProfileLoop<f64> {
    ProfileLoop::polygon([
        Point2::new(x0, y0),
        Point2::new(x0 + w, y0),
        Point2::new(x0 + w, y0 + h),
        Point2::new(x0, y0 + h),
    ])
}

/// The L-profile (counterclockwise hexagon).
pub fn l_profile() -> ProfileLoop<f64> {
    ProfileLoop::polygon([
        Point2::new(0.0, 0.0),
        Point2::new(2.0, 0.0),
        Point2::new(2.0, 1.0),
        Point2::new(1.0, 1.0),
        Point2::new(1.0, 2.0),
        Point2::new(0.0, 2.0),
    ])
}

/// A circle as two counterclockwise semicircular arcs, split
/// horizontally (vertices at (cx ± r, cy)).
pub fn circle_h(cx: f64, cy: f64, r: f64) -> ProfileLoop<f64> {
    chain(&[(cx - r, cy, 1.0), (cx + r, cy, 1.0)])
}

/// A circle as two counterclockwise semicircular arcs, split
/// vertically (vertices at (cx, cy ± r)) — used when a fixture needs
/// the points (cx ± r, cy) free of vertices.
pub fn circle_v(cx: f64, cy: f64, r: f64) -> ProfileLoop<f64> {
    chain(&[(cx, cy - r, 1.0), (cx, cy + r, 1.0)])
}

/// A rounded rectangle: straight sides, counterclockwise quarter-arc
/// corners of radius `r`.
pub fn rounded_rect(w: f64, h: f64, r: f64) -> ProfileLoop<f64> {
    let b = quarter_bulge();
    let mut lp = chain(&[
        (r, 0.0, 0.0),
        (w - r, 0.0, b),
        (w, r, 0.0),
        (w, h - r, b),
        (w - r, h, 0.0),
        (r, h, b),
        (0.0, h - r, 0.0),
        (0.0, r, b),
    ]);
    // Every joint is an exact quarter-arc/side tangency — declared
    // (the #101 discipline).
    let n = lp.vertices().len();
    lp = lp.with_tangent_joints((0..n).collect());
    lp
}

/// The demo bracket's filleted-corner shape: an L with one r = 0.5
/// tangent fillet, authored through the PATHS algebra (which declares
/// its two joints by construction) — the mixed declared/undeclared
/// fixture (2 tangent joints of 7).
///
/// The fillet's corner is never authored: the incoming ray leaves
/// (3, 1) toward −x, the arrival side runs +y and ends at its own
/// anchor (1, 3), and the r = 0.5 arc is inserted at the carriers'
/// intersection, trimming both to T₁ = (1.5, 1) and T₂ = (1, 1.5).
pub fn bracket() -> ProfileLoop<f64> {
    let closed = Open
        .at(Point2::new(0.0, 0.0))
        .line_to(Point2::new(3.0, 0.0), Tol::witness())
        .expect("bottom side")
        .line_to(Point2::new(3.0, 1.0), Tol::witness())
        .expect("right side")
        .toward(-1.0, 0.0, Tol::witness())
        .expect("the incoming ray leaves (3, 1) toward −x")
        .fillet(0.5, Tol::witness())
        .expect("r = 0.5 is a positive radius")
        .toward(0.0, 1.0, Tol::witness())
        .expect("the arrival side runs +y")
        .to(Point2::new(1.0, 3.0), Tol::witness())
        .expect("the bracket fillet fits both legs")
        .line_to(Point2::new(0.0, 3.0), Tol::witness())
        .expect("top side")
        .line_to(Start, Tol::witness())
        .expect("the straight seam closes");
    pinned(closed)
}

/// A lens (lune): a semicircular arc out and a shallower arc back —
/// the legal two-vertex loop with distinct carriers.
pub fn lens() -> ProfileLoop<f64> {
    // Out: counterclockwise semicircle (apex below the chord);
    // back: clockwise quarter arc (apex also below, shallower).
    chain(&[(0.0, 0.0, 1.0), (2.0, 0.0, -quarter_bulge())])
}

/// The annulus: circle outer + concentric circle hole.
pub fn annulus() -> Profile<f64> {
    profile(vec![circle_h(0.0, 0.0, 2.0), circle_h(0.0, 0.0, 1.0)])
}

/// Bowtie: a self-crossing quadrilateral (segments 0 and 2 cross).
pub fn bowtie() -> ProfileLoop<f64> {
    ProfileLoop::polygon([
        Point2::new(0.0, 0.0),
        Point2::new(1.0, 1.0),
        Point2::new(1.0, 0.0),
        Point2::new(0.0, 1.0),
    ])
}

/// Internally tangent circle-in-circle hole: exact tangency at (1, 0),
/// interior to arcs of both loops (both circles split vertically).
pub fn tangent_hole() -> Profile<f64> {
    profile(vec![circle_v(0.0, 0.0, 1.0), circle_v(0.5, 0.0, 0.5)])
}

/// An arc kissing a line: the top arc of this profile dips down and
/// touches the bottom segment tangentially at (2, 0), interior to
/// both.
pub fn arc_kisses_line() -> Profile<f64> {
    profile(vec![chain(&[
        (0.0, 0.0, 0.0),
        (4.0, 0.0, 0.0),
        (4.0, 3.0, 0.0),
        (3.0, 3.0, -3.0),
        (1.0, 3.0, 0.0),
        (0.0, 3.0, 0.0),
    ])])
}

/// A near-tangent circle-in-circle hole: the internal clearance
/// d − |r₁ − r₂| is exactly −5ε — inside the ambiguity band (ε, Kε), so
/// validation must escalate rather than guess.
pub fn near_tangent_hole(eps: f64) -> Profile<f64> {
    profile(vec![
        circle_v(0.0, 0.0, 1.0),
        circle_v(0.5 - 5.0 * eps, 0.0, 0.5),
    ])
}

/// **The v2 differential pin** (LIB-SWITCH §3d, PROFILES-V2 §V1): the
/// program a chain RECORDED as it lowered replays — through the driver,
/// hence through the same typed binders — to a BIT-IDENTICAL loop.
///
/// Every closing verb in the corpus funnels through here, so the pin
/// covers every typed chain the suites author rather than a sampled
/// subset: wrap the chain's result and keep asserting whatever the test
/// was already asserting on the loop.
pub fn pinned(closed: ClosedLoop<f64>) -> ProfileLoop<f64> {
    let replayed = match profile::replay(&closed.program, Tol::witness()) {
        Ok(lp) => lp,
        Err(e) => panic!("the recorded program refused at replay: {e}"),
    };
    assert_bit_identical(&closed.loop_, &replayed);
    closed.loop_
}

/// Bit-level loop identity: vertex count, every coordinate and bulge by
/// `to_bits`, and the declared joints as a MULTISET.
///
/// Order is not semantic, so the lists are sorted — but they are NOT
/// deduped: the two sides here come from the same emission machinery
/// driven two ways, so a replay that declared one joint twice where the
/// lowering declared it once is a real divergence, and deduping would
/// hide it. (The looser set-compare belongs in `path_differential.rs`,
/// where the two sides are the algebra and the hand builder.)
pub fn assert_bit_identical(lowered: &ProfileLoop<f64>, replayed: &ProfileLoop<f64>) {
    assert_eq!(
        lowered.vertices().len(),
        replayed.vertices().len(),
        "vertex count: lowered vs replayed"
    );
    for (i, (a, b)) in lowered
        .vertices()
        .iter()
        .zip(replayed.vertices())
        .enumerate()
    {
        assert_eq!(a.pos().x.to_bits(), b.pos().x.to_bits(), "vertex {i} x");
        assert_eq!(a.pos().y.to_bits(), b.pos().y.to_bits(), "vertex {i} y");
        assert_eq!(a.bulge().to_bits(), b.bulge().to_bits(), "vertex {i} bulge");
    }
    let mut la = lowered.tangent_joints().to_vec();
    let mut lb = replayed.tangent_joints().to_vec();
    la.sort_unstable();
    lb.sort_unstable();
    assert_eq!(la, lb, "declared tangent joints (multiset)");
}
