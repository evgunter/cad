//! **QUAD frame re-measure probes** (2026-10-21): the rows of
//! `work/quad/` that say a certified width or verdict of the
//! mass-properties quadrature moves when the body is translated or
//! rotated, re-taken on main. Every row prints and asserts nothing; each
//! is `#[ignore]`d and run by hand:
//!
//! `cargo test --release -p sweep --test all quad_frame_remeasure -- --ignored --nocapture --test-threads 1`

#![allow(clippy::unwrap_used, clippy::expect_used, clippy::panic)]

use core::f64::consts::PI;

use crate::common::arc_section;
use geom_brep::props::quad::{PatchNet, RVec3, mean_boundary_displacement, nurbs_patch_face};
use geom_core::linalg::frame::path_start_frame;
use geom_core::spline::KnotVector;
use geom_core::{Band, Bounds, Interval, Point2, Point3, Real, RigidMotion, Tol, UnitVec3};
use profile::{ProfileLoop, SketchPlane, circle, circle_split};
use sweep::{loft_body, sweep_body};
use topo::{Body, ValidationError};

fn tol() -> Tol {
    Tol::witness()
}

// ---------------------------------------------------------------- rows 1, 5

/// `geom_brep::ssi::enclose`'s `rational_wall` (cubic × linear, weights
/// 1.8 / 0.7), its net mapped through `map`.
fn rational_wall(
    map: &dyn Fn([f64; 3]) -> [f64; 3],
) -> (KnotVector, KnotVector, Vec<RVec3>, Vec<f64>) {
    let ku = KnotVector::clamped(vec![0.0, 0.0, 0.0, 0.0, 1.0, 1.0, 1.0, 1.0], 3).unwrap();
    let kv = KnotVector::clamped(vec![0.0, 0.0, 1.0, 1.0], 1).unwrap();
    let mut control = Vec::new();
    for (x, y) in [(0.0, 0.0), (0.35, 0.14), (0.70, 0.24), (1.05, 0.30)] {
        for z in [0.0, 0.8] {
            let p = map([x, y, z]);
            control.push(p.map(Interval::from_f64));
        }
    }
    (
        ku,
        kv,
        control,
        vec![1.0, 1.0, 1.8, 1.8, 0.7, 0.7, 1.0, 1.0],
    )
}

fn patch(label: &str, map: &dyn Fn([f64; 3]) -> [f64; 3]) {
    let (ku, kv, control, w) = rational_wall(map);
    let net = PatchNet::new(&ku, &kv, &control, &w).unwrap();
    let band = Band::linear(tol()).unwrap();
    let out = nurbs_patch_face::<f64>(&net, (0.0, 1.0, 0.0, 1.0), 3.8, 0.0, band, &mut |b, _| {
        Ok(mean_boundary_displacement(b.flux, b.area)? <= topo::test_support::pinned_len(tol()))
    });
    match out.map(|run| (run.end, run.bounds)) {
        Ok((end, fb)) => println!(
            "PATCH {label} ({end:?}): area [{:.15e}, {:.15e}] width {:.3e} | flux width {:.3e} (mid {:.6e})",
            fb.area.lo(),
            fb.area.hi(),
            fb.area.hi() - fb.area.lo(),
            fb.flux.hi() - fb.flux.lo(),
            0.5 * (fb.flux.hi() + fb.flux.lo()),
        ),
        Err(e) => println!("PATCH {label}: REFUSES {e:?}"),
    }
}

#[test]
#[ignore = "measurement probe; run by hand"]
fn row1_rational_wall_translated() {
    for t in [0.0, 1.0, 10.0, 100.0, 1.0e3, 1.0e5] {
        patch(&format!("translated t={t:e} (t,t,t)"), &|p| {
            [p[0] + t, p[1] + t, p[2] + t]
        });
    }
}

#[test]
#[ignore = "measurement probe; run by hand"]
fn row5_rational_wall_rotated() {
    let c = [0.525, 0.15, 0.4];
    // `ROW5_CENTRED=1` carries the wall's centre to the origin first, so
    // the rotation moves no point's distance from the flux origin.
    let back = if std::env::var("ROW5_CENTRED").is_ok() {
        0.0
    } else {
        1.0
    };
    let axes: [(&str, [f64; 3]); 3] = [
        ("z", [0.0, 0.0, 1.0]),
        ("x", [1.0, 0.0, 0.0]),
        ("(1,2,3)/|.|", [1.0, 2.0, 3.0]),
    ];
    for (name, a) in axes {
        let n = (a[0] * a[0] + a[1] * a[1] + a[2] * a[2]).sqrt();
        let k = [a[0] / n, a[1] / n, a[2] / n];
        for deg in [0.0, 15.0, 30.0, 45.0, 60.0, 90.0] {
            let (s, co) = (deg * PI / 180.0).sin_cos();
            // Rodrigues about `k` through the wall's centre `c`.
            let rot = |p: [f64; 3]| {
                let v = [p[0] - c[0], p[1] - c[1], p[2] - c[2]];
                let kxv = [
                    k[1] * v[2] - k[2] * v[1],
                    k[2] * v[0] - k[0] * v[2],
                    k[0] * v[1] - k[1] * v[0],
                ];
                let kv = k[0] * v[0] + k[1] * v[1] + k[2] * v[2];
                core::array::from_fn(|i| {
                    back * c[i] + v[i] * co + kxv[i] * s + k[i] * kv * (1.0 - co)
                })
            };
            patch(&format!("rotated {deg}° about {name}"), &rot);
        }
    }
}

// ---------------------------------------------------------------- row 3

const RLOOP: f64 = 1.2;
const ZTOP: f64 = 3.0;

fn loop_spine(off: [f64; 3]) -> geom::NurbsCurve3<f64> {
    let z_tube = ZTOP - 2.0 * RLOOP;
    let over = (0..=36u32).map(|k| {
        let th = 1.5 * PI * f64::from(k) / 36.0;
        (RLOOP * (1.0 - th.cos()), ZTOP + RLOOP * th.sin())
    });
    let into = (1..=12u32).map(|k| {
        let psi = 0.5 * PI + 0.5 * PI * f64::from(k) / 12.0;
        (RLOOP * (1.0 + psi.cos()), z_tube + RLOOP * psi.sin())
    });
    let pts: Vec<Point3<f64>> = over
        .chain(into)
        .map(|(x, z)| Point3::new(x + off[0], off[1], z + off[2]))
        .collect();
    geom::NurbsCurve3::interpolate(&pts, 3).unwrap()
}

fn annulus(split: usize) -> Vec<ProfileLoop<f64>> {
    [0.275, 0.225]
        .into_iter()
        .map(|r| {
            let c = Point2::new(0.0, 0.0);
            if split == 0 {
                circle(c, r, tol()).unwrap().into()
            } else {
                circle_split(c, r, split, 0.0, tol()).unwrap().into()
            }
        })
        .collect()
}

fn lofted_loop(
    spine: &geom::NurbsCurve3<f64>,
    section: &[ProfileLoop<f64>],
    stations: usize,
    vdeg: usize,
) -> Body<f64> {
    let band = Band::linear(tol()).unwrap();
    let (t0, t1) = spine.domain();
    let (start, tangent) = spine.ders1(t0);
    let roll = path_start_frame(start, tangent, tol()).unwrap().u().get();
    let places: Vec<SketchPlane<f64>> = (0..stations)
        .map(|i| {
            #[allow(clippy::cast_precision_loss)]
            let s = i as f64 / (stations - 1) as f64;
            let (p, d) = spine.ders1((t1 - t0).mul_add(s, t0));
            let aim = UnitVec3::new(d, "probe_tangent", band).unwrap();
            SketchPlane::from_frame(
                RigidMotion::from_aim_and_reference(p, aim, roll, "probe_roll", band).unwrap(),
            )
        })
        .collect();
    let sections = vec![section.to_vec(); stations];
    loft_body::<f64>(&sections, &places, vdeg, tol())
        .unwrap()
        .body
}

fn verdict(r: Result<(), Vec<ValidationError>>) -> String {
    match r {
        Ok(()) => "decides".into(),
        Err(e) => match &e[..] {
            [ValidationError::VolumeSignUnresolved { limit, volume, .. }] => {
                format!(
                    "leaves the sign unresolved: {limit:?}, bracket {volume:?} width {:.3e}",
                    volume.map_or(f64::NAN, |(lo, hi)| hi - lo)
                )
            }
            other => format!("OTHER {other:?}"),
        },
    }
}

#[test]
#[ignore = "measurement probe; run by hand"]
fn row3_swept_loop() {
    let env = |k: &str, d: &str| std::env::var(k).unwrap_or_else(|_| d.into());
    let splits: Vec<usize> = env("ROW3_SPLITS", "0,4")
        .split(',')
        .map(|x| x.parse().unwrap())
        .collect();
    let stations: Vec<usize> = env("ROW3_STATIONS", "9,17,33")
        .split(',')
        .map(|x| x.parse().unwrap())
        .collect();
    let degs: Vec<usize> = env("ROW3_DEGS", "2,3")
        .split(',')
        .map(|x| x.parse().unwrap())
        .collect();
    let offs: Vec<[f64; 3]> = env("ROW3_OFFS", "0:0:0;-1.2:0:-1.8")
        .split(';')
        .map(|o| {
            let v: Vec<f64> = o.split(':').map(|x| x.parse().unwrap()).collect();
            [v[0], v[1], v[2]]
        })
        .collect();
    if env("ROW3_SWEEP", "0") == "1" {
        let spine = loop_spine([0.0; 3]);
        let (t0, _) = spine.domain();
        let (start, tangent) = spine.ders1(t0);
        let place = path_start_frame(start, tangent, tol()).unwrap();
        let r = sweep_body::<f64>(
            &annulus(0),
            SketchPlane::from_frame(place),
            &spine,
            9,
            2,
            tol(),
        );
        println!(
            "ROW3 sweep_body circle 9/2: {:?}",
            r.map(|_| "builds").map_err(|e| format!("{e:?}"))
        );
    }
    for off in &offs {
        let spine = loop_spine(*off);
        for &sp in &splits {
            for &st in &stations {
                for &dg in &degs {
                    let t = std::time::Instant::now();
                    let body = lofted_loop(&spine, &annulus(sp), st, dg);
                    let v = (|| match topo::validate_geometric_certificate(&body, tol()) {
                        Ok(cert) => {
                            let e = cert.enclosure();
                            let gate = format!(
                                "decides, width {:.3e} [{cert:?}]",
                                e.volume_hi - e.volume_lo
                            );
                            if env("ROW3_MEASURE", "1") == "0" {
                                return gate;
                            }
                            match cert.measure(topo::test_support::pinned_resolution(tol())) {
                                Ok(m) => format!(
                                    "{gate}; measure {:?} volume {:.12e} pad {:.3e}",
                                    m.stop, m.enclosure.volume, m.enclosure.volume_pad
                                ),
                                Err(e) => format!("{gate}; measure REFUSES {e:?}"),
                            }
                        }
                        Err(e) => verdict(Err(e)),
                    })();
                    println!(
                        "ROW3 eps={:e} off={off:?} section={} stations={st} vdeg={dg}: {v} ({:.1}s)",
                        tol().eps(),
                        if sp == 0 {
                            "circle".to_string()
                        } else {
                            format!("split{sp}")
                        },
                        t.elapsed().as_secs_f64()
                    );
                }
            }
        }
    }
}

// ---------------------------------------------------------------- row 4

fn arc_prism_at(s: f64, dx: f64) -> Body<f64> {
    loft_body::<f64>(
        &[arc_section(s), arc_section(s)],
        &[0.0, 1.0].map(|h| SketchPlane::xy_at(Point3::new(dx * s, 0.0, h * s))),
        1,
        tol(),
    )
    .unwrap()
    .body
}

#[test]
#[ignore = "measurement probe; run by hand"]
fn row4_translated_arc_prism() {
    let eps = tol().eps();
    let s = 1e5 * eps;
    println!("ROW4 debug_assertions = {}", cfg!(debug_assertions));
    for metres in [0.0, 0.04, 0.4, 4.0, 40.0, 400.0, 4000.0] {
        let dx = metres / s;
        let body = arc_prism_at(s, dx);
        // The finish inside the catch, as the measurement is: a tier-3
        // refusal at a far placement prints as the row's PANIC line.
        let r = std::panic::catch_unwind(|| {
            topo::mass_properties(
                &topo::test_support::finished("body", body, tol()),
                tol(),
                topo::test_support::pinned_resolution(tol()),
            )
        });
        match r {
            Ok(Ok(mp)) => println!(
                "ROW4 eps={eps:e} s={s:e} at {metres} m: volume {:.6e} pad {:.3e} area {:.6e} pad {:.3e}",
                mp.enclosure.volume,
                mp.enclosure.volume_pad,
                mp.enclosure.surface_area,
                mp.enclosure.area_pad
            ),
            Ok(Err(e)) => println!("ROW4 eps={eps:e} at {metres} m: REFUSES {e:?}"),
            Err(p) => println!(
                "ROW4 eps={eps:e} at {metres} m: PANIC {:?}",
                p.downcast_ref::<String>()
                    .cloned()
                    .or_else(|| p.downcast_ref::<&str>().map(|s| (*s).to_string()))
            ),
        }
    }
}
