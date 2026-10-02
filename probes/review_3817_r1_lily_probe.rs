// Appended to demos/tour/src/lily.rs under #[cfg(test)] (uses the module's private plant/ball).
mod review_3817_r1_lily_probe {
    //! Reviewer probe (PR #3817, lane reach-dual3817-r1): wall 7's carve,
    //! its two sphere carriers checked by Monte Carlo against the set
    //! algebra near the ball, point-in-face by stereographic winding of
    //! the face's sampled loops — no props, no chart.
    use super::*;
    use core::f64::consts::PI;
    use pncad::geom::Surface;

    struct Lcg(u64);
    impl Lcg {
        fn f(&mut self) -> f64 {
            self.0 = self.0.wrapping_mul(6364136223846793005).wrapping_add(1442695040888963407);
            ((self.0 >> 11) as f64) / ((1u64 << 53) as f64)
        }
        fn unit(&mut self) -> Vec3<f64> {
            loop {
                let v = Vec3::new(2.0 * self.f() - 1.0, 2.0 * self.f() - 1.0, 2.0 * self.f() - 1.0);
                let n = v.norm();
                if n > 0.1 && n <= 1.0 {
                    return v / n;
                }
            }
        }
    }

    fn face_loops(b: &Body<f64>, f: &pncad::topo::entity::Face) -> Vec<Vec<Point3<f64>>> {
        let mut out = Vec::new();
        for lk in core::iter::once(f.outer).chain(f.rings.iter().copied()) {
            let Some(l) = b.get_loop(lk) else { continue };
            let pncad::topo::LoopBoundary::Cycle { first } = l.boundary else { continue };
            let mut pts = Vec::new();
            for he in b.loop_cycle(first).unwrap() {
                let h = b.get_half_edge(he).unwrap();
                let e = b.get_edge(h.edge).unwrap();
                let Some(cv) = b.get_curve_geom(e.curve).and_then(|g| g.certified()) else { continue };
                let (t0, t1) = cv.params();
                let fwd = e.he_plus == he;
                for i in 0..800 {
                    let s = i as f64 / 800.0;
                    let t = if fwd { t0 + (t1 - t0) * s } else { t1 - (t1 - t0) * s };
                    pts.push(cv.carrier().eval(t));
                }
            }
            if !pts.is_empty() {
                out.push(pts);
            }
        }
        out
    }

    fn in_face(c: Point3<f64>, r: f64, sigma: f64, loops: &[Vec<Point3<f64>>], q: Point3<f64>, pole: Vec3<f64>) -> bool {
        let a = if pole.x.abs() < 0.9 { Vec3::new(1.0, 0.0, 0.0) } else { Vec3::new(0.0, 1.0, 0.0) };
        let e1 = pole.cross(a);
        let e1 = e1 / e1.norm();
        let e2 = pole.cross(e1);
        let proj = |p: Point3<f64>| {
            let u = (p - c) / r;
            let d = 1.0 - u.dot(pole);
            (u.dot(e1) / d, u.dot(e2) / d)
        };
        let (qx, qy) = proj(q);
        let (mut wind, mut area) = (0.0, 0.0);
        for lp in loops {
            let pp: Vec<(f64, f64)> = lp.iter().map(|&p| proj(p)).collect();
            for i in 0..pp.len() {
                let (x0, y0) = pp[i];
                let (x1, y1) = pp[(i + 1) % pp.len()];
                area += x0 * y1 - x1 * y0;
                let mut d = (y1 - qy).atan2(x1 - qx) - (y0 - qy).atan2(x0 - qx);
                if d > PI { d -= 2.0 * PI; }
                if d < -PI { d += 2.0 * PI; }
                wind += d;
            }
        }
        let w = (wind / (2.0 * PI)).round();
        let v = -sigma * w;
        v == if -sigma * area > 0.0 { 1.0 } else { 0.0 }
    }

    #[test]
    fn r1_wall7_carve_sphere_faces_against_the_set_algebra() {
        let tol = Tol::witness();
        let pieces = plant::<f64>(tol);
        let lant = &pieces.iter().find(|p| p.name == "lily_lantern").unwrap().body;
        let (bc, br) = (Point3::new(-2.80, 0.0, 0.90), 0.16);
        let ball_body = ball::<f64>(bc, br, tol);
        let mut zone = None;
        for (_k, f) in lant.faces() {
            if let Some(&Surface::Sphere { center, radius, .. }) = lant.get_surface(f.surface) {
                let d = (center - bc).norm();
                if d <= radius + br && d + br >= radius {
                    zone = Some((center, radius));
                }
            }
        }
        let (zc, zr) = zone.unwrap();
        let mut fails = Vec::new();
        for (label, which) in [("A∖B", 0), ("A∪B", 1), ("A∩B", 2), ("B∖A", 3)] {
            let r = match which {
                0 => pncad::topo::subtract(lant, &ball_body, tol),
                1 => pncad::topo::union(lant, &ball_body, tol),
                2 => pncad::topo::intersect(lant, &ball_body, tol),
                _ => pncad::topo::subtract(&ball_body, lant, tol),
            }
            .unwrap();
            let body = &r.body().unwrap().body;
            let mut rng = Lcg(23);
            let (mut bad, mut n) = (0, 0);
            // (sphere centre, radius, other centre, other radius, is ball)
            for (c, rr, oc, or, is_ball) in [(bc, br, zc, zr, true), (zc, zr, bc, br, false)] {
                let faces: Vec<_> = body
                    .faces()
                    .filter_map(|(_, f)| match body.get_surface(f.surface) {
                        Some(&Surface::Sphere { center, radius, .. })
                            if (center - c).norm() < 1e-9 && (radius - rr).abs() < 1e-9 =>
                        {
                            Some((if f.sense { 1.0 } else { -1.0 }, face_loops(body, f)))
                        }
                        _ => None,
                    })
                    .collect();
                for _ in 0..3000 {
                    let q = c + rng.unit() * rr;
                    if !is_ball && (q - bc).norm() > 2.0 * br {
                        continue; // only the zone near the ball
                    }
                    let din = (q - oc).norm() - or;
                    if din.abs() < 2e-3 * br {
                        continue;
                    }
                    let inside_other = din < 0.0;
                    // A = lantern (zone side), B = ball
                    let expect = match (which, is_ball) {
                        (1, _) => (!inside_other).then_some(1.0),
                        (2, _) => inside_other.then_some(1.0),
                        (0, false) | (3, true) => (!inside_other).then_some(1.0),
                        (0, true) | (3, false) => inside_other.then_some(-1.0),
                        _ => unreachable!(),
                    };
                    let pole = rng.unit();
                    let hits: Vec<f64> = faces
                        .iter()
                        .filter(|(s, l)| in_face(c, rr, *s, l, q, pole))
                        .map(|(s, _)| *s)
                        .collect();
                    n += 1;
                    let ok = match expect {
                        None => hits.is_empty(),
                        Some(s) => hits.len() == 1 && hits[0] == s,
                    };
                    bad += usize::from(!ok);
                }
            }
            println!("wall-7 r1 oracle {label}: mismatches {bad}/{n}");
            if bad > 0 {
                fails.push(label);
            }
        }
        assert!(fails.is_empty(), "{fails:?}");
    }
}
