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
//! tighter one, and every row pins both honest outcomes rather
//! than widening a target. What is asserted UNCONDITIONALLY is the
//! structure — the knots really are interior and really are off-grid —
//! because that is the hypothesis of the row, and a fixture that
//! quietly stopped producing off-grid knots would otherwise pass by
//! testing nothing.
#![allow(clippy::unwrap_used, clippy::expect_used, clippy::panic)]

use crate::common::arc_section;
use geom_core::Point3;
use geom_core::Tol;
use profile::{Profile, SketchPlane};
use sweep::ExtrudeSide;
use sweep::{Section, loft_body};
use topo::{MassMeasurement, Stop};

/// The resolution this row reads at, as a multiple of ε: the pinned
/// resolution's (`topo::test_support::pinned_resolution`).
const QUAD_TARGET_LEN_FACTOR: f64 = 1024.0;

// The arc section is the crate's one copy (`tests/common/`), which
// `m8_3_rational_volume` lofts too — so nothing but the station count
// differs between these rows and that one.

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
    let places: Vec<SketchPlane<f64>> = (0..n)
        .map(|k| {
            SketchPlane::xy_at(Point3::new(
                0.0,
                0.0,
                BLADE_HEIGHT * k as f64 / (n - 1) as f64,
            ))
        })
        .collect();
    loft_body::<f64>(&sections, &places, v_degree, Tol::witness()).expect("the blade lofts")
}

/// The two honest outcomes, as `m8_3_rational_volume.rs` names them.
#[derive(Debug, PartialEq, Eq)]
enum EpsPosture {
    Certified,
    Budget,
}

fn body_posture(row: &str, out: &(MassMeasurement<f64>, Option<f64>)) -> EpsPosture {
    let target = QUAD_TARGET_LEN_FACTOR * Tol::witness().get().eps;
    match (out.0.stop, out.1) {
        (Stop::Met, None) => EpsPosture::Certified,
        (Stop::Floor | Stop::Wall, Some(width_len)) => {
            assert!(
                width_len.is_finite() && width_len > target,
                "{row}: a face short of the resolution has a width that really missed: \
                 {width_len:e} vs {target:e}"
            );
            EpsPosture::Budget
        }
        other => panic!("{row}: not an honest quadrature posture: {other:?}"),
    }
}

/// The hypothesis of every row here, asserted on the fixture itself:
/// the body's walls really do carry interior knots in the section
/// direction, and they really are off any grid the composite could cut
/// at.
///
/// It reads the knot vectors the built walls carry, not the loft's
/// `section_params`: those are the skin's parameters, and at v-degree 1
/// through three or more sections the body's walls are one slab each,
/// with no interior v knot at all (`sweep::Lofted::section_params`). A
/// fixture whose walls stopped carrying the knots fails here rather than
/// passing by testing nothing. Every wall must carry the same interior
/// knots: the sections are one skin.
///
/// "Off-grid" is tested against the coarsest cut the rounds make
/// (`AXIS_INIT_PIECES = 8`) and every doubling of it up to the last
/// round's `1024`: a knot that coincided with a cell edge at any of
/// those resolutions would never have straddled, and the row would be
/// measuring the on-grid case by accident.
fn offgrid_interior_v_knots(row: &str, lofted: &sweep::Lofted<f64>) -> usize {
    let body = &lofted.body;
    let interior_of = |face: topo::FaceKey| -> Vec<f64> {
        let face = body.get_face(face).expect("a wall face");
        let Some(geom::Surface::Nurbs(net)) = body.get_surface(face.surface) else {
            panic!("{row}: a loft wall is a spline")
        };
        let knots = net.knot_values_v();
        let (lo, hi) = knots.domain();
        let mut interior: Vec<f64> = knots
            .values()
            .iter()
            .copied()
            .filter(|k| *k > lo && *k < hi)
            .collect();
        interior.dedup();
        interior
    };
    let walls: Vec<topo::FaceKey> = lofted
        .side_faces
        .iter()
        .flatten()
        .flatten()
        .copied()
        .collect();
    let interior = interior_of(walls[0]);
    for &w in &walls[1..] {
        assert_eq!(
            interior_of(w),
            interior,
            "{row}: every wall carries the skin's interior v knots"
        );
    }
    assert!(
        !interior.is_empty(),
        "{row}: a fixture whose walls carry no interior v knots tests nothing"
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
        sweep::Extrusion::Distance {
            depth: BLADE_HEIGHT,
            side: ExtrudeSide::Along,
        },
        Tol::witness(),
    )
    .expect("extrude");
    let want = topo::mass_properties(
        &topo::test_support::finished(
            "the measured body",
            topo::Body::clone(&oracle.body),
            Tol::witness(),
        ),
        Tol::witness(),
        topo::test_support::pinned_resolution(Tol::witness()),
    )
    .expect("analytic mass properties");
    assert_eq!(
        want.enclosure.volume_pad, 0.0,
        "ORACLE: the extrude oracle must be a closed form"
    );
    want.enclosure.volume
}

/// The default ε — the tolerance the rows below make their CERTIFIED
/// claim at. At a tighter ε the schedule is unchanged and only the
/// `1024·ε` target moves, so a row may honestly refuse there; what it
/// may never do is refuse at a width anywhere near the retired floor.
const DEFAULT_EPS: f64 = 1e-9;

/// One row.
///
/// `pre_fix_floor` is the width this exact body refused at before
/// knot-aligned cells, where that was measured (`None` for a body first
/// built after them), and `retired_below` the ceiling the enclosure
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
    let before = pre_fix_floor.map_or_else(
        || "the straddle floor".to_owned(),
        |w| format!("the {w:e} m floor this body used to sit on"),
    );
    let lofted = blade(n, v_degree);
    let off = offgrid_interior_v_knots(name, &lofted);
    assert_eq!(
        off, want_offgrid,
        "{name}: the fixture must carry exactly {want_offgrid} OFF-GRID interior v \
         knots — a fixture that stopped producing them would pass by testing nothing"
    );
    let body = topo::test_support::finished("body", lofted.body, Tol::witness());
    // Structural tiers never touch quadrature, so this is pinned at
    // every ε regardless of the posture below.
    topo::validate_closed(&body).expect("TIER-1/2: tiers 1/2 admit the blade");
    let eps = Tol::witness().get().eps;
    let got = topo::test_support::measure_with_unmet_width(
        &body,
        Tol::witness(),
        topo::test_support::pinned_resolution(Tol::witness()),
    )
    .unwrap_or_else(|e| panic!("{name}: the blade measures: {e}"));
    let posture = body_posture(name, &got);
    eprintln!(
        "EPS-ROW {name} @ eps={eps:e}: {posture:?} volume {} ± {}",
        got.0.enclosure.volume, got.0.enclosure.volume_pad
    );
    if let Some(w) = got.1 {
        assert!(
            w < retired_below,
            "{name}: the straddle floor is retired — a refusal may only carry a \
             width the SCHEDULE could not close, not {before} (got {w:e}, ceiling \
             {retired_below:e})"
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
        "{name}: this wall must certify at the default ε, not refuse at {before} \
         against a 1.024e-6 m target"
    );
    let got = got.0;
    // ACCURACY: the certified enclosure contains the analytic volume.
    let want = oracle_volume();
    assert!(
        (got.enclosure.volume - want).abs() <= got.enclosure.volume_pad,
        "ACCURACY {name}: the enclosure must CONTAIN the analytic volume: got {} ± {}, \
         oracle {want}",
        got.enclosure.volume,
        got.enclosure.volume_pad
    );
    // PAD CEILING, pinned separately so a loosening enclosure cannot
    // absorb the accuracy row: the schedule is fixed, so the pad is
    // what the schedule achieves. Each face is read about a vertex of
    // the shell's first quadrature face (`topo::props`'
    // `shell_reference`), so on these walls, centred on the world
    // origin, a quadrature face's lever is their diameter where the
    // origin's was their radius (`m8_3_rational_volume`'s ceiling).
    // blade-10 measured 1.90e-6 at 1e-9, 1.86 times the target: the
    // ceiling is the measure with headroom, not a bound the rule
    // guarantees.
    let ceiling = 4.0 * QUAD_TARGET_LEN_FACTOR * eps;
    assert!(
        got.enclosure.volume_pad < ceiling,
        "PAD CEILING {name}: volume pad {} vs {ceiling}",
        got.enclosure.volume_pad
    );
    // The measure the ceiling's comment cites, held where it was taken.
    if name == "blade-10 (deg 3)" && eps == DEFAULT_EPS {
        assert!(
            (got.enclosure.volume_pad / BLADE_10_PAD_AT_DEFAULT_EPS - 1.0).abs() < 0.01,
            "MEASURED {name}: volume pad {} moved from the {BLADE_10_PAD_AT_DEFAULT_EPS} the \
             ceiling's comment cites",
            got.enclosure.volume_pad
        );
    }
}

/// blade-10's volume pad at the default ε, the measure the ceiling's
/// comment cites (blade-8 at v-degree 1 read the same pad, before its
/// walls became one slab each).
const BLADE_10_PAD_AT_DEFAULT_EPS: f64 = 1.899_6e-6;

/// **The regression row** (issue 453): SIX stations, a quadratic skin,
/// and a rational wall whose interior v knots are off the composite's
/// grid.
///
/// Before knot-aligned cells this stopped at a width of 2.143e-4 m
/// against a 1.024e-6 m target — 209× over, and provably unreachable: the straddling cells' share of
/// the enclosure halved per round while the cell count quadrupled, so
/// the seven-round schedule could not close it at any budget.
#[test]
fn six_stations_with_offgrid_knots_certify() {
    row("blade-6 (deg 2)", 6, 2, 2, Some(2.143e-4), 3.0e-7);
}

/// The same solid at TEN stations on a cubic skin: six interior v
/// knots, `2/9 … 7/9`, all of them off-grid, twice the six-station
/// row's count and three times its off-grid count. Certifying here says
/// the fix scales with knot count rather than tolerating a fixed few.
///
/// This row was blade-8 at v-degree 1, whose six knots were `k/7` and
/// whose pre-fix floor was 6.425e-4 m. A degree-1 loft through three or
/// more sections is now one wall per slab, each a single span in `v`
/// (`work/tess/loft-of-three-or-more-sections-at-degree-one-is-unmeshable.md`),
/// so its walls carry no interior knot. Degree 3 is the lowest degree
/// at which ten even stations put six knots in one wall, all off-grid
/// (at degree 2, nine stations' knots land on sixteenths). No pre-fix
/// floor was measured on this body.
///
/// **What this row does NOT exercise, stated so it is not read for
/// more than it is.** The stations here are identical sections evenly
/// stacked, so `z(v)` is globally linear, the control points are
/// collinear, and the surface is a plain extrusion said in a knotty
/// way. What the row pins is therefore the enclosure the knot
/// STRUCTURE used to cost, not the enclosure a genuine jump costs.
///
/// A cell rule that ignored the knots entirely would still contain the
/// truth on this body. The row that is lethal to that mutation is
/// `geom-brep`'s `cert5_arm_and_cells::a_genuine_c0_jump_stays_
/// contained`, which needs a real discontinuity and the composite arm
/// to expose it; these body rows are the door-level evidence that the
/// floor is gone, and that one is the rule-level evidence that the
/// cells are the reason.
#[test]
fn a_cubic_skin_with_six_offgrid_knots_certifies() {
    row("blade-10 (deg 3)", 10, 3, 6, None, 3.0e-7);
}
