//! PROBE (temporary)
#![allow(clippy::unwrap_used, clippy::expect_used, clippy::panic)]

use geom_core::Tol;
use sweep::Revolution;
use sweep::blend::build::fillet_edges;
use sweep::chamfer::chamfer_edges;
use sweep::test_support::{finished, lantern, rim_arcs_at, sphere_zone};

#[test]
fn probe_blend_sizes() {
    let t = Tol::witness();
    let bodies = [
        ("lantern", lantern(t), vec![(1.0, 0.0), (0.8, 0.6), (0.2, 1.2)]),
        (
            "zone",
            sphere_zone(0.6, Revolution::Full, t),
            vec![(1.936_491_673_103_708_5, -0.5), (1.732_050_807_568_877_2, 1.0)],
        ),
    ];
    for (name, body, rims) in bodies {
        let body = finished("b", body, t);
        for sel in rims {
            let hits = rim_arcs_at(&body, sel.0, sel.1);
            for size in [f64::INFINITY, f64::MAX, 1.5e308, 1e200] {
                for which in ["fillet", "chamfer"] {
                    let got = std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| {
                        if which == "fillet" {
                            fillet_edges(&body, &hits, size, t).map(|_| ()).map_err(|e| format!("{:?}", e.error))
                        } else {
                            chamfer_edges(&body, &hits, size, t).map(|_| ()).map_err(|e| format!("{:?}", e.error))
                        }
                    }));
                    let s = match got {
                        Ok(Ok(())) => "Ok".to_string(),
                        Ok(Err(e)) => e.chars().take(70).collect(),
                        Err(_) => "PANIC".into(),
                    };
                    eprintln!("PROBE {name} {sel:?} {which} {size:e}: {s}");
                }
            }
        }
    }
}

#[test]
fn probe_offset_sizes() {
    use geom_core::Vec3;
    use sweep::test_support::{ball_poled_z, dome, hemisphere_on_flat_base};
    let t = Tol::witness();
    let band = geom_core::Band::linear(t).unwrap();
    let bodies = [
        ("lantern", lantern(t)),
        ("zone", sphere_zone(0.6, Revolution::Full, t)),
        ("dome", dome(1.0, t)),
        ("hemi", hemisphere_on_flat_base(1.0, t)),
        ("ball", ball_poled_z(1.0, Vec3::new(0.0, 0.0, 0.0), t)),
    ];
    for (name, body) in bodies {
        let faces: Vec<_> = body.faces().map(|(k, _)| k).collect();
        for d in [f64::INFINITY, f64::NEG_INFINITY, f64::MAX, 1e306, -1e306, f64::NAN] {
            for &face in &faces {
                let kind = body.face(face).map(|_| 0).unwrap_or(1);
                let _ = kind;
                let got = std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| {
                    let mut b = body.clone();
                    topo::replace_face_offset(&mut b, face, d, t).map(|_| ()).map_err(|e| format!("{e:?}"))
                }));
                if got.is_err() {
                    eprintln!("PROBE replace {name} {face:?} {d:e}: PANIC");
                }
            }
            // every chart together
            let mut by_key: std::collections::BTreeMap<_, Vec<_>> = Default::default();
            for &f in &faces {
                by_key.entry(format!("{:?}", body.get_face(f).unwrap().surface)).or_default().push(f);
            }
            for (_, fs) in by_key {
                let got = std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| {
                    let mut b = body.clone();
                    topo::offset_charts_together(&mut b, &[topo::ChartMove { faces: fs.clone(), distance: d }], band, t)
                        .map(|_| ())
                        .map_err(|e| format!("{e:?}"))
                }));
                if got.is_err() {
                    eprintln!("PROBE together {name} {fs:?} {d:e}: PANIC");
                }
            }
        }
    }
    eprintln!("PROBE offsets done");
}
