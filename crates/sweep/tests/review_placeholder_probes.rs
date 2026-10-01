//! Reviewer probes for PR #3614 (placeholder chart sup arms).

#![allow(clippy::unwrap_used, clippy::expect_used, clippy::panic)]

use std::num::NonZeroUsize;
use std::sync::Arc;

use geom::{
    ApproxSurface, ApproxWindow, NurbsSurface, OffsetCertificate, Surface, SurfaceDescription,
    SurfaceSpec,
};
use geom_core::{KnotVector, Point3};
use topo::chart_boundary;

use crate::common::approx::{band, twisted_loft};

/// An approximating surface whose FIT is the placeholder: `ApproxSurface::certify`
/// runs whatever certifier it is handed, so the public API builds one.
fn approx_placeholder() -> Surface<f64> {
    let ph = NurbsSurface::<f64>::placeholder();
    let spec = SurfaceSpec {
        description: SurfaceDescription::Offset {
            base: Arc::new(ph.clone()),
            d: 0.0,
        },
        window: ApproxWindow::of(&ph),
        fit: ph,
    };
    let a = ApproxSurface::certify(spec, |_, _, _| {
        Ok::<_, ()>(OffsetCertificate {
            distance: 0.0,
            cells: 1,
            samples: 1,
            on_locus_max: 0.0,
            hull_sup: 0.0,
            normal_floor: 1.0,
            curvature_reach: 1.0,
            rounds: 0,
        })
    })
    .unwrap();
    Surface::Approx(Arc::new(a))
}

#[test]
fn probe_approx_placeholder_fit_chart_boundary() {
    let s = approx_placeholder();
    eprintln!(
        "[probe] spline_chart placeholder? {:?}",
        s.spline_chart().map(NurbsSurface::is_placeholder)
    );
    eprintln!(
        "[probe] chart_stretch_sup(approx ph) = {:?}",
        geom_brep::chart_stretch_sup(&s)
    );
    eprintln!(
        "[probe] chart_stretch_sup_v(approx ph) = {:?}",
        geom_brep::chart_stretch_sup_v(&s)
    );
    let body = twisted_loft(0.3);
    let mut ok = 0;
    for (face, _) in body.faces() {
        let r = chart_boundary(&body, face, &s, band());
        match &r {
            Ok(b) => {
                ok += 1;
                eprintln!("[probe] face {face:?}: OK {b:?}");
            }
            Err(e) => eprintln!("[probe] face {face:?}: Err {e}"),
        }
    }
    eprintln!("[probe] approx-placeholder chart_boundary Ok count = {ok}");
}

/// Knot insertion on a validated net whose weights are the smallest
/// positive subnormal: the refined weights are computed, not validated.
#[test]
fn probe_weight_ratio_after_knot_insertion() {
    let w = f64::from_bits(1); // 4.9e-324, passes validate_counts
    let k = KnotVector::unit_segment(NonZeroUsize::MIN);
    let ctl = vec![
        Point3::new(0.0, 0.0, 0.0),
        Point3::new(0.0, 1.0, 0.0),
        Point3::new(1.0, 0.0, 0.0),
        Point3::new(1.0, 1.0, 0.0),
    ];
    let s = NurbsSurface::new(k.clone(), k, ctl, vec![w; 4]).unwrap();
    let r = s.insert_knot_u(0.5, 1);
    eprintln!(
        "[probe] insert_knot_u -> {:?}",
        r.as_ref().map(|n| n.weights().to_vec())
    );
    if let Ok(n) = r {
        let surf = Surface::Nurbs(Arc::new(n));
        let got = std::panic::catch_unwind(|| {
            geom_brep::chart_stretch_sup(&surf).map(|(a, b)| (a.get(), b.get()))
        });
        eprintln!("[probe] chart_stretch_sup after insertion: {got:?}");
    }
    // Ratio overflow: validated weights 1e-200 and 1.
    let k = KnotVector::unit_segment(NonZeroUsize::MIN);
    let ctl = vec![
        Point3::new(0.0, 0.0, 0.0),
        Point3::new(0.0, 1.0, 0.0),
        Point3::new(1.0, 0.0, 0.0),
        Point3::new(1.0, 1.0, 0.0),
    ];
    let s = NurbsSurface::new(k.clone(), k, ctl, vec![1e-200, 1.0, 1.0, 1.0]).unwrap();
    let surf = Surface::Nurbs(Arc::new(s));
    eprintln!(
        "[probe] chart_stretch_sup wide weights: {:?}",
        geom_brep::chart_stretch_sup(&surf).map(|(a, b)| (a.get(), b.get()))
    );
}
