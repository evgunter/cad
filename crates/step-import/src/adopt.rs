//! D7 adoption (Leg C, phases B–C): loop→face designation, surface
//! attachment with key sharing, `same_sense` honored verbatim, and the
//! **edge-adoption ladder** — every edge's intensional description
//! rebuilt and certified by the kernel's own gates.
//!
//! # Phase B — faces
//!
//! Phase A realizes the file's loops exactly, but the transient `mef`
//! partition owns them arbitrarily. Normalization promotes every
//! ring-designated loop to its own face (`mfkrh`), then each file
//! face's rings are demoted onto its outer loop's face (`kfmrh`) —
//! pure topology, no certification, ending with exactly the file's
//! loop→face partition. Surfaces then attach: **bitwise-identical
//! surface records share one kernel surface key** (`FaceSurface::
//! Shared`), restoring the writer-side sharing that the per-face
//! record emission flattened — this is what makes a seam edge's
//! same-surface-both-sides state visible to the ladder. `same_sense`
//! lands via [`topo::Body::set_face_sense`] exactly as read (the
//! corpus's reversed faces are deliberate kernel output — honored,
//! never healed).
//!
//! # Phase C — the edge ladder (D7 stage 2)
//!
//! For each edge, candidates in preference order (intrinsic before
//! conventional — the kernel's own at-rest preference), each attempt a
//! full [`topo::Body::set_edge_curve`] certification of the **parsed
//! carrier** over the derived interval:
//!
//! - distinct adjacent surfaces: `Intersection` (transverse), then
//!   `TangentIntersection` (the fillet-trimline contact class);
//! - one surface on both sides: `Seam` (periodic charts only), then
//!   the conventional `MappedCurve` self-description (a line as its
//!   own extrusion trajectory, a circular arc as its own revolution
//!   trajectory — honest data for loci the surfaces under-determine,
//!   e.g. a sphere's π-copy meridian or a profile edge inside a
//!   plane).
//!
//! The first candidate that certifies wins; if none does, the typed
//! [`StepImportError::Adoption`] carries every attempt and its typed
//! refusal (structured, for future remedy flows). A wrong guess
//! cannot survive: certification pins endpoints, the mid-parameter
//! witness, transversality/tangency margins, and seam-side residuals
//! at the kernel's own ε.

use geom_brep::{EdgeCurveSpec, EdgeGeometry, MappedCurve};
use geom_core::{Affine3, Point2, Point3};
use geom_curves::Curve3;
use geom_surfaces::Surface;
use topo::{Body, FaceKey, FaceSurface, LoopKey};

use crate::assemble::Assembled;
use crate::entities::SolidSpec;
use crate::error::{AdoptionAttempt, AdoptionCandidate, StepImportError};

/// Runs phases B and C for one assembled solid (module docs).
pub(crate) fn finish(
    body: &mut Body<f64>,
    solid: &SolidSpec,
    asm: &Assembled,
) -> Result<(), StepImportError> {
    let face_keys = designate_faces(body, solid, asm)?;
    attach_surfaces(body, solid, &face_keys)?;
    adopt_edges(body, solid, asm)
}

/// The body loop realizing target loop `l`.
fn body_loop(
    body: &Body<f64>,
    asm: &Assembled,
    l: usize,
    solid_id: u64,
) -> Result<LoopKey, StepImportError> {
    let first = asm.target.loops[l][0];
    Ok(body
        .get_half_edge(asm.use_he[first])
        .ok_or(StepImportError::Topology {
            id: solid_id,
            what: "internal: a realized half-edge does not resolve in adoption",
        })?
        .parent_loop)
}

/// The face currently owning a body loop.
fn owning_face(body: &Body<f64>, l: LoopKey, solid_id: u64) -> Result<FaceKey, StepImportError> {
    Ok(body
        .get_loop(l)
        .ok_or(StepImportError::Topology {
            id: solid_id,
            what: "internal: a realized loop does not resolve in adoption",
        })?
        .face)
}

/// Phase B's partition surgery: normalize (every loop an outer), then
/// demote each file face's rings onto its outer's face. Returns the
/// body face per file face, in `CLOSED_SHELL` order.
fn designate_faces(
    body: &mut Body<f64>,
    solid: &SolidSpec,
    asm: &Assembled,
) -> Result<Vec<FaceKey>, StepImportError> {
    let op_err = |source| StepImportError::Assembly {
        id: solid.id,
        source,
    };
    // Normalize: promote every ring-designated realized loop.
    for l in 0..asm.target.loops.len() {
        let lk = body_loop(body, asm, l, solid.id)?;
        let f = owning_face(body, lk, solid.id)?;
        let outer = body
            .get_face(f)
            .ok_or(StepImportError::Topology {
                id: solid.id,
                what: "internal: a loop's face does not resolve in adoption",
            })?
            .outer;
        if outer != lk {
            body.mfkrh_plug(lk).map_err(op_err)?;
        }
    }
    // Designate: each file face's rings become rings of its outer's
    // face (`kfmrh` demotes the ring's transient face wholesale).
    let mut face_keys = Vec::with_capacity(asm.target.face_loops.len());
    for (outer_l, ring_ls) in &asm.target.face_loops {
        let outer_lk = body_loop(body, asm, *outer_l, solid.id)?;
        let f0 = owning_face(body, outer_lk, solid.id)?;
        for &rl in ring_ls {
            let ring_lk = body_loop(body, asm, rl, solid.id)?;
            let fr = owning_face(body, ring_lk, solid.id)?;
            body.kfmrh(f0, fr).map_err(op_err)?;
        }
        face_keys.push(f0);
    }
    Ok(face_keys)
}

/// A surface's exact structural signature: variant tag + field bits,
/// the dedup key that restores writer-side surface-key sharing
/// (bitwise identity — an exact structural comparison, no ε).
fn surface_sig(surface: &Surface<f64>) -> Vec<u64> {
    let p = |p: Point3<f64>| [p.x.to_bits(), p.y.to_bits(), p.z.to_bits()];
    let v = |v: geom_core::Vec3<f64>| [v.x.to_bits(), v.y.to_bits(), v.z.to_bits()];
    match *surface {
        Surface::Plane {
            origin,
            normal,
            u_ref,
        } => [&[0u64][..], &p(origin), &v(normal), &v(u_ref)].concat(),
        Surface::Cylinder {
            origin,
            axis,
            radius,
            u_ref,
        } => [
            &[1u64][..],
            &p(origin),
            &v(axis),
            &[radius.to_bits()],
            &v(u_ref),
        ]
        .concat(),
        Surface::Cone {
            apex,
            axis,
            half_angle,
            u_ref,
        } => [
            &[2u64][..],
            &p(apex),
            &v(axis),
            &[half_angle.to_bits()],
            &v(u_ref),
        ]
        .concat(),
        Surface::Sphere {
            center,
            radius,
            axis,
            u_ref,
        } => [
            &[3u64][..],
            &p(center),
            &[radius.to_bits()],
            &v(axis),
            &v(u_ref),
        ]
        .concat(),
        Surface::Torus {
            center,
            axis,
            major_radius,
            minor_radius,
            u_ref,
        } => [
            &[4u64][..],
            &p(center),
            &v(axis),
            &[major_radius.to_bits(), minor_radius.to_bits()],
            &v(u_ref),
        ]
        .concat(),
        // Unreachable for parsed subset surfaces (resolution refuses
        // NURBS); a distinct tag keeps the map total anyway.
        Surface::Nurbs(_) => vec![5u64],
    }
}

/// Phase B's attachment: surfaces (deduped to shared keys) and the
/// `same_sense` bit, in `CLOSED_SHELL` face order.
fn attach_surfaces(
    body: &mut Body<f64>,
    solid: &SolidSpec,
    face_keys: &[FaceKey],
) -> Result<(), StepImportError> {
    let op_err = |source| StepImportError::Assembly {
        id: solid.id,
        source,
    };
    let mut seen: std::collections::BTreeMap<Vec<u64>, topo::SurfaceKey> =
        std::collections::BTreeMap::new();
    for (spec, &fk) in solid.faces.iter().zip(face_keys) {
        let sig = surface_sig(&spec.surface);
        let attached = match seen.get(&sig) {
            Some(&key) => body
                .set_face_surface(fk, FaceSurface::Shared(key))
                .map_err(op_err)?,
            None => body
                .set_face_surface(fk, FaceSurface::New(spec.surface.clone()))
                .map_err(op_err)?,
        };
        seen.insert(sig, attached);
        body.set_face_sense(fk, spec.sense).map_err(op_err)?;
    }
    Ok(())
}

/// Phase C: the per-edge adoption ladder (module docs).
fn adopt_edges(
    body: &mut Body<f64>,
    solid: &SolidSpec,
    asm: &Assembled,
) -> Result<(), StepImportError> {
    for (&edge_id, spec) in &solid.edges {
        let (fwd, rev) = asm
            .target
            .edge_uses_of(edge_id)
            .ok_or(StepImportError::Topology {
                id: edge_id,
                what: "internal: an edge without realized uses in adoption",
            })?;
        let he_plus = asm.use_he[fwd];
        let he_minus = asm.use_he[rev];
        let resolve = |what| StepImportError::Topology { id: edge_id, what };
        let edge_key = body
            .get_half_edge(he_plus)
            .ok_or(resolve("internal: a realized half-edge does not resolve"))?
            .edge;
        let face_surface = |body: &Body<f64>, he| -> Result<topo::SurfaceKey, StepImportError> {
            let loop_key = body
                .get_half_edge(he)
                .ok_or(resolve("internal: a realized half-edge does not resolve"))?
                .parent_loop;
            let face = body
                .get_loop(loop_key)
                .ok_or(resolve("internal: a realized loop does not resolve"))?
                .face;
            Ok(body
                .get_face(face)
                .ok_or(resolve("internal: a realized face does not resolve"))?
                .surface)
        };
        let fs_plus = face_surface(body, he_plus)?;
        let fs_minus = face_surface(body, he_minus)?;
        let witness = spec.carrier.eval((spec.t0 + spec.t1) / 2.0);
        let p_start = solid.vertices[&spec.start];
        let p_end = solid.vertices[&spec.end];

        // The candidate descriptions, in preference order (module
        // docs: intrinsic before conventional).
        let mut candidates: Vec<(AdoptionCandidate, EdgeGeometry<f64>)> = Vec::new();
        if fs_plus != fs_minus {
            candidates.push((
                AdoptionCandidate::Intersection,
                EdgeGeometry::Intersection {
                    s1: fs_plus,
                    s2: fs_minus,
                    witness,
                },
            ));
            candidates.push((
                AdoptionCandidate::TangentIntersection,
                EdgeGeometry::TangentIntersection {
                    s1: fs_plus,
                    s2: fs_minus,
                    witness,
                },
            ));
        } else {
            let periodic = body.get_surface(fs_plus).is_some_and(|s| {
                matches!(
                    s,
                    Surface::Cylinder { .. }
                        | Surface::Cone { .. }
                        | Surface::Sphere { .. }
                        | Surface::Torus { .. }
                )
            });
            if periodic {
                candidates.push((
                    AdoptionCandidate::Seam,
                    EdgeGeometry::Seam { surface: fs_plus },
                ));
            }
            if let Some(mapped) =
                mapped_self_description(&spec.carrier, p_start, p_end, spec.t0, spec.t1)
            {
                candidates.push((
                    AdoptionCandidate::MappedCurve,
                    EdgeGeometry::MappedCurve(mapped),
                ));
            }
        }

        let mut attempts = Vec::new();
        let mut adopted = false;
        for (candidate, description) in candidates {
            let attempt = EdgeCurveSpec {
                description,
                carrier: spec.carrier.clone(),
                param_start: spec.t0,
                param_end: spec.t1,
            };
            match body.set_edge_curve(edge_key, attempt) {
                Ok(_) => {
                    adopted = true;
                    break;
                }
                Err(refusal) => attempts.push(AdoptionAttempt { candidate, refusal }),
            }
        }
        if !adopted {
            return Err(StepImportError::Adoption {
                id: edge_id,
                attempts,
            });
        }
    }
    Ok(())
}

/// The conventional self-description for carriers the surfaces
/// under-determine: a line is its start point's trajectory under the
/// translation to its end, a circular arc its start point's
/// trajectory under the rotation through the derived angle — honest
/// pushforward data (the same shapes native constructions store),
/// exact for the parsed carrier. Ellipse/NURBS carriers have no
/// mapped form (none exists in `geom_brep::MappedCurve`'s vocabulary,
/// and no exported body puts one inside a single surface); the ladder
/// then reports every refusal typed.
fn mapped_self_description(
    carrier: &Curve3<f64>,
    p_start: Point3<f64>,
    p_end: Point3<f64>,
    t0: f64,
    t1: f64,
) -> Option<MappedCurve<f64>> {
    match carrier {
        Curve3::Line { .. } => Some(MappedCurve::ExtrudedPoint {
            point: Point2::new(0.0, 0.0),
            place: Affine3::translation(p_start - Point3::origin()),
            vec: p_end - p_start,
        }),
        Curve3::Circle { center, axis, .. } => Some(MappedCurve::RevolvedPoint {
            point: Point2::new(0.0, 0.0),
            place: Affine3::translation(p_start - Point3::origin()),
            axis_origin: *center,
            axis_dir: *axis,
            angle: t1 - t0,
        }),
        Curve3::Ellipse { .. } | Curve3::Nurbs(_) => None,
    }
}
