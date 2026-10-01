//! **A chart boundary on the mvfs placeholder refuses, typed.**
//!
//! `topo::chart_boundary` takes its chart from the caller. Handed the
//! placeholder for a face whose edges are described on the face's OWN
//! chart, the walk never evaluates the chart — it reads each image off
//! the description — so nothing poison stops it, and before the door
//! refused it metred every joint gap through the placeholder's unit
//! lever arms and returned a description of a region on a chart that
//! has no locus. A lofted body's planar caps are such faces.

#![allow(clippy::unwrap_used, clippy::expect_used, clippy::panic)]

use std::sync::Arc;

use geom::{
    ApproxSurface, ApproxWindow, NurbsSurface, OffsetCertificate, Surface, SurfaceDescription,
    SurfaceSpec,
};
use geom_brep::{NoChartSup, chart_stretch_sup, chart_stretch_sup_v};
use topo::{PcurveMintError, chart_boundary};

use crate::common::approx::{band, twisted_loft};

/// An approximating surface whose FIT is the placeholder: the public
/// API builds one, because `ApproxSurface::certify` runs whatever
/// certifier it is handed. Its chart is the placeholder's.
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
fn chart_boundary_refuses_the_placeholder_chart_on_every_face() {
    for (name, placeholder) in [
        ("Nurbs placeholder", Surface::<f64>::nurbs_placeholder()),
        ("Approx with a placeholder fit", approx_placeholder()),
    ] {
        refuses_on_every_face(name, &placeholder);
    }
}

fn refuses_on_every_face(name: &str, placeholder: &Surface<f64>) {
    assert!(
        placeholder.is_placeholder_chart(),
        "{name}: the one predicate"
    );
    assert!(
        matches!(chart_stretch_sup(placeholder), Err(NoChartSup::Placeholder)),
        "{name}: the sup pair refuses"
    );
    assert!(
        matches!(
            chart_stretch_sup_v(placeholder),
            Err(NoChartSup::Placeholder)
        ),
        "{name}: the v sup refuses"
    );
    let body = twisted_loft(0.3);
    let mut own_described = 0usize;
    for (face, f) in body.faces() {
        let own = body.get_surface(f.surface).unwrap();
        if chart_boundary(&body, face, own, band()).is_ok() {
            own_described += 1;
        }
        let refused = chart_boundary(&body, face, placeholder, band())
            .expect_err("no region of the placeholder is described");
        assert_eq!(
            refused,
            PcurveMintError::PlaceholderChart { face },
            "{name}, face {face:?}: the refusal names the placeholder, not a symptom of its poison"
        );
    }
    assert!(
        own_described >= 2,
        "the fixture must hold faces that describe on their own chart, or the \
         refusal above is not shown to be the placeholder's: {own_described}"
    );
}
