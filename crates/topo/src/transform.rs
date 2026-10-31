//! Rigid placement of a whole body (M4 PR 2, spec D3's Transform
//! clause): apply an isometry to every geometric datum of a [`Body`]
//! while leaving the topology — and every arena key — untouched. The
//! same map, at the improper hand, is the geometry half of a mirror
//! ([`crate::mirror`]); everything below is stated for the rigid door
//! and says where the reflection differs.
//!
//! Landed here (not in editor-core) per the spec's report-or-land
//! path: no public rigid-transform op existed on `Body`, and bolting
//! geometry mapping into the editor layer would violate G1 layering.
//!
//! # Contract
//!
//! [`transform_rigid`]'s map is a [`RigidMotion`] (orthonormal linear
//! part, determinant +1, finite): rigidity is **carried by the witness
//! or decided at its door** ([`RigidMotion::check_rigid`]), and this
//! door asks nothing about it. A mirror's map is a
//! [`crate::Reflection`], a Householder matrix (determinant −1) decided
//! at its own door. A uniform scale is affinely self-consistent on planar
//! bodies, so re-certification alone would NOT catch one while it
//! silently broke every unit-vector convention downstream; the type is
//! what keeps one out, to the type's own standard: a minted motion is
//! rigid up to rounding, a door-admitted factor within the band, and a
//! product of `k` admitted factors — not decided again — within about
//! `k` bands (`RigidMotion`'s module docs). On top of that, every edge
//! carrier is **re-attached** against the mapped geometry through the
//! scalar's edge door ([`crate::AtRestPolicy::attach_edge`]: certified in
//! full where the scalar certifies, minted where it never does) — so a
//! map that breaks carrier consistency
//! surfaces as a typed [`TransformError::Certify`] refusal, never as
//! silently corrupt geometry. Re-running the checks rather than
//! copying the old certificate keeps the certificate honest (D4 ¶2).
//! A rigid map preserves sampled distances and implicit-form residuals
//! up to rounding. The plane × NURBS lane's limb 2 reads the norm of
//! each vector coefficient of its residual composite, which no rotation
//! moves; a residue remains, ≈6e-13 m on the
//! near-ε reproduction and not scaling with the residual, so an edge
//! certified within that width of ε can still refuse, typed
//! [`TransformError::Certify`]. An `Approx` face's `hull_sup` reads its
//! vector upper bounds from coefficient norms, but its lower bounds are
//! box-assembled in the ambient frame, so it can re-derive above ε for
//! a face certified near it; that face is re-fitted (see `map_approx`).
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
//!   window unchanged, and the two-limb certificate **re-derived** at
//!   the run's ε on the mapped pair through the injected fit door
//!   ([`geom_brep::OffsetFitLane::remap`]) — never the stored one,
//!   which is a claim about a different geometry. The
//!   composition law is what makes the mapped pair a pair at all: an
//!   isometry carries unit normals to unit normals, so
//!   `M(S + d·n) = M(S) + d·M·n`, and `M·n` is the mapped net's chart
//!   normal under a rigid map and its negation under a reflection, which
//!   therefore negates `d`. The certificate's hull bound is not
//!   frame-invariant, so the image of a face that certifies where it
//!   stands can re-derive above ε on a limb; that face is re-fitted
//!   construction-fresh on the mapped description through the same
//!   door's mint, as the witnesses are re-minted, rather than refused.
//!   A meter that goes in-band in the new frame, and an edge that rode
//!   the replaced fit net, can still refuse (`map_approx` says which).
//!   A run with no fit door refuses
//!   [`TransformError::ApproxLaneUnsupported`] naming its scalar, and a fit
//!   door that refuses — the image of a face that does not certify
//!   where it stands, or the moved description itself — refuses
//!   [`TransformError::ApproxRecertify`] with its own error verbatim;

use std::sync::Arc;

use geom::Curve3;
use geom::Surface;
use geom_brep::{
    CertifyError, EdgeCurveSpec, EdgeDescription, EdgeDescriptionSpec, MappedCurve, MappedSource,
};
use geom_core::Tol;
use geom_core::predicate::{Band, BandError};
use geom_core::{Affine3, Decide, Point3, Real, RigidMotion, Vec3};
use slotmap::SecondaryMap;

use crate::body::Body;
use crate::entity::EdgeKey;
use crate::geometry::CurveKey;
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
    /// Re-certification of a mapped edge carrier failed: the input
    /// body's geometry was already out of certification, or the mapped
    /// data no longer certify together (at an enclosure scalar, a
    /// widened motion's images are enclosed apart). Read the nested
    /// `source` for which check refused.
    Certify {
        /// The edge whose carrier failed.
        edge: EdgeKey,
        /// The certification refusal, unaltered.
        source: CertifyError,
    },
    /// The body carries a null-scaffold edge: what an unfinished edit
    /// leaves behind (a lone `mev_null` on a fresh face, say). At a
    /// build, a caller reaches it by moving a body in the middle of its
    /// own edit, so it ends in that edit's lever
    /// ([`crate::Unfinished::SCAFFOLDING_RECOURSE`]). Tier 2 refuses it
    /// at rest, and the import door mints none, so read there it is a
    /// stored contradiction and ends in the door's defect ending.
    NullScaffold {
        /// The offending edge.
        edge: EdgeKey,
    },
    /// A `Nurbs` placeholder surface or carrier — the "no description
    /// yet" net, whose control points are all-poison, so transforming
    /// it would launder poison as geometry. A DESCRIBED net is not
    /// this arm: it maps, by its control points.
    NurbsPlaceholder,
    /// An approximating surface with no fit door in hand: its
    /// certificate cannot be re-derived on the mapped pair, and a
    /// certificate is never carried across a geometry change.
    ///
    /// Where `Some` comes from, and what its absence means:
    /// [`crate::AtRestPolicy::offset_fit_lane`].
    ApproxLaneUnsupported {
        /// The scalar the map ran at ([`geom_core::Real::NAME`]).
        scalar: &'static str,
    },
    /// The fit door refused a mapped approximating surface: the image
    /// of a fit that does not certify in its own frame either (a limb
    /// above tolerance), a door meter, a window the derivation does not
    /// cover, or the fresh fit of the mapped description that a sound
    /// face's refused image falls back to.
    ApproxRecertify {
        /// The fit door's typed refusal, unaltered.
        source: geom_brep::OffsetFitError,
    },
    /// The body is corrupt (the validators would refuse it too): a
    /// reference that does not resolve, an arena entry nothing
    /// references, or an image no door builds.
    Corrupt {
        /// What the walk found, as a clause.
        found: &'static str,
    },
}

impl TransformError {
    /// This refusal's text read at `at`: a nested certification or
    /// pcurve refusal renders at that door, a defect ends in the door's
    /// defect ending ([`geom_brep::recourse::defect_ending`]), and
    /// scaffolding ends in its edit's lever at a build.
    #[must_use]
    pub fn render(&self, at: impl Into<geom_brep::recourse::ReadAt>) -> String {
        let at = at.into();
        let defect = geom_brep::recourse::defect_ending(at);
        match self {
            Self::Pcurve { source } => source.render(at),
            Self::Band(e) => e.to_string(),
            Self::Certify { source, .. } => format!(
                "an edge the map moved failed re-certification: {}",
                source.render(at)
            ),
            Self::NullScaffold { edge } => match at {
                geom_brep::recourse::ReadAt::Run(geom_brep::recourse::Reading::Build) => {
                    format!(
                        "the body still carries what an edit left behind, which the map cannot \
                         move. Recourse: {}",
                        crate::Unfinished::SCAFFOLDING_RECOURSE
                    )
                }
                _ => format!(
                    "edge {edge:?} carries a transient null-scaffold curve, which a body at \
                     rest never does. {defect}"
                ),
            },
            Self::ApproxLaneUnsupported { scalar } => format!(
                "an approximating surface cannot be moved at the {scalar} scalar with no \
                 offset-fit door in hand: its certificate is re-derived on the moved pair, and \
                 only {holders} holds that door (the fit is derived there alone). Recourse: \
                 move the body at {holders}",
                holders = geom_brep::ScalarList(geom_brep::OFFSET_FIT_DOOR_HOLDERS),
            ),
            Self::ApproxRecertify { source } => format!(
                "a moved approximating surface could not be re-certified or re-fitted: {source}"
            ),
            Self::NurbsPlaceholder => format!(
                "a spline (NURBS) surface or carrier cannot be transformed yet. {}",
                geom_core::NOT_YET_ENDING
            ),
            Self::Corrupt { found } => format!("the body is corrupt: {found}. {defect}"),
        }
    }
}

/// Read at a build: the map is the operation that built what it moved.
impl core::fmt::Display for TransformError {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.write_str(&self.render(geom_brep::recourse::Reading::Build))
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

/// Whether an isometry keeps orientation ([`Hand::Proper`], a rigid
/// motion) or reverses it ([`Hand::Improper`], a reflection —
/// [`crate::mirror`]). Under an improper map every datum is written so
/// the result is the INSIDE-OUT image: a plane's stored normal, an
/// analytic face's outward side and every carrier's parameter are
/// those of the reflected body turned inside out, which
/// [`crate::mirror`]'s reversal then turns right side out.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub(crate) enum Hand {
    /// Determinant `+1`.
    Proper,
    /// Determinant `−1`.
    Improper,
}

impl Hand {
    /// `v` under `map`'s linear part, negated under an improper map:
    /// the axis that keeps a right-handed frame's sense of turning
    /// (`(−M·a) × (M·u) = M·(a × u)` when `det M = −1`).
    fn axial<T: Real>(self, map: &Affine3<T>, v: Vec3<T>) -> Vec3<T> {
        match self {
            Hand::Proper => map_vec(map, v),
            Hand::Improper => -map_vec(map, v),
        }
    }
}

/// Every datum of one surface under `map` at `hand` (module docs: what
/// maps how).
///
/// **Sense** (M5 S10 audit). Under [`Hand::Proper`] every face's `sense`
/// is copied unchanged, and correctly so: a rigid motion carries the
/// chart normal to the transformed chart normal and the material side
/// along with it, so the bit's *meaning* ("does the material side agree
/// with the chart normal?") survives because both move together. Under
/// [`Hand::Improper`] the map writes the inside-out image ([`Hand`]): a
/// plane's stored normal, its outward side, is the axial `−M·n`; an
/// analytic chart's frame maps forward, so its chart normal crosses as
/// `M·n` while the material side crosses inside out, and
/// [`map_isometry`] flips that face's `sense`; a net's chart normal
/// crosses as `−M·n`, already inside out, and its `sense` is kept.
/// [`crate::Body::revert`]'s map then reverses every loop's cycle and
/// every outward side once ([`crate::mirror`]).
fn map_surface<T: Decide + crate::props::AtRestPolicy>(
    map: &Affine3<T>,
    hand: Hand,
    s: &Surface<T>,
    tol: Tol,
) -> Result<Surface<T>, TransformError> {
    Ok(match *s {
        // The normal is the face's outward side, so the inside-out
        // image's is the axial one; the chart is then `M∘S` exactly.
        Surface::Plane {
            origin,
            normal,
            u_ref,
        } => Surface::Plane {
            origin: map.transform_point(origin),
            normal: hand.axial(map, normal),
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
        Surface::Approx(ref a) => Surface::Approx(map_approx(
            map,
            hand,
            a,
            tol,
            <T as crate::props::AtRestPolicy>::offset_fit_lane(),
        )?),
    })
}

/// The mapped approximating surface: the mapped description, and a fit
/// certified against it at the run's ε (`tol`, the number tier 3
/// classifies against) — the IMAGE of the operand's fit where that
/// certifies, a fresh fit of the mapped description where it does not.
///
/// The image is a pair by the composition law (module docs), but its
/// certificate is re-derived, never carried: `hull_sup`'s lower bounds
/// (the regularity floor and the floors on `‖E‖`) are assembled from
/// ambient-frame control hulls and move under a rotation (`geom-brep`'s
/// composition-law row pins that it does). The fit loop
/// stops at the first round that certifies, so a sound face can sit
/// within that drift of ε and its image re-derive above it.
///
/// **A LIMB refusal of a sound face's image is answered by re-fitting**:
/// the mint door runs on the mapped description at the same `tol`, and
/// what it certifies ships. "Sound" is decided, not assumed — the
/// operand is re-derived at `tol` in its own frame first, and one that
/// fails there (degraded, planted, minted at a looser ε) refuses with
/// the image's limb, so the map cannot launder a body tier 3 rejects.
/// `rounds` is carried onto an image; a re-fit carries its own.
///
/// **What this does not cover**, both open as
/// `work/encl/a-rigid-map-can-still-refuse-a-sound-approx-face-at-its-edges-or-meters.md`:
///
/// - **Meters.** The regularity floor and the collapse headroom are
///   box-assembled over the MAPPED base, so a sound face whose meter
///   goes in-band after a rotation refuses `ApproxRecertify { Meter }`,
///   and a re-fit runs the same meters on the same base, so it cannot
///   answer. Meter and window refusals are passed on verbatim.
/// - **Edges on a re-fitted face.** A re-fit replaces the fit net, and
///   the edges whose carrier or chart image rides the old net's bits
///   (a `Chart` image on the fit, the iso rows `replace_face` cuts from
///   `approx.fit()`) are mapped control-point-wise and re-certified
///   against the NEW net below, so they can still refuse
///   [`TransformError::Certify`] or [`TransformError::Pcurve`].
///
/// A refusal of the re-fit itself — the loop cannot reach ε in the new
/// frame — is `ApproxRecertify` with the mint door's error. `offset_fit`
/// is the door, as a parameter ([`crate::AtRestPolicy::offset_fit_lane`]
/// says what `None` means); with none the map refuses typed rather than
/// carry a certificate across a geometry change.
fn map_approx<T: Decide>(
    map: &Affine3<T>,
    hand: Hand,
    a: &geom::ApproxSurface<T>,
    tol: Tol,
    offset_fit: Option<geom_brep::OffsetFitLane<T>>,
) -> Result<Arc<geom::ApproxSurface<T>>, TransformError> {
    let Some(lane) = offset_fit else {
        return Err(TransformError::ApproxLaneUnsupported { scalar: T::NAME });
    };
    let old = a.spec();
    let geom::SurfaceDescription::Offset { ref base, d } = old.description;
    let base = Arc::new(base.map_points(|p| map.transform_point(p)));
    // A reflected net's chart normal is the reflected normal negated,
    // so the same offset surface stands at the opposite signed distance.
    let d = match hand {
        Hand::Proper => d,
        Hand::Improper => -d,
    };
    let spec = geom::SurfaceSpec {
        description: geom::SurfaceDescription::Offset {
            base: Arc::clone(&base),
            d,
        },
        fit: old.fit.map_points(|p| map.transform_point(p)),
        window: old.window,
    };
    let rounds = a.certificate().rounds;
    let image = geom::ApproxSurface::certify(spec, |description, fit, window| {
        lane.remap(description, fit, window, tol)
            .map(|certificate| certificate.carrying_rounds(rounds))
    });
    match image {
        Ok(image) => Ok(Arc::new(image)),
        Err(source @ geom_brep::OffsetFitError::Limb { .. }) => {
            if lane.recertify(a, tol).is_err() {
                return Err(TransformError::ApproxRecertify { source });
            }
            lane.mint(base, d, tol)
                .map_err(|source| TransformError::ApproxRecertify { source })
        }
        Err(source) => Err(TransformError::ApproxRecertify { source }),
    }
}

/// The carrier `map ∘ c` on `c`'s own parameter, under either hand: a
/// carrier has no outward side, and keeping its parameter keeps every
/// edge's range and its forward half-edge.
fn map_carrier<T: Real>(
    map: &Affine3<T>,
    hand: Hand,
    c: &Curve3<T>,
) -> Result<Curve3<T>, TransformError> {
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
            axis: hand.axial(map, axis),
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
            semi_u,
            semi_v,
            u_ref,
        } => Curve3::Ellipse {
            center: map.transform_point(center),
            axis: hand.axial(map, axis),
            semi_u,
            semi_v,
            u_ref: map_vec(map, u_ref),
        },
        // The same argument: a rigid map preserves both radii, the
        // stand-off and the frame's orthonormality, so the two-oval
        // regime and the carried side transfer unchanged — no
        // re-decision (the mint decided; this is data motion).
        Curve3::Spiric {
            center,
            axis,
            u_ref,
            major_radius,
            minor_radius,
            offset,
        } => {
            // `(axis, u_ref, offset)` and `(−axis, −u_ref, −offset)` name
            // one oval; under a reflection the image keeps its `v` with
            // `axis` mapped and the plane's side negated.
            let (u_ref, offset) = match hand {
                Hand::Proper => (map_vec(map, u_ref), offset),
                Hand::Improper => (-map_vec(map, u_ref), -offset),
            };
            Curve3::Spiric {
                center: map.transform_point(center),
                axis: map_vec(map, axis),
                u_ref,
                major_radius,
                minor_radius,
                offset,
            }
        }
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

/// Applies the rigid motion to every geometric datum of `body`, returning
/// a NEW body with identical topology and identical arena keys (the
/// three geometry arenas are rewritten in place of the clone; nothing
/// is re-minted, so downstream key handles remain meaningful). The
/// operand's contact records therefore carry as
/// [`crate::ContactRecords::carried_from`]`(0)`.
///
/// Every edge carrier is re-certified against the mapped endpoints and
/// mapped surfaces — see the module docs for the contract and the
/// refusal doors.
///
/// # Panics
///
/// On a kernel bug: an edge the body stores, re-minted or restated on its
/// own carrier and interval, whose span or winding reads certainly
/// refused at the run's band.
///
/// # Errors
///
/// [`TransformError`] — closed and typed.
pub fn transform_rigid<T: Decide + crate::props::AtRestPolicy>(
    body: &Body<T>,
    motion: &RigidMotion<T>,
    tol: Tol,
) -> Result<Body<T>, TransformError> {
    let mut out = map_isometry(body, &motion.to_affine(), Hand::Proper, tol)?;
    // Pcurve caches (M5 PR 6, C4): the SAME posture as carriers and
    // witnesses — construction-fresh re-derivation against the mapped
    // geometry, never a mapped stored cache. A rigid map carries the
    // chart frame with the surface, so chart coordinates are invariant
    // and the re-derived caches are the same numbers; running the
    // derivation anyway is what keeps the certificate honest (D4 ¶2).
    // An operand at rest stores a row on every face that owes one, so an
    // operand storing none has no face that does.
    if out.pcurves().next().is_some() {
        crate::pcurves::mint_pcurves(&mut out, tol)
            .map_err(|source| TransformError::Pcurve { source })?;
    }
    Ok(out)
}

/// The chart image `image` on `surface` of `body` under a map of `hand`:
/// itself under a proper map, whose mapped surface carries the whole map,
/// and reflected with its chart under an improper one.
fn mapped_image<T: Real>(
    body: &Body<T>,
    surface: geom_brep::SurfaceKey,
    image: &geom_brep::Pcurve<T>,
    hand: Hand,
) -> Result<geom_brep::Pcurve<T>, TransformError> {
    match hand {
        Hand::Proper => Ok(image.clone()),
        Hand::Improper => {
            let class = body.surfaces.get(surface).map(OrientationClass::of).ok_or(
                TransformError::Corrupt {
                    found: "a chart image lies on a missing surface",
                },
            )?;
            reflected_image(image, class)
        }
    }
}

/// **An edge's chart image under a reflection, in the inside-out
/// image's own terms** ([`map_isometry`] at [`Hand::Improper`]).
///
/// A spiric image is restated for its mapped carrier, whose stand-off
/// is `−d` ([`map_carrier`]): `f(t) = √((R + r·cos t)² − d²)` is even in
/// `d`, so a plane cap keeps its coefficients (its chart is unmoved),
/// and a torus wall, whose chart reverses its azimuth, reads
/// `−u0 − σμ·atan2(f, d) = u0′ + σμ·atan2(f, −d)` with
/// `u0′ = −u0 − σμπ`, since `atan2(f, −d) = π − atan2(f, d)` for the
/// `f > 0` of the two-oval regime; its minor angle, its sense and its
/// torus are unmoved. Every other image on an analytic chart has its
/// azimuth negated, and one on a plane or a net is unmoved.
///
/// # Errors
///
/// [`TransformError::Corrupt`] for an image whose winding count has no
/// negation in `i32`, which no body a door built carries.
fn reflected_image<T: Real>(
    p: &geom_brep::Pcurve<T>,
    class: OrientationClass,
) -> Result<geom_brep::Pcurve<T>, TransformError> {
    use geom_brep::{Pcurve, SpiricImage};
    if let Pcurve::Spiric {
        major,
        minor,
        offset,
        image,
    } = p
    {
        let image = match *image {
            SpiricImage::Cap { .. } => *image,
            SpiricImage::Wall {
                u0,
                v0,
                sense,
                torus,
            } => SpiricImage::Wall {
                u0: -u0 - sense * torus.mu::<T>() * T::pi(),
                v0,
                sense,
                torus,
            },
        };
        return Ok(Pcurve::Spiric {
            major: *major,
            minor: *minor,
            offset: -*offset,
            image,
        });
    }
    match class {
        OrientationClass::Analytic => p
            .map_affine(geom_brep::ChartMap {
                u: geom_brep::AxisMap::reflect(),
                v: geom_brep::AxisMap::identity(),
            })
            .map_err(|refusal| TransformError::Corrupt {
                found: match refusal {
                    geom_brep::ChartMapRefusal::WindingUnnegatable => {
                        "a chart image's winding count has no negation"
                    }
                    geom_brep::ChartMapRefusal::WallReflected => {
                        "a spiric wall image reached a reflection that cannot restate it"
                    }
                },
            }),
        OrientationClass::Plane | OrientationClass::Net => Ok(p.clone()),
    }
}

/// **How a surface's face carries its orientation**, the one
/// classification [`crate::Body::revert`] and the reflection read.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub(crate) enum OrientationClass {
    /// A plane: its stored normal is the outward side, so it carries
    /// its own reversal, and a reflection writes it so its chart is
    /// unmoved ([`map_surface`]).
    Plane,
    /// A cylinder, cone, sphere or torus: the outward side rides the
    /// face's `sense`, and a reflection maps the frame forward,
    /// reversing the azimuth `u` (MIRROR-DESIGN P1).
    Analytic,
    /// A spline net or a fit: the outward side rides `sense`, and a
    /// reflection maps the net by its points, its chart unmoved.
    Net,
}

impl OrientationClass {
    /// The class of `s`.
    pub(crate) fn of<T: Real>(s: &Surface<T>) -> Self {
        match s {
            Surface::Plane { .. } => Self::Plane,
            Surface::Cylinder { .. }
            | Surface::Cone { .. }
            | Surface::Sphere { .. }
            | Surface::Torus { .. } => Self::Analytic,
            Surface::Nurbs(_) | Surface::Approx(_) => Self::Net,
        }
    }
}

/// The geometry half of [`transform_rigid`] and [`crate::mirror`]: every
/// datum of `body` under `map`, topology and keys untouched, every edge
/// re-certified. The pcurve rows are left as found, for the caller to
/// re-mint once the body is whole.
///
/// Under [`Hand::Improper`] the result is the reflected body INSIDE OUT
/// ([`Hand`]): the analytic faces' outward side is flipped on their
/// `sense`, since their mapped frames carry the outward normal across,
/// and an edge image in an analytic chart has its azimuth negated with
/// the chart.
pub(crate) fn map_isometry<T: Decide + crate::props::AtRestPolicy>(
    body: &Body<T>,
    map: &Affine3<T>,
    hand: Hand,
    tol: Tol,
) -> Result<Body<T>, TransformError> {
    let band = Band::linear(tol).map_err(TransformError::Band)?;
    let mut out = body.clone();

    // Points and surfaces first — edge re-certification below reads
    // the MAPPED versions of both.
    for (_k, p) in &mut out.points {
        *p = map.transform_point(*p);
    }
    let mut mapped_surfaces = Vec::new();
    for (k, s) in &out.surfaces {
        mapped_surfaces.push((k, map_surface(map, hand, s, tol)?));
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
    let mut rewritten: SecondaryMap<CurveKey, ()> = SecondaryMap::new();
    let edge_keys: Vec<EdgeKey> = body.edges().map(|(k, _)| k).collect();
    for ek in edge_keys {
        let edge = out.edges.get(ek).ok_or(TransformError::Corrupt {
            found: "an edge vanished mid-walk",
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
                    found: "an edge references a missing curve",
                });
            }
        };
        let start = endpoint(&out, he_plus, false)?;
        let end = endpoint(&out, he_plus, true)?;
        let (param_start, param_end) = old.params();
        let carrier = map_carrier(map, hand, old.carrier())?;
        let description = match old.description() {
            // Re-mint (module docs above): construction-fresh witness
            // from the MAPPED carrier at the pinned mid parameter
            // (params are transform-invariant).
            EdgeDescription::Intersection { pair, selector } => EdgeDescriptionSpec::Intersection {
                pair: *pair,
                selector: match selector {
                    witness @ geom_brep::CurveSelector::Witness(_) => {
                        witness.repinned(&carrier, param_start, param_end)
                    }
                    // Each wall's row maps as a chart image does.
                    geom_brep::CurveSelector::Structural(geom_brep::Structural::SharedRow(row)) => {
                        let [ka, kb] = pair.keys();
                        let mapped = |k| -> Result<_, TransformError> {
                            let image = row.image_on(*pair, k).ok_or(TransformError::Corrupt {
                                found: "a shared row names a surface outside its pair",
                            })?;
                            Ok((k, mapped_image(body, k, image, hand)?))
                        };
                        let row = geom_brep::SharedRow::new(*pair, mapped(ka)?, mapped(kb)?)
                            .ok_or(TransformError::Corrupt {
                                found: "a shared row's mapped image is no row",
                            })?;
                        geom_brep::CurveSelector::Structural(geom_brep::Structural::SharedRow(row))
                    }
                },
            },
            // TangentIntersection maps as Intersection does: keys are
            // stable, the witness re-mints from the mapped carrier.
            EdgeDescription::TangentIntersection { pair, .. } => {
                EdgeDescriptionSpec::TangentIntersection {
                    pair: *pair,
                    witness: carrier.mid_point(param_start, param_end),
                }
            }
            // The pair's keys are stable; the declaration maps as a
            // chart image's does.
            EdgeDescription::OnPair { pair } => {
                let on_pair = EdgeDescriptionSpec::on_pair(*pair);
                match old.authority() {
                    geom_brep::EdgeAuthority::Declared(mc) => {
                        on_pair.declared_by(map_mapped_curve(map, hand, &mc))
                    }
                    geom_brep::EdgeAuthority::Derived => on_pair,
                }
            }
            // A chart image is a PARAMETER-SPACE fact, invariant under
            // a rigid map: the mapped SURFACE carries the whole map,
            // and the image on it is the image it always was (M6-3;
            // a seam is likewise defined by its key-stable surface).
            // The authority record travels with the map, because a
            // sketch pushforward is stated in 3-space.
            EdgeDescription::Chart(c) => {
                let image = mapped_image(body, c.surface, &c.pcurve, hand)?;
                let chart = EdgeDescriptionSpec::Chart {
                    surface: c.surface,
                    image: Some(image),
                    wrap: c.wrap,
                    declared: None,
                };
                match old.authority() {
                    geom_brep::EdgeAuthority::Declared(mc) => {
                        chart.declared_by(map_mapped_curve(map, hand, &mc))
                    }
                    geom_brep::EdgeAuthority::Derived => chart,
                }
            }
            EdgeDescription::Scaffold(mc) => {
                EdgeDescriptionSpec::Scaffold(map_mapped_curve(map, hand, mc))
            }
        };
        let spec = EdgeCurveSpec {
            description,
            carrier,
            param_start,
            param_end,
        };
        let surfaces = |k| out.surfaces.get(k).cloned();
        // An isometry keeps every length, so the wedge's reach is the
        // source body's, read before `out`'s curves are half mapped.
        let reach = crate::wedge_reach(
            body,
            crate::edge_faces(body, ek),
            old.carrier(),
            (param_start, param_end),
        );
        let origin =
            crate::policy_lane::Origin::Kernels(crate::policy_lane::KernelSite::RigidTransform);
        let mapped = crate::policy_lane::certify(origin, spec, start, end, surfaces, reach, band)
            .map_err(|source| TransformError::Certify { edge: ek, source })?;
        out.curves[curve_key] = CurveGeom::Certified(mapped);
        rewritten.insert(curve_key, ());
    }
    if let Some(_orphan) = out
        .curves
        .iter()
        .map(|(k, _)| k)
        .find(|&k| !rewritten.contains_key(k))
    {
        return Err(TransformError::Corrupt {
            found: "a curve entry is referenced by no edge",
        });
    }

    if hand == Hand::Improper {
        let flipped: Vec<crate::entity::FaceKey> = out
            .faces
            .iter()
            .filter(|(_, f)| {
                out.surfaces
                    .get(f.surface)
                    .is_some_and(|s| OrientationClass::of(s) == OrientationClass::Analytic)
            })
            .map(|(k, _)| k)
            .collect();
        for k in flipped {
            out.faces[k].sense = !out.faces[k].sense;
        }
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
            found: "an edge's primary half-edge has no end vertex",
        })?
    } else {
        body.half_edges
            .get(he_plus)
            .ok_or(TransformError::Corrupt {
                found: "an edge references a missing half-edge",
            })?
            .start
    };
    let v = body.vertices.get(vk).ok_or(TransformError::Corrupt {
        found: "a half-edge references a missing vertex",
    })?;
    body.points
        .get(v.point)
        .copied()
        .ok_or(TransformError::Corrupt {
            found: "a vertex references a missing point",
        })
}

/// The pushforward under `map ∘ place`: the sketch datum stays as
/// authored, so under a reflection `place` is improper and reads it
/// back unchanged through its inverse.
fn map_mapped_curve<T: Real>(map: &Affine3<T>, hand: Hand, mc: &MappedCurve<T>) -> MappedCurve<T> {
    MappedCurve {
        source: match mc.source {
            MappedSource::PlacedSegment { segment, place } => MappedSource::PlacedSegment {
                segment,
                place: *map * place,
            },
            MappedSource::ExtrudedPoint { point, place, vec } => MappedSource::ExtrudedPoint {
                point,
                place: *map * place,
                vec: map_vec(map, vec),
            },
            MappedSource::RevolvedPoint {
                point,
                place,
                axis_origin,
                axis_dir,
                angle,
            } => MappedSource::RevolvedPoint {
                point,
                place: *map * place,
                axis_origin: map.transform_point(axis_origin),
                axis_dir: hand.axial(map, axis_dir),
                angle,
            },
        },
        range: mc.range,
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
        aside_motion().to_affine()
    }

    /// [`aside`] as the witness the body door takes.
    fn aside_motion() -> RigidMotion<f64> {
        RigidMotion::translation(Vec3::new(3.0, -1.5, 0.25)).expect("a finite translation")
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
                KnotVector::unit_segment(core::num::NonZeroUsize::MIN),
                KnotVector::unit_segment(core::num::NonZeroUsize::MIN),
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
                KnotVector::unit_segment(core::num::NonZeroUsize::MIN),
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
                Hand::Proper,
                &Surface::nurbs_placeholder(),
                Tol::witness()
            ),
            Err(TransformError::NurbsPlaceholder)
        ));
    }

    #[test]
    fn the_carrier_placeholder_is_what_refuses() {
        assert!(matches!(
            map_carrier(&aside(), Hand::Proper, &Curve3::nurbs_placeholder()),
            Err(TransformError::NurbsPlaceholder)
        ));
    }

    #[test]
    fn a_described_surface_maps_by_its_control_points() {
        let map = aside();
        let before = described_surface();
        let after =
            map_surface(&map, Hand::Proper, &before, Tol::witness()).expect("a described net maps");
        let (Surface::Nurbs(b), Surface::Nurbs(a)) = (&before, &after) else {
            panic!("the variant changed under the map");
        };
        assert_eq!(a.weights(), b.weights(), "weights are rigid-invariant");
        assert_eq!(
            a.knots_u_f64_interim(),
            b.knots_u_f64_interim(),
            "knots are rigid-invariant"
        );
        assert_eq!(
            a.knots_v_f64_interim(),
            b.knots_v_f64_interim(),
            "knots are rigid-invariant"
        );
        for (p, q) in b.control().iter().zip(a.control()) {
            let want = map.transform_point(*p);
            assert_eq!((q.x, q.y, q.z), (want.x, want.y, want.z));
        }
    }

    #[test]
    fn a_described_carrier_maps_by_its_control_points() {
        let map = aside();
        let before = described_carrier();
        let after = map_carrier(&map, Hand::Proper, &before).expect("a described net maps");
        let (Curve3::Nurbs(b), Curve3::Nurbs(a)) = (&before, &after) else {
            panic!("the variant changed under the map");
        };
        assert_eq!(a.weights(), b.weights(), "weights are rigid-invariant");
        assert_eq!(
            a.knots_f64_interim(),
            b.knots_f64_interim(),
            "knots are rigid-invariant"
        );
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
            transform_rigid(&placeheld, &aside_motion(), Tol::witness()),
            Err(TransformError::NurbsPlaceholder)
        ));

        let mut described: Body<f64> = Body::new();
        described.add_surface(described_surface());
        assert!(transform_rigid(&described, &aside_motion(), Tol::witness()).is_ok());
    }

    /// A curve entry no edge names is refused, and the refusal says the
    /// entry is unreferenced rather than that a reference dangles.
    #[test]
    fn an_orphan_curve_entry_reads_as_unreferenced() {
        let mut orphaned: Body<f64> = Body::new();
        orphaned.add_null_curve(crate::null::NullEdge {
            below_end: crate::VertexKey::default(),
            above_end: crate::VertexKey::default(),
        });
        let err = transform_rigid(&orphaned, &aside_motion(), Tol::witness())
            .expect_err("an orphan curve entry is corruption");
        let shown = err.to_string();
        assert!(
            shown.starts_with("the body is corrupt: a curve entry is referenced by no edge."),
            "{shown}"
        );
        assert!(!shown.contains("missing"), "{shown}");
    }
}

/// **The transform door's fit door, as the pass takes it** — the rows
/// that say what each of its two answers costs.
///
/// [`map_approx`] is called directly because these rows are about the
/// PARAMETER: [`map_surface`] reads the scalar's own seam
/// (`crate::AtRestPolicy::offset_fit_lane`), and a row that could only reach the
/// door the seam hands it could not tell an absent door from a scalar
/// that has none.
#[cfg(test)]
#[allow(clippy::unwrap_used, clippy::expect_used, clippy::panic)]
mod offset_fit_door_rows {
    use geom_brep::OffsetFitLane;
    use geom_core::{Affine3, Point3, Tol, Vec3};

    use super::{TransformError, map_approx};

    /// A rotation about `z` — a rigid map that genuinely re-splits the
    /// hull bound across the axes, so the re-derivation has work to do.
    fn turned() -> Affine3<f64> {
        Affine3::rotation_about_axis(Point3::origin(), Vec3::new(0.0, 0.0, 1.0), 0.7)
    }

    /// **No door: the map refuses**, naming the scalar it ran at — here
    /// `f64`, the door's own scalar, handed none — and the certificate
    /// is never carried across a geometry change.
    #[test]
    fn no_door_refuses_the_mapped_surface_by_name() {
        let tol = Tol::witness();
        let approx = crate::fixtures::bowed_offset_approx::<f64>();
        match map_approx(&turned(), super::Hand::Proper, &approx, tol, None) {
            Err(TransformError::ApproxLaneUnsupported { scalar }) => assert_eq!(scalar, "f64"),
            other => panic!("the absence must name the scalar the map ran at: {other:?}"),
        }
    }

    /// **The `f64` door re-derives on the mapped pair**, and what it
    /// hands back is `geom-brep`'s own certifier run on that pair, limb
    /// for limb, bit for bit — the assertion that the body moved rather
    /// than being rewritten. `rounds` is the FIT's provenance and is
    /// carried, which is why it is read off the operand.
    ///
    /// **The reference is the FREE FUNCTION, not the door**, the way
    /// the mint and recertify twins pin theirs. Comparing the door
    /// against its own `remap` would compare the door with itself and
    /// could not see that body re-pointed — at `certify_offset_at`,
    /// say, which drops the window rule. The free door here is the
    /// `Tol` one, at the same witness the map was handed.
    #[test]
    fn the_f64_door_re_derives_the_mapped_pair() {
        let tol = Tol::witness();
        let map = turned();
        let approx = crate::fixtures::bowed_offset_approx::<f64>();
        let mapped = map_approx(
            &map,
            super::Hand::Proper,
            &approx,
            tol,
            Some(OffsetFitLane::fit()),
        )
        .expect("a rigid map of a certified fit re-certifies at the run's ε");
        let spec = mapped.spec();
        let reference =
            geom_brep::certify_offset_over(&spec.description, &spec.fit, spec.window, tol)
                .expect("`geom-brep`'s certifier measures the mapped pair");
        let got = mapped.certificate();
        crate::fixtures::assert_certificates_agree("the mapped pair", got, &reference);
        assert_eq!(
            got.rounds,
            approx.certificate().rounds,
            "`rounds` is the fit's provenance and is carried, not re-measured"
        );
    }
}
