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
//! **The rows are the mint's.** The cylinder face's rows are minted by
//! the public pass (`mint_pcurves`): the carrier has no closed-form
//! image on the cylinder chart, so its row is the projected image, the
//! chart's inverse of the carrier (C4). The uniqueness tube that says
//! the carrier is one arc of the pair's crossing is the EDGE's: `mev`
//! certifies the rung-3 edge through the scalar's lane, which runs it
//! (C2's limb 3), and tier 3 re-derives it at rest.
//!
//! **The scaffold caveat, stated once.** The body is assembled by
//! hand: the cyl×sphere fitted-chord join lane is banked past M6. When
//! that lane lands, this row should re-anchor to a constructor-built
//! body and the hand assembly below should go.
//!
//! Nothing here invents a certificate: the edge goes in through
//! `EdgeCurve::certify`'s rung-3 gate and the rows through the mint,
//! both of which refuse typed.
#![allow(dead_code)]
// one instance per binary; no single consumer uses all of it
// Why a helper tree allows these: `crates/editor-core/tests/fixture/mod.rs`.
#![allow(clippy::unwrap_used, clippy::expect_used, clippy::panic)]
#![allow(unreachable_pub)] // why: root Cargo.toml, the `unreachable_pub` stanza

use std::sync::Arc;

use geom::Surface;
use geom::{Curve3, NurbsCurve3};
use geom_brep::ssi::{self, SsiDomain};
use geom_brep::{EdgeCurveSpec, EdgeDescriptionSpec, PcurveCache};
use geom_core::Tol;
use geom_core::{Band, Point3, Real, Vec3};
use topo::{Body, HalfEdgeKey};

pub mod arc_chain;
pub mod m7_8;

/// The fixture's cylinder: offset from the sphere's centre so the two
/// intersection loops differ wildly in size (the PR 7 planted shape).
const CYL_ORIGIN: Point3<f64> = Point3::new(0.03, 0.0, 0.0);
const CYL_RADIUS: f64 = 0.08;
const SPH_RADIUS: f64 = 1.0;

/// A built fixture: the body and the two half-edges of its rung-3 edge.
pub struct Built<T: Real> {
    pub body: Body<T>,
    pub he_plus: HalfEdgeKey,
    pub he_minus: HalfEdgeKey,
    pub carrier: Arc<NurbsCurve3<T>>,
    pub cylinder: Surface<T>,
    pub sphere: Surface<T>,
}

fn cylinder<T: Real>() -> Surface<T> {
    Surface::Cylinder {
        origin: CYL_ORIGIN.map(T::from_f64),
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

/// One certified rung-3 branch of the planted fixture.
///
/// **Memoized per process.** The trace is the expensive part of the
/// fixture, and the module's own opening line already says the
/// fixture is "built once, at any scalar": every caller here restricts
/// the SAME traced locus, so a second trace re-derives a bit-identical
/// branch (D9) and buys nothing. INVARIANT: no row asserts that two
/// INDEPENDENT traces agree; a row that ever wants two must call
/// `trace_branch` directly and say why.
///
/// nextest is process-per-test, so this only helps WITHIN one test —
/// which is exactly where the duplication is (`build` + `foreign_cache`
/// in one row).
fn branch() -> &'static ssi::SsiBranch {
    static BRANCH: std::sync::OnceLock<ssi::SsiBranch> = std::sync::OnceLock::new();
    BRANCH.get_or_init(trace_branch)
}

/// The trace itself — the full `cylinder_sphere_ssi` exhaustiveness run
/// the memo above wraps.
fn trace_branch() -> ssi::SsiBranch {
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
        Ok(out) => out.branches.into_iter().next().expect("two loops"),
        Err(e) => panic!("the planted fixture: {e}"),
    }
}

/// The branch's own carrier restricted to one sub-arc, by knot
/// insertion (exact in ℝ, so the arc is the traced one).
///
/// `frac` picks which quarter of the loop — the fixture uses the first,
/// and the planted-corruption row the third, which is a genuinely
/// different arc of the same locus and therefore the sharpest thing to
/// attach to the first one's edge.
fn restrict(branch: &ssi::SsiBranch, frac: (f64, f64)) -> NurbsCurve3<f64> {
    let Curve3::Nurbs(ref loop_carrier) = branch.carrier else {
        panic!("a rung-3 carrier is a NURBS curve")
    };
    // `fit_branch` interpolates on chord parameters, so the traced
    // carrier's domain is exactly [0, 1]. Checked rather than assumed.
    let (d0, d1) = loop_carrier.domain();
    assert!(
        d0 == 0.0 && d1 == 1.0,
        "a fitted carrier's domain is [0, 1]: [{d0}, {d1}]"
    );
    let tail = if frac.0 > 0.0 {
        loop_carrier.split_at(frac.0).expect("the carrier's sub-arc").1
    } else {
        (**loop_carrier).clone()
    };
    if frac.1 < 1.0 {
        tail.split_at(frac.1).expect("the carrier's sub-arc").0
    } else {
        tail
    }
}

fn lift3<T: Real>(c: &NurbsCurve3<f64>) -> NurbsCurve3<T> {
    let control = c.control().iter().map(|p| p.map(T::from_f64)).collect();
    NurbsCurve3::new(c.knots().clone(), control, c.weights().to_vec()).expect("lifted structure")
}

/// Build the fixture at `T`: the edge, and the rows the mint derives.
pub fn build<T>() -> Built<T>
where
    T: topo::AtRestPolicy + geom_core::Bounds,
{
    let mut built = assemble(&restrict(branch(), (0.0, 0.25)));
    topo::mint_pcurves(&mut built.body, Tol::witness()).expect("the cylinder face mints");
    built
}

/// The planted corruption for the at-rest row: a row the mint derived
/// and certified — honestly — for a DIFFERENT arc of the same locus (the
/// third quarter). Attaching it to this edge is the sharpest test of
/// "the re-certification re-derives and never consults the stored
/// certificate": every stored number in it is true, and true about the
/// wrong carrier.
pub fn foreign_cache(built: &Built<f64>) -> PcurveCache<f64> {
    let mut other = assemble::<f64>(&restrict(branch(), (0.5, 0.75)));
    topo::mint_pcurves(&mut other.body, Tol::witness()).expect("the other arc mints");
    let _ = built;
    other
        .body
        .pcurve(other.he_plus)
        .expect("the foreign row")
        .clone()
}

/// Assemble the body: the cylinder face carries the edge (its chart is
/// the well-conditioned one — azimuth about the axis, height along it),
/// the sphere is the mate the edge's own DESCRIPTION names. No row is
/// stored yet.
fn assemble<T>(arc: &NurbsCurve3<f64>) -> Built<T>
where
    T: topo::AtRestPolicy + geom_core::Bounds,
{
    let carrier = Arc::new(lift3::<T>(arc));
    let (f0, f1) = arc.domain();
    let (t0, t1) = (T::from_f64(f0), T::from_f64(f1));
    let (p0, p1) = (carrier.eval(t0), carrier.eval(t1));

    let mut body = Body::<T>::new();
    let seed = body.mvfs(p0, true).unwrap();
    let cyl_key = body
        .set_face_surface(
            seed.face,
            topo::FaceSurface::New {
                surface: cylinder::<T>(),
                sense: true,
            },
        )
        .unwrap();
    let anchor = body.mvfs(p1, true).unwrap();
    let sph_key = body
        .set_face_surface(
            anchor.face,
            topo::FaceSurface::New {
                surface: sphere::<T>(),
                sense: true,
            },
        )
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

    let edge = body.get_edge(made.edge).expect("the edge resolves");
    let (he_plus, he_minus) = (edge.he_plus, edge.he_minus);
    Built {
        body,
        he_plus,
        he_minus,
        carrier,
        cylinder: cylinder::<T>(),
        sphere: sphere::<T>(),
    }
}
