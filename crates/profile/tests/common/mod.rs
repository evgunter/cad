//! Shared fixtures and helpers for the profile integration suites.
//!
//! Fixtures are built at `f64` and lifted to other scalars via
//! [`lift`]; geometry that must sit at an ε-relative margin takes the
//! run's ε explicitly so the multi-ε CI rows genuinely re-exercise the
//! bands. The run tolerance comes from `Tol::witness().get()` — one read
//! per test process, and the crate's suites all run inside the one
//! aggregated `all` binary, so the geom-core global-state discipline is
//! satisfied by a single process-wide read.
#![allow(dead_code)] // one instance per binary; no single consumer uses all of it
#![allow(unreachable_pub)] // why: root Cargo.toml, the `unreachable_pub` stanza
#![allow(clippy::unwrap_used, clippy::expect_used, clippy::panic)]

use geom_core::Tol;
use geom_core::{Point2, Real};
use profile::RawLoop;
use profile::{
    ArcSweep, Center, ClosedLoop, CornerReason, CornerRefusal, FilletLeg, FilletLegCarrier, Open,
    PathError, Profile, ProfileLoop, SketchPlane, Start, test_support::bulge_loop,
};

/// A point in the profile frame, from its two coordinates.
pub fn p2(x: f64, y: f64) -> Point2<f64> {
    Point2::new(x, y)
}

/// **The one accessor**: a refusal's corner entries, in the order the
/// kernel reported them (nearest the bracketing anchors first), or the
/// EMPTY SLICE for a refusal that is not the envelope.
///
/// Total on purpose, so the searching helpers below and the sweep rows
/// that ask "did some corner refuse this way" can call it on an
/// arbitrary refusal. A row that needs the SHAPE asserts it — the
/// length, and which corner each entry names — and every such
/// assertion carries the refusal in its message, so an empty slice
/// reads as the wrong refusal rather than as a silent zero.
pub fn corners<T: Real>(err: &PathError<T>) -> &[CornerRefusal<T>] {
    match err {
        PathError::NoCornerOfPair { corners, .. } => corners,
        _ => &[],
    }
}

/// **Which corners the refusal is about, exactly.**
///
/// Asserts the envelope's LENGTH and each entry's point, in the order
/// reported. A row whose subject is attribution — which corner refused,
/// and whether the other one is listed beside it — has to say both: an
/// existential "some entry refused this way" passes on an envelope that
/// names the wrong corner, which is the defect the envelope exists to
/// remove.
#[track_caller]
pub fn assert_corners(err: &PathError<f64>, want: &[(f64, f64)], what: &str) {
    let got: Vec<(f64, f64)> = corners(err).iter().map(|c| (c.at.x, c.at.y)).collect();
    assert_eq!(
        got.len(),
        want.len(),
        "{what}: the envelope lists {got:?}, not {want:?} — refusal {err:?}"
    );
    for (i, (g, w)) in got.iter().zip(want).enumerate() {
        let off = (g.0 - w.0).hypot(g.1 - w.1);
        let scale = w.0.hypot(w.1).max(1.0);
        assert!(
            off <= 1e-9 * scale,
            "{what}: entry {i} names {g:?}, not {w:?} (off by {off}) — refusal {err:?}"
        );
    }
}

/// Whether ANY entry of the envelope refused for the named shape.
pub fn any_reason<T: Real>(err: &PathError<T>, pred: impl Fn(&CornerReason<T>) -> bool) -> bool {
    corners(err).iter().any(|c| pred(&c.reason))
}

/// The first entry refusing with the enclosing class, as its payload.
pub fn enclosing<T: Real>(err: &PathError<T>) -> Option<(Option<FilletLeg>, T, T, Option<T>)> {
    corners(err).iter().find_map(|c| match &c.reason {
        CornerReason::EnclosesLegCarrier {
            side,
            carrier_radius,
            offset_radius,
            largest_tangent_radius,
        } => Some((
            *side,
            *carrier_radius,
            *offset_radius,
            *largest_tangent_radius,
        )),
        _ => None,
    })
}

/// The first entry refusing on the anchor fit, as its payload.
pub fn anchor_fit<T: Real>(err: &PathError<T>) -> Option<(FilletLeg, FilletLegCarrier, T, T)> {
    corners(err).iter().find_map(|c| match &c.reason {
        CornerReason::AnchorOutsideTrimmedExtent {
            side,
            carrier,
            setback,
            available,
        } => Some((*side, *carrier, *setback, *available)),
        _ => None,
    })
}

/// Whether some corner of the envelope refused with the enclosing
/// class.
pub fn is_enclosing<T: Real>(err: &PathError<T>) -> bool {
    enclosing(err).is_some()
}

/// The run's tolerance (env-driven; the multi-ε matrix parameterizes
/// it).
/// The K funnel name this suite's authored frame axes are decided
/// under. One name for both, because which axis a refusal names is the
/// refusal's own field.
pub const FRAME_AXIS_SITE: &str = "profile_test_frame_axis";

/// A frame witness from an authored pair — the mint every plane in
/// this suite goes through, spelled once.
///
/// # Panics
///
/// If the band cannot be formed, or if the pair spans no plane.
pub fn frame_of(
    o: geom_core::Point3<f64>,
    u: geom_core::Vec3<f64>,
    v: geom_core::Vec3<f64>,
) -> geom_core::OrthoFrame<f64> {
    geom_core::OrthoFrame::gram_schmidt(
        o,
        u,
        v,
        FRAME_AXIS_SITE,
        geom_core::Band::linear(tol()).expect("the witness band"),
    )
    .expect("the pair spans a plane")
}

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
///
/// `Profile::map_scalar` except for the plane: this mints `xy` at `T`
/// rather than lifting `p`'s, and every caller's profile is on `xy`.
pub fn lift<T: Real>(p: &Profile<f64>) -> Profile<T> {
    Profile::new(
        SketchPlane::xy(),
        p.loops
            .iter()
            .map(|lp| lp.map_scalar(T::from_f64))
            .collect(),
    )
}

/// A loop from `(x, y, bulge)` triples.
pub fn chain(vs: &[(f64, f64, f64)]) -> ProfileLoop<f64> {
    bulge_loop(
        vs.iter()
            .map(|&(x, y, bulge)| (Point2::new(x, y), bulge))
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
    assert_spans_partition(&closed);
    assert_pieces_name_one_segment_each(&closed);
    closed.loop_
}

/// **Every segment is exactly one piece, and no piece is two
/// segments**: one piece per segment of the loop, each naming a step
/// of the program, no two alike — which is what lets a `{ step, role }`
/// locator denote one wall. A step whose role list the lowering can
/// draw twice would land here, over the whole corpus.
pub fn assert_pieces_name_one_segment_each(closed: &ClosedLoop<f64>) {
    let pieces = &closed.structure.pieces;
    assert_eq!(
        pieces.len(),
        closed.loop_.vertices().len(),
        "one piece per segment"
    );
    for (k, p) in pieces.iter().enumerate() {
        assert!(
            p.step < closed.program.len(),
            "segment {k}'s piece names step {}, past the program's {} steps",
            p.step,
            closed.program.len()
        );
        if let Some(j) = pieces[..k].iter().position(|q| q == p) {
            panic!("segments {j} and {k} are both {p}: a locator on it would denote two walls");
        }
    }
    assert_runs_ride_their_carriers(closed);
}

/// **A fillet's run lies on its own side's carrier**: a run in on the
/// incoming side's, a run out on the arrival side's — straight where
/// that side is a ray, an arc where the fused verb authored an arc
/// carrier for it. A segment on any other carrier is the piece of the
/// step that drew it, so a run of the wrong kind is a later step's
/// segment credited to the fillet: a name on it would move to whatever
/// the fillet's run becomes once that step is dropped.
pub fn assert_runs_ride_their_carriers(closed: &ClosedLoop<f64>) {
    use profile::{PieceRole, Step};
    for (k, p) in closed.structure.pieces.iter().enumerate() {
        let straight = !matches!(closed.loop_.segments()[k], profile::Segment::Arc { .. });
        // (incoming side straight, arrival side straight) per fillet verb.
        let sides = match &closed.program[p.step] {
            Step::Fillet { .. } => (true, true),
            Step::FilletArc { .. } => (true, false),
            Step::ArcFillet { .. } => (false, true),
            Step::ArcFilletArc { .. } => (false, false),
            _ => continue,
        };
        let want = match p.role {
            PieceRole::RunIn => sides.0,
            PieceRole::RunOut => sides.1,
            PieceRole::Leg | PieceRole::Arc | PieceRole::Piece(_) => continue,
        };
        assert_eq!(
            straight,
            want,
            "segment {k} is {p} but is {} while that side's carrier is {}",
            if straight { "straight" } else { "an arc" },
            if want { "a ray" } else { "a circle" },
        );
    }
}

/// **The per-step segment span partitions the loop**: one span per
/// authored step, in program order, the spans meeting end-to-start and
/// covering every segment exactly once.
///
/// Rides the same blanket funnel as the differential above, so it holds
/// over every typed chain the suites author rather than a sampled few.
///
/// **What it can and cannot catch, measured.** On a CHAIN the three
/// clauses are what `Core::step_spans` makes true by construction: it
/// derives every boundary from one non-decreasing `step_starts` vector
/// and ends the last span at the closed chain's own length, so
/// contiguity, the cover and the count hold however wrong the
/// boundaries themselves are. A mutant that shifts every boundary one
/// step later — a step credited with its NEIGHBOUR's segments — passes
/// all three, and the row that reds on it is
/// `editor-core/tests/edit_step_segments.rs`'s attribution section,
/// which reads each step's own authored endpoint. What this DOES catch
/// is a span minted outside that arithmetic against a loop it does not
/// describe: `ReplayStructure::carrier(n)` is built from the carrier
/// kernel's own vertex count at a different site from the loop this
/// compares against, and a re-shaped `step_spans` that broke the
/// partition would land here on the whole corpus rather than on
/// whichever suite noticed.
pub fn assert_spans_partition(closed: &ClosedLoop<f64>) {
    let spans = &closed.structure.steps;
    assert_eq!(
        spans.len(),
        closed.program.len(),
        "one span per authored step"
    );
    let n = closed.loop_.vertices().len();
    let mut next = 0;
    for (j, span) in spans.iter().enumerate() {
        assert_eq!(
            span.start(),
            next,
            "step {j}'s span starts where step {} left off",
            j.wrapping_sub(1)
        );
        next = span.end();
    }
    assert_eq!(
        next, n,
        "the spans cover every segment of the {n}-segment loop"
    );
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
        assert_eq!(a.x.to_bits(), b.x.to_bits(), "vertex {i} x");
        assert_eq!(a.y.to_bits(), b.y.to_bits(), "vertex {i} y");
        assert_eq!(
            lowered.bulges()[i].to_bits(),
            replayed.bulges()[i].to_bits(),
            "vertex {i} bulge"
        );
    }
    let mut la = lowered.tangent_joints().to_vec();
    let mut lb = replayed.tangent_joints().to_vec();
    la.sort_unstable();
    lb.sort_unstable();
    assert_eq!(la, lb, "declared tangent joints (multiset)");
}

/// The census corpus: closed chains whose union covers every declared
/// verb. Each is authored through the typed surface, so its recorded
/// program is the table's own output.
pub fn coverage_corpus() -> Vec<ClosedLoop<f64>> {
    use profile::{ArcLen, ArcSide, Bulge, Center, Radius, Sweep, Via};
    use std::f64::consts::{FRAC_PI_2, FRAC_PI_8, PI};

    // 1. The fused entry verb, the plain binders and the straight legs.
    let fused = Open
        .arc_fillet(
            Center {
                c: p2(0.0, 0.0),
                winding: ArcSweep::Ccw,
                p: p2(5.0, 0.0),
            },
            0.5,
            Tol::witness(),
        )
        .unwrap()
        .at(p2(0.0, 3.0), Tol::witness())
        .unwrap()
        .toward(-1.0, 0.0, Tol::witness())
        .unwrap()
        .line(3.0, Tol::witness())
        .unwrap()
        .line_to(Start, Tol::witness())
        .unwrap();

    // 2. An endpoint-free sharp leg, ray extension, an arc arrival and
    //    the mid-chain Radius arc extension.
    let walk = Open
        .at(p2(0.0, 0.0))
        .angle(0.0, Tol::witness())
        .unwrap()
        .arc_to(
            Sweep {
                r: 2.0,
                side: ArcSide::Left,
                angle: 0.6,
            },
            Tol::witness(),
        )
        .unwrap()
        .fillet(0.2, Tol::witness())
        .unwrap()
        .at(p2(4.0, 3.0), Tol::witness())
        .unwrap()
        .toward(0.0, 1.0, Tol::witness())
        .unwrap()
        .fillet_arc(
            0.25,
            Center {
                c: p2(2.0, 6.0),
                winding: ArcSweep::Ccw,
                p: p2(2.0, 9.0),
            },
            Tol::witness(),
        )
        .unwrap()
        .arc_fillet(
            Radius {
                r: 3.0,
                side: ArcSide::Left,
            },
            0.25,
            Tol::witness(),
        )
        .unwrap()
        .at(p2(1.0, 4.0), Tol::witness())
        .unwrap()
        .toward(0.0, -1.0, Tol::witness())
        .unwrap()
        .line(3.0, Tol::witness())
        .unwrap()
        .line_to(Start, Tol::witness())
        .unwrap();

    // 3. `.turn(δ)` at the corners. Each δ is far from both 0 (which
    //    refuses — `.tangent()` is its recourse) and ±π (the reverse
    //    class), so substituting any other director moves real geometry
    //    and the round-trip reddens on the first vertex it reaches.
    let turned = Open
        .at(p2(0.0, 0.0))
        .angle(0.0, Tol::witness())
        .unwrap()
        .line(3.0, Tol::witness())
        .unwrap()
        .turn(FRAC_PI_2, Tol::witness())
        .unwrap()
        .line(3.0, Tol::witness())
        .unwrap()
        .turn(FRAC_PI_2, Tol::witness())
        .unwrap()
        .line(3.0, Tol::witness())
        .unwrap()
        .line_to(Start, Tol::witness())
        .unwrap();

    // 4. The seam-fillet close: mid-side anchors, every corner filleted
    //    including the one under the entry vertex, which `.to(Start)`
    //    retrims.
    let seam = Open
        .at(p2(1.5, 0.0))
        .angle(0.0, Tol::witness())
        .unwrap()
        .fillet(0.5, Tol::witness())
        .unwrap()
        .at(p2(3.0, 1.5), Tol::witness())
        .unwrap()
        .angle(FRAC_PI_2, Tol::witness())
        .unwrap()
        .fillet(0.5, Tol::witness())
        .unwrap()
        .at(p2(1.5, 3.0), Tol::witness())
        .unwrap()
        .angle(PI, Tol::witness())
        .unwrap()
        .fillet(0.5, Tol::witness())
        .unwrap()
        .at(p2(0.0, 1.5), Tol::witness())
        .unwrap()
        .angle(-FRAC_PI_2, Tol::witness())
        .unwrap()
        .fillet(0.5, Tol::witness())
        .unwrap()
        .to(Start, Tol::witness())
        .unwrap();

    // 5. The declared tangent joint and the unique tangent arc.
    let tangent_arc = Open
        .at(p2(0.0, 0.0))
        .line_to(p2(2.0, 0.0), Tol::witness())
        .unwrap()
        .tangent()
        .tangent_arc_to(p2(3.0, 1.0), Tol::witness())
        .unwrap()
        .line_to(Start, Tol::witness())
        .unwrap();

    // 6. Two quarter arcs on one carrier — the half-disc equator —
    //    the second through the lattice's own declared-joint
    //    spelling, `.tangent().tangent_arc_to(p)`.
    let subdivided = Open
        .at(p2(0.0, -0.5))
        .arc_to(
            Bulge {
                p: p2(0.5, 0.0),
                b: FRAC_PI_8.tan(),
            },
            Tol::witness(),
        )
        .unwrap()
        .tangent()
        .tangent_arc_to(p2(0.0, 0.5), Tol::witness())
        .unwrap()
        .line_to(Start, Tol::witness())
        .unwrap();

    // 7. The far-end anchor: the arrival side ENDS at its authored point.
    let far_end = Open
        .at(p2(0.0, 0.0))
        .line_to(p2(3.0, 0.0), Tol::witness())
        .unwrap()
        .line_to(p2(3.0, 1.0), Tol::witness())
        .unwrap()
        .toward(-1.0, 0.0, Tol::witness())
        .unwrap()
        .fillet(0.5, Tol::witness())
        .unwrap()
        .toward(0.0, 1.0, Tol::witness())
        .unwrap()
        .to(p2(1.0, 3.0), Tol::witness())
        .unwrap()
        .line_to(p2(0.0, 3.0), Tol::witness())
        .unwrap()
        .line_to(Start, Tol::witness())
        .unwrap();

    // 8. The fused verb with an ARC arrival, closing on the far lobe.
    let tip = 0.75f64.sqrt();
    let eye = Open
        .arc_fillet_arc(
            Center {
                c: p2(-0.5, 0.0),
                winding: ArcSweep::Ccw,
                p: p2(0.0, -tip),
            },
            0.25,
            Center {
                c: p2(0.5, 0.0),
                winding: ArcSweep::Ccw,
                p: Start,
            },
            Tol::witness(),
        )
        .unwrap();

    // 9. The DECLARED cusp: the lune between two internally tangent
    //    circles, cut on the y axis so the region is the one lip —
    //    the cross-section of D1's kissing-cylinders figure. The
    //    junction at the kiss is authored by `.cusp()`, which reverses
    //    the arriving ray exactly; every other corner is a right
    //    angle, so nothing but the cusp is declared.
    let lune = Open
        .at(p2(0.0, 4.0))
        .angle(-FRAC_PI_2, Tol::witness())
        .unwrap()
        .line(2.0, Tol::witness())
        .unwrap()
        .turn(FRAC_PI_2, Tol::witness())
        .unwrap()
        .tangent_arc_to(p2(0.0, 0.0), Tol::witness())
        .unwrap()
        .cusp()
        .tangent_arc_to(Start, Tol::witness())
        .unwrap();

    // 12. The two arc modes the chains above never reach: the
    //     endpoint-free `ArcLen` leg off a directed tip (the extent
    //     authored as a length rather than a swept angle), and the
    //     three-point `Via` leg off the bare point it lands on.
    let mode_legs = Open
        .at(p2(0.0, 0.0))
        .angle(0.0, Tol::witness())
        .unwrap()
        .arc_to(
            ArcLen {
                r: 2.0,
                side: ArcSide::Left,
                len: 1.2,
            },
            Tol::witness(),
        )
        .unwrap()
        .arc_to(
            Via {
                q: p2(2.0, 1.5),
                p: p2(3.0, 0.5),
            },
            Tol::witness(),
        )
        .unwrap()
        .line_to(Start, Tol::witness())
        .unwrap();

    // 13. The DECLARED point-target continuation and its structural
    //     CLOSER: a square whose every side is subdivided at one
    //     interior vertex — four corners said on eight — with the seam
    //     cut at a corner and the last side crossing it. Each
    //     subdivision names the point it lands on and is checked
    //     against the ray it declared; the closer names `Start`. This
    //     is the shape the ruling was for, and the only chain here in
    //     which a straight run crosses the seam.
    let subdivided_square = Open
        .at(p2(0.0, 0.0))
        .angle(0.0, Tol::witness())
        .unwrap()
        .line(1.0, Tol::witness())
        .unwrap()
        .continue_to(p2(2.0, 0.0), Tol::witness())
        .unwrap()
        .turn(FRAC_PI_2, Tol::witness())
        .unwrap()
        .line(1.0, Tol::witness())
        .unwrap()
        .continue_to(p2(2.0, 2.0), Tol::witness())
        .unwrap()
        .turn(FRAC_PI_2, Tol::witness())
        .unwrap()
        .line(1.0, Tol::witness())
        .unwrap()
        .continue_to(p2(0.0, 2.0), Tol::witness())
        .unwrap()
        .turn(FRAC_PI_2, Tol::witness())
        .unwrap()
        .line(1.0, Tol::witness())
        .unwrap()
        .continue_to(Start, Tol::witness())
        .unwrap();

    // 14. The DECLARED STRAIGHT ARRIVAL at the seam (BOOL-12): Ev's
    //     D-shape, whose entry sits at a SUBDIVISION point of its one
    //     straight side. The closing leg declares both facts — its own
    //     departure continues the run, and its arrival continues the
    //     entry's first side — and each is checked, never inferred.
    let d_shape = Open
        .at(p2(0.0, 0.0))
        .angle(FRAC_PI_2, Tol::witness())
        .unwrap()
        .line(2.0, Tol::witness())
        .unwrap()
        .arc_to(
            Bulge {
                p: p2(0.0, -2.0),
                b: 1.0,
            },
            Tol::witness(),
        )
        .unwrap()
        .line_to(p2(0.0, -1.0), Tol::witness())
        .unwrap()
        .continue_to(Start.arrives_tangent(), Tol::witness())
        .unwrap();

    // 15. The DECLARED G1 ARRIVAL at the seam (BOOL-12): a stadium,
    //     tangent at all four joints. The closing cap's departure
    //     tangency CONSTRUCTS the arc and its arrival tangency is
    //     CHECKED, so the seam joint carries a declared flag the verify
    //     layer re-checks.
    let stadium = Open
        .at(p2(0.0, 0.0))
        .angle(0.0, Tol::witness())
        .unwrap()
        .line(2.0, Tol::witness())
        .unwrap()
        .tangent()
        .tangent_arc_to(p2(2.0, 2.0), Tol::witness())
        .unwrap()
        .tangent()
        .line(2.0, Tol::witness())
        .unwrap()
        .tangent()
        .tangent_arc_to(Start.arrives_tangent(), Tol::witness())
        .unwrap();

    // 16. The fused verb with a RADIUS ARRIVAL: the one chain here
    //     whose three radius arguments each draw a segment of their
    //     own — the incoming `Sweep` carrier, the fillet, and the
    //     arrival `Radius` carrier. It is the only shape that reaches
    //     the `Carrier2` emission role at all (`Sweep` and `ArcLen`
    //     are not admissible arrival specs, and `Via` and `Center`
    //     carry no radius), so without it the guided fence replays
    //     every verb and two of the three roles.
    let radius_arrival = Open
        .at(p2(0.0, 0.0))
        .angle(0.0, Tol::witness())
        .unwrap()
        .line(4.0, Tol::witness())
        .unwrap()
        .tangent()
        .arc_fillet_arc(
            Sweep {
                r: 2.0,
                side: ArcSide::Left,
                angle: 0.6,
            },
            0.25,
            Radius {
                r: 3.0,
                side: ArcSide::Left,
            },
            Tol::witness(),
        )
        .unwrap()
        .at(p2(2.0, 6.0))
        .toward(-1.0, 0.0, Tol::witness())
        .unwrap()
        .line(2.0, Tol::witness())
        .unwrap()
        .line_to(Start, Tol::witness())
        .unwrap();

    // 10/11. The complete-loop program forms.
    let circle = profile::circle(p2(1.0, 2.0), 0.75, Tol::witness()).unwrap();
    let split = profile::circle_split(p2(0.0, 0.0), 1.0, 5, 0.3, Tol::witness()).unwrap();

    vec![
        fused,
        walk,
        turned,
        seam,
        tangent_arc,
        subdivided,
        far_end,
        eye,
        lune,
        mode_legs,
        subdivided_square,
        d_shape,
        stadium,
        radius_arrival,
        circle,
        split,
    ]
}

// ------------------------------------------------------------------
// The arc-carrier fillet grids and the anchor-fit readers: FILLET-ATTR's
// grid A and the line×arc grid, homed once so a suite that walks them
// reads the same authorings by the same ordinals.
// ------------------------------------------------------------------

/// The point `angle` radians round the circle of radius `r` about
/// `centre`.
pub fn on_circle(centre: Point2<f64>, r: f64, angle: f64) -> Point2<f64> {
    p2(centre.x + r * angle.cos(), centre.y + r * angle.sin())
}

/// **Grid A's authoring** (PR 1895's parameters): the corner at the
/// origin, each carrier of radius `r_c` winding `tau` with the corner at
/// angle `a` about its centre, each far anchor `delta` radians from the
/// corner along its own leg, filleted at `r`. `case` is
/// `[a_in, r_in, tau_in, delta_in, a_out, r_out, tau_out, delta_out]`.
pub fn arc_arc(case: [f64; 8], r: f64) -> Result<ProfileLoop<f64>, PathError<f64>> {
    let [
        a_in,
        r_in,
        tau_in,
        delta_in,
        a_out,
        r_out,
        tau_out,
        delta_out,
    ] = case;
    let c1 = p2(-r_in * a_in.cos(), -r_in * a_in.sin());
    let c2 = p2(-r_out * a_out.cos(), -r_out * a_out.sin());
    let head = on_circle(c1, r_in, a_in - tau_in * delta_in);
    let next = on_circle(c2, r_out, a_out + tau_out * delta_out);
    let w = |t: f64| if t > 0.0 { ArcSweep::Ccw } else { ArcSweep::Cw };
    let closed = Open
        .arc_fillet_arc(
            Center {
                c: c1,
                winding: w(tau_in),
                p: head,
            },
            r,
            Center {
                c: c2,
                winding: w(tau_out),
                p: next,
            },
            Tol::witness(),
        )?
        .line_to(Start, Tol::witness())?;
    Ok(closed.loop_)
}

/// **Grid A**, PR 1895's grid verbatim: R_in in {0.2, 0.4, 0.15}, R_out
/// in {0.2, 0.15, 0.5}, tau in {+1, -1} on both sides, corner angle 0.4k
/// for k = 1..7, deltas in {0.3, 0.95π} × {0.3, 0.95π/2, 2.6} with 2.6
/// on both, r = 0.05m for m = 1..8 — 18 144 authorings, each visited
/// with its ordinal (from 1), its case, its radius and its outcome. The
/// ordinal is how a row names an authoring, so the enumeration order
/// here is part of the fixture.
pub fn grid_a(
    mut visit: impl FnMut(usize, [f64; 8], f64, &Result<ProfileLoop<f64>, PathError<f64>>),
) -> usize {
    let mut n = 0_usize;
    for r_in in [0.2, 0.4, 0.15] {
        for r_out in [0.2, 0.15, 0.5] {
            for tau_in in [1.0, -1.0] {
                for tau_out in [1.0, -1.0] {
                    for k in 1..=7 {
                        let a_out = 0.4 * f64::from(k);
                        for delta_in in [0.3, 0.95 * core::f64::consts::PI, 2.6] {
                            for delta_out in [0.3, 0.95 * core::f64::consts::PI / 2.0, 2.6] {
                                for m in 1..=8 {
                                    n += 1;
                                    let case = [
                                        0.0, r_in, tau_in, delta_in, a_out, r_out, tau_out,
                                        delta_out,
                                    ];
                                    let r = 0.05 * f64::from(m);
                                    visit(n, case, r, &arc_arc(case, r));
                                }
                            }
                        }
                    }
                }
            }
        }
    }
    n
}

/// **The line×arc authoring**: a ray from `(sx·R/2, 0)` east onto the
/// circle of radius `big_r` about the origin, anchored `ang` radians
/// round it, filleted at `r` and closed back to the start. The derived
/// corner is `(R, 0)`.
pub fn line_arc(
    big_r: f64,
    sx: f64,
    winding: ArcSweep,
    ang: f64,
    r: f64,
) -> Result<ProfileLoop<f64>, PathError<f64>> {
    Open.at(p2(sx * big_r / 2.0, 0.0))
        .toward(1.0, 0.0, Tol::witness())?
        .fillet_arc(
            r,
            Center {
                c: p2(0.0, 0.0),
                winding,
                p: on_circle(p2(0.0, 0.0), big_r, ang),
            },
            Tol::witness(),
        )?
        .line_to(Start, Tol::witness())
        .map(|c| c.loop_)
}

/// **The line×arc grid**: R ∈ {2, 1, 0.5}, sx ∈ {0.2, 0.8, 1.4, 1.9},
/// both windings, anchor angle ∈ {0.3, 1.0, 2.0, 2.9}, r = 0.05mR for
/// m = 1..10 — 960 authorings, visited in that order with their
/// outcome.
pub fn line_arc_grid(mut visit: impl FnMut(&Result<ProfileLoop<f64>, PathError<f64>>)) -> usize {
    let mut n = 0_usize;
    for big_r in [2.0, 1.0, 0.5] {
        for sx in [0.2, 0.8, 1.4, 1.9] {
            for winding in [ArcSweep::Ccw, ArcSweep::Cw] {
                for ang in [0.3, 1.0, 2.0, 2.9] {
                    for m in 1..=10 {
                        n += 1;
                        visit(&line_arc(
                            big_r,
                            sx,
                            winding,
                            ang,
                            0.05 * f64::from(m) * big_r,
                        ));
                    }
                }
            }
        }
    }
    n
}

/// Every anchor-fit entry of a refusal, in envelope order, as
/// `(corner, side, setback, available)`; empty for any other refusal.
pub fn anchor_fit_entries(err: &PathError<f64>) -> Vec<(Point2<f64>, FilletLeg, f64, f64)> {
    corners(err)
        .iter()
        .filter_map(|c| match c.reason {
            CornerReason::AnchorOutsideTrimmedExtent {
                side,
                setback,
                available,
                ..
            } => Some((c.at, side, setback, available)),
            _ => None,
        })
        .collect()
}

/// The anchor-fit entry at the corner `at` (to 1e-9), as
/// `(side, carrier, setback, available)`; a row asserting a corner's
/// numbers names the corner, so a wrong-corner entry reads as the
/// wrong refusal rather than as the wrong number.
pub fn anchor_fit_at(
    err: &PathError<f64>,
    at: (f64, f64),
) -> (FilletLeg, FilletLegCarrier, f64, f64) {
    corners(err)
        .iter()
        .find(|c| (c.at.x - at.0).abs() < 1e-9 && (c.at.y - at.1).abs() < 1e-9)
        .and_then(|c| match c.reason {
            CornerReason::AnchorOutsideTrimmedExtent {
                side,
                carrier,
                setback,
                available,
            } => Some((side, carrier, setback, available)),
            _ => None,
        })
        .unwrap_or_else(|| panic!("no anchor-fit entry at {at:?} in {err:?}"))
}

/// Two `f64`s within 1e-9 — the tolerance a measured payload number is
/// pinned at.
pub fn close(a: f64, b: f64) -> bool {
    (a - b).abs() < 1e-9
}
