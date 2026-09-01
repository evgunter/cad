//! The approximating surface: an intensional description, a fitted
//! NURBS that stands in for it, and a private certificate binding the
//! two.
//!
//! The offset of a NURBS surface is not a NURBS — normalizing the chart
//! normal introduces a square root that breaks rationality — so the
//! kernel fits one and carries the *intent* alongside the fit. That is
//! the same triple `EdgeCurve` uses one dimension down (description,
//! uncertified spec, certified product with private fields), and the
//! same invariant: **an uncertified approximating surface is
//! unrepresentable**. [`ApproxSurface`] has no public constructor other
//! than [`ApproxSurface::certify`], which takes the certifier as an
//! argument and stores only what that certifier returned.
//!
//! # Why the certifier is injected
//!
//! The certificate's derivation (hull bounds over a span schedule, a
//! regularity floor, a curvature-reach meter) lives one crate up, in
//! `geom_brep::offset_fit` — it needs the ring/interval composite
//! machinery this crate does not carry, and it is `f64`-only while
//! [`Surface`](crate::Surface) is generic. So the door takes the
//! capability as a parameter, exactly as edge certification takes its
//! plane × NURBS lane: a caller that can derive the certificate hands
//! one in, and there is no second door that skips it.
//!
//! # Why the base is owned, not an arena key
//!
//! `EdgeDescription::Intersection` names its surfaces by arena key, and
//! that is the precedent this description would otherwise follow. Two
//! concrete obstructions rule it out here:
//!
//! - **Layering.** `SurfaceKey` is `geom_brep`'s, one crate above the
//!   [`Surface`](crate::Surface) enum this description is stored
//!   inside. A key-carrying description inverts the dependency.
//! - **Self-containment.** An edge description is only ever read with
//!   its body in hand (certification is a body-level pass). A surface
//!   is not: `Surface` values are cloned out of arenas and evaluated,
//!   boxed, tessellated and exported with no arena anywhere in reach —
//!   `mesh`, `step-export` and `geom`'s own evaluators all take a bare
//!   `&Surface<T>`. A keyed base would make those paths unable to read
//!   the description at all, and would leave a dangling handle behind
//!   every clone that outlives its arena.
//!
//! So the base travels as an `Arc<NurbsSurface<T>>`: shared, immutable,
//! cheap to clone, and readable wherever the surface is.
//!
//! # The base is NURBS, here
//!
//! [`SurfaceDescription::Offset`]'s base is a NURBS surface, not a
//! `Surface`. Analytic kinds are closed under offset — plane, cylinder,
//! cone, sphere and torus all mint their offset exactly through
//! `geom_brep::offset_surface` — so an analytic base never needs a fit
//! and never reaches this type. Making that structural (rather than a
//! refusal inside the fit door) is what dissolves the apex-window
//! question for an approximating surface: no cone-based description can
//! be built, so there is no apex band to window around. Mixed analytic
//! surgery is the face-replacement problem, and it lives with the
//! consumer that performs it.

use std::sync::Arc;

use geom_core::Real;

use super::nurbs::NurbsSurface;

/// The `(u, v)` rectangle an approximating surface's fit is certified
/// over — the base's own chart domain, in parameter units.
///
/// Knot vectors are `f64` whatever the surface's scalar is, so this
/// window is `f64` too: it names parameters, not geometry.
///
/// **Named for the surface, not the chart**, because `geom_brep` has a
/// generic `ChartWindow<T>` of its own with a different job (the
/// pcurve lane's trim window); two types under one name across two
/// crates is a reader's trap, and this one is the newcomer.
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct ApproxWindow {
    /// The u-direction interval `(u₀, u₁)`.
    pub u: (f64, f64),
    /// The v-direction interval `(v₀, v₁)`.
    pub v: (f64, f64),
}

impl ApproxWindow {
    /// The window a NURBS surface's own knot vectors span.
    pub fn of<T: Real>(s: &NurbsSurface<T>) -> Self {
        Self {
            u: s.knots_u().domain(),
            v: s.knots_v().domain(),
        }
    }
}

/// The two-limb certificate of a fitted offset surface. Every field is
/// a bound, in metres, that the corresponding limb proved — or the
/// structure it proved it over.
///
/// The derivation is `geom_brep::offset_fit`'s; the record lives here
/// because [`ApproxSurface`] stores it and [`Surface`](crate::Surface)
/// stores that.
#[derive(Clone, Copy, Debug)]
pub struct OffsetCertificate {
    /// The signed offset distance the claim is about, in metres.
    pub distance: f64,
    /// How many `(u,v)` cells the span schedule has.
    pub cells: u32,
    /// The per-direction on-locus sample count inside each cell.
    pub samples: u32,
    /// Limb 1: the largest on-locus residual over the schedule, in
    /// metres (the sampled max — it steers, it does not certify).
    pub on_locus_max: f64,
    /// Limb 2: the certified **sup-norm** bound over the whole chart
    /// rectangle, in metres. This is the number that certifies.
    pub hull_sup: f64,
    /// The regularity floor the normal enclosure rests on — a
    /// certified lower bound on `‖S_u × S_v‖` (m² per unit parameter
    /// area).
    pub normal_floor: f64,
    /// The collapse meter's certified fold radius on the folding side,
    /// in metres.
    pub curvature_reach: f64,
    /// How many refinement rounds the fit that this certificate is
    /// about took to reach its tolerance.
    ///
    /// **Provenance of the FIT, not a limb**, and the one field a
    /// re-derivation cannot recompute: re-measuring a finished fit runs
    /// no refinement. So [`crate::ApproxSurface`]'s stored copy carries
    /// the MINT loop's count across the re-derivation the storage door
    /// makes (see `geom_brep::approx_offset_surface`), and a bare
    /// `certify_offset` — which has no loop behind it — reports `0`.
    pub rounds: u32,
}

/// What an approximating surface **is**, independent of any fit — the
/// intensional layer, and the authority a re-derivation measures
/// against.
///
/// One inhabitant today. The canal blend is the next, and the enum is
/// closed for the same reason `Surface` is (D3): a second inhabitant
/// must make every consumer say what it does with it.
#[derive(Clone, Debug)]
pub enum SurfaceDescription<T: Real> {
    /// The normal pushforward `S(u, v) + d·n(u, v)` of a NURBS base at
    /// signed distance `d`, with `n` the base's unit chart normal
    /// (`∂u × ∂v` normalized — `geom`'s derived normal, so the sign is
    /// the base's parameterization's).
    Offset {
        /// The base surface. Shared and immutable; see the module docs
        /// on why it is owned rather than an arena key, and why it is
        /// NURBS rather than a `Surface`.
        base: Arc<NurbsSurface<T>>,
        /// The signed offset distance in metres.
        d: T,
    },
}

impl<T: Real> SurfaceDescription<T> {
    /// The same description read at another scalar: the base through
    /// [`NurbsSurface::map_scalar`], the distance through `f`. A
    /// structural map, exact whenever `f` is.
    pub fn map_scalar<U: Real>(&self, f: impl Fn(T) -> U) -> SurfaceDescription<U> {
        match self {
            SurfaceDescription::Offset { base, d } => SurfaceDescription::Offset {
                base: Arc::new(base.map_scalar(&f)),
                d: f(*d),
            },
        }
    }
}

/// The uncertified input to [`ApproxSurface::certify`]: the intent, the
/// fit that claims to realize it, the window the claim is made over,
/// and the tolerance it claims. Plain data — the certified product is
/// [`ApproxSurface`].
#[derive(Clone, Debug)]
pub struct SurfaceSpec<T: Real> {
    /// The intensional description (authoritative).
    pub description: SurfaceDescription<T>,
    /// The fitted surface that stands in for it.
    pub fit: NurbsSurface<T>,
    /// The `(u, v)` rectangle the claim is made over. Passed to the
    /// certifier, which decides whether it can honour it.
    pub window: ApproxWindow,
    /// The precision tolerance the claim is against, in metres (D4's
    /// ε_precision, not ε_input).
    pub tolerance: f64,
}

/// A certified approximating surface: description, fit, window,
/// tolerance and the [`OffsetCertificate`] of the run that bound them
/// together.
///
/// Fields are private and the only constructor from uncertified parts
/// is [`ApproxSurface::certify`], so an uncertified value is
/// unrepresentable (D4 ¶2 made structural — the `EdgeCurve` invariant,
/// lifted one dimension). [`ApproxSurface::map_scalar`] only re-reads
/// a value that already passed that door at another scalar.
///
/// **The certificate is provenance, not authority.** Tier-3 validation
/// re-derives it against the description on every call and never
/// consults the stored copy; the stored copy is what the construction
/// run measured, kept so a consumer can report it.
#[derive(Clone, Debug)]
pub struct ApproxSurface<T: Real> {
    description: SurfaceDescription<T>,
    fit: NurbsSurface<T>,
    window: ApproxWindow,
    tolerance: f64,
    certificate: OffsetCertificate,
}

impl<T: Real> ApproxSurface<T> {
    /// **The only door.** Runs `certifier` against the spec's
    /// description and fit, and stores the certificate it returned.
    ///
    /// The certifier's refusal propagates verbatim — this door neither
    /// interprets it nor works around it, so a capability the
    /// certification stack does not have (a rational fit, today) stays
    /// a refusal all the way out.
    ///
    /// **The window is handed to the certifier**, not merely stored
    /// beside its answer: the today's certifier
    /// (`geom_brep::certify_offset`) derives over the base's whole
    /// chart rectangle and so can check the window it is asked for
    /// rather than have one attested at it. A certifier that ever
    /// bounds a SUB-rectangle needs this argument to know which.
    ///
    /// # Errors
    ///
    /// Whatever `certifier` returns as its error, unchanged.
    pub fn certify<E>(
        spec: SurfaceSpec<T>,
        certifier: impl FnOnce(
            &SurfaceDescription<T>,
            &NurbsSurface<T>,
            ApproxWindow,
            f64,
        ) -> Result<OffsetCertificate, E>,
    ) -> Result<Self, E> {
        let certificate = certifier(&spec.description, &spec.fit, spec.window, spec.tolerance)?;
        Ok(Self {
            description: spec.description,
            fit: spec.fit,
            window: spec.window,
            tolerance: spec.tolerance,
            certificate,
        })
    }

    /// The intensional description — what this surface is *meant* to
    /// be, and what a re-derivation measures against.
    pub fn description(&self) -> &SurfaceDescription<T> {
        &self.description
    }

    /// The fitted NURBS that stands in for the description. Every
    /// evaluation, box and tessellation of an approximating surface
    /// goes through this: the fit **is** the geometry, and the
    /// certificate bounds its distance from the intent.
    pub fn fit(&self) -> &NurbsSurface<T> {
        &self.fit
    }

    /// The `(u, v)` rectangle the certificate covers.
    pub fn window(&self) -> ApproxWindow {
        self.window
    }

    /// The precision tolerance the certificate was classified against,
    /// in metres — the claim a re-derivation must re-establish.
    pub fn tolerance(&self) -> f64 {
        self.tolerance
    }

    /// The construction-time certificate. Provenance: the validator
    /// re-derives rather than reading this (see the type docs).
    pub fn certificate(&self) -> &OffsetCertificate {
        &self.certificate
    }

    /// The same certified surface read at another scalar: the
    /// description and the fit through their own `map_scalar`s, the
    /// window, tolerance and certificate carried over verbatim.
    ///
    /// **Not a second door.** The certificate is provenance, not
    /// authority (type docs): it records what the construction run
    /// measured about this geometry, and a structural map of that
    /// geometry — exact for every scalar embedding, `Real::from_f64`
    /// or `Dual::constant` — is the same geometry, so the record still
    /// describes it. The validator re-derives against the description
    /// on every call whatever scalar it reads, so a lift can neither
    /// mint a claim nor launder one.
    pub fn map_scalar<U: Real>(&self, f: impl Fn(T) -> U) -> ApproxSurface<U> {
        ApproxSurface {
            description: self.description.map_scalar(&f),
            fit: self.fit.map_scalar(&f),
            window: self.window,
            tolerance: self.tolerance,
            certificate: self.certificate,
        }
    }

    /// The uncertified spec this surface would certify from — the
    /// input a re-derivation reconstructs.
    pub fn spec(&self) -> SurfaceSpec<T> {
        SurfaceSpec {
            description: self.description.clone(),
            fit: self.fit.clone(),
            window: self.window,
            tolerance: self.tolerance,
        }
    }
}
