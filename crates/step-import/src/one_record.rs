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
//! **A round surface** (a cylinder, cone, sphere or torus) stated in two
//! records is asked the same question where its faces are
//! ([`crate::reach::Reach`] of both groups): its normal is fixed by the
//! locus, so two faces face one way exactly where their senses agree, and
//! the gap is how far apart the two axes stand at the ends of the faces'
//! extent along them, never at the records' stored axis points, with a
//! centre's or an apex's offset, a tilt levered at the faces' reach, and
//! the difference of the radii ([`round_gap`]). The faces take the earlier
//! record as stated; their senses already agree.
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
/// faces on one locus face one way exactly where their senses agree. The
/// question is asked where the faces are ([`crate::reach::Reach`] of both
/// groups): how far apart the two axes stand at the ends of the faces'
/// extent, never at the records' stored axis points, with the difference
/// of their radii ([`round_gap`]).
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
            let Some(gap) =
                crate::reach::Reach::of(solid, keep_faces.iter().chain(faces.iter()).copied())
                    .and_then(|reach| round_gap(a, b, &reach))
            else {
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

/// **How far two round records of one kind are from one locus**, in
/// metres, read over the region `reach` their faces occupy: two axes'
/// separation there ([`crate::reach::Reach::axes_apart`]), a centre's or
/// an apex's offset with an axis's tilt levered at the faces' reach from
/// it, and the difference of their radii, a cone's half-angles levered at
/// the faces' reach from its apex. It bounds the farthest a point of
/// either group's faces lies from the other surface. `None` for two kinds,
/// or two cones whose axes point opposite ways (two nappes, never one
/// locus).
fn round_gap(a: &Surface<f64>, b: &Surface<f64>, reach: &crate::reach::Reach) -> Option<f64> {
    let (la, lb) = (a.axis_line()?.unit(), b.axis_line()?.unit());
    let Datums {
        radii,
        on_axis,
        turns,
        half_angles,
    } = datums(a, b)?;
    if !on_axis && la.dir.dot(lb.dir) <= 0.0 && half_angles.is_some() {
        return None;
    }
    let tilt = la.dir.cross(lb.dir).norm();
    let apart = if on_axis {
        reach.axes_apart(la, lb)
    } else {
        (lb.origin - la.origin).norm()
            + if turns {
                tilt * reach.from(la.origin)
            } else {
                0.0
            }
    };
    Some(apart + radii + half_angles.map_or(0.0, |h| h * reach.from(la.origin)))
}

/// What two round records of one kind state beside their axes: whether
/// the axis line is the datum (a cylinder's, whose stored point is
/// arbitrary) or a point on it is (an apex, a centre), whether the axis
/// turns the surface (not a sphere's, whose stored axis is only its
/// chart's), the difference of their radii, and of two cones'
/// half-angles.
struct Datums {
    radii: f64,
    on_axis: bool,
    turns: bool,
    half_angles: Option<f64>,
}

/// [`Datums`] of `a` and `b`; `None` for two kinds.
fn datums(a: &Surface<f64>, b: &Surface<f64>) -> Option<Datums> {
    let point = |radii: f64, turns: bool| Datums {
        radii,
        on_axis: false,
        turns,
        half_angles: None,
    };
    Some(match (a, b) {
        (&Surface::Cylinder { radius: r1, .. }, &Surface::Cylinder { radius: r2, .. }) => Datums {
            radii: (r1 - r2).abs(),
            on_axis: true,
            turns: true,
            half_angles: None,
        },
        (&Surface::Cone { half_angle: h1, .. }, &Surface::Cone { half_angle: h2, .. }) => Datums {
            radii: 0.0,
            on_axis: false,
            turns: true,
            half_angles: Some((h1 - h2).abs()),
        },
        (&Surface::Sphere { radius: r1, .. }, &Surface::Sphere { radius: r2, .. }) => {
            point((r1 - r2).abs(), false)
        }
        (
            &Surface::Torus {
                major_radius: big1,
                minor_radius: r1,
                ..
            },
            &Surface::Torus {
                major_radius: big2,
                minor_radius: r2,
                ..
            },
        ) => point((big1 - big2).abs() + (r1 - r2).abs(), true),
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
        Surface::Plane { frame } => (frame.origin(), frame.axis()),
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

#[cfg(test)]
#[allow(clippy::unwrap_used, clippy::expect_used, clippy::panic)]
mod tests {
    use super::*;

    /// Two faces on one cylinder of radius 1 about `y` (`y` from 0 to 1
    /// and from 1 to 2), each its own record, with senses `senses`.
    fn two_walls(senses: [bool; 2]) -> (SolidSpec, [Record; 2]) {
        let y = Vec3::new(0.0, 1.0, 0.0);
        let x = Vec3::new(1.0, 0.0, 0.0);
        let rim = |at: f64| crate::entities::EdgeSpec {
            start: 1,
            end: 1,
            carrier: geom::Curve3::circle(Point3::new(0.0, at, 0.0), y, 1.0, x),
            t0: 0.0,
            t1: std::f64::consts::TAU,
            reversed: false,
            curve: None,
        };
        let surface = Surface::Cylinder {
            frame: geom::Frame::new(Point3::origin(), y, x),
            radius: 1.0,
        };
        let face = |id: u64, sense: bool, edges: [u64; 2]| crate::entities::FaceSpec {
            id,
            surface: surface.clone(),
            sense,
            loops: edges
                .into_iter()
                .map(|edge| crate::entities::LoopSpec {
                    outer: true,
                    uses: vec![crate::entities::EdgeUse {
                        edge,
                        forward: true,
                    }],
                })
                .collect(),
            band: false,
        };
        let solid = SolidSpec {
            id: 1,
            faces: vec![face(2, senses[0], [10, 11]), face(3, senses[1], [11, 12])],
            edges: [(10, rim(0.0)), (11, rim(1.0)), (12, rim(2.0))]
                .into_iter()
                .collect(),
            vertices: [(1, Point3::new(1.0, 0.0, 0.0))].into_iter().collect(),
            band_seams: std::collections::BTreeSet::new(),
        };
        let records = [
            Record {
                position: 1,
                id: 20,
            },
            Record {
                position: 2,
                id: 21,
            },
        ];
        (solid, records)
    }

    /// **Two records of one round surface are asked only of faces facing
    /// one way.** Two faces on one cylinder, each its own record: facing
    /// one way they become one record; facing opposite ways (their senses
    /// differing) they are never asked.
    #[test]
    fn round_records_join_only_facing_one_way() {
        let file = crate::interpret::FileBand::test(1e-6);
        for (senses, joined) in [([true, true], 1), ([true, false], 0)] {
            let (mut solid, records) = two_walls(senses);
            let mut sink = Vec::new();
            one_record(&mut solid, &records, file, &mut sink).unwrap();
            let joins = sink
                .iter()
                .filter(|n| matches!(n.kind, NormalizationKind::OneRecord { .. }))
                .count();
            assert_eq!(joins, joined, "senses {senses:?}");
        }
    }
}
