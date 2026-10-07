//! PR 4246 reviewer probe: does a public `topo::split` reach a ringed
//! cone face's re-homing? Every fixture is imported; a body holding a
//! cone face with rings is split by planes through that face.
#![allow(clippy::unwrap_used, clippy::expect_used, clippy::panic)]

use geom_core::{Point3, Tol, UnitVec3, Vec3};

fn files(dir: &std::path::Path, out: &mut Vec<std::path::PathBuf>) {
    for e in std::fs::read_dir(dir).unwrap() {
        let p = e.unwrap().path();
        if p.is_dir() {
            files(&p, out);
        } else if p.extension().is_some_and(|x| x == "step" || x == "stp") {
            out.push(p);
        }
    }
}

#[test]
#[ignore = "reviewer evidence: reports reach, asserts nothing"]
fn rp_ringed_cone_faces_split() {
    let tol = Tol::witness();
    let mut paths = Vec::new();
    for d in ["tests/fixtures", "../step-export/tests/fixtures"] {
        files(std::path::Path::new(d), &mut paths);
    }
    paths.sort();
    for p in paths {
        let Ok(text) = std::fs::read_to_string(&p) else { continue };
        let Ok(imp) = std::panic::catch_unwind(|| step_import::import_step(&text, &Default::default(), tol)) else {
            eprintln!("RPS {p:?}: import panicked");
            continue;
        };
        let Ok(step_import::StepImport::Solid { body, .. }) = imp else { continue };
        let ringed: Vec<_> = body
            .faces()
            .filter(|(_, f)| !f.rings.is_empty())
            .filter(|(_, f)| matches!(body.get_surface(f.surface), Some(geom::Surface::Cone { .. })))
            .map(|(k, _)| k)
            .collect();
        let cones = body.faces().filter(|(_, f)| matches!(body.get_surface(f.surface), Some(geom::Surface::Cone { .. }))).count();
        eprintln!("RPS {p:?}: {cones} cone faces, {} ringed", ringed.len());
        if ringed.is_empty() {
            continue;
        }
        let Ok(at) = topo::AtRestBody::validate(body.clone(), tol) else {
            eprintln!("RPS {p:?}: not at rest");
            continue;
        };
        for &face in &ringed {
            let f = body.get_face(face).unwrap();
            let Some(geom::Surface::Cone { apex, axis, .. }) = body.get_surface(f.surface).cloned() else { unreachable!() };
            // Points on the face: its outer and ring vertices.
            let mut pts = Vec::new();
            for l in std::iter::once(f.outer).chain(f.rings.iter().copied()) {
                if let topo::LoopBoundary::Cycle { first } = body.get_loop(l).unwrap().boundary {
                    let mut he = first;
                    loop {
                        pts.push(body.half_edge_start_point(he).unwrap());
                        he = body.get_half_edge(he).unwrap().next;
                        if he == first {
                            break;
                        }
                    }
                }
            }
            let (lo, hi) = pts.iter().fold((f64::MAX, f64::MIN), |(lo, hi), q| {
                let h = (*q - apex).dot(axis);
                (lo.min(h), hi.max(h))
            });
            for k in 0..24 {
                let h = lo + (hi - lo) * (f64::from(k) + 0.5) / 24.0;
                for tilt in [0.0, 0.2, -0.35] {
                    let n = (axis + axis.orthonormal_basis().0 * tilt).normalize();
                    let plane = topo::SplitPlane {
                        origin: apex + axis * h,
                        normal: UnitVec3::new(n, "rp", geom_core::Band::linear(tol).unwrap()).unwrap(),
                    };
                    eprintln!("RPS-SPLIT {p:?} face {face:?} h {h:.4} tilt {tilt}");
                    let r = std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| topo::split(&at, &plane, tol)));
                    let tag = match r {
                        Err(_) => "PANIC".to_string(),
                        Ok(Ok(_)) => "OK".to_string(),
                        Ok(Err(e)) => format!("{e:?}").chars().take(160).collect(),
                    };
                    eprintln!("RPS-RES {tag}");
                }
            }
        }
        let _ = (Point3::<f64>::origin(), Vec3::<f64>::unit_x());
    }
}
