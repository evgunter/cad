//! The headroom argument behind re-mint-at-transform (PR #83 ruling),
//! demonstrated deterministically at the certification layer: a
//! marginal-but-valid stored witness KEEPS its consumed slack under
//! the old map-the-witness path, while the re-mint formula's witness
//! has bit-zero mid residual — construction-fresh headroom.

#![allow(clippy::unwrap_used, clippy::expect_used, clippy::panic)]

use geom::Curve3;
use geom::Surface;
use geom_brep::{CERT_SAMPLES, EdgeCurve, EdgeCurveSpec, EdgeDescription, EdgeDescriptionSpec};
use geom_core::Tol;
use geom_core::{Affine3, Band, Mat3, Point3, Vec3};

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

/// Line x-axis = intersection of z=0 and y=0 planes; witness offset
/// ALONG the line by 0.9ε from the mid point — on both surfaces
/// exactly (surface residuals 0), only the mid-parameter pin carries
/// the offset. Marginal-but-valid: certify passes with ~0.9ε mid
/// residual. The re-mint formula's witness has bit-zero mid residual.
#[test]
fn marginal_witness_slack_vs_remint_freshness() {
    let eps = Tol::witness().get().eps;
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
    let offset = eps * 0.9;
    let marginal = Point3::new(true_mid.x + offset, 0.0, 0.0);
    let spec = EdgeCurveSpec {
        description: EdgeDescriptionSpec::Intersection {
            s1: keys[0],
            s2: keys[1],
            witness: marginal,
        },
        carrier,
        param_start: t0,
        param_end: t1,
    };
    // Marginal-but-valid: certification ACCEPTS (0.9ε < ε).
    let ec = EdgeCurve::certify(
        spec,
        Point3::new(0.0, 0.0, 0.0),
        Point3::new(2.0, 0.0, 0.0),
        &resolve,
        band,
    )
    .expect("0.9eps witness offset is marginal but VALID");
    // Old path: an exact-in-principle isometry (identity here — the
    // cleanest possible map) keeps the stored witness's consumed
    // slack: mid residual stays ~0.9ε.
    let mapped_witness =
        Affine3::from_parts(Mat3::identity(), Vec3::zero()).transform_point(marginal);
    let old_residual = ec
        .carrier()
        .eval(ec.sample_param(mid_i))
        .distance(mapped_witness);
    assert!(
        old_residual > offset * 0.99,
        "old path keeps consumed slack: {old_residual:e}"
    );
    // Re-mint: a spec whose witness IS the pinned formula certifies
    // with full headroom (the non-vacuous statement of freshness — the
    // bit-level pin on the real transform path lives in
    // crates/topo/tests/m4_remint_transform.rs).
    let fresh_spec = EdgeCurveSpec {
        description: EdgeDescriptionSpec::Intersection {
            s1: keys[0],
            s2: keys[1],
            witness: ec.carrier().eval(ec.sample_param(mid_i)),
        },
        carrier: ec.carrier().clone(),
        param_start: t0,
        param_end: t1,
    };
    EdgeCurve::certify(
        fresh_spec,
        Point3::new(0.0, 0.0, 0.0),
        Point3::new(2.0, 0.0, 0.0),
        &resolve,
        band,
    )
    .expect("the re-mint formula's witness certifies with fresh headroom");
}
