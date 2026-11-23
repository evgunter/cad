//! SCRATCH, branch `oneround/conic-harmonic-probe`: never merges.
//! M1, `plane_roots`: a unit path circle against the plane of an
//! arc-held ellipse, the plane stated through the derived centre (as the
//! branch stands) or through the stored point.
#![allow(clippy::all, clippy::pedantic, clippy::disallowed_methods)]

use std::io::Write;

use geom::Conic;
use geom_core::{Band, Point3, Tol, Vec3};

use super::{CircleArc, ConicArc, LoopArc, plane_roots};

#[test]
#[ignore = "probe"]
fn m1_plane_roots() {
    let band = Band::linear(Tol::witness()).unwrap();
    let dir = std::env::var("CONIC_PROBE_OUT").unwrap();
    let mut f = std::fs::File::create(format!("{dir}/m1e-eps{:e}.jsonl", band.zero())).unwrap();
    for a in [1e4, 1e5, 1e6, 1e7, 1e8, 1e9] {
        // The stored numbers `plane_cone_section` mints on the probe's
        // fat ladder (geom-brep's `the_mint_ladder`), `b = 0.3 a`.
        let e = geom::Ellipse::anchored(
            Point3::new(1.3, -0.7, 0.4),
            Vec3::new(0.5522240050794545, 0.3110026418424273, 0.7735153553621528),
            Vec3::new(0.6816514125372342, -0.702621773621506, -0.20414209516648582),
            a,
            0.3 * a,
        );
        let b = 0.3 * a;
        let theta = 0.3 / b;
        let half = 10.0 / b;
        let carrier = geom::Curve3::Ellipse(e);
        let arc = LoopArc::Conic(ConicArc { carrier: carrier.clone(), span: (theta - half, theta + half), own: None });
        let slack = geom_brep::carrier_resolution::resolution(&carrier, theta - half, theta + half).unwrap();
        let t = e.point_at(theta);
        let tan = e.derivative_at(theta).normalize();
        let n = e.axis();
        let side = n.cross(tan).normalize();
        for (i, gap) in [1.0, 0.5, 1e-3, 1e-6, 1e-7, 3e-8, 3e-9, -3e-8].into_iter().enumerate() {
            // A unit circle in the plane of `n` and `side`, dipping
            // `gap` through the ellipse's plane beside the target.
            let centre = t + n * (1.0 - gap);
            let piece = CircleArc {
                circle: geom::Circle::new(geom::Frame::new(centre, tan, side), 1.0),
                span: (0.0, 6.0),
            };
            for variant in 0..2u8 {
                let sec = geom::probe::with_variant(variant, || arc.plane().unwrap());
                let got = plane_roots(&piece, (sec.origin, sec.normal), false, slack, band);
                let (kind, roots) = match &got {
                    Ok(Some(r)) if r.is_empty() => ("misses".to_owned(), vec![]),
                    Ok(Some(r)) => ("meets".to_owned(), r.iter().map(|x| x.0).collect::<Vec<_>>()),
                    Ok(None) => ("graze-in-band".to_owned(), vec![]),
                    Err(_) => ("escalated".to_owned(), vec![]),
                };
                let (s, _) = e.probe_stored();
                writeln!(
                    f,
                    "{{\"a\":{a:?},\"case\":{i},\"gap\":{gap:?},\"variant\":{variant},\"S\":[{:?},{:?},{:?}],\"n\":[{:?},{:?},{:?}],\"c\":[{:?},{:?},{:?}],\"caxis\":[{:?},{:?},{:?}],\"cu\":[{:?},{:?},{:?}],\"slack\":{slack:?},\"kind\":{kind:?},\"roots\":{roots:?}}}",
                    s.x, s.y, s.z, n.x, n.y, n.z, centre.x, centre.y, centre.z, tan.x, tan.y, tan.z, side.x, side.y, side.z
                )
                .unwrap();
            }
        }
    }
}
