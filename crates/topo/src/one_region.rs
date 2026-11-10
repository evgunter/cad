//! **One region per face, off the plane** (DESIGN `[topology.tiers]`,
//! tier 3): a curved face's surface cycles
//! ([`crate::surface_boundary::surface_cycles`]) each lie on the face's
//! side of every other, read by the point-in-face doors with their
//! boundary cut to the other cycles ([`crate::contain::cycle_side`]).
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
//! - **Where a cycle is read.** At every one of its vertices
//!   ([`crate::loop_side`]): on the face's side where every vertex placed
//!   reads it; off it at a vertex placed off, whether the cycle lies off
//!   or crosses the cycles it is read against
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
//!   ([`crate::contain::cycle_side`]).
//!
//! **An open surface's ends lie off the face** (the same clause) is
//! decided by check 6: a planar face's loop windings against its outward
//! normal (`LoopRoleInverted`), and a cylinder's or a cone's boundary
//! material side against its sense (`CurvedSenseInverted`). A face whose
//! loops enclose the surface's end rather than its own region reads
//! inverted there, and this module skips it.

use geom_brep::ladder::Kept;
use geom_core::{Band, Decide};

use crate::body::Body;
use crate::contain::{ContainError, CurvedPlacement, FaceContainment};
use crate::entity::{EdgeKey, FaceKey, HalfEdgeKey, VertexKey};
use crate::loop_side::{EveryVertex, inside_at_every_vertex};
use crate::solid_contain::PointInSolidError;
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
/// ([`crate::contain::cycle_side`]): `Ok(Some(true))` on the face's side
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
    match crate::contain::cycle_side(body, face, cut, q, band) {
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

/// `cycle` read against the cycles `cut` at every one of its vertices
/// ([`crate::loop_side::inside_at_every_vertex`]): on the face's side
/// where every vertex placed reads it, off it at a vertex placed off,
/// whether the cycle lies wholly off or crosses the cut. Nothing joins
/// here: the cycles are the face's own, so a crossing is the
/// validator's to find, and no vertex's reading is taken for the rest.
/// A vertex on a cycle of the cut is set aside. An unplaced vertex is
/// exempt and passes, so a cycle no vertex places is exempt unless one
/// read in band.
fn read_cycle<T: Decide>(
    body: &Body<T>,
    face: FaceKey,
    cycle: &[HalfEdgeKey],
    cut: &[usize],
    band: Band,
) -> Read {
    let vertices: Vec<VertexKey> = cycle
        .iter()
        .filter_map(|&h| body.get_half_edge(h).map(|h| h.start))
        .collect();
    match inside_at_every_vertex(&vertices, |vertex| {
        let Some(q) = body
            .get_vertex(vertex)
            .and_then(|v| body.points.get(v.point))
        else {
            return Ok(None);
        };
        door_side(body, face, cut, *q, band).map_err(|diag| Kept::of(diag).map(|d| (vertex, d)))
    }) {
        EveryVertex::Inside => Read::Face,
        EveryVertex::Outside { at } => Read::Off(at),
        EveryVertex::Unplaced => Read::Exempt,
        EveryVertex::Undecided((at, diag)) => Read::InBand(at, diag),
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

    /// Re-carries every edge with an end at `vertex` on the line between
    /// its end points, as certified chords: after a row moves a vertex,
    /// its edges' departures there are read from the point where it now
    /// stands, not from the line it was built on.
    fn recarry_lines_at(
        body: &mut Body<f64>,
        vertex: crate::entity::VertexKey,
        tol: geom_core::Tol,
    ) {
        let edges: Vec<EdgeKey> = body
            .edges
            .iter()
            .filter(|(_, e)| {
                [e.he_plus, e.he_minus]
                    .iter()
                    .any(|&h| body.get_half_edge(h).is_some_and(|h| h.start == vertex))
            })
            .map(|(k, _)| k)
            .collect();
        for edge in edges {
            let e = body.get_edge(edge).expect("the edge").clone();
            let end = |v| {
                let key = body.get_vertex(v).expect("a vertex").point;
                *body.points.get(key).expect("its point")
            };
            let (s, t) = body.edge_vertices(edge).expect("the edge");
            let (s, t) = (end(s), end(t));
            let curve = geom_brep::EdgeCurve::certify(
                geom_brep::EdgeCurveSpec::line_between(s, t),
                s,
                t,
                |_| None,
                geom_core::test_support::fixture_lever(1.0),
                geom_core::Band::linear(tol).expect("the witness band"),
                geom_brep::NurbsLane::certified(),
            )
            .expect("the chord certifies");
            *body.curves.get_mut(e.curve).expect("its curve") =
                crate::null::CurveGeom::Certified(curve);
        }
    }

    /// The findings of a cylinder sheet whose one loop walks two surface
    /// cycles that meet at a point: the outer loop carries a strut from
    /// its corner `B` (the one after `A`, its first corner) to an inner
    /// corner `W`, and a polygon
    /// `W → X → Y`, closed at `W` by a membrane, with the strut kept, so
    /// the surface boundary drops the strut and walks two cycles, the outer
    /// one and the polygon (asserted). Where `meet`, `X` is then moved onto
    /// `A`'s point and its edges re-carried ([`recarry_lines_at`]), so the loop passes it at two corners. `y` is the polygon's third
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
        let (body, face, a_point) =
            strut_split_sheet(y, if meet { Meet::Shared } else { Meet::AsBuilt });
        let errors = crate::validate_geometric(&body, geom_core::Tol::witness())
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

    /// Where the polygon's corner `X` stands in [`strut_split_sheet`].
    #[derive(Clone, Copy)]
    enum Meet {
        /// Where it was built, apart from `A`.
        AsBuilt,
        /// On `A`'s point key.
        Shared,
        /// On a key of its own, this far from `A`'s point along the
        /// cylinder's axis.
        Near(f64),
    }

    /// The body of [`one_loop_two_cycles_at_a_point`], `X` standing as
    /// `meet` says: the sheet, its face and `A`'s point key.
    fn strut_split_sheet(
        y: (f64, f64),
        meet: Meet,
    ) -> (Body<f64>, FaceKey, crate::geometry::PointKey) {
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
        let target = match meet {
            Meet::AsBuilt => None,
            Meet::Shared => Some(a_point),
            Meet::Near(d) => {
                let a = *body.points.get(a_point).expect("A's point");
                Some(body.points.insert(a + geom_core::Vec3::new(0.0, 0.0, d)))
            }
        };
        if let Some(key) = target {
            let gone = body.get_vertex(x_vertex).expect("X").point;
            body.vertices.get_mut(x_vertex).expect("X").point = key;
            body.points.remove(gone);
            recarry_lines_at(&mut body, x_vertex, tol);
        }
        (body, face, a_point)
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
    /// lobes' edges, each a slice (Ev's rule, PR 4057), and passes, on a
    /// plane (`validate::tests::a_figure_eight_hole_with_a_closed_arc_passes`)
    /// and on this cylinder
    /// ([`a_figure_eight_ring_on_a_cylinder_passes_and_its_crossed_twin_refuses`]).
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

    /// How `read_cycle` reads each of `face`'s surface cycles against
    /// each other, started at every one of its vertices in turn: per
    /// (cycle, cut), the verdicts the rotations give, as `Face`, `Off`,
    /// `InBand` or `Exempt`.
    fn verdicts_in_every_walk_order(
        body: &Body<f64>,
        face: FaceKey,
    ) -> Vec<((usize, usize), Vec<&'static str>)> {
        let band = geom_core::Band::linear(geom_core::Tol::witness()).expect("the band");
        let cycles =
            crate::surface_boundary::surface_cycle_windings(body, face).expect("the cycles");
        let mut out = Vec::new();
        for i in 0..cycles.len() {
            for k in (0..cycles.len()).filter(|&k| k != i) {
                let cycle = &cycles[i].0;
                let verdicts = (0..cycle.len())
                    .map(|r| {
                        let rotated: Vec<_> =
                            cycle[r..].iter().chain(&cycle[..r]).copied().collect();
                        match read_cycle(body, face, &rotated, &[k], band) {
                            Read::Face => "Face",
                            Read::Off(_) => "Off",
                            Read::InBand(..) => "InBand",
                            Read::Exempt => "Exempt",
                        }
                    })
                    .collect();
                out.push(((i, k), verdicts));
            }
        }
        out
    }

    /// Every (cycle, cut) reads one verdict in every walk order, and some
    /// cycle reads off the face's side in every one: the crossing is
    /// refused, whichever vertex the walk starts at.
    fn assert_one_verdict_and_a_refusal(readings: &[((usize, usize), Vec<&'static str>)]) {
        for (pair, verdicts) in readings {
            assert!(
                verdicts.iter().all(|v| *v == verdicts[0]),
                "cycle {pair:?} reads one verdict in every walk order: {verdicts:?}"
            );
        }
        assert!(
            readings.iter().any(|(_, v)| v.iter().all(|v| *v == "Off")),
            "a cycle reads off the face's side in every walk order: {readings:?}"
        );
    }

    /// **A loop through one place at two point keys is read the same in
    /// every walk order**.
    /// The strut-split sheet of [`one_loop_two_cycles_at_a_point`], its
    /// polygon's corner `X` put 1e-12 from the outer corner `A` on a key
    /// of its own, and the polygon crossing the outer cycle there (its
    /// third corner outside the sheet). Check 9's corner arm groups
    /// corners by key, so it reads nothing at `A`. The outer cycle has
    /// vertices on both sides of the polygon, so it reads off the face's
    /// side whichever vertex the walk starts at.
    #[test]
    fn a_loop_crossing_itself_at_two_keys_within_band_reads_one_verdict_in_every_walk_order() {
        let (body, face, a_point) = strut_split_sheet((-0.5, 0.5), Meet::Near(1e-12));
        let errors = crate::validate_geometric(&body, geom_core::Tol::witness())
            .err()
            .unwrap_or_default();
        assert!(
            !errors
                .iter()
                .any(|e| matches!(e, ValidationError::PinchCornerCrossed { .. })),
            "the corner arm reads no pinch at two keys: {errors:?}"
        );
        let _ = a_point;
        assert_one_verdict_and_a_refusal(&verdicts_in_every_walk_order(&body, face));
    }

    /// **A cone sector sheet whose one loop passes the apex at two
    /// corners**: the cone of half-angle 0.5 about `+z` from the origin,
    /// its sector `u ∈ [0, π/2]` up to `z = 1` bounded by two generators
    /// and the rim arc, the apex its first corner. As in
    /// [`strut_split_sheet`], a strut leaves the rim corner `P` to an
    /// inner corner `W`, and a polygon `W → X → Y` is closed by a
    /// membrane with the strut kept, so the surface boundary walks two
    /// cycles; `X` is then moved onto the apex's key. `Y` at `u = −π/8`
    /// lies outside the sector, so the polygon's corner at the apex
    /// (generators `π/4` and `−π/8`) interleaves with the sector's (`0`
    /// and `π/2`): the cycles cross there. The polygon's edges are
    /// straight chords between points of the cone, so the body carries
    /// other findings; a row reads only the corner arm and the cycles.
    fn cone_apex_crossing_sheet() -> (Body<f64>, FaceKey, crate::geometry::PointKey) {
        use crate::construction::{FaceSurface, MefSite, MevSite};
        use geom_core::{Point3, Vec3};
        let tol = geom_core::Tol::witness();
        let alpha: f64 = 0.5;
        let at = |u: f64, z: f64| {
            let r = z * alpha.tan();
            Point3::new(r * u.cos(), r * u.sin(), z)
        };
        let quarter = core::f64::consts::FRAC_PI_2;
        let mut body = Body::<f64>::new();
        let seed = body.mvfs(Point3::origin(), true).expect("the apex");
        let cone = body
            .set_face_surface(
                seed.face,
                FaceSurface::New {
                    surface: geom::Surface::Cone {
                        apex: Point3::origin(),
                        axis: Vec3::unit_z(),
                        half_angle: alpha,
                        u_ref: Vec3::unit_x(),
                    },
                    sense: true,
                },
            )
            .expect("the cone");
        let to_p = body
            .mev_line(
                MevSite::Lone {
                    r#loop: seed.r#loop,
                },
                at(0.0, 1.0),
                tol,
            )
            .expect("the first generator");
        let plane = body.add_surface(geom::Surface::Plane {
            origin: Point3::new(0.0, 0.0, 1.0),
            normal: Vec3::unit_z(),
            u_ref: Vec3::unit_x(),
        });
        let rim = geom_brep::EdgeCurveSpec {
            description: geom_brep::EdgeDescriptionSpec::Intersection {
                pair: geom_brep::SurfacePair::new(cone, plane),
                selector: geom_brep::CurveSelector::Witness(at(quarter / 2.0, 1.0)),
            },
            carrier: geom::Curve3::circle(
                Point3::new(0.0, 0.0, 1.0),
                Vec3::unit_z(),
                alpha.tan(),
                Vec3::unit_x(),
            ),
            param_start: 0.0,
            param_end: quarter,
        };
        let to_q = body
            .mev(
                MevSite::Fan {
                    he1: to_p.he_minus,
                    he2: to_p.he_minus,
                },
                at(quarter, 1.0),
                rim,
                tol,
            )
            .expect("the rim");
        let from_q = body
            .find_half_edge(seed.face, to_q.vertex, to_p.vertex)
            .expect("Q's half-edge");
        let face = body
            .mef(
                MefSite::Chords {
                    he1: from_q,
                    he2: to_p.he_plus,
                },
                geom_brep::EdgeCurveSpec::line_between(at(quarter, 1.0), Point3::origin()),
                FaceSurface::Shared {
                    key: cone,
                    sense: true,
                },
                tol,
            )
            .expect("the second generator")
            .face;
        let apex_point = body.get_vertex(seed.vertex).expect("the apex").point;
        crate::test_support_fixtures::set_sheet_sense(&mut body, face, true, tol);
        let outer = body.get_face(face).expect("the sheet").outer;
        let crate::entity::LoopBoundary::Cycle { first } =
            body.get_loop(outer).expect("its outer loop").boundary
        else {
            panic!("the sheet's outer loop is a cycle")
        };
        let mut from_p = first;
        while body.get_half_edge(from_p).expect("a half-edge").start != to_p.vertex {
            from_p = body.get_half_edge(from_p).expect("a half-edge").next;
            assert_ne!(from_p, first, "the sheet's loop passes P");
        }
        let strut = body
            .mev_line(
                MevSite::Fan {
                    he1: from_p,
                    he2: from_p,
                },
                at(quarter / 2.0, 0.6),
                tol,
            )
            .expect("the strut");
        let x = body
            .mev_line(
                MevSite::Fan {
                    he1: strut.he_minus,
                    he2: strut.he_minus,
                },
                at(quarter / 4.0, 0.3),
                tol,
            )
            .expect("W to X");
        let yv = body
            .mev_line(
                MevSite::Fan {
                    he1: x.he_minus,
                    he2: x.he_minus,
                },
                at(-quarter / 4.0, 0.5),
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
        let x_vertex = body.get_half_edge(x.he_minus).map(|h| h.start).expect("X");
        let gone = body.get_vertex(x_vertex).expect("X").point;
        body.vertices.get_mut(x_vertex).expect("X").point = apex_point;
        body.points.remove(gone);
        recarry_lines_at(&mut body, x_vertex, tol);
        let cycles = crate::surface_boundary::surface_cycles(&body, face).expect("the cycles");
        assert_eq!(cycles.len(), 2, "one loop walks two surface cycles");
        (body, face, apex_point)
    }

    /// **Two cycles crossing at a cone's apex are read the same in every
    /// walk order**:
    /// [`cone_apex_crossing_sheet`]. The corner arm is silent at the apex,
    /// which has no normal. A cycle with vertices on both sides of the
    /// other reads off the face's side whichever vertex the walk starts
    /// at.
    #[test]
    fn two_cycles_crossing_at_a_cones_apex_read_one_verdict_in_every_walk_order() {
        let (body, face, apex) = cone_apex_crossing_sheet();
        let errors = crate::validate_geometric(&body, geom_core::Tol::witness())
            .err()
            .unwrap_or_default();
        assert!(
            !errors.iter().any(|e| matches!(e,
                ValidationError::PinchCornerCrossed { vertex, .. }
                    if body.get_vertex(*vertex).map(|v| v.point) == Some(apex))),
            "the corner arm is silent at the apex: {errors:?}"
        );
        assert_one_verdict_and_a_refusal(&verdicts_in_every_walk_order(&body, face));
    }

    /// The findings at `P` of a cylinder sheet (sensed, as in
    /// [`one_loop_two_cycles_at_a_point`]) carrying one ring through `P`
    /// twice: a hexagon pinched at its waist, `P → R1 → R2 → Q → L2 → L1`
    /// in the sheet's `(u, v)`, its two lobes either side of `u = 1.5`,
    /// with `Q` then moved onto `P`'s key and its edges re-carried
    /// ([`recarry_lines_at`]). Where `crossed`, the right lobe is walked
    /// the other way (`R2` before `R1`), so each corner at `P` bridges the
    /// lobes the wrong way. The rim's order is the hole's: walked the
    /// other way round, the ring bounds its lobes rather than the sheet
    /// outside them, every corner at `P` is its complement and both rows
    /// refuse (measured; check 6 does not refuse that role on this
    /// chord-built sheet). Returns the `PinchCornerCrossed` findings at
    /// `P` and whether check 6 refused the face's orientation, which would
    /// leave its corners unread. The edges are straight chords, as in the
    /// strut rows, and only check 9's corner verdict at `P` is read.
    fn figure_eight_ring_at_a_point(crossed: bool) -> (Vec<ValidationError>, bool) {
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
        crate::test_support_fixtures::set_sheet_sense(&mut body, face, true, tol);
        let outer = body.get_face(face).expect("the sheet").outer;
        let crate::entity::LoopBoundary::Cycle { first } =
            body.get_loop(outer).expect("its outer loop").boundary
        else {
            panic!("the sheet's outer loop is a cycle")
        };
        let (p, q) = (frame.at(1.5, 0.45), frame.at(1.5, 0.55));
        let (r1, r2) = (frame.at(2.0, 0.2), frame.at(2.0, 0.8));
        let right = if crossed { [r2, r1] } else { [r1, r2] };
        let rim = [
            p,
            right[0],
            right[1],
            q,
            frame.at(1.0, 0.8),
            frame.at(1.0, 0.2),
        ];
        plant_ring_face(&mut body, first, &rim, tol);
        let ring = *body
            .get_face(face)
            .expect("the sheet")
            .rings
            .first()
            .expect("the sheet holds the ring");
        let crate::entity::LoopBoundary::Cycle { first: start } =
            body.get_loop(ring).expect("the ring").boundary
        else {
            panic!("the ring is a cycle")
        };
        let mut cycle = vec![start];
        loop {
            let next = body
                .get_half_edge(*cycle.last().expect("a half-edge"))
                .expect("a half-edge")
                .next;
            if next == start {
                break;
            }
            cycle.push(next);
        }
        let at = |body: &Body<f64>, w: geom_core::Point3<f64>| {
            cycle
                .iter()
                .map(|&h| body.get_half_edge(h).expect("a half-edge").start)
                .find(|&v| {
                    let key = body.get_vertex(v).expect("a vertex").point;
                    body.points.get(key).is_some_and(|x| x.distance(w) < 1e-12)
                })
                .expect("a corner of the ring")
        };
        let (vp, vq) = (at(&body, p), at(&body, q));
        let p_key = body.get_vertex(vp).expect("P").point;
        let gone = body.get_vertex(vq).expect("Q").point;
        body.vertices.get_mut(vq).expect("Q").point = p_key;
        body.points.remove(gone);
        recarry_lines_at(&mut body, vq, tol);
        let errors = crate::validate_geometric(&body, tol)
            .err()
            .unwrap_or_default();
        let inverted = errors.iter().any(|e| {
            matches!(e,
                ValidationError::LoopRoleInverted { face: f, .. }
                | ValidationError::CurvedSenseInverted { face: f } if *f == face)
        });
        let crossed_at_p = errors
            .iter()
            .filter(|e| match e {
                ValidationError::PinchCornerCrossed {
                    face: f, vertex, ..
                } => *f == face && body.get_vertex(*vertex).map(|v| v.point) == Some(p_key),
                _ => false,
            })
            .cloned()
            .collect();
        (crossed_at_p, inverted)
    }

    /// **The legal pinch passes on a curved face** (Ev's rule, PR 4057,
    /// on a cylinder): one ring through `P` at two corners, each pairing
    /// the end of one lobe with the start of the other, both lobes walked
    /// alike ([`figure_eight_ring_at_a_point`]), has no corner holding the
    /// other's sides, and check 9 refuses nothing at `P`. Its twin, the
    /// right lobe walked the other way, crosses at `P` and is refused, so
    /// the corner arm reads this face (check 6 refuses neither
    /// orientation). The positive half the strut rows above cannot show.
    #[test]
    fn a_figure_eight_ring_on_a_cylinder_passes_and_its_crossed_twin_refuses() {
        let (legal, inverted) = figure_eight_ring_at_a_point(false);
        assert!(!inverted, "check 6 reads the legal sheet's orientation");
        assert!(legal.is_empty(), "the legal pinch passes at P: {legal:?}");
        let (cross, inverted) = figure_eight_ring_at_a_point(true);
        assert!(!inverted, "check 6 reads the crossed sheet's orientation");
        assert!(
            !cross.is_empty(),
            "check 9's corner arm refuses the crossed twin at P"
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
