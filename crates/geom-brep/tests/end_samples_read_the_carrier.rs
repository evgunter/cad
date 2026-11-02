//! **Check 4's end samples are carrier points, not the edge's vertices.**
//! `EdgeCurve::certify` pins each carrier end within ε of its vertex
//! (check 3) and then reads the description residuals at every schedule
//! sample of the carrier, the two ends included (check 4): the
//! certificate bounds the carrier, D4 ¶2's derived cache, and a vertex
//! within ε of its surfaces says nothing about a carrier end within ε of
//! that vertex. So the verdict follows the carrier's end whatever the
//! vertex reads, at either orientation.
//!
//! The poses: the line from the origin to `(1, 0, r·ε)`, certified as
//! the intersection of the planes `z = 0` and `y = 0`. Its residual
//! against `z = 0` is `t·r·ε` along it, so for `r ≤ 8/7` only the far
//! end's sample can leave the zero band. The far vertex sits `s·ε`
//! below the carrier's end, inside check 3's pin.

#![allow(clippy::unwrap_used, clippy::expect_used, clippy::panic)]

use crate::shared::surf::table;
use crate::shared::tol::{band, eps};
use geom::{Curve3, Surface};
use geom_brep::{
    CERT_SAMPLES, CertCheck, CertifyError, EdgeCurve, EdgeCurveSpec, EdgeDescriptionSpec,
    SurfaceKey, SurfacePair,
};
use geom_core::{Point3, Vec3};

/// The pose's certification: the carrier's far end `r·ε` off `z = 0`,
/// its vertex `(r − s)·ε` off it, and the chord run from the far end when
/// `reversed`. Returns the `z = 0` plane's key beside the verdict.
fn certify_pose(r: f64, s: f64, reversed: bool) -> (SurfaceKey, Result<(), CertifyError>) {
    let e = eps();
    let (keys, lookup) = table(vec![
        Surface::Plane {
            origin: Point3::origin(),
            normal: Vec3::unit_z(),
            u_ref: Vec3::unit_x(),
        },
        Surface::Plane {
            origin: Point3::origin(),
            normal: Vec3::unit_y(),
            u_ref: Vec3::unit_x(),
        },
    ]);
    let near = Point3::origin();
    let far = Point3::new(1.0, 0.0, r * e);
    let far_vertex = Point3::new(1.0, 0.0, (r - s) * e);
    let (a, b, start, end) = if reversed {
        (far, near, far_vertex, near)
    } else {
        (near, far, near, far_vertex)
    };
    let len = (b - a).norm();
    let carrier = Curve3::Line {
        origin: a,
        dir: (b - a) / len,
    };
    let spec = EdgeCurveSpec {
        description: EdgeDescriptionSpec::Intersection {
            pair: SurfacePair::new(keys[0], keys[1]),
            selector: geom_brep::CurveSelector::Witness(carrier.mid_point(0.0, len)),
        },
        carrier,
        param_start: 0.0,
        param_end: len,
    };
    let verdict = EdgeCurve::certify(
        spec,
        start,
        end,
        &lookup,
        1.0,
        band(),
        geom_brep::NurbsLane::certified(),
    )
    .map(|_| ());
    (keys[0], verdict)
}

/// A carrier whose end lies just past the zero band escalates at that
/// end's sample, the far end's index at either orientation, though its
/// vertex reads inside the band.
#[test]
fn a_carrier_end_in_band_escalates_whatever_its_vertex_reads() {
    let mut wrong = Vec::new();
    for r in [1.01, 1.07, 1.14] {
        for s in [0.2, 0.5, 0.9] {
            for reversed in [false, true] {
                let far_sample = if reversed { 0 } else { CERT_SAMPLES - 1 };
                let (z_plane, verdict) = certify_pose(r, s, reversed);
                match verdict {
                    Err(CertifyError::Escalated {
                        check: CertCheck::SurfaceResidual { surface },
                        sample,
                        ..
                    }) if surface == z_plane && sample == far_sample => {}
                    other => wrong.push(format!(
                        "r {r}, s {s}, reversed {reversed}: {other:?}, wanted the z = 0 \
                         residual escalated at sample {far_sample}"
                    )),
                }
            }
        }
    }
    assert!(wrong.is_empty(), "{}", wrong.join("\n"));
}

/// A carrier inside the zero band certifies at either orientation, though
/// its far vertex reads in band of `z = 0`: whether a vertex lies on a
/// surface is not check 4's question.
#[test]
fn a_carrier_in_the_zero_band_certifies_whatever_its_vertex_reads() {
    let mut wrong = Vec::new();
    for r in [0.5, 0.8, 0.95] {
        for s in [-0.6, -0.9] {
            for reversed in [false, true] {
                if let (_, Err(err)) = certify_pose(r, s, reversed) {
                    wrong.push(format!("r {r}, s {s}, reversed {reversed}: {err:?}"));
                }
            }
        }
    }
    assert!(wrong.is_empty(), "{}", wrong.join("\n"));
}
