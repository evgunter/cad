//! The SLIVER side of the re-mint two-class contract (review EXTRA,
//! banked from #83/#84): `m4_remint_headroom.rs` pins that a 0.9ε
//! witness offset ACCEPTS; this pins that offsets past ε refuse
//! TYPED at certification — in-band offsets escalate, definite ones
//! refuse `ResidualExceeded` — never a silent acceptance, never a
//! panic. All probe magnitudes derive from the ambient
//! `Tol::witness().get()` (the ε matrix reruns this at every row).

#![allow(clippy::unwrap_used, clippy::expect_used, clippy::panic)]

use geom::Curve3;
use geom::Surface;
use geom_brep::{
    CERT_SAMPLES, CertifyError, EdgeCurve, EdgeCurveSpec, EdgeDescriptionSpec,
};
use geom_core::Tol;
use geom_core::{Band, Point3, Vec3};

fn table(
    surfs: Vec<Surface<f64>>,
) -> (
    Vec<geom_brep::SurfaceKey>,
    impl Fn(geom_brep::SurfaceKey) -> Option<Surface<f64>>,
) {
    let mut map: slotmap::SlotMap<geom_brep::SurfaceKey, Surface<f64>> =
        slotmap::SlotMap::with_key();
    let keys: Vec<geom_brep::SurfaceKey> = surfs.into_iter().map(|s| map.insert(s)).collect();
    (keys, move |k| map.get(k).cloned())
}

/// Certifies the headroom fixture's line-intersection edge with a
/// witness offset of `offset` along the line from the pinned mid
/// point; returns the certification outcome.
fn certify_with_offset(offset: f64) -> Result<EdgeCurve<f64>, CertifyError> {
    let band = Band::linear(Tol::witness()).unwrap();
    let (keys, resolve) = table(vec![
        Surface::Plane {
            origin: Point3::new(0.0, 0.0, 0.0),
            normal: Vec3::new(0.0, 0.0, 1.0),
            u_ref: Vec3::new(1.0, 0.0, 0.0),
        },
        Surface::Plane {
            origin: Point3::new(0.0, 0.0, 0.0),
            normal: Vec3::new(0.0, 1.0, 0.0),
            u_ref: Vec3::new(1.0, 0.0, 0.0),
        },
    ]);
    let carrier = Curve3::Line {
        origin: Point3::new(0.0, 0.0, 0.0),
        dir: Vec3::new(1.0, 0.0, 0.0),
    };
    let (t0, t1) = (0.0, 2.0);
    let mid_i = (CERT_SAMPLES - 1) / 2;
    let mid_t = t0 + (t1 - t0) * (f64::from(mid_i) / f64::from(CERT_SAMPLES - 1));
    let true_mid = carrier.eval(mid_t);
    // The witness stays ON both planes exactly (offset along the
    // line): only the mid-parameter pin carries the offset — the
    // isolated two-class knob.
    let witness = Point3::new(true_mid.x + offset, 0.0, 0.0);
    let spec = EdgeCurveSpec {
        description: EdgeDescriptionSpec::Intersection {
            s1: keys[0],
            s2: keys[1],
            witness,
        },
        carrier,
        param_start: t0,
        param_end: t1,
    };
    EdgeCurve::certify(
        spec,
        Point3::new(0.0, 0.0, 0.0),
        Point3::new(2.0, 0.0, 0.0),
        &resolve,
        band,
    )
}

#[test]
fn witness_offset_definitely_past_the_band_refuses_residual_exceeded() {
    let tol = Tol::witness().get();
    // Definitely past the escalation band: (k + 1)·ε.
    let outcome = certify_with_offset(tol.eps * (tol.k + 1.0));
    match outcome {
        Err(CertifyError::ResidualExceeded { .. }) => {}
        other => panic!("expected ResidualExceeded, got {other:?}"),
    }
}

#[test]
fn witness_offset_in_the_sliver_band_refuses_typed_never_accepts() {
    let tol = Tol::witness().get();
    // Just past ε, inside the escalation band: mid of (ε, kε) if the
    // band has width, else nudged past ε.
    let offset = if tol.k > 1.0 {
        tol.eps * (1.0 + tol.k) * 0.5
    } else {
        tol.eps * 1.5
    };
    match certify_with_offset(offset) {
        Err(CertifyError::Escalated { .. } | CertifyError::ResidualExceeded { .. }) => {}
        Ok(_) => panic!("a witness residual past epsilon must never certify"),
        Err(other) => panic!("unexpected refusal shape: {other:?}"),
    }
}

#[test]
fn the_accepting_side_still_accepts() {
    // The companion boundary from the headroom test, re-stated here
    // so the two-class contract lives in one file's assertions too.
    let tol = Tol::witness().get();
    certify_with_offset(tol.eps * 0.9).expect("0.9eps stays marginal but VALID");
}
