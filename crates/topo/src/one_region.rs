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

use geom_brep::ladder::{Answering, Climb, Rung, climb};
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
            | geom::Surface::Cone { .. }
            | geom::Surface::Spindle { .. } => false,
            geom::Surface::Torus { .. } => true,
            geom::Surface::Plane { .. } | geom::Surface::Nurbs(_) | geom::Surface::Approx(_) => {
                continue;
            }
        };
        let Ok(cycles) = crate::surface_boundary::surface_cycle_windings(body, face) else {
            continue; // a rotation that does not close: tier 1 refuses it
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

/// **The face's door at `q`, its surface cycles `cut` kept**
/// ([`crate::boolean::cycle_side`]): `Ok(Some(true))` on the face's side
/// of them, `Ok(Some(false))` off it, `Ok(None)` unplaced. The one mapping
/// of the door's answer both of tier 3's region readings take: this
/// module's cycles, and a chart singularity's side (`crate::validate`'s
/// check 6). Unplaced, exempt (the module docs): on a kept cycle by the
/// door's own boundary reading, a face no door reads, no path settled,
/// an edge whose carrier's reading cannot cross it, or an arc wound past
/// a turn (which the doors carry as `CorruptFace`, as they carry a
/// builder's unresolved key, which tiers 1 and 2 refuse first).
///
/// # Errors
///
/// A reading in band, with its escalation.
///
/// # Panics
///
/// On an answer the cut reading does not give: an edge or a vertex (it
/// reads the region alone, whose boundary reading answers `None`), a
/// carrier, or a door refusal other than those above.
pub(crate) fn door_side<T: Decide>(
    body: &Body<T>,
    face: FaceKey,
    cut: &[usize],
    q: geom_core::Point3<T>,
    band: Band,
) -> Result<Option<bool>, geom_core::Indeterminate> {
    match crate::boolean::cycle_side(body, face, cut, q, band) {
        Ok(CurvedPlacement::Trim(Some(FaceContainment::In))) => Ok(Some(true)),
        Ok(CurvedPlacement::Trim(Some(FaceContainment::Out))) => Ok(Some(false)),
        Ok(CurvedPlacement::Trim(None)) => Ok(None),
        Ok(CurvedPlacement::Trim(Some(
            FaceContainment::OnEdge(_) | FaceContainment::OnVertex(_),
        ))) => unreachable!("a cut reading reads the region alone, which places no edge"),
        Ok(CurvedPlacement::OffCarrier) => unreachable!("a cut reading reads no carrier"),
        Err(ContainError::Escalated { diag, .. }) => Err(diag),
        Err(
            ContainError::RayExhausted
            | ContainError::Curved(
                PointInSolidError::EdgeCarrierUnsupported { .. }
                | PointInSolidError::CorruptFace { .. },
            ),
        ) => Ok(None),
        Err(other) => unreachable!("the curved doors answer no {other:?}"),
    }
}

/// `cycle` read against the cycles `cut` at each of its vertices in walk
/// order (module docs): the ladder ([`climb`]) whose answer is the first
/// vertex placed. Every vertex of a cycle reads the one side
/// ([`Answering::Any`]; the vertex an `Off` names is its witness) where
/// the cycle does not cross the cut: a cycle that only touches it, at a
/// vertex it shares with a cycle of the cut (set aside), lies on one
/// side (`chord_join::paths_rung`'s premise). Nothing joins here: the
/// cycles are the face's own, so a crossing is the validator's to find.
/// One inside two edges is the census's `EdgeEdgeCross`, and two loops
/// meeting at a point are check 9's contact arm 1 (`RingMeetsOuter`,
/// `RingMeetsRing`). Two surface cycles of one loop, split by a seam or
/// a strut, that share a point key pass it at two corners of the loop,
/// and a crossing there is check 9's `PinchCornerCrossed`, save where that
/// arm is silent. None reaches this read alone except in those cases:
/// `pinch_corner_errors`' residue, a cone's apex (which this read does
/// read: `work/onecopy/two-cycles-crossing-at-a-cones-apex-are-read-by-their-first-vertex.md`),
/// a NURBS or `Approx` face, a side along a NURBS edge, two sides leaving
/// along one tangent, and a cusp corner; and a loop passing one place
/// twice through two distinct point keys within band, since the arm
/// groups its corners by key equality
/// (`work/onecopy/a-loop-through-one-place-at-two-point-keys-is-read-by-its-first-vertex.md`).
/// So a
/// vertex read in band ahead of the answer is superseded. An unplaced
/// vertex is exempt and passes, so a cycle no vertex places is exempt
/// unless one read in band.
fn read_cycle<T: Decide>(
    body: &Body<T>,
    face: FaceKey,
    cycle: &[HalfEdgeKey],
    cut: &[usize],
    band: Band,
) -> Read {
    let placed = climb(Answering::Any, cycle, |&h| {
        let Some(vertex) = body.get_half_edge(h).map(|h| h.start) else {
            return Rung::Passes;
        };
        let Some(q) = body
            .get_vertex(vertex)
            .and_then(|v| body.points.get(v.point))
        else {
            return Rung::Passes;
        };
        Rung::of_reading(door_side(body, face, cut, *q, band).map(|side| {
            side.map(|on_face| {
                if on_face {
                    Read::Face
                } else {
                    Read::Off(vertex)
                }
            })
        }))
        .map_kept(|diag| (vertex, diag))
    });
    match placed {
        Climb::Answered { answer, .. } => answer,
        Climb::Passed => Read::Exempt,
        Climb::Undecided((at, diag)) => Read::InBand(at, diag),
    }
}

/// The edge of `cycle`'s first half-edge, which names the cycle.
fn first_edge<T: Decide>(body: &Body<T>, cycle: &[HalfEdgeKey]) -> Option<EdgeKey> {
    body.get_half_edge(*cycle.first()?).map(|h| h.edge)
}

#[cfg(test)]
#[allow(clippy::panic, clippy::expect_used)]
mod tests {
    use super::*;
    use crate::joint::{Deck, Winding};

    /// The findings of a cylinder sheet whose one loop walks two surface
    /// cycles that meet at a point: the outer loop carries a strut from
    /// its corner `B` (the one after `A`, its first corner) to an inner
    /// corner `W`, and a polygon
    /// `W → X → Y`, closed at `W` by a membrane, with the strut kept, so
    /// the surface boundary drops the strut and walks two cycles, the outer
    /// one and the polygon (asserted). Where `meet`, `X` is then moved onto
    /// `A`'s point, so the loop passes it at two corners. `y` is the polygon's third
    /// corner in the sheet's `(u, v)`. The sheet's sense is set so its
    /// loop bounds its rectangle (an inverted sheet's corners are their
    /// complements, which the corner arm reads as crossings everywhere).
    /// The edges are deliberately straight chords between points of the
    /// cylinder, so the body carries description, scaffold and chart
    /// findings too; a row reads only check 9's corner arm at `A`.
    fn one_loop_two_cycles_at_a_point(
        y: (f64, f64),
        meet: bool,
    ) -> (Vec<ValidationError>, crate::geometry::PointKey) {
        use crate::construction::{MefSite, MevSite};
        use crate::test_support_fixtures::{CylFrame, cyl_wall_sheet};
        let tol = geom_core::Tol::witness();
        let frame = CylFrame::canonical(1.0);
        let mut body = Body::<f64>::new();
        let face = cyl_wall_sheet(
            &mut body,
            frame,
            (0.0, core::f64::consts::PI),
            (0.0, 1.0),
            tol,
        );
        crate::test_support_fixtures::set_sheet_sense(&mut body, face, true, tol);
        let outer = body.get_face(face).expect("the sheet").outer;
        let crate::entity::LoopBoundary::Cycle { first } =
            body.get_loop(outer).expect("its outer loop").boundary
        else {
            panic!("the sheet's outer loop is a cycle")
        };
        let a = body.get_half_edge(first).expect("a half-edge").start;
        let a_point = body.get_vertex(a).expect("A").point;
        // The strut leaves the next corner `B`, so no polygon edge runs
        // along it once `X` stands on `A`.
        let from_b = body.get_half_edge(first).expect("A's half-edge").next;
        let strut = body
            .mev_line(
                MevSite::Fan {
                    he1: from_b,
                    he2: from_b,
                },
                frame.at(0.5, 0.5),
                tol,
            )
            .expect("the strut");
        let x = body
            .mev_line(
                MevSite::Fan {
                    he1: strut.he_minus,
                    he2: strut.he_minus,
                },
                frame.at(0.3, 0.1),
                tol,
            )
            .expect("W to X");
        let yv = body
            .mev_line(
                MevSite::Fan {
                    he1: x.he_minus,
                    he2: x.he_minus,
                },
                frame.at(y.0, y.1),
                tol,
            )
            .expect("X to Y");
        body.mef_chord(
            MefSite::Chords {
                he1: x.he_plus,
                he2: yv.he_minus,
            },
            tol,
        )
        .expect("the membrane");
        let cycles = crate::surface_boundary::surface_cycles(&body, face).expect("the cycles");
        assert_eq!(cycles.len(), 2, "one loop walks two surface cycles");
        let x_vertex = body.get_half_edge(x.he_minus).map(|h| h.start).expect("X");
        if meet {
            let gone = body.get_vertex(x_vertex).expect("X").point;
            body.vertices.get_mut(x_vertex).expect("X").point = a_point;
            body.points.remove(gone);
        }
        let errors = crate::validate_geometric(&body, tol)
            .err()
            .unwrap_or_default();
        let crossed_at_a = errors
            .iter()
            .filter(|e| match e {
                ValidationError::PinchCornerCrossed {
                    face: f, vertex, ..
                } => *f == face && body.get_vertex(*vertex).map(|v| v.point) == Some(a_point),
                _ => false,
            })
            .cloned()
            .collect();
        (crossed_at_a, a_point)
    }

    /// **Two surface cycles of ONE loop that meet at a point key are
    /// check 9's `PinchCornerCrossed`** ([`one_loop_two_cycles_at_a_point`]).
    /// Apart, the polygon's corner `X` off the outer corner `A`, nothing is
    /// refused at `A`. Moved onto `A`'s key, crossing the outer cycle there
    /// (its third corner outside the sheet) or touching it (inside), the
    /// corner arm refuses at `A`. Neither is a legal pinch: the strut
    /// splits the loop so that each cycle's corner at `A` pairs that
    /// cycle's own two edges, and the outer corner's wedge holds the
    /// polygon's edges there, so neither corner is a slice of the face. A
    /// legal pinch is one loop whose corners at the point pair the two
    /// lobes' edges, each a slice (Ev's rule, PR 4057), and passes
    /// (`validate::tests::a_figure_eight_hole_with_a_closed_arc_passes`).
    /// The strut leaves another corner `B`, so no polygon edge runs along
    /// it. The body is deliberately ill-formed (scaffold line chords
    /// between points of the cylinder), and only check 9's corner verdict
    /// at `A` is read.
    #[test]
    fn two_surface_cycles_of_one_loop_crossing_at_a_shared_point_are_check_9s() {
        let (apart, _) = one_loop_two_cycles_at_a_point((-0.5, 0.5), false);
        assert!(
            apart.is_empty(),
            "the cycles apart, nothing at A: {apart:?}"
        );
        let (cross, _) = one_loop_two_cycles_at_a_point((-0.5, 0.5), true);
        assert!(
            !cross.is_empty(),
            "check 9's corner arm refuses the crossing at A"
        );
        let (touch, _) = one_loop_two_cycles_at_a_point((0.5, 0.1), true);
        assert!(
            !touch.is_empty(),
            "check 9's corner arm refuses the touch at A"
        );
    }

    /// **Two cycles crossing at points they share are check 9's
    /// `RingMeetsOuter`**, so `read_cycle` never stands alone on them. A
    /// cylinder sheet over `u ∈ [0, π]`, `v ∈ [0, 1]` carries a ring
    /// through two of its outer corners, at `(0, 0)` and `(0, 1)`, with one
    /// corner inside the sheet and one outside it: the ring crosses the
    /// outer loop at those two points and nowhere else. Arm 1 of check 9's
    /// contact half reads points, whatever the carrier, and refuses the
    /// ring standing on the outer vertex.
    /// The ring's edges are deliberately straight chords between points of
    /// the cylinder, so the body carries description, scaffold and chart
    /// findings too; the row reads only arm 1's verdict.
    #[test]
    fn a_ring_crossing_its_outer_loop_at_shared_points_is_check_9s() {
        use crate::test_support_fixtures::{CylFrame, cyl_wall_sheet, plant_ring_face};
        let tol = geom_core::Tol::witness();
        let frame = CylFrame::canonical(1.0);
        let mut body = Body::<f64>::new();
        let face = cyl_wall_sheet(
            &mut body,
            frame,
            (0.0, core::f64::consts::PI),
            (0.0, 1.0),
            tol,
        );
        let outer = body.get_face(face).expect("the sheet").outer;
        let crate::entity::LoopBoundary::Cycle { first } =
            body.get_loop(outer).expect("its outer loop").boundary
        else {
            panic!("the sheet's outer loop is a cycle")
        };
        let rim = [
            frame.at(0.5, 0.5),
            frame.at(0.0, 1.0),
            frame.at(-0.5, 0.5),
            frame.at(0.0, 0.0),
        ];
        plant_ring_face(&mut body, first, &rim, tol);
        let errors = crate::validate_geometric(&body, tol).expect_err("the ring crosses");
        assert!(
            errors.iter().any(|e| matches!(
                e,
                crate::ValidationError::RingMeetsOuter {
                    face: f,
                    contact: crate::RingContact::Vertex { .. },
                    ..
                } if *f == face
            )),
            "check 9 refuses the ring on the outer vertex: {errors:?}"
        );
    }

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
