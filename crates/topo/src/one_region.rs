//! **One region per face, off the plane** (DESIGN `[topology.tiers]`,
//! tier 3): a curved face's surface cycles
//! ([`crate::surface_boundary::surface_cycles`]) each lie on the face's
//! side of every other, read by the point-in-face doors with their
//! boundary cut to the other cycles ([`crate::boolean::cycle_side`]).
//!
//! - **What each cycle is read against.** A cycle on a sphere, a cylinder
//!   or a cone separates its surface, so each is read against each other
//!   cycle alone. On a torus a cycle separates only where it is
//!   contractible: one whose joint elements compose to the identity
//!   ([`crate::surface_boundary::surface_cycle_windings`], the lift's
//!   integers, read off the walk and not re-derived). A face holds none
//!   of the essential ones or two wound opposite, which bound one band of
//!   the torus between them ([`ValidationError::EssentialCyclesUnpaired`]);
//!   the pair is read as one cut. A contractible cycle is read against
//!   another with the pair kept in the cut, the band less the other's
//!   region, so that each path from it meets a cycle the band's parallels
//!   bound; an essential one against each contractible cycle alone.
//! - **Where a cycle is read.** At each of its vertices in walk order,
//!   until one decides: on the face's side passes, off it refuses
//!   ([`ValidationError::CycleOffFaceSide`]). A vertex on a cycle of the
//!   cut, or one the door does not place, is set aside. A cycle no vertex
//!   of which is placed refuses as undecided where a reading was in band
//!   ([`ValidationError::CycleSideUndecided`], D4 ¶1), and is exempt
//!   otherwise.
//! - **What is exempt** (validate's not-yet-checked list): a face no door
//!   reads (a spline chart, or an edge of a kind its door does not admit,
//!   the doors' `admits`), a torus face one of whose cycles has no stored
//!   joint element to read its winding from, a reading that does not
//!   place the vertex (no path settled, an arc its door reads as wound
//!   past a turn, or an edge its carrier's reading cannot cross), and a
//!   face check 6 refused for its sense (`CurvedSenseInverted`), whose
//!   cycles all lie off its side and which is reported once. A plane is
//!   check 9's nesting arm.
//!
//! **An open surface's ends lie off the face** (the same clause) is
//! decided by check 6: a planar face's loop windings against its outward
//! normal (`LoopRoleInverted`), and a cylinder's or a cone's boundary
//! material side against its sense (`CurvedSenseInverted`). A face whose
//! loops enclose the surface's end rather than its own region reads
//! inverted there, and this module skips it.

use geom_core::{Band, Decide};

use crate::body::Body;
use crate::boolean::{ContainError, CurvedPlacement, FaceContainment};
use crate::entity::{EdgeKey, FaceKey, HalfEdgeKey, VertexKey};
use crate::validate::ValidationError;

/// **Every region finding on `body`'s curved faces**, in face-arena
/// order (module docs), past `sense_refused`, the faces check 6 refused
/// for their sense.
pub(crate) fn region_errors<T: Decide>(
    body: &Body<T>,
    band: Band,
    sense_refused: &[FaceKey],
) -> Vec<ValidationError> {
    let mut errors = Vec::new();
    for (face, f) in body.faces.iter() {
        if sense_refused.contains(&face) {
            continue;
        }
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
        let Ok(cycles) = crate::surface_boundary::surface_cycle_windings(body, face) else {
            continue; // tier 1 refuses a boundary that does not walk
        };
        let readings = if torus {
            match torus_readings(&cycles) {
                Some(Ok(readings)) => readings,
                Some(Err(essential)) => {
                    errors.push(ValidationError::EssentialCyclesUnpaired { face, essential });
                    continue;
                }
                None => continue, // a cycle with no stored element: exempt
            }
        } else {
            let n = cycles.len();
            (0..n)
                .flat_map(|i| (0..n).filter(move |&k| k != i).map(move |k| (i, vec![k])))
                .collect()
        };
        for (i, cut) in readings {
            let Some(other) = first_edge(body, &cycles[cut[0]].0) else {
                continue; // unreachable: the walk resolved it
            };
            match read_cycle(body, face, &cycles[i].0, &cut, band) {
                Read::Face | Read::Exempt => {}
                Read::Off(at) => errors.push(ValidationError::CycleOffFaceSide { face, at, other }),
                Read::InBand(at, source) => errors.push(ValidationError::CycleSideUndecided {
                    face,
                    at,
                    other,
                    source,
                }),
            }
        }
    }
    errors
}

/// **A torus face's readings** (module docs): each cycle read and the
/// cut it is read against, the first cycle of the cut naming it.
/// `Err(n)` where the `n` essential cycles are not one pair wound
/// opposite; `None` where a cycle's winding has no stored element to
/// read.
#[allow(clippy::type_complexity)] // a cycle and its cut, per reading
fn torus_readings(
    cycles: &[(Vec<HalfEdgeKey>, Option<crate::joint::Winding>)],
) -> Option<Result<Vec<(usize, Vec<usize>)>, usize>> {
    let mut contractible = Vec::new();
    let mut essential = Vec::new();
    for (k, (_, winding)) in cycles.iter().enumerate() {
        let winding = (*winding)?;
        if winding.is_zero() {
            contractible.push(k);
        } else {
            essential.push((k, winding.deck));
        }
    }
    let pair: Vec<usize> = match essential.as_slice() {
        [] => Vec::new(),
        &[(a, da), (b, db)] if da.compose(db) == crate::joint::Deck::IDENTITY => vec![a, b],
        more => return Some(Err(more.len())),
    };
    let mut readings = Vec::new();
    for &i in &contractible {
        for &k in contractible.iter().filter(|&&k| k != i) {
            readings.push((i, core::iter::once(k).chain(pair.iter().copied()).collect()));
        }
        if !pair.is_empty() {
            readings.push((i, pair.clone()));
        }
    }
    for &i in &pair {
        for &k in &contractible {
            readings.push((i, vec![k]));
        }
    }
    Some(Ok(readings))
}

/// How one cycle reads against a cut.
enum Read {
    /// A vertex decided on the face's side.
    Face,
    /// A vertex decided off it.
    Off(VertexKey),
    /// No vertex decided, and the first reading in band.
    InBand(VertexKey, geom_core::Indeterminate),
    /// No vertex decided, none in band.
    Exempt,
}

/// `cycle` read against the cycles `cut` at each of its vertices in walk
/// order (module docs).
fn read_cycle<T: Decide>(
    body: &Body<T>,
    face: FaceKey,
    cycle: &[HalfEdgeKey],
    cut: &[usize],
    band: Band,
) -> Read {
    let mut in_band = None;
    for &h in cycle {
        let Some(vertex) = body.get_half_edge(h).map(|h| h.start) else {
            continue;
        };
        let Some(q) = body
            .get_vertex(vertex)
            .and_then(|v| body.points.get(v.point))
        else {
            continue;
        };
        match crate::boolean::cycle_side(body, face, cut, *q, band) {
            Ok(CurvedPlacement::Trim(Some(FaceContainment::In))) => return Read::Face,
            Ok(CurvedPlacement::Trim(Some(FaceContainment::Out))) => return Read::Off(vertex),
            // On a cycle of the cut, no door, or off the carrier (check 3's).
            Ok(CurvedPlacement::Trim(_) | CurvedPlacement::OffCarrier) => {}
            Err(ContainError::Escalated { diag, .. }) => {
                in_band.get_or_insert((vertex, diag));
            }
            // Unplaced: exempt (the module docs).
            Err(_) => {}
        }
    }
    in_band.map_or(Read::Exempt, |(at, diag)| Read::InBand(at, diag))
}

/// The edge of `cycle`'s first half-edge, which names the cycle.
fn first_edge<T: Decide>(body: &Body<T>, cycle: &[HalfEdgeKey]) -> Option<EdgeKey> {
    body.get_half_edge(*cycle.first()?).map(|h| h.edge)
}
