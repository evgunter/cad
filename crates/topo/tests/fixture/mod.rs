//! The M6-2 acceptance fixture: one cylinder×sphere rung-3 edge, at
//! rest, carrying a fitted chart image — built once, at any scalar.
//!
//! **The shape of the construction, and why it is this shape.** The
//! trace is `f64` BY DESIGN (`ssi::march`/`jet`/`system` are untrusted
//! candidate generation and stay `f64`-only), so the fixture traces at
//! `f64`, fits the carrier and its chart image on ONE shared
//! parameterization — the `ssi::fit_branch` idiom, which is what makes
//! `P(t)` and `C(t)` the same `t` by construction (OQ4) — and then
//! LIFTS the finished structure to the caller's scalar. That is the
//! ratified f64-structure + T-lift pattern, and it is exactly how a
//! recipe replayed at the interval scalar reaches a body at rest.
//!
//! Nothing here invents a certificate: the edge goes in through
//! `EdgeCurve::certify`'s rung-3 gate and the cache through
//! `PcurveCache::certify_fitted`, both of which refuse typed.

#![allow(dead_code)]

use std::sync::Arc;

use geom_brep::ssi::{self, SsiDomain, SsiError};
use geom_brep::{ChartWindow, EdgeCurveSpec, EdgeGeometry, Pcurve, PcurveCache};
use geom_core::{Band, Point2, Point3, Real, Tolerance, Vec3};
use geom_curves::{Curve3, NurbsCurve2, NurbsCurve3};
use geom_surfaces::Surface;
use topo::{Body, HalfEdgeKey};

/// The fixture's cylinder: offset from the sphere's centre so the two
/// intersection loops differ wildly in size (the PR 7 planted shape).
const CYL_ORIGIN: (f64, f64, f64) = (0.03, 0.0, 0.0);
const CYL_RADIUS: f64 = 0.08;
const SPH_RADIUS: f64 = 1.0;

/// How many locus samples the sub-arc is refit from. Structure (C6),
/// fixed for determinism (D9).
const SAMPLES: usize = 121;

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
fn branch_or_budget() -> Option<ssi::SsiBranch> {
    let slab = SsiDomain {
        center: Point3::new(0.0, 0.0, 0.0),
        half_extent: 1.5,
        extent: 2.0,
        eps: Tolerance::get().eps,
        floor_scale: 1.0,
    };
    match ssi::cylinder_sphere_ssi(
        &cylinder::<f64>(),
        &sphere::<f64>(),
        slab,
        Band::linear().unwrap(),
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

/// The `f64` structure both lanes share: the refit sub-arc carrier, its
/// chart image, and the parameterization they agree on.
struct Structure {
    carrier: NurbsCurve3<f64>,
    image: NurbsCurve2<f64>,
}

/// Refit one sub-arc of the traced loop, carrier and chart image
/// TOGETHER on the sub-arc's own chord parameters.
///
/// Refitting (rather than knot-splitting the traced carrier) is what
/// puts both curves on the same `[0, 1]` domain, which is the OQ4
/// identity's storage form: a chart image whose parameter is the
/// carrier's, by construction. `frac` picks which quarter of the loop
/// — the fixture uses the first, and the planted-corruption row uses
/// the second, which is a genuinely different arc of the same locus.
fn refit(branch: &ssi::SsiBranch, frac: (f64, f64)) -> Option<Structure> {
    let Curve3::Nurbs(ref loop_carrier) = branch.carrier else {
        panic!("a rung-3 carrier is a NURBS curve")
    };
    let (d0, d1) = loop_carrier.domain();
    let pts: Vec<Point3<f64>> = (0..SAMPLES)
        .map(|i| {
            #[allow(clippy::cast_precision_loss)]
            let s = frac.0 + (frac.1 - frac.0) * (i as f64 / (SAMPLES - 1) as f64);
            loop_carrier.eval(d0 + (d1 - d0) * s)
        })
        .collect();
    let params = NurbsCurve3::<f64>::chord_parameters(&pts).ok()?;
    // The chart images, on ONE branch: `atan2` is principal, so the
    // azimuths are unwrapped continuously ONCE here, exactly as the
    // per-face loop walk chooses a branch once and never per sample
    // (the M2 PR 5 meridian finding).
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
    let carrier = NurbsCurve3::<f64>::interpolate_with_params(&pts, DEGREE, &params).ok()?;
    let image = NurbsCurve2::<f64>::interpolate_with_params(&chart, DEGREE, &params).ok()?;
    Some(Structure { carrier, image })
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
    let s = refit(&branch, (0.0, 0.25))?;
    Some(assemble(&s))
}

/// The planted corruption for the at-rest row: a cache certified —
/// honestly, through the same door — for a DIFFERENT arc of the same
/// locus. Attaching it to this edge is the sharpest test of "the
/// re-certification re-derives and never consults the stored
/// certificate": every stored number in it is true, and true about the
/// wrong carrier.
pub fn foreign_cache(built: &Built<f64>) -> PcurveCache<f64> {
    let branch = branch_or_budget().expect("the fixture already built once");
    let s = refit(&branch, (0.5, 0.75)).expect("the second quarter refits");
    let other = assemble::<f64>(&s);
    let _ = built;
    other
        .body
        .pcurve(other.he_plus)
        .expect("the foreign cache")
        .clone()
}

/// The dual lane's refusal, executed: the same fitted image offered at
/// a scalar with no bracket.
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
        Band::linear().unwrap(),
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
    let band = Band::linear().unwrap();
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
                description: EdgeGeometry::Intersection {
                    s1: cyl_key,
                    s2: sph_key,
                    witness: carrier.eval(mid),
                },
                carrier: Curve3::Nurbs(Arc::clone(&carrier)),
                param_start: t0,
                param_end: t1,
            },
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
