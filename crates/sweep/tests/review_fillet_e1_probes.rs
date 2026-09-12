//! **FILLET-E1 review probes** — rows against the shared
//! `nonpositive_size_gate` both blend doors now run, written against
//! the public doors and the public refusal type only.
//!
//! What the unit's own flipped row (`review_blend6_r1_probes` probe 2)
//! pins is the FILLET door at `f64`. The rows here pin what it leaves
//! to prose:
//!
//! 1. **The preamble order** — the size gate runs BEFORE the
//!    repeated-edge gate on both doors, so a request that is malformed
//!    in both ways reads `NonpositiveSize`. Nothing else asserts the
//!    order; a swap compiles and passes every other row.
//! 2. **Door parity** — one nonpositive size yields one inner error at
//!    both doors: same variant, same payload (the bracket's low end,
//!    NaN carried as NaN), and texts that differ only in the verb
//!    prefix `V1` attaches.
//! 3. **The certified scalar** — the gate is a `Bounds::lo` read, so
//!    at `Interval` a poisoned (NaI) and a zero-straddling bracket must
//!    refuse too, and the payload is the LOW end. Runs only in the
//!    interval lane; the door's own row is `f64`-typed and never
//!    reaches this arm there.
//! 4. **Nothing is metered first** — at the recording scalar, a
//!    nonpositive size leaves the `k_stats` sink EMPTY: the refusal is
//!    a fact about the request and no predicate fires before it. Runs
//!    only under `--features probe`.

#![allow(clippy::unwrap_used, clippy::expect_used, clippy::panic)]

use geom_core::Tol;
use sweep::blend::build::fillet_edges;
use sweep::blend::{BlendError, BlendKind};
use sweep::chamfer::chamfer_edges;
use sweep::test_support::cube;
use topo::{Body, EdgeKey};

/// The three shapes the gate must refuse: zero, negative, poisoned.
const NONPOSITIVE: [f64; 3] = [0.0, -0.1, f64::NAN];

fn all_edges<T: geom_core::Decide>(body: &Body<T>) -> Vec<EdgeKey> {
    body.edges().map(|(k, _)| k).collect()
}

/// `a == b`, with NaN equal to NaN — the payload carries the caller's
/// poison as poison, and `==` cannot see that.
fn same_f64(a: f64, b: f64) -> bool {
    a == b || (a.is_nan() && b.is_nan())
}

/// **The size gate runs before the repeated-edge gate, on both doors.**
/// A request that is both nonpositive and repeats an edge is malformed
/// twice over; which refusal the caller reads is a fact about the
/// preamble's order, and this row is the only thing that pins it. The
/// chamfer door has always read `NonpositiveSize` here; the fillet
/// door reads the same now that the two share one preamble.
#[test]
fn the_size_gate_runs_before_the_repeated_edge_gate_on_both_doors() {
    let t = Tol::witness();
    let body = cube(1.0, t);
    let mut edges = all_edges(&body);
    edges.push(edges[0]);
    for size in NONPOSITIVE {
        let f = fillet_edges(&body, &edges, size, t).expect_err("nonpositive AND repeated");
        assert!(
            matches!(f.error, BlendError::NonpositiveSize { .. }),
            "fillet: the size gate must answer before the repeated-edge gate at {size}: {f:?}"
        );
        let c = chamfer_edges(&body, &edges, size, t).expect_err("nonpositive AND repeated");
        assert!(
            matches!(c.error, BlendError::NonpositiveSize { .. }),
            "chamfer: the size gate must answer before the repeated-edge gate at {size}: {c:?}"
        );
    }
    // The control: the same repeated request at a positive size reads
    // the repeated edge, so the row above is about ORDER and not about
    // the repeated-edge gate having gone quiet.
    let f = fillet_edges(&body, &edges, 0.1, t).expect_err("a repeated edge is malformed");
    assert!(
        matches!(f.error, BlendError::RepeatedEdge { .. }),
        "the repeated-edge gate still answers once the size is positive: {f:?}"
    );
}

/// **One nonpositive size, one inner error at both doors.** The gate
/// is shared, so the fillet and the chamfer must agree on the variant,
/// on the payload — the size as handed in, its bracket's low end, with
/// NaN carried as NaN — and on every word of the sentence after the
/// verb prefix.
#[test]
fn both_doors_mint_one_refusal_for_one_nonpositive_size() {
    let t = Tol::witness();
    let body = cube(1.0, t);
    let edges = all_edges(&body);
    for size in NONPOSITIVE {
        let f = fillet_edges(&body, &edges, size, t).expect_err("a nonpositive radius refuses");
        let c = chamfer_edges(&body, &edges, size, t).expect_err("a nonpositive setback refuses");
        assert!(matches!(f.verb, BlendKind::Fillet), "{f:?}");
        assert!(matches!(c.verb, BlendKind::Chamfer), "{c:?}");
        match (&f.error, &c.error) {
            (
                BlendError::NonpositiveSize { size: fs },
                BlendError::NonpositiveSize { size: cs },
            ) => {
                assert!(
                    same_f64(*fs, size) && same_f64(*cs, size),
                    "the payload is the size as handed in ({size}): fillet {fs}, chamfer {cs}"
                );
            }
            other => panic!("both doors must refuse NonpositiveSize at {size}: {other:?}"),
        }
        let (ft, ct) = (f.to_string(), c.to_string());
        assert_eq!(
            ft.strip_prefix("fillet: "),
            ct.strip_prefix("chamfer: "),
            "after the verb prefix the two doors read the same sentence"
        );
        assert!(ft.strip_prefix("fillet: ").is_some(), "{ft}");
    }
}

/// **CHARACTERIZATION — a positive size under the band's zero reaches
/// a false-fact refusal at both doors.** The gate's rule is `> 0`, not
/// `> ε`, so a size the band cannot tell from zero passes it and the
/// battery meters it: predicate 1's margin is `r − r²/arm`, which
/// saturates at `r` on a plane, so at `r = 1e-12` (three decades under
/// the band's zero) the fillet classifies `Zero` and the caller reads
/// "radius 1e-12 m exceeds the curvature headroom of [a plane] —
/// reduce the fillet radius", false in both halves; the chamfer reads
/// `DependentNormals` about an orthonormal cube corner, the levered
/// `|det|·d` the `NonpositiveSize` doc names.
///
/// This row pins that behaviour AS IT IS so the class is measured
/// rather than remembered. It is the witness that flips when
/// `work/fillet/blend-size-gate-unmetered-under-epsilon.md` is taken:
/// whichever way that unit decides (meter the size against the band,
/// or keep `> 0` and narrow the promise), this row goes red and is
/// rewritten to the decided behaviour.
#[test]
fn a_positive_size_under_epsilon_reads_a_false_fact_at_both_doors_today() {
    let t = Tol::witness();
    let body = cube(1.0, t);
    let edges = all_edges(&body);
    let size = 1e-12;

    let f = fillet_edges(&body, &edges, size, t).expect_err("today a sub-band radius refuses");
    assert!(
        matches!(f.error, BlendError::RadiusHeadroom { radius, .. } if radius == size),
        "today the fillet meters predicate 1 at {size} m: {f:?}"
    );
    let ft = f.to_string();
    assert!(
        ft.contains("curvature headroom") && ft.contains("reduce the fillet radius"),
        "and reads the headroom sentence a plane cannot owe: {ft}"
    );

    let c = chamfer_edges(&body, &edges, size, t).expect_err("today a sub-band setback refuses");
    assert!(
        matches!(
            c.error,
            BlendError::UnsupportedCorner {
                corner: sweep::blend::CornerConfig::DependentNormals,
                ..
            }
        ),
        "today the chamfer reads the levered corner determinant at {size} m: {c:?}"
    );
}

/// The gate at the CERTIFIED scalar: a `Bounds::lo` read over real
/// brackets, not points.
#[cfg(feature = "interval")]
mod certified {
    use super::{all_edges, same_f64};
    use geom_core::{Bounds, Interval, Point2, Real, Tol};
    use profile::{Profile, ProfileLoop, RawLoop, SketchPlane};
    use sweep::blend::BlendError;
    use sweep::blend::build::fillet_edges;
    use sweep::chamfer::chamfer_edges;
    use sweep::{Extrusion, extrude};
    use topo::Body;

    fn iv(x: f64) -> Interval {
        Interval::from_f64(x)
    }

    fn cube() -> Body<Interval> {
        let lp = <ProfileLoop<Interval> as RawLoop<Interval>>::polygon([
            Point2::new(iv(0.0), iv(0.0)),
            Point2::new(iv(1.0), iv(0.0)),
            Point2::new(iv(1.0), iv(1.0)),
            Point2::new(iv(0.0), iv(1.0)),
        ]);
        let profile = Profile::new(SketchPlane::xy(), vec![lp])
            .validate(Tol::witness())
            .expect("a square validates");
        extrude(&profile, Extrusion::Distance(iv(1.0)), Tol::witness())
            .expect("the cube extrudes")
            .body
    }

    /// **Zero, negative, poisoned and STRADDLING brackets all refuse at
    /// both doors, and the payload is the bracket's low end.** The
    /// straddling row is the one `f64` cannot express: a size whose
    /// enclosure is `[-1e-3, 1e-3]` is not definitely positive, and
    /// the payload names the end that fails.
    #[test]
    fn a_not_definitely_positive_bracket_refuses_with_its_low_end() {
        let t = Tol::witness();
        let body = cube();
        let edges = all_edges(&body);
        let sizes = [
            iv(0.0),
            iv(-0.1),
            iv(f64::NAN),
            Interval::from_bounds(-1e-3, 1e-3),
            Interval::from_bounds(-2.0, -1.0),
        ];
        for size in sizes {
            let f = fillet_edges(&body, &edges, size, t).expect_err("not definitely positive");
            let c = chamfer_edges(&body, &edges, size, t).expect_err("not definitely positive");
            for (door, e) in [("fillet", &f.error), ("chamfer", &c.error)] {
                match e {
                    BlendError::NonpositiveSize { size: s } => assert!(
                        same_f64(*s, size.lo()),
                        "{door}: payload {s} must be the low end {} of {size:?}",
                        size.lo()
                    ),
                    other => panic!("{door} at {size:?} must refuse NonpositiveSize: {other:?}"),
                }
            }
        }
    }

    /// **A definitely positive bracket passes the gate** — whatever
    /// the battery says next, it is not the size gate speaking.
    #[test]
    fn a_definitely_positive_bracket_passes_the_gate() {
        let t = Tol::witness();
        let body = cube();
        let edges = all_edges(&body);
        let size = Interval::from_bounds(0.1 - 1e-9, 0.1 + 1e-9);
        for (door, r) in [
            ("fillet", fillet_edges(&body, &edges, size, t).map(|_| ())),
            ("chamfer", chamfer_edges(&body, &edges, size, t).map(|_| ())),
        ] {
            if let Err(e) = r {
                assert!(
                    !matches!(e.error, BlendError::NonpositiveSize { .. }),
                    "{door}: a definitely positive bracket must not be refused as nonpositive: {e:?}"
                );
            }
        }
    }
}

/// The gate at the RECORDING scalar: what is metered before it answers.
#[cfg(feature = "probe")]
mod recorded {
    use super::all_edges;
    use geom_core::k_stats::{self, Probe};
    use geom_core::{Point2, Tol};
    use profile::{Profile, ProfileLoop, ProfileVertex, RawLoop, SketchPlane};
    use sweep::blend::BlendError;
    use sweep::blend::build::fillet_edges;
    use sweep::chamfer::chamfer_edges;
    use sweep::{Extrusion, extrude};
    use topo::Body;

    fn cube() -> Body<Probe> {
        let lp: ProfileLoop<Probe> = ProfileLoop::new(
            [(0.0, 0.0), (1.0, 0.0), (1.0, 1.0), (0.0, 1.0)]
                .into_iter()
                .map(|(x, y)| ProfileVertex::new(Point2::new(Probe(x), Probe(y)), Probe(0.0)))
                .collect(),
        );
        let profile = Profile::new(SketchPlane::xy(), vec![lp])
            .validate(Tol::witness())
            .expect("a square validates");
        extrude(&profile, Extrusion::Distance(Probe(1.0)), Tol::witness())
            .expect("the cube extrudes")
            .body
    }

    /// **A nonpositive size is refused before any predicate fires.** The
    /// sink is empty after the refusal at both doors; the positive
    /// control at the end shows the sink was live (a positive size
    /// meters the battery).
    #[test]
    fn a_nonpositive_size_meters_nothing_before_it_refuses() {
        let t = Tol::witness();
        let body = cube();
        let edges = all_edges(&body);
        for size in [0.0, -0.1, f64::NAN] {
            for door in ["fillet", "chamfer"] {
                k_stats::start_recording();
                let e = match door {
                    "fillet" => fillet_edges(&body, &edges, Probe(size), t).map(|_| ()),
                    _ => chamfer_edges(&body, &edges, Probe(size), t).map(|_| ()),
                }
                .expect_err("a nonpositive size refuses");
                let samples = k_stats::take_samples();
                assert!(
                    matches!(e.error, BlendError::NonpositiveSize { .. }),
                    "{door} at {size}: {e:?}"
                );
                assert!(
                    samples.is_empty(),
                    "{door} at {size}: {} predicate(s) metered before the door refused: {:?}",
                    samples.len(),
                    samples.iter().map(|s| s.predicate).collect::<Vec<_>>()
                );
            }
        }
        // The control: the recorder is live, and a positive size does
        // meter the battery.
        k_stats::start_recording();
        fillet_edges(&body, &edges, Probe(0.1), t).expect("the cube fillets");
        assert!(
            !k_stats::take_samples().is_empty(),
            "the positive control must meter something, or the rows above prove nothing"
        );
    }
}
