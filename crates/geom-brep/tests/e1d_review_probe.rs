//! E1d review probes (not for merge): the edge certificate's incidence
//! endings at f64 and Interval, at the build, at rest and at the import
//! door, with a pose height past and within ε_in.

#![allow(clippy::unwrap_used, clippy::expect_used, clippy::panic)]

use crate::shared::tol::band;
use geom::{Curve3, Surface};
use geom_brep::recourse::{ReadAt, Reading};
use geom_brep::{
    CertCheck, CertifyError, EdgeCurve, EdgeCurveSpec, EdgeDescriptionSpec, OffMargin,
    PcurveCertifyError, chart_pcurve,
};
use geom_core::{FileCoincidence, Interval, Point3, Real, Vec3};

fn cone() -> Surface<f64> {
    Surface::Cone {
        apex: Point3::origin(),
        axis: Vec3::unit_z(),
        half_angle: 0.5,
        u_ref: Vec3::unit_x(),
    }
}

fn sphere() -> Surface<f64> {
    Surface::Sphere {
        center: Point3::origin(),
        radius: 1.0,
        axis: Vec3::unit_z(),
        u_ref: Vec3::unit_x(),
    }
}

fn certify<T: geom_core::Decide>(s: Surface<f64>, c: Curve3<f64>) -> CertifyError {
    let at = <T as Real>::from_f64;
    let s = s.map_scalar(at);
    let c: Curve3<T> = c.map_scalar(at);
    let (keys, lookup) = crate::shared::surf::table(vec![s]);
    let (t0, t1) = (at(0.0), at(1.0));
    EdgeCurve::certify(
        EdgeCurveSpec {
            description: EdgeDescriptionSpec::chart(keys[0]),
            carrier: c.clone(),
            param_start: t0,
            param_end: t1,
        },
        c.eval(t0),
        c.eval(t1),
        &lookup,
        at(1.0),
        band(),
    )
    .expect_err("no curve of its chart")
}

fn rows() -> Vec<(&'static str, Surface<f64>, Curve3<f64>)> {
    let mid = 0.5 * (band().zero() + band().escalate());
    vec![
        (
            "off-none",
            sphere(),
            Curve3::Line {
                origin: Point3::origin(),
                dir: Vec3::unit_x(),
            },
        ),
        (
            // a circle tilted 30° about (0,0,1) of radius 3: its lowest
            // point dips below the apex height by ~0.5 → Past
            "off-past",
            cone(),
            Curve3::Circle {
                center: Point3::new(0.0, 0.0, 1.0),
                axis: Vec3::new(0.5, 0.0, 0.75_f64.sqrt()),
                radius: 3.0,
                u_ref: Vec3::new(0.75_f64.sqrt(), 0.0, -0.5),
            },
        ),
        (
            "off-past-tiny",
            cone(),
            Curve3::Circle {
                center: Point3::new(0.0, 0.0, 1.0),
                axis: Vec3::new(0.5, 0.0, 0.75_f64.sqrt()),
                radius: 2.0 + 40.0 * band().escalate(),
                u_ref: Vec3::new(0.75_f64.sqrt(), 0.0, -0.5),
            },
        ),
        (
            "undecided",
            sphere(),
            Curve3::Circle {
                center: Point3::origin(),
                axis: Vec3::new(0.3_f64.sin(), 0.0, 0.3_f64.cos()),
                radius: 1.0 + mid,
                u_ref: Vec3::unit_y(),
            },
        ),
        (
            "at-pose",
            cone(),
            Curve3::Circle {
                center: Point3::new(0.0, 0.0, 1.0),
                axis: Vec3::new(0.5, 0.0, 0.75_f64.sqrt()),
                radius: 2.0,
                u_ref: Vec3::new(0.75_f64.sqrt(), 0.0, -0.5),
            },
        ),
        ("sphere-parallel-inband", sphere(), Curve3::Circle { center: Point3::new(0.0,0.0,0.6), axis: Vec3::unit_z(), radius: 0.8 + mid, u_ref: Vec3::unit_x() }),
        ("sphere-parallel-off", sphere(), Curve3::Circle { center: Point3::new(0.0,0.0,0.6), axis: Vec3::unit_z(), radius: 0.8 + 1e-3, u_ref: Vec3::unit_x() }),
        ("cone-rim-inband", cone(), Curve3::Circle { center: Point3::new(0.0,0.0,1.0), axis: Vec3::unit_z(), radius: 0.5_f64.tan() + mid, u_ref: Vec3::unit_x() }),
        ("cone-rim-off", cone(), Curve3::Circle { center: Point3::new(0.0,0.0,1.0), axis: Vec3::unit_z(), radius: 0.5_f64.tan() + 1e-3, u_ref: Vec3::unit_x() }),
        ("torus-parallel-off", Surface::Torus { center: Point3::origin(), axis: Vec3::unit_z(), major_radius: 2.0, minor_radius: 0.5, u_ref: Vec3::unit_x() }, Curve3::Circle { center: Point3::origin(), axis: Vec3::unit_z(), radius: 2.5 + 1e-3, u_ref: Vec3::unit_x() }),
        ("torus-parallel-inband", Surface::Torus { center: Point3::origin(), axis: Vec3::unit_z(), major_radius: 2.0, minor_radius: 0.5, u_ref: Vec3::unit_x() }, Curve3::Circle { center: Point3::origin(), axis: Vec3::unit_z(), radius: 2.5 + mid, u_ref: Vec3::unit_x() }),
        (
            // sphere pole: a small circle at the pole off the sphere by mid
            "sphere-pole-small",
            sphere(),
            Curve3::Circle {
                center: Point3::new(0.0, 0.0, 1.0 - 1e-7),
                axis: Vec3::unit_z(),
                radius: (1.0 - (1.0 - 1e-7_f64).powi(2)).sqrt() + mid,
                u_ref: Vec3::unit_x(),
            },
        ),
    ]
}

fn read<T: geom_core::Decide>(lane: &str) {
    for (what, s, c) in rows() {
        let e = certify::<T>(s.clone(), c.clone());
        let pc = chart_pcurve(&c.map_scalar(<T as Real>::from_f64), &s.map_scalar(<T as Real>::from_f64), band());
        eprintln!("PROBE {lane} {what}: {e:?}\n   pcurve: {:?}", pc.as_ref().err());
        let definite = matches!(
            e,
            CertifyError::CarrierOffChart {
                margin: None | Some(OffMargin::Past(_)),
                ..
            }
        );
        let inband = matches!(
            e,
            CertifyError::CarrierOffChart {
                margin: Some(OffMargin::InBand(_)),
                ..
            } | CertifyError::Escalated {
                check: CertCheck::ChartIncidence,
                ..
            }
        );
        for (door, culprit) in [
            (ReadAt::from(Reading::Build), geom_core::KERNEL_DEFECT_ENDING),
            (ReadAt::from(Reading::AtRest), geom_core::KERNEL_OR_FILE_DEFECT_ENDING),
            (ReadAt::from(FileCoincidence::new(1e-6)), geom_core::DAMAGED_FILE_ENDING),
            (ReadAt::from(FileCoincidence::new(1.0)), geom_core::DAMAGED_FILE_ENDING),
            (ReadAt::from(FileCoincidence::new(1e-12)), geom_core::DAMAGED_FILE_ENDING),
        ] {
            let text = e.render(door);
            eprintln!("   {door:?}: {text}");
            if text.contains("ε_in") { eprintln!("VIOLATION ε_in {lane} {what} {door:?}"); }
            if definite {
                if !text.ends_with(culprit) { eprintln!("VIOLATION culprit {lane} {what} {door:?}"); }
                assert!(!text.contains("Recourse"), "{lane} {what} {door:?}: {text}");
            } else if inband {
                assert!(text.contains("Recourse: "), "{lane} {what} {door:?}: {text}");
                for w in ["defect", "loosen", "tighten", "report it", "set ε"] {
                    assert!(!text.contains(w), "{lane} {what} {door:?}: {w}: {text}");
                }
            }
        }
        if let Err(p) = pc {
            for door in [
                ReadAt::from(Reading::Build),
                ReadAt::from(Reading::AtRest),
                ReadAt::from(FileCoincidence::new(1.0)),
            ] {
                let t = p.render(door);
                if let Some(read) = geom_brep::IncidenceRead::of(&p) {
                    assert!(t.ends_with(&read.ending(door)), "{t}");
                    assert_eq!(p.ending(door), Some(read.ending(door)));
                }
                let _: &PcurveCertifyError = &p;
            }
        }
    }
}

#[test]
fn e1d_probe_edge_certificate_incidence_by_door() {
    read::<f64>("f64");
    read::<Interval>("Interval");
}

/// A hand-built (pub-field) refusal whose decided check is ChartIncidence
/// but which is not one IncidenceRead::of reads: does the public
/// `ending` / `render` / `Display` panic?
#[test]
fn e1d_probe_residual_exceeded_chart_incidence_panics() {
    let e = PcurveCertifyError::ResidualExceeded {
        check: geom_brep::PcurveCheck::ChartIncidence,
        sample: 0,
    };
    let r = std::panic::catch_unwind(|| e.ending(Reading::Build));
    eprintln!("PROBE ending panics: {}", r.is_err());
    let r = std::panic::catch_unwind(|| e.to_string());
    eprintln!("PROBE Display panics: {} {:?}", r.is_err(), r.ok());
    let r = std::panic::catch_unwind(|| e.refused_check());
    eprintln!("PROBE refused_check: {:?}", r.ok());
}
