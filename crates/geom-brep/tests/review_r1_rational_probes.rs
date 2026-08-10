//! R1 adversarial review probes for PR #309 (M8-3 half 2): the
//! rational patch-flux enclosure. Every probe pits the delivered
//! enclosure against an INDEPENDENT dense oracle (plain-f64 Cox-de
//! Boor evaluation + composite Gauss-Legendre), and where a closed
//! form exists, against that too. The contract under test: the true
//! flux/area lie INSIDE the returned brackets, or the call refuses
//! typed. A returned bracket that EXCLUDES the truth is the failure.

use geom_brep::props::PropsError;
use geom_brep::props::quad::nurbs_patch_face;
use geom_core::spline::KnotVector;
use geom_core::{Band, RingInterval, Tolerance};

fn band() -> Band {
    Band::linear().unwrap()
}

fn p(x: f64, y: f64, z: f64) -> [RingInterval; 3] {
    [
        RingInterval::point(x),
        RingInterval::point(y),
        RingInterval::point(z),
    ]
}

// ---------------------------------------------------------------
// The independent oracle: plain-f64 B-spline basis + derivatives
// (Cox-de Boor, no kernel spline code), rational surface point and
// partials through the quotient rule, composite Gauss-Legendre.
// ---------------------------------------------------------------

/// All basis values N_{i,p}(t) for a clamped knot vector.
fn basis(knots: &[f64], degree: usize, ncp: usize, t: f64) -> Vec<f64> {
    let n = knots.len() - 1;
    let mut nn = vec![0.0f64; n]; // degree-0
    let d1 = knots[n - degree];
    for i in 0..n {
        let inside = if t >= d1 {
            // clamp into the last nonzero span
            knots[i] < knots[i + 1] && t >= knots[i] && t <= knots[i + 1]
        } else {
            knots[i] <= t && t < knots[i + 1]
        };
        if inside {
            nn[i] = 1.0;
        }
    }
    // at t == d1 several degenerate spans may have matched; keep only
    // the last nonzero span
    if t >= d1 {
        let mut last = None;
        for i in 0..n {
            if nn[i] == 1.0 {
                last = Some(i);
            }
        }
        for i in 0..n {
            nn[i] = 0.0;
        }
        if let Some(i) = last {
            nn[i] = 1.0;
        }
    }
    for pdeg in 1..=degree {
        let mut out = vec![0.0f64; n];
        for i in 0..(n - pdeg) {
            let da = knots[i + pdeg] - knots[i];
            let db = knots[i + pdeg + 1] - knots[i + 1];
            let a = if da > 0.0 {
                (t - knots[i]) / da * nn[i]
            } else {
                0.0
            };
            let b = if db > 0.0 {
                (knots[i + pdeg + 1] - t) / db * nn[i + 1]
            } else {
                0.0
            };
            out[i] = a + b;
        }
        nn = out;
    }
    nn.truncate(ncp);
    nn
}

/// Basis derivatives N'_{i,p}(t).
fn dbasis(knots: &[f64], degree: usize, ncp: usize, t: f64) -> Vec<f64> {
    let n = knots.len() - 1;
    // degree-(p-1) values over the full index range
    let mut low = basis(knots, degree - 1, n - (degree - 1), t);
    low.resize(n, 0.0);
    let mut out = vec![0.0f64; ncp];
    let pf = degree as f64;
    for (i, o) in out.iter_mut().enumerate() {
        let da = knots[i + degree] - knots[i];
        let db = knots[i + degree + 1] - knots[i + 1];
        let a = if da > 0.0 { pf / da * low[i] } else { 0.0 };
        let b = if db > 0.0 { pf / db * low[i + 1] } else { 0.0 };
        *o = a - b;
    }
    out
}

struct Patch {
    ku: Vec<f64>,
    kv: Vec<f64>,
    du: usize,
    dv: usize,
    nu: usize,
    nv: usize,
    /// homogeneous control: (w*x, w*y, w*z, w), row-major [i*nv + j]
    cp: Vec<[f64; 4]>,
}

impl Patch {
    /// (S, S_u, S_v) by the quotient rule on the homogeneous sums.
    fn eval(&self, u: f64, v: f64) -> ([f64; 3], [f64; 3], [f64; 3]) {
        let bu = basis(&self.ku, self.du, self.nu, u);
        let bv = basis(&self.kv, self.dv, self.nv, v);
        let dbu = dbasis(&self.ku, self.du, self.nu, u);
        let dbv = dbasis(&self.kv, self.dv, self.nv, v);
        let mut a = [0.0f64; 4];
        let mut au = [0.0f64; 4];
        let mut av = [0.0f64; 4];
        for i in 0..self.nu {
            for j in 0..self.nv {
                let c = self.cp[i * self.nv + j];
                for k in 0..4 {
                    a[k] += bu[i] * bv[j] * c[k];
                    au[k] += dbu[i] * bv[j] * c[k];
                    av[k] += bu[i] * dbv[j] * c[k];
                }
            }
        }
        let w = a[3];
        let s = [a[0] / w, a[1] / w, a[2] / w];
        let su = [
            (au[0] - s[0] * au[3]) / w,
            (au[1] - s[1] * au[3]) / w,
            (au[2] - s[2] * au[3]) / w,
        ];
        let sv = [
            (av[0] - s[0] * av[3]) / w,
            (av[1] - s[1] * av[3]) / w,
            (av[2] - s[2] * av[3]) / w,
        ];
        (s, su, sv)
    }

    /// Dense (flux, area) by composite 5-pt Gauss-Legendre per span
    /// rectangle, `cells` cells per span per axis.
    fn dense(&self, cells: usize) -> (f64, f64) {
        let gx = [
            -0.906_179_845_938_664,
            -0.538_469_310_105_683,
            0.0,
            0.538_469_310_105_683,
            0.906_179_845_938_664,
        ];
        let gw = [
            0.236_926_885_056_189,
            0.478_628_670_499_366,
            0.568_888_888_888_889,
            0.478_628_670_499_366,
            0.236_926_885_056_189,
        ];
        let spans = |knots: &[f64]| -> Vec<(f64, f64)> {
            let mut s = Vec::new();
            for w in knots.windows(2) {
                if w[1] > w[0] {
                    s.push((w[0], w[1]));
                }
            }
            s
        };
        let su = spans(&self.ku);
        let sv = spans(&self.kv);
        let mut flux = 0.0;
        let mut area = 0.0;
        for (ua, ub) in &su {
            for (va, vb) in &sv {
                for cu in 0..cells {
                    for cv in 0..cells {
                        let hu = (ub - ua) / cells as f64;
                        let hv = (vb - va) / cells as f64;
                        let u0 = ua + cu as f64 * hu;
                        let v0 = va + cv as f64 * hv;
                        for a in 0..5 {
                            for b in 0..5 {
                                let u = u0 + hu * 0.5 * (1.0 + gx[a]);
                                let v = v0 + hv * 0.5 * (1.0 + gx[b]);
                                let (s, sud, svd) = self.eval(u, v);
                                let cx = [
                                    sud[1] * svd[2] - sud[2] * svd[1],
                                    sud[2] * svd[0] - sud[0] * svd[2],
                                    sud[0] * svd[1] - sud[1] * svd[0],
                                ];
                                let f = s[0] * cx[0] + s[1] * cx[1] + s[2] * cx[2];
                                let g = (cx[0] * cx[0] + cx[1] * cx[1] + cx[2] * cx[2]).sqrt();
                                let wq = gw[a] * gw[b] * hu * hv * 0.25;
                                flux += wq * f;
                                area += wq * g;
                            }
                        }
                    }
                }
            }
        }
        (flux, area)
    }
}

/// Build the oracle patch from the same data handed to the kernel.
fn patch(
    ku: &KnotVector,
    kv: &KnotVector,
    control: &[[RingInterval; 3]],
    weights: &[f64],
) -> Patch {
    let nu = ku.control_count();
    let nv = kv.control_count();
    let cp = control
        .iter()
        .zip(weights)
        .map(|(c, w)| [c[0].lo() * w, c[1].lo() * w, c[2].lo() * w, *w])
        .collect();
    Patch {
        ku: ku.knots().to_vec(),
        kv: kv.knots().to_vec(),
        du: ku.degree(),
        dv: kv.degree(),
        nu,
        nv,
        cp,
    }
}

/// One probe: run the engine; if it answers, the dense oracle (and
/// any closed forms) must lie inside both brackets. Returns the
/// engine output for extra checks.
#[allow(clippy::too_many_arguments)]
fn probe(
    name: &str,
    ku: &KnotVector,
    kv: &KnotVector,
    control: &[[RingInterval; 3]],
    weights: &[f64],
    perimeter: f64,
    closed_flux: Option<f64>,
    closed_area: Option<f64>,
) -> Option<geom_brep::props::quad::FaceCutBounds> {
    let pa = patch(ku, kv, control, weights);
    let (of1, oa1) = pa.dense(24);
    let (of2, oa2) = pa.dense(48);
    // A boundary-layer patch (extreme weights) can defeat uniform
    // Gauss cells; with a closed form in hand the oracle is advisory
    // there, and the closed-form containment below is the real check.
    let oracle_ok = (of1 - of2).abs() < 1e-7 * (1.0 + of2.abs())
        && (oa1 - oa2).abs() < 1e-7 * (1.0 + oa2.abs());
    assert!(
        oracle_ok || (closed_flux.is_some() && closed_area.is_some()),
        "{name}: oracle did not converge: flux {of1} vs {of2}, area {oa1} vs {oa2}"
    );
    if !oracle_ok {
        println!("{name}: WARN oracle unconverged ({of1} vs {of2}); closed forms carry the probe");
    }
    if oracle_ok {
        if let Some(cf) = closed_flux {
            assert!(
                (of2 - cf).abs() < 1e-6 * (1.0 + cf.abs()),
                "{name}: oracle flux {of2} disagrees with closed form {cf}"
            );
        }
        if let Some(ca) = closed_area {
            assert!(
                (oa2 - ca).abs() < 1e-6 * (1.0 + ca.abs()),
                "{name}: oracle area {oa2} disagrees with closed form {ca}"
            );
        }
    }
    let out = nurbs_patch_face::<f64>(
        ku,
        kv,
        control,
        weights,
        {
            let (a, b) = ku.domain();
            let (c, d) = kv.domain();
            (a, b, c, d)
        },
        perimeter,
        0.0,
        Tolerance::get().eps,
        band(),
    );
    match out {
        Ok(fb) => {
            println!(
                "{name}: flux [{:.12e}, {:.12e}] oracle {:.12e}; area [{:.12e}, {:.12e}] oracle {:.12e}",
                fb.flux.lo(),
                fb.flux.hi(),
                of2,
                fb.area.lo(),
                fb.area.hi(),
                oa2
            );
            if oracle_ok {
                assert!(
                    fb.flux.lo() <= of2 && of2 <= fb.flux.hi(),
                    "{name}: FLUX EXCLUSION — enclosure [{}, {}] excludes oracle {}",
                    fb.flux.lo(),
                    fb.flux.hi(),
                    of2
                );
                assert!(
                    fb.area.lo() <= oa2 && oa2 <= fb.area.hi(),
                    "{name}: AREA EXCLUSION — enclosure [{}, {}] excludes oracle {}",
                    fb.area.lo(),
                    fb.area.hi(),
                    oa2
                );
            }
            if let Some(cf) = closed_flux {
                assert!(
                    fb.flux.lo() <= cf && cf <= fb.flux.hi(),
                    "{name}: FLUX EXCLUSION of closed form {cf}: [{}, {}]",
                    fb.flux.lo(),
                    fb.flux.hi()
                );
            }
            if let Some(ca) = closed_area {
                assert!(
                    fb.area.lo() <= ca && ca <= fb.area.hi(),
                    "{name}: AREA EXCLUSION of closed form {ca}: [{}, {}]",
                    fb.area.lo(),
                    fb.area.hi()
                );
            }
            Some(fb)
        }
        Err(e) => {
            println!("{name}: typed refusal (sound): {e}");
            None
        }
    }
}

const W2: f64 = core::f64::consts::FRAC_1_SQRT_2;
const PI: f64 = core::f64::consts::PI;

/// Sphere octant (unit radius, degenerate pole row): |flux| = area =
/// pi/2. Signed flux from the oracle.
#[test]
fn probe_sphere_octant() {
    let kv2 = KnotVector::unit_segment(2);
    let net = [
        p(0.0, 0.0, 1.0),
        p(0.0, 0.0, 1.0),
        p(0.0, 0.0, 1.0),
        p(1.0, 0.0, 1.0),
        p(1.0, 1.0, 1.0),
        p(0.0, 1.0, 1.0),
        p(1.0, 0.0, 0.0),
        p(1.0, 1.0, 0.0),
        p(0.0, 1.0, 0.0),
    ];
    let weights = [1.0, W2, 1.0, W2, 0.5, W2, 1.0, W2, 1.0];
    probe(
        "sphere-octant",
        &kv2,
        &kv2,
        &net,
        &weights,
        3.0 * PI / 2.0,
        None,
        Some(PI / 2.0),
    );
}

/// Quarter torus patch (R=2, r=0.5): dense oracle only.
#[test]
fn probe_quarter_torus() {
    let kv2 = KnotVector::unit_segment(2);
    let (rr, r) = (2.0, 0.5);
    // tube quarter arc in xz-plane: (R+r,0,0) -> (R+r,0,r) -> (R,0,r)
    let prof = [(rr + r, 0.0), (rr + r, r), (rr, r)];
    let pw = [1.0, W2, 1.0];
    let mut net = Vec::new();
    let mut weights = Vec::new();
    for (k, (x, z)) in prof.iter().enumerate() {
        net.push(p(*x, 0.0, *z));
        net.push(p(*x, *x, *z));
        net.push(p(0.0, *x, *z));
        for wj in [1.0, W2, 1.0] {
            weights.push(pw[k] * wj);
        }
    }
    probe(
        "quarter-torus",
        &kv2,
        &kv2,
        &net,
        &weights,
        2.0 * (PI / 2.0) * (rr + r) + 2.0 * (PI / 2.0) * r,
        None,
        None,
    );
}

/// Moebius-reparameterized quarter cylinder (weights scaled by
/// lambda^i, lambda = 10 -> same locus): flux = area = pi exactly.
#[test]
fn probe_moebius_quarter_cylinder() {
    let ku = KnotVector::unit_segment(2);
    let kv = KnotVector::unit_segment(1);
    let h = 2.0;
    let net = [
        p(1.0, 0.0, 0.0),
        p(1.0, 0.0, h),
        p(1.0, 1.0, 0.0),
        p(1.0, 1.0, h),
        p(0.0, 1.0, 0.0),
        p(0.0, 1.0, h),
    ];
    let l = 10.0;
    let weights = [1.0, 1.0, W2 * l, W2 * l, l * l, l * l];
    probe(
        "moebius-quarter-cylinder",
        &ku,
        &kv,
        &net,
        &weights,
        4.0 + PI,
        Some(PI),
        Some(PI),
    );
}

/// Bilinear rational reparameterized unit square at z=1 with EXTREME
/// mixed corner weights (1e-3 / 1e3): flux = area = 1 exactly.
#[test]
fn probe_extreme_weight_square() {
    let kv1 = KnotVector::unit_segment(1);
    let net = [
        p(0.0, 0.0, 1.0),
        p(0.0, 1.0, 1.0),
        p(1.0, 0.0, 1.0),
        p(1.0, 1.0, 1.0),
    ];
    let weights = [1e-3, 1e3, 1e3, 1e-3];
    probe(
        "extreme-weight-square",
        &kv1,
        &kv1,
        &net,
        &weights,
        4.0,
        Some(1.0),
        Some(1.0),
    );
}

/// Warped bilinear (hyperbolic paraboloid) with mixed extreme
/// weights: dense oracle only.
#[test]
fn probe_extreme_weight_hypar() {
    let kv1 = KnotVector::unit_segment(1);
    let net = [
        p(0.0, 0.0, 0.0),
        p(0.0, 1.0, 1.0),
        p(1.0, 0.0, 1.0),
        p(1.0, 1.0, 0.0),
    ];
    let weights = [1e-1, 1.0, 1.0, 1e1];
    probe(
        "extreme-weight-hypar",
        &kv1,
        &kv1,
        &net,
        &weights,
        6.0,
        None,
        None,
    );
}

/// Scale extremes: quarter cylinders at r = 1e-6 and r = 1e3.
/// Closed forms: flux = (pi/2) r^2 h, area = (pi/2) r h.
#[test]
fn probe_scale_extremes() {
    let ku = KnotVector::unit_segment(2);
    let kv = KnotVector::unit_segment(1);
    for (r, h, name) in [(1e-6, 2e-6, "tiny-cylinder"), (1e3, 2e3, "huge-cylinder")] {
        let net = [
            p(r, 0.0, 0.0),
            p(r, 0.0, h),
            p(r, r, 0.0),
            p(r, r, h),
            p(0.0, r, 0.0),
            p(0.0, r, h),
        ];
        let weights = [1.0, 1.0, W2, W2, 1.0, 1.0];
        probe(
            name,
            &ku,
            &kv,
            &net,
            &weights,
            2.0 * h + PI * r,
            Some(PI / 2.0 * r * r * h),
            Some(PI / 2.0 * r * h),
        );
    }
}

/// Half cylinder as a TWO-SPAN quadratic (interior double knot at
/// 1/2, non-uniform): flux = pi r^2 h = 2 pi, area = pi r h = 2 pi.
#[test]
fn probe_half_cylinder_interior_knot() {
    let ku = KnotVector::clamped(vec![0.0, 0.0, 0.0, 0.5, 0.5, 1.0, 1.0, 1.0], 2).unwrap();
    let kv = KnotVector::unit_segment(1);
    let h = 2.0;
    let net = [
        p(1.0, 0.0, 0.0),
        p(1.0, 0.0, h),
        p(1.0, 1.0, 0.0),
        p(1.0, 1.0, h),
        p(0.0, 1.0, 0.0),
        p(0.0, 1.0, h),
        p(-1.0, 1.0, 0.0),
        p(-1.0, 1.0, h),
        p(-1.0, 0.0, 0.0),
        p(-1.0, 0.0, h),
    ];
    let weights = [1.0, 1.0, W2, W2, 1.0, 1.0, W2, W2, 1.0, 1.0];
    probe(
        "half-cylinder-2span",
        &ku,
        &kv,
        &net,
        &weights,
        2.0 * h + 2.0 * PI,
        Some(2.0 * PI),
        Some(2.0 * PI),
    );
}

/// A C0 CORNER inside the domain (double knot, degree 2, kink not on
/// any cell grid: knot at 1/3): a genuinely discontinuous surface
/// derivative. The area pass has no straddle rule, so the honest
/// outcomes are containment or a typed refusal — a returned bracket
/// excluding the truth is the unsound outcome this probe hunts.
#[test]
fn probe_c0_kink_area() {
    let ku =
        KnotVector::clamped(vec![0.0, 0.0, 0.0, 1.0 / 3.0, 1.0 / 3.0, 1.0, 1.0, 1.0], 2).unwrap();
    let kv = KnotVector::unit_segment(1);
    // profile in the xy-plane: straight (0,0)->(1,0), corner, then
    // straight (1,0)->(1,4); extruded in z by 1.
    let prof = [(0.0, 0.0), (0.5, 0.0), (1.0, 0.0), (1.0, 2.0), (1.0, 4.0)];
    let mut net = Vec::new();
    for (x, y) in prof {
        net.push(p(x, y, 0.0));
        net.push(p(x, y, 1.0));
    }
    let weights = [0.9; 10];
    probe(
        "c0-kink-wall",
        &ku,
        &kv,
        &net,
        &weights,
        2.0 * 5.0 + 2.0,
        None,
        Some(5.0),
    );
}

/// D9 determinism: the same rational flux computed twice must be
/// BIT-identical (also printed for the debug/release cross-check).
#[test]
fn probe_determinism_bits() {
    let ku = KnotVector::unit_segment(2);
    let kv = KnotVector::unit_segment(1);
    let h = 2.0;
    let net = [
        p(1.0, 0.0, 0.0),
        p(1.0, 0.0, h),
        p(1.0, 1.0, 0.0),
        p(1.0, 1.0, h),
        p(0.0, 1.0, 0.0),
        p(0.0, 1.0, h),
    ];
    let weights = [1.0, 1.0, W2, W2, 1.0, 1.0];
    let run = || {
        nurbs_patch_face::<f64>(
            &ku,
            &kv,
            &net,
            &weights,
            (0.0, 1.0, 0.0, 1.0),
            4.0 + PI,
            0.0,
            Tolerance::get().eps,
            band(),
        )
        .unwrap()
    };
    let a = run();
    let b = run();
    println!(
        "DETBITS flux {:016x} {:016x} area {:016x} {:016x}",
        a.flux.lo().to_bits(),
        a.flux.hi().to_bits(),
        a.area.lo().to_bits(),
        a.area.hi().to_bits()
    );
    assert_eq!(a.flux.lo().to_bits(), b.flux.lo().to_bits());
    assert_eq!(a.flux.hi().to_bits(), b.flux.hi().to_bits());
    assert_eq!(a.area.lo().to_bits(), b.area.lo().to_bits());
    assert_eq!(a.area.hi().to_bits(), b.area.hi().to_bits());
    // the meter, as claimed: width(flux)/(3*area_mid)
    let meter = a.flux.width() / (3.0 * (a.area.lo() + a.area.hi()) * 0.5);
    println!(
        "METER quarter-cylinder {:.6e} target {:.6e}",
        meter,
        1024.0 * Tolerance::get().eps
    );
    // budget-defeat check lives in probe_budget_refusal below
    let refused = nurbs_patch_face::<f64>(
        &ku,
        &kv,
        &net,
        &weights,
        (0.0, 1.0, 0.0, 1.0),
        4.0 + PI,
        0.0,
        Tolerance::get().eps * 1e-6,
        band(),
    );
    match refused {
        Err(PropsError::QuadratureBudget {
            width_len,
            target_len,
        }) => {
            println!("BUDGET width_len {width_len:.6e} target {target_len:.6e}");
            assert!(width_len.is_finite() && width_len > target_len);
        }
        other => panic!("expected QuadratureBudget on a 1e-6-scaled eps, got {other:?}"),
    }
}

/// DIAGNOSIS: is certified knot refinement exact on the two-span
/// half-circle? Refine the HOMOGENEOUS 1-D net with the kernel's own
/// refine_plan chain (weights-1 plans, plain lerp — exactly what
/// refine_dir does) and compare rational curve points before/after
/// with the independent evaluator.
#[test]
fn diag_refine_half_circle() {
    use geom_core::spline::algebra::refine_plan;
    let kv = KnotVector::clamped(vec![0.0, 0.0, 0.0, 0.5, 0.5, 1.0, 1.0, 1.0], 2).unwrap();
    let pts = [(1.0, 0.0), (1.0, 1.0), (0.0, 1.0), (-1.0, 1.0), (-1.0, 0.0)];
    let ws = [1.0, W2, 1.0, W2, 1.0];
    // homogeneous 3-vectors (x*w, y*w, w)
    let mut hom: Vec<[f64; 3]> = pts
        .iter()
        .zip(&ws)
        .map(|((x, y), w)| [x * w, y * w, *w])
        .collect();
    let add: Vec<f64> = (1..16)
        .map(|k| k as f64 / 16.0)
        .filter(|t| !kv.knots().contains(t))
        .collect();
    let plans = refine_plan(&kv, &vec![1.0; 5], &add).unwrap();
    let mut cur_kv = kv.clone();
    for plan in &plans {
        hom = plan.apply_points(&hom, [f64::NAN; 3], |x, y, l| {
            [
                x[0] + (y[0] - x[0]) * l,
                x[1] + (y[1] - x[1]) * l,
                x[2] + (y[2] - x[2]) * l,
            ]
        });
        cur_kv = plan.knots().clone();
    }
    // evaluate both with the independent basis
    let eval = |kvv: &KnotVector, net: &[[f64; 3]], t: f64| -> (f64, f64) {
        let b = basis(kvv.knots(), 2, kvv.control_count(), t);
        let mut a = [0.0f64; 3];
        for (i, c) in net.iter().enumerate() {
            for k in 0..3 {
                a[k] += b[i] * c[k];
            }
        }
        (a[0] / a[2], a[1] / a[2])
    };
    let orig: Vec<[f64; 3]> = pts
        .iter()
        .zip(&ws)
        .map(|((x, y), w)| [x * w, y * w, *w])
        .collect();
    let mut worst = 0.0f64;
    for k in 0..=200 {
        let t = k as f64 / 200.0;
        let (x0, y0) = eval(&kv, &orig, t);
        let (x1, y1) = eval(&cur_kv, &hom, t);
        let d = ((x1 - x0).powi(2) + (y1 - y0).powi(2)).sqrt();
        if d > worst {
            worst = d;
            if d > 1e-12 {
                println!("t={t} orig=({x0},{y0}) refined=({x1},{y1}) d={d}");
            }
        }
    }
    println!("worst refinement deviation: {worst:e}");
    assert!(
        worst < 1e-12,
        "refinement is NOT locus-preserving: {worst:e}"
    );
}

/// ISOLATION: uniform weights w = 0.5 make the rational lane's
/// integrand identical to the weight-1 polynomial patch (w is
/// constant, f = N/w^3 reduces exactly). The integral lane certifies
/// the weight-1 twin; the two enclosures must OVERLAP (both contain
/// the same truth). Two-span half-cylinder net.
#[test]
fn diag_uniform_weight_twins() {
    let ku = KnotVector::clamped(vec![0.0, 0.0, 0.0, 0.5, 0.5, 1.0, 1.0, 1.0], 2).unwrap();
    let kv = KnotVector::unit_segment(1);
    let h = 2.0;
    let net = [
        p(1.0, 0.0, 0.0),
        p(1.0, 0.0, h),
        p(1.0, 1.0, 0.0),
        p(1.0, 1.0, h),
        p(0.0, 1.0, 0.0),
        p(0.0, 1.0, h),
        p(-1.0, 1.0, 0.0),
        p(-1.0, 1.0, h),
        p(-1.0, 0.0, 0.0),
        p(-1.0, 0.0, h),
    ];
    let run = |ws: &[f64]| {
        nurbs_patch_face::<f64>(
            &ku,
            &kv,
            &net,
            ws,
            (0.0, 1.0, 0.0, 1.0),
            2.0 * h + 2.0 * PI,
            0.0,
            Tolerance::get().eps,
            band(),
        )
    };
    let poly = run(&[1.0; 10]).expect("integral lane");
    println!(
        "poly-first flux [{:.12}, {:.12}] area [{:.12}, {:.12}]",
        poly.flux.lo(),
        poly.flux.hi(),
        poly.area.lo(),
        poly.area.hi()
    );
    let rat = run(&[0.5; 10]).expect("rational lane");
    println!(
        "poly flux [{:.12}, {:.12}]  rational flux [{:.12}, {:.12}]",
        poly.flux.lo(),
        poly.flux.hi(),
        rat.flux.lo(),
        rat.flux.hi()
    );
    let pa = patch(&ku, &kv, &net, &[0.5; 10]);
    let (of, _) = pa.dense(48);
    println!("oracle flux {of:.12}");
    assert!(
        rat.flux.lo() <= of && of <= rat.flux.hi(),
        "rational lane excludes the oracle: [{}, {}] vs {of}",
        rat.flux.lo(),
        rat.flux.hi()
    );
}

/// Genericity spot check: the rational lane driven by the certified
/// Interval decision scalar must agree with the f64 lane bit-for-bit
/// on the returned enclosure (the RingInterval arithmetic is shared;
/// only decisions route through T).
#[cfg(feature = "interval")]
#[test]
fn probe_interval_scalar_agrees() {
    use geom_core::Interval;
    let ku = KnotVector::unit_segment(2);
    let kv = KnotVector::unit_segment(1);
    let h = 2.0;
    let net = [
        p(1.0, 0.0, 0.0),
        p(1.0, 0.0, h),
        p(1.0, 1.0, 0.0),
        p(1.0, 1.0, h),
        p(0.0, 1.0, 0.0),
        p(0.0, 1.0, h),
    ];
    let weights = [1.0, 1.0, W2, W2, 1.0, 1.0];
    let rect = (0.0, 1.0, 0.0, 1.0);
    let eps = Tolerance::get().eps;
    let a = nurbs_patch_face::<f64>(&ku, &kv, &net, &weights, rect, 4.0 + PI, 0.0, eps, band())
        .unwrap();
    let b =
        nurbs_patch_face::<Interval>(&ku, &kv, &net, &weights, rect, 4.0 + PI, 0.0, eps, band())
            .unwrap();
    assert_eq!(a.flux.lo().to_bits(), b.flux.lo().to_bits());
    assert_eq!(a.flux.hi().to_bits(), b.flux.hi().to_bits());
    assert_eq!(a.area.lo().to_bits(), b.area.lo().to_bits());
    assert_eq!(a.area.hi().to_bits(), b.area.hi().to_bits());
    assert!(a.flux.lo() <= PI && PI <= a.flux.hi());
}
