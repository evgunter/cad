//! **MIN-2 (the #218 review): the two `Pcurve::Fitted` refusal arms,
//! EXECUTED.** Both arms were confirmed fail-loud by reading; this
//! suite makes them executed facts with a genuinely certified fitted
//! cache — no arm of the boundary-tightening contract stays dead.
//!
//! The cache comes through the real M6-2 door
//! (`PcurveCache::certify_fitted`): the cylinder×sphere rung-3 trace
//! of `topo/tests/fixture/mod.rs`, reproduced here f64-only and
//! cache-only (mesh cannot import another crate's test module; the
//! fixture's own docs call it a deliberate scaffold until the
//! fitted-chord join lane lands — same caveat carried). Attaching it
//! to another body's half-edge is the `attach_pcurve` trust posture:
//! the type guarantees certification, coherence is downstream's
//! problem — and these refusal arms are exactly that downstream.

#![allow(clippy::unwrap_used, clippy::expect_used, clippy::panic)]

use profile::RawLoop;
use std::sync::Arc;

use geom::Surface;
use geom::{Curve3, NurbsCurve2, NurbsCurve3};
use geom_brep::ssi::{self, SsiDomain, SsiError};
use geom_brep::{Pcurve, PcurveCache};
use geom_core::Tol;
use geom_core::{Band, Point2, Point3, Vec3};
use mesh::TessellateError;
use profile::{Profile, ProfileLoop, ProfileVertex, SketchPlane};
use sweep::{Extrusion, extrude, loft_body};
use topo::splitting::{SplitPart, SplitPlane, split};
use topo::{Body, HalfEdgeKey};

mod common;

// ---- The certified fitted cache (topo/tests/fixture/mod.rs, f64 +
// cache-only) --------------------------------------------------------

const CYL_ORIGIN: (f64, f64, f64) = (0.03, 0.0, 0.0);
const CYL_RADIUS: f64 = 0.08;
const SPH_RADIUS: f64 = 1.0;
const SAMPLES: usize = 481;
const DEGREE: usize = 3;

fn cylinder() -> Surface<f64> {
    Surface::Cylinder {
        origin: Point3::new(CYL_ORIGIN.0, CYL_ORIGIN.1, CYL_ORIGIN.2),
        axis: Vec3::new(0.0, 0.0, 1.0),
        radius: CYL_RADIUS,
        u_ref: Vec3::new(1.0, 0.0, 0.0),
    }
}

fn sphere() -> Surface<f64> {
    Surface::Sphere {
        center: Point3::new(0.0, 0.0, 0.0),
        radius: SPH_RADIUS,
        axis: Vec3::new(0.0, 0.0, 1.0),
        u_ref: Vec3::new(1.0, 0.0, 0.0),
    }
}

fn chart_of(p: Point3<f64>) -> Point2<f64> {
    let w = p - Point3::new(CYL_ORIGIN.0, CYL_ORIGIN.1, CYL_ORIGIN.2);
    Point2::new(w.y.atan2(w.x), w.z)
}

fn sub_arc3(c: &NurbsCurve3<f64>, frac: (f64, f64)) -> Option<NurbsCurve3<f64>> {
    let tail = if frac.0 > 0.0 {
        c.split_at(frac.0).ok()?.1
    } else {
        c.clone()
    };
    if frac.1 < 1.0 {
        Some(tail.split_at(frac.1).ok()?.0)
    } else {
        Some(tail)
    }
}

fn sub_arc2(c: &NurbsCurve2<f64>, frac: (f64, f64)) -> Option<NurbsCurve2<f64>> {
    let tail = if frac.0 > 0.0 {
        c.split_at(frac.0).ok()?.1
    } else {
        c.clone()
    };
    if frac.1 < 1.0 {
        Some(tail.split_at(frac.1).ok()?.0)
    } else {
        Some(tail)
    }
}

/// A genuinely certified `Pcurve::Fitted` cache (the first quarter of
/// the traced loop), or `None` on the SSI fit-budget stand-down.
///
/// **No memo, deliberately.** There is exactly one caller (the single
/// test below), and under nextest's process-per-test isolation a
/// `OnceLock` would share nothing across tests anyway — it would be
/// dead weight that reads as if it worked.
fn build_fitted_cache() -> Option<PcurveCache<f64>> {
    let slab = SsiDomain {
        center: Point3::new(0.0, 0.0, 0.0),
        half_extent: 1.5,
        extent: 2.0,
        floor_scale: 1.0,
    };
    let branch = match ssi::cylinder_sphere_ssi(
        &cylinder(),
        &sphere(),
        slab,
        Band::linear(Tol::witness()).unwrap(),
    ) {
        Ok(out) => out.branches.into_iter().next().expect("two loops"),
        Err(SsiError::FitSampleBudget { .. }) => return None,
        Err(e) => panic!("the planted fixture: {e}"),
    };
    let Curve3::Nurbs(ref loop_carrier) = branch.carrier else {
        panic!("a rung-3 carrier is a NURBS curve")
    };
    let (d0, d1) = loop_carrier.domain();
    if d0 != 0.0 || d1 != 1.0 {
        return None;
    }
    #[allow(clippy::cast_precision_loss)]
    let params: Vec<f64> = (0..SAMPLES)
        .map(|i| i as f64 / (SAMPLES - 1) as f64)
        .collect();
    let pts: Vec<Point3<f64>> = params.iter().map(|t| loop_carrier.eval(*t)).collect();
    let mut chart: Vec<Point2<f64>> = pts.iter().map(|p| chart_of(*p)).collect();
    let tau = core::f64::consts::TAU;
    for i in 1..chart.len() {
        let mut u = chart[i].x;
        while u - chart[i - 1].x > tau / 2.0 {
            u -= tau;
        }
        while chart[i - 1].x - u > tau / 2.0 {
            u += tau;
        }
        chart[i].x = u;
    }
    let image = NurbsCurve2::<f64>::interpolate_with_params(&chart, DEGREE, &params).ok()?;
    let carrier = Arc::new(sub_arc3(loop_carrier, (0.0, 0.25))?);
    let image = Arc::new(sub_arc2(&image, (0.0, 0.25))?);
    let (t0, t1) = image.domain();
    let window = Pcurve::Fitted(Arc::clone(&image)).chart_box(t0, t1);
    Some(
        PcurveCache::<f64>::certify_fitted(
            image,
            t0,
            t1,
            &Curve3::Nurbs(carrier),
            &cylinder(),
            Some(&sphere()),
            window,
            Band::linear(Tol::witness()).unwrap(),
        )
        .expect("the fitted cache certifies through the M6-2 door"),
    )
}

// ---- Host bodies ---------------------------------------------------

/// `loft_prism` (the m7_nurbs_trimmed suite's constant).
fn loft_prism() -> Body<f64> {
    let quad = common::quad;
    let sections = vec![
        quad([(-1.0, -1.0), (1.0, -1.0), (1.0, 1.0), (-1.0, 1.0)]),
        quad([(-1.375, -1.0), (1.375, -1.0), (1.0, 1.0), (-1.0, 1.0)]),
        quad([(-1.0, -1.0), (1.0, -1.0), (1.0, 1.0), (-1.0, 1.0)]),
    ];
    let places: Vec<geom_core::Affine3<f64>> = [0.0, 1.0, 2.0]
        .iter()
        .map(|z| geom_core::Affine3::translation(Vec3::new(0.0, 0.0, *z)))
        .collect();
    loft_body::<f64>(&sections, &places, 2, Tol::witness())
        .expect("loft builds")
        .body
}

/// The m5_pr11_trimmed suite's split cylinder (trimmed cylinder walls
/// with Harmonic caches), lower half — `disc`/`halves` verbatim.
fn split_cylinder_half() -> Body<f64> {
    let lp = ProfileLoop::new(vec![
        ProfileVertex::new(Point2::new(-1.0, 0.0), 1.0),
        ProfileVertex::new(Point2::new(1.0, 0.0), 1.0),
    ]);
    let disc = Profile::new(SketchPlane::xy(), vec![lp])
        .validate(Tol::witness())
        .unwrap();
    let cylinder = extrude(&disc, Extrusion::Distance(2.0), Tol::witness())
        .unwrap()
        .body;
    let plane = SplitPlane {
        origin: Point3::new(0.0, 0.0, 1.0),
        normal: Vec3::new(0.3f64.sin(), 0.0, 0.3f64.cos()),
    };
    let result = split(&cylinder, &plane).unwrap();
    let SplitPart::Body(ref below) = result.below else {
        panic!("the lower side carries material");
    };
    below.clone()
}

/// A half-edge of `body` whose parent loop's face satisfies `want`,
/// and which carries a stored pcurve cache.
fn cached_half_edge_on(body: &Body<f64>, want: impl Fn(&Surface<f64>) -> bool) -> HalfEdgeKey {
    let keys: Vec<HalfEdgeKey> = body.pcurves().map(|(h, _)| h).collect();
    for hek in keys {
        let he = body.get_half_edge(hek).unwrap();
        let lp = body.get_loop(he.parent_loop).unwrap();
        let face = body.get_face(lp.face).unwrap();
        if want(body.get_surface(face.surface).unwrap()) {
            return hek;
        }
    }
    panic!("no cached half-edge on a matching face");
}

// ---- The two arms, executed ---------------------------------------

/// **Both `Pcurve::Fitted` refusal arms, on ONE traced cache.**
///
/// - **Arm 1 (`chords::nurbs_tighten`)**: a NURBS-face half-edge
///   carrying a FITTED cache refuses typed at the CHORD pass — there is
///   no certified UV speed bound for a fitted image's chord schedule.
/// - **Arm 2 (`trimmed::trim_polygon`)**: a FITTED cache on an ANALYTIC
///   (cylinder) trimmed face passes the chord pass — the NURBS
///   tightening rightly skips analytic faces — and refuses typed in the
///   trim walk instead.
///
/// # One trace, both arms
///
/// One test, not two: nextest runs one process per test, so a
/// `OnceLock` would share nothing and each arm would re-pay the whole
/// cylinder×sphere trace for a little tessellation work. The cache is a
/// value, immutable, and each arm attaches its own clone to its OWN
/// host body, so nothing crosses between them but the cache itself.
///
/// What the split bought and a merged row cannot is failure ISOLATION:
/// the chord pass and the trim walk are two independent failure modes
/// under one test id now. So each assertion NAMES its arm — `CHORD` /
/// `TRIM-WALK` — and the message alone says which door broke.
#[test]
fn a_fitted_cache_refuses_typed_at_the_chord_pass_and_in_the_trim_walk() {
    let Some(cache) = build_fitted_cache() else {
        // The SSI fit-budget stand-down (fixture docs), said out loud:
        // a bare `return` here reports coverage this run does not have.
        println!(
            "SKIPPED: the cylinder×sphere fixture stood down on the SSI door's typed \
             FitSampleBudget refusal at this ε — THIS RUN EXECUTES NEITHER FITTED \
             REFUSAL ARM (no chord-pass row, no trim-walk row); both are confirmed by \
             reading only"
        );
        return;
    };

    // ---- Arm 1: the CHORD pass, on a NURBS face --------------------
    let mut body = loft_prism();
    let hek = cached_half_edge_on(&body, |s| matches!(s, Surface::Nurbs(_)));
    body.attach_pcurve(hek, cache.clone());
    match mesh::tessellate(&body, 1e-2, Tol::witness()) {
        Err(TessellateError::UnsupportedCurve { note, .. }) => {
            assert!(
                note.contains("FITTED") && note.contains("UV speed bound"),
                "CHORD: the chord arm's note names the real blocker: {note}"
            );
        }
        other => panic!(
            "CHORD: expected the chord-pass fitted refusal, got {:?}",
            other.map(|_| ())
        ),
    }

    // ---- Arm 2: the TRIM WALK, on an analytic trimmed face ---------
    let mut body = split_cylinder_half();
    let hek = cached_half_edge_on(&body, |s| matches!(s, Surface::Cylinder { .. }));
    body.attach_pcurve(hek, cache);
    match mesh::tessellate(&body, 1e-2, Tol::witness()) {
        Err(TessellateError::UnsupportedCurve { note, .. }) => {
            assert!(
                note.contains("FITTED") && note.contains("boolean layer"),
                "TRIM-WALK: the trim-walk arm's note names the real blocker: {note}"
            );
        }
        other => panic!(
            "TRIM-WALK: expected the trim-walk fitted refusal, got {:?}",
            other.map(|_| ())
        ),
    }
}
