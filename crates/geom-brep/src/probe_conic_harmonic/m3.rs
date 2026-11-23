//! M3: the ladder's anchors. The sphere poses of M2 and grazes inside
//! the band, through `conic_quadric_roots` with the ladder made to run
//! (the noise zeroed, the subdivision on the offsets jet): variant 3
//! hands the ladder the centre harmonics, 4 re-bases their constant to
//! the anchor from the offsets, 5 forms the quartic's coefficients about
//! the anchor from the offsets jet at the anchor and at its pole.

use std::io::Write;

use geom::{Conic, Curve3, Frame, Surface};

use super::{band, minted, out, p3, stored, triple};
use crate::curve_face::conic_quadric::conic_quadric_roots;

#[test]
#[ignore = "probe"]
fn m3_the_ladders_anchors() {
    let band = band();
    let eps = band.zero();
    let mut f = out("m3");
    let (ex, _, ez) = triple();
    for fat in [true, false] {
        for target in [1e4, 1e5, 1e6, 1e7, 1e8] {
            let Some(e) = minted(target, fat) else { continue };
            let (_, b) = e.semi_axes();
            let carrier = Curve3::Ellipse(e);
            let mut case = 0;
            for arc in [0.4, -3.0, 300.0] {
                for radius in [1.0, 0.01, 30.0] {
                    // `dip`: how far the arc dips into the sphere, in
                    // metres; the last three are grazes inside the band,
                    // in its gap, and a miss inside the band.
                    for (name, dip) in [("deep", 0.8 * radius), ("mid", 0.3 * radius), ("shallow", 1e-4 * radius), ("in-band", 0.3 * eps), ("in-gap", 4.0 * eps), ("miss-in-band", -0.3 * eps)] {
                        case += 1;
                        let theta = arc / b;
                        let t = e.point_at(theta);
                        let tan = e.derivative_at(theta).normalize();
                        let n = e.axis();
                        let side = (n.cross(tan).normalize() * 0.6 + n * 0.8).normalize();
                        let centre = t + side * (radius - dip);
                        let sphere = Surface::Sphere { frame: Frame::new(centre, ez, ex), radius };
                        let half = (3.0 * radius / b).max(1e-12);
                        let (t0, t1) = (theta - half, theta + half);
                        for variant in [1u8, 3, 4, 5] {
                            let _ = geom::probe::take_log();
                            let got = geom::probe::with_variant(variant, || conic_quadric_roots(&carrier, t0, t1, &sphere, band));
                            let kind = match &got {
                                Ok(Ok(crate::roots::Roots::Certified(found))) => format!("roots {}", found.count()),
                                Ok(Ok(crate::roots::Roots::Miss)) => "miss".to_owned(),
                                Ok(Ok(crate::roots::Roots::On(_))) => "on".to_owned(),
                                Ok(Err(u)) => format!("unsettled:{u:?}"),
                                Err(crate::curve_face::conic_quadric::ConicQuadricRefusal::Sphere(d)) => format!("escalated:{}", d.predicate.unwrap_or("?")),
                                Err(other) => format!("refused:{other:?}"),
                            };
                            let log: Vec<String> = geom::probe::take_log().into_iter().filter(|l| l.starts_with("ladder-")).map(|l| format!("{l:?}")).collect();
                            writeln!(
                                f,
                                "{{\"variant\":{variant},\"fat\":{fat},\"target\":{target:?},\"case\":{case},\"arc\":{arc:?},\"radius\":{radius:?},\"pose\":\"{name}\",\"dip\":{dip:?},{},\"span\":[{t0:?},{t1:?}],\"c\":{},\"kind\":{kind:?},\"log\":[{}]}}",
                                stored(&e), p3(centre), log.join(",")
                            )
                            .unwrap();
                        }
                    }
                }
            }
        }
    }
}
