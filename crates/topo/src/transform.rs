//! Rigid placement of a whole body (M4 PR 2, spec D3's Transform
//! clause): apply an isometry to every geometric datum of a [`Body`]
//! while leaving the topology — and every arena key — untouched.
//!
//! Landed here (not in editor-core) per the spec's report-or-land
//! path: no public rigid-transform op existed on `Body`, and bolting
//! geometry mapping into the editor layer would violate G1 layering.
//!
//! # Contract
//!
//! `map` must be a RIGID map (orthonormal linear part, determinant
//! +1). Rigidity is **checked at the door** with decided predicates
//! (column norms, pairwise orthogonality, and the determinant, each
//! classified against the linear band) — a uniform scale is affinely
//! self-consistent on planar bodies, so re-certification alone would
//! NOT catch it while it silently broke every unit-vector convention
//! downstream; the explicit check makes that a typed
//! [`TransformError::NotRigid`] refusal. On top of that, every edge
//! carrier is **re-certified** against the mapped geometry through
//! [`EdgeCurve::certify`], so a map that breaks carrier consistency
//! surfaces as a typed [`TransformError::Certify`] refusal, never as
//! silently corrupt geometry. (A rigid map preserves every
//! distance-valued residual up to rounding, so re-certification of a
//! valid body succeeds; re-running the checks rather than copying the
//! old certificate keeps the certificate honest — D4 ¶2.)
//!
//! # What maps how
//!
//! - points: `p ↦ map(p)`;
//! - surfaces: origins/centers/apexes by the full affine map,
//!   direction frames (`axis`, `u_ref`, `normal`) by the linear part,
//!   radii/angles untouched (rigid invariants);
//! - curve carriers: same treatment per [`Curve3`] variant;
//! - edge descriptions: [`EdgeDescription::Intersection`] keeps its
//!   surface keys (the arenas are key-stable) and **RE-MINTS its
//!   witness construction-fresh** — `witness′ = carrier′(mid)`, the
//!   mapped carrier evaluated at the pinned mid parameter (the S2
//!   formula the `WitnessMidpoint` check verifies) — never the mapped
//!   stored witness. Ruled with Ev on PR #83: mapping stored
//!   witnesses consumes inherited residual slack (a body certified
//!   near ε could refuse after an exact-in-principle isometry, and
//!   transform chains would ratchet); re-minting is per-entity local,
//!   D9-deterministic, and writes nothing to satisfy a check — it is
//!   the same formula construction uses. Consequence, the two-class
//!   contract: bodies re-certify with construction-fresh residual
//!   headroom regardless of stored-data marginality (the provable
//!   class); what can still refuse is authored VERDICT marginality
//!   (in-band classifications — Q1 bedrock), typed as always;
//!   [`MappedCurve`] pre-composes the isometry into its rigid
//!   placement (`place ↦ map ∘ place`) and maps its world-space
//!   vectors/axis data — sketch-space payloads are untouched;
//! - a DESCRIBED `Nurbs` surface or carrier maps by its CONTROL
//!   POINTS, with knots and weights carried over verbatim: a rigid map
//!   is affine, and the Euclidean-storage section of `geom`'s
//!   `curves::nurbs` data model is where the commutation that makes
//!   the mapped net the exact IMAGE — never a re-fit — is argued. No
//!   certificate rides on a net, so nothing has to be re-derived,
//!   which is the whole of why this arm maps where `Approx` refuses;
//! - the `Nurbs` PLACEHOLDER — and only it — is refused typed (its
//!   evaluation is all-poison; transforming one would launder poison
//!   as geometry).

//! - approximating surfaces: the offset DESCRIPTION's base net and the
//!   FIT net both by the full affine map (weights and knots are
//!   invariants of it — [`geom::NurbsSurface::map_points`]), `d`, the
//!   window and the tolerance unchanged, and the two-limb certificate
//!   **re-derived** on the mapped pair through the scalar's own fit
//!   lane ([`geom_brep::PcurveFittedLane::remap_certificate`]) — never
//!   the stored one, which is a claim about a different geometry. The
//!   composition law is what makes the mapped pair a pair at all: a
//!   rigid map carries unit normals to unit normals, so
//!   `M(S + d·n) = M(S) + d·n_M`. A scalar with no fit lane refuses
//!   [`TransformError::ApproxLaneUnsupported`] naming it, and a fit
//!   door that refuses the re-derivation refuses
//!   [`TransformError::ApproxRecertify`] with its own error verbatim;

use std::sync::Arc;

use geom::Curve3;
use geom::Surface;
use geom_brep::{
    CertifyError, EdgeCurve, EdgeCurveSpec, EdgeDescription, EdgeDescriptionSpec, MappedCurve,
};
use geom_core::Tol;
use geom_core::predicate::{Band, BandError};
use geom_core::{Affine3, Decide, Margin, Point3, Real, Vec3};

use crate::body::Body;
use crate::entity::EdgeKey;
use crate::null::CurveGeom;

/// Typed failure of [`transform_rigid`] (closed enum, D4 ¶3).
#[derive(Debug)]
pub enum TransformError {
    /// Re-deriving the body's pcurve caches against the mapped
    /// geometry refused (M5 PR 6). A rigid map cannot break a valid
    /// body's chart images, so this is loud by design rather than
    /// expected.
    Pcurve {
        /// The typed pcurve-pass refusal, nested whole.
        source: crate::pcurves::PcurveMintError,
    },
    /// The run's tolerance could not form a classification band.
    Band(BandError),
    /// Re-certification of a mapped edge carrier failed. THREE causes
    /// reach this arm, and the third is not about the caller's
    /// geometry at all: the map is not an isometry at tolerance; or
    /// the input body's geometry was already out of certification; or
    /// the certification DOOR this pass uses admits a narrower class
    /// than the at-rest validator does, and declined a body that is
    /// perfectly sound. That last one is a `CertifyError::Unimplemented`
    /// from an `Intersection` naming a described `Nurbs` operand: this
    /// pass certifies through the plain [`EdgeCurve::certify`] while
    /// tier 3 uses the lane-wired door. Read the nested `source`, not
    /// this list, for which one it was.
    Certify {
        /// The edge whose carrier failed.
        edge: EdgeKey,
        /// The certification refusal, unaltered.
        source: CertifyError,
    },
    /// The map's linear part failed a decided rigidity predicate —
    /// a column norm or the determinant off 1, or two columns not
    /// orthogonal, at tolerance (in-band indeterminacy refuses too:
    /// a maybe-rigid map is not a rigid map).
    NotRigid {
        /// The named predicate that refused.
        check: &'static str,
    },
    /// A map component is non-finite (NaN/inf translation or linear
    /// entry) — refused at the door with the component named, before
    /// certification would refuse it obliquely (PR 2 review, R1b).
    NonFiniteMap {
        /// The named finiteness predicate that refused.
        check: &'static str,
    },
    /// The body carries a transient null-scaffold curve (M3 boolean
    /// machinery mid-flight); bodies at rest never do (tier 2).
    NullScaffold {
        /// The offending edge.
        edge: EdgeKey,
    },
    /// A `Nurbs` placeholder surface or carrier — the "no description
    /// yet" net, whose control points are all-poison, so transforming
    /// it would launder poison as geometry. A DESCRIBED net is not
    /// this arm: it maps, by its control points.
    NurbsPlaceholder,
    /// An approximating surface at a scalar with no fit lane: its
    /// certificate cannot be re-derived on the mapped pair, and a
    /// certificate is never carried across a geometry change.
    ApproxLaneUnsupported {
        /// The scalar's lane name, as the lane itself reports it.
        lane: &'static str,
    },
    /// The fit door refused the re-derivation of a mapped
    /// approximating surface's certificate — a limb above tolerance, a
    /// door meter, or a window the derivation does not cover.
    ApproxRecertify {
        /// The fit door's typed refusal, unaltered.
        source: geom_brep::OffsetFitError,
    },
    /// The body's topology references a missing arena entry — a
    /// corrupt body (the validators would refuse it too).
    Corrupt {
        /// What was missing, for the log.
        what: &'static str,
    },
}

impl core::fmt::Display for TransformError {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        match self {
            Self::Pcurve { source } => write!(f, "transform pcurve pass: {source}"),
            Self::Band(e) => write!(f, "transform could not form a band: {e}"),
            Self::Certify { edge, source } => write!(
                f,
                "transform: mapped edge {edge:?} failed re-certification: {source}"
            ),
            Self::NotRigid { check } => write!(
                f,
                "transform: the map's linear part is not an isometry at tolerance — \
                 predicate {check} refused, definitely or in-band"
            ),
            Self::NonFiniteMap { check } => write!(
                f,
                "transform: the map has a non-finite component — predicate {check} refused"
            ),
            Self::NullScaffold { edge } => write!(
                f,
                "transform: edge {edge:?} carries a transient null-scaffold curve; bodies at \
                 rest never do"
            ),
            Self::ApproxLaneUnsupported { lane } => write!(
                f,
                "transform: an approximating surface's certificate must be re-derived on the \
                 mapped description and fit, and the {lane} lane has no fit derivation to do \
                 it with — a certificate is never carried across a geometry change"
            ),
            Self::ApproxRecertify { source } => write!(
                f,
                "transform: re-deriving a mapped approximating surface's certificate refused: \
                 {source}"
            ),
            Self::NurbsPlaceholder => f.write_str(
                "transform: a Nurbs surface or carrier is refused, and the refusal is by \
                 VARIANT: the placeholder payload evaluates to poison, so mapping it would \
                 launder poison as geometry, but a DESCRIBED net evaluates for real and would \
                 map exactly — narrowing this arm to the placeholder state is unbuilt work, \
                 not a property of the geometry",
            ),
            Self::Corrupt { what } => write!(
                f,
                "transform: the body's topology references a missing {what} — a corrupt body"
            ),
        }
    }
}

// No `source()`, matching every error type in this crate: each arm's
// `Display` renders its nested payload's own `Display` in full, so the
// chain is already in the message and a walker would re-read what it
// just printed.
impl std::error::Error for TransformError {}

fn map_vec<T: Real>(map: &Affine3<T>, v: Vec3<T>) -> Vec3<T> {
    map.linear * v
}

/// The decided rigidity door (module docs): every margin must classify
/// `Zero` against the linear band — definite non-zero AND in-band
/// indeterminacy both refuse (a maybe-rigid map is not a rigid map).
fn check_rigid<T: Decide>(map: &Affine3<T>, band: Band) -> Result<(), TransformError> {
    let l = &map.linear;
    let one = T::one();
    // A table of K row names: they reach the funnel through the loop
    // variable below, so no grep for a literal at the decide site finds
    // them. A row added here is a roster change (`docs/K-REPORT.md`,
    // "The inventory method, restated").
    let checks: [(&'static str, T); 7] = [
        ("transform_rigid_col0_unit", l.c0.dot(l.c0) - one),
        ("transform_rigid_col1_unit", l.c1.dot(l.c1) - one),
        ("transform_rigid_col2_unit", l.c2.dot(l.c2) - one),
        ("transform_rigid_col01_orth", l.c0.dot(l.c1)),
        ("transform_rigid_col12_orth", l.c1.dot(l.c2)),
        ("transform_rigid_col02_orth", l.c0.dot(l.c2)),
        ("transform_rigid_det_plus_one", l.determinant() - one),
    ];
    for (check, margin) in checks {
        // Ledger row F10: the rigidity residuals of the linear map are
        // DIMENSIONLESS (unit-column/orthogonality/det defects) against
        // the metre band; the natural arm is the model/session-box
        // extent — an arm-policy question deferred by the row. No door
        // fits; flagged, not cast.
        match geom_core::k_stats::decide_flagged(check, margin, band, "F10") {
            Ok(geom_core::Sign::Zero) => {}
            _ => return Err(TransformError::NotRigid { check }),
        }
    }
    // Translation finiteness: x * 0 is exactly 0 iff x is finite
    // (component-wise — a norm could overflow to inf for large finite
    // translations). The linear part needs no separate door: NaN/inf
    // entries poison the rigidity margins above and refuse there.
    let t = &map.translation;
    // The levered door with the ZERO dimensionless factor: each margin
    // is 0·t_i — the zero length exactly when the component is finite,
    // poison otherwise (the finiteness probe the module docs state).
    let finite: [(&'static str, Margin<T>); 3] = [
        (
            "transform_rigid_trans_finite_x",
            Margin::levered(T::zero(), t.x),
        ),
        (
            "transform_rigid_trans_finite_y",
            Margin::levered(T::zero(), t.y),
        ),
        (
            "transform_rigid_trans_finite_z",
            Margin::levered(T::zero(), t.z),
        ),
    ];
    for (check, margin) in finite {
        match geom_core::k_stats::decide(check, margin, band) {
            Ok(geom_core::Sign::Zero) => {}
            _ => return Err(TransformError::NonFiniteMap { check }),
        }
    }
    Ok(())
}

/// **Sense-invariant** (M5 S10 audit). Every face's `sense` is copied
/// unchanged, and correctly so: this map's ratified contract is a
/// RIGID motion with `det = +1` (module docs), which carries the chart
/// normal to the transformed chart normal and the material side along
/// with it — the bit's *meaning* ("does the material side agree with
/// the chart normal?") survives because both sides move together.
/// The tripwire for any future extension: an orientation-REVERSING
/// map (a mirror, `det = −1`) would carry the chart normal to the
/// NEGATION of the transformed chart normal, and would therefore have
/// to flip `sense` on every face. `det = +1` is enforced upstream, so
/// there is no such branch to write here today.
fn map_surface<T: Decide + geom_brep::PcurveFittedLane>(
    map: &Affine3<T>,
    s: &Surface<T>,
    band: Band,
) -> Result<Surface<T>, TransformError> {
    Ok(match *s {
        Surface::Plane {
            origin,
            normal,
            u_ref,
        } => Surface::Plane {
            origin: map.transform_point(origin),
            normal: map_vec(map, normal),
            u_ref: map_vec(map, u_ref),
        },
        Surface::Cylinder {
            origin,
            axis,
            radius,
            u_ref,
        } => Surface::Cylinder {
            origin: map.transform_point(origin),
            axis: map_vec(map, axis),
            radius,
            u_ref: map_vec(map, u_ref),
        },
        Surface::Cone {
            apex,
            axis,
            half_angle,
            u_ref,
        } => Surface::Cone {
            apex: map.transform_point(apex),
            axis: map_vec(map, axis),
            half_angle,
            u_ref: map_vec(map, u_ref),
        },
        Surface::Sphere {
            center,
            radius,
            axis,
            u_ref,
        } => Surface::Sphere {
            center: map.transform_point(center),
            radius,
            axis: map_vec(map, axis),
            u_ref: map_vec(map, u_ref),
        },
        Surface::Torus {
            center,
            axis,
            major_radius,
            minor_radius,
            u_ref,
        } => Surface::Torus {
            center: map.transform_point(center),
            axis: map_vec(map, axis),
            major_radius,
            minor_radius,
            u_ref: map_vec(map, u_ref),
        },
        // A DESCRIBED net maps by its control points, weights and
        // knots untouched — the exact image, not a re-fit, so there is
        // no certificate to re-derive and no fit door to reach
        // (`NurbsSurface::map_points` states what the caller owes).
        // The PLACEHOLDER is the state this refusal's text describes
        // and the only state it refuses: its net is all-poison, so
        // mapping it would launder poison as geometry.
        Surface::Nurbs(ref n) => {
            if n.is_placeholder() {
                return Err(TransformError::NurbsPlaceholder);
            }
            Surface::Nurbs(Arc::new(n.map_points(|p| map.transform_point(p))))
        }
        Surface::Approx(ref a) => Surface::Approx(std::sync::Arc::new(map_approx(map, a, band)?)),
    })
}

/// The mapped approximating surface: mapped description, mapped fit,
/// same window and tolerance, certificate **re-derived** on the mapped
/// pair through the scalar's fit lane.
///
/// The composition law is what makes the mapped pair a pair: a rigid
/// map carries unit normals to unit normals, so
/// `M(S + d·n) = M(S) + d·n_M` — the map of an offset IS the offset of
/// the map, and a net mapped control-point-wise is the map of the
/// surface it describes ([`geom::NurbsSurface::map_points`], whose docs
/// carry the affine-combination argument). So the mapped fit stands to
/// the mapped base exactly as the fit stood to the base, at the same
/// `d` and the same tolerance.
///
/// What is NOT carried is the two-limb claim. The stored certificate is
/// a measurement of a different geometry, and re-running the
/// measurement is what keeps it honest (D4 ¶2, the same posture the
/// carriers and witnesses above take). It is re-derived against the
/// surface's OWN stored tolerance, because that is the claim the mapped
/// surface will store and therefore the claim it must be shown to
/// honour; the run's ε is tier 3's to classify against, per call.
///
/// **The re-derivation is not a formality, and the module docs'
/// distance-preservation parenthetical does not cover it.** Only one
/// limb of this certificate is a distance: `on_locus_max` is a sampled
/// residual and survives a rigid map to rounding. `hull_sup` is a
/// certified BOUND assembled from control-hull enclosures in the
/// ambient frame, so a rotation re-splits the same geometry across the
/// axes and the bound genuinely moves — measured at 7.4e-9 on a bowed
/// base fitted at 1e-6, and `curvature_reach` moves further. That is
/// why the mapped surface may not carry the operand's numbers: they
/// are the wrong numbers, not merely unverified ones. What the map
/// preserves is the CLAIM — the mapped pair certifies at the same
/// tolerance — and re-deriving is what establishes it.
///
/// `rounds` is the exception, and it is not a limb: it is carried, for
/// the reason [`geom::OffsetCertificate::rounds`] states once.
///
/// **The band is the RUN's while the tolerance is the SURFACE's**, and
/// the pair is deliberate rather than an oversight. The band reaches
/// only the fit door's degeneracy meters (the regularity floor and the
/// collapse reach); the two limbs are classified against `tolerance`
/// directly. So a tighter run band can make this door refuse a surface
/// its mint accepted, which is the fail-loud direction, and the shape
/// is exactly `geom_brep::recertify_approx`'s — the door tier 3 reaches
/// per face, which likewise meters at the run's band and classifies at
/// the caller's tolerance. The map and the validator therefore agree
/// about any given surface, which is the property that matters.
fn map_approx<T: Decide + geom_brep::PcurveFittedLane>(
    map: &Affine3<T>,
    a: &geom::ApproxSurface<T>,
    band: Band,
) -> Result<geom::ApproxSurface<T>, TransformError> {
    let old = a.spec();
    let geom::SurfaceDescription::Offset { ref base, d } = old.description;
    let spec = geom::SurfaceSpec {
        description: geom::SurfaceDescription::Offset {
            base: std::sync::Arc::new(base.map_points(|p| map.transform_point(p))),
            d,
        },
        fit: old.fit.map_points(|p| map.transform_point(p)),
        window: old.window,
        tolerance: old.tolerance,
    };
    let rounds = a.certificate().rounds;
    geom::ApproxSurface::certify(spec, |description, fit, window, tolerance| {
        match T::remap_certificate(description, fit, window, tolerance, band) {
            None => Err(TransformError::ApproxLaneUnsupported {
                lane: <T as geom_brep::PcurveFittedLane>::lane_name(),
            }),
            Some(Err(source)) => Err(TransformError::ApproxRecertify { source }),
            Some(Ok(certificate)) => Ok(geom::OffsetCertificate {
                rounds,
                ..certificate
            }),
        }
    })
}

fn map_carrier<T: Real>(map: &Affine3<T>, c: &Curve3<T>) -> Result<Curve3<T>, TransformError> {
    Ok(match *c {
        Curve3::Line { origin, dir } => Curve3::Line {
            origin: map.transform_point(origin),
            dir: map_vec(map, dir),
        },
        Curve3::Circle {
            center,
            axis,
            radius,
            u_ref,
        } => Curve3::Circle {
            center: map.transform_point(center),
            axis: map_vec(map, axis),
            radius,
            u_ref: map_vec(map, u_ref),
        },
        // A rigid map preserves the semi-axis lengths and the frame's
        // orthonormality — the axis ordering invariant transfers
        // unchanged, so no re-decision is needed (the constructor
        // decided at mint; this is evaluation-lane data motion).
        Curve3::Ellipse {
            center,
            axis,
            major,
            minor,
            u_ref,
        } => Curve3::Ellipse {
            center: map.transform_point(center),
            axis: map_vec(map, axis),
            major,
            minor,
            u_ref: map_vec(map, u_ref),
        },
        // The surface arm's argument, one dimension down: a described
        // net maps by its control points (weights and knots verbatim)
        // and the placeholder alone is refused.
        Curve3::Nurbs(ref n) => {
            if n.is_placeholder() {
                return Err(TransformError::NurbsPlaceholder);
            }
            Curve3::Nurbs(Arc::new(n.map_points(|p| map.transform_point(p))))
        }
    })
}

/// Applies the rigid map to every geometric datum of `body`, returning
/// a NEW body with identical topology and identical arena keys (the
/// three geometry arenas are rewritten in place of the clone; nothing
/// is re-minted, so downstream key handles remain meaningful).
///
/// Every edge carrier is re-certified against the mapped endpoints and
/// mapped surfaces — see the module docs for the contract and the
/// refusal doors.
///
/// # Errors
///
/// [`TransformError`] — closed and typed.
pub fn transform_rigid<T: Decide + geom_brep::PcurveFittedLane>(
    body: &Body<T>,
    map: &Affine3<T>,
    tol: Tol,
) -> Result<Body<T>, TransformError> {
    let band = Band::linear(tol).map_err(TransformError::Band)?;
    check_rigid(map, band)?;
    let mut out = body.clone();

    // GeomSource hygiene (N6, M4 PR 5): this op rewrites every
    // description's bits without recipe context, so the cloned source
    // records' same-source ⇒ same-bits claim would become FALSE here.
    // Clear them all; the recipe layer re-stamps composed sources
    // (`GeomSource::placed`) right after the op — it, not this
    // kernel-level map, knows the placing node's identity.
    out.clear_geom_sources();

    // Points and surfaces first — edge re-certification below reads
    // the MAPPED versions of both.
    for (_k, p) in &mut out.points {
        *p = map.transform_point(*p);
    }
    let mut mapped_surfaces = Vec::new();
    for (k, s) in &out.surfaces {
        mapped_surfaces.push((k, map_surface(map, s, band)?));
    }
    for (k, s) in mapped_surfaces {
        out.surfaces[k] = s;
    }

    // Curves: walk the EDGES (each edge names its curve; tier 1 keeps
    // the reference well-formed), map description + carrier, and
    // re-certify against the mapped endpoints and surfaces. Track
    // which curve keys were rewritten so an orphaned curve entry —
    // which the walk would silently leave UNMAPPED — is refused as
    // corruption instead.
    let mut rewritten = std::collections::HashSet::new();
    let edge_keys: Vec<EdgeKey> = body.edges().map(|(k, _)| k).collect();
    for ek in edge_keys {
        let edge = out.edges.get(ek).ok_or(TransformError::Corrupt {
            what: "edge key vanished mid-walk",
        })?;
        let curve_key = edge.curve;
        let he_plus = edge.he_plus;
        let old = match out.curves.get(curve_key) {
            Some(CurveGeom::Certified(ec)) => ec.clone(),
            Some(CurveGeom::NullScaffold(_)) => {
                return Err(TransformError::NullScaffold { edge: ek });
            }
            None => {
                return Err(TransformError::Corrupt {
                    what: "edge references a missing curve",
                });
            }
        };
        let start = endpoint(&out, he_plus, false)?;
        let end = endpoint(&out, he_plus, true)?;
        let (param_start, param_end) = old.params();
        let carrier = map_carrier(map, old.carrier())?;
        let description = match old.description() {
            // Re-mint (module docs above): construction-fresh witness
            // from the MAPPED carrier at the pinned mid parameter.
            // Params are transform-invariant, so the pre-transform
            // schedule parameter is the post-transform one.
            EdgeDescription::Intersection { s1, s2, .. } => EdgeDescriptionSpec::Intersection {
                s1: *s1,
                s2: *s2,
                witness: carrier.eval(old.sample_param((geom_brep::CERT_SAMPLES - 1) / 2)),
            },
            // TangentIntersection maps as Intersection does: keys are
            // stable, the witness re-mints from the mapped carrier.
            EdgeDescription::TangentIntersection { s1, s2, .. } => {
                EdgeDescriptionSpec::TangentIntersection {
                    s1: *s1,
                    s2: *s2,
                    witness: carrier.eval(old.sample_param((geom_brep::CERT_SAMPLES - 1) / 2)),
                }
            }
            // A chart image is a PARAMETER-SPACE fact, invariant under
            // a rigid map: the mapped SURFACE carries the whole map,
            // and the image on it is the image it always was (M6-3;
            // a seam is likewise defined by its key-stable surface).
            // The authority record travels with the map, because a
            // sketch pushforward is stated in 3-space.
            EdgeDescription::Chart(c) => {
                let chart = EdgeDescriptionSpec::Chart {
                    surface: c.surface,
                    image: Some(c.pcurve.clone()),
                    seam: c.seam,
                    declared: None,
                };
                match old.authority() {
                    geom_brep::EdgeAuthority::Declared(mc) => {
                        chart.declared_by(map_mapped_curve(map, &mc))
                    }
                    geom_brep::EdgeAuthority::Derived => chart,
                }
            }
            EdgeDescription::Scaffold(mc) => {
                EdgeDescriptionSpec::Scaffold(map_mapped_curve(map, mc))
            }
        };
        let spec = EdgeCurveSpec {
            description,
            carrier,
            param_start,
            param_end,
        };
        let surfaces = |k| out.surfaces.get(k).cloned();
        let mapped = EdgeCurve::certify(spec, start, end, surfaces, band)
            .map_err(|source| TransformError::Certify { edge: ek, source })?;
        out.curves[curve_key] = CurveGeom::Certified(mapped);
        rewritten.insert(curve_key);
    }
    if let Some(_orphan) = out
        .curves
        .iter()
        .map(|(k, _)| k)
        .find(|k| !rewritten.contains(k))
    {
        return Err(TransformError::Corrupt {
            what: "curve entry referenced by no edge (left unmapped)",
        });
    }

    // Pcurve caches (M5 PR 6, C4): the SAME posture as carriers and
    // witnesses above — construction-fresh re-derivation against the
    // mapped geometry, never a mapped stored cache. A rigid map carries
    // the chart frame with the surface, so chart coordinates are
    // invariant and the re-derived caches are the same numbers; running
    // the derivation anyway is what keeps the certificate honest (D4 ¶2)
    // and costs a body that carried none exactly nothing — the pass only
    // runs when the operand actually carried caches, so transform never
    // MINTS caches a body did not have.
    if out.pcurves().next().is_some() {
        crate::pcurves::mint_pcurves(&mut out, tol)
            .map_err(|source| TransformError::Pcurve { source })?;
    }
    Ok(out)
}

/// The mapped world position of `he_plus`'s start (or end) vertex.
fn endpoint<T: Real>(
    body: &Body<T>,
    he_plus: crate::entity::HalfEdgeKey,
    end: bool,
) -> Result<Point3<T>, TransformError> {
    let vk = if end {
        body.half_edge_end(he_plus).ok_or(TransformError::Corrupt {
            what: "he_plus has no end vertex",
        })?
    } else {
        body.half_edges
            .get(he_plus)
            .ok_or(TransformError::Corrupt {
                what: "edge references a missing half-edge",
            })?
            .start
    };
    let v = body.vertices.get(vk).ok_or(TransformError::Corrupt {
        what: "half-edge references a missing vertex",
    })?;
    body.points
        .get(v.point)
        .copied()
        .ok_or(TransformError::Corrupt {
            what: "vertex references a missing point",
        })
}

fn map_mapped_curve<T: Real>(map: &Affine3<T>, mc: &MappedCurve<T>) -> MappedCurve<T> {
    match *mc {
        MappedCurve::PlacedSegment { segment, place } => MappedCurve::PlacedSegment {
            segment,
            place: *map * place,
        },
        MappedCurve::ExtrudedPoint { point, place, vec } => MappedCurve::ExtrudedPoint {
            point,
            place: *map * place,
            vec: map_vec(map, vec),
        },
        MappedCurve::RevolvedPoint {
            point,
            place,
            axis_origin,
            axis_dir,
            angle,
        } => MappedCurve::RevolvedPoint {
            point,
            place: *map * place,
            axis_origin: map.transform_point(axis_origin),
            axis_dir: map_vec(map, axis_dir),
            angle,
        },
    }
}

#[cfg(test)]
#[allow(clippy::unwrap_used, clippy::expect_used, clippy::panic)]
mod tests {
    //! The NURBS gate: the discriminator is the placeholder STATE, not
    //! the `Nurbs` variant. These rows pin both directions, so the gate
    //! cannot silently invert — a described net refusing and a
    //! placeholder mapping are each one edit away from each other, and
    //! only one of them is loud on its own.

    use super::*;
    use geom::{NurbsCurve3, NurbsSurface};
    use geom_core::spline::KnotVector;

    /// A translation: rigid, with exact entries.
    fn aside() -> Affine3<f64> {
        Affine3::translation(Vec3::new(3.0, -1.5, 0.25))
    }

    /// A described bilinear patch on the unit square — four live
    /// corners, unit weights. The minimal counterexample to "a `Nurbs`
    /// payload evaluates to poison".
    fn described_surface() -> Surface<f64> {
        let corners = vec![
            Point3::new(0.0, 0.0, 0.0),
            Point3::new(0.0, 1.0, 0.0),
            Point3::new(1.0, 0.0, 0.0),
            Point3::new(1.0, 1.0, 0.5),
        ];
        Surface::Nurbs(Arc::new(
            NurbsSurface::new(
                KnotVector::unit_segment(1),
                KnotVector::unit_segment(1),
                corners,
                vec![1.0; 4],
            )
            .expect("the bilinear patch validates"),
        ))
    }

    /// A described segment as a degree-1 rational curve.
    fn described_carrier() -> Curve3<f64> {
        Curve3::Nurbs(Arc::new(
            NurbsCurve3::new(
                KnotVector::unit_segment(1),
                vec![Point3::new(0.0, 0.0, 0.0), Point3::new(2.0, 1.0, 0.0)],
                vec![1.0, 2.0],
            )
            .expect("the segment validates"),
        ))
    }

    #[test]
    fn the_surface_placeholder_is_what_refuses() {
        assert!(matches!(
            map_surface(
                &aside(),
                &Surface::nurbs_placeholder(),
                Band::linear(Tol::witness()).unwrap()
            ),
            Err(TransformError::NurbsPlaceholder)
        ));
    }

    #[test]
    fn the_carrier_placeholder_is_what_refuses() {
        assert!(matches!(
            map_carrier(&aside(), &Curve3::nurbs_placeholder()),
            Err(TransformError::NurbsPlaceholder)
        ));
    }

    #[test]
    fn a_described_surface_maps_by_its_control_points() {
        let map = aside();
        let before = described_surface();
        let after = map_surface(&map, &before, Band::linear(Tol::witness()).unwrap())
            .expect("a described net maps");
        let (Surface::Nurbs(b), Surface::Nurbs(a)) = (&before, &after) else {
            panic!("the variant changed under the map");
        };
        assert_eq!(a.weights(), b.weights(), "weights are rigid-invariant");
        assert_eq!(a.knots_u(), b.knots_u(), "knots are rigid-invariant");
        assert_eq!(a.knots_v(), b.knots_v(), "knots are rigid-invariant");
        for (p, q) in b.control().iter().zip(a.control()) {
            let want = map.transform_point(*p);
            assert_eq!((q.x, q.y, q.z), (want.x, want.y, want.z));
        }
    }

    #[test]
    fn a_described_carrier_maps_by_its_control_points() {
        let map = aside();
        let before = described_carrier();
        let after = map_carrier(&map, &before).expect("a described net maps");
        let (Curve3::Nurbs(b), Curve3::Nurbs(a)) = (&before, &after) else {
            panic!("the variant changed under the map");
        };
        assert_eq!(a.weights(), b.weights(), "weights are rigid-invariant");
        assert_eq!(a.knots(), b.knots(), "knots are rigid-invariant");
        for (p, q) in b.control().iter().zip(a.control()) {
            let want = map.transform_point(*p);
            assert_eq!((q.x, q.y, q.z), (want.x, want.y, want.z));
        }
    }

    /// The gate as the public door reports it: a body whose only
    /// geometry is a placeholder surface refuses, and the same body
    /// carrying a described patch instead maps.
    #[test]
    fn the_body_door_refuses_the_placeholder_and_admits_the_description() {
        let mut placeheld: Body<f64> = Body::new();
        placeheld.add_surface(Surface::nurbs_placeholder());
        assert!(matches!(
            transform_rigid(&placeheld, &aside(), Tol::witness()),
            Err(TransformError::NurbsPlaceholder)
        ));

        let mut described: Body<f64> = Body::new();
        described.add_surface(described_surface());
        assert!(transform_rigid(&described, &aside(), Tol::witness()).is_ok());
    }
}
