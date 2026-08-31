//! **Rational walls whose skinned direction carries INTERIOR knots at
//! non-dyadic parameters** — the row the rational patch-flux lane had
//! no coverage for (issue 453).
//!
//! `m8_3_rational_volume.rs`, the only closed-solid rational-wall
//! volume rows before this file, lofts THREE sections at
//! `v_degree = 2`: three control points, clamped, and therefore ZERO
//! interior v knots. A skin through more stations has interior knots,
//! and `skin.rs`'s averaged chord-length parameters (Book Eq. 9.8/10.8)
//! put them at generic parameters rather than on the quadrature grid.
//!
//! Why that used to be the whole difficulty: the composite rounds cut
//! the trim rectangle into `pieces²` UNIFORM cells, so a cell either
//! contained an interior knot in its open interior — where the
//! integrand may genuinely jump and only the smoothness-free hull rule
//! applies — or it did not. The hull rule's width is a control-net
//! fact and control-net facts are span-granular, so once cells were
//! finer than a span the straddling cells stopped shrinking and the
//! whole enclosure inherited a Θ(1/pieces) floor. Halving per round
//! instead of quartering is not convergence, and the schedule is
//! fixed, so refinement provably could not reach the target.
//!
//! The cells are cut ON the interior knots now, so no cell straddles
//! one and the floor is gone. These rows pin that: the enclosure meets
//! the run's own target on a body that missed it by two orders before.
//!
//! # ε posture
//!
//! Same discipline as `m8_3_rational_volume.rs`: the schedule is fixed
//! (D9), so a body that certifies at one ε honestly may refuse at a
//! tighter one, and every row pins all three honest outcomes rather
//! than widening a target. What is asserted UNCONDITIONALLY is the
//! structure — the knots really are interior and really are off-grid —
//! because that is the hypothesis of the row, and a fixture that
//! quietly stopped producing off-grid knots would otherwise pass by
//! testing nothing.
#![allow(clippy::unwrap_used, clippy::expect_used, clippy::panic)]

use geom_brep::PropsError;
use geom_core::Tol;
use geom_core::{Affine3, Point2, Vec3};
use profile::RawLoop;
use profile::{Profile, ProfileLoop, ProfileVertex, SketchPlane};
use sweep::{Section, loft_body};
use topo::{MassProperties, MassPropsError};

/// Mirrored from `geom_brep::props::quad` (private there).
const QUAD_TARGET_LEN_FACTOR: f64 = 1024.0;

/// A unit square with a quarter-circle bulge on the `+x` side — the
/// same arc-bearing section `m8_3_rational_volume.rs` lofts, so the
/// wall is RATIONAL (weights `1, cos 22.5°, 1` over two 45° sub-arcs)
/// and nothing but the station count differs.
fn arc_section(s: f64) -> Section {
    let v = |x: f64, y: f64, bulge: f64| ProfileVertex::new(Point2::new(x, y), bulge);
    vec![ProfileLoop::new(vec![
        v(-s, -s, 0.0),
        // tan(π/8): a quarter-circle bulge-out.
        v(s, -s, 0.4142135623730951),
        v(s, s, 0.0),
        v(-s, s, 0.0),
    ])]
}

/// The total height every blade below is lofted over.
const BLADE_HEIGHT: f64 = 2.0;

/// The blade: `n` IDENTICAL arc sections, evenly stacked over
/// `[0, BLADE_HEIGHT]`, skinned at `v_degree`.
///
/// Two things are deliberate.
///
/// **Identical sections, even stacking** buys the strongest oracle
/// available: the loft reproduces the EXTRUSION of the same profile
/// over the same height exactly, and that extrusion's bulged wall is
/// an analytic `Surface::Cylinder` with a closed-form volume and pad
/// exactly 0 — a different surface representation, a different props
/// lane, no shared arithmetic with the quadrature under test. So the
/// station count is the ONLY variable across these rows: the locus is
/// one solid, described `n` different ways.
///
/// **The station count then chooses the knots.** `skin.rs`
/// parameterizes by averaged chord length (Book Eq. 9.8/10.8), so `n`
/// evenly stacked sections parameterize at `k/(n − 1)` and the
/// interior knots are running means of those. `n − 1` a power of two
/// puts every knot on the composite's own dyadic grid; anything else
/// puts most of them off it. That is the issue's own variable, and it
/// is why the rows below differ only in `n`.
fn blade(n: usize, v_degree: usize) -> sweep::Lofted<f64> {
    let sections: Vec<Section> = (0..n).map(|_| arc_section(1.0)).collect();
    #[allow(clippy::cast_precision_loss)]
    let places: Vec<Affine3<f64>> = (0..n)
        .map(|k| {
            Affine3::translation(Vec3::new(
                0.0,
                0.0,
                BLADE_HEIGHT * k as f64 / (n - 1) as f64,
            ))
        })
        .collect();
    loft_body::<f64>(&sections, &places, v_degree, Tol::witness()).expect("the blade lofts")
}

/// The three honest outcomes, as `m8_3_rational_volume.rs` names them.
#[derive(Debug, PartialEq, Eq)]
enum EpsPosture {
    Certified,
    Budget,
    Escalated,
}

fn body_posture(row: &str, out: &Result<MassProperties<f64>, MassPropsError>) -> EpsPosture {
    let target = QUAD_TARGET_LEN_FACTOR * Tol::witness().get().eps;
    match out {
        Ok(_) => EpsPosture::Certified,
        Err(MassPropsError::Face {
            source:
                PropsError::QuadratureBudget {
                    width_len,
                    target_len,
                },
            ..
        }) => {
            assert!(
                width_len.is_finite() && width_len > target_len,
                "{row}: a budget refusal must carry a width that really missed: \
                 {width_len:e} vs {target_len:e}"
            );
            assert!(
                (target_len - target).abs() <= target * 1e-12,
                "{row}: the refused target must BE 1024·ε for this run: \
                 {target_len:e} vs {target:e}"
            );
            EpsPosture::Budget
        }
        Err(MassPropsError::Face {
            source: PropsError::Escalated { cause },
            ..
        }) => {
            assert_eq!(
                cause.predicate,
                Some("props_quad_converged"),
                "{row}: only the convergence predicate may escalate here: {cause:?}"
            );
            EpsPosture::Escalated
        }
        Err(other) => panic!("{row}: not an honest quadrature posture: {other}"),
    }
}

/// The hypothesis of every row here, asserted on the fixture itself:
/// the skin really does carry interior knots in the section direction,
/// and they really are off any grid the composite could cut at.
///
/// **What this reads, and what it does not.** It re-derives the knots
/// the averaging rule would produce from the parameters the loft
/// reports, which is a hand-copy of `skin.rs`'s Eq 9.8 rather than a
/// reading of the vector the built surface actually carries. Nothing
/// public on `Lofted` exposes that vector, and walking the body to its
/// wall surface to fetch it would couple these rows to the topology
/// layer for a hypothesis check. The cost of the copy is stated rather
/// than hidden: if the skin ever changes its knot rule, this helper
/// keeps agreeing with itself while describing a vector the kernel no
/// longer builds. What would still be true in that case is everything
/// the rows actually assert — the measured widths, the postures, and
/// the containment of the analytic oracle — because those come from
/// the body, not from here.
///
/// "Off-grid" is tested against the coarsest cut the rounds make
/// (`QUAD2_INIT_PIECES = 8`) and every doubling of it up to the last
/// round's `1024`: a knot that coincided with a cell edge at any of
/// those resolutions would never have straddled, and the row would be
/// measuring the on-grid case by accident.
fn offgrid_interior_v_knots(row: &str, params: &[f64], v_degree: usize) -> usize {
    // The averaged-parameter knot vector `skin.rs` fits: interior
    // knots are the running means of `v_degree` consecutive params
    // (Book Eq. 9.8).
    let n = params.len();
    assert!(
        n > v_degree + 1,
        "{row}: a fixture with no interior knots tests nothing ({n} stations at \
         degree {v_degree})"
    );
    #[allow(clippy::cast_precision_loss)]
    let interior: Vec<f64> = (1..n - v_degree)
        .map(|j| params[j..j + v_degree].iter().sum::<f64>() / v_degree as f64)
        .collect();
    assert!(
        !interior.is_empty(),
        "{row}: the fixture must have interior v knots"
    );
    let mut off = 0usize;
    for k in &interior {
        let mut on_grid = false;
        let mut pieces = 8usize;
        while pieces <= 1024 {
            #[allow(clippy::cast_precision_loss)]
            let scaled = k * pieces as f64;
            if (scaled - scaled.round()).abs() < 1e-12 {
                on_grid = true;
            }
            pieces *= 2;
        }
        if !on_grid {
            off += 1;
        }
    }
    eprintln!("CERT5-FIXTURE {row}: interior v knots {interior:?}, off-grid {off}");
    off
}

/// The extrusion oracle: the same solid through `extrude`, whose
/// bulged wall is an analytic `Surface::Cylinder` — a closed form with
/// pad exactly 0.
fn oracle_volume() -> f64 {
    let prof = Profile::new(SketchPlane::xy(), arc_section(1.0))
        .validate(Tol::witness())
        .expect("the profile validates");
    let oracle = sweep::extrude::<f64>(
        &prof,
        sweep::Extrusion::Distance(BLADE_HEIGHT),
        Tol::witness(),
    )
    .expect("extrude");
    let want =
        topo::mass_properties(&oracle.body, Tol::witness()).expect("analytic mass properties");
    assert_eq!(
        want.volume_pad, 0.0,
        "ORACLE: the extrude oracle must be a closed form"
    );
    want.volume
}

/// The measured width a budget refusal carries, if that is the
/// posture.
fn budget_width(out: &Result<MassProperties<f64>, MassPropsError>) -> Option<f64> {
    match out {
        Err(MassPropsError::Face {
            source: PropsError::QuadratureBudget { width_len, .. },
            ..
        }) => Some(*width_len),
        _ => None,
    }
}

/// The default ε — the tolerance the rows below make their CERTIFIED
/// claim at. At a tighter ε the schedule is unchanged and only the
/// `1024·ε` target moves, so a row may honestly refuse there; what it
/// may never do is refuse at a width anywhere near the retired floor.
const DEFAULT_EPS: f64 = 1e-9;

/// One row.
///
/// `pre_fix_floor` is the width this exact body refused at before
/// knot-aligned cells, and `retired_below` the ceiling the enclosure
/// must now sit under. The pair is what makes this row ε-honest
/// WITHOUT going slack: at any ε the enclosure must be under
/// `retired_below`, so a regression to the straddle floor fails here
/// on every tolerance row rather than only on the one that happens to
/// straddle the target; at the default ε the row additionally requires
/// the full certified answer, with the closed-form oracle inside it.
fn row(
    name: &str,
    n: usize,
    v_degree: usize,
    want_offgrid: usize,
    pre_fix_floor: Option<f64>,
    retired_below: f64,
) {
    let lofted = blade(n, v_degree);
    let off = offgrid_interior_v_knots(name, &lofted.section_params, v_degree);
    assert_eq!(
        off, want_offgrid,
        "{name}: the fixture must carry exactly {want_offgrid} OFF-GRID interior v \
         knots — a fixture that stopped producing them would pass by testing nothing"
    );
    let body = lofted.body;
    // Structural tiers never touch quadrature, so this is pinned at
    // every ε regardless of the posture below.
    topo::validate_closed(&body).expect("TIER-1/2: tiers 1/2 admit the blade");
    let eps = Tol::witness().get().eps;
    let got = topo::mass_properties(&body, Tol::witness());
    let posture = body_posture(name, &got);
    eprintln!(
        "EPS-ROW {name} @ eps={eps:e}: {posture:?}{}",
        match &got {
            Ok(m) => format!(" volume {} ± {}", m.volume, m.volume_pad),
            Err(e) => format!(" ({e})"),
        }
    );
    if let Some(w) = budget_width(&got) {
        assert!(
            w < retired_below,
            "{name}: the straddle floor is retired — a refusal may only carry a \
             width the SCHEDULE could not close, not the {pre_fix_floor:?} m floor \
             this body used to sit on (got {w:e}, ceiling {retired_below:e})"
        );
    }
    if eps < DEFAULT_EPS {
        // A tighter run: the schedule is fixed (D9), so the achieved
        // width is the same and only the target moved. The floor
        // assertion above is the whole claim there.
        return;
    }
    assert_eq!(
        posture,
        EpsPosture::Certified,
        "{name}: this wall must certify at the default ε (it refused at \
         {pre_fix_floor:?} m against a 1.024e-6 m target before knot-aligned cells)"
    );
    let got = got.expect("certified");
    // ACCURACY: the certified enclosure contains the analytic volume.
    let want = oracle_volume();
    assert!(
        (got.volume - want).abs() <= got.volume_pad,
        "ACCURACY {name}: the enclosure must CONTAIN the analytic volume: got {} ± {}, \
         oracle {want}",
        got.volume,
        got.volume_pad
    );
    // PAD CEILING, pinned separately so a loosening enclosure cannot
    // absorb the accuracy row: the schedule is fixed, so the pad is
    // what the schedule achieves, and it is never widened past the
    // run's own target.
    let ceiling = 2.0 * QUAD_TARGET_LEN_FACTOR * eps;
    assert!(
        got.volume_pad < ceiling,
        "PAD CEILING {name}: volume pad {} vs {ceiling}",
        got.volume_pad
    );
}

/// **The regression row** (issue 453): SIX stations, a quadratic skin,
/// and a rational wall whose interior v knots are off the composite's
/// grid.
///
/// Before knot-aligned cells this refused
/// `QuadratureBudget { width_len: 2.143e-4, target_len: 1.024e-6 }` —
/// 209× over, and provably unreachable: the straddling cells' share of
/// the enclosure halved per round while the cell count quadrupled, so
/// the seven-round schedule could not close it at any budget.
#[test]
fn six_stations_with_offgrid_knots_certify() {
    row("blade-6 (deg 2)", 6, 2, 2, Some(2.143e-4), 3.0e-7);
}

/// The same solid at EIGHT stations on a linear skin: six interior v
/// knots, all but none of them off-grid, and a floor that was three
/// times the six-station one (6.425e-4) because it is proportional to
/// the off-grid count. Certifying here says the fix scales with knot
/// count rather than tolerating a fixed few.
///
/// **What this row does NOT exercise, stated so it is not read for
/// more than it is.** Every interior knot of a degree-1 skin has
/// multiplicity equal to the degree, so the REPRESENTATION is only C0
/// there — but the locus is not. The stations here are identical
/// sections evenly stacked, so `z(v)` is globally linear, the control
/// points are collinear, and `S_v` is continuous across every one of
/// those knots: the surface is a plain extrusion said in a knotty way.
/// What the row pins is therefore the enclosure the knot STRUCTURE
/// used to cost, not the enclosure a genuine jump costs.
///
/// A cell rule that ignored the knots entirely would still contain the
/// truth on this body. The row that is lethal to that mutation is
/// `geom-brep`'s `cert5_arm_and_cells::a_genuine_c0_jump_stays_
/// contained`, which needs a real discontinuity and the composite arm
/// to expose it; these body rows are the door-level evidence that the
/// floor is gone, and that one is the rule-level evidence that the
/// cells are the reason.
#[test]
fn a_linear_skin_with_six_offgrid_knots_certifies() {
    row("blade-8 (deg 1)", 8, 1, 6, Some(6.425e-4), 3.0e-7);
}

/// **The control row**, and the reason this file pins an exact
/// off-grid COUNT rather than a knot count: at five stations the
/// averaged parameters are quarters and every interior knot lands ON
/// the composite grid, where the strict `>` / `<` of the straddle test
/// meant no cell ever straddled. This row certified before the fix and
/// certifies after, and it is what makes the rows above evidence about
/// off-grid knots specifically rather than about knots.
#[test]
fn dyadic_knots_were_free_and_stay_free() {
    row("blade-5 (deg 2, dyadic)", 5, 2, 0, None, 3.0e-7);
}

/// **The lily flip, executed.** `lily_leaf_b`'s own geometry from
/// `demo/lily-crescent-restoration` — a 1.25 m spine turning through
/// 0.40 rad, a crescent section 0.170 m across with a 0.015 m ridge and
/// a 0.007 m keel, swept at nine stations and skinned at cubic degree —
/// with the straight kite margins replaced by the arcs the stop was
/// drawn with.
///
/// This is the body issue 453 names as its flip condition, and it is a
/// GATE rather than a note because the alternative is a digit in a PR
/// description that nothing re-takes. It is built through the public
/// sweep door, the way the demo builds it, so what it measures is what
/// a demo would get.
///
/// It is NOT the tour's leaf: the tour still draws straight margins,
/// and restoring them there is demo work with a render re-baseline
/// attached, carried on that branch. What this row settles is the only
/// question the kernel owned — whether an arc-margined blade of these
/// proportions can be certified at all.
#[test]
fn the_lily_crescent_blade_certifies() {
    use geom::NurbsCurve3;
    use geom_core::{Mat3, Point3};

    let (len, curl) = (1.25f64, -0.40f64);
    let r = len / curl;
    let pts: Vec<Point3<f64>> = (0..9)
        .map(|k| {
            #[allow(clippy::cast_precision_loss)]
            let a = curl * (k as f64) / 8.0;
            Point3::new(r * a.sin(), r * (1.0 - a.cos()), 0.0)
        })
        .collect();
    let path = NurbsCurve3::<f64>::interpolate(&pts, 3).expect("the leaf spine interpolates");
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
    let w = n.cross(u);
    let p = path.eval(lo);
    let place = Affine3::from_parts(Mat3::from_cols(u, w, n), Vec3::new(p.x, p.y, p.z));

    // The crescent: chord 0.170, ridge sagitta 0.015, keel 0.007. For a
    // circular arc, bulge = tan(θ/4) = 2·sagitta/chord.
    let vtx = |x: f64, y: f64, b: f64| ProfileVertex::new(Point2::new(x, y), b);
    let (hw, ridge, keel) = (0.085f64, 0.015f64, 0.007f64);
    let section = vec![ProfileLoop::new(vec![
        vtx(-hw, 0.0, 2.0 * ridge / (2.0 * hw)),
        vtx(hw, 0.0, 2.0 * keel / (2.0 * hw)),
    ])];

    let lofted = sweep::sweep_body::<f64>(&section, place, &path, 9, 3, Tol::witness())
        .expect("the arc-margined blade sweeps");
    let off = offgrid_interior_v_knots("lily-crescent", &lofted.section_params, 3);
    assert!(
        off >= 2,
        "lily-crescent: the blade's own spine must put interior v knots off the \
         grid — that is the condition the flip was about (got {off})"
    );
    let got = topo::mass_properties(&lofted.body, Tol::witness());
    let posture = body_posture("lily-crescent", &got);
    eprintln!(
        "EPS-ROW lily-crescent @ eps={:e}: {posture:?}{}",
        Tol::witness().get().eps,
        match &got {
            Ok(m) => format!(" volume {} ± {}", m.volume, m.volume_pad),
            Err(e) => format!(" ({e})"),
        }
    );
    if let Some(w) = budget_width(&got) {
        assert!(
            w < 3.0e-7,
            "lily-crescent: a refusal here may only carry a width the SCHEDULE \
             could not close, not the 4.09e-4 m floor this blade used to sit on \
             (got {w:e})"
        );
    }
    if Tol::witness().get().eps < DEFAULT_EPS {
        return;
    }
    assert_eq!(
        posture,
        EpsPosture::Certified,
        "lily-crescent: the blade issue 453 names as its flip condition must \
         certify (it refused at width_len 4.09e-4 against a 1.024e-6 target)"
    );
    let got = got.expect("certified");
    assert!(
        got.volume > 0.0 && got.volume_pad > 0.0,
        "lily-crescent: {} ± {}",
        got.volume,
        got.volume_pad
    );
}
