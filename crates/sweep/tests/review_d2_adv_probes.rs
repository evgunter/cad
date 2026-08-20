//! **Adversarial falsification probes for D2** (PR #740 — the fillet
//! catch-all partitioned across the D2 addendum's rows 1/2/4).
//!
//! The unit's headline claim is that **none of the 18 `unreachable!`
//! sites it introduced is reachable from the public API on any
//! input** — which is what keeps D9's "the kernel never panics on any
//! input" literally true. A proof-by-inspection of 18 local arguments
//! is exactly the shape that a counterexample search can attack from
//! the other side, so these rows drive the ONE public door
//! (`fillet_edges`) with adversarial bodies and adversarial edge sets
//! and assert that every outcome is a value — `Ok` or a typed `Err`,
//! never a panic.
//!
//! Shapes, per `memories/test-suite-cost.md`:
//!
//! - `d2_no_input_reaches_a_panic` is a **counterexample search**
//!   (*for all sampled requests, the door returns a value*): the seed
//!   varies, is logged unconditionally, and the count rides the
//!   `D2_ADV_EFFORT` dial. Cutting the count loses detection power,
//!   never correctness.
//! - `d2_the_battery_never_hands_the_surgery_an_empty_chain` is a
//!   **witness-free property** over the same corpus: it pins the
//!   reason `FilletError::EmptyChain` has no input witness, which is
//!   what the PR's row-1 classification of those two sites asserts.
//! - `d2_reached_variants` asserts nothing about coverage and is
//!   therefore **evidence, not a gate** — it prints which refusal
//!   classes the corpus actually reaches, which is the number a
//!   future reader of the partition wants.
//!
//! Gating: these are written against `crates/sweep/src/fillet`; run
//! them when that directory changes.

#![allow(clippy::unwrap_used, clippy::expect_used, clippy::panic, dead_code)]

use core::f64::consts::PI;

use geom_core::{Affine3, Band, Point2, Tolerance, Vec2, Vec3};
use profile::{Profile, ProfileLoop, ProfileVertex, RawLoop, SketchPlane};
use sweep::fillet::{FilletError, FilletRequest, fillet_edges, run_battery};
use sweep::test_support::cube;
use sweep::{Revolution, RevolveAxis, revolve};
use topo::boolean::{BooleanOp, SweepStrategy, boolean_op_with};
use topo::{Body, BooleanDeclarations, EdgeKey};

/// How many random edge subsets each corpus body gets. One multiple of
/// the dial per body; raise `D2_ADV_EFFORT` to buy depth.
fn effort() -> usize {
    std::env::var("D2_ADV_EFFORT")
        .ok()
        .and_then(|s| s.parse().ok())
        .unwrap_or(24)
}

/// A fresh seed every run (never a literal — the standing rule), with
/// an env override so a red run replays exactly.
fn seed() -> u64 {
    std::env::var("D2_ADV_SEED")
        .ok()
        .and_then(|s| s.parse().ok())
        .unwrap_or_else(|| {
            use std::time::{SystemTime, UNIX_EPOCH};
            let t = SystemTime::now().duration_since(UNIX_EPOCH).unwrap();
            u64::from(t.subsec_nanos()) ^ (t.as_secs() << 17) ^ 0x9E37_79B9_7F4A_7C15
        })
}

/// splitmix64 — a generator written out rather than pulled in, so the
/// row has no dependency of its own.
struct Rng(u64);
impl Rng {
    fn next_u64(&mut self) -> u64 {
        self.0 = self.0.wrapping_add(0x9E37_79B9_7F4A_7C15);
        let mut z = self.0;
        z = (z ^ (z >> 30)).wrapping_mul(0xBF58_476D_1CE4_E5B9);
        z = (z ^ (z >> 27)).wrapping_mul(0x94D0_49BB_1331_11EB);
        z ^ (z >> 31)
    }
    fn below(&mut self, n: usize) -> usize {
        if n == 0 {
            0
        } else {
            (self.next_u64() % n as u64) as usize
        }
    }
}

fn band() -> Band {
    let tol = Tolerance::get();
    Band::new(tol.eps, tol.k * tol.eps).unwrap()
}

fn p2(x: f64, y: f64) -> Point2<f64> {
    Point2::new(x, y)
}

fn ball_at(r: f64, c: Vec3<f64>) -> Body<f64> {
    let lp = ProfileLoop::new(vec![
        ProfileVertex::new(p2(0.0, -r), 1.0),
        ProfileVertex::new(p2(0.0, r), 0.0),
    ]);
    let vp = Profile::new(SketchPlane::xy(), vec![lp])
        .validate(Tolerance::get())
        .unwrap();
    let axis = RevolveAxis {
        origin: p2(0.0, 0.0),
        dir: Vec2::new(0.0, 1.0),
    };
    let ball = revolve(&vp, axis, Revolution::Full).unwrap().body;
    topo::transform_rigid(&ball, &Affine3::translation(c)).unwrap()
}

fn ball_top(r: f64, c: Vec3<f64>) -> Body<f64> {
    let ball = ball_at(r, Vec3::new(0.0, 0.0, 0.0));
    let rot = topo::transform_rigid(
        &ball,
        &Affine3::rotation_about_axis(
            geom_core::Point3::new(0.0, 0.0, 0.0),
            Vec3::new(1.0, 0.0, 0.0),
            PI / 2.0,
        ),
    )
    .unwrap();
    topo::transform_rigid(&rot, &Affine3::translation(c)).unwrap()
}

fn subtract(a: &Body<f64>, b: &Body<f64>) -> Option<Body<f64>> {
    let out = boolean_op_with(
        BooleanOp::Subtract,
        a,
        b,
        &BooleanDeclarations::none(),
        SweepStrategy::Realized,
    )
    .ok()?;
    Some(out.body()?.body.clone())
}

fn union(a: &Body<f64>, b: &Body<f64>) -> Option<Body<f64>> {
    let out = boolean_op_with(
        BooleanOp::Union,
        a,
        b,
        &BooleanDeclarations::none(),
        SweepStrategy::Realized,
    )
    .ok()?;
    Some(out.body()?.body.clone())
}

/// The corpus: bodies whose provenance is deliberately varied —
/// primitives, revolves, boolean results, transplanted multi-solid
/// bodies, and rigidly-moved copies. The charter's list: degenerate
/// chains, self-intersecting rims, apex-level corners, repeated and
/// reversed edges, coincident vertices, and bodies from booleans
/// rather than primitives.
fn corpus() -> Vec<(&'static str, Body<f64>)> {
    let mut out: Vec<(&'static str, Body<f64>)> = Vec::new();
    let c = cube(1.0);
    out.push(("cube", c.clone()));

    // A rigidly rotated cube — the same topology on a different frame,
    // so no predicate can be riding an axis-aligned accident.
    if let Ok(t) = topo::transform_rigid(
        &c,
        &Affine3::rotation_about_axis(
            geom_core::Point3::new(0.0, 0.0, 0.0),
            Vec3::new(1.0, 1.0, 1.0).normalize(),
            0.7,
        ),
    ) {
        out.push(("cube_rotated", t));
    }

    // A sphere: no plane–plane link anywhere, and a POLE — the
    // apex-level corner the charter names.
    out.push(("ball", ball_at(0.4, Vec3::new(0.0, 0.0, 0.0))));

    // Boolean-minted bodies. `pip` is the shallow spherical cap the
    // rim arm is built for (a whole circular plane-sphere ring); the
    // others deform it until it is not.
    let pip = |cx: f64, cy: f64, r: f64, h: f64| ball_top(r, Vec3::new(cx, cy, 1.0 + (r - h)));
    if let Some(b) = subtract(&c, &pip(0.5, 0.5, 0.09, 0.05)) {
        out.push(("die_one_pip", b.clone()));
        // A body the SURGERY itself minted, filleted again: its faces
        // and carriers are the surgery's own output, not a primitive's.
        let box_edges: Vec<EdgeKey> = cube(1.0)
            .edges()
            .map(|(k, _)| k)
            .filter(|k| b.get_edge(*k).is_some())
            .collect();
        if let Ok(f) = fillet_edges(&b, &box_edges, 0.12, band()) {
            out.push(("die_one_pip_blended", f.body));
        }
        if let Some(b2) = subtract(&b, &pip(0.25, 0.25, 0.09, 0.05)) {
            out.push(("die_two_pips", b2));
        }
    }
    // A pip straddling an edge: its rim crosses two support faces, so
    // the closed chain is not a ring of one plane.
    if let Some(b) = subtract(&c, &pip(0.5, 0.0, 0.12, 0.06)) {
        out.push(("die_edge_straddling_pip", b));
    }
    // A pip at a CORNER: coincident-ish rim and corner features.
    if let Some(b) = subtract(&c, &pip(0.0, 0.0, 0.12, 0.06)) {
        out.push(("die_corner_pip", b));
    }
    // A boss: material added, so the ring's links are CONCAVE.
    if let Some(b) = union(&c, &pip(0.5, 0.5, 0.2, 0.1)) {
        out.push(("boss_union", b));
    }

    // A TWO-SOLID body through the public transplant door, and the
    // door's own destination after it has been written into twice —
    // the shape the PR's row-1 refutation is about.
    let mut dst = c.clone();
    if topo::instance::graft_disjoint_all(&mut dst, &cube(0.5)).is_ok() {
        out.push(("grafted_two_solid", dst.clone()));
        let mut again = dst.clone();
        if topo::instance::graft_disjoint_all(&mut again, &ball_at(0.3, Vec3::new(9.0, 0.0, 0.0)))
            .is_ok()
        {
            out.push(("grafted_three_solid", again));
        }
    }
    out
}

/// Every request shape worth firing at one body: the empty set, each
/// singleton, each adjacent pair, the whole set, a repeated edge, a
/// reversed whole set, and `effort()` random subsets.
fn requests(body: &Body<f64>, rng: &mut Rng, n: usize) -> Vec<Vec<EdgeKey>> {
    let all: Vec<EdgeKey> = body.edges().map(|(k, _)| k).collect();
    let mut out: Vec<Vec<EdgeKey>> = vec![Vec::new(), all.clone()];
    let mut rev = all.clone();
    rev.reverse();
    out.push(rev);
    if let Some(&e) = all.first() {
        out.push(vec![e]);
        out.push(vec![e, e]);
    }
    if let Some(&e) = all.last() {
        out.push(vec![e]);
    }
    for w in all.windows(2).take(6) {
        out.push(w.to_vec());
    }
    // The RIM arm's own door: the plane-sphere edges (the pip rims)
    // on their own, and the circle-carried edges. Without these the
    // sample never reaches `rim_phase`, which holds 6 of the 18
    // `unreachable!` sites.
    let kind_of = |f: topo::FaceKey| -> Option<u8> {
        match body.get_surface(body.get_face(f)?.surface)? {
            geom::Surface::Plane { .. } => Some(0),
            geom::Surface::Sphere { .. } => Some(1),
            _ => Some(2),
        }
    };
    let face_of = |he: topo::HalfEdgeKey| -> Option<topo::FaceKey> {
        Some(body.get_loop(body.get_half_edge(he)?.parent_loop)?.face)
    };
    let rims: Vec<EdgeKey> = body
        .edges()
        .filter(|(_, e)| {
            let (Some(fa), Some(fb)) = (face_of(e.he_plus), face_of(e.he_minus)) else {
                return false;
            };
            let mut kinds = [kind_of(fa), kind_of(fb)];
            kinds.sort_unstable();
            kinds == [Some(0), Some(1)]
        })
        .map(|(k, _)| k)
        .collect();
    if !rims.is_empty() {
        out.push(rims.clone());
        if rims.len() > 1 {
            out.push(rims[..1].to_vec());
            out.push(rims[1..].to_vec());
        }
    }
    let circles: Vec<EdgeKey> = all
        .iter()
        .copied()
        .filter(|&k| {
            body.get_edge(k)
                .and_then(|e| body.get_curve_geom(e.curve))
                .and_then(|g| g.certified())
                .is_some_and(|c| matches!(c.carrier(), geom::Curve3::Circle { .. }))
        })
        .collect();
    if !circles.is_empty() {
        out.push(circles.clone());
        let straights: Vec<EdgeKey> = all
            .iter()
            .copied()
            .filter(|k| !circles.contains(k))
            .collect();
        out.push(straights.clone());
        let mut both = straights;
        both.extend(circles.iter().copied());
        out.push(both);
    }
    for _ in 0..n {
        let k = 1 + rng.below(all.len().max(1));
        let mut pick: Vec<EdgeKey> = Vec::with_capacity(k);
        for _ in 0..k {
            if let Some(&e) = all.get(rng.below(all.len().max(1))) {
                // Deduped: a repeated edge refuses at the door before
                // the battery, so leaving duplicates in would spend the
                // whole sample on `RepeatedEdge`. The repeat is covered
                // by its own explicit row above.
                if !pick.contains(&e) {
                    pick.push(e);
                }
            }
        }
        out.push(pick);
    }
    out
}

/// The class name of a refusal — the partition's own vocabulary, so
/// the evidence row reads as the addendum's rows.
fn class(e: &FilletError) -> &'static str {
    match e {
        FilletError::Band(_) => "Band",
        FilletError::ChainNotConnected { .. } => "ChainNotConnected",
        FilletError::RadiusHeadroom { .. } => "RadiusHeadroom",
        FilletError::FaceClearanceUncertified { .. } => "FaceClearanceUncertified",
        FilletError::TangentialEdge { .. } => "TangentialEdge",
        FilletError::SpineIrregular { .. } => "SpineIrregular",
        FilletError::ChainNotG1 { .. } => "ChainNotG1",
        FilletError::ConvexitySignFlip { .. } => "ConvexitySignFlip",
        FilletError::FilletCornerUnsupported { .. } => "FilletCornerUnsupported",
        FilletError::SpineUnsupported { .. } => "SpineUnsupported",
        FilletError::Escalated { .. } => "Escalated",
        FilletError::RepeatedEdge { .. } => "RepeatedEdge(row 1)",
        FilletError::UnsupportedBody { .. } => "UnsupportedBody(row 2)",
        FilletError::UnsupportedChain { .. } => "UnsupportedChain(row 2)",
        FilletError::UnsupportedRunOut { .. } => "UnsupportedRunOut(row 2)",
        FilletError::UnsupportedGeometry { .. } => "UnsupportedGeometry(row 2)",
        FilletError::EmptyChain => "EmptyChain(row 1)",
        FilletError::BodyNotIntact { .. } => "BodyNotIntact(row 1)",
        FilletError::RingClearance { .. } => "RingClearance",
        FilletError::Certify { .. } => "Certify",
        FilletError::Op { .. } => "Op",
    }
}

/// Radii spanning the scale: below, at, and above the features.
const RADII: [f64; 6] = [1e-6, 1e-3, 0.02, 0.05, 0.12, 0.9];

/// **The headline row.** Every request the corpus can express must
/// come back as a VALUE. A panic here is a reachable `unreachable!`
/// (or an index) on the public path, which falsifies both the unit's
/// central claim and D9's headline bullet.
#[test]
fn d2_no_input_reaches_a_panic() {
    let s = seed();
    println!(
        "d2_no_input_reaches_a_panic: D2_ADV_SEED={s} D2_ADV_EFFORT={}",
        effort()
    );
    for (n, b) in corpus() {
        println!(
            "    corpus: {n} — {} edges, {} solids",
            b.edges().count(),
            b.solids().count()
        );
    }
    let mut rng = Rng(s);
    let hook = std::panic::take_hook();
    std::panic::set_hook(Box::new(|_| {}));
    let mut fired: Vec<String> = Vec::new();
    let mut calls = 0usize;
    for (name, body) in corpus() {
        for req in requests(&body, &mut rng, effort()) {
            for r in RADII {
                calls += 1;
                let outcome = std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| {
                    let _ = fillet_edges(&body, &req, r, band());
                }));
                if outcome.is_err() {
                    fired.push(format!(
                        "{name}: {} edge(s), radius {r} — seed {s}",
                        req.len()
                    ));
                }
            }
        }
    }
    std::panic::set_hook(hook);
    assert!(
        fired.is_empty(),
        "the fillet door panicked on {} of {calls} requests (seed {s}): {fired:#?}",
        fired.len()
    );
    println!("d2_no_input_reaches_a_panic: {calls} requests, every one a value");
}

/// **The row-1 premise for `EmptyChain`.** Both of its two sites read
/// `chain.links` off a verdict the SAME public call produced one frame
/// up. If `run_battery` can never hand out a chain with no links, the
/// variant has no input witness — which is what the PR's split turns
/// on, and what a future reader of the partition needs pinned.
#[test]
fn d2_the_battery_never_hands_the_surgery_an_empty_chain() {
    let s = seed();
    println!("d2_the_battery_never_hands_the_surgery_an_empty_chain: D2_ADV_SEED={s}");
    let mut rng = Rng(s);
    let mut verdicts = 0usize;
    for (name, body) in corpus() {
        for edges in requests(&body, &mut rng, effort()) {
            if edges.is_empty() {
                continue;
            }
            for r in RADII {
                let req = FilletRequest {
                    body: &body,
                    edges: edges.clone(),
                    radius: r,
                };
                let Ok(v) = run_battery(&req, band()) else {
                    continue;
                };
                verdicts += 1;
                for (i, chain) in v.chains.iter().enumerate() {
                    assert!(
                        !chain.links.is_empty(),
                        "{name}: the battery emitted chain {i} with no links \
                         (radius {r}, seed {s}) — `FilletError::EmptyChain` would \
                         then have an input witness"
                    );
                }
            }
        }
    }
    println!(
        "d2_the_battery_never_hands_the_surgery_an_empty_chain: {verdicts} verdicts, \
         no empty chain"
    );
}

/// EVIDENCE, not a gate (it asserts nothing about coverage): which
/// refusal classes the corpus reaches. A reader checking the
/// partition wants to know that the row-2 names and the row-1 names
/// are not equally reachable — and, in particular, whether
/// `EmptyChain` ever appears.
#[test]
fn d2_reached_variants() {
    let s = seed();
    println!("d2_reached_variants: D2_ADV_SEED={s}");
    let mut rng = Rng(s);
    let mut seen: Vec<(&'static str, usize)> = Vec::new();
    let mut ok = 0usize;
    for (_, body) in corpus() {
        for req in requests(&body, &mut rng, effort()) {
            for r in RADII {
                match fillet_edges(&body, &req, r, band()) {
                    Ok(_) => ok += 1,
                    Err(e) => {
                        let c = class(&e);
                        match seen.iter_mut().find(|(k, _)| *k == c) {
                            Some((_, n)) => *n += 1,
                            None => seen.push((c, 1)),
                        }
                    }
                }
            }
        }
    }
    seen.sort_unstable();
    println!("d2_reached_variants: {ok} successes; refusals by class:");
    for (c, n) in &seen {
        println!("    {n:>6}  {c}");
    }
}

/// **The row-1 refutation's witness, attacked at the door.** The PR
/// classifies 46 sites as `BodyNotIntact` — reachable-but-invalid
/// rather than kernel bug — on the strength of ONE named public door:
/// `topo::instance::graft_disjoint_all`, whose docs say a refusal
/// raised mid-transplant leaves the destination *spent, never
/// resumable*, so a caller who discards the `Err` hands `fillet_edges`
/// a tier-1-invalid body with no kernel bug in the trace.
///
/// All 46 sites sit BELOW `fillet_surgery`'s entry gate
/// (`solids != 1 || shells != 1` — `surgery.rs:212`). This row pins
/// the arithmetic that decides whether the witness can get there in
/// the scenario the refutation describes — *a caller keeps the body it
/// already had*: a graft ADDS a solid (`graft_disjoint_all_keyed`
/// mints one empty destination solid per source solid before any
/// fallible step) or a shell (`graft_disjoint_all_onto_keyed`), so a
/// destination that already held a solid never presents as one solid
/// with one shell afterwards, spent or whole, and the door refuses
/// `UnsupportedBody` above all 46 sites.
///
/// It does NOT close the question: a graft into an EMPTY destination
/// that failed mid-transplant would leave one solid and one shell and
/// would pass the gate. No such failure is reachable today —
/// `graft_disjoint_all` runs `Bridge::RemapKeys`, which returns before
/// the `GraftRecertify` its own docs name (`combine.rs:357`), and every
/// remaining `JoinDesync` path needs an already-corrupt source — but
/// that is a fact about today's `combine`, not a fact this row pins.
#[test]
fn d2_a_grafted_destination_is_stopped_at_the_entry_gate() {
    let base = cube(1.0);
    let edges: Vec<EdgeKey> = base.edges().map(|(k, _)| k).collect();
    // The whole-body fillet succeeds before the graft: the request is
    // inside the front door, so any refusal below is the graft's doing
    // and not the request's.
    assert!(
        fillet_edges(&base, &edges, 0.12, band()).is_ok(),
        "the control request must pass, or this row proves nothing"
    );

    let mut dst = base.clone();
    topo::instance::graft_disjoint_all(&mut dst, &cube(0.5)).expect("a disjoint graft");
    assert!(
        dst.solids().count() > 1 || dst.shells().count() > 1,
        "a graft that added nothing countable cannot be the witness the \
         refutation needs"
    );
    let after: Vec<EdgeKey> = edges
        .iter()
        .copied()
        .filter(|k| dst.get_edge(*k).is_some())
        .collect();
    match fillet_edges(&dst, &after, 0.12, band()) {
        Err(FilletError::UnsupportedBody { solids, shells }) => {
            println!(
                "d2_a_grafted_destination_is_stopped_at_the_entry_gate: \
                 {solids} solid(s), {shells} shell(s) — refused at surgery.rs:212, \
                 above all 46 `BodyNotIntact` sites"
            );
        }
        other => panic!("a grafted destination must be refused at the entry gate, got {other:?}"),
    }
}
