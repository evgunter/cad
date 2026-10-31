//! **One region per face, off the plane** (DESIGN `[topology.tiers]`,
//! tier 3): a curved face's surface cycles
//! ([`crate::surface_boundary::surface_cycles`]) each lie on the face's
//! side of every other, read by the point-in-face doors with their
//! boundary cut to the other cycle alone ([`crate::boolean::cycle_side`]).
//!
//! - **Which cycles are read alone.** A cycle on a sphere, a cylinder or a
//!   cone separates its surface, so each is read alone. On a torus a cycle
//!   separates only where it is contractible: one whose joint elements
//!   compose to the identity ([`crate::surface_boundary::cycle_winding`],
//!   the lift's integers, read and not re-derived). The essential ones are
//!   read as one pair, and a face holds none or two of them, wound
//!   opposite, so that they bound one band of the torus
//!   ([`ValidationError::EssentialCyclesUnpaired`]).
//! - **Where a cycle is read.** At each of its vertices in walk order,
//!   until one decides: on the face's side passes, off it refuses
//!   ([`ValidationError::CycleOffFaceSide`]). A vertex on the other cycle,
//!   or one no path of the door settles, is set aside. A cycle no vertex
//!   of which is placed refuses as undecided where a reading was in band
//!   ([`ValidationError::CycleSideUndecided`], D4 ¶1), and is exempt
//!   otherwise.
//! - **What is exempt**: a face no door reads (a spline chart, or an edge
//!   of a kind its door does not admit, the doors' `admits`), a torus face
//!   one of whose cycles has no stored joint element to read its winding
//!   from, and a cycle no vertex of which a path settles. A plane is check
//!   9's nesting arm.
//!
//! **An open surface's ends lie off the face** (the same clause) is
//! decided by check 6: a planar face's loop windings against its outward
//! normal (`LoopRoleInverted`), and a cylinder's or a cone's boundary
//! material side against its sense (`CurvedSenseInverted`). A face whose
//! loops enclose the surface's end rather than its own region reads
//! inverted there, so this module does not read it again.

use geom_core::{Band, Decide};

use crate::body::Body;
use crate::entity::{FaceKey, HalfEdgeKey};
use crate::validate::ValidationError;

/// **Every region finding on `body`'s curved faces**, in face-arena
/// order (module docs).
pub(crate) fn region_errors<T: Decide>(body: &Body<T>, band: Band) -> Vec<ValidationError> {
    let mut errors = Vec::new();
    for (face, f) in body.faces.iter() {
        let Some(surface) = body.surfaces.get(f.surface) else {
            continue; // unreachable on tier-1 input
        };
        let torus = match surface {
            geom::Surface::Sphere { .. }
            | geom::Surface::Cylinder { .. }
            | geom::Surface::Cone { .. } => false,
            geom::Surface::Torus { .. } => true,
            geom::Surface::Plane { .. } | geom::Surface::Nurbs(_) | geom::Surface::Approx(_) => {
                continue;
            }
        };
        let Ok(cycles) = crate::surface_boundary::surface_cycles(body, face) else {
            continue; // tier 1 refuses a boundary that does not walk
        };
        let units = if torus {
            match torus_units(body, face, &cycles) {
                Some(Ok(units)) => units,
                Some(Err(essential)) => {
                    errors.push(ValidationError::EssentialCyclesUnpaired { face, essential });
                    continue;
                }
                None => continue, // a cycle with no stored element: exempt
            }
        } else {
            (0..cycles.len()).map(|k| vec![k]).collect()
        };
        'face: for (i, cycle) in cycles.iter().enumerate() {
            for unit in units.iter().filter(|u| !u.contains(&i)) {
                let read = read_cycle(body, face, cycle, unit, band);
                let other = || first_edge(body, &cycles[unit[0]]);
                match read {
                    Read::Face | Read::Exempt => {}
                    Read::NoDoor => break 'face,
                    Read::Off(at) => {
                        if let Some(other) = other() {
                            errors.push(ValidationError::CycleOffFaceSide { face, at, other });
                        }
                    }
                    Read::InBand(at, source) => {
                        if let Some(other) = other() {
                            errors.push(ValidationError::CycleSideUndecided {
                                face,
                                at,
                                other,
                                source,
                            });
                        }
                    }
                }
            }
        }
    }
    errors
}

/// **A torus face's cycles grouped as they separate**: each contractible
/// cycle alone, and the essential pair together. `Err(n)` where the `n`
/// essential cycles are not one pair wound opposite; `None` where a
/// cycle's winding has no stored element to read.
fn torus_units<T: Decide>(
    body: &Body<T>,
    face: FaceKey,
    cycles: &[Vec<HalfEdgeKey>],
) -> Option<Result<Vec<Vec<usize>>, usize>> {
    let mut units = Vec::new();
    let mut essential = Vec::new();
    for (k, cycle) in cycles.iter().enumerate() {
        let winding = crate::surface_boundary::cycle_winding(body, face, cycle)?;
        if winding.is_zero() {
            units.push(vec![k]);
        } else {
            essential.push((k, winding.deck));
        }
    }
    match essential.as_slice() {
        [] => {}
        &[(a, da), (b, db)] if da.compose(db) == crate::joint::Deck::IDENTITY => {
            units.push(vec![a, b]);
        }
        more => return Some(Err(more.len())),
    }
    Some(Ok(units))
}

/// How one cycle reads against a unit of others.
enum Read {
    /// A vertex decided on the face's side.
    Face,
    /// A vertex decided off it.
    Off(crate::entity::VertexKey),
    /// No vertex decided, and the first reading in band.
    InBand(crate::entity::VertexKey, geom_core::Indeterminate),
    /// No vertex decided, none in band.
    Exempt,
    /// No door reads the face.
    NoDoor,
}

/// `cycle` read against the cycles `unit` at each of its vertices in
/// walk order (module docs).
fn read_cycle<T: Decide>(
    body: &Body<T>,
    face: FaceKey,
    cycle: &[HalfEdgeKey],
    unit: &[usize],
    band: Band,
) -> Read {
    use crate::boolean::region::RegionRefusal;
    let mut in_band = None;
    for &h in cycle {
        let Some(vertex) = body.get_half_edge(h).map(|h| h.start) else {
            continue;
        };
        let Some(q) = body.get_vertex(vertex).and_then(|v| body.points.get(v.point)) else {
            continue;
        };
        match crate::boolean::cycle_side(body, face, unit, *q, band) {
            Ok(None) => return Read::NoDoor,
            Ok(Some(Some(true))) => return Read::Face,
            Ok(Some(Some(false))) => return Read::Off(vertex),
            Ok(Some(None)) => {}
            Err(RegionRefusal::Escalated(diag)) => {
                in_band.get_or_insert((vertex, diag));
            }
            Err(
                RegionRefusal::RayExhausted { .. }
                | RegionRefusal::WoundPastPeriod
                | RegionRefusal::Unreadable(_),
            ) => {}
        }
    }
    in_band.map_or(Read::Exempt, |(at, diag)| Read::InBand(at, diag))
}

/// The edge of `cycle`'s first half-edge, which names the cycle.
fn first_edge<T: Decide>(body: &Body<T>, cycle: &[HalfEdgeKey]) -> Option<crate::entity::EdgeKey> {
    body.get_half_edge(*cycle.first()?).map(|h| h.edge)
}
