
/// Re-review probes for PR 6385's fix pass (reviewer lane; not part of the PR).
#[cfg(test)]
#[allow(clippy::unwrap_used, clippy::expect_used, clippy::panic, dead_code)]
mod rerev6385 {
    use super::*;
    use geom_core::Tol;

    fn dflt() -> Band {
        Band::linear(Tol::witness()).unwrap()
    }
    fn tight() -> Band {
        Band::new(1e-12, 1e-11).unwrap()
    }

    fn rng(state: &mut u64) -> f64 {
        *state ^= *state << 13;
        *state ^= *state >> 7;
        *state ^= *state << 17;
        (*state >> 11) as f64 / (1u64 << 53) as f64
    }

    fn foot(s: &geom::Surface<f64>, mut q: Point3<f64>) -> Point3<f64> {
        for _ in 0..80 {
            let f = geom_brep::implicit_residual(s, q);
            let g = geom_brep::implicit_gradient(s, q);
            q = q - g * (f / g.norm_squared());
        }
        q
    }

    /// The four kinds, scaled by `k` about `o + shift`.
    fn surfaces(k: f64, shift: Vec3<f64>) -> Vec<(&'static str, geom::Surface<f64>)> {
        let o = Point3::new(0.1 * k + shift.x, -0.2 * k + shift.y, 0.3 * k + shift.z);
        let a = Vec3::new(0.2, 0.3, 1.0).normalize();
        let u = a.cross(Vec3::new(1.0, 0.0, 0.0)).normalize();
        vec![
            ("cylinder", geom::Surface::Cylinder { origin: o, axis: a, radius: 0.7 * k, u_ref: u }),
            ("cone", geom::Surface::Cone { apex: o, axis: a, half_angle: 0.5, u_ref: u }),
            ("sphere", geom::Surface::Sphere { center: o, radius: 0.9 * k, axis: a, u_ref: u }),
            (
                "torus",
                geom::Surface::Torus {
                    center: o,
                    axis: a,
                    major_radius: 2.0 * k,
                    minor_radius: 0.6 * k,
                    u_ref: u,
                },
            ),
        ]
    }

    fn circle(p: Point3<f64>, d: Vec3<f64>, toward: Vec3<f64>, kappa: f64, span: f64) -> (geom::Curve3<f64>, Vec3<f64>, Reach<f64>) {
        let (radius, u_ref) = (1.0 / kappa, -toward);
        let carrier = geom::Curve3::circle(p + toward * radius, u_ref.cross(d), radius, u_ref);
        let chord = carrier.eval(span).distance(p);
        let extent = geom_brep::edge_extent_inf(&carrier, 0.0, span, chord);
        let tangent = carrier.deriv(0.0);
        let bend = ConicBend::of(&carrier, tangent, carrier.deriv2(0.0), span);
        let reach = mk_conic(extent, bend, &carrier, span);
        (carrier, tangent.normalize() * extent, reach)
    }

    fn cat(r: &Result<Descent, ReadFault<f64>>) -> String {
        match r {
            Ok(d) => format!("{d:?}"),
            Err(e) => {
                let s = format!("{e:?}");
                format!("Err({})", s.split('(').next().unwrap_or("?"))
            }
        }
    }

    fn tie_of(d: Vec3<f64>, reach: Reach<f64>, face: FaceDatum<f64>, b: Band) -> Option<Tie<f64>> {
        match read(d, reach, face, Reading::plain(b)) {
            Ok(FirstOrder::Tied(t)) => Some(t),
            _ => None,
        }
    }

    /// One pose's verdict per (kind, surface, trial), at a scale/shift/band.
    fn sweep(k: f64, shift: Vec3<f64>, b: Band, edges_on_round: bool) -> Vec<(String, String)> {
        let mut out = Vec::new();
        let mut st = 0x2545_f491_4f6c_dd1du64;
        for (name, s) in surfaces(k, shift) {
            let round = matches!(name, "sphere" | "cylinder");
            for trial in 0..300 {
                let seed = Point3::new(
                    (rng(&mut st) * 2.0 - 1.0) * 1.5,
                    (rng(&mut st) * 2.0 - 1.0) * 1.5,
                    (rng(&mut st) * 2.0 - 1.0) * 1.5,
                );
                let seed = if name == "torus" { seed + Vec3::new(2.0, 0.0, 0.0) } else { seed };
                let seed = Point3::new(seed.x * k + shift.x, seed.y * k + shift.y, seed.z * k + shift.z);
                let r = Vec3::new(rng(&mut st) - 0.5, rng(&mut st) - 0.5, rng(&mut st) - 0.5);
                let (r1, r2, r3, r4) = (rng(&mut st), rng(&mut st), rng(&mut st), rng(&mut st));
                let sense = trial % 2 == 0;
                let p = foot(&s, seed);
                let n_out = geom_brep::implicit_outward_normal(&s, sense, p);
                let n = n_out.vec();
                if !n.x.is_finite() || geom_brep::implicit_residual(&s, p).abs() > 1e-9 * k.max(1.0) {
                    continue;
                }
                let d = (r - n * r.dot(n)).normalize();
                let w = n.cross(d);
                let datum = FaceDatum::of_face(n_out, fb(&s, sense, p), f64::MAX);
                let kn = datum.bend.section(d, n).unwrap_or(0.0);
                let key = |kind: &str| format!("{name} {kind} t{trial}");
                // a line
                if !round || edges_on_round {
                    let l = (0.01 + 2.0 * r1) * k;
                    let reach = Reach::Chord { base: p, far: p + d * l };
                    if let Some(t) = tie_of(d * l, reach, datum, b) {
                        out.push((key("line"), cat(&t.descend())));
                    }
                    // a circle bending in the plane of (d, toward), near the normal section's curvature
                    let th = r2 * core::f64::consts::TAU;
                    let toward = n * th.cos() + w * th.sin();
                    let kappa = (kn.abs() * (0.5 + 1.5 * r3) + 0.05 / k).max(0.01 / k);
                    let span = (0.2 + 2.0 * r4).min(3.0);
                    let (_, dir, reach) = circle(p, d, toward, kappa, span);
                    if let Some(t) = tie_of(dir, reach, datum, b) {
                        out.push((key("circle"), cat(&t.descend())));
                    }
                    // a circle bending in the normal plane, curvature close to the normal section's
                    let toward = if kn > 0.0 { n } else { -n };
                    let kappa = (kn.abs() * (1.0 + (r3 - 0.5) * 1e-3)).max(1e-3 / k);
                    let (_, dir, reach) = circle(p, d, toward, kappa, 1.5);
                    if let Some(t) = tie_of(dir, reach, datum, b) {
                        out.push((key("circle-near"), cat(&t.descend())));
                    }
                }
                // a bisector of this face against a plane holding it, tilted by psi
                for (j, cpsi) in [1e-9f64, 1e-7, 1e-5, 1e-3, 0.1, 0.7].into_iter().enumerate() {
                    let sgn = if r1 > 0.5 { 1.0 } else { -1.0 };
                    let spsi = sgn * (1.0 - cpsi * cpsi).sqrt();
                    let m = (n * cpsi + w * spsi).normalize();
                    let plane = FaceDatum::of_plane(geom_brep::ReferenceNormal::of_face_outward(
                        OutwardNormal::from_chart(m, true),
                    ));
                    let arm = (0.01 + r2) * k;
                    let bis = Reach::bisector(arm, d, n, fb(&s, sense, p));
                    if let Some(t) = tie_of(d, bis, plane, b) {
                        out.push((key(&format!("bisector{j}")), cat(&t.descend())));
                    }
                }
            }
        }
        out
    }

    fn dump(tag: &str, v: &[(String, String)]) {
        for (k, c) in v {
            println!("PROBE {tag} | {k} | {c}");
        }
    }

    /// Claim 1 + main-vs-head comparison: every kind, unit scale, both bands.
    #[test]
    fn rerev6385_dump_verdicts() {
        for (bn, b) in [("dflt", dflt()), ("tight", tight())] {
            dump(&format!("unit-{bn}"), &sweep(1.0, Vec3::zero(), b, true));
        }
    }

    /// Claim 3: translations and scales.
    #[test]
    fn rerev6385_dump_translated() {
        for (bn, b) in [("dflt", dflt()), ("tight", tight())] {
            for k in [1e-3, 1.0, 1e3] {
                for sh in [0.0, 10.0, 100.0, 1000.0, 1e4] {
                    let shift = Vec3::new(sh, -0.6 * sh, 0.8 * sh);
                    dump(&format!("tr-{bn}-k{k:e}-s{sh:e}"), &sweep(k, shift, b, true));
                }
            }
        }
    }

    /// The first review's cylinder bisector poses.
    fn bisector_pose(cos_psi: f64, arm: f64) -> Result<Descent, ReadFault<f64>> {
        let c = core::f64::consts::FRAC_1_SQRT_2;
        let p = Point3::new(0.0, 0.0, 1.0);
        let (z, d) = (Vec3::new(0.0, 0.0, 1.0), Vec3::new(c, c, 0.0));
        let w = z.cross(d);
        let cyl = geom::Surface::Cylinder {
            origin: Point3::origin(),
            axis: Vec3::new(1.0, 0.0, 0.0),
            radius: 1.0,
            u_ref: z,
        };
        let sin_psi = -(1.0 - cos_psi * cos_psi).sqrt();
        let m = (z * cos_psi + w * sin_psi).normalize();
        let face = FaceDatum::of_plane(geom_brep::ReferenceNormal::of_face_outward(
            OutwardNormal::from_chart(m, true),
        ));
        let bis = Reach::bisector(arm, d, z, fb(&cyl, true, p));
        tie_of(d, bis, face, dflt()).expect("ties").descend()
    }

    #[test]
    fn rerev6385_first_review_poses() {
        println!("PROBE pose | gap cos=5e-7 arm=0.1 | {}", cat(&bisector_pose(5e-7, 0.1)));
        println!("PROBE pose | decided cos=0.1 arm=1 | {}", cat(&bisector_pose(0.1, 1.0)));
        let mid = 0.5 * (dflt().zero() + dflt().escalate()) / 0.0025;
        println!("PROBE pose | head-test gap cos={mid:e} arm=0.1 | {}", cat(&bisector_pose(mid, 0.1)));
        println!("PROBE pose | band zero={:e} esc={:e}", dflt().zero(), dflt().escalate());
    }

    fn fb(s: &geom::Surface<f64>, _sense: bool, p: Point3<f64>) -> FaceBend<f64> { FaceBend::at(s, p) }
    fn mk_conic(extent: f64, bend: ConicBend<f64>, _c: &geom::Curve3<f64>, _span: f64) -> Reach<f64> {
        Reach::Conic { extent, bend }
    }
}
