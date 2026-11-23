//! **The replay driver at scalars other than `f64`.**
//!
//! [`profile::replay`] is generic over `ArcCarrierScalar` (`Decide +
//! Bounds`) and type-checks at every evaluation scalar, but the whole
//! shipped corpus instantiates it at `f64` alone. A generic path with
//! one instantiation is a compile-time claim, not an exercised one:
//! every `from_f64`, every band read, every `Bounds` read inside the
//! fused fillet family is unmeasured off the value lane until something
//! runs there.
//!
//! These rows run the verb-coverage corpus — the same closed chains
//! `path_program.rs` uses to prove every declared verb has a replayed
//! arm — through the driver at `Dual64` and at `Interval`, and pin what
//! each lane owes the f64 lane:
//!
//! - **`Dual64`**: the value channel is bit-identical to `f64`. `Dual`
//!   delegates every value operation exactly and a derivative never
//!   influences a branch, so a differing bit is a real divergence, not
//!   a tolerance question. The tangent channel of a constant-seeded
//!   replay is identically zero — which is exactly the seam a profile
//!   parameter lift exists to open, so it is pinned as the STATE, not
//!   as a desideratum.
//! - **`Interval`**: every emitted coordinate and bulge ENCLOSES the
//!   f64 lane's answer. Containment, not equality, is the interval
//!   lane's contract; a lane that certifies a box excluding the f64
//!   answer is unsound.
//!
//! Neither row is a structure-selection claim. The corpus is authored
//! macroscopically, so every discrete decision inside replay decides
//! definitely at both lanes here; the hairline cases where they need
//! not agree are the guided path's business, not this suite's.
#![allow(clippy::unwrap_used, clippy::expect_used, clippy::panic)]

use crate::common;

use common::{coverage_corpus, tol, try_replay_at};
use geom_core::{Dual64, Point2, Real};
use profile::{ConstructedLoop, ReplayError, Step};

/// One corpus row's program, embedded and replayed at `T`.
fn replay_at<T: profile::ArcCarrierScalar>(program: &[Step<f64>]) -> ConstructedLoop<T> {
    try_replay_at(program)
        .unwrap_or_else(|e| panic!("the corpus program refused at the lifted scalar: {e}"))
}

/// The `Dual64` instantiation: same structure, same value bits, zero
/// tangent.
///
/// The zero tangent is the load-bearing observation, not an incidental
/// one: a constant-seeded replay carries no derivative anywhere, which
/// is precisely why a `Dual` seed on a profile dimension propagates
/// nothing today. This row records that state at the driver.
#[test]
fn the_corpus_replays_at_dual_with_bit_identical_values() {
    for (i, closed) in coverage_corpus().into_iter().enumerate() {
        let base = replay_at::<f64>(&closed.program);
        let dual = replay_at::<Dual64>(&closed.program);
        assert_eq!(
            base.vertices().len(),
            dual.vertices().len(),
            "row {i}: vertex count"
        );
        for (k, (a, b)) in base.vertices().iter().zip(dual.vertices()).enumerate() {
            assert_eq!(
                a.x.to_bits(),
                b.x.value.to_bits(),
                "row {i} vertex {k}: x value channel"
            );
            assert_eq!(
                a.y.to_bits(),
                b.y.value.to_bits(),
                "row {i} vertex {k}: y value channel"
            );
            let mut tangents = vec![("x", b.x.deriv), ("y", b.y.deriv)];
            match (base.segments()[k], dual.segments()[k]) {
                (profile::Segment::Line, profile::Segment::Line) => {}
                (profile::Segment::Arc(f), profile::Segment::Arc(d)) => {
                    for (what, f, d) in [
                        ("centre x", f.centre.x, d.centre.x),
                        ("centre y", f.centre.y, d.centre.y),
                        ("radius", f.radius, d.radius),
                        ("sweep", f.sweep, d.sweep),
                    ] {
                        assert_eq!(
                            f.to_bits(),
                            d.value.to_bits(),
                            "row {i} segment {k}: {what} value channel"
                        );
                        tangents.push((what, d.deriv));
                    }
                }
                (f, d) => panic!("row {i} segment {k}: {f:?} at f64 vs {d:?} at Dual"),
            }
            for (what, d) in tangents {
                assert_eq!(
                    d, 0.0,
                    "row {i} vertex {k}: {what} tangent — a constant-seeded \
                     replay carries no derivative"
                );
            }
        }
        assert_eq!(
            base.constructed_joints(),
            dual.constructed_joints(),
            "row {i}: constructed tangent joints"
        );
    }
}

/// The `Interval` instantiation: the f64 answer lies inside every
/// emitted enclosure, and the declared structure is the same.
///
/// Every row reaches that comparison today. That is a fact about the
/// corpus, not a property of the loop, so the `continue` below is
/// backed by a census rather than left to be trusted — see
/// [`no_corpus_row_escalates_at_interval`], which pins the escalating
/// set as EMPTY and says what a row joining it would mean.
#[test]
fn the_corpus_replays_at_interval_and_encloses_the_f64_lane() {
    use geom_core::{Bounds, Interval};
    for (i, closed) in coverage_corpus().into_iter().enumerate() {
        let base = replay_at::<f64>(&closed.program);
        // A row that does not reach the comparison is not skipped
        // quietly: the SET of such rows is pinned, by name, in
        // `no_corpus_row_escalates_at_interval` below, which asserts it
        // is empty. Without that companion this `continue` would let the
        // whole corpus fall out of the enclosure claim one row at a time
        // and still pass.
        let Ok(iv) = try_replay_at::<Interval>(&closed.program) else {
            continue;
        };
        assert_eq!(
            base.vertices().len(),
            iv.vertices().len(),
            "row {i}: vertex count"
        );
        for (k, (a, b)) in base.vertices().iter().zip(iv.vertices()).enumerate() {
            let mut channels = vec![("x", a.x, b.x), ("y", a.y, b.y)];
            if let (profile::Segment::Arc(f), profile::Segment::Arc(e)) =
                (base.segments()[k], iv.segments()[k])
            {
                channels.push(("sweep", f.sweep, e.sweep));
            }
            for (what, exact, enc) in channels {
                assert!(
                    enc.lo() <= exact && exact <= enc.hi(),
                    "row {i} vertex {k}: the {what} enclosure [{}, {}] excludes the \
                     f64 lane's {exact}",
                    enc.lo(),
                    enc.hi()
                );
            }
        }
        assert_eq!(
            base.constructed_joints(),
            iv.constructed_joints(),
            "row {i}: constructed tangent joints"
        );
    }
}

/// **The census the interval-lane wall left behind.** No corpus row
/// escalates at `Interval`: the whole coverage corpus replays, and the
/// enclosure claim its companion above makes therefore covers all of
/// it. The set is pinned as EMPTY, and pinned by SHAPE — what this row
/// watches for is a row JOINING it.
///
/// # What used to be here, and what closed it
///
/// One row escalated: the rocker eye, a fused `ArcFilletArc` whose two
/// carriers are circles about (∓½, 0) through (0, −√3⁄2). That anchor
/// is itself one of the pair's two intersections, so the derived corner
/// list contains the anchor exactly — by design, and bitwise, since the
/// squared-radius rule makes a derived corner reproduce an authored
/// one. The incoming advance gate measures the signed swept angle from
/// the anchor TO the anchor: exactly `0.0` at `f64`, which classifies
/// definitely Zero and discards that corner. At `Interval` the two
/// angular coordinates are `atan2` enclosures of two separately-rounded
/// points, so the difference straddles zero — and the signed sweep used
/// to pass it through TWO successive `floor`-based reductions, `x mod
/// τ` and then the signed fold, each of which spans an integer on a box
/// straddling its own jump. The gate saw `[−τ, τ]` and no band could
/// classify it. The signed sweep now folds the raw difference ONCE,
/// through a window whose jump is at ±π rather than at 0
/// ([`geom_core::Real::reduce_periodic_centred`]), so a hairline
/// difference comes back a hairline;
/// [`the_anchor_coincident_corner_reduces_to_input_width_at_interval`]
/// is the width row that holds it there.
///
/// # The class, stated by SHAPE rather than by helper name
///
/// The class is **any floor-based period fold evaluated at `Interval`
/// whose argument box straddles a step of the `floor`** — `x mod τ`
/// (`reduce_periodic`), the centred fold `x − τ⌊x/τ + ½⌋`, and every
/// open-coded `((a − b)/p + ½).floor()` that means the same thing.
/// `floor` is a step function, so a box spanning one of its steps
/// enclosing two integers is not a looseness to be tightened away: it
/// is the honest enclosure of a discontinuous function, and the
/// widening is proportional to the PERIOD, not to the input box. What
/// IS a defect is a fold whose jump has been put where the live values
/// are — the composition above being the worst form of it, since the
/// inner fold hands the outer one a box already a period wide.
///
/// So a row joining this set is one of two things, and the diagnostic
/// says which: an escalation naming an ANGULAR gate is a new instance
/// of the class, and anything else is an unrelated finding this census
/// has caught in passing. Naming the class by helper was the first
/// survey's mistake — grepping `reduce_periodic` alone misses every
/// open-coded fold, and grepping this crate alone misses `topo`, which
/// carries most of them. The tree-wide hit list and each site's
/// disposition ride the class issue filed for it (evgunter/cad#1191),
/// not this comment.
///
/// # What the path door RELAYS, and why it is not a row of this census
///
/// The door reads every fillet arc it is about to emit the way
/// `Profile::validate` reads it — `seg::build_seg` on the stored
/// segment, `seg::joint_tangency` on each joint the fillet declares —
/// and an in-band classification leaves as `PathError::Escalated`
/// carrying that predicate verbatim. So an escalation naming one of
/// those classifications is validation's OWN verdict about the loop,
/// arriving at the door instead of after it: the same refusal, earlier,
/// and the loop it withholds is one nothing downstream could have used.
///
/// No corpus row is in that state at any ε row: the read decides each
/// declared joint by its turn levered by the shorter leg's arm, and
/// the fillet's own arm is the short one, over which its construction's
/// rounding stays far inside ε.
///
/// The census therefore keeps its teeth where its subject is: the
/// escalations that are NOT the door relaying a stored-form
/// classification are pinned EMPTY, exactly as before.
#[test]
fn no_corpus_row_escalates_at_interval() {
    use geom_core::Interval;
    use profile::Verb;
    /// The gate of a replay refusal raised at the door's stored-form
    /// read: the path door relaying validation's own verdict about the
    /// loop it was about to emit (see this row's docs) rather than a
    /// fact about replaying at another scalar.
    fn stored_form_predicate<T: Real>(e: &ReplayError<T>) -> Option<&'static str> {
        match &e.kind {
            profile::ReplayErrorKind::Path(profile::PathError::Escalated {
                site: profile::EscalatedAt::StoredForm,
                gate,
                ..
            }) => Some(gate.predicate()),
            _ => None,
        }
    }
    // The relayed set is PINNED, not merely printed, and it is empty:
    // a relay joining it is a new fact about the door and reds this
    // row, so there is no standing pass for the stored-form site.
    let pinned: &[(usize, &str)] = &[];
    let mut escalated: Vec<(usize, Vec<Verb>, String)> = Vec::new();
    let mut relayed: Vec<(usize, String)> = Vec::new();
    for (i, closed) in coverage_corpus().into_iter().enumerate() {
        let verbs: Vec<Verb> = closed.program.iter().map(Step::verb).collect();
        let Err(e) = try_replay_at::<Interval>(&closed.program) else {
            continue;
        };
        match stored_form_predicate(&e) {
            Some(predicate) => relayed.push((i, predicate.to_string())),
            None => escalated.push((i, verbs, format!("{e}"))),
        }
    }
    let seen: Vec<(usize, &str)> = relayed.iter().map(|(i, p)| (*i, p.as_str())).collect();
    assert_eq!(
        seen,
        pinned,
        "the corpus rows the path door relays a stored-form classification for are pinned \
         at eps = {:e}; a row joining or leaving that set is a change in what the door \
         withholds, and belongs in a PR body rather than in a silent exemption",
        tol().eps()
    );
    assert!(
        escalated.is_empty(),
        "the interval-lane escalating set is pinned EMPTY and a row joined it: \
         {escalated:#?} — an escalation naming an angular gate is a new instance \
         of the period-fold widening class (see this test\'s rustdoc); anything \
         else is an unrelated finding this census caught in passing. Rows the \
         path door relayed from validation's own classifiers are counted \
         separately and are not this set: {relayed:#?}"
    );
}

/// **The live instance of issue 1191, driven as a width row.** The
/// rocker eye's incoming advance gate measures the signed swept angle
/// from the entry anchor to a derived corner that reproduces that
/// anchor bitwise. At `Interval` the two `atan2` coordinates are
/// enclosures of separately-rounded points, so the difference handed to
/// the fold straddles ZERO — and this row asserts that what comes back
/// out is a box the width of that difference rather than a box the
/// width of the period.
///
/// # The ceiling is RELATIVE, because the quantity it bounds is
///
/// An enclosure width scales with the coordinates it encloses: the same
/// correct reformulation on a fixture 1000× larger returns boxes 1000×
/// wider, in metres, having lost nothing. An absolute ceiling would
/// therefore be a statement about this fixture's size and not about the
/// fold — it passes at unit scale for a reason that has nothing to do
/// with what the row claims, and a fixture scaled up would red it
/// while the kernel was working perfectly. So the ceiling below is a
/// multiple of the fixture's own scale, and the row runs at three
/// scales to make the relativity operative rather than merely stated.
///
/// It **consults no tolerance** — not an ε, not a band; nothing about
/// the verdict changes with the tolerance the suite runs at. The
/// separation it needs is enormous and is what makes the loose constant
/// safe: an input-width answer is ~1e-16 relative, and a regression to
/// the composed fold returns a whole period — at unit scale ~6.3, i.e.
/// sixteen orders up. Any constant in between distinguishes them.
#[test]
fn the_anchor_coincident_corner_reduces_to_input_width_at_interval() {
    use geom_core::{Bounds, Interval};
    use profile::Verb;

    /// The eye's one fused step, with every length scaled by `s` — the
    /// same geometry, read at a different size. A scale, not a scalar
    /// lift: `Step::map_scalar` would move the angles and bulges too, so
    /// this walks the lengths itself and refuses any step or mode whose
    /// fields it has not sorted into the two.
    fn scaled(step: &Step<f64>, s: f64) -> Step<f64> {
        use profile::{ArcData, Target};
        let pt = |p: Point2<f64>| Point2::new(p.x * s, p.y * s);
        let tgt = |t: Target<f64>| match t {
            Target::Start => Target::Start,
            Target::StartArriving => Target::StartArriving,
            Target::Point(p) => Target::Point(pt(p)),
        };
        let spec = |d: ArcData<f64>| match d {
            ArcData::Center { c, winding, target } => ArcData::Center {
                c: pt(c),
                winding,
                target: tgt(target),
            },
            other => panic!("the eye authors a Center-mode arc, got {other:?}"),
        };
        match *step {
            Step::ArcFilletArc {
                spec: a,
                radius,
                spec2,
            } => Step::ArcFilletArc {
                spec: spec(a),
                radius: radius * s,
                spec2: spec(spec2),
            },
            ref other => panic!("the eye is one fused step, got {other:?}"),
        }
    }

    let eye = coverage_corpus()
        .into_iter()
        .find(|c| c.program.iter().map(Step::verb).eq([Verb::ArcFilletArc]))
        .expect("the corpus carries the one-step fused eye");

    // Relative: widths are compared against the scale of the geometry
    // that produced them. A period-width answer is ~6.3 ABSOLUTE and so
    // fails this at every scale; an input-width answer is ~1e-16
    // relative and passes at every scale.
    const RELATIVE_CEILING: f64 = 1e-12;
    // **The premise is per ε band.** What this row is about is the WIDTH
    // of the enclosures a successful replay produces — input-width, not
    // period-width. Whether the replay succeeds at all is a different
    // question and one the run's tolerance owns: the fillet places its
    // tangent point by dividing by an offset lever, and at a tight
    // ambient ε that lever is too short for the larger scales (measured
    // at ε = 1e-12: scale 1 replays, scale 100 refuses with the lever's
    // own typed message). That refusal is the geometry layer being
    // honest about a corner it cannot place, not the signed fold
    // regressing, so this row steps past it and keeps its claim on
    // every scale that does replay.
    let mut replayed = 0usize;
    for scale in [1.0f64, 100.0, 1000.0] {
        let program: Vec<Step<f64>> = eye.program.iter().map(|st| scaled(st, scale)).collect();
        let iv = match try_replay_at::<Interval>(&program) {
            Ok(iv) => iv,
            Err(e) => {
                println!("eye at scale {scale}: typed refusal at this eps — {e}");
                continue;
            }
        };
        replayed += 1;
        let mut widest = 0.0f64;
        let mut widest_rel = 0.0f64;
        for (k, v) in iv.vertices().iter().enumerate() {
            let mut channels = vec![("x", v.x, true), ("y", v.y, true)];
            // The sweep is an ANGLE — dimensionless, so it does not
            // scale and is measured against 1, not against `scale`.
            if let profile::Segment::Arc(arc) = iv.segments()[k] {
                channels.push(("sweep", arc.sweep, false));
            }
            for (what, enc, is_length) in channels {
                let w = enc.hi() - enc.lo();
                let unit = if is_length { scale } else { 1.0 };
                let rel = w / unit;
                widest = widest.max(w);
                widest_rel = widest_rel.max(rel);
                assert!(
                    rel <= RELATIVE_CEILING,
                    "at scale {scale}, vertex {k}'s {what} enclosure is {w:e} wide \
                     ([{}, {}]) = {rel:e} relative — a period-width enclosure, not an \
                     input-width one",
                    enc.lo(),
                    enc.hi()
                );
            }
        }
        assert!(
            widest > 0.0,
            "at scale {scale}: the eye's enclosures are all degenerate"
        );
        println!(
            "eye at scale {scale}: widest absolute {widest:e}, widest relative {widest_rel:e}"
        );
    }
    // Anti-vacuity: a run in which nothing replayed has asserted
    // nothing about enclosure width, and must not read as green.
    assert!(
        replayed > 0,
        "no scale replayed at all, so the input-width claim was never exercised"
    );
}
