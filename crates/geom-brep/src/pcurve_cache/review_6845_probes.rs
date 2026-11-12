//! Review probes for PR 6845 (the closed-form bound and the enclosing
//! patch). A child of `closed_image_rows`, so its private doors are in
//! reach. Not for merge.

use super::*;
use crate::pcurve_cache::patch::{self, PatchClass};
use geom_core::Bounds;

fn harmonic_angle_free(phase: f64, winding: i32, z: f64) -> Pcurve<f64> {
    Pcurve::Harmonic {
        u: Channel::Angle { phase, winding },
        v: Channel::Free {
            c: z,
            a: 0.0,
            b: 0.0,
            l: 0.0,
        },
    }
}

fn rim(z: f64) -> Curve3<f64> {
    Curve3::circle(
        Point3::new(0.0, 0.0, z),
        Vec3::unit_z(),
        1.0,
        Vec3::unit_x(),
    )
}

/// Probe 1: an angle read as an angle forgives a whole period and
/// nothing more.
#[test]
fn probe_angle_phase_a_period_plus_delta() {
    let b = band();
    for door in DOORS {
        for turns in [-3.0, 1.0, 5.0] {
            let on = harmonic_angle_free(TAU * turns, 1, 0.5);
            let got = (door.certify)(&unit_cylinder(), &on, &rim(0.5), (0.3, 2.9));
            println!("{} turns {turns} on: {:?}", door.name, ended(&got));
            assert!(got.is_ok(), "{got:?}");
            for delta in [1e-3, 1e3 * b.escalate(), 4.0 * b.escalate()] {
                let off = harmonic_angle_free(TAU * turns + delta, 1, 0.5);
                let got = (door.certify)(&unit_cylinder(), &off, &rim(0.5), (0.3, 2.9));
                println!(
                    "{} turns {turns} delta {delta:e}: {:?}",
                    door.name,
                    ended(&got)
                );
                assert_eq!(ended(&got), Ended::Missed, "{got:?}");
            }
        }
        // A winding that is no mirror: the reversed winding on the
        // unreflected frame. The image runs the rim backwards.
        let wrong = harmonic_angle_free(0.0, -1, 0.5);
        let got = (door.certify)(&unit_cylinder(), &wrong, &rim(0.5), (0.3, 2.9));
        println!("{} reversed winding: {:?}", door.name, ended(&got));
        assert_eq!(ended(&got), Ended::Missed, "{got:?}");
        // Winding 2 against a rim of winding 1.
        let double = harmonic_angle_free(0.0, 2, 0.5);
        let got = (door.certify)(&unit_cylinder(), &double, &rim(0.5), (0.0, 1e-3));
        println!(
            "{} double winding on a short span: {:?} {got:?}",
            door.name,
            ended(&got)
        );
        assert_ne!(ended(&got), Ended::Certified, "{got:?}");
        // A seam crossing: a span through u = 0 with a phase near τ.
        let seam = harmonic_angle_free(TAU, 1, 0.5);
        let got = (door.certify)(&unit_cylinder(), &seam, &rim(0.5), (-0.5, 0.5));
        assert!(got.is_ok(), "{got:?}");
        let seam_off = harmonic_angle_free(TAU + 4.0 * b.escalate(), 1, 0.5);
        let got = (door.certify)(&unit_cylinder(), &seam_off, &rim(0.5), (-0.5, 0.5));
        assert_eq!(ended(&got), Ended::Missed, "{got:?}");
    }
}

/// Probe 2: the sphere's involution twin, and the twin a hair off.
#[test]
fn probe_sphere_twin() {
    let b = band();
    let sphere = Surface::Sphere {
        center: Point3::origin(),
        radius: 2.0,
        axis: Vec3::unit_z(),
        u_ref: Vec3::unit_x(),
    };
    // The parallel at latitude 0.4.
    let lat: f64 = 0.4;
    let parallel = Curve3::circle(
        Point3::new(0.0, 0.0, 2.0 * lat.sin()),
        Vec3::unit_z(),
        2.0 * lat.cos(),
        Vec3::unit_x(),
    );
    let image = |du: f64, dv: f64| Pcurve::Harmonic {
        u: Channel::Angle {
            phase: PI + du,
            winding: 1,
        },
        v: Channel::Angle {
            phase: PI - lat + dv,
            winding: 0,
        },
    };
    for door in DOORS {
        let got = (door.certify)(&sphere, &image(0.0, 0.0), &parallel, (0.1, 5.0));
        println!("{} twin: {:?}", door.name, ended(&got));
        assert!(got.is_ok(), "{got:?}");
        for d in [4.0 * b.escalate(), 1e-4] {
            for (du, dv) in [(d, 0.0), (0.0, d)] {
                let got = (door.certify)(&sphere, &image(du, dv), &parallel, (0.1, 5.0));
                println!("{} twin off ({du:e},{dv:e}): {:?}", door.name, ended(&got));
                assert_eq!(ended(&got), Ended::Missed, "{got:?}");
            }
        }
    }
}

/// Probe 3: a spiric image whose copies of the carrier's scalars are
/// one ulp off. The pre-PR door decided each copy in band; this door
/// takes the two-factor arm.
#[test]
fn probe_spiric_copies_an_ulp_off() {
    let mut g = fuzz::start("review_6845_spiric_ulp");
    let mut tally = std::collections::BTreeMap::<String, usize>::new();
    for _ in 0..8 {
        let (oval, charts) = super::spiric(&mut g);
        for chart in charts {
            let span = (0.3, 2.2);
            let Ok(image) = chart_pcurve_over(&oval, span.0, span.1, &chart, band()) else {
                continue;
            };
            let Pcurve::Spiric {
                major,
                minor,
                offset,
                image: im,
            } = image
            else {
                continue;
            };
            for (what, nudged) in [
                (
                    "major",
                    Pcurve::Spiric {
                        major: major.next_up(),
                        minor,
                        offset,
                        image: im,
                    },
                ),
                (
                    "offset",
                    Pcurve::Spiric {
                        major,
                        minor,
                        offset: offset.next_up(),
                        image: im,
                    },
                ),
            ] {
                for door in DOORS {
                    let exact = (door.certify)(&chart, &image, &oval, span);
                    let got = (door.certify)(&chart, &nudged, &oval, span);
                    let key = format!(
                        "{} {what} exact={:?} nudged={:?} {}",
                        door.name,
                        ended(&exact),
                        ended(&got),
                        match &got {
                            Err(e) => format!("{e}").chars().take(90).collect::<String>(),
                            Ok(()) => String::new(),
                        }
                    );
                    *tally.entry(key).or_default() += 1;
                }
            }
        }
    }
    for (k, n) in &tally {
        println!("{n:4} {k}");
    }
}

/// Probe 4: the enclosing patch contains its chart at every point of the
/// window, including a cone's apex, a sphere's poles, a torus's inner
/// equator, a spindle, and wide `u` windows.
fn patch_holds(chart: &Surface<f64>, u: (f64, f64), v: (f64, f64), b: geom_core::Band) -> usize {
    let lifted = chart.map_scalar(Interval::from_certified);
    let (ku, kv, _w, coords) = match patch::patch(PatchClass::Enclosed, &lifted, (u, v), b) {
        Ok(p) => p,
        Err(e) => {
            println!("refused {e:?} over {u:?} {v:?}");
            return 0;
        }
    };
    let nv = kv.control_count();
    let mut g = fuzz::start("review_6845_patch");
    let mut worst = 0.0f64;
    let mut failures = 0;
    let n = 400;
    for i in 0..=n {
        // Grid plus random points, inside the image's window.
        let (uu, vv) = if i % 2 == 0 {
            (g.range(u.0, u.1), g.range(v.0, v.1))
        } else {
            let f = f64::from(i) / f64::from(n);
            (u.0 + (u.1 - u.0) * f, v.1 - (v.1 - v.0) * f)
        };
        let su = ku.span_at(uu).unwrap();
        let sv = kv.span_at(vv).unwrap();
        let bu = geom_core::spline::basis::basis_funs(su, Interval::from_bounds(uu, uu));
        let bv = geom_core::spline::basis::basis_funs(sv, Interval::from_bounds(vv, vv));
        let (fu, fv) = (su.first_control(), sv.first_control());
        let mut at = [Interval::from_bounds(0.0, 0.0); 3];
        for (a, &nu) in bu.iter().enumerate() {
            for (c, &nvv) in bv.iter().enumerate() {
                let idx = (fu + a) * nv + (fv + c);
                for (lane, slot) in at.iter_mut().enumerate() {
                    *slot = *slot + nu * nvv * coords[lane][idx];
                }
            }
        }
        let truth = lifted.eval(Interval::from_bounds(uu, uu), Interval::from_bounds(vv, vv));
        for (lane, t) in [truth.x, truth.y, truth.z].into_iter().enumerate() {
            let p = at[lane];
            let out = (p.lo() - t.lo()).max(t.hi() - p.hi());
            if out > 0.0 {
                failures += 1;
                worst = worst.max(out);
            }
        }
    }
    if failures > 0 {
        println!(
            "{:?} over {u:?} {v:?}: {failures} coordinates outside, worst {worst:e}",
            chart.kind()
        );
    }
    failures
}

#[test]
fn probe_enclosing_patch_holds_its_chart() {
    let zero_of = |eps: f64| geom_core::Band::linear_at(geom_core::Tol::witness(), eps).unwrap();
    let cyl = unit_cylinder();
    let cone = Surface::Cone {
        apex: Point3::new(0.1, 0.2, 0.3),
        axis: Vec3::unit_z(),
        half_angle: 0.6,
        u_ref: Vec3::unit_x(),
    };
    let sphere = Surface::Sphere {
        center: Point3::new(1.0, -2.0, 0.5),
        radius: 3.0,
        axis: Vec3::unit_y(),
        u_ref: Vec3::unit_z(),
    };
    let torus = Surface::Torus {
        center: Point3::origin(),
        axis: Vec3::unit_z(),
        major_radius: 2.0,
        minor_radius: 0.5,
        u_ref: Vec3::unit_x(),
    };
    let spindle = Surface::Spindle {
        center: Point3::origin(),
        axis: Vec3::unit_z(),
        major_radius: 0.5,
        minor_radius: 1.0,
        u_ref: Vec3::unit_x(),
    };
    let big_torus = Surface::Torus {
        center: Point3::new(8.0, 3.0, -2.0),
        axis: Vec3::unit_z(),
        major_radius: 9.0,
        minor_radius: 1.0,
        u_ref: Vec3::unit_x(),
    };
    let cases: Vec<(&str, &Surface<f64>, (f64, f64), (f64, f64))> = vec![
        ("cylinder wide", &cyl, (-20.0, 20.0), (0.0, 1.0)),
        ("cone across apex", &cone, (0.0, TAU), (-1.0, 1.0)),
        ("cone near apex", &cone, (0.0, 3.0), (-1e-3, 1e-3)),
        ("cone wide", &cone, (-9.0, 9.0), (2.0, 5.0)),
        ("sphere pole", &sphere, (0.0, TAU), (1.4, PI / 2.0)),
        (
            "sphere both poles",
            &sphere,
            (0.0, 1.0),
            (-PI / 2.0, PI / 2.0),
        ),
        (
            "torus inner equator",
            &torus,
            (0.0, 4.0),
            (PI - 0.3, PI + 0.3),
        ),
        ("torus whole", &torus, (0.0, TAU), (0.0, TAU)),
        ("spindle", &spindle, (0.0, 2.0), (2.0, 4.0)),
        ("big torus", &big_torus, (0.0, 2.0), (-1.0, 3.0)),
    ];
    let mut failures = 0;
    for eps in [1e-6, 1e-9, 1e-12] {
        for (what, s, u, v) in &cases {
            let f = patch_holds(s, *u, *v, zero_of(eps));
            println!("eps {eps:e} {what}: {f} outside");
            failures += f;
        }
    }
    assert_eq!(failures, 0);
}

/// Probe 5: the structural refusals' endings. The PR says a winding past
/// 64 and a closed form on a chart that holds none end in their own
/// defect ending, and no closed-form route ends "not yet".
#[test]
fn probe_structural_endings() {
    let wound = harmonic_angle_free(0.0, 65, 0.5);
    let helix_free = Pcurve::Harmonic {
        u: Channel::Free {
            c: 0.0,
            a: 0.0,
            b: 0.0,
            l: 1.0,
        },
        v: Channel::Free {
            c: 0.5,
            a: 0.0,
            b: 0.0,
            l: 0.0,
        },
    };
    let focal = Pcurve::FocalSection(FocalImage {
        u0: 0.0,
        t0: 0.0,
        v0: 0.5,
        va: 0.0,
        vb: 0.0,
        vl: 0.0,
        beta: 0.0,
        sense: 1.0,
    });
    for door in DOORS {
        for (what, image) in [
            ("winding 65", &wound),
            ("free u on a cylinder", &helix_free),
            ("focal on a cylinder", &focal),
        ] {
            let got = (door.certify)(&unit_cylinder(), image, &rim(0.5), (0.3, 2.9));
            let text = match &got {
                Err(e) => format!("{e:?} || {:?}", e.ending(ReadAt::Run(Reading::Build))),
                Ok(()) => "Ok".into(),
            };
            println!("{} {what}: {text}", door.name);
        }
    }
}

/// Probe 6: cost spot-check, one cylinder rim circle and one cylinder
/// seam line, stated by their harmonic images (run in release).
#[test]
#[ignore = "timing probe, run in release by hand"]
fn probe_cost_cylinder_edges() {
    let rim_image = harmonic_angle_free(0.0, 1, 0.5);
    let seam_image = Pcurve::Harmonic {
        u: Channel::Angle {
            phase: 0.0,
            winding: 0,
        },
        v: Channel::Free {
            c: 0.0,
            a: 0.0,
            b: 0.0,
            l: 1.0,
        },
    };
    let seam = Curve3::line(Point3::new(1.0, 0.0, 0.0), Vec3::unit_z());
    for door in DOORS {
        let n = if door.name == "f64" { 3000 } else { 300 };
        for (what, image, carrier, span) in [
            ("rim circle", &rim_image, rim(0.5), (0.0, TAU)),
            ("seam line", &seam_image, seam.clone(), (0.0, 1.0)),
        ] {
            assert!((door.certify)(&unit_cylinder(), image, &carrier, span).is_ok());
            let start = std::time::Instant::now();
            for _ in 0..n {
                std::hint::black_box((door.certify)(&unit_cylinder(), image, &carrier, span)).ok();
            }
            println!(
                "{} {what}: {:.2} µs per certify",
                door.name,
                start.elapsed().as_secs_f64() * 1e6 / f64::from(n)
            );
        }
    }
}
