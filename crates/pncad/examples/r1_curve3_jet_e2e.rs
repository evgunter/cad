//! R1 review-lane end-to-end exercise for CURVE3-JET: a user-facing
//! NURBS path sweep through `pncad` (`sweep_places` and `sweep_body`,
//! the tour's `tube_place` shape), then a boolean over the swept
//! solid and a closed-body validation, all digested to bits so the
//! whole program's output can be compared across two builds.
//!
//! Run: `cargo run --release -p pncad --example r1_curve3_jet_e2e`

#![allow(clippy::unwrap_used, clippy::expect_used, clippy::panic)]

use std::fmt::Write as _;

use pncad::authoring::{p3, polygon, validated};
use pncad::geom_core::linalg::frame::path_start_frame;
use pncad::geom_core::{Affine3, OrthoFrame, Point3, Tol};
use pncad::sweep::Extrusion;
use pncad::profile::SketchPlane;

fn spine(turns: f64, pts: usize, degree: usize) -> pncad::geom::NurbsCurve3<f64> {
    let p: Vec<Point3<f64>> = (0..=pts)
        .map(|k| {
            #[allow(clippy::cast_precision_loss)]
            let s = k as f64 / pts as f64;
            let a = std::f64::consts::TAU * turns * s;
            Point3::new(1.3 * a.cos(), 0.9 * a.sin(), 0.75 * s)
        })
        .collect();
    pncad::geom::NurbsCurve3::interpolate(&p, degree).expect("spine")
}

fn fnv(h: &mut u64, v: f64) {
    *h = (*h ^ v.to_bits()).wrapping_mul(0x0000_0100_0000_01b3);
}

fn place_digest(places: &[Affine3<f64>]) -> u64 {
    let mut h = 0xcbf2_9ce4_8422_2325u64;
    for a in places {
        for v in [
            a.translation.x,
            a.translation.y,
            a.translation.z,
            a.linear.c0.x,
            a.linear.c0.y,
            a.linear.c0.z,
            a.linear.c1.x,
            a.linear.c1.y,
            a.linear.c1.z,
            a.linear.c2.x,
            a.linear.c2.y,
            a.linear.c2.z,
        ] {
            fnv(&mut h, v);
        }
    }
    h
}

fn cutter(tol: Tol) -> pncad::topo::Body<f64> {
    let poly = [(-0.45, -0.7), (0.45, -0.7), (0.45, 0.7), (-0.45, 0.7)];
    let plane = SketchPlane::from_frame(OrthoFrame::axes_yz(p3(0.55, -0.45, -0.7)));
    pncad::sweep::extrude::<f64>(
        &validated(plane, vec![polygon(&poly, tol).expect("cutter loop")], tol)
            .expect("cutter profile"),
        Extrusion::Distance(1.2),
        tol,
    )
    .expect("cutter extrudes")
    .body
}

fn main() {
    let tol = Tol::witness();
    let mut out = String::new();
    let cut = cutter(tol);

    for (turns, pts, deg) in [(0.4, 32usize, 3usize), (0.9, 24, 3), (0.25, 12, 2)] {
        let path = spine(turns, pts, deg);
        let (t0, _t1) = path.domain();

        // --- 1. The start-frame door, as a user reaches it (the tour
        // and example shape; the `ders1` fold site).
        let (p, tau) = path.ders1(t0);
        let place = path_start_frame(p, tau, tol).expect("start frame");
        writeln!(
            out,
            "spine({turns},{pts},{deg}) start p=({:x},{:x},{:x}) tau=({:x},{:x},{:x})",
            p.x.to_bits(),
            p.y.to_bits(),
            p.z.to_bits(),
            tau.x.to_bits(),
            tau.y.to_bits(),
            tau.z.to_bits()
        )
        .unwrap();

        // --- 2. `sweep_places`: the folded `skin.rs` site, two pairs.
        for stations in [5usize, 9, 17] {
            match pncad::sweep::sweep_places(place, &path, stations) {
                Ok(places) => writeln!(
                    out,
                    "  places({stations}) n={} fnv={:016x}",
                    places.len(),
                    place_digest(&places)
                )
                .unwrap(),
                Err(e) => writeln!(out, "  places({stations}) refused {e:?}").unwrap(),
            }
        }

        // --- 3. `sweep_body` + closed-body validation (tier 3 walks
        // the NURBS-carried edges: `validate.rs`'s folded site).
        let section = vec![
            polygon(
                &[(-0.12, -0.12), (0.12, -0.12), (0.12, 0.12), (-0.12, 0.12)],
                tol,
            )
            .expect("square"),
        ];
        for stations in [9usize, 17] {
            match pncad::sweep::sweep_body::<f64>(&section, place, &path, stations, deg, tol) {
                Ok(swept) => {
                    let m = pncad::topo::mass_properties(&swept.body, tol).expect("props");
                    writeln!(
                        out,
                        "  body({stations}) vol={:x} pad={:x} valid={:?}",
                        m.volume.to_bits(),
                        m.volume_pad.to_bits(),
                        pncad::topo::validate_closed(&swept.body).map_err(|e| e.len())
                    )
                    .unwrap();

                    // --- 4. A boolean over the swept NURBS solid: the
                    // `ops.rs` / `contact_verify.rs` / `certify.rs`
                    // fold sites run on its minted edges.
                    match pncad::topo::boolean::subtract::<f64>(&swept.body, &cut, tol) {
                        Ok(r) => match r.body() {
                            Some(bb) => {
                                let m2 = pncad::topo::mass_properties(&bb.body, tol)
                                    .expect("cut props");
                                writeln!(
                                    out,
                                    "  cut({stations}) kind={:?} vol={:x} valid={:?}",
                                    bb.kind,
                                    m2.volume.to_bits(),
                                    pncad::topo::validate_closed(&bb.body).map_err(|e| e.len())
                                )
                                .unwrap();
                            }
                            None => writeln!(out, "  cut({stations}) empty result").unwrap(),
                        },
                        Err(e) => writeln!(out, "  cut({stations}) refused {e:?}").unwrap(),
                    }
                }
                Err(e) => writeln!(out, "  body({stations}) refused {e:?}").unwrap(),
            }
        }
    }
    print!("{out}");
}
