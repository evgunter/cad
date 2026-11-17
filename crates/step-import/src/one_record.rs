//! **Two records of one surface become one** (`[import]`, D7;
//! `crates/topo/README.md` `[topo.one-record]`): an interpretation act.
//!
//! A file may state one locus as several surface records: a boolean
//! union's re-tiled flat side arrives as coplanar `PLANE` records with
//! different origins. Two records at rest are two surfaces, so where the
//! file's statements read them as one locus at the file's band they are
//! made one record before anything is built: every face on the later
//! record (in file order) takes the earlier one, its sense composed with
//! the two normals' agreement, and the move is reported
//! ([`NormalizationKind::OneRecord`]).
//!
//! **Decided at the faces, never at stored origins.** A plane record's
//! origin is an arbitrary point of it, so the question is asked of what
//! the faces are: the farthest any boundary carrier of a face on one
//! record lies from the other record's plane, bounded in closed form
//! (a line's ends, a conic's extreme over its whole curve, a spline's
//! control hull), read both ways. A face's region is the convex hull of
//! its boundary's span in its own plane, on which the distance from the
//! other plane is affine, so the bound covers the whole face. Zero is one
//! locus; apart keeps two records; in band refuses as an ambiguity,
//! naming the ε_in that decides it. A carrier with no closed-form bound
//! keeps the two records.
//!
//! Each record is read against the records kept so far, never through a
//! third (`[verdicts.closeness]`). Faces of different solids are never
//! asked: they stay two records, with the contact rows the at-rest gate
//! records.

use geom::{Curve3, Surface};
use geom_core::{Point3, Vec3};

use crate::entities::SolidSpec;
use crate::error::StepImportError;
use crate::interpret::{FileBand, Read};
use crate::{FaceCensus, NormalizationKind, StructureNormalization};

/// Whether two surface records state one locus: the farthest a face on
/// either lies from the other's surface, read at the file's band.
pub(crate) const STEP_ONE_RECORD: &str = "step_one_record";

/// Makes the records of `solid`'s faces that read as one locus one record
/// (module docs). `records[i]` is the surface entity id `solid.faces[i]`
/// states.
pub(crate) fn one_record(
    solid: &mut SolidSpec,
    records: &[u64],
    file: FileBand,
    sink: &mut Vec<StructureNormalization>,
) -> Result<(), StepImportError> {
    // The plane records, in file order, each with its faces.
    let mut planes: std::collections::BTreeMap<u64, Vec<usize>> = std::collections::BTreeMap::new();
    for (fi, face) in solid.faces.iter().enumerate() {
        if matches!(face.surface, Surface::Plane { .. }) {
            planes.entry(records[fi]).or_default().push(fi);
        }
    }
    let mut kept: Vec<(u64, usize)> = Vec::new();
    for (record, faces) in planes {
        let lead = faces[0];
        let mut joined = None;
        for &(keep, keep_face) in &kept {
            // Two records facing opposite ways are two surfaces to the
            // kernel's own builders (a stepped prism's two faces on one
            // plane, material on opposite sides), so only records that
            // face one way are asked.
            if normal(&solid.faces[keep_face].surface).dot(normal(&solid.faces[lead].surface))
                <= 0.0
            {
                continue;
            }
            let Some(gap) = locus_gap(solid, keep_face, &faces, records, keep) else {
                continue;
            };
            match file.read(STEP_ONE_RECORD, gap) {
                Read::Apart => {}
                Read::Zero => {
                    joined = Some((keep, keep_face, gap));
                    break;
                }
                Read::InBand => {
                    let refusal = file
                        .refusal(STEP_ONE_RECORD, gap, Read::InBand)
                        .unwrap_or_else(|| unreachable!("an in-band reading refuses"));
                    return Err(file.refused(record, refusal));
                }
            }
        }
        let Some((keep, keep_face, gap)) = joined else {
            kept.push((record, lead));
            continue;
        };
        let surface = solid.faces[keep_face].surface.clone();
        for &fi in &faces {
            solid.faces[fi].surface = surface.clone();
        }
        let empty = FaceCensus {
            faces: 0,
            edges: 0,
            vertices: 0,
        };
        sink.push(StructureNormalization {
            face: solid.faces[lead].id,
            kind: NormalizationKind::OneRecord {
                kept: keep,
                joined: record,
                moved_by: gap,
            },
            file_census: empty,
            kernel_census: empty,
        });
    }
    Ok(())
}

/// The plane's unit normal.
fn normal(surface: &Surface<f64>) -> Vec3<f64> {
    match *surface {
        Surface::Plane { normal, .. } => normal,
        _ => unreachable!("only plane records are read"),
    }
}

/// The farthest a face on one of the two records lies from the other's
/// plane, read both ways: `faces` (on the later record) against the kept
/// record's plane, and every face on the kept record against the later
/// one's. `None` where a carrier has no closed-form bound.
fn locus_gap(
    solid: &SolidSpec,
    keep_face: usize,
    faces: &[usize],
    records: &[u64],
    keep: u64,
) -> Option<f64> {
    let kept_faces: Vec<usize> = (0..solid.faces.len())
        .filter(|&fi| records[fi] == keep)
        .collect();
    let one = faces
        .iter()
        .map(|&fi| face_reach(solid, fi, &solid.faces[keep_face].surface))
        .try_fold(0.0_f64, |m, r| Some(m.max(r?)))?;
    let other = kept_faces
        .iter()
        .map(|&fi| face_reach(solid, fi, &solid.faces[faces[0]].surface))
        .try_fold(0.0_f64, |m, r| Some(m.max(r?)))?;
    Some(one.max(other))
}

/// An upper bound on the farthest face `fi`'s boundary carriers lie from
/// `plane` (module docs).
fn face_reach(solid: &SolidSpec, fi: usize, plane: &Surface<f64>) -> Option<f64> {
    let Surface::Plane { origin, normal, .. } = *plane else {
        return None;
    };
    let height = |p: Point3<f64>| normal.dot(p - origin).abs();
    let mut reach = 0.0_f64;
    let mut any = false;
    for lp in &solid.faces[fi].loops {
        for u in &lp.uses {
            let spec = solid.edges.get(&u.edge)?;
            let r = match spec.carrier {
                Curve3::Line(_) => {
                    height(spec.carrier.eval(spec.t0)).max(height(spec.carrier.eval(spec.t1)))
                }
                Curve3::Circle(ref c) => {
                    height(c.frame.origin) + c.radius * normal.cross(c.frame.axis).norm()
                }
                Curve3::Ellipse(ref e) => {
                    let v = e.frame.axis.cross(e.frame.u_ref);
                    height(e.frame.origin)
                        + (e.semi_u * normal.dot(e.frame.u_ref)).hypot(e.semi_v * normal.dot(v))
                }
                Curve3::Nurbs(ref c) => c.control().iter().fold(0.0_f64, |m, &p| m.max(height(p))),
                _ => return None,
            };
            if !r.is_finite() {
                return None;
            }
            reach = reach.max(r);
            any = true;
        }
    }
    any.then_some(reach)
}
