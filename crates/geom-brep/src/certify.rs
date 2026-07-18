//! Certified carriers: the D4 ¶2 attachment gate for edge geometry.
//!
//! An edge's concrete 3-D curve (its *carrier*) is a derived cache of
//! its intensional description ([`crate::EdgeGeometry`]). This module
//! is the only way to marry the two: [`EdgeCurve::certify`] takes an
//! uncertified [`EdgeCurveSpec`] plus the edge's endpoint points and
//! either returns a certified [`EdgeCurve`] — whose fields are private,
//! so **an uncertified carrier is unrepresentable** — or fails with a
//! typed [`CertifyError`] (operation-time enforcement, D4 ¶3). The
//! tier-3 validator re-runs the same checks at rest
//! ([`EdgeCurve::recertify`]).
//!
//! # The deterministic sampling schedule (D9)
//!
//! Certification is closed-form residual evaluation at a **fixed
//! schedule**: [`CERT_SAMPLES`] = 9 parameters
//! `t_i = t₀ + (t₁ − t₀)·(i/8)`, i = 0…8 — endpoints included, the
//! fractions exact dyadics, the arithmetic a fixed association order —
//! so two runs over the same body produce byte-identical
//! [`Certificate`]s. Interior samples (i = 1…7) additionally carry the
//! transversality check for `Intersection` descriptions (endpoint
//! vertices may legitimately sit on chart/surface singularities — cone
//! apexes, sphere poles — where angular classification honestly
//! poisons; residual checks, which never need a gradient, still run at
//! the endpoints).
//!
//! # Dimensional honesty
//!
//! Every classified residual is a **length in meters** against the
//! run's linear band: point-to-point distances directly, implicit
//! surface residuals in the linearized forms of [`crate::implicit`],
//! angular transversality as displacement through its lever arm
//! ([`crate::classify_dihedral`]'s margin). No squared bands, no
//! dimensionless margins.
//!
//! # The stored parameter interval — a certified cache, not an authority
//!
//! `geom-curves`' ratified convention stands: an edge's bounds are
//! *derived from its vertices* — the authority is the vertex geometry.
//! The [`EdgeCurve`] nevertheless **stores** the parameter interval,
//! as a certified derived cache in exactly the carrier's sense: checks
//! 1–2 of every certification pin `carrier(t₀)`/`carrier(t₁)` to the
//! endpoint points within ε, at attachment and again at tier 3. Storing
//! the certified interval is what keeps periodic carriers total
//! (a full-period scaffolding edge's `(0, τ)` is not recoverable from
//! its coincident endpoints) and keeps bound recovery evaluation-free
//! (no atan2 branch selection). It can never disagree with the vertices
//! by more than ε without failing loudly — a cache, never a peer.

use geom_core::{Band, BandError, Decide, Indeterminate, Point3, Real, Sign};
use geom_curves::Curve3;
use geom_surfaces::Surface;

use crate::dihedral::{DihedralClass, classify_dihedral, decide};
use crate::edge_geometry::EdgeGeometry;
use crate::implicit::{implicit_residual, seam_frame};
use crate::keys::SurfaceKey;

/// The fixed certification sample count (module docs): 9 uniform
/// parameters, endpoints included.
pub const CERT_SAMPLES: u32 = 9;

/// Which certification check a [`CertifyError`] names — the residual
/// taxonomy, one variant per documented check.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum CertCheck {
    /// `|carrier(t₀) − start point|` (check 1).
    EndpointStart,
    /// `|carrier(t₁) − end point|` (check 2).
    EndpointEnd,
    /// Intersection: implicit residual against `s1` at a sample.
    Surface1Residual,
    /// Intersection: implicit residual against `s2` at a sample.
    Surface2Residual,
    /// Intersection: the witness point's residual against `s1`.
    WitnessSurface1,
    /// Intersection: the witness point's residual against `s2`.
    WitnessSurface2,
    /// Intersection: the transversality margin at an interior sample
    /// (the dihedral displacement margin — must be definitely
    /// transverse).
    Transversality,
    /// MappedCurve: `|carrier(t_i) − description(s_i)|` at a sample.
    MappedSource,
    /// Seam: implicit residual against the seam's surface at a sample.
    SeamSurface,
    /// Seam: the out-of-halfplane component `|w · v_ref|` at a sample.
    SeamHalfplane,
    /// Seam: the wrong-side excess `max(0, −w · u_ref)` at a sample
    /// (distinguishes the seam from the antipodal meridian).
    SeamSide,
}

/// Typed certification failure (D4 ¶3): actionable, closed enum. The
/// body-side attachment gates (`topo`'s operators and setters) wrap
/// this; the tier-3 validator carries it inside its own error.
///
/// Magnitude diagnostics ride on [`CertifyError::Escalated`]'s
/// [`Indeterminate`] (band + margin view). Definite exceedances name
/// the check and sample; the magnitude itself is scalar-typed and is
/// not extracted into the error (no `f64`-projection of a generic `T`
/// exists for every lane — the Dual lane in particular).
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum CertifyError {
    /// A surface key in the description did not resolve in the owning
    /// body (stale, or the surface does not exist yet — attach the
    /// intrinsic description once its surfaces are in the arena).
    UnresolvedSurface {
        /// The unresolved key.
        key: SurfaceKey,
    },
    /// The carrier (or a described surface) is the `Nurbs`
    /// representable-unimplemented placeholder: nothing can be
    /// certified against it at M2.
    Unimplemented,
    /// An `Intersection` description names one surface twice — a
    /// same-surface locus is a `Seam`, never an intersection.
    IntersectionSameSurface {
        /// The doubly-named key.
        key: SurfaceKey,
    },
    /// A `Seam` description on a non-periodic surface (a plane) — no
    /// seam exists.
    SeamOnNonPeriodic,
    /// A residual definitely exceeded the escalation threshold: the
    /// cache does not represent the description (D4 ¶2's `residual ≤ ε`
    /// kernel invariant violated beyond doubt).
    ResidualExceeded {
        /// The check that failed.
        check: CertCheck,
        /// The schedule sample index (0…[`CERT_SAMPLES`]−1; 0 for the
        /// witness/endpoint checks).
        sample: u32,
    },
    /// `Intersection` only: the tangent planes coincide at a sample —
    /// the transversality precondition fails, so the locus is not an
    /// `Intersection` (a tangential contact is `TangencyLocus`
    /// territory, M5; a seam is `Seam`).
    NotTransverse {
        /// The interior sample index.
        sample: u32,
    },
    /// A classification escalated: the margin landed in the sliver band
    /// or was poisoned (D4 ¶3's escalate-never-guess).
    Escalated {
        /// The check that escalated.
        check: CertCheck,
        /// The schedule sample index.
        sample: u32,
        /// The classifier's diagnostic (band + margin view).
        cause: Indeterminate,
    },
    /// The linear band could not be built from the run's tolerance
    /// (absurd ε — see `Band::linear`'s error docs).
    Band(BandError),
}

impl core::fmt::Display for CertifyError {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        match self {
            Self::UnresolvedSurface { key } => {
                write!(f, "certification: surface key {key:?} does not resolve")
            }
            Self::Unimplemented => write!(
                f,
                "certification: Nurbs (representable-unimplemented) kinds cannot be \
                 certified at M2"
            ),
            Self::IntersectionSameSurface { key } => write!(
                f,
                "certification: Intersection names surface {key:?} twice (a same-surface \
                 locus is a Seam)"
            ),
            Self::SeamOnNonPeriodic => write!(
                f,
                "certification: Seam described on a non-periodic surface (a plane has no \
                 seam)"
            ),
            Self::ResidualExceeded { check, sample } => write!(
                f,
                "certification: {check:?} residual at sample {sample} definitely exceeds \
                 the tolerance band (the cache does not represent the description, D4 ¶2)"
            ),
            Self::NotTransverse { sample } => write!(
                f,
                "certification: tangent planes coincide at interior sample {sample} — the \
                 Intersection transversality precondition fails (D2)"
            ),
            Self::Escalated {
                check,
                sample,
                cause,
            } => write!(
                f,
                "certification: {check:?} at sample {sample} escalated: {cause}"
            ),
            Self::Band(e) => write!(f, "certification: {e}"),
        }
    }
}

impl std::error::Error for CertifyError {}

/// The uncertified input to [`EdgeCurve::certify`]: description,
/// carrier cache, and the carrier-parameter interval, exactly as the
/// construction prepared them. Plain data — the certified product is
/// [`EdgeCurve`].
#[derive(Clone, Copy, Debug)]
pub struct EdgeCurveSpec<T: Real> {
    /// The intensional description (authoritative).
    pub description: EdgeGeometry<T>,
    /// The carrier cache to certify against it.
    pub carrier: Curve3<T>,
    /// Carrier parameter at `start(he_plus)` — the `he_plus` forward
    /// contract's t₀ (increasing parameter runs start → end).
    pub param_start: T,
    /// Carrier parameter at `end(he_plus)` — t₁.
    pub param_end: T,
}

/// The certification record stored with a certified carrier: the
/// schedule that ran and the worst distance residual it observed
/// (meters). Byte-identical across replays of the same construction
/// (D9) — under test via its `Debug` form.
#[derive(Clone, Copy, Debug)]
pub struct Certificate<T: Real> {
    /// The sample count of the schedule that ran ([`CERT_SAMPLES`]).
    pub samples: u32,
    /// The maximum magnitude over every classified **distance** residual
    /// (endpoint, surface, mapped-source, seam checks; transversality
    /// margins are clearance margins, not residuals, and are excluded).
    /// Certified ≤ ε by construction.
    pub max_residual: T,
}

/// A certified edge carrier: the intensional description, its cached
/// [`Curve3`] carrier with the certified parameter interval, and the
/// [`Certificate`] of the attachment-time run. Constructible only
/// through [`EdgeCurve::certify`] — fields are private so an
/// uncertified value is unrepresentable (D4 ¶2 made structural).
#[derive(Clone, Copy, Debug)]
pub struct EdgeCurve<T: Real> {
    description: EdgeGeometry<T>,
    carrier: Curve3<T>,
    param_start: T,
    param_end: T,
    certificate: Certificate<T>,
}

impl<T: Decide> EdgeCurve<T> {
    /// Certifies `spec` against the edge's endpoint points and the
    /// owning body's surfaces, returning the certified carrier.
    ///
    /// `start`/`end` are the points of `start(he_plus)` and
    /// `end(he_plus)` (the `he_plus` forward contract); `surfaces`
    /// resolves the description's arena keys (injected by the owning
    /// body — this layer never touches arenas, see [`crate::keys`]);
    /// `band` is the run's linear band (callers build it once at
    /// operation entry via `Band::linear()`).
    ///
    /// The check sequence (fixed order, D9; every check's margin is
    /// meters against `band`):
    ///
    /// 1. Implementedness: the carrier is not `Nurbs`; described
    ///    surfaces resolve and are not `Nurbs`; `Intersection`'s two
    ///    surfaces are distinct; `Seam`'s surface is periodic.
    /// 2. Endpoint pinning: `|carrier(t₀) − start| ≤ ε`,
    ///    `|carrier(t₁) − end| ≤ ε`.
    /// 3. Per-sample description residuals, samples i = 0…8 in order
    ///    (per-sample check order as listed in [`CertCheck`]):
    ///    - `Intersection`: implicit residual vs `s1`, then `s2`; at
    ///      interior samples (i = 1…7) the transversality margin
    ///      (definitely transverse required).
    ///    - `MappedCurve`: `|carrier(t_i) − description(i/8)|`.
    ///    - `Seam`: implicit residual, halfplane residual `|w·v_ref|`,
    ///      wrong-side excess `max(0, −w·u_ref)`.
    /// 4. `Intersection`: the witness's implicit residuals vs both
    ///    surfaces (the witness must lie on the locus it selects;
    ///    *which component* it selects is unverifiable before marching
    ///    exists — M3).
    ///
    /// # Errors
    ///
    /// The first failing check, as a typed [`CertifyError`] (D4 ¶3).
    pub fn certify(
        spec: EdgeCurveSpec<T>,
        start: Point3<T>,
        end: Point3<T>,
        surfaces: impl Fn(SurfaceKey) -> Option<Surface<T>>,
        band: Band,
    ) -> Result<Self, CertifyError> {
        let certificate = run_checks(&spec, start, end, &surfaces, band)?;
        Ok(Self {
            description: spec.description,
            carrier: spec.carrier,
            param_start: spec.param_start,
            param_end: spec.param_end,
            certificate,
        })
    }

    /// Re-runs the full certification of this carrier at rest — the
    /// tier-3 validator's per-edge pass. Same checks, same schedule,
    /// same errors as [`EdgeCurve::certify`]; the stored certificate is
    /// not consulted (re-certification re-derives, it does not trust).
    ///
    /// # Errors
    ///
    /// As [`EdgeCurve::certify`].
    pub fn recertify(
        &self,
        start: Point3<T>,
        end: Point3<T>,
        surfaces: impl Fn(SurfaceKey) -> Option<Surface<T>>,
        band: Band,
    ) -> Result<Certificate<T>, CertifyError> {
        run_checks(&self.spec(), start, end, &surfaces, band)
    }
}

impl<T: Real> EdgeCurve<T> {
    /// The intensional description (authoritative, D2).
    pub fn description(&self) -> &EdgeGeometry<T> {
        &self.description
    }

    /// The cached carrier curve (a certified derived cache, D4 ¶2).
    pub fn carrier(&self) -> &Curve3<T> {
        &self.carrier
    }

    /// The certified carrier-parameter interval `(t₀, t₁)` —
    /// `he_plus`-forward, a certified cache of the vertex-derived
    /// bounds (module docs).
    pub fn params(&self) -> (T, T) {
        (self.param_start, self.param_end)
    }

    /// The attachment-time certification record.
    pub fn certificate(&self) -> &Certificate<T> {
        &self.certificate
    }

    /// The carrier parameter at schedule sample `i` (i ∈ 0…8):
    /// `t₀ + (t₁ − t₀)·(i/8)`, the module-doc schedule. Exposed so the
    /// tier-3 validator samples the *same* parameters the certification
    /// did (D9).
    pub fn sample_param(&self, i: u32) -> T {
        sample_param(self.param_start, self.param_end, i)
    }

    /// This carrier's spec view (for re-certification).
    fn spec(&self) -> EdgeCurveSpec<T> {
        EdgeCurveSpec {
            description: self.description,
            carrier: self.carrier,
            param_start: self.param_start,
            param_end: self.param_end,
        }
    }
}

/// The schedule parameter `t₀ + (t₁ − t₀)·(i/8)` (exact dyadic
/// fraction; fixed association order, D9).
fn sample_param<T: Real>(t0: T, t1: T, i: u32) -> T {
    let frac = T::from_f64(f64::from(i) / f64::from(CERT_SAMPLES - 1));
    t0 + (t1 - t0) * frac
}

/// Folds a residual into the running max and classifies it: must be
/// coincident with zero (|r| ≤ ε). Positive/Negative beyond the band ⇒
/// [`CertifyError::ResidualExceeded`]; in-band or poisoned ⇒
/// [`CertifyError::Escalated`].
fn check_residual<T: Decide>(
    name: &'static str,
    check: CertCheck,
    sample: u32,
    residual: T,
    band: Band,
    max_residual: &mut T,
) -> Result<(), CertifyError> {
    *max_residual = max_residual.max(residual.abs());
    match decide(name, residual, band) {
        Ok(Sign::Zero) => Ok(()),
        Ok(Sign::Positive | Sign::Negative) => {
            Err(CertifyError::ResidualExceeded { check, sample })
        }
        Err(cause) => Err(CertifyError::Escalated {
            check,
            sample,
            cause,
        }),
    }
}

/// The shared certification engine (check sequence documented on
/// [`EdgeCurve::certify`]).
fn run_checks<T: Decide>(
    spec: &EdgeCurveSpec<T>,
    start: Point3<T>,
    end: Point3<T>,
    surfaces: &impl Fn(SurfaceKey) -> Option<Surface<T>>,
    band: Band,
) -> Result<Certificate<T>, CertifyError> {
    // ---- Check 1: implementedness / description well-formedness. ----
    if matches!(spec.carrier, Curve3::Nurbs) {
        return Err(CertifyError::Unimplemented);
    }
    let resolve = |key: SurfaceKey| -> Result<Surface<T>, CertifyError> {
        let s = surfaces(key).ok_or(CertifyError::UnresolvedSurface { key })?;
        if matches!(s, Surface::Nurbs) {
            return Err(CertifyError::Unimplemented);
        }
        Ok(s)
    };
    enum Resolved<T: Real> {
        Intersection {
            surf1: Surface<T>,
            surf2: Surface<T>,
            witness: Point3<T>,
        },
        Mapped(crate::edge_geometry::MappedCurve<T>),
        Seam(Surface<T>),
    }
    let resolved = match spec.description {
        EdgeGeometry::Intersection { s1, s2, witness } => {
            if s1 == s2 {
                return Err(CertifyError::IntersectionSameSurface { key: s1 });
            }
            Resolved::Intersection {
                surf1: resolve(s1)?,
                surf2: resolve(s2)?,
                witness,
            }
        }
        EdgeGeometry::MappedCurve(mc) => Resolved::Mapped(mc),
        EdgeGeometry::Seam { surface } => {
            let s = resolve(surface)?;
            // Periodicity is structural: the plane is the one
            // non-periodic analytic kind (Nurbs already rejected).
            if matches!(s, Surface::Plane { .. }) {
                return Err(CertifyError::SeamOnNonPeriodic);
            }
            Resolved::Seam(s)
        }
    };

    let mut max_residual = T::zero();
    let (t0, t1) = (spec.param_start, spec.param_end);

    // ---- Check 2: endpoint pinning. ----
    check_residual(
        "carrier_endpoint_start",
        CertCheck::EndpointStart,
        0,
        spec.carrier.eval(t0).distance(start),
        band,
        &mut max_residual,
    )?;
    check_residual(
        "carrier_endpoint_end",
        CertCheck::EndpointEnd,
        CERT_SAMPLES - 1,
        spec.carrier.eval(t1).distance(end),
        band,
        &mut max_residual,
    )?;

    // ---- Check 3: per-sample description residuals. ----
    // The chord is the transversality extent arm (dihedral module docs).
    let chord = start.distance(end);
    for i in 0..CERT_SAMPLES {
        let p = spec.carrier.eval(sample_param(t0, t1, i));
        match &resolved {
            Resolved::Intersection { surf1, surf2, .. } => {
                check_residual(
                    "carrier_on_surface_1",
                    CertCheck::Surface1Residual,
                    i,
                    implicit_residual(surf1, p),
                    band,
                    &mut max_residual,
                )?;
                check_residual(
                    "carrier_on_surface_2",
                    CertCheck::Surface2Residual,
                    i,
                    implicit_residual(surf2, p),
                    band,
                    &mut max_residual,
                )?;
                if i > 0 && i < CERT_SAMPLES - 1 {
                    match classify_dihedral(surf1, surf2, p, chord, band) {
                        Ok(DihedralClass::Transverse) => {}
                        Ok(DihedralClass::Smooth) => {
                            return Err(CertifyError::NotTransverse { sample: i });
                        }
                        Err(cause) => {
                            return Err(CertifyError::Escalated {
                                check: CertCheck::Transversality,
                                sample: i,
                                cause,
                            });
                        }
                    }
                }
            }
            Resolved::Mapped(mc) => {
                let s = T::from_f64(f64::from(i) / f64::from(CERT_SAMPLES - 1));
                check_residual(
                    "carrier_matches_mapped_source",
                    CertCheck::MappedSource,
                    i,
                    p.distance(mc.eval(s)),
                    band,
                    &mut max_residual,
                )?;
            }
            Resolved::Seam(s) => {
                check_residual(
                    "carrier_on_seam_surface",
                    CertCheck::SeamSurface,
                    i,
                    implicit_residual(s, p),
                    band,
                    &mut max_residual,
                )?;
                // seam_frame is Some: plane and Nurbs were rejected in
                // check 1, and every remaining kind is axisymmetric.
                if let Some((w, u_ref, v_ref)) = seam_frame(s, p) {
                    check_residual(
                        "carrier_in_seam_halfplane",
                        CertCheck::SeamHalfplane,
                        i,
                        w.dot(v_ref),
                        band,
                        &mut max_residual,
                    )?;
                    check_residual(
                        "carrier_on_seam_side",
                        CertCheck::SeamSide,
                        i,
                        (T::zero() - w.dot(u_ref)).max(T::zero()),
                        band,
                        &mut max_residual,
                    )?;
                }
            }
        }
    }

    // ---- Check 4: witness residuals (Intersection). ----
    if let Resolved::Intersection {
        surf1,
        surf2,
        witness,
    } = &resolved
    {
        check_residual(
            "witness_on_surface_1",
            CertCheck::WitnessSurface1,
            0,
            implicit_residual(surf1, *witness),
            band,
            &mut max_residual,
        )?;
        check_residual(
            "witness_on_surface_2",
            CertCheck::WitnessSurface2,
            0,
            implicit_residual(surf2, *witness),
            band,
            &mut max_residual,
        )?;
    }

    Ok(Certificate {
        samples: CERT_SAMPLES,
        max_residual,
    })
}

#[cfg(test)]
#[allow(clippy::unwrap_used, clippy::expect_used, clippy::panic)]
mod tests {
    use geom_core::{Affine3, Point2, Tolerance, Vec3};

    use crate::edge_geometry::{MappedCurve, SketchSegment};

    use super::*;

    fn band() -> Band {
        Band::linear().unwrap()
    }

    fn eps() -> f64 {
        Tolerance::get().eps
    }

    /// A resolver over a tiny fixed table (keys minted through a local
    /// slotmap, mirroring how a Body resolves).
    fn table(
        surfs: Vec<Surface<f64>>,
    ) -> (Vec<SurfaceKey>, impl Fn(SurfaceKey) -> Option<Surface<f64>>) {
        let mut map: slotmap::SlotMap<SurfaceKey, Surface<f64>> = slotmap::SlotMap::with_key();
        let keys: Vec<SurfaceKey> = surfs.into_iter().map(|s| map.insert(s)).collect();
        (keys, move |k| map.get(k).copied())
    }

    fn line_spec(p0: Point3<f64>, p1: Point3<f64>) -> EdgeCurveSpec<f64> {
        let len = p0.distance(p1);
        EdgeCurveSpec {
            description: EdgeGeometry::MappedCurve(MappedCurve::ExtrudedPoint {
                point: Point2::new(0.0, 0.0),
                place: Affine3::translation(p0 - Point3::origin()),
                vec: p1 - p0,
            }),
            carrier: Curve3::Line {
                origin: p0,
                dir: (p1 - p0) / len,
            },
            param_start: 0.0,
            param_end: len,
        }
    }

    #[test]
    fn mapped_line_certifies_and_is_deterministic() {
        let p0 = Point3::new(0.25, -1.0, 2.0);
        let p1 = Point3::new(1.25, 0.5, 3.5);
        let spec = line_spec(p0, p1);
        let a = EdgeCurve::certify(spec, p0, p1, |_| None, band()).unwrap();
        let b = EdgeCurve::certify(spec, p0, p1, |_| None, band()).unwrap();
        // D9: byte-identical certification records across runs.
        assert_eq!(
            format!("{:?}", a.certificate()),
            format!("{:?}", b.certificate())
        );
        assert_eq!(a.certificate().samples, CERT_SAMPLES);
        assert!(a.certificate().max_residual <= eps());
        // Recertification at rest agrees.
        a.recertify(p0, p1, |_| None, band()).unwrap();
    }

    #[test]
    fn wrong_cache_is_rejected() {
        // A carrier offset well beyond the escalation band (100·ε ≥ K·ε
        // at every CI ε row) must be rejected: the cache does not
        // represent the description.
        let p0 = Point3::new(0.0, 0.0, 0.0);
        let p1 = Point3::new(1.0, 0.0, 0.0);
        let mut spec = line_spec(p0, p1);
        let off = 100.0 * eps();
        spec.carrier = Curve3::Line {
            origin: Point3::new(0.0, off, 0.0),
            dir: Vec3::unit_x(),
        };
        let err = EdgeCurve::certify(spec, p0, p1, |_| None, band()).unwrap_err();
        assert_eq!(
            err,
            CertifyError::ResidualExceeded {
                check: CertCheck::EndpointStart,
                sample: 0
            }
        );
        // An in-band offset (3·ε) escalates instead of passing.
        let mut spec = line_spec(p0, p1);
        spec.carrier = Curve3::Line {
            origin: Point3::new(0.0, 3.0 * eps(), 0.0),
            dir: Vec3::unit_x(),
        };
        let err = EdgeCurve::certify(spec, p0, p1, |_| None, band()).unwrap_err();
        assert!(matches!(err, CertifyError::Escalated { .. }), "{err:?}");
    }

    #[test]
    fn intersection_certifies_against_both_planes() {
        // The unit-cube edge x ∈ [0,1], y = 0, z = 0 as the intersection
        // of the bottom plane (z = 0) and front plane (y = 0).
        let (keys, lookup) = table(vec![
            Surface::Plane {
                origin: Point3::origin(),
                normal: Vec3::unit_z(),
                u_ref: Vec3::unit_x(),
            },
            Surface::Plane {
                origin: Point3::origin(),
                normal: Vec3::unit_y(),
                u_ref: Vec3::unit_x(),
            },
        ]);
        let p0 = Point3::origin();
        let p1 = Point3::new(1.0, 0.0, 0.0);
        let spec = EdgeCurveSpec {
            description: EdgeGeometry::Intersection {
                s1: keys[0],
                s2: keys[1],
                witness: Point3::new(0.5, 0.0, 0.0),
            },
            carrier: Curve3::Line {
                origin: p0,
                dir: Vec3::unit_x(),
            },
            param_start: 0.0,
            param_end: 1.0,
        };
        EdgeCurve::certify(spec, p0, p1, &lookup, band()).unwrap();

        // Both-surface teeth: a carrier lying ON s1 but definitely off
        // s2 fails the s2 residual specifically.
        let mut bad = spec;
        bad.carrier = Curve3::Line {
            origin: Point3::new(0.0, 0.5, 0.0), // on z = 0, off y = 0
            dir: Vec3::unit_x(),
        };
        let err = EdgeCurve::certify(
            bad,
            Point3::new(0.0, 0.5, 0.0),
            Point3::new(1.0, 0.5, 0.0),
            &lookup,
            band(),
        )
        .unwrap_err();
        assert_eq!(
            err,
            CertifyError::ResidualExceeded {
                check: CertCheck::Surface2Residual,
                sample: 0
            }
        );

        // A displaced witness fails the witness checks.
        let mut bad = spec;
        bad.description = EdgeGeometry::Intersection {
            s1: keys[0],
            s2: keys[1],
            witness: Point3::new(0.5, 0.0, 0.25),
        };
        let err = EdgeCurve::certify(bad, p0, p1, &lookup, band()).unwrap_err();
        assert_eq!(
            err,
            CertifyError::ResidualExceeded {
                check: CertCheck::WitnessSurface1,
                sample: 0
            }
        );

        // Same surface twice is structurally malformed.
        let mut bad = spec;
        bad.description = EdgeGeometry::Intersection {
            s1: keys[0],
            s2: keys[0],
            witness: Point3::new(0.5, 0.0, 0.0),
        };
        assert_eq!(
            EdgeCurve::certify(bad, p0, p1, &lookup, band()).unwrap_err(),
            CertifyError::IntersectionSameSurface { key: keys[0] }
        );

        // A stale key is a typed error.
        let mut bad = spec;
        bad.description = EdgeGeometry::Intersection {
            s1: keys[0],
            s2: SurfaceKey::default(),
            witness: Point3::new(0.5, 0.0, 0.0),
        };
        assert_eq!(
            EdgeCurve::certify(bad, p0, p1, &lookup, band()).unwrap_err(),
            CertifyError::UnresolvedSurface {
                key: SurfaceKey::default()
            }
        );
    }

    #[test]
    fn tangent_intersection_is_refused() {
        // Two planes meeting at the run-scaled sliver angle: the
        // transversality check escalates (never a guess).
        let theta = 3.0 * eps();
        let (keys, lookup) = table(vec![
            Surface::Plane {
                origin: Point3::origin(),
                normal: Vec3::unit_z(),
                u_ref: Vec3::unit_x(),
            },
            Surface::Plane {
                origin: Point3::origin(),
                normal: Vec3::new(0.0, theta.sin(), theta.cos()),
                u_ref: Vec3::unit_x(),
            },
        ]);
        let p0 = Point3::origin();
        let p1 = Point3::new(1.0, 0.0, 0.0);
        let spec = EdgeCurveSpec {
            description: EdgeGeometry::Intersection {
                s1: keys[0],
                s2: keys[1],
                witness: Point3::new(0.5, 0.0, 0.0),
            },
            carrier: Curve3::Line {
                origin: p0,
                dir: Vec3::unit_x(),
            },
            param_start: 0.0,
            param_end: 1.0,
        };
        let err = EdgeCurve::certify(spec, p0, p1, &lookup, band()).unwrap_err();
        assert!(
            matches!(
                err,
                CertifyError::Escalated {
                    check: CertCheck::Transversality,
                    ..
                }
            ),
            "{err:?}"
        );
        // Exactly coincident planes: definitely smooth ⇒ NotTransverse.
        let (keys, lookup) = table(vec![
            Surface::Plane {
                origin: Point3::origin(),
                normal: Vec3::unit_z(),
                u_ref: Vec3::unit_x(),
            },
            Surface::Plane {
                origin: Point3::origin(),
                normal: Vec3::unit_z(),
                u_ref: Vec3::unit_y(),
            },
        ]);
        let spec = EdgeCurveSpec {
            description: EdgeGeometry::Intersection {
                s1: keys[0],
                s2: keys[1],
                witness: Point3::new(0.5, 0.0, 0.0),
            },
            carrier: Curve3::Line {
                origin: p0,
                dir: Vec3::unit_x(),
            },
            param_start: 0.0,
            param_end: 1.0,
        };
        assert_eq!(
            EdgeCurve::certify(spec, p0, p1, &lookup, band()).unwrap_err(),
            CertifyError::NotTransverse { sample: 1 }
        );
    }

    #[test]
    fn seam_certifies_on_cylinder_and_rejects_antiseam() {
        // Cylinder about +z through the origin, seam (u_ref) at +x: the
        // seam ruling is the line x = r, y = 0.
        let r = 2.0;
        let (keys, lookup) = table(vec![Surface::Cylinder {
            origin: Point3::origin(),
            axis: Vec3::unit_z(),
            radius: r,
            u_ref: Vec3::unit_x(),
        }]);
        let p0 = Point3::new(r, 0.0, 0.0);
        let p1 = Point3::new(r, 0.0, 3.0);
        let spec = EdgeCurveSpec {
            description: EdgeGeometry::Seam { surface: keys[0] },
            carrier: Curve3::Line {
                origin: p0,
                dir: Vec3::unit_z(),
            },
            param_start: 0.0,
            param_end: 3.0,
        };
        EdgeCurve::certify(spec, p0, p1, &lookup, band()).unwrap();

        // The antipodal ruling (x = −r) is on the surface and in the
        // seam plane, but on the wrong side: the side check has teeth.
        let q0 = Point3::new(-r, 0.0, 0.0);
        let q1 = Point3::new(-r, 0.0, 3.0);
        let mut bad = spec;
        bad.carrier = Curve3::Line {
            origin: q0,
            dir: Vec3::unit_z(),
        };
        let err = EdgeCurve::certify(bad, q0, q1, &lookup, band()).unwrap_err();
        assert_eq!(
            err,
            CertifyError::ResidualExceeded {
                check: CertCheck::SeamSide,
                sample: 0
            }
        );

        // A seam on a plane is malformed.
        let (pkeys, plookup) = table(vec![Surface::Plane {
            origin: Point3::origin(),
            normal: Vec3::unit_z(),
            u_ref: Vec3::unit_x(),
        }]);
        let mut bad = spec;
        bad.description = EdgeGeometry::Seam { surface: pkeys[0] };
        assert_eq!(
            EdgeCurve::certify(bad, p0, p1, &plookup, band()).unwrap_err(),
            CertifyError::SeamOnNonPeriodic
        );
    }

    #[test]
    fn nurbs_carrier_is_refused_and_nothing_panics_on_poison() {
        let p0 = Point3::origin();
        let p1 = Point3::new(1.0, 0.0, 0.0);
        let mut spec = line_spec(p0, p1);
        spec.carrier = Curve3::Nurbs;
        assert_eq!(
            EdgeCurve::certify(spec, p0, p1, |_| None, band()).unwrap_err(),
            CertifyError::Unimplemented
        );
        // Poisoned endpoints: typed escalation, no panic (totality).
        let spec = line_spec(p0, p1);
        let nan = Point3::new(f64::NAN, 0.0, 0.0);
        let err = EdgeCurve::certify(spec, nan, p1, |_| None, band()).unwrap_err();
        assert!(matches!(err, CertifyError::Escalated { .. }));
    }

    /// A full-period circle carrier (the scaffolding self-loop case):
    /// coincident endpoints, params (0, τ) — certifiable because the
    /// interval is a stored certified cache (module docs).
    #[test]
    fn full_period_circle_certifies() {
        use core::f64::consts::TAU;
        let center = Point3::new(1.0, 2.0, 3.0);
        let p = Point3::new(2.0, 2.0, 3.0); // center + u_ref·r
        let spec = EdgeCurveSpec {
            description: EdgeGeometry::MappedCurve(MappedCurve::RevolvedPoint {
                point: Point2::new(2.0, 2.0),
                place: Affine3::translation(Vec3::new(0.0, 0.0, 3.0)),
                axis_origin: center,
                axis_dir: Vec3::unit_z(),
                angle: TAU,
            }),
            carrier: Curve3::Circle {
                center,
                axis: Vec3::unit_z(),
                radius: 1.0,
                u_ref: Vec3::unit_x(),
            },
            param_start: 0.0,
            param_end: TAU,
        };
        EdgeCurve::certify(spec, p, p, |_| None, band()).unwrap();
    }

    /// A placed-arc mapped curve against a circle carrier: the quarter
    /// arc of PR 2's bulge conventions, certified against its Circle3.
    #[test]
    fn placed_arc_certifies_against_circle_carrier() {
        use core::f64::consts::FRAC_PI_2;
        let bulge = (core::f64::consts::PI / 8.0).tan();
        let place = Affine3::translation(Vec3::new(0.0, 0.0, 1.0));
        let spec = EdgeCurveSpec {
            description: EdgeGeometry::MappedCurve(MappedCurve::PlacedSegment {
                segment: SketchSegment::Arc {
                    a: Point2::new(1.0, 0.0),
                    b: Point2::new(0.0, 1.0),
                    bulge,
                },
                place,
            }),
            carrier: Curve3::Circle {
                center: Point3::new(0.0, 0.0, 1.0),
                axis: Vec3::unit_z(),
                radius: 1.0,
                u_ref: Vec3::unit_x(),
            },
            param_start: 0.0,
            param_end: FRAC_PI_2,
        };
        let p0 = Point3::new(1.0, 0.0, 1.0);
        let p1 = Point3::new(0.0, 1.0, 1.0);
        EdgeCurve::certify(spec, p0, p1, |_| None, band()).unwrap();
    }
}
