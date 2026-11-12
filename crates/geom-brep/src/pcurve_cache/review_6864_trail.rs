//! Review probe (PR 6864), compiles on main and the branch alike:
//! check 4's decision trail (k_stats verdicts and escalations, and the
//! Sym revalue recorder) on a bulging span over a ladder of bands.
//! Run with --nocapture and diff main's output against the branch's.

use super::*;

fn spline(controls: &[Point3<f64>]) -> Curve3<f64> {
    let spans = controls.len() - 3;
    let inner = (1..spans).map(|i| {
        #[allow(clippy::cast_precision_loss)]
        let t = i as f64 / spans as f64;
        t
    });
    let knots: Vec<f64> = [0.0; 4].into_iter().chain(inner).chain([1.0; 4]).collect();
    let knots = KnotVector::clamped(knots, 3).unwrap();
    Curve3::Nurbs(Arc::new(
        NurbsCurve3::new(knots, controls.to_vec(), vec![1.0; controls.len()]).unwrap(),
    ))
}

fn cylinder() -> Surface<f64> {
    Surface::Cylinder {
        origin: Point3::new(0.0, 0.0, 0.0),
        axis: Vec3::new(0.0, 0.0, 1.0),
        radius: 1.0,
        u_ref: Vec3::new(1.0, 0.0, 0.0),
    }
}

/// The PR's `bulge` fixture, verbatim.
fn bulge(delta: f64) -> Curve3<f64> {
    let arc = core::f64::consts::TAU / 10.0;
    let k = 4.0 / 3.0 * (arc / 4.0).tan();
    let on = |a: f64| Point3::new(a.cos(), a.sin(), 0.0);
    let handle = |a: f64, sense: f64, push: f64| {
        let r = (1.0 + k * k).sqrt() + push;
        let turn = a + sense * k.atan();
        Point3::new(r * turn.cos(), r * turn.sin(), 0.0)
    };
    spline(&[
        on(0.0),
        handle(0.0, 1.0, delta),
        handle(arc, -1.0, 0.0),
        on(arc),
    ])
}

fn kind(r: &Result<f64, PcurveCertifyError>) -> String {
    match r {
        Ok(_) => "Ok".into(),
        Err(e) => {
            let s = format!("{e:?}");
            s.split([' ', '{', '(']).next().unwrap_or("").to_string()
        }
    }
}

#[test]
fn review_6864_trail_ladder() {
    use geom_core::Bounds;
    let (carrier, surface) = (bulge(1e-4), cylinder());
    for i in 0..60 {
        let zero = 10f64.powf(-6.0 + f64::from(i) * 0.05);
        let band = Band::new(zero, 10.0 * zero).unwrap();
        // Interval: the k_stats trail.
        let at = geom_core::Interval::from_f64;
        let (c, s) = (carrier.map_scalar(at), surface.map_scalar(at));
        let Ok(image) = projected::project(&c, None, &s, band) else {
            println!("TRAIL {i:02} zero={zero:.3e} project refused");
            continue;
        };
        let (read, rec) = geom_core::k_stats::detached(|| {
            projected::run_projected_checks(
                &Pcurve::Projected(Box::new(image.clone())),
                at(0.0),
                at(1.0),
                &c,
                &s,
                band,
                Some(crate::FittedLane::certified()),
            )
            .map(|cert| cert.limbs().1.hi())
        });
        let r = rec.recorded();
        let env: Vec<String> = r
            .verdicts
            .iter()
            .map(|v| format!("{}:{:?}", v.predicate, v.sign))
            .collect();
        let esc: Vec<&str> = r.escalations.iter().map(|e| e.predicate()).collect();
        // Sym<f64>: the revalue recorder.
        let sym = {
            use geom_core::sym::revalue::record;
            use geom_core::sym::with_session;
            use geom_core::{Sym, SymBudget};
            let budget = SymBudget {
                max_terms: 64,
                max_degree: 8,
            };
            let ((out, decisions), _) = with_session(budget, || {
                let at = Sym::<f64>::from_f64;
                let (c, s) = (carrier.map_scalar(at), surface.map_scalar(at));
                record(|| {
                    projected::project(&c, None, &s, band).map(|image| {
                        projected::run_projected_checks(
                            &Pcurve::Projected(Box::new(image)),
                            at(0.0),
                            at(1.0),
                            &c,
                            &s,
                            band,
                            Some(crate::FittedLane::certified()),
                        )
                        .is_ok()
                    })
                })
            });
            let env = decisions
                .iter()
                .filter(|d| d.predicate == "pcurve_envelope")
                .count();
            format!(
                "sym ok={:?} decisions={} pcurve_envelope={env}",
                out.ok(),
                decisions.len()
            )
        };
        println!(
            "TRAIL {i:02} zero={zero:.3e} {} verdicts={} escalations={} | {sym} | {}",
            kind(&read),
            r.verdicts.len(),
            esc.len(),
            env.join(","),
        );
    }
}
