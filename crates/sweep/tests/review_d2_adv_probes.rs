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
//!   varies, is logged unconditionally, and the count rides
//!   `CAD_FUZZ_EFFORT` through [`test_utils::fuzz`]. Cutting the count
//!   loses detection power, never correctness.
//! - `d2_reached_variants` asserts nothing about coverage and is
//!   therefore **evidence, not a gate** — it prints which refusal
//!   classes the corpus actually reaches, which is the number a
//!   future reader of the partition wants.
//!
//! **Anti-vacuity floors** ([`test_utils::vacuity`]). Every interesting
//! body in the corpus is minted behind a fallible door, so the gates
//! state how much they actually exercised and assert it:
//! [`census_corpus`] floors the corpus and the rim/circle/multi-solid
//! coverage the claims rest on, and `d2_no_input_reaches_a_panic` floors
//! its request count and the number of requests that came back **with a
//! band face** — the receipt `rim_phase` leaves, and the only evidence
//! from outside the door that the phase holding 6 of the 18
//! `unreachable!` sites ran at all. Without these a run in which
//! `boolean_op_with` or `revolve` began refusing would shrink to a bare
//! cube and stay green.
//!
//! **A floor on a request SHAPE would not have done.** The request
//! builder pushes the rim set unconditionally for any rim-carrying body,
//! so counting all-rim requests floors something `census_corpus` already
//! implies, and every one of those calls could refuse above `rim_phase`
//! with the count unmoved. The floor has to be on an outcome.
//!
//! Gating: these are written against `crates/sweep/src/fillet`; run
//! them when that directory changes.
//!
//! **The dial is `test_utils::fuzz`, not one of this suite's own.**
//! `scripts/gates/no-ambient-env.sh` greps `crates/*/src` and would
//! not have caught a suite-local dial here; the reason to use the
//! ratified one is not the gate's reach but that `CAD_FUZZ_EFFORT`
//! scales this row with every other randomized sweep in the workspace
//! from one place.

#![allow(clippy::unwrap_used, clippy::expect_used, clippy::panic, dead_code)]

use core::f64::consts::PI;

use geom_core::Tol;
use geom_core::{Affine3, Band, Point2, Vec2, Vec3};
use profile::{Profile, ProfileLoop, ProfileVertex, RawLoop, SketchPlane};
use sweep::fillet::{FilletError, fillet_edges};
use sweep::test_support::cube;
use sweep::{Revolution, RevolveAxis, revolve};
use test_utils::fuzz::{self, Rng};
use test_utils::vacuity::Exposure;
use topo::boolean::{BooleanOp, SweepStrategy, boolean_op_with};
use topo::{Body, BooleanDeclarations, EdgeKey};

/// How many random edge subsets each corpus body gets — one multiple
/// of `CAD_FUZZ_EFFORT` per body. `CAD_FUZZ_SEED` pins the draws.
fn effort() -> usize {
    fuzz::scaled(24)
}

fn band() -> Band {
    let tol = Tol::witness().get();
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
        .validate(Tol::witness())
        .unwrap();
    let axis = RevolveAxis {
        origin: p2(0.0, 0.0),
        dir: Vec2::new(0.0, 1.0),
    };
    let ball = revolve(&vp, axis, Revolution::Full, Tol::witness())
        .unwrap()
        .body;
    topo::transform_rigid(&ball, &Affine3::translation(c), Tol::witness()).unwrap()
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
        Tol::witness(),
    )
    .unwrap();
    topo::transform_rigid(&rot, &Affine3::translation(c), Tol::witness()).unwrap()
}

fn subtract(a: &Body<f64>, b: &Body<f64>) -> Option<Body<f64>> {
    let out = boolean_op_with(
        BooleanOp::Subtract,
        a,
        b,
        &BooleanDeclarations::none(),
        SweepStrategy::Realized,
        Tol::witness(),
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
        Tol::witness(),
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
    let c = cube(1.0, Tol::witness());
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
        Tol::witness(),
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
        let box_edges: Vec<EdgeKey> = cube(1.0, Tol::witness())
            .edges()
            .map(|(k, _)| k)
            .filter(|k| b.get_edge(*k).is_some())
            .collect();
        if let Ok(f) = fillet_edges(&b, &box_edges, 0.12, band(), Tol::witness()) {
            out.push(("die_one_pip_blended", f.body));
        }
        if let Some(b2) = subtract(&b, &pip(0.25, 0.25, 0.09, 0.05)) {
            out.push(("die_two_pips", b2));
        }
    }
    // A pip straddling an edge: its rim crosses two support faces, so
    // the closed chain is not a ring of one plane.
    // ABSENT AT THE DEFAULT ε, AND THAT IS WHY THE FLOORS BELOW DO NOT
    // NAME THESE THREE. Measured on an intact tree: `die_two_pips`
    // refuses `Containment(PartialSphereFace)` and the straddling and
    // corner pips refuse `CurvedPierceUnsupported` — declared-unsupported
    // variants from `boolean_op_with`, not defects. So the corpus mints
    // 8 of the 11 bodies written here, and `census_corpus` prints which,
    // every run. A reader asking "does this ever fire?" reads the
    // evidence line, not a comment that would go stale.
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
    if topo::instance::graft_disjoint_all(&mut dst, &cube(0.5, Tol::witness()), Tol::witness())
        .is_ok()
    {
        out.push(("grafted_two_solid", dst.clone()));
        let mut again = dst.clone();
        if topo::instance::graft_disjoint_all(
            &mut again,
            &ball_at(0.3, Vec3::new(9.0, 0.0, 0.0)),
            Tol::witness(),
        )
        .is_ok()
        {
            out.push(("grafted_three_solid", again));
        }
    }
    out
}

/// The plane-sphere edges of `body` — the **rim** requests' own door.
/// Named as its own function because it is the thing the sweep's
/// anti-vacuity floor counts: without a body that has one, the sample
/// never reaches `rim_phase`, which holds 6 of the 18 `unreachable!`
/// sites this file exists to attack.
fn rim_edges(body: &Body<f64>) -> Vec<EdgeKey> {
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
    body.edges()
        .filter(|(_, e)| {
            let (Some(fa), Some(fb)) = (face_of(e.he_plus), face_of(e.he_minus)) else {
                return false;
            };
            let mut kinds = [kind_of(fa), kind_of(fb)];
            kinds.sort_unstable();
            kinds == [Some(0), Some(1)]
        })
        .map(|(k, _)| k)
        .collect()
}

/// The circle-carried edges of `body`, likewise counted by the floor:
/// a corpus of straight-edged boxes reaches none of the curved-spine
/// arms.
fn circle_edges(body: &Body<f64>) -> Vec<EdgeKey> {
    body.edges()
        .map(|(k, _)| k)
        .filter(|&k| {
            body.get_edge(k)
                .and_then(|e| body.get_curve_geom(e.curve))
                .and_then(|g| g.certified())
                .is_some_and(|c| matches!(c.carrier(), geom::Curve3::Circle { .. }))
        })
        .collect()
}

/// **The corpus's anti-vacuity floor.** Every interesting body above is
/// minted behind a fallible door — `revolve`, `transform_rigid`,
/// `boolean_op_with`, `graft_disjoint_all` — and a refusal from any of
/// them silently removes a body rather than failing. Left unasserted,
/// this file degrades to a bare cube and stays green while covering
/// none of what it was written for.
///
/// What is floored is what the file's own claims rest on, not the
/// literal roster: the three doors that mint bodies with no boolean in
/// them (so the floor cannot red on a boolean's honest refusal), the
/// **rim** coverage the `rim_phase` sentence above depends on, and the
/// **circle** coverage the curved-spine arms depend on. A run that
/// loses any of those loses the file's reason to exist.
fn census_corpus(bodies: &[(&'static str, Body<f64>)]) {
    let mut c = Exposure::new("d2 corpus");
    for (name, b) in bodies {
        c.note("bodies");
        c.note(name);
        if !rim_edges(b).is_empty() {
            c.note("bodies with a plane-sphere rim");
        }
        if !circle_edges(b).is_empty() {
            c.note("bodies with a circle-carried edge");
        }
        if b.solids().count() > 1 {
            c.note("multi-solid bodies");
        }
    }
    c.report();
    let why = "the corpus has degraded — a door that mints one of these bodies now refuses, \
               and the sweep below would cover none of what this file was written for";
    // Measured on an intact tree: 8 bodies, 3 with a rim, 5 with a
    // circle, 2 multi-solid. The floors sit below those, not at them —
    // a boolean is allowed to start refusing one pip without reddening
    // a row that still has the coverage its claims rest on.
    c.require("bodies", 6, why);
    c.require_each(
        &["cube", "cube_rotated", "ball", "grafted_two_solid"],
        1,
        why,
    );
    c.require("bodies with a plane-sphere rim", 1, why);
    c.require("bodies with a circle-carried edge", 1, why);
    c.require("multi-solid bodies", 1, why);
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
    let rims = rim_edges(body);
    if !rims.is_empty() {
        out.push(rims.clone());
        if rims.len() > 1 {
            out.push(rims[..1].to_vec());
            out.push(rims[1..].to_vec());
        }
    }
    let circles = circle_edges(body);
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
        FilletError::BodyNotIntact { .. } => "BodyNotIntact(row 1)",
        FilletError::RingClearance { .. } => "RingClearance",
        FilletError::Certify { .. } => "Certify",
        FilletError::Op { .. } => "Op",
    }
}

/// A completed rim blend, counted as the receipt `rim_phase` leaves.
const RIM_PHASE: &str = "requests that came back with a band face";

/// Radii spanning the scale: below, at, and above the features.
const RADII: [f64; 6] = [1e-6, 1e-3, 0.02, 0.05, 0.12, 0.9];

/// **The headline row.** Every request the corpus can express must
/// come back as a VALUE. A panic here is a reachable `unreachable!`
/// (or an index) on the public path, which falsifies both the unit's
/// central claim and D9's headline bullet.
#[test]
fn d2_no_input_reaches_a_panic() {
    let mut rng = fuzz::start("d2_no_input_reaches_a_panic");
    // ONE corpus, not two: minting it runs several booleans and a
    // revolve.
    let bodies = corpus();
    census_corpus(&bodies);
    let hook = std::panic::take_hook();
    std::panic::set_hook(Box::new(|_| {}));
    let mut fired: Vec<String> = Vec::new();
    let mut census = Exposure::new("d2_no_input_reaches_a_panic");
    census.add(RIM_PHASE, 0);
    for (name, body) in &bodies {
        let rims = rim_edges(body);
        for req in requests(body, &mut rng, effort()) {
            let all_rim =
                !rims.is_empty() && !req.is_empty() && req.iter().all(|k| rims.contains(k));
            for r in RADII {
                census.note("requests");
                if all_rim {
                    census.note("all-rim requests");
                }
                // The closure yields the BAND-FACE COUNT, not the body:
                // `Filleted::band_faces` is *"one per CLOSED chain (the
                // rim blends)"* and is written by `rim_phase` itself, so
                // a nonzero count is a receipt that the phase ran. That
                // is the only outcome-level proof available from outside
                // the door, and it is what the floor below counts.
                let outcome = std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| {
                    fillet_edges(body, &req, r, band(), Tol::witness())
                        .map(|f| f.band_faces.len())
                        .unwrap_or(0)
                }));
                match outcome {
                    Err(_) => fired.push(format!(
                        "{name}: {} edge(s), radius {r} — {}",
                        req.len(),
                        fuzz::replay()
                    )),
                    Ok(bands) if bands > 0 => census.note(RIM_PHASE),
                    Ok(_) => {}
                }
            }
        }
    }
    std::panic::set_hook(hook);
    assert!(
        fired.is_empty(),
        "the fillet door panicked on {} of {} requests ({}): {fired:#?}",
        fired.len(),
        census.count("requests"),
        fuzz::replay()
    );
    census.report();
    // Measured on an intact tree at `CAD_FUZZ_EFFORT=1`: 1842 requests.
    // The floor is 300, which the enumerated shapes alone (empty, whole,
    // reversed, two singletons, the repeat, six windows — 11 per body,
    // six radii) clear from the floored body count of 6.
    census.require(
        "requests",
        300,
        &format!(
            "the request builder has collapsed, so `fired.is_empty()` is a claim about \
             almost nothing — {}",
            fuzz::replay()
        ),
    );
    // THE FLOOR THAT HAD TO BE AN OUTCOME. Counting all-rim REQUESTS
    // floors a shape the request builder produces unconditionally for
    // any rim-carrying body — it is implied by `census_corpus`'s own rim
    // floor twenty lines up, so it can never red first, and all of its
    // calls could refuse above `rim_phase` with the count unmoved.
    // `rim_phase` sits below five `?` returns; only a band face proves
    // it ran. Measured on an intact tree: 12 all-rim requests reach `Ok`.
    census.require(
        RIM_PHASE,
        1,
        &format!(
            "no request came back with a band face, so nothing this run did reached \
             `rim_phase`, which holds 6 of the 18 `unreachable!` sites — {}",
            fuzz::replay()
        ),
    );
}

/// EVIDENCE, not a gate (it asserts nothing about coverage): which
/// refusal classes the corpus reaches. A reader checking the
/// partition wants to know that the row-2 names and the row-1 names
/// are not equally reachable.
#[test]
fn d2_reached_variants() {
    let mut rng = fuzz::start("d2_reached_variants");
    let mut seen: Vec<(&'static str, usize)> = Vec::new();
    let mut ok = 0usize;
    for (_, body) in corpus() {
        for req in requests(&body, &mut rng, effort()) {
            for r in RADII {
                match fillet_edges(&body, &req, r, band(), Tol::witness()) {
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
    let base = cube(1.0, Tol::witness());
    let edges: Vec<EdgeKey> = base.edges().map(|(k, _)| k).collect();
    // The whole-body fillet succeeds before the graft: the request is
    // inside the front door, so any refusal below is the graft's doing
    // and not the request's.
    assert!(
        fillet_edges(&base, &edges, 0.12, band(), Tol::witness()).is_ok(),
        "the control request must pass, or this row proves nothing"
    );

    let mut dst = base.clone();
    topo::instance::graft_disjoint_all(&mut dst, &cube(0.5, Tol::witness()), Tol::witness())
        .expect("a disjoint graft");
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
    match fillet_edges(&dst, &after, 0.12, band(), Tol::witness()) {
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
