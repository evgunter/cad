//! A plain-`f64` dense oracle for the patch door, the quadrature loop
//! and derivative recurrence every such oracle in this crate stands on,
//! and the honest-posture gate every caller of that door shares.
//!
//! **What is in here and what is deliberately not.** The oracle below —
//! Cox–de Boor bases evaluated straight from the knots, composite
//! 5-point Gauss–Legendre over knot-aligned cells — is ONE derivation
//! of a patch's flux and area, independent of `geom_brep::props::quad`.
//! `cert5_r2_probes.rs` and `review_r1_rational_probes.rs` each carry
//! another, and the crate keeps more than one on purpose: a probe whose
//! whole claim is "the kernel's bracket contains a truth I computed
//! without it" loses that claim the moment two such probes compute the
//! truth with one shared routine.
//!
//! **What is shared between them is therefore drawn on purpose, not by
//! what happens to look alike.** The part a `props::quad` defect could
//! plausibly be mirrored by is the BASIS — how the Cox–de Boor recursion
//! is seeded and what it does at the domain end — and that is the part
//! each oracle keeps its own of, saying so at its copy. The parts below
//! it are one job with one answer: [`dbasis_over`] is the divided
//! difference of lower-degree values, taken over WHICHEVER basis the
//! caller hands it, and [`dense_over`] is the composite Gauss–Legendre
//! sum, taken over whichever evaluator the caller hands it. Both are
//! parameterised for exactly that reason — sharing them does not merge
//! two derivations, it stops two derivations from restating one loop.
//!
//! The copy this module simply removed is the one that was never
//! independent at all: `cert5_arm_and_cells.rs` already reached into
//! `cert5_r1_patch_probes.rs` for this exact code rather than restating
//! it, and now both reach here.

use geom_brep::props::PropsError;
use geom_brep::props::quad::{FaceCutBounds, nurbs_patch_face};
use geom_core::spline::KnotVector;
use geom_core::{Band, RingInterval, Tol};

/// All basis values `N_{i,p}(t)`, seeded by a `t >= knots[n]` branch at
/// the domain end.
///
/// **Deliberately not the crate's only spelling of this**, and the
/// reason is in this module's header: `cert5_r2_probes.rs` and
/// `review_r1_rational_probes.rs` seed the recursion by clamping `t`
/// into the last nonzero span instead, and that difference is the
/// independence their claims rest on.
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

/// Basis derivatives `N'_{i,p}(t)`, from the degree-`p-1` values that
/// `basis` — **the caller's own** — produces.
///
/// This is the divided-difference identity and nothing else, which is
/// why it takes the basis rather than picking one: an oracle that keeps
/// its own seeding (see the header) keeps it here too, and still does
/// not restate these fourteen lines.
pub(crate) fn dbasis_over(
    basis: impl Fn(&[f64], usize, usize, f64) -> Vec<f64>,
    knots: &[f64],
    degree: usize,
    ncp: usize,
    t: f64,
) -> Vec<f64> {
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

fn dbasis(knots: &[f64], degree: usize, ncp: usize, t: f64) -> Vec<f64> {
    dbasis_over(basis, knots, degree, ncp, t)
}

/// (flux, area) by composite 5-point Gauss–Legendre, `cells` per knot
/// span of `ku` x `kv`, over the surface `eval` describes as
/// `(S, S_u, S_v)`.
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
/// **Who runs this loop, and at which ladder.** Every dense oracle in
/// the crate does, each over its OWN `eval` — which is where their
/// independence lives, not here: `cert5_r1_patch_probes.rs` and
/// `cert5_arm_and_cells.rs` at 12/24, `cert5_r2_probes.rs` at 8/16,
/// `review_r1_rational_probes.rs` at 24/48. The first two reach it
/// through [`Patch::dense`]; the last two hand in an evaluator built on
/// their own bases.
///
/// **Who cites this argument in prose** rather than inheriting it by
/// calling: `crates/geom-brep/tests/cert5_r2_probes.rs` (on its
/// `drive`'s doc) and `crates/geom-brep/tests/cert5_arm_and_cells.rs`
/// (at its convergence assertion). Both point here by path; keep them
/// pointing at this doc if it moves.
pub(crate) fn dense_over(
    ku: &[f64],
    kv: &[f64],
    cells: usize,
    eval: impl Fn(f64, f64) -> ([f64; 3], [f64; 3], [f64; 3]),
) -> (f64, f64) {
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
    let su = spans(ku);
    let sv = spans(kv);
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
                            let (s, sud, svd) = eval(u, v);
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

    /// This module's own oracle's (flux, area) at `cells` per span —
    /// [`dense_over`] over [`Patch::eval`], where the ladder argument
    /// this caller owes its reader is written.
    pub(crate) fn dense(&self, cells: usize) -> (f64, f64) {
        dense_over(&self.ku, &self.kv, cells, |u, v| self.eval(u, v))
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
/// Anything else PANICS: a door that answers a probe with a posture
/// outside that set has broken a contract the probe cannot express as a
/// number, and each caller spelling that panic itself is how two of
/// them came to spell it differently. The panic is `#[track_caller]`,
/// so it names the caller's own line rather than a string the caller
/// had to remember to pass — put `#[track_caller]` on any wrapper that
/// sits between a row and this, or the row will not be the line named.
///
/// The rectangle is the two knot vectors' own domain, which is what
/// every caller passed; `boundary_defect` is zero, which is what an
/// untrimmed rectangle means. `perimeter` and `eps` stay the caller's —
/// they are what the probes vary. The BAND does not: both callers
/// passed `Band::linear(Tol::witness())`, so it is taken here, and a
/// row that ever needs a different one should call the door itself
/// rather than grow a parameter nothing varies.
#[track_caller]
#[allow(clippy::panic)] // the posture contract above is the point of it
pub(crate) fn face_posture(
    ku: &KnotVector,
    kv: &KnotVector,
    control: &[[RingInterval; 3]],
    weights: &[f64],
    perimeter: f64,
    eps: f64,
) -> Result<FaceCutBounds, PropsError> {
    let (a, b) = ku.domain();
    let (c, d) = kv.domain();
    #[allow(clippy::unwrap_used)] // a linear band on the witness tolerance is valid by construction
    let band = Band::linear(Tol::witness()).unwrap();
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
        Err(other) => panic!("not an honest posture: {other}"),
    }
}
