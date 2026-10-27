//! Review probe (E1d): incidence endings by read and door, at f64 and Interval.
#![allow(clippy::unwrap_used, clippy::expect_used, clippy::panic)]
use crate::shared::tol::{band, eps};
use geom::{Curve3, Surface};
use geom_brep::recourse::{ReadAt, Reading};
use geom_brep::{IncidenceRead, PcurveCertifyError, chart_pcurve_over};
use geom_core::{Decide, FileCoincidence, Interval, Point3, Tol, Vec3};

fn doors() -> [(&'static str, ReadAt); 3] {
    [
        ("build", Reading::Build.into()),
        ("rest", Reading::AtRest.into()),
        ("import", FileCoincidence::new(1e-6).into()),
    ]
}

fn report(label: &str, r: Result<(), PcurveCertifyError>) {
    match r {
        Ok(()) => println!("PROBE {label}: imaged"),
        Err(e) => {
            let read = IncidenceRead::of(&e);
            println!("PROBE {label}: {e:?}\n  read={read:?}");
            for (name, at) in doors() {
                let t = e.render(at);
                println!(
                    "  [{name}] defect={} lever={} eps_in={} tol_offer={} :: {t}",
                    t.contains("defect"),
                    t.contains("Recourse:"),
                    t.contains("ε_in"),
                    t.contains("loosen") || t.contains("tighten") || t.contains("tolerance to"),
                );
            }
        }
    }
}

fn run<T: Decide>(lane: &str) {
    let f = T::from_f64;
    let e = eps();
    let k = Tol::witness().k();
    let cone = Surface::Cone {
        apex: Point3::new(f(0.0), f(0.0), f(0.0)),
        axis: Vec3::new(f(0.0), f(0.0), f(1.0)),
        half_angle: f(0.5),
        u_ref: Vec3::new(f(1.0), f(0.0), f(0.0)),
    };
    // A rim at the apex's height within the band: at a pose.
    for h in [0.5 * e, 2.0 * e, 0.5 * (1.0 + k) * e, 3.0 * k * e] {
        let r = h * 0.5f64.tan();
        let c = Curve3::Circle {
            center: Point3::new(f(0.0), f(0.0), f(h)),
            axis: Vec3::new(f(0.0), f(0.0), f(1.0)),
            radius: f(r.max(1e-300)),
            u_ref: Vec3::new(f(1.0), f(0.0), f(0.0)),
        };
        report(
            &format!("{lane} cone rim at h={h:e}"),
            chart_pcurve_over(&c, f(0.0), f(6.0), &cone, band()).map(|_| ()),
        );
    }
    // A generator moved off the cone by s.
    for s in [0.5 * e, 3.0 * e, 0.5 * (1.0 + k) * e, 3.0 * k * e, 1e-3] {
        let (sa, ca) = 0.5f64.sin_cos();
        let dir = Vec3::new(f(sa), f(0.0), f(ca));
        // the outward normal of the generator in the xz plane
        let nrm = Vec3::new(f(ca), f(0.0), f(-sa));
        let origin = Point3::new(f(0.0), f(0.0), f(0.0)) + nrm * f(s) + dir * f(1.0);
        let l = Curve3::Line { origin, dir };
        report(
            &format!("{lane} cone generator moved {s:e}"),
            chart_pcurve_over(&l, f(0.0), f(1.0), &cone, band()).map(|_| ()),
        );
    }
    // A tiny circle at a sphere's pole, and a thin torus's meridian.
    let sphere = Surface::Sphere {
        center: Point3::new(f(0.0), f(0.0), f(0.0)),
        radius: f(1.0),
        axis: Vec3::new(f(0.0), f(0.0), f(1.0)),
        u_ref: Vec3::new(f(1.0), f(0.0), f(0.0)),
    };
    for rr in [0.5 * e, 2.0 * e, 3.0 * k * e] {
        let z = (1.0 - rr * rr).sqrt();
        let c = Curve3::Circle {
            center: Point3::new(f(0.0), f(0.0), f(z)),
            axis: Vec3::new(f(0.0), f(0.0), f(1.0)),
            radius: f(rr),
            u_ref: Vec3::new(f(1.0), f(0.0), f(0.0)),
        };
        report(
            &format!("{lane} sphere polar circle r={rr:e}"),
            chart_pcurve_over(&c, f(0.0), f(6.0), &sphere, band()).map(|_| ()),
        );
        // the same circle tilted a hair: no longer polar, near the pole
        let tilt = 3.0 * k * e;
        let ax = Vec3::new(f(tilt), f(0.0), f(1.0));
        let ax = ax / ax.norm();
        let c = Curve3::Circle {
            center: Point3::new(f(0.0), f(0.0), f(0.0)) + ax * f(z),
            axis: ax,
            radius: f(rr),
            u_ref: Vec3::new(f(0.0), f(1.0), f(0.0)),
        };
        report(
            &format!("{lane} sphere near-pole tilted circle r={rr:e}"),
            chart_pcurve_over(&c, f(0.0), f(6.0), &sphere, band()).map(|_| ()),
        );
    }
    for rt in [0.5 * e, 2.0 * e] {
        let torus = Surface::Torus {
            center: Point3::new(f(0.0), f(0.0), f(0.0)),
            axis: Vec3::new(f(0.0), f(0.0), f(1.0)),
            major_radius: f(2.0),
            minor_radius: f(rt),
            u_ref: Vec3::new(f(1.0), f(0.0), f(0.0)),
        };
        let c = Curve3::Circle {
            center: Point3::new(f(2.0), f(0.0), f(0.0)),
            axis: Vec3::new(f(0.0), f(1.0), f(0.0)),
            radius: f(rt),
            u_ref: Vec3::new(f(1.0), f(0.0), f(0.0)),
        };
        report(
            &format!("{lane} thin torus meridian r={rt:e}"),
            chart_pcurve_over(&c, f(0.0), f(6.0), &torus, band()).map(|_| ()),
        );
    }
}

#[test]
fn e1d_probe_reads_by_door() {
    run::<f64>("f64");
    run::<Interval>("Interval");
}
