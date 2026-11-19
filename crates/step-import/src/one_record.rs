//! **Two records of one surface become one** (`[import]`, D7;
//! `crates/topo/README.md` `[topo.one-record]`): an interpretation act.
//!
//! A file may state one locus as several surface records: a boolean
//! union's re-tiled flat side arrives as coplanar `PLANE` records with
//! different origins. Two records at rest are two surfaces, so where the
//! file's statements read faces on two records as one locus at the file's
//! band, facing one way, they are made one record before anything is
//! built: those faces take the record that stands earlier in the DATA
//! section, each sense restated so its material side is unchanged (as
//! `topo`'s act does, `[topo.one-record.act]`), and the move is reported
//! ([`NormalizationKind::OneRecord`]).
//!
//! **A face's facing is its record's normal composed with its sense**,
//! never the record's normal alone: a writer states opposite sides of a
//! box on planes of one normal with opposite senses, and one side on
//! either normal. Faces facing opposite ways on one locus (a stepped
//! prism's two faces on one plane, material on opposite sides) are two
//! surfaces to the kernel's own builders, and are never asked.
//!
//! **Decided at the faces, never at stored origins.** A plane record's
//! origin is an arbitrary point of it, so the question is asked of what
//! the faces are: the farthest any boundary carrier of a face of one
//! group lies from the other group's plane, read off each carrier's cover
//! over its span, both ways. A face's region is the convex hull of its
//! boundary's span in its own plane, on which the distance from the other
//! plane is affine, so the bound covers the whole face. Zero is one
//! locus; apart keeps two records; in band refuses as an ambiguity,
//! naming what would decide it. A carrier with no cover keeps the two
//! records.
//!
//! Each group is read against the groups kept so far, never through a
//! third (`[verdicts.closeness]`). Faces of different solids are never
//! asked: they stay two records, with the contact rows the at-rest gate
//! records. `topo`'s own act (`Body::one_record`) takes a built body and
//! a token its deciding reading mints; this act runs on the file's
//! interpreted shell before any body exists, so it restates the act's
//! rule rather than calling it.

use geom::Surface;
use geom_core::{Point3, Vec3};

use crate::entities::SolidSpec;
use crate::error::StepImportError;
use crate::interpret::{FileBand, Read};
use crate::{FaceCensus, NormalizationKind, StructureNormalization};

/// Whether two surface records state one locus: the farthest a face on
/// either lies from the other's surface, read at the file's band.
pub(crate) const STEP_ONE_RECORD: &str = "step_one_record";

/// Which way two unit vectors point relative to each other, read for
/// its sign alone: whether two faces on one locus face one way, and
/// whether a face's facing agrees with a record's normal.
pub(crate) const STEP_ONE_RECORD_FACING: &str = "step_one_record_facing";

/// One surface record as the file places it: its entity id and where it
/// stands in the DATA section.
#[derive(Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Ord)]
pub(crate) struct Record {
    /// Its position in the DATA section, the order "earlier" reads.
    pub(crate) position: usize,
    /// Its entity id.
    pub(crate) id: u64,
}

/// The faces of one record that face one way.
struct Group {
    record: Record,
    /// Their facing: the record's normal composed with their sense.
    facing: Vec3<f64>,
    faces: Vec<usize>,
}

/// Whether `a` and `b` point one way, read for the sign alone. A pair
/// whose sign the least band cannot read (orthogonal within an ulp) is
/// not one way.
fn one_way(a: Vec3<f64>, b: Vec3<f64>) -> bool {
    geom_core::k_stats::prove(STEP_ONE_RECORD_FACING, geom_core::Margin::of(a.dot(b)))
        .is_ok_and(|sign| sign.is_positive())
}

/// Makes the faces of `solid` that read as one locus facing one way one
/// record (module docs). `records[i]` is the surface record
/// `solid.faces[i]` states.
pub(crate) fn one_record(
    solid: &mut SolidSpec,
    records: &[Record],
    file: FileBand,
    sink: &mut Vec<StructureNormalization>,
) -> Result<(), StepImportError> {
    // The plane faces' groups, by record (in DATA order) and by sense.
    let mut by: std::collections::BTreeMap<(Record, bool), Vec<usize>> =
        std::collections::BTreeMap::new();
    for (fi, face) in solid.faces.iter().enumerate() {
        if matches!(face.surface, Surface::Plane { .. }) {
            by.entry((records[fi], !face.sense)).or_default().push(fi);
        }
    }
    let groups: Vec<Group> = by
        .into_iter()
        .map(|((record, reversed), faces)| {
            let n = normal(&solid.faces[faces[0]].surface);
            Group {
                record,
                facing: if reversed { -n } else { n },
                faces,
            }
        })
        .collect();
    let mut kept: Vec<usize> = Vec::new();
    for (gi, group) in groups.iter().enumerate() {
        let mut joined = None;
        for &ki in &kept {
            let keep = &groups[ki];
            if keep.record == group.record || !one_way(keep.facing, group.facing) {
                continue;
            }
            let Some(gap) = locus_gap(solid, &keep.faces, &group.faces) else {
                continue;
            };
            match file.read(STEP_ONE_RECORD, gap) {
                Read::Apart => {}
                Read::Zero => {
                    joined = Some((ki, gap));
                    break;
                }
                Read::InBand => {
                    return Err(file.refused(group.record.id, file.ambiguous(STEP_ONE_RECORD, gap)));
                }
            }
        }
        let Some((ki, gap)) = joined else {
            kept.push(gi);
            continue;
        };
        let keep = &groups[ki];
        let surface = solid.faces[keep.faces[0]].surface.clone();
        let n = normal(&surface);
        // The sense that keeps each face's material side: its facing
        // read against the kept record's normal.
        let sense = one_way(n, group.facing);
        for &fi in &group.faces {
            solid.faces[fi].surface = surface.clone();
            solid.faces[fi].sense = sense;
        }
        sink.push(StructureNormalization {
            face: solid.faces[group.faces[0]].id,
            kind: NormalizationKind::OneRecord {
                kept: keep.record.id,
                joined: group.record.id,
                moved_by: gap,
            },
            file_census: FaceCensus::EMPTY,
            kernel_census: FaceCensus::EMPTY,
        });
    }
    one_round_record(solid, records, file, sink)
}

/// **Two records of one round surface become one**: a cylinder, cone,
/// sphere or torus stated twice. Its normal is fixed by the locus, so two
/// faces on one locus face one way exactly where their senses agree. Its
/// datum (an axis line, a centre, its radii) is the locus itself, unlike a
/// plane's arbitrary origin, so the question is asked of the datum, at the
/// faces' reach: a bound on the farthest a face of either group lies from
/// the other record's surface, an axis's tilt levered at the farthest
/// vertex of either group from the kept record's datum point
/// ([`round_gap`]).
fn one_round_record(
    solid: &mut SolidSpec,
    records: &[Record],
    file: FileBand,
    sink: &mut Vec<StructureNormalization>,
) -> Result<(), StepImportError> {
    let mut by: std::collections::BTreeMap<(Record, bool), Vec<usize>> =
        std::collections::BTreeMap::new();
    for (fi, face) in solid.faces.iter().enumerate() {
        if round(&face.surface) {
            by.entry((records[fi], face.sense)).or_default().push(fi);
        }
    }
    let groups: Vec<((Record, bool), Vec<usize>)> = by.into_iter().collect();
    let mut kept: Vec<usize> = Vec::new();
    for (gi, ((record, sense), faces)) in groups.iter().enumerate() {
        let mut joined = None;
        for &ki in &kept {
            let ((keep, keep_sense), keep_faces) = &groups[ki];
            if keep == record || keep_sense != sense {
                continue;
            }
            let (a, b) = (
                &solid.faces[keep_faces[0]].surface,
                &solid.faces[faces[0]].surface,
            );
            let lever = farthest_from(solid, keep_faces.iter().chain(faces), a);
            let Some(gap) = round_gap(a, b, lever) else {
                continue;
            };
            match file.read(STEP_ONE_RECORD, gap) {
                Read::Apart => {}
                Read::Zero => {
                    joined = Some((ki, gap));
                    break;
                }
                Read::InBand => {
                    return Err(file.refused(record.id, file.ambiguous(STEP_ONE_RECORD, gap)));
                }
            }
        }
        let Some((ki, gap)) = joined else {
            kept.push(gi);
            continue;
        };
        let ((keep, _), keep_faces) = &groups[ki];
        let surface = solid.faces[keep_faces[0]].surface.clone();
        for &fi in faces {
            solid.faces[fi].surface = surface.clone();
        }
        sink.push(StructureNormalization {
            face: solid.faces[faces[0]].id,
            kind: NormalizationKind::OneRecord {
                kept: keep.id,
                joined: record.id,
                moved_by: gap,
            },
            file_census: FaceCensus::EMPTY,
            kernel_census: FaceCensus::EMPTY,
        });
    }
    Ok(())
}

/// A cylinder, cone, sphere or torus.
fn round(surface: &Surface<f64>) -> bool {
    matches!(
        surface,
        Surface::Cylinder { .. }
            | Surface::Cone { .. }
            | Surface::Sphere { .. }
            | Surface::Torus { .. }
    )
}

/// The farthest a vertex of the faces `faces` lies from the datum point
/// of `surface` (its axis point, apex or centre): the lever an axis tilt
/// is read at across them.
fn farthest_from<'a>(
    solid: &SolidSpec,
    faces: impl Iterator<Item = &'a usize>,
    surface: &Surface<f64>,
) -> f64 {
    let at = match *surface {
        Surface::Cylinder { origin, .. } => origin,
        Surface::Cone { apex, .. } => apex,
        Surface::Sphere { center, .. } | Surface::Torus { center, .. } => center,
        _ => return 0.0,
    };
    let mut far = 0.0_f64;
    for &fi in faces {
        for lp in &solid.faces[fi].loops {
            for u in &lp.uses {
                if let Some(edge) = solid.edges.get(&u.edge) {
                    for v in [edge.start, edge.end] {
                        if let Some(p) = solid.vertices.get(&v) {
                            far = far.max((*p - at).norm());
                        }
                    }
                }
            }
        }
    }
    far
}

/// **How far two round records of one kind are from one locus**, in
/// metres: the sum of their datums' differences, an axis's tilt and a
/// half-angle levered at `lever`, which bounds the farthest a point of
/// either within `lever` of the datum point lies from the other. `None` for two kinds, or two cones whose axes point
/// opposite ways (two nappes, never one locus).
fn round_gap(a: &Surface<f64>, b: &Surface<f64>, lever: f64) -> Option<f64> {
    let off = |o: Point3<f64>, at: Point3<f64>, dir: Vec3<f64>| {
        let d = at - o;
        (d - dir * d.dot(dir)).norm()
    };
    let tilt = |a1: Vec3<f64>, a2: Vec3<f64>| a1.cross(a2).norm() * lever;
    Some(match (a, b) {
        (
            &Surface::Cylinder {
                origin: o1,
                axis: a1,
                radius: r1,
                ..
            },
            &Surface::Cylinder {
                origin: o2,
                axis: a2,
                radius: r2,
                ..
            },
        ) => tilt(a1, a2) + off(o1, o2, a1) + (r1 - r2).abs(),
        (
            &Surface::Cone {
                apex: p1,
                axis: a1,
                half_angle: h1,
                ..
            },
            &Surface::Cone {
                apex: p2,
                axis: a2,
                half_angle: h2,
                ..
            },
        ) => {
            if a1.dot(a2) <= 0.0 {
                return None;
            }
            tilt(a1, a2) + (p2 - p1).norm() + (h1 - h2).abs() * lever
        }
        (
            &Surface::Sphere {
                center: c1,
                radius: r1,
                ..
            },
            &Surface::Sphere {
                center: c2,
                radius: r2,
                ..
            },
        ) => (c2 - c1).norm() + (r1 - r2).abs(),
        (
            &Surface::Torus {
                center: c1,
                axis: a1,
                major_radius: big1,
                minor_radius: r1,
                ..
            },
            &Surface::Torus {
                center: c2,
                axis: a2,
                major_radius: big2,
                minor_radius: r2,
                ..
            },
        ) => tilt(a1, a2) + (c2 - c1).norm() + (big1 - big2).abs() + (r1 - r2).abs(),
        _ => return None,
    })
}

/// The plane's unit normal.
fn normal(surface: &Surface<f64>) -> Vec3<f64> {
    plane_of(surface).1
}

/// A plane record's origin and unit normal.
fn plane_of(surface: &Surface<f64>) -> (Point3<f64>, Vec3<f64>) {
    match *surface {
        Surface::Plane { origin, normal, .. } => (origin, normal),
        _ => unreachable!("only plane records are read"),
    }
}

/// The farthest a face of one group lies from the other group's plane,
/// read both ways. `None` where a carrier has no cover.
fn locus_gap(solid: &SolidSpec, kept: &[usize], faces: &[usize]) -> Option<f64> {
    let reach = |of: &[usize], plane: usize| {
        of.iter()
            .map(|&fi| face_reach(solid, fi, &solid.faces[plane].surface))
            .try_fold(0.0_f64, |m, r| Some(m.max(r?)))
    };
    Some(reach(faces, kept[0])?.max(reach(kept, faces[0])?))
}

/// An upper bound on the farthest face `fi`'s boundary carriers lie from
/// `plane`: each edge's span read off its carrier's cover along the
/// plane's normal ([`geom::Curve3::cover_over`]). `None` where a carrier
/// has no cover.
fn face_reach(solid: &SolidSpec, fi: usize, plane: &Surface<f64>) -> Option<f64> {
    let (origin, normal) = plane_of(plane);
    let level = normal.dot(origin - Point3::origin());
    let mut reach = 0.0_f64;
    let mut any = false;
    for lp in &solid.faces[fi].loops {
        for u in &lp.uses {
            let spec = solid.edges.get(&u.edge)?;
            let cover = spec
                .carrier
                .cover_over(geom::cover::ParamWindow::new(spec.t0, spec.t1))
                .ok()?;
            let (lo, hi) = cover.range_along(normal);
            let r = (lo - level).abs().max((hi - level).abs());
            if !r.is_finite() {
                return None;
            }
            reach = reach.max(r);
            any = true;
        }
    }
    any.then_some(reach)
}
