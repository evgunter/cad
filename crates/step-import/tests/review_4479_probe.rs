//! REVIEW PROBE (PR 4479 delta): one-segment lofts, whose strut now
//! lives on [-1, 0], export to STEP and import back.
#![allow(clippy::unwrap_used, clippy::expect_used, clippy::panic)]
use core::f64::consts::TAU;
use geom_core::{Affine3, Arc2, Point2, Tol, Vec3};
use profile::{ProfileLoop, RawLoop, Segment};

fn circle() -> ProfileLoop<f64> {
    RawLoop::new([(
        Point2::new(1.0, 0.0),
        Segment::Arc(Arc2 {
            centre: Point2::new(0.0, 0.0),
            radius: 1.0,
            sweep: TAU,
        }),
    )])
}

#[test]
fn review_probe_one_segment_loft_step_round_trip() {
    for (z, degree) in [
        (&[0.0, 1.0, 2.0, 3.0][..], 1),
        (&[0.0, 1.0, 3.0][..], 1),
        (&[0.0, 1.0, 2.0, 3.0, 4.0, 5.0, 6.0, 7.0][..], 2),
        (&[0.0, 2.0][..], 1),
    ] {
        let places: Vec<_> = z
            .iter()
            .map(|z| Affine3::translation(Vec3::new(0.0, 0.0, *z)))
            .collect();
        let sections: Vec<_> = z.iter().map(|_| vec![circle()]).collect();
        let body = sweep::loft_body::<f64>(&sections, &places, degree, Tol::witness())
            .unwrap()
            .body;
        let doc =
            step_export::step_string(&body, &step_export::StepOptions::default(), Tol::witness())
                .unwrap_or_else(|e| panic!("{z:?}: export: {e}"));
        let neg = doc
            .lines()
            .filter(|l| l.contains("B_SPLINE_CURVE_WITH_KNOTS") && l.contains("(-1."))
            .count();
        eprintln!(
            "PROBE {z:?} d{degree}: {} bytes, {neg} negative-knot B-spline curve lines",
            doc.len()
        );
        for l in doc
            .lines()
            .filter(|l| l.contains("B_SPLINE_CURVE_WITH_KNOTS") && l.contains("(-"))
        {
            eprintln!("  {}", &l[..l.len().min(400)]);
        }
        let back =
            step_import::import_step(&doc, &step_import::ImportOptions::default(), Tol::witness())
                .unwrap_or_else(|e| panic!("{z:?}: import: {e}"));
        eprintln!(
            "PROBE {z:?}: import ok: {}",
            match &back {
                _ => "Ok",
            }
        );
    }
}
