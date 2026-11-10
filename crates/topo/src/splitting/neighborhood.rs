//! The ON-vertex neighbourhood: the vertex's [`Star`] read against the
//! split plane, each bound's side before rules (a) and (b).
//!
//! The walk, its convex sectors and their refinement are the star's
//! ([`crate::star`]); the plane is the datum it is refined against. What
//! is the split's here is its reading of each bound's side: a line edge's
//! from its far vertex's record (a straight edge's interior is its
//! chord), every other bound's through the one bound-side reading
//! ([`crate::bound_side::read_plane_side`], with its descent), its error
//! vocabulary, and the carriers it cuts (plane, cylinder, cone).
//!
//! # Sector geometry under OUR conventions (derived, mirror-checked)
//!
//! The orbit ([`crate::Body::vertex_orbit`], step `next(mate(he))`)
//! visits the half-edges leaving the base vertex `v` **clockwise viewed
//! from outside**. For orbit-consecutive `he_i`, `he_{i+1}`, the sector
//! between them is the corner of the face traversed
//! `… mate(he_i) → he_{i+1} …` — so the **sector's face is
//! `face(loop(mate(he_i)))`** (equivalently `face(loop(he_{i+1}))`),
//! uniform for a divider because it shares `he_i`. By the interior-left
//! rule the sector's interior directions sweep **from `dir(he_{i+1})`
//! counterclockwise (around the face's outward normal `n`) to
//! `dir(he_i)`** — the corner between the loop's incoming direction
//! reversed and its outgoing direction.

use geom_core::{Band, Decide, Indeterminate};
use slotmap::SecondaryMap;

use super::rules;
use super::{PlaneSide, SectorReading, SplitPlane, SplitReduceError};
use crate::body::Body;
use crate::bound_side::{BoundSide, BoundSupport, Descent, Side};
use crate::entity::{HalfEdgeKey, VertexKey};
use crate::sector_face::{SectorCarrier, SectorFaceError};
use crate::sector_shape::SlitTips;
use crate::star::{Bound, Datum, Star, StarFault};
use geom_brep::sector_shape::SectorFault;

impl SectorFaceError {
    /// The split's refusal of `vertex`, whose sector the walk could not
    /// resolve: a kind the split does not cut, or a cone's apex undecided.
    ///
    /// A crease's reading is not carried: only a spline face has one, and
    /// the carrier gate ([`super::classify::carrier_gate`]) refuses a
    /// spline face wherever the plane may meet its padded reach box, which
    /// holds any of its corners on the plane, before any sector is read.
    /// So the face's kind is the refusal whatever the reading, a row-2
    /// "not built" (`[failure.taxonomy]`).
    pub(crate) fn split_refusal(self, vertex: VertexKey) -> SplitReduceError {
        match self {
            Self::Unsupported { face, kind }
            | Self::Crease { face, kind, .. }
            | Self::Unplaced { face, kind } => {
                SplitReduceError::CurvedBooleanUnsupported { face, kind }
            }
            Self::ApexUndecided { face, diag } => {
                SplitReduceError::ApexUndecided { vertex, face, diag }
            }
        }
    }
}

impl StarFault {
    /// The split's refusal of `vertex`, whose star the walk did not read.
    pub(crate) fn split_refusal(self, vertex: VertexKey, band: Band) -> SplitReduceError {
        let sliver = |face, diag| SplitReduceError::SliverSector {
            vertex,
            face,
            reading: SectorReading::Escalated(diag),
        };
        match self {
            Self::Lone => SplitReduceError::LoneVertex { vertex },
            Self::NullEdge(edge) => SplitReduceError::NullEdgeAtVertex { vertex, edge },
            Self::Face(e) => e.split_refusal(vertex),
            Self::NearAxis(face) => SplitReduceError::VertexNearConeAxis { vertex, face, band },
            Self::Shape(face, SectorFault::NonFiniteChord) => {
                SplitReduceError::NonFiniteSectorChord { vertex, face }
            }
            Self::Shape(face, SectorFault::UnderflowedChord) => {
                SplitReduceError::UnderflowedSectorChord { vertex, face }
            }
            Self::Shape(face, SectorFault::Rung { diag, .. }) | Self::Cut(face, diag) => {
                sliver(face, diag)
            }
        }
    }
}

/// **The split's star of `vertex`**: its walk refined against the
/// plane, every sector's carrier one the split cuts (a sphere, a torus
/// and a spline refuse here, C12.1, per arm: the shared walk serves the
/// Boolean's carriers, and the split's gate refuses those kinds wherever
/// the plane may meet them).
///
/// # Errors
///
/// [`StarFault::split_refusal`]'s, and the carrier refusals.
pub(super) fn split_star<T: Decide>(
    body: &Body<T>,
    normal: geom_core::Vec3<T>,
    vertex: VertexKey,
    tips: &SlitTips,
    band: Band,
) -> Result<Star<T>, SplitReduceError> {
    let refused = |fault: StarFault| fault.split_refusal(vertex, band);
    let mut star = Star::walk(body, vertex, tips, band).map_err(refused)?;
    for s in &star.sectors {
        let kind = match s.carrier {
            SectorCarrier::Plane | SectorCarrier::Cylinder | SectorCarrier::Cone => continue,
            SectorCarrier::Sphere => geom::SurfaceKind::Sphere,
            SectorCarrier::Torus => geom::SurfaceKind::Torus,
            SectorCarrier::Spline(kind) => kind,
        };
        return Err(SplitReduceError::CurvedBooleanUnsupported { face: s.face, kind });
    }
    star.refine(&[Datum::Plane(normal)], band)
        .map_err(refused)?;
    Ok(star)
}

/// Why [`bound_side`] read no side: the far vertex's (`far`'s), or an
/// escalated reading, which the caller names its sector for.
#[derive(Debug)]
pub(crate) enum EntryFault<E> {
    /// The far vertex's side did not read.
    Far(E),
    /// A curved edge's or a divider's reading lies in the band.
    Undecided(Indeterminate),
}

/// **Which side of the plane a bound of `vertex`'s star leaves into**:
/// its class before rules (a) and (b), the one reading the split's
/// neighbourhood and [`crate::vertex_section`] both take.
///
/// - A LINE edge: its far vertex's side (`far`); a straight edge's
///   interior is the chord.
/// - Every other bound, a curved edge or a divider: through the one
///   reading of a bound's side ([`crate::bound_side::read_plane_side`]),
///   at first order levered at its reach, and one order down where that
///   ties: a curved edge by which side it bends to, a divider by its
///   face's normal section along it. The far vertex misleads on conics:
///   a belly arc between ON vertices departs into its own side while
///   both endpoints read On. Post-root-insertion the interior is
///   sign-constant (every interior crossing was split out), so the
///   departure IS the interior side. A bound that rides the plane, or
///   whose descent reads nothing past its tie, reads On.
pub(crate) fn bound_side<T: Decide, E>(
    body: &Body<T>,
    bound: &Bound<T>,
    reference: geom_brep::ReferenceNormal<T>,
    far: impl FnOnce(VertexKey) -> Result<PlaneSide, E>,
    band: Band,
) -> Result<PlaneSide, EntryFault<E>> {
    bound_reading(body, bound, reference, far, band).map(|(side, _)| side)
}

/// [`bound_side`], and whether an `On` is a descent that read nothing
/// past its tie ([`Descent::Unresolved`]) rather than one that rides the
/// plane: rule (b) reads such a divider as main's first order left it,
/// by its wall.
fn bound_reading<T: Decide, E>(
    body: &Body<T>,
    bound: &Bound<T>,
    reference: geom_brep::ReferenceNormal<T>,
    far: impl FnOnce(VertexKey) -> Result<PlaneSide, E>,
    band: Band,
) -> Result<(PlaneSide, bool), EntryFault<E>> {
    if let BoundSupport::Chord { .. } = bound.reach {
        return far(body.proven_half_edge_end(bound.he))
            .map(|side| (side, false))
            .map_err(EntryFault::Far);
    }
    match crate::bound_side::read_plane_side(bound.dir, bound.reach, reference, band) {
        Ok(BoundSide::Off(Side::Below) | BoundSide::Tied(Descent::Leaves(Side::Below))) => {
            Ok((PlaneSide::Below, false))
        }
        Ok(BoundSide::Off(Side::Above) | BoundSide::Tied(Descent::Leaves(Side::Above))) => {
            Ok((PlaneSide::Above, false))
        }
        Ok(BoundSide::Tied(Descent::Rides)) => Ok((PlaneSide::On, false)),
        Ok(BoundSide::Tied(Descent::Unresolved(_))) => Ok((PlaneSide::On, true)),
        Err(fault) => Err(EntryFault::Undecided(fault.diag())),
    }
}

/// One ON vertex's classified neighbourhood: its star, each bound's side
/// (after rules (a) and (b)), and whether the plane held it before the
/// rules sent it to one.
pub(super) struct Classified<T: geom_core::Real> {
    pub star: Star<T>,
    pub classes: Vec<PlaneSide>,
    pub in_plane: Vec<bool>,
}

/// One bound of a classified neighbourhood: its orbit half-edge, whether
/// it is a real edge or a divider, and its side after rules (a) and (b).
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct BoundClass {
    /// The orbit half-edge (starts at the base vertex): the bound's own
    /// edge, or the edge a divider's corner follows.
    pub he: HalfEdgeKey,
    /// Whether the bound is a real edge of the orbit, not a divider the
    /// walk cut a sector at.
    pub real: bool,
    /// The bound's side after rules (a) and (b).
    pub class: PlaneSide,
}

/// Builds and fully classifies ON vertex `vertex`'s neighbourhood: its
/// star against the plane, each bound's side ([`bound_side`]), then rule
/// (a) and rule (b) ([`rules`]). Public so tests and the joining step
/// can inspect classification independently of insertion.
///
/// # Errors
///
/// [`SplitReduceError`] — sliver escalations, the consecutive-ON
/// invariant, a `vertex` that does not resolve or is a lone vertex, a
/// far vertex `sides` holds no verdict for, a null edge at `vertex`
/// ([`SplitReduceError::NullEdgeAtVertex`]), or a vertex near a cone's
/// axis ([`SplitReduceError::VertexNearConeAxis`]).
pub fn classify_neighborhood<T: Decide>(
    body: &Body<T>,
    plane: &SplitPlane<T>,
    sides: &SecondaryMap<VertexKey, PlaneSide>,
    vertex: VertexKey,
    band: Band,
) -> Result<Vec<BoundClass>, SplitReduceError> {
    let hood = classify(body, plane, sides, vertex, &SlitTips::of(body), band)?;
    Ok(hood
        .star
        .bounds
        .iter()
        .zip(&hood.classes)
        .map(|(b, &class)| BoundClass {
            he: b.he,
            real: b.real,
            class,
        })
        .collect())
}

/// [`classify_neighborhood`], keeping the star, `tips` the body's slit
/// tips.
pub(super) fn classify<T: Decide>(
    body: &Body<T>,
    plane: &SplitPlane<T>,
    sides: &SecondaryMap<VertexKey, PlaneSide>,
    vertex: VertexKey,
    tips: &SlitTips,
    band: Band,
) -> Result<Classified<T>, SplitReduceError> {
    if body.get_vertex(vertex).is_none() {
        return Err(SplitReduceError::StaleVertex { vertex });
    }
    let star = split_star(body, plane.normal.get(), vertex, tips, band)?;
    let reference = geom_brep::ReferenceNormal::of_split_plane(plane.normal.get());
    let n = star.bounds.len();
    let mut classes = Vec::with_capacity(n);
    let mut unresolved = Vec::with_capacity(n);
    for (k, bound) in star.bounds.iter().enumerate() {
        let far = |far| {
            sides
                .get(far)
                .copied()
                .ok_or(SplitReduceError::UnrecordedSide { vertex: far })
        };
        let (class, tie) =
            bound_reading(body, bound, reference, far, band).map_err(|fault| match fault {
                EntryFault::Far(e) => e,
                EntryFault::Undecided(diag) => SplitReduceError::SliverSector {
                    vertex,
                    face: star.sectors[k].face,
                    reading: SectorReading::Escalated(diag),
                },
            })?;
        classes.push(class);
        unresolved.push(tie);
    }
    let in_plane: Vec<bool> = classes.iter().map(|&c| c == PlaneSide::On).collect();
    let readings = rules::apply_rule_a(body, plane, &star, &mut classes, band)?;
    rules::apply_rule_b(body, &star, (&mut classes, &unresolved), &readings, band)?;
    Ok(Classified {
        star,
        classes,
        in_plane,
    })
}

#[cfg(test)]
#[allow(clippy::unwrap_used, clippy::expect_used, clippy::panic)]
mod tests {
    use super::*;
    use geom_core::{Tol, Vec3};

    /// **A cylinder-graze divider reads its descended side, in the
    /// split** (the pierce's half is
    /// `boolean::sectors::tests::a_cylinder_graze_divider_reads_its_descended_side`):
    /// the oblique rim's straight corner on the wall holds a divider, and a
    /// plane tangent to the wall there reads it one order down, Below
    /// against the wall's outward normal and Above against its reverse,
    /// not `On` by convention.
    #[test]
    fn a_cylinder_graze_divider_reads_its_descended_side() {
        let band = Band::linear(Tol::witness()).unwrap();
        let (body, wall, vertex) =
            crate::test_support_fixtures::oblique_rim_wall_from::<f64>(0.3, 1.0);
        let star = Star::walk(&body, vertex, &SlitTips::default(), band).unwrap();
        let k = (0..star.bounds.len())
            .find(|&k| !star.bounds[k].real && star.sectors[k].face == wall)
            .expect("the wall's straight corner holds a divider");
        let crate::star::StarFace::Tangent(normal) = star.sectors[k].at else {
            panic!("the wall has a tangent plane at the rim")
        };
        for (n, want) in [
            (normal.vec(), PlaneSide::Below),
            (-normal.vec(), PlaneSide::Above),
        ] {
            let side = bound_side(
                &body,
                &star.bounds[k],
                geom_brep::ReferenceNormal::of_split_plane(n),
                |_| Err::<PlaneSide, ()>(()),
                band,
            )
            .unwrap();
            assert_eq!(side, want, "n = {n:?}: descended");
        }
    }

    /// **A vertex near a cone's axis refuses, typed**: on a needle cone,
    /// a vertex a band's width off the axis and well off the apex has no
    /// tangent plane the split reads, and the star's fault is the split's
    /// [`SplitReduceError::VertexNearConeAxis`], naming the vertex, the
    /// face and the band.
    #[test]
    fn a_vertex_near_a_cones_axis_refuses_typed() {
        use crate::construction::{FaceSurface, MevSite};
        use geom_core::Point3;
        let alpha: f64 = 0.01;
        let at = |z: f64| Point3::new(z * alpha.tan(), 0.0, z);
        let mut body = Body::<f64>::new();
        let seed = body.mvfs(at(1e-5), true).unwrap();
        body.set_face_surface(
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
        .unwrap();
        body.mev_line(
            MevSite::Lone {
                r#loop: seed.r#loop,
            },
            at(1.0),
            Tol::witness(),
        )
        .unwrap();
        let band = Band::new(1e-6, 1e-5).unwrap();
        let normal = Vec3::new(0.0, 1.0, 0.0);
        match split_star(&body, normal, seed.vertex, &SlitTips::of(&body), band) {
            Err(SplitReduceError::VertexNearConeAxis {
                vertex,
                face,
                band: b,
            }) => {
                assert_eq!((vertex, face), (seed.vertex, seed.face));
                assert_eq!(b, band);
            }
            other => panic!("expected the typed near-axis refusal, got {other:?}"),
        }
    }

    /// The split lane has no sphere arm and says so BY NAME, on the
    /// shared walk's report rather than by re-matching the surface —
    /// an arm the shared walk created and nothing covered until this
    /// row.
    #[test]
    fn a_sphere_carried_sector_refuses_by_name() {
        let mut body = crate::test_support_fixtures::geometric_cube::<f64>(Tol::witness()).body;
        let vertex = body
            .vertices()
            .map(|(k, _)| k)
            .find(|&v| {
                body.resolve_vertex_point(v, crate::live::Proven)
                    .equals_exactly(geom_core::Point3::origin())
            })
            .unwrap();
        let orbit_he = body.vertex_orbit_linked(vertex)[0];
        let face = crate::sector_face::face_of(&body, vertex, orbit_he);
        body.set_face_surface(
            face,
            crate::construction::FaceSurface::New {
                surface: geom::Surface::Sphere {
                    center: geom_core::Point3::new(2.0, 2.0, 2.0),
                    radius: 12f64.sqrt(),
                    axis: Vec3::new(0.0, 0.0, 1.0),
                    u_ref: Vec3::new(1.0, 0.0, 0.0),
                },
                sense: true,
            },
        )
        .unwrap();
        let band = Band::linear(Tol::witness()).unwrap();
        let normal = Vec3::new(0.0, 0.0, 1.0);
        match split_star(&body, normal, vertex, &SlitTips::of(&body), band) {
            Err(SplitReduceError::CurvedBooleanUnsupported { face: f, kind }) => {
                assert_eq!(f, face);
                assert_eq!(kind, geom::SurfaceKind::Sphere);
            }
            other => panic!("expected the typed sphere refusal, got {other:?}"),
        }
    }
}

/// **A spline leaves a vertex along its tangent, not its chord**: the
/// boolean sweep rows' Bézier sheet, whose arc leaves `(0, 0, 0)` along
/// `(1, 2, 0)` while its chord runs along `x̂`
/// (`boolean::sectors::spline_departure_rows`, the boolean walk's twin).
#[cfg(test)]
#[allow(clippy::unwrap_used, clippy::expect_used, clippy::panic)]
mod spline_departure_rows {
    use crate::boolean::reduce::planar_lane_carrier_rows::arc_sheet;
    use geom_core::Vec3;

    #[test]
    fn the_split_walk_reads_a_splines_tangent() {
        let (body, edge) = arc_sheet(1.0, 1.0);
        let he = body.get_edge(edge).unwrap().he_plus;
        let vertex = body.get_half_edge(he).unwrap().start;
        let (dir, reach) = crate::star::departure(&body, vertex, he).unwrap();
        let tangent = Vec3::new(1.0, 2.0, 0.0).normalize();
        assert!(
            (dir.normalize() - tangent).norm() < 1e-12,
            "the arc leaves along its tangent (1, 2, 0)/√5, got {dir:?}"
        );
        assert!(
            (dir.norm() - 2.0).abs() < 1e-12,
            "levered at its extent, the chord's length 2: {dir:?}"
        );
        assert!(
            matches!(reach, crate::bound_side::BoundSupport::Conic { .. }),
            "a curved edge carries its bend: {reach:?}"
        );
    }
}
