//! Shared vocabulary for the sweep test corpus: section authoring
//! below, the orientation checking in [`orient`], the vented-cavity
//! fixtures in [`cavity`] and the closed forms in [`oracles`].
//!
//! **Routing rule**, so a further home does not appear without one.
//! These are the places a `sweep` suite can share from, and an item
//! lives at the narrowest one all of its consumers can reach:
//!
//! - `sweep::test_support` — fixtures the LIBRARY can build, reachable
//!   from in-crate tests, from here, and (behind the same dev-only
//!   feature) from another crate's suites, which is where a fixture
//!   with consumers OUTSIDE this crate has to live: the swept elbow
//!   the `mesh` and `step-export` suites meter is there for that
//!   reason;
//! - this module — section authoring, the profile vocabulary a suite
//!   builds a body FROM;
//! - [`orient`] — what a suite CHECKS of a body it built, by reading
//!   POSITIONS off the shipped charts;
//! - [`cap_rims`] — what a suite checks of a body's CAP RIMS: the
//!   boundary walk, the face across a rim, and the description each
//!   rim carries. A reader, not an evaluator, which is why it is not
//!   [`orient`];
//! - [`approx`] — the `Surface::Approx` surgery vocabulary (body
//!   authoring, so it routes to this module rather than to a suite);
//! - [`cavity`] — the vented-cavity fixture vocabulary (body
//!   authoring, same routing);
//! - [`oracles`] — closed-form volumes, which are neither: a truth
//!   derived without the kernel, so its own doc carries the rule for
//!   which per-suite spellings may come here at all;
//! - `revolve_common` — the revolve suites' own, and the place `p2`
//!   and `eps` presently live despite belonging to no verb.
//!
//! A helper one suite uses stays in that suite.
//!
//! **Two rules bind every module here, and both are checkable by
//! reading**: each module says which of its neighbours it deliberately
//! did NOT absorb, as a list that claims to be the whole of it; and
//! every suite that keeps its own copy of something this tree holds
//! says why AT the copy.
//!
//! The second rule has a MARKER so it can be compared rather than
//! sampled: every such copy's note carries the literal
//! ``NOT `common::`` naming the item it is not, on ONE line, so
//!
//! ```text
//! grep -rn 'NOT `common::' crates/sweep/tests
//! ```
//!
//! returns exactly the kept copies inside this crate and nothing else.
//! Its hits and the two module lists below name the same set; a hit
//! missing from a list, or a list entry with no hit, is the rule
//! broken. (Copies OUTSIDE `crates/sweep` are out of the recipe's
//! scope by construction — [`oracles`]'s list names the ones it knows
//! of, and they are tracked as their own item.)
//!
//! Section authoring (LIB-U3): loft/sweep sections in the profile
//! vocabulary, one copy per crate. Cross-crate constant deduplication
//! (the 1/16-offset table relation) is LIB-U6's territory,
//! deliberately not built here.

#![allow(dead_code)]
// one instance per binary; no single consumer uses all of it
// Why a helper tree allows these: `crates/editor-core/tests/fixture/mod.rs`.
#![allow(clippy::unwrap_used, clippy::expect_used, clippy::panic)]
#![allow(unreachable_pub)] // why: root Cargo.toml, the `unreachable_pub` stanza

/// Orientation checking: the face-facing probe and the two level-set
/// indexes it decides against. Split out because it is a different kind
/// of shared thing from the section authoring below — not a fixture,
/// but the check several suites make of a body they built.
pub mod orient;

/// Reading a built body's cap rims — the boundary walk, the face
/// across a rim, and the description each rim carries. What a suite
/// CHECKS of a body it built, so it routes beside [`orient`] rather
/// than into it: nothing here evaluates a surface.
pub mod cap_rims;

/// The Euler–Poincaré census — a built body's ring count and genus,
/// through the kernel's census door. A check several suites make of
/// a body they built, so it routes beside [`orient`].
pub mod census;

/// The `Surface::Approx` surgery vocabulary — the pulled-back base,
/// the fixtures the OFF-C rows convert, and the surface + carrier +
/// pcurve surgery itself. Body authoring, so it routes here.
pub mod approx;

/// The vented-cavity fixture vocabulary the concave blend suites
/// carve — brick, rod, prism, the vented cavity itself and the
/// find-an-edge-by-its-endpoints traversal. Body authoring, so it
/// routes here.
pub mod cavity;

/// The intersecting equal-radius cylinder pair — the germ lane's
/// fixture and the parameter-identity channel's, one authoring for
/// the one door both read. Body authoring, so it routes here.
pub mod germ_pair;

/// The cone-nappe fixtures and the corner walk the SHELL-6 suites
/// share — two mirrored frustums, the coned tube, and the reader that
/// takes a face's own corner stations. Body authoring plus the one
/// reader three suites check a cone face with, so it routes here.
pub mod cone_nappe;

/// The closed-form volumes those suites meter against. Not a fixture
/// and not a check of a body, but a truth derived WITHOUT the kernel;
/// its module doc carries the rule for which per-suite spellings come
/// here and which are second derivations that must not.
pub mod oracles;

use geom::NurbsCurve3;
use geom_core::{Affine3, Mat3, Point2, Point3, Tol, Vec3};
use profile::RawLoop;
use profile::{Profile, SketchPlane};
use sweep::{ProfileLoop, ProfileVertex, Section};
use topo::Body;

/// The placement a path sweep starts from: the plane through the
/// path's start point whose normal is the start TANGENT, with the
/// in-plane axes built off whichever world axis is least parallel to
/// it. `sweep::sweep_places` carries this frame along the path by
/// minimal rotation, so a section placed here stays normal to the
/// path — the recipe every path-swept fixture in this corpus starts
/// from, and the one the tour's sweep cells narrate.
pub fn normal_start_place(path: &NurbsCurve3<f64>) -> Affine3<f64> {
    let (lo, _) = path.domain();
    let d = path.deriv(lo);
    let n = d / d.norm();
    let helper = if n.z.abs() < 0.9 {
        Vec3::new(0.0, 0.0, 1.0)
    } else {
        Vec3::new(1.0, 0.0, 0.0)
    };
    let u = helper.cross(n);
    let u = u / u.norm();
    let v = n.cross(u);
    let p = path.eval(lo);
    Affine3::from_parts(Mat3::from_cols(u, v, n), Vec3::new(p.x, p.y, p.z))
}

/// A closed four-line quad section (one loop, four vertices) — the
/// plainest INTEGRAL profile: unit weights, no arc anywhere.
pub fn quad(pts: [(f64, f64); 4]) -> Section {
    vec![ProfileLoop::polygon(
        pts.iter().map(|&(x, y)| Point2::new(x, y)),
    )]
}

/// The M5 PR 10 review section: a square-with-an-arc loop scaled by
/// `s` — three lines and one bulge-0.25 arc, so the skin exercises
/// the rational lane.
pub fn chain(s: f64) -> Section {
    let v = |x: f64, y: f64, bulge: f64| ProfileVertex::new(Point2::new(x * s, y * s), bulge);
    vec![ProfileLoop::new(vec![
        v(0.0, 0.0, 0.0),
        v(2.0, 0.0, 0.25),
        v(2.0, 1.0, 0.0),
        v(0.0, 1.0, 0.0),
    ])]
}

/// **The arc section**: a square of half-width `s` with a
/// quarter-circle bulge on the `+x` side — the arc-bearing profile
/// whose lofted wall is RATIONAL (weights `1, cos 22.5°, 1` over two
/// 45° sub-arcs), and so the cheapest profile in this corpus that puts
/// a body's enclosure on the QUADRATURE lane rather than a closed
/// form.
///
/// One copy for the crate. It was four — `m8_3_rational_volume`,
/// `cert5_offgrid_knot_rational` and the certificate suite each held a
/// byte-identical spelling of it, and each restated the same weights
/// comment. `step-import`'s copies (`nurbs_import`, `rw2_probes`,
/// `review_probes_m7_3`, `recognize_pins`) are NOT folded in here:
/// cross-crate constant deduplication is LIB-U6's territory, which
/// this module's routing rule says is deliberately not built here.
pub fn arc_section(s: f64) -> Section {
    let v = |x: f64, y: f64, bulge: f64| ProfileVertex::new(Point2::new(x, y), bulge);
    vec![ProfileLoop::new(vec![
        v(-s, -s, 0.0),
        // tan(π/8): a quarter-circle bulge-out.
        v(s, -s, 0.4142135623730951),
        v(s, s, 0.0),
        v(-s, s, 0.0),
    ])]
}

/// **Runs `run` on a pool of exactly `threads` threads**, and answers
/// its value.
///
/// One home for the rule, which every caller would otherwise restate:
/// `RAYON_NUM_THREADS` configures the GLOBAL pool once per process, so
/// it cannot give one test binary a one-thread row and a four-thread
/// row. A `ThreadPool` can, and `ThreadPool::install` runs the closure
/// on that pool's worker — which is also the pool `topo::props`' face
/// map then uses, so everything a row measures sits inside the width it
/// names. (`benches/` is a separate cargo root and cannot reach this;
/// its copy carries a one-line pointer here.)
pub fn on_pool<R: Send>(threads: usize, run: impl Fn() -> R + Send + Sync) -> R {
    rayon::ThreadPoolBuilder::new()
        .num_threads(threads)
        .build()
        .expect("the pool builds")
        .install(run)
}

/// The quintic prism: six square sections of DIFFERING scale at
/// v-degree 5, which is outside the exact per-span rule window (> 4 per
/// direction), so its walls take the patch engine's COMPOSITE rounds
/// and the round they stop at is an ε question.
///
/// The scale is chosen so the committed rows DIFFER across the matrix:
/// the composite's starting width lands between the 1e-12 target and
/// the 1e-9 one, so this body refuses on budget at the tight ε and
/// certifies at the other two. The sections must differ, or the walls
/// are flat — a flat patch's second derivatives are zero, the
/// composite's remainder vanishes and round 0 certifies at
/// ring-rounding width whatever ε is.
pub fn quintic_prism() -> Body<f64> {
    let s = 0.1;
    let sq = |k: f64| {
        quad([
            (-k * s, -k * s),
            (k * s, -k * s),
            (k * s, k * s),
            (-k * s, k * s),
        ])
    };
    sweep::loft_body::<f64>(
        &[sq(1.0), sq(1.05), sq(1.15), sq(1.3), sq(1.5), sq(1.75)],
        &stacked(&[0.0, 0.4, 0.8, 1.2, 1.6, 2.0], s),
        5,
        Tol::witness(),
    )
    .expect("the quintic prism lofts")
    .body
}

/// The tilted cylinder cut, upper part: a cylinder split by a plane at
/// `φ = 0.3`, whose wall pieces are bounded by exact `Ellipse`
/// carriers — the CYLINDER chart's Green form, which no loft or sweep
/// verb can produce (their walls carry iso boundaries and take the
/// closed forms).
pub fn tilted_cut_upper() -> Body<f64> {
    let lp = ProfileLoop::new(vec![
        ProfileVertex::new(Point2::new(-0.5, 0.0), 1.0),
        ProfileVertex::new(Point2::new(0.5, 0.0), 1.0),
    ]);
    let disc = Profile::new(SketchPlane::xy(), vec![lp])
        .validate(Tol::witness())
        .expect("the disc profile validates");
    let cylinder = sweep::extrude::<f64>(&disc, sweep::Extrusion::Distance(1.0), Tol::witness())
        .expect("the cylinder extrudes")
        .body;
    let phi = 0.3f64;
    let result = topo::splitting::split(
        &cylinder,
        &topo::splitting::SplitPlane {
            origin: Point3::new(0.0, 0.0, 0.5),
            normal: Vec3::new(phi.sin(), 0.0, phi.cos()),
        },
        Tol::witness(),
    )
    .expect("the tilted cut splits");
    let topo::splitting::SplitPart::Body(above) = result.above else {
        panic!("both sides of the tilted cut carry material");
    };
    above
}

/// The bulged extrusion: an analytic cylinder wall with a CURVED trim
/// loop — the cylinder chart's Green form.
pub fn bulged_extrusion() -> Body<f64> {
    let prof = Profile::new(SketchPlane::xy(), arc_section(1.0))
        .validate(Tol::witness())
        .expect("the profile validates");
    sweep::extrude::<f64>(&prof, sweep::Extrusion::Distance(2.0), Tol::witness())
        .expect("extrude")
        .body
}

/// The **sup-norm distance** between two points — the largest
/// coordinate disagreement, which is the honest meter for "these two
/// constructions produced the same point": it bounds every coordinate
/// at once and never averages a bad axis away, as a Euclidean norm
/// would. Lives here because a per-coordinate comparison is what any
/// exactness row in this tree wants, and a fourth hand-rolled copy is
/// how a suite ends up with a subtly different one.
pub fn sup_dist(a: Point3<f64>, b: Point3<f64>) -> f64 {
    (a.x - b.x)
        .abs()
        .max((a.y - b.y).abs())
        .max((a.z - b.z).abs())
}

/// **The certified quadrature's rounds, counted rather than timed** —
/// the number of `props_quad_*` classifications the kernel's one
/// recording funnel made while `run` executed. One certificate over
/// one body at one band contributes a fixed number of them; two
/// contribute twice that, and a caller that stopped early contributes
/// fewer.
///
/// Here rather than in a suite because three suites count the same
/// thing (`tcost_k3_certificate`, `sign_certified_plus_v`, and
/// `step-import`'s import-path row across the crate boundary), and a
/// counter that drifts between them is two different instruments
/// reporting one number. The routing rule above does not have a slot
/// for an instrument; this is the slot.
pub fn quad_verdicts(run: impl FnOnce()) -> usize {
    let bracket = geom_core::k_stats::Bracket::open();
    run();
    bracket
        .finish()
        .verdicts
        .iter()
        .filter(|v| v.predicate.starts_with("props_quad"))
        .count()
}

/// **FNV-1a over a byte stream** — an order- and element-sensitive
/// fold of a recorded channel into one hex word, so a golden line
/// stays a line. Not a cryptographic claim: what it has to do is
/// change when any element, sign or position changes, which is what a
/// golden compares.
///
/// Beside [`quad_verdicts`] for its reason: the thread-count goldens
/// of two walks read the same channels through the same fold, and a
/// digest that drifts between them is two instruments reporting one
/// number. One more copy of the basis than the tree needs is what
/// `work/perf/fnv-digest-and-memo-machinery-copies.md` tracks; this is
/// the sweep suites' one home for it.
pub fn fnv1a(bytes: &[u8]) -> u64 {
    let mut h: u64 = 0xcbf2_9ce4_8422_2325;
    for &b in bytes {
        h ^= u64::from(b);
        h = h.wrapping_mul(0x1000_0000_01b3);
    }
    h
}

/// **The recorded channels of one walk, folded**: the counts
/// (readable) and the order-sensitive hash (complete). What a
/// thread-count golden compares, for any walk that composes a
/// worker's K-funnel recording back into the caller's frame.
pub fn channels(r: &geom_core::k_stats::Recorded) -> String {
    let mut v = Vec::new();
    for verdict in &r.verdicts {
        v.extend_from_slice(verdict.predicate.as_bytes());
        v.push(1);
        v.extend_from_slice(format!("{:?}", verdict.sign).as_bytes());
        v.push(0);
    }
    let mut e = Vec::new();
    for esc in &r.escalations {
        e.extend_from_slice(esc.predicate().as_bytes());
        e.push(0);
    }
    format!(
        "verdicts n={} h={:016x} esc n={} h={:016x}",
        r.verdicts.len(),
        fnv1a(&v),
        r.escalations.len(),
        fnv1a(&e),
    )
}

/// A thin curved STRIP section: a rectangle `[-s, s] × [0, delta]`
/// whose two long sides are quarter-circle bulges in OPPOSITE
/// directions, so the loft's two big rational walls contribute fluxes
/// that nearly cancel and the body's volume is `≈ 2·s·delta` per unit
/// height — arbitrarily small against the enclosure width the walls
/// themselves carry.
///
/// That is what makes it the fixture for an UNDECIDED sign: at a small
/// enough `delta/s` the round-0 enclosure straddles zero, and at a
/// large enough `s` the schedule has already run out. `reversed`
/// builds the same strip traversed the other way, which is the
/// inside-out twin — the body an orientation check exists to catch.
pub fn strip_section(s: f64, delta: f64, reversed: bool) -> Section {
    // tan(π/8): a quarter-circle bulge-out, as `arc_section` uses.
    let b = 0.414_213_562_373_095_1;
    let v = |x: f64, y: f64, bulge: f64| ProfileVertex::new(Point2::new(x, y), bulge);
    if reversed {
        return vec![ProfileLoop::new(vec![
            v(-s, 0.0, 0.0),
            v(-s, delta, b),
            v(s, delta, 0.0),
            v(s, 0.0, -b),
        ])];
    }
    vec![ProfileLoop::new(vec![
        v(-s, 0.0, b),
        v(s, 0.0, 0.0),
        v(s, delta, -b),
        v(-s, delta, 0.0),
    ])]
}

/// Loft placements: the given heights, each scaled by `s`, as pure
/// `+z` translations — the stacking that makes a loft of identical
/// sections reproduce the EXTRUSION of that section exactly.
pub fn stacked(z: &[f64], s: f64) -> Vec<Affine3<f64>> {
    z.iter()
        .map(|h| Affine3::translation(Vec3::new(0.0, 0.0, h * s)))
        .collect()
}
