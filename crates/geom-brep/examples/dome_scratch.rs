//! Scratch: re-measure the curved-dome table of
//! `work/ssi/plane-nurbs-ssi-does-not-certify-a-curved-dome.md`.
//! Run once per ε with `CAD_TOLERANCE_EPS` set, and
//! `CAD_SCRATCH_REFINE=1` for the refinement rounds.
#![allow(clippy::unwrap_used, clippy::expect_used, clippy::panic)]

use geom::{NurbsSurface, Surface};
use geom_brep::ssi::{self, SsiDomain};
use geom_core::{Band, KnotVector, Point3, Tol, Vec3};

fn dome_wall(d: f64) -> NurbsSurface<f64> {
    let k = KnotVector::clamped(vec![0.0, 0.0, 0.0, 1.0, 1.0, 1.0], 2).unwrap();
    let mut control = Vec::with_capacity(9);
    for i in 0..3u8 {
        for j in 0..3u8 {
            let y = if i == 1 && j == 1 { -d } else { 0.0 };
            control.push(Point3::new(f64::from(i) / 2.0, y, f64::from(j) / 2.0));
        }
    }
    NurbsSurface::new(k.clone(), k, control, vec![1.0; 9]).unwrap()
}

fn main() {
    let band = Band::linear(Tol::witness()).unwrap();
    let eps = band.zero();
    let s2 = std::f64::consts::FRAC_1_SQRT_2;
    let ds: Vec<f64> = std::env::var("DOME_DS")
        .map(|v| v.split(',').map(|x| x.parse().unwrap()).collect())
        .unwrap_or_else(|_| vec![0.0, 0.05, 0.25, 0.5, 1.0, 2.0, 3.0, 4.0]);
    let cuts: Vec<String> = std::env::var("DOME_CUTS")
        .map(|v| v.split(',').map(str::to_string).collect())
        .unwrap_or_else(|_| {
            ["oblique", "tilt", "level", "zcut"]
                .iter()
                .map(|s| (*s).to_string())
                .collect()
        });
    for &d in &ds {
        let at = Point3::new(0.5, -d / 8.0, 0.5);
        let dom = SsiDomain {
            center: at,
            half_extent: std::env::var("DOME_HALF").map_or(2.0, |v| v.parse().unwrap()),
            extent: 1.0,
            floor_scale: 1.0,
        };
        for cut in &cuts {
            let plane = match cut.as_str() {
                "oblique" => Surface::Plane {
                    origin: at,
                    normal: Vec3::new(s2, 0.0, s2),
                    u_ref: Vec3::new(0.0, 1.0, 0.0),
                },
                "tilt" => Surface::Plane {
                    origin: at,
                    normal: Vec3::new(s2, s2, 0.0),
                    u_ref: Vec3::new(0.0, 0.0, 1.0),
                },
                "level" => Surface::Plane {
                    origin: at,
                    normal: Vec3::new(0.0, 1.0, 0.0),
                    u_ref: Vec3::new(1.0, 0.0, 0.0),
                },
                "zcut" => Surface::Plane {
                    origin: Point3::new(0.5, -d / 8.0, 0.2),
                    normal: Vec3::new(0.0, 0.0, 1.0),
                    u_ref: Vec3::new(1.0, 0.0, 0.0),
                },
                _ => panic!("cut {cut}"),
            };
            eprintln!("CASE eps={eps:e} d={d} cut={cut}");
            let t0 = std::time::Instant::now();
            let r = ssi::plane_nurbs_ssi(&plane, &dome_wall(d), dom, band);
            let secs = t0.elapsed().as_secs_f64();
            let what = match &r {
                Ok(o) => {
                    let ns: Vec<String> = o
                        .branches
                        .iter()
                        .map(|b| {
                            format!(
                                "{}:{:?}",
                                b.pcurve_b.as_ref().map_or(0, |c| c.control().len()),
                                b.end
                            )
                        })
                        .collect();
                    format!("OK branches={} [{}]", o.branches.len(), ns.join(" "))
                }
                Err(e) => format!("ERR {e:?}"),
            };
            println!("ROW eps={eps:e} d={d} cut={cut} secs={secs:.1} {what}");
        }
    }
}
