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
//! - edge descriptions: [`EdgeGeometry::Intersection`] keeps its
//!   surface keys (the arenas are key-stable) and maps its witness;
//!   [`MappedCurve`] pre-composes the isometry into its rigid
//!   placement (`place ↦ map ∘ place`) and maps its world-space
//!   vectors/axis data — sketch-space payloads are untouched;
//! - the `Nurbs` placeholders are refused typed (their evaluation is
//!   all-poison; transforming one would launder poison as geometry).

use geom_brep::{CertifyError, EdgeCurve, EdgeCurveSpec, EdgeGeometry, MappedCurve};
use geom_core::predicate::{Band, BandError};
use geom_core::{Affine3, Decide, Point3, Real, Vec3};
use geom_curves::Curve3;
use geom_surfaces::Surface;

use crate::body::Body;
use crate::entity::EdgeKey;
use crate::null::CurveGeom;

/// Typed failure of [`transform_rigid`] (closed enum, D4 ¶3).
#[derive(Debug)]
pub enum TransformError {
    /// The run's tolerance could not form a classification band.
    Band(BandError),
    /// Re-certification of a mapped edge carrier failed — the map is
    /// not an isometry at tolerance, or the input body's geometry was
    /// already out of certification.
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
    /// The body carries a transient null-scaffold curve (M3 boolean
    /// machinery mid-flight); bodies at rest never do (tier 2).
    NullScaffold {
        /// The offending edge.
        edge: EdgeKey,
    },
    /// A `Nurbs` placeholder surface or carrier — unimplemented
    /// geometry evaluates to poison, so transforming it is refused.
    NurbsPlaceholder,
    /// The body's topology references a missing arena entry — a
    /// corrupt body (the validators would refuse it too).
    Corrupt {
        /// What was missing, for the log.
        what: &'static str,
    },
}

fn map_vec<T: Real>(map: &Affine3<T>, v: Vec3<T>) -> Vec3<T> {
    map.linear * v
}

/// The decided rigidity door (module docs): every margin must classify
/// `Zero` against the linear band — definite non-zero AND in-band
/// indeterminacy both refuse (a maybe-rigid map is not a rigid map).
fn check_rigid<T: Decide>(map: &Affine3<T>, band: Band) -> Result<(), TransformError> {
    let l = &map.linear;
    let one = T::one();
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
        match geom_core::k_stats::decide(check, margin, band) {
            Ok(geom_core::Sign::Zero) => {}
            _ => return Err(TransformError::NotRigid { check }),
        }
    }
    Ok(())
}

fn map_surface<T: Real>(map: &Affine3<T>, s: &Surface<T>) -> Result<Surface<T>, TransformError> {
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
        Surface::Nurbs => return Err(TransformError::NurbsPlaceholder),
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
        Curve3::Nurbs => return Err(TransformError::NurbsPlaceholder),
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
pub fn transform_rigid<T: Decide>(
    body: &Body<T>,
    map: &Affine3<T>,
) -> Result<Body<T>, TransformError> {
    let band = Band::linear().map_err(TransformError::Band)?;
    check_rigid(map, band)?;
    let mut out = body.clone();

    // Points and surfaces first — edge re-certification below reads
    // the MAPPED versions of both.
    for (_k, p) in &mut out.points {
        *p = map.transform_point(*p);
    }
    let mut mapped_surfaces = Vec::new();
    for (k, s) in &out.surfaces {
        mapped_surfaces.push((k, map_surface(map, s)?));
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
            Some(CurveGeom::Certified(ec)) => *ec,
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
        let spec = EdgeCurveSpec {
            description: map_description(map, old.description()),
            carrier: map_carrier(map, old.carrier())?,
            param_start,
            param_end,
        };
        let surfaces = |k| out.surfaces.get(k).copied();
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

fn map_description<T: Real>(map: &Affine3<T>, d: &EdgeGeometry<T>) -> EdgeGeometry<T> {
    match d {
        EdgeGeometry::Intersection { s1, s2, witness } => EdgeGeometry::Intersection {
            s1: *s1,
            s2: *s2,
            witness: map.transform_point(*witness),
        },
        // A seam is defined intrinsically by its (key-stable) surface;
        // the mapped surface carries the whole map.
        EdgeGeometry::Seam { surface } => EdgeGeometry::Seam { surface: *surface },
        EdgeGeometry::MappedCurve(mc) => EdgeGeometry::MappedCurve(match *mc {
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
        }),
    }
}
