
/// Re-review honesty probe (claim 2): every witnessed edge verdict against
/// an independent offset of the edge's point at the arm.
#[cfg(test)]
#[allow(clippy::unwrap_used, clippy::expect_used, clippy::panic, dead_code)]
mod rerev6385_honesty {
    use super::*;
    use geom_core::Tol;

    fn rng(state: &mut u64) -> f64 {
        *state ^= *state << 13;
        *state ^= *state >> 7;
        *state ^= *state << 17;
        (*state >> 11) as f64 / (1u64 << 53) as f64
    }

    /// independent signed offset, positive away from material
    fn true_off(s: &geom::Surface<f64>, sense: bool, q: Point3<f64>) -> f64 {
        let d = match *s {
            geom::Surface::Sphere { center, radius, .. } => (q - center).norm() - radius,
            geom::Surface::Cylinder { origin, axis, radius, .. } => {
                let a = axis.normalize();
                let v = q - origin;
                (v - a * a.dot(v)).norm() - radius
            }
            _ => unreachable!(),
        };
        if sense { d } else { -d }
    }

    #[test]
    fn rerev6385_witness_is_honest() {
        for b in [Band::linear(Tol::witness()).unwrap(), Band::new(1e-12, 1e-11).unwrap()] {
            let mut st = 0x1234_5678_9abc_def1u64;
            let (mut wrong, mut spurious, mut leaves, mut refused, mut other) = (Vec::new(), Vec::new(), 0, 0, 0);
            for trial in 0..6000 {
                let rad = 10f64.powf(rng(&mut st) * 6.0 - 3.0);
                let a = Vec3::new(rng(&mut st) - 0.5, rng(&mut st) - 0.5, rng(&mut st) - 0.5).normalize();
                let u = a.cross(Vec3::new(1.0, 0.0, 0.0)).normalize();
                let o = Point3::new(0.3, -0.1, 0.2);
                let cyl = trial % 2 == 0;
                let sense = (trial / 2) % 2 == 0;
                let s = if cyl {
                    geom::Surface::Cylinder { origin: o, axis: a, radius: rad, u_ref: u }
                } else {
                    geom::Surface::Sphere { center: o, radius: rad, axis: a, u_ref: u }
                };
                // a point on the carrier
                let t = Vec3::new(rng(&mut st) - 0.5, rng(&mut st) - 0.5, rng(&mut st) - 0.5);
                let t = if cyl { (t - a * a.dot(t)).normalize() } else { t.normalize() };
                let p = o + t * rad + if cyl { a * (rng(&mut st) - 0.5) } else { Vec3::zero() };
                let n_out = geom_brep::implicit_outward_normal(&s, sense, p);
                let n = n_out.vec();
                let r = Vec3::new(rng(&mut st) - 0.5, rng(&mut st) - 0.5, rng(&mut st) - 0.5);
                let d = (r - n * r.dot(n)).normalize();
                let w = n.cross(d);
                let datum = FaceDatum::of_face(n_out, FaceBend::at(&s, sense, p), f64::MAX);
                let kn = datum.bend.section(d, n).unwrap();
                // a circle bending in a plane through d, curvature near the section's
                let th = rng(&mut st) * core::f64::consts::TAU;
                let toward = n * th.cos() + w * th.sin();
                let kappa = (kn.abs() / th.cos().abs().max(1e-3)) * (1.0 + (rng(&mut st) - 0.5) * 0.4);
                let kappa = kappa.max(1e-3 / rad);
                let span = 0.2 + 2.5 * rng(&mut st);
                let radius_c = 1.0 / kappa;
                let carrier = geom::Curve3::circle(p + toward * radius_c, (-toward).cross(d), radius_c, -toward);
                let chord = carrier.eval(span).distance(p);
                let extent = geom_brep::edge_extent_inf(&carrier, 0.0, span, chord);
                let tangent = carrier.deriv(0.0);
                let bend = ConicBend::of(&carrier, tangent, carrier.deriv2(0.0), span);
                let reach = Reach::Conic { extent, bend, arc: ConicArc::on(&carrier, 0.0, span) };
                let Ok(FirstOrder::Tied(tie)) = read(tangent.normalize() * extent, reach, datum, Reading::plain(b)) else {
                    continue;
                };
                let arm = datum.bend.radius().unwrap().min(extent);
                // the arc point at arc length `arm` (the circle's speed is its radius)
                let q = carrier.eval((arm / radius_c).min(span));
                let off = true_off(&s, sense, q);
                let got = tie.descend();
                match got {
                    Ok(Descent::Leaves(side)) => {
                        leaves += 1;
                        let ok = match side { Side::Above => off > 0.0, Side::Below => off < 0.0 };
                        if !ok { wrong.push(format!("t{trial} cyl {cyl} sense {sense} rad {rad:e}: {side:?} off {off:e}")); }
                    }
                    Err(ReadFault::Witness(_)) => {
                        refused += 1;
                        // a contradiction should see the arc's point at the arm on a decided side
                        if off.abs() < 2.0 * b.escalate() * rad.max(1.0) {
                            spurious.push(format!("t{trial} cyl {cyl} rad {rad:e}: refused with off {off:e}"));
                        }
                    }
                    _ => other += 1,
                }
            }
            println!("PROBE honesty esc {:e}: leaves {leaves} refused {refused} other {other} wrong {} spurious {}", b.escalate(), wrong.len(), spurious.len());
            for w in wrong.iter().take(10) { println!("PROBE wrong {w}"); }
            for w in spurious.iter().take(10) { println!("PROBE spurious {w}"); }
        }
    }
}
