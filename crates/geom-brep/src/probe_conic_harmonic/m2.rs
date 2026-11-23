//! M2: the sphere beside the anchor, through `conic_quadric_roots`.
//! Variant 0 is the branch; 1 hands the existing subdivision the
//! residual's jet formed in the offsets about the stored point
//! (`conic_quadric::ProbeOffsetJet`, `Terms::Local`), the harmonics'
//! noise as the door charges it; 3 also zeroes that noise, so the
//! ladder runs on the centre harmonics.

use std::io::Write;

use geom::{Conic, Curve3, Frame, Surface};

use super::{band, minted, out, p3, stored, triple};
use crate::curve_face::conic_quadric::{ProbeOffsetJet, conic_quadric_roots};

fn log_json() -> String {
    let lines: Vec<String> = geom::probe::take_log().into_iter().map(|l| format!("{l:?}")).collect();
    format!("[{}]", lines.join(","))
}

#[test]
#[ignore = "probe"]
fn m2_sphere_beside_the_anchor() {
    let band = band();
    let mut f = out("m2");
    let (ex, _, ez) = triple();
    for fat in [true, false] {
        for target in [1e4, 1e5, 1e6, 1e7, 1e8] {
            let Some(e) = minted(target, fat) else { continue };
            let (_, b) = e.semi_axes();
            let carrier = Curve3::Ellipse(e);
            let mut case = 0;
            for arc in [0.0, 0.4, -3.0, 300.0] {
                for radius in [1.0, 0.01, 30.0] {
                    for depth in [0.2, 0.7, 0.99, 0.9999] {
                        case += 1;
                        let theta = arc / b;
                        let t = e.point_at(theta);
                        let tan = e.derivative_at(theta).normalize();
                        let n = e.axis();
                        let side = (n.cross(tan).normalize() * 0.6 + n * 0.8).normalize();
                        let centre = t + side * (depth * radius);
                        let sphere = Surface::Sphere { frame: Frame::new(centre, ez, ex), radius };
                        let half = (3.0 * radius / b).max(1e-12);
                        let (t0, t1) = (theta - half, theta + half);
                        for variant in [0u8, 1, 3] {
                            let _ = geom::probe::take_log();
                            let got = geom::probe::with_variant(variant, || conic_quadric_roots(&carrier, t0, t1, &sphere, band));
                            let (kind, roots) = match &got {
                                Ok(Ok(crate::roots::Roots::Certified(found))) => ("roots".to_owned(), found.roots().to_vec()),
                                Ok(Ok(crate::roots::Roots::Miss)) => ("miss".to_owned(), vec![]),
                                Ok(Ok(crate::roots::Roots::On(_))) => ("on".to_owned(), vec![]),
                                Ok(Err(u)) => (format!("unsettled:{u:?}"), vec![]),
                                Err(crate::curve_face::conic_quadric::ConicQuadricRefusal::Sphere(d)) => (format!("escalated:{:?}", d.predicate), vec![]),
                                Err(other) => (format!("refused:{other:?}"), vec![]),
                            };
                            // The offsets jet at each root and at the span's ends, for the oracle to check its bounds.
                            let jet = ProbeOffsetJet::new(&e, (centre, None, radius));
                            let probes: Vec<String> = roots.iter().copied().chain([t0, t1, theta, 0.5, 2.0]).map(|x| {
                                let j = jet.jet(x);
                                format!("[{x:?},{:?},{:?},{:?},{:?},{:?},{:?},{:?},{:?}]", j[0].value, j[0].error, j[1].value, j[1].error, j[2].value, j[2].error, j[3].value, j[3].error)
                            }).collect();
                            writeln!(
                                f,
                                "{{\"variant\":{variant},\"fat\":{fat},\"target\":{target:?},\"case\":{case},\"arc\":{arc:?},\"radius\":{radius:?},\"depth\":{depth:?},{},\"span\":[{t0:?},{t1:?}],\"c\":{},\"kind\":{kind:?},\"roots\":{roots:?},\"jet\":[{}],\"fourth\":{:?},\"log\":{}}}",
                                stored(&e), p3(centre), probes.join(","), jet.fourth(), log_json()
                            )
                            .unwrap();
                        }
                    }
                }
            }
        }
    }
}
