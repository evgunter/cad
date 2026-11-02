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
//!   the pair is read as one cut, each contractible cycle against it and
//!   against each other contractible cycle alone, and the pair's cycles
//!   against each contractible cycle alone, which is the reading that
//!   sees a contractible cycle wound as an outer loop (its face side the
//!   disc it bounds, holding no essential cycle).
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
//!   face check 6 refused for its orientation, whose cycles all lie off
//!   its side and which is reported once. A plane is check 9's nesting
//!   arm. The vertices read are the body's own, which check 3 holds to
//!   the face's carrier, so no carrier decision is read here
//!   ([`crate::boolean::cycle_side`]).
//!
//! **An open surface's ends lie off the face** (the same clause) is
//! decided by check 6: a planar face's loop windings against its outward
//! normal (`LoopRoleInverted`), and a cylinder's or a cone's boundary
//! material side against its sense (`CurvedSenseInverted`). A face whose
//! loops enclose the surface's end rather than its own region reads
//! inverted there, and this module skips it.

use geom_core::{Band, Decide};

use crate::body::Body;
use crate::boolean::solid_contain::PointInSolidError;
use crate::boolean::{ContainError, CurvedPlacement, FaceContainment};
use crate::entity::{EdgeKey, FaceKey, HalfEdgeKey, VertexKey};
use crate::surface_boundary::WoundCycle;
use crate::validate::ValidationError;

/// **Every region finding on `body`'s curved faces**, in face-arena
/// order (module docs), past `inverted`, the faces check 6 refused for
/// their orientation.
pub(crate) fn region_errors<T: Decide>(
    body: &Body<T>,
    band: Band,
    inverted: &[FaceKey],
) -> Vec<ValidationError> {
    let mut errors = Vec::new();
    for (face, f) in body.faces.iter() {
        if inverted.contains(&face) {
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

/// A cycle read, by its index, and the cut it is read against.
type Reading = (usize, Vec<usize>);

/// **A torus face's readings** (module docs): each cycle read and the
/// cut it is read against, the first cycle of the cut naming it.
/// `Err(n)` where the `n` essential cycles are not one pair wound
/// opposite; `None` where a cycle's winding has no stored element to
/// read.
fn torus_readings(cycles: &[WoundCycle]) -> Option<Result<Vec<Reading>, usize>> {
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
            readings.push((i, vec![k]));
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
            // On a cycle of the cut, or a face no door reads.
            Ok(CurvedPlacement::Trim(
                None | Some(FaceContainment::OnEdge(_) | FaceContainment::OnVertex(_)),
            )) => {}
            Ok(CurvedPlacement::OffCarrier) => {
                unreachable!("a cut reading reads no carrier")
            }
            Err(ContainError::Escalated { diag, .. }) => {
                in_band.get_or_insert((vertex, diag));
            }
            // Unplaced, exempt (the module docs): no path settled; an edge
            // whose carrier's reading cannot cross it; an arc wound past a
            // turn, which the doors carry as `CorruptFace`, as they carry a
            // builder's unresolved key (which tiers 1 and 2 refuse first).
            Err(
                ContainError::RayExhausted
                | ContainError::Curved(
                    PointInSolidError::EdgeCarrierUnsupported { .. }
                    | PointInSolidError::CorruptFace { .. },
                ),
            ) => {}
            Err(other) => unreachable!("the curved doors answer no {other:?}"),
        }
    }
    in_band.map_or(Read::Exempt, |(at, diag)| Read::InBand(at, diag))
}

/// The edge of `cycle`'s first half-edge, which names the cycle.
fn first_edge<T: Decide>(body: &Body<T>, cycle: &[HalfEdgeKey]) -> Option<EdgeKey> {
    body.get_half_edge(*cycle.first()?).map(|h| h.edge)
}

#[cfg(test)]
#[allow(clippy::panic)]
mod tests {
    use super::*;
    use crate::joint::{Deck, Winding};

    fn wound(u: i32) -> WoundCycle {
        let deck = Deck {
            u,
            ..Deck::IDENTITY
        };
        (Vec::new(), Some(Winding { deck, reset: false }))
    }

    /// **The torus readings**: on a band with an essential pair (cycles
    /// 0 and 1, wound ±1) and two contractible rings (2 and 3), each ring
    /// is read against the other alone and against the pair, and each of
    /// the pair against each ring alone; the pair is never read against
    /// itself. The pair's readings are the ones that see a ring wound as
    /// an outer loop.
    #[test]
    fn a_torus_face_reads_each_ring_and_the_pair_against_each_other() {
        let cycles = [wound(1), wound(-1), wound(0), wound(0)];
        let Some(Ok(readings)) = torus_readings(&cycles) else {
            panic!("an essential pair and two rings plan readings")
        };
        let want: Vec<Reading> = vec![
            (2, vec![3]),
            (2, vec![0, 1]),
            (3, vec![2]),
            (3, vec![0, 1]),
            (0, vec![2]),
            (0, vec![3]),
            (1, vec![2]),
            (1, vec![3]),
        ];
        assert_eq!(readings, want);
        let unpaired = [wound(1), wound(1), wound(0)];
        assert_eq!(torus_readings(&unpaired), Some(Err(2)), "wound alike");
        let lone = [wound(1), wound(0)];
        assert_eq!(torus_readings(&lone), Some(Err(1)), "one essential cycle");
        let missing = [wound(1), (Vec::new(), None)];
        assert_eq!(torus_readings(&missing), None, "a winding with no element");
    }
}
