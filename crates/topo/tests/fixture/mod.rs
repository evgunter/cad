//! The M6-2 acceptance fixture: one cylinder×sphere rung-3 edge, at
//! rest, carrying a fitted chart image — built once, at any scalar.
//!
//! **The shape of the construction, and why it is this shape.** The
//! trace is `f64` BY DESIGN (`ssi::march`/`jet`/`system` are untrusted
//! candidate generation and stay `f64`-only), so the fixture traces at
//! `f64`, restricts the branch, and then LIFTS the finished structure
//! to the caller's scalar — the ratified f64-structure + T-lift
//! pattern, and exactly how a recipe replayed at the interval scalar
//! reaches a body at rest.
//!
//! **The carrier is the kernel's own.** `cylinder_sphere_ssi` returns a
//! marched-and-fitted `SsiBranch`, and the edge's carrier is that
//! curve restricted by `NurbsCurve3::split_at` — knot insertion, exact
//! in ℝ, so the arc is the traced one and not an interpolation of it.
//! Splitting also preserves the parameter, which is what lets the
//! chart image be split at the SAME parameters and keep the OQ4
//! same-`t` identity trivially.
//!
//! **The chart image is fitted here, and the reason is a real
//! absence.** `SsiBranch` carries `pcurve_a`/`pcurve_b` only when the
//! trace produced them — the ℝ⁴ parametric lane does; the ℝ³ implicit
//! lane this analytic pair runs on calls `fit_branch(&points, None)`
//! and both fields are `None` (`geom_brep::ssi::finish_r3`). There is
//! therefore no kernel-minted chart image to restrict, so the fixture
//! builds one the way `fit_branch` would: sample the branch's own
//! carrier, take the exact cylinder-chart preimages, and interpolate
//! them **on the carrier's own parameters**, so `P(t)` and `C(t)` are
//! the same `t` by construction rather than by coincidence.
//!
//! **The scaffold caveat, stated once.** This is a scaffold BY DESIGN:
//! no kernel constructor mints a `Pcurve::Fitted` cache into a body
//! today, because the cyl×sphere fitted-chord join lane is banked past
//! M6. When that lane lands, this row should re-anchor to a
//! constructor-built body and the hand assembly below should go.
//!
//! Nothing here invents a certificate: the edge goes in through
//! `EdgeCurve::certify`'s rung-3 gate and the cache through
//! `PcurveCache::certify_fitted`, both of which refuse typed.

#![allow(dead_code)] // loaded once per consumer; each uses a subset
#![allow(unreachable_pub)] // why: root Cargo.toml, the `unreachable_pub` stanza

use std::sync::Arc;

use geom::Surface;
use geom::{Curve3, NurbsCurve2, NurbsCurve3};
use geom_brep::ssi::{self, SsiDomain, SsiError};
use geom_brep::{ChartWindow, EdgeCurveSpec, EdgeDescriptionSpec, Pcurve, PcurveCache};
use geom_core::Tol;
use geom_core::{Band, Point2, Point3, Real, Vec3};
use topo::{Body, HalfEdgeKey};

/// The fixture's cylinder: offset from the sphere's centre so the two
/// intersection loops differ wildly in size (the PR 7 planted shape).
const CYL_ORIGIN: (f64, f64, f64) = (0.03, 0.0, 0.0);
const CYL_RADIUS: f64 = 0.08;
const SPH_RADIUS: f64 = 1.0;

/// How many samples the chart image is interpolated from, over the
/// WHOLE traced loop. Structure (C6), fixed for determinism (D9); the
/// quarter each row uses keeps a quarter of them.
const SAMPLES: usize = 481;

/// The fitted degree — the SSI lane's own.
const DEGREE: usize = 3;

/// A built fixture: the body and the two half-edges of its rung-3 edge.
pub struct Built<T: Real> {
    pub body: Body<T>,
    pub he_plus: HalfEdgeKey,
    pub he_minus: HalfEdgeKey,
    pub carrier: Arc<NurbsCurve3<T>>,
    pub image: Arc<NurbsCurve2<T>>,
    pub cylinder: Surface<T>,
    pub sphere: Surface<T>,
    pub window: ChartWindow<T>,
}

fn cylinder<T: Real>() -> Surface<T> {
    Surface::Cylinder {
        origin: Point3::new(
            T::from_f64(CYL_ORIGIN.0),
            T::from_f64(CYL_ORIGIN.1),
            T::from_f64(CYL_ORIGIN.2),
        ),
        axis: Vec3::new(T::zero(), T::zero(), T::one()),
        radius: T::from_f64(CYL_RADIUS),
        u_ref: Vec3::new(T::one(), T::zero(), T::zero()),
    }
}

fn sphere<T: Real>() -> Surface<T> {
    Surface::Sphere {
        center: Point3::new(T::zero(), T::zero(), T::zero()),
        radius: T::from_f64(SPH_RADIUS),
        axis: Vec3::new(T::zero(), T::zero(), T::one()),
        u_ref: Vec3::new(T::one(), T::zero(), T::zero()),
    }
}

/// One certified rung-3 branch of the planted fixture, or `None` when
/// this ε's sample demand exceeds the SSI door's named fit budget — the
/// typed stand-down, never an ε literal.
///
/// **Memoized per process.** The trace is the expensive part of the
/// fixture, and the module's own opening line already says the
/// fixture is "built once, at any scalar": every caller here restricts
/// the SAME traced locus, so a second trace re-derives a bit-identical
/// branch (D9) and buys nothing. INVARIANT: no row asserts that two
/// INDEPENDENT traces agree — the cross-scalar dominance claim (the
/// `DOMINANCE` half of
/// `certified::the_fitted_certificate_is_derived_at_the_interval_scalar_and_dominates_f64`)
/// compares an interval LIFT against the f64 one, which is a claim
/// about the lift and is unaffected by (indeed sharpened by) sharing
/// one f64 structure. Sharing must therefore never become an assertion
/// this file relies on: if a row ever wants two independent traces, it
/// must call `trace_branch` directly and say why.
///
/// nextest is process-per-test, so this only helps WITHIN one test —
/// which is exactly where the duplication is (`build` + `foreign_cache`
/// in one row, and the interval + f64 `build`s of the dominance half of
/// the row above; the test-cost audit merged that row INTO the interval
/// row precisely so the memo has something to share).
fn branch_or_budget() -> Option<&'static ssi::SsiBranch> {
    static BRANCH: std::sync::OnceLock<Option<ssi::SsiBranch>> = std::sync::OnceLock::new();
    BRANCH.get_or_init(trace_branch).as_ref()
}

/// The trace itself — the full `cylinder_sphere_ssi` exhaustiveness run
/// the memo above wraps.
fn trace_branch() -> Option<ssi::SsiBranch> {
    let slab = SsiDomain {
        center: Point3::new(0.0, 0.0, 0.0),
        half_extent: 1.5,
        extent: 2.0,
        floor_scale: 1.0,
    };
    match ssi::cylinder_sphere_ssi(
        &cylinder::<f64>(),
        &sphere::<f64>(),
        slab,
        Band::linear(Tol::witness()).unwrap(),
    ) {
        Ok(out) => Some(out.branches.into_iter().next().expect("two loops")),
        Err(SsiError::FitSampleBudget { .. }) => None,
        Err(e) => panic!("the planted fixture: {e}"),
    }
}

/// The cylinder chart coordinates of a locus point: `u` the azimuth
/// about the axis, `v` the axial height. Both exact arithmetic on the
/// chart's own frame — this is the chart map's inverse, not a fit.
fn chart_of(p: Point3<f64>) -> Point2<f64> {
    let w = p - Point3::new(CYL_ORIGIN.0, CYL_ORIGIN.1, CYL_ORIGIN.2);
    Point2::new(w.y.atan2(w.x), w.z)
}

/// The `f64` structure both lanes share: a sub-arc of the branch's own
/// carrier, its chart image, and the parameter they agree on.
struct Structure {
    carrier: NurbsCurve3<f64>,
    image: NurbsCurve2<f64>,
}

/// Restrict the branch to one sub-arc: the kernel's own carrier and the
/// chart image built on its parameter, both cut at the SAME parameters
/// by knot insertion.
///
/// `frac` picks which quarter of the loop — the fixture uses the first,
/// and the planted-corruption row the third, which is a genuinely
/// different arc of the same locus and therefore the sharpest thing to
/// attach to the first one's edge.
fn restrict(branch: &ssi::SsiBranch, frac: (f64, f64)) -> Option<Structure> {
    let Curve3::Nurbs(ref loop_carrier) = branch.carrier else {
        panic!("a rung-3 carrier is a NURBS curve")
    };
    // `fit_branch` interpolates on chord parameters, so the traced
    // carrier's domain is exactly [0, 1] — which is also what
    // `interpolate_with_params` requires of the image's parameters.
    // Checked rather than assumed: a domain convention change should
    // stand the fixture down, not silently skew the parameter identity.
    let (d0, d1) = loop_carrier.domain();
    if d0 != 0.0 || d1 != 1.0 {
        return None;
    }
    // The image's interpolation nodes ARE carrier parameters, so
    // `P(tᵢ) = chart(C(tᵢ))` holds at every node by construction and
    // the shared-parameter contract survives the split below.
    #[allow(clippy::cast_precision_loss)]
    let params: Vec<f64> = (0..SAMPLES)
        .map(|i| i as f64 / (SAMPLES - 1) as f64)
        .collect();
    let pts: Vec<Point3<f64>> = params.iter().map(|t| loop_carrier.eval(*t)).collect();
    // The chart images, on ONE branch: `atan2` is principal, so the
    // azimuths are unwrapped continuously ONCE here, exactly as the
    // per-face loop walk chooses a branch once and never per sample
    // (the M2 PR 5 meridian finding). The loop winds exactly once, so
    // the unwrapped channel is monotone across the whole domain and no
    // quarter of it contains a period jump.
    let mut chart: Vec<Point2<f64>> = pts.iter().map(|p| chart_of(*p)).collect();
    let tau = core::f64::consts::TAU;
    for i in 1..chart.len() {
        let mut u = chart[i].x;
        while u - chart[i - 1].x > tau / 2.0 {
            u -= tau;
        }
        while chart[i - 1].x - u > tau / 2.0 {
            u += tau;
        }
        chart[i].x = u;
    }
    let image = NurbsCurve2::<f64>::interpolate_with_params(&chart, DEGREE, &params).ok()?;
    // Knot insertion is exact in ℝ and preserves the parameter, so
    // both halves of the pair stay the same `t` after cutting.
    let carrier = sub_arc3(loop_carrier, frac)?;
    let image = sub_arc2(&image, frac)?;
    Some(Structure { carrier, image })
}

/// The `[a, b]` restriction of a 3-D curve, by two exact splits.
fn sub_arc3(c: &NurbsCurve3<f64>, frac: (f64, f64)) -> Option<NurbsCurve3<f64>> {
    let tail = if frac.0 > 0.0 {
        c.split_at(frac.0).ok()?.1
    } else {
        c.clone()
    };
    if frac.1 < 1.0 {
        Some(tail.split_at(frac.1).ok()?.0)
    } else {
        Some(tail)
    }
}

/// The 2-D counterpart, cut at the SAME parameters.
fn sub_arc2(c: &NurbsCurve2<f64>, frac: (f64, f64)) -> Option<NurbsCurve2<f64>> {
    let tail = if frac.0 > 0.0 {
        c.split_at(frac.0).ok()?.1
    } else {
        c.clone()
    };
    if frac.1 < 1.0 {
        Some(tail.split_at(frac.1).ok()?.0)
    } else {
        Some(tail)
    }
}

fn lift3<T: Real>(c: &NurbsCurve3<f64>) -> NurbsCurve3<T> {
    let control = c
        .control()
        .iter()
        .map(|p| Point3::new(T::from_f64(p.x), T::from_f64(p.y), T::from_f64(p.z)))
        .collect();
    NurbsCurve3::new(c.knots().clone(), control, c.weights().to_vec()).expect("lifted structure")
}

fn lift2<T: Real>(c: &NurbsCurve2<f64>) -> NurbsCurve2<T> {
    let control = c
        .control()
        .iter()
        .map(|p| Point2::new(T::from_f64(p.x), T::from_f64(p.y)))
        .collect();
    NurbsCurve2::new(c.knots().clone(), control, c.weights().to_vec()).expect("lifted structure")
}

/// Build the fixture at `T`. `None` is the typed budget stand-down.
pub fn build<T>() -> Option<Built<T>>
where
    T: geom_brep::PcurveFittedLane + geom_core::Bounds,
{
    let branch = branch_or_budget()?;
    let s = restrict(branch, (0.0, 0.25))?;
    Some(assemble(&s))
}

/// The planted corruption for the at-rest row: a cache certified —
/// honestly, through the same door — for a DIFFERENT arc of the same
/// locus (the third quarter). Attaching it to this edge is the sharpest
/// test of "the re-certification re-derives and never consults the
/// stored certificate": every stored number in it is true, and true
/// about the wrong carrier.
pub fn foreign_cache(built: &Built<f64>) -> PcurveCache<f64> {
    let branch = branch_or_budget().expect("the fixture already built once");
    let s = restrict(branch, (0.5, 0.75)).expect("the third quarter restricts");
    let other = assemble::<f64>(&s);
    let _ = built;
    other
        .body
        .pcurve(other.he_plus)
        .expect("the foreign cache")
        .clone()
}

/// The dual lane's refusal, executed: the same fitted image offered at
/// a scalar that may not certify. Since the D1 ruling (2026-08-19) a
/// dual DOES carry a bracket, so the refusal this exercises is the
/// lane's own, not a missing `Bounds` impl standing in for it.
pub fn certify_at_dual(built: &Built<f64>) -> geom_brep::PcurveCertifyError {
    type D = geom_core::Dual<f64>;
    let carrier = Curve3::Nurbs(Arc::new(lift3::<D>(&built.carrier)));
    let image = Arc::new(lift2::<D>(&built.image));
    let (t0, t1) = image.domain();
    let window = Pcurve::Fitted(Arc::clone(&image)).chart_box(D::from_f64(t0), D::from_f64(t1));
    PcurveCache::<D>::certify_fitted(
        image,
        D::from_f64(t0),
        D::from_f64(t1),
        &carrier,
        &cylinder::<D>(),
        Some(&sphere::<D>()),
        window,
        Band::linear(Tol::witness()).unwrap(),
    )
    .expect_err("a dual scalar has no fitted lane")
}

/// Assemble the body: the cylinder face carries the edge (its chart is
/// the well-conditioned one — azimuth about the axis, height along it),
/// the sphere is the mate the edge's own DESCRIPTION names.
fn assemble<T>(s: &Structure) -> Built<T>
where
    T: geom_brep::PcurveFittedLane + geom_core::Bounds,
{
    let band = Band::linear(Tol::witness()).unwrap();
    let carrier = Arc::new(lift3::<T>(&s.carrier));
    let image = Arc::new(lift2::<T>(&s.image));
    let (f0, f1) = s.carrier.domain();
    let (t0, t1) = (T::from_f64(f0), T::from_f64(f1));
    let (p0, p1) = (carrier.eval(t0), carrier.eval(t1));

    let mut body = Body::<T>::new();
    let seed = body.mvfs(p0).unwrap();
    let cyl_key = body
        .set_face_surface(seed.face, topo::FaceSurface::New(cylinder::<T>()))
        .unwrap();
    let anchor = body.mvfs(p1).unwrap();
    let sph_key = body
        .set_face_surface(anchor.face, topo::FaceSurface::New(sphere::<T>()))
        .unwrap();
    let mid = T::from_f64(0.5 * (f0 + f1));
    let made = body
        .mev(
            topo::MevSite::Lone {
                r#loop: seed.r#loop,
            },
            p1,
            EdgeCurveSpec {
                description: EdgeDescriptionSpec::Intersection {
                    s1: cyl_key,
                    s2: sph_key,
                    witness: carrier.eval(mid),
                },
                carrier: Curve3::Nurbs(Arc::clone(&carrier)),
                param_start: t0,
                param_end: t1,
            },
            Tol::witness(),
        )
        .expect("the rung-3 edge certifies at rest");

    let window = Pcurve::Fitted(Arc::clone(&image)).chart_box(t0, t1);
    let edge = body.get_edge(made.edge).expect("the edge resolves");
    let (he_plus, he_minus) = (edge.he_plus, edge.he_minus);
    for he in [he_plus, he_minus] {
        let cache = PcurveCache::<T>::certify_fitted(
            Arc::clone(&image),
            t0,
            t1,
            &Curve3::Nurbs(Arc::clone(&carrier)),
            &cylinder::<T>(),
            Some(&sphere::<T>()),
            window,
            band,
        )
        .expect("the fitted cache certifies through the M6-2 door");
        body.attach_pcurve(he, cache);
    }

    Built {
        body,
        he_plus,
        he_minus,
        carrier,
        image,
        cylinder: cylinder::<T>(),
        sphere: sphere::<T>(),
        window,
    }
}
