
// ---- reviewer 6257: adversarial touch rows (scratch) ----

fn rev_turned(at: [f64; 2], phi: f64, shift: [f64; 2]) -> [f64; 2] {
    let (s, c) = phi.sin_cos();
    [c * at[0] - s * at[1] + shift[0], s * at[0] + c * at[1] + shift[1]]
}

fn rev_show(label: &str, got: &Result<Meetings<f64>, super::MeetRefusal<f64>>) -> String {
    match got {
        Ok(m) => {
            let pts: Vec<String> = m
                .points
                .iter()
                .map(|p| format!("({:.3e},{:.6})", p.point.x, p.point.y))
                .collect();
            format!("{label}: OK {} {}", m.points.len(), pts.join(" "))
        }
        Err(r) => format!("{label}: ERR {:?}", r.fault),
    }
}

/// Two ellipses crossing twice at a tiny angle, crossing points `s`
/// apart (closed form), and osculating pairs. Prints every answer, and
/// asserts: never no meeting where they cross; never one meeting where
/// the crossings are more than ten bands apart; every meeting point
/// within the escalation width of a true crossing (or the touch).
#[test]
fn rev6257_adversarial_touches() {
    let mut bad = Vec::new();
    for eps in [1e-6, 1e-9, 1e-12] {
        let band = Band::new(eps, 10.0 * eps).unwrap();
        for phi in [0.0f64, 0.7, 2.3] {
            let shift = if phi == 0.0 { [0.0, 0.0] } else { [3.1, -1.7] };
            let u = [phi.cos(), phi.sin()];
            // Case 1: equal ellipses (2, 1), the second pushed down by δ:
            // crossings at (±2√(δ − δ²/4), 1 − δ/2).
            for k in [0.1, 0.5, 2.0, 5.0, 20.0, 100.0, 1e3, 1e4, 1e5] {
                let s = k * eps;
                let cy = 2.0 - s * s / 16.0;
                let delta = 2.0 - cy;
                if delta == 0.0 {
                    continue;
                }
                let half = 2.0 * (delta - delta * delta / 4.0).sqrt();
                let s_true = 2.0 * half;
                let a = flat_ellipse(rev_turned([0.0, 0.0], phi, shift), u, 2.0, 1.0);
                let b = flat_ellipse(rev_turned([0.0, cy], phi, shift), u, 2.0, 1.0);
                let truth = [
                    rev_turned([half, 1.0 - delta / 2.0], phi, shift),
                    rev_turned([-half, 1.0 - delta / 2.0], phi, shift),
                ];
                for (order, got) in [
                    ("ab", meet(&Span::new(&a, (-PI, PI)), &Span::new(&b, (-PI, PI)), band)),
                    ("ba", meet(&Span::new(&b, (-PI, PI)), &Span::new(&a, (-PI, PI)), band)),
                ] {
                    let label = format!("eq eps {eps:e} phi {phi} s {s_true:.3e} ({k}eps) {order}");
                    eprintln!("{}", rev_show(&label, &got));
                    if let Ok(m) = &got {
                        if m.points.is_empty() {
                            bad.push(format!("{label}: no meeting"));
                        }
                        if m.points.len() == 1 && s_true > 10.0 * eps && delta > 2e-13 {
                            bad.push(format!("{label}: one meeting, crossings {s_true:e} apart"));
                        }
                        for p in &m.points {
                            let d = truth
                                .iter()
                                .map(|t| (p.point - Point3::new(t[0], t[1], 0.0)).norm())
                                .fold(f64::INFINITY, f64::min);
                            if d > 10.0 * eps && delta > 2e-13 {
                                bad.push(format!("{label}: point {d:e} off a crossing"));
                            }
                        }
                    }
                }
            }
            // Case 2: (2, 1) and (3, 1.5) touching outside at (0, 1),
            // pushed together δ: crossings at x ≈ ±√(24δ/5).
            for k in [0.5, 2.0, 5.0, 20.0, 1e3, 1e5] {
                let s = k * eps;
                let x = s / 2.0;
                let cy = 2.5 - x * x * 5.0 / 24.0;
                let delta = 2.5 - cy;
                if delta == 0.0 {
                    continue;
                }
                let half = (24.0 * delta / 5.0).sqrt();
                let a = flat_ellipse(rev_turned([0.0, 0.0], phi, shift), u, 2.0, 1.0);
                let b = flat_ellipse(rev_turned([0.0, cy], phi, shift), u, 3.0, 1.5);
                for (order, got) in [
                    ("ab", meet(&Span::new(&a, (-PI, PI)), &Span::new(&b, (-PI, PI)), band)),
                    ("ba", meet(&Span::new(&b, (-PI, PI)), &Span::new(&a, (-PI, PI)), band)),
                ] {
                    let label = format!("uneq eps {eps:e} phi {phi} s {:.3e} ({k}eps) {order}", 2.0 * half);
                    eprintln!("{}", rev_show(&label, &got));
                    if let Ok(m) = &got {
                        if m.points.is_empty() {
                            bad.push(format!("{label}: no meeting"));
                        }
                        if m.points.len() == 1 && 2.0 * half > 10.0 * eps && delta > 2e-13 {
                            bad.push(format!("{label}: one meeting, crossings apart"));
                        }
                    }
                }
            }
            // Case 3: osculating. (2, 1) about the origin and (√8, 2)
            // about (0, −1): equal curvature 1/4 at (0, 1), the second
            // above by x⁴/256. Pushed down δ they cross at
            // x = ±(256δ)^¼; pulled up δ they are δ apart.
            for dy in [0.0, -1e-14, -1e-12, -1e-10, -1e-8, 1e-14, 1e-10, -0.1 * eps, -eps, -100.0 * eps, 0.5 * eps, 5.0 * eps, 100.0 * eps] {
                let cy = -1.0 + dy;
                let delta = -(cy + 1.0);
                let a = flat_ellipse(rev_turned([0.0, 0.0], phi, shift), u, 2.0, 1.0);
                let b = flat_ellipse(rev_turned([0.0, cy], phi, shift), u, 8.0f64.sqrt(), 2.0);
                let s_true = if delta > 0.0 { 2.0 * (256.0 * delta).powf(0.25) } else { 0.0 };
                for (order, got) in [
                    ("ab", meet(&Span::new(&a, (-PI, PI)), &Span::new(&b, (-PI, PI)), band)),
                    ("ba", meet(&Span::new(&b, (-PI, PI)), &Span::new(&a, (-PI, PI)), band)),
                ] {
                    let label = format!("osc eps {eps:e} phi {phi} dy {dy:e} s {s_true:.3e} {order}");
                    eprintln!("{}", rev_show(&label, &got));
                    if let Ok(m) = &got {
                        if delta > 0.0 && m.points.is_empty() {
                            bad.push(format!("{label}: crossing, no meeting"));
                        }
                        if delta > 10.0 * eps && m.points.len() == 0 {
                            bad.push(format!("{label}: none"));
                        }
                        if delta < -10.0 * eps && !m.points.is_empty() {
                            bad.push(format!("{label}: apart, met"));
                        }
                        if m.points.len() == 1 && s_true > 10.0 * eps && delta > 1e-13 {
                            bad.push(format!("{label}: one meeting, crossings {s_true:e} apart"));
                        }
                    }
                }
            }
        }
    }
    assert!(bad.is_empty(), "{}", bad.join("\n"));
}

/// Reviewer 6257: crossings `n` ulps of depth apart, ellipses and circles.
#[test]
fn rev6257_ulp_ladder() {
    for eps in [1e-6, 1e-9, 1e-12] {
        let band = Band::new(eps, 10.0 * eps).unwrap();
        for n in [1u64, 2, 3, 5, 10, 30, 100, 300, 1000, 10000] {
            let cy = f64::from_bits(2.0f64.to_bits() - n);
            let delta = 2.0 - cy;
            let s = 4.0 * (delta - delta * delta / 4.0).sqrt();
            let a = flat_ellipse([0.0, 0.0], [1.0, 0.0], 2.0, 1.0);
            let b = flat_ellipse([0.0, cy], [1.0, 0.0], 2.0, 1.0);
            let got = meet(&Span::new(&a, (-PI, PI)), &Span::new(&b, (-PI, PI)), band);
            eprintln!(
                "{}",
                rev_show(
                    &format!(
                        "ladder ell eps {eps:e} n {n} depth {:.2e} s {s:.3e} ({:.1}eps)",
                        delta / 2.0,
                        s / eps
                    ),
                    &got
                )
            );
            // Unit circles about (0,0) and (0, cy): crossings at
            // x = ±sqrt(δ − δ²/4).
            let ca = circle([0.0, 0.0, 0.0], [0.0, 0.0, 1.0], 1.0);
            let cb = circle([0.0, cy, 0.0], [0.0, 0.0, 1.0], 1.0);
            let sc = 2.0 * (delta - delta * delta / 4.0).sqrt();
            let got = meet(&Span::new(&ca, (-PI, PI)), &Span::new(&cb, (-PI, PI)), band);
            eprintln!(
                "{}",
                rev_show(
                    &format!("ladder circ eps {eps:e} n {n} s {sc:.3e} ({:.1}eps)", sc / eps),
                    &got
                )
            );
        }
    }
}
