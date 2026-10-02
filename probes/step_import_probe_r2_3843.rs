//! Reviewer probe (PR 3843, lane reach-dual3843-r2). Not a suite row.
//!
//! A ball of radius 10 mm whose sphere is split into a chart rectangle
//! R (latitude 10°..40°, azimuth 0..1 rad) and its COMPLEMENT C, which
//! holds both poles. C's boundary is R's, reversed: the same latitude
//! levels and the same azimuth hull. Cut by planes near the south pole
//! that miss R but cut C. Oracle: the spherical cap below `z = -h0` has
//! volume `π·h²(3R − h)/3`, `h = R − h0`.

#![allow(clippy::unwrap_used, clippy::expect_used, clippy::panic)]

use core::f64::consts::PI;

use geom_core::{Band, Point3, Tol, UnitVec3, Vec3};
use step_import::{ImportOptions, StepImport, import_step};
use topo::splitting::{SplitPart, SplitPlane, split};
use topo::{DATUM_UNIT_NORM, SolidContainment, point_in_solid};

const R: f64 = 0.010;

#[test]
fn probe_complement_face_split_near_a_pole() {
    let path = format!(
        "{}/../../probes/rect_complement.step",
        env!("CARGO_MANIFEST_DIR")
    );
    let text = std::fs::read_to_string(&path).unwrap();
    let body = match import_step(&text, &ImportOptions::default(), Tol::witness()) {
        Ok(StepImport::Solid { body, .. }) => body,
        other => panic!("import: {other:?}"),
    };
    eprintln!("tier1 {:?}", topo::validate(&body));
    eprintln!("tier2 {:?}", topo::validate_closed(&body));
    eprintln!("tier3 {:?}", topo::validate_geometric(&body, Tol::witness()).map_err(|e| e.len()));
    match topo::mass_properties(&body, Tol::witness()) {
        Ok(mp) => eprintln!("volume {} ± {} (ball {})", mp.volume, mp.volume_pad, 4.0 / 3.0 * PI * R * R * R),
        Err(e) => eprintln!("props: {e:?}"),
    }
    let band = Band::linear(Tol::witness()).unwrap();
    for (label, q) in [
        ("near S pole", Point3::new(0.0, 0.0, -0.0095)),
        ("near N pole", Point3::new(0.0, 0.0, 0.0095)),
        ("centre", Point3::new(0.0, 0.0, 0.0)),
    ] {
        eprintln!("pis {label}: {:?}", point_in_solid(&body, q, band, Tol::witness()));
    }
    let mut wrong = Vec::new();
    for (normal, h0) in [
        (Vec3::new(0.0, 0.0, 1.0), -0.0095),
        (Vec3::new(0.0, 0.0, 1.0), -0.009),
        (Vec3::new(0.0, 0.0, -1.0), -0.0095),
        (Vec3::new(0.0, 0.0, 1.0), 0.0095),
        (Vec3::new(1.0, 0.0, 0.0), -0.0095),
    ] {
        let n = UnitVec3::new(normal, DATUM_UNIT_NORM, band).unwrap();
        // The plane n·p = h0 (h0 signed along n).
        let plane = SplitPlane {
            origin: Point3::new(normal.x * h0, normal.y * h0, normal.z * h0),
            normal: n,
        };
        // Oracle: the material on the negative side of n·p = h0 is a cap
        // of height R + h0 when h0 < 0 (below), the rest otherwise.
        let ball = 4.0 / 3.0 * PI * R * R * R;
        let cap = |h: f64| PI * h * h * (3.0 * R - h) / 3.0;
        let below = if h0 < 0.0 { cap(R + h0) } else { ball - cap(R - h0) };
        let above = ball - below;
        match split(&body, &plane, Tol::witness()) {
            Err(e) => eprintln!("n={normal:?} h0={h0}: refused {e}"),
            Ok(res) => {
                let vol = |p: &SplitPart<f64>| match p {
                    SplitPart::Empty => 0.0,
                    SplitPart::Body(b) => topo::mass_properties(b, Tol::witness())
                        .map(|m| m.volume)
                        .unwrap_or(f64::NAN),
                };
                let (ka, kb) = (vol(&res.above), vol(&res.below));
                eprintln!(
                    "n={normal:?} h0={h0}: SPLIT above {ka:e} (oracle {above:e}) below {kb:e} (oracle {below:e}) empty? above={} below={}",
                    matches!(res.above, SplitPart::Empty),
                    matches!(res.below, SplitPart::Empty)
                );
                if (ka - above).abs() > 1e-9 * ball.max(1e-30) * 1e3 || (kb - below).abs() > 1e-6 * ball {
                    wrong.push((normal, h0, ka, kb, above, below));
                }
            }
        }
    }
    assert!(wrong.is_empty(), "silently wrong splits: {wrong:?}");
}
