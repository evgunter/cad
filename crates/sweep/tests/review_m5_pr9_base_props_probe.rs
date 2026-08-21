//! Merge-base probe (charter D): can a constructible-at-rest body
//! reach the du_of_rims first-arc undercount SILENTLY at base?
//! Split BOTH rims of one wall face at their parameter midpoints:
//! four equal-span rim arcs on one face — the first-arc rule sees
//! consistent dts and returns HALF the true du.
#![allow(clippy::unwrap_used, clippy::expect_used, clippy::panic)]
use geom_core::{Point2, Tolerance};
use profile::RawLoop;
use profile::{Profile, ProfileLoop, ProfileVertex, SketchPlane};
use sweep::{Extrusion, extrude};
use geom_core::Tol;

#[test]
fn symmetric_double_rim_split_volume_at_base() {
    let lp = ProfileLoop::new(vec![
        ProfileVertex::new(Point2::new(-0.5, 0.0), 1.0),
        ProfileVertex::new(Point2::new(0.5, 0.0), 1.0),
    ]);
    let profile = Profile::new(SketchPlane::xy(), vec![lp])
        .validate(Tol::witness())
        .unwrap();
    let mut body = extrude(&profile, Extrusion::Distance(1.0)).unwrap().body;
    let true_vol = std::f64::consts::PI * 0.25 * 1.0;
    let v0 = topo::mass_properties(&body).unwrap().volume;
    assert!((v0 - true_vol).abs() < 1e-9, "pre-split volume sane");
    // Find one cylinder wall face and its two circle-carrier edges.
    let wall = body
        .faces()
        .find(|(_, f)| {
            matches!(
                body.get_surface(f.surface),
                Some(geom::Surface::Cylinder { .. })
            )
        })
        .map(|(k, _)| k)
        .unwrap();
    let mut rim_edges = Vec::new();
    for (ek, e) in body.edges() {
        let Some(c) = body.get_curve_geom(e.curve).and_then(|g| g.certified()) else {
            continue;
        };
        if !matches!(c.carrier(), geom::Curve3::Circle { .. }) {
            continue;
        }
        let on_wall = [e.he_plus, e.he_minus].iter().any(|&he| {
            body.get_half_edge(he)
                .and_then(|h| body.get_loop(h.parent_loop))
                .is_some_and(|l| l.face == wall)
        });
        if on_wall {
            let (t0, t1) = c.params();
            rim_edges.push((ek, t0 + (t1 - t0) * 0.5));
        }
    }
    assert_eq!(rim_edges.len(), 2, "one wall has two rim arcs");
    for (ek, tmid) in rim_edges {
        body.split_edge(ek, tmid).expect("rim split at rest");
    }
    let tier3 = topo::validate_geometric(&body);
    match topo::mass_properties(&body) {
        Err(e) => eprintln!("BASE: props refused typed ({e:?}) — main honest"),
        Ok(p) => {
            eprintln!(
                "BASE: props accepted: vol {} vs true {true_vol}; tier3 = {:?}",
                p.volume,
                tier3.as_ref().map(|_| "valid").map_err(|e| e.len())
            );
            assert!(
                (p.volume - true_vol).abs() < 1e-9,
                "SILENT WRONG VOLUME AT MERGE BASE: {} vs {true_vol} (tier3: {tier3:?})",
                p.volume
            );
        }
    }
}
