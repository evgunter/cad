//! A plain-`f64` dense oracle for the patch door, and the honest-posture
//! gate every caller of that door shares.
//!
//! **What is in here and what is deliberately not.** The oracle below —
//! Cox–de Boor bases evaluated straight from the knots, composite
//! 5-point Gauss–Legendre over knot-aligned cells — is ONE derivation
//! of a patch's flux and area, independent of `geom_brep::props::quad`.
//! `cert5_r2_probes.rs` and `review_r1_rational_probes.rs` each carry
//! another derivation of the same two numbers. Those are not moved
//! here: a probe whose whole claim is "the kernel's bracket contains a
//! truth I computed without it" loses that claim the moment two such
//! probes compute the truth with one shared routine, so the crate keeps
//! more than one on purpose. What this module removes is the copy that
//! was NOT independent — `cert5_arm_and_cells.rs` already reached into
//! `cert5_r1_patch_probes.rs` for this exact code rather than restating
//! it, and now both reach here.

// `face_posture` panics on a posture the probes do not accept, which is
// the point of it; the suites that used to spell that panic themselves
// all carry this allow.
#![allow(clippy::panic)]

use geom_brep::props::PropsError;
use geom_brep::props::quad::{FaceCutBounds, nurbs_patch_face};
use geom_core::spline::KnotVector;
use geom_core::{Band, RingInterval};

fn basis(knots: &[f64], degree: usize, ncp: usize, t: f64) -> Vec<f64> {
    let n = knots.len() - 1;
    let mut nn = vec![0.0f64; n];
    let mut last = None;
    for i in 0..n {
        if knots[i] <= t && t < knots[i + 1] {
            nn[i] = 1.0;
        }
        if knots[i] < knots[i + 1] {
            last = Some(i);
        }
    }
    if t >= knots[n] {
        for x in nn.iter_mut().take(n) {
            *x = 0.0;
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

fn dbasis(knots: &[f64], degree: usize, ncp: usize, t: f64) -> Vec<f64> {
    let n = knots.len() - 1;
    let mut low = basis(knots, degree - 1, n - (degree - 1), t);
    low.resize(n, 0.0);
    let mut out = vec![0.0f64; ncp];
    #[allow(clippy::cast_precision_loss)]
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

pub(crate) struct Patch {
    ku: Vec<f64>,
    kv: Vec<f64>,
    du: usize,
    dv: usize,
    nu: usize,
    nv: usize,
    cp: Vec<[f64; 4]>,
}

impl Patch {
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

    /// (flux, area) by composite 5-point Gauss-Legendre, `cells` per
    /// knot span.
    ///
    /// **THE LADDER ARGUMENT — this doc is its one home.** A dense
    /// oracle is only a truth if you can say how far off it is, and
    /// the way every probe in this family says so is to evaluate at
    /// `cells` and at `2 * cells` and require the two to agree.
    /// Why that is enough: `spans` below never lets a cell straddle a
    /// knot, so the integrand on one cell is smooth and the composite
    /// 5-point rule converges as `h^10`. Halving `h` therefore divides
    /// the error by `2^10`, which means the OBSERVED gap between the
    /// two evaluations bounds the finer one's own error by that gap
    /// over `2^10 - 1` — roughly three orders below the agreement
    /// threshold the caller asserts. A ladder is chosen by putting the
    /// finer rung where the containment slack needs it and the coarser
    /// one a factor of two below; it is not made safer by starting
    /// higher, only slower.
    ///
    /// Cited by, and not repeated in, the crate's OTHER derivations of
    /// these two numbers — each its own spelling, sharing this
    /// reasoning and none of this code:
    /// `crates/geom-brep/tests/cert5_r2_probes.rs` (ladder 8/16) and
    /// `crates/geom-brep/tests/review_r1_rational_probes.rs`
    /// (ladder 24/48). The two callers of THIS routine,
    /// `cert5_r1_patch_probes.rs` and `cert5_arm_and_cells.rs`, both
    /// run it at 12/24 and inherit the argument by using it.
    pub(crate) fn dense(&self, cells: usize) -> (f64, f64) {
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
                        #[allow(clippy::cast_precision_loss)]
                        let hu = (ub - ua) / cells as f64;
                        #[allow(clippy::cast_precision_loss)]
                        let hv = (vb - va) / cells as f64;
                        #[allow(clippy::cast_precision_loss)]
                        let u0 = ua + cu as f64 * hu;
                        #[allow(clippy::cast_precision_loss)]
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

pub(crate) fn oracle_patch(
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

/// Drive the patch door over a knot vector pair's own domain and hand
/// back its posture, having already ruled out the postures no probe in
/// this crate accepts.
///
/// `Ok(bounds)` is a certified return. `Err(e)` is one of the four
/// HONEST refusals — a quadrature budget stop, an unsupported
/// configuration, an escalated funnel decision, a degenerate face — and
/// a caller is free to treat it as a pass, print it, or count it.
/// Anything else PANICS here, naming `name`: a door that answers a
/// probe with a posture outside that set has broken a contract the
/// probe cannot express as a number, and each caller spelling that
/// panic itself is how two of them came to spell it differently.
///
/// The rectangle is the two knot vectors' own domain, which is what
/// every caller passed; `boundary_defect` is zero, which is what an
/// untrimmed rectangle means. `perimeter`, `eps` and `band` stay the
/// caller's — they are what the probes vary.
#[allow(clippy::too_many_arguments)] // one parameter per named quantity
pub(crate) fn face_posture(
    name: &str,
    ku: &KnotVector,
    kv: &KnotVector,
    control: &[[RingInterval; 3]],
    weights: &[f64],
    perimeter: f64,
    eps: f64,
    band: Band,
) -> Result<FaceCutBounds, PropsError> {
    let (a, b) = ku.domain();
    let (c, d) = kv.domain();
    let out = nurbs_patch_face::<f64>(
        ku,
        kv,
        control,
        weights,
        (a, b, c, d),
        perimeter,
        0.0,
        eps,
        band,
    );
    match out {
        Ok(fb) => Ok(fb),
        Err(
            e @ (PropsError::QuadratureBudget { .. }
            | PropsError::QuadratureUnsupported { .. }
            | PropsError::Escalated { .. }
            | PropsError::DegenerateFace),
        ) => Err(e),
        Err(other) => panic!("{name}: not an honest posture: {other}"),
    }
}
