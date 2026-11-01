//! The vertex-neighborhood **sector face**: which face a sector belongs
//! to, and that face's outward normal at the base vertex — one
//! implementation, shared by the two lanes that ask this question.
//!
//! # What the question is
//!
//! Both the splitting lane (`splitting::neighborhood`) and the boolean
//! lane ([`crate::boolean::sectors`]) walk a vertex orbit and, for the
//! sector CW-after orbit half-edge `he`, must resolve two things before
//! any predicate can run:
//!
//! 1. **Whose corner is this?** The sector after `he` is the corner of
//!    `face(loop(mate(he)))` — the traversal is the same on both sides,
//!    and its derivation lives in `splitting::neighborhood`'s module
//!    docs, which own it.
//! 2. **Which way is out, HERE?** Sector predicates meter against the
//!    face's outward normal **at the base vertex**, which for a charted
//!    face is a local quantity: a plane hands back its stored normal, a
//!    cylinder its chart-outward radial at the vertex point, a sphere
//!    the radial from its centre, a torus the offset from the vertex's
//!    foot on its core circle, a cone its chart normal off the
//!    generator through the vertex, a spline face its chart's
//!    `S_u × S_v` at the vertex's chart parameters. A cone has no
//!    tangent plane at its apex, and is read there as its circle of
//!    rulings ([`VertexFace::Apex`]), each with its own normal: a
//!    reader takes the normal along the ruling it means
//!    ([`VertexFace::normal_along`]), and none is NaN.
//!
//! **Every arm folds in the face's `sense` bit** (S10) by minting an
//! [`OutwardNormal`]: each reads a CHART normal and returns it as the
//! face's OUTWARD normal, and the chart is the only encoding of
//! orientation the face has. This function is therefore the chokepoint
//! for both vertex-neighborhood lanes: everything downstream —
//! `splitting::rules::apply_rule_a`'s `enters_material` call, the
//! boolean's `within` / `side_code` / `sector_overlap` / `germ_dir` /
//! `pierce_germ_dir`, and the sector-shape rungs on both sides — is
//! sense-invariant GIVEN this value and must NOT multiply again. Those
//! sites pair the normal with the STORED orbit order, which `revert`
//! reverses in the same breath as the sense bit, so a second
//! sense fold would cancel this one.
//!
//! # Why the code is here and not in either lane
//!
//! This walk used to exist **twice**, once per lane, and the splitting
//! copy's own doc called itself *"the twin of"* the
//! boolean's. They differed in exactly three things, none of them
//! geometric: the error type they refuse into, whether the caller is
//! told the face was planar, and whether a `Sphere` face has a wired
//! arm. The first two are adaptation the callers do; the third is
//! reported here as a [`SectorCarrier`] and refused by the caller that
//! has no arm for it. What is left — the traversal and every arm's
//! normal — is one body.
//!
//! Like [`crate::sector_shape`], this module is a **top-level sibling**
//! of `boolean/` and `splitting/` precisely so neither half hosts the
//! other's core; both lanes already depend on this scope
//! (`crate::body`, `crate::entity`), so sharing here adds no dependency
//! edge between the halves and no public API.
//!
//! `sector_shape`'s docs name this unification as the trigger to
//! re-open whether that module belongs in `geom-brep` instead. **It
//! fires and resolves the other way**: this body takes a [`Body`] and
//! three arena keys, so `geom-brep` — which has no `Body` and no arena
//! — cannot host it at any price. The crate root is the deepest scope
//! that can, and the two shared sector modules belong together.

use geom::SurfaceKind;
use geom_brep::OutwardNormal;
use geom_core::{Band, Decide, Point3, Vec3};

use crate::face_normal::ConePoint;

use crate::body::Body;
use crate::entity::{EntityId, FaceKey, HalfEdgeKey, VertexKey};
use crate::face_normal::face_outward_normal;

/// Which charted carrier a sector's face sits on — the arms with a
/// wired outward-normal-at-the-vertex construction.
///
/// This is a **capability** report, not a surface taxonomy: the kinds
/// with no arm never reach it (they refuse as
/// [`SectorFaceError::Unsupported`]), and a caller whose own pipeline
/// does not execute one of these arms refuses on it by name.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub(crate) enum SectorCarrier {
    /// A plane: the normal is the stored chart normal.
    Plane,
    /// A cylinder: the normal is the chart-outward radial at the vertex.
    Cylinder,
    /// A sphere: the normal is the radial from the centre.
    Sphere,
    /// A ring torus: the normal is the point minus its foot on the
    /// core circle.
    Torus,
    /// A cone: the normal is the chart-outward one off the generator
    /// through the vertex, on that vertex's nappe.
    Cone,
    /// A spline chart (a `Nurbs` or an `Approx` face, the kind named):
    /// the normal is `S_u × S_v` at the vertex's chart parameters, read
    /// off the face's own pcurve of the edge into the vertex.
    Spline(SurfaceKind),
}

/// The resolved sector face.
#[derive(Debug)]
pub(crate) struct SectorFace<T: geom_core::Real> {
    /// The face the sector is a corner of.
    pub face: FaceKey,
    /// What the face is at the base vertex: a tangent plane, or a cone's
    /// apex.
    pub at_vertex: VertexFace<T>,
    /// Which arm read it.
    pub carrier: SectorCarrier,
}

/// **What a face is at a vertex.** Every arm but one has a tangent plane
/// there. A cone has none at its apex, where its directions are the
/// circle of its rulings, each with a normal of its own: a reader that
/// asks for one normal at the vertex has to say which ruling it means.
#[derive(Clone, Copy, Debug)]
pub(crate) enum VertexFace<T: geom_core::Real> {
    /// The face's OUTWARD unit normal at the vertex, minted here from the
    /// chart normal and the face's `sense` bit — the type carries the
    /// obligation not to re-apply the sense downstream.
    Tangent(OutwardNormal<T>),
    /// A cone face's tangent plane at a vertex within the band of the
    /// cone's axis but definitely off its apex
    /// ([`ConePoint::NearAxis`]): read, ill-conditioned, for a lane that
    /// reads it (the split, as it did before the apex was read); the
    /// Boolean refuses it under its own name.
    NearAxis(OutwardNormal<T>),
    /// The vertex is the apex of the face's cone.
    Apex(ConeApex<T>),
}

impl<T: Decide> VertexFace<T> {
    /// The outward normal approached along `d`, a direction the face
    /// leaves the vertex in: the tangent plane's, or at an apex the
    /// normal along the ruling `d` lies on, which is constant along it
    /// ([`ConeApex::normal_along`]).
    pub(crate) fn normal_along(&self, d: Vec3<T>) -> OutwardNormal<T> {
        match self {
            Self::Tangent(normal) | Self::NearAxis(normal) => *normal,
            Self::Apex(apex) => apex.normal_along(d),
        }
    }

    /// The tangent plane's outward normal; a row that read an apex
    /// panics naming it.
    #[cfg(test)]
    #[allow(clippy::panic)]
    pub(crate) fn tangent(&self) -> Vec3<T> {
        match self {
            Self::Tangent(normal) | Self::NearAxis(normal) => normal.vec(),
            Self::Apex(apex) => panic!("a tangent plane, not {apex:?}"),
        }
    }
}

/// **A cone at its apex**: the face's cone, read as the circle of rulings
/// the face leaves its apex along, and the face's `sense` bit.
#[derive(Clone, Copy, Debug)]
pub(crate) struct ConeApex<T: geom_core::Real> {
    /// The apex.
    pub apex: Point3<T>,
    /// The cone's unit axis.
    pub axis: Vec3<T>,
    /// The half-angle between a ruling and the axis.
    pub half_angle: T,
    /// The cone's seam direction, which places azimuth zero.
    pub u_ref: Vec3<T>,
    /// The face's `sense` bit.
    pub sense: bool,
}

impl<T: Decide> ConeApex<T> {
    /// The cone as a surface.
    pub(crate) fn surface(&self) -> geom::Surface<T> {
        geom::Surface::Cone {
            apex: self.apex,
            axis: self.axis,
            half_angle: self.half_angle,
            u_ref: self.u_ref,
        }
    }

    /// The outward normal along the ruling `d` leaves the apex by: the
    /// chart normal at the ruling's point a unit from the apex, the
    /// face's sense folded once. A cone's normal is constant along a
    /// ruling, so this is the normal at every point of it, and the limit
    /// of the face's normal at points approaching the apex along it.
    pub(crate) fn normal_along(&self, d: Vec3<T>) -> OutwardNormal<T> {
        let chart = geom_brep::implicit_gradient(&self.surface(), self.apex + d.normalize());
        OutwardNormal::from_chart(chart.normalize(), self.sense)
    }

    /// The unit ruling along the line through the apex along `d`, on the
    /// nappe `like` (a ruling the face leaves the apex along) lies on: the
    /// line holds one ruling of each nappe, opposite in azimuth.
    pub(crate) fn onto_nappe_of(&self, d: Vec3<T>, like: Vec3<T>) -> Vec3<T> {
        d.normalize() * T::one().copysign(d.dot(self.axis) * like.dot(self.axis))
    }

    /// Which way round the axis the face's interior leaves the ruling
    /// along `d` (`n × d̂`, `n` the outward normal along it: the orbit's
    /// interior-left convention), as `±1` against `axis × d̂`. Constant
    /// over the face's rulings.
    pub(crate) fn sweep(&self, d: Vec3<T>) -> T {
        let d = d.normalize();
        T::one().copysign(self.normal_along(d).vec().cross(d).dot(self.axis.cross(d)))
    }

    /// How far the face's sweep turns from the ruling along `from` to the
    /// one along `to`, as the chord between their points a unit out from
    /// the apex, signed by [`ConeApex::sweep`]: positive where `to` lies
    /// less than a half-turn into the sweep.
    pub(crate) fn turn(&self, from: Vec3<T>, to: Vec3<T>) -> T {
        let (f, t) = (from.normalize(), to.normalize());
        (t - f)
            .norm()
            .copysign(f.cross(t).dot(self.axis) * self.sweep(f))
    }

    /// The azimuth the face's sweep turns through from the ruling along
    /// `from` to the one along `d`, in `(0, 2π]`: zero along `from`,
    /// increasing the way the face's interior leaves it
    /// ([`ConeApex::sweep`]).
    pub(crate) fn azimuth_from(&self, from: Vec3<T>, d: Vec3<T>) -> T {
        let (e1, e2) = self.sweep_frame(from);
        let a = d.dot(e2).atan2(d.dot(e1));
        a.select_le_zero(a + T::tau(), a)
    }

    /// The unit ruling the face's sweep reaches from the one along `from`
    /// after turning through `azimuth` ([`ConeApex::azimuth_from`]), on
    /// `from`'s nappe.
    pub(crate) fn ruling_from(&self, from: Vec3<T>, azimuth: T) -> Vec3<T> {
        let (e1, e2) = self.sweep_frame(from);
        let along = self.axis * self.half_angle.cos().copysign(from.dot(self.axis));
        along + (e1 * azimuth.cos() + e2 * azimuth.sin()) * self.half_angle.sin()
    }

    /// The sweep's frame square to the axis at the ruling along `from`:
    /// `e1` along its azimuth, `e2` the way the face's interior leaves it.
    fn sweep_frame(&self, from: Vec3<T>) -> (Vec3<T>, Vec3<T>) {
        let from = from.normalize();
        let e1 = (from - self.axis * from.dot(self.axis)).normalize();
        (e1, self.axis.cross(e1) * self.sweep(from))
    }

    /// Whether the face's material lies inside its cone: the outward
    /// normal points off the axis on both nappes exactly where the
    /// face's sense keeps the chart normal. Inside, the cone bends into
    /// its material along every ruling.
    pub(crate) const fn material_inside(&self) -> bool {
        self.sense
    }
}

/// Why a sector face could not be resolved. Each lane maps this onto
/// its own error type — the boolean's twins additionally carry the
/// `Operand`, which is why the adaptation stays with the callers.
#[derive(Clone, Copy, Debug)]
pub(crate) enum SectorFaceError {
    /// The face's surface has no wired sector arm.
    Unsupported {
        /// The face whose carrier has no arm.
        face: FaceKey,
        /// The kind that has none.
        kind: SurfaceKind,
    },
    /// A spline face's chart has no one normal at the vertex: the vertex
    /// lies within the band of an irregular point of the chart (a pole,
    /// coincident control rows) or of a line its normal is not continuous
    /// across (an interior knot run of multiplicity at least the degree,
    /// a closed chart's seam), where a normal read off one side of the
    /// line is not the other's ([`geom_brep::chart_normal_at`]).
    Crease {
        /// The spline face.
        face: FaceKey,
        /// Its kind, `Nurbs` or `Approx`.
        kind: SurfaceKind,
    },
    /// A spline face's half-edge into the vertex has no chart image to
    /// read the vertex's chart parameters off: it stores no pcurve, or
    /// its edge no certified carrier.
    Unplaced {
        /// The spline face.
        face: FaceKey,
        /// Its kind, `Nurbs` or `Approx`.
        kind: SurfaceKind,
    },
    /// A cone face's vertex lies in the band of the cone's axis
    /// ([`crate::face_normal::cone_point`] escalated): neither a
    /// tangent plane nor the apex is decided.
    ApexUndecided {
        /// The cone face.
        face: FaceKey,
        /// The escalation.
        diag: geom_core::Indeterminate,
    },
}

/// The face orbit half-edge `he`'s sector at `vertex` is a corner of:
/// the face of its mate's loop. `vertex` and `he` are as for
/// [`resolve`].
#[track_caller]
pub(crate) fn face_of<T: Decide>(body: &Body<T>, vertex: VertexKey, he: HalfEdgeKey) -> FaceKey {
    let mate = body.proven_mate(he, crate::live::link(EntityId::Vertex(vertex), "orbit"));
    body.face_of_linked(mate.mate)
}

/// Resolves the sector CW-after `he` (`face(loop(mate(he)))`) to its
/// face, what that face is at `vertex` (its outward normal there, or a
/// cone's apex, decided by [`crate::face_normal::cone_point`]), and
/// the carrier arm that read it.
///
/// Named `resolve` rather than `sector_face`: the module is the noun,
/// and each lane's thin wrapper keeps that name for its own callers,
/// so `sector_face` in prose means one thing per scope instead of
/// three.
///
/// `vertex` and `he` are the caller's orbit walk's — a vertex it
/// resolved and a member of its orbit — so every hop from them is a
/// link of a record, and a miss panics naming it (D2 row 4).
///
/// `band` decides whether a spline face's vertex lies on a crease of
/// its chart.
///
/// # Errors
///
/// [`SectorFaceError`] — a carrier with no arm, a spline face with no
/// one normal at the vertex or no chart image to read it at, or a cone
/// face whose vertex lies in the band of its axis.
#[track_caller]
pub(crate) fn resolve<T: Decide>(
    body: &Body<T>,
    vertex: VertexKey,
    he: HalfEdgeKey,
    band: Band,
) -> Result<SectorFace<T>, SectorFaceError> {
    let into = body
        .proven_mate(he, crate::live::link(EntityId::Vertex(vertex), "orbit"))
        .mate;
    let face = body.face_of_linked(into);
    // The planar arm goes through the crate's ONE sense-flip door
    // ([`crate::face_normal`]) rather than re-deriving the flip, and
    // reads nothing else: a plane's outward normal is a property of
    // the face alone.
    if let Some(normal) = face_outward_normal(body, face) {
        return Ok(SectorFace {
            face,
            at_vertex: VertexFace::Tangent(normal),
            carrier: SectorCarrier::Plane,
        });
    }
    // The charted arms need the face's own chart and the base vertex.
    let face_data = crate::live::proven(&body.faces, face, EntityId::Face);
    let sense = face_data.sense;
    let point = || {
        body.point_of(
            vertex,
            crate::live::proven(&body.vertices, vertex, EntityId::Vertex),
        )
    };
    let charted = |chart, carrier| {
        Ok(SectorFace {
            face,
            at_vertex: VertexFace::Tangent(OutwardNormal::from_chart(chart, sense)),
            carrier,
        })
    };
    let surface = body.face_surface_linked(face, face_data);
    if let Some(chart) = surface.spline_chart() {
        let kind = surface.kind();
        let normal = spline_normal(
            body,
            point(),
            (he, into),
            (face, sense),
            (chart, kind),
            band,
        )?;
        return Ok(SectorFace {
            face,
            at_vertex: VertexFace::Tangent(normal),
            carrier: SectorCarrier::Spline(kind),
        });
    }
    match surface {
        geom::Surface::Cylinder { origin, axis, .. } => {
            let w = point() - *origin;
            charted(
                (w - *axis * w.dot(*axis)).normalize(),
                SectorCarrier::Cylinder,
            )
        }
        geom::Surface::Sphere { center, .. } => {
            charted((point() - *center).normalize(), SectorCarrier::Sphere)
        }
        // The chart normal of a ring torus at `p` is `p` minus its foot
        // on the core circle, over `r` — the implicit gradient, read from
        // its one home rather than re-derived here. It is `0/0` only on
        // the axis, which no point of a ring torus's tube reaches
        // (`ρ ≥ R − r > 0`); a vertex there poisons the normal, and
        // poison escalates typed at the first decide that reads it.
        s @ geom::Surface::Torus { .. } => charted(
            geom_brep::implicit_gradient(s, point()).normalize(),
            SectorCarrier::Torus,
        ),
        // A cone's implicit gradient IS its chart normal, nappe sign
        // included, everywhere off its axis. On the axis it is `0/0`, and
        // the axis point of the surface is the apex, read as its rulings.
        s @ &geom::Surface::Cone {
            apex,
            axis,
            half_angle,
            u_ref,
        } => {
            let p = point();
            match crate::face_normal::cone_point(apex, axis, p, band) {
                Ok(ConePoint::OffAxis) => charted(
                    geom_brep::implicit_gradient(s, p).normalize(),
                    SectorCarrier::Cone,
                ),
                Ok(ConePoint::NearAxis) => Ok(SectorFace {
                    face,
                    at_vertex: VertexFace::NearAxis(OutwardNormal::from_chart(
                        geom_brep::implicit_gradient(s, p).normalize(),
                        sense,
                    )),
                    carrier: SectorCarrier::Cone,
                }),
                Ok(ConePoint::Apex) => Ok(SectorFace {
                    face,
                    at_vertex: VertexFace::Apex(ConeApex {
                        apex,
                        axis: axis.normalize(),
                        half_angle,
                        u_ref,
                        sense,
                    }),
                    carrier: SectorCarrier::Cone,
                }),
                Err(diag) => Err(SectorFaceError::ApexUndecided { face, diag }),
            }
        }
        // The planar and spline arms returned above; anything else has
        // no arm.
        s => Err(SectorFaceError::Unsupported {
            face,
            kind: s.kind(),
        }),
    }
}

/// A spline face's outward normal at the vertex `at`, which ends the
/// face's half-edge `into`, the mate of the orbit half-edge `he`.
///
/// The vertex's chart parameters are that half-edge's pcurve (C4) at the
/// edge's parameter of the vertex, not a projection foot: the pcurve is
/// the edge's image in this face's own chart, so the parameters are on
/// the face's sheet by construction, where a foot of the point could
/// land on another sheet of the carrier that passes within the band.
/// The normal is read only where the chart has one there
/// ([`geom_brep::regular_jet`], [`geom_brep::chart_normal_at`]), over the
/// carrier's farthest reach from the vertex, its control net's (the face
/// is a trim of the carrier, so it reaches no farther).
fn spline_normal<T: Decide>(
    body: &Body<T>,
    at: geom_core::Point3<T>,
    (he, into): (HalfEdgeKey, HalfEdgeKey),
    (face, sense): (FaceKey, bool),
    (chart, kind): (&geom::NurbsSurface<T>, SurfaceKind),
    band: Band,
) -> Result<OutwardNormal<T>, SectorFaceError> {
    if chart.is_placeholder() {
        return Err(SectorFaceError::Unsupported { face, kind });
    }
    let unplaced = SectorFaceError::Unplaced { face, kind };
    let he_data = crate::live::proven(&body.half_edges, he, EntityId::HalfEdge);
    let edge = crate::live::linked(
        &body.edges,
        he_data.edge,
        EntityId::Edge,
        EntityId::HalfEdge(he),
        "edge",
    );
    let curve = body
        .edge_curve_linked(he_data.edge, edge)
        .certified()
        .ok_or(unplaced)?;
    // `he` leaves the vertex, so the vertex is where it starts: the
    // carrier's first end where `he` walks it forward.
    let (t0, t1) = curve.params();
    let t = if he == edge.he_plus { t0 } else { t1 };
    let uv = body.pcurve(into).ok_or(unplaced)?.pcurve().eval(t);
    let uv = (uv.x, uv.y);
    let reach = geom_brep::control_net_reach(chart.control(), at);
    let crease = |_| SectorFaceError::Crease { face, kind };
    let jet = geom_brep::regular_jet(chart, uv, reach, band).map_err(crease)?;
    geom_brep::chart_normal_at(chart, sense, &jet, uv, reach, band)
        .map(|(normal, _)| normal)
        .map_err(crease)
}

#[cfg(test)]
#[allow(clippy::unwrap_used, clippy::expect_used, clippy::panic)]
pub(crate) mod tests {
    use super::*;
    use crate::fixtures::raw_prism;
    use geom_core::Tol;

    fn band() -> Band {
        Band::linear(Tol::witness()).unwrap()
    }

    /// A sphere face, installed on a prism's side face so the orbit
    /// around one of its vertices walks a sphere-carried sector.
    fn sphere_sided_prism() -> (crate::Body<f64>, crate::entity::FaceKey) {
        let p = raw_prism(3, Tol::witness());
        let face = p.face_side[0];
        let mut body = p.body;
        body.set_face_surface(
            face,
            crate::construction::FaceSurface::New {
                surface: geom::Surface::Sphere {
                    center: geom_core::Point3::new(0.0, 0.0, 0.0),
                    radius: 2.0,
                    axis: geom_core::Vec3::new(0.0, 0.0, 1.0),
                    u_ref: geom_core::Vec3::new(1.0, 0.0, 0.0),
                },
                sense: true,
            },
        )
        .unwrap();
        (body, face)
    }

    /// A torn link panics naming the record that holds it: the orbit
    /// member's `parent_loop`, not the base vertex the lanes' public
    /// arms are limited to.
    #[test]
    fn a_torn_link_panics_naming_its_record() {
        let p = raw_prism(3, Tol::witness());
        let mut body = p.body;
        let (orbit_he, _) = body.half_edges().next().expect("a half-edge");
        let vertex = body.get_half_edge(orbit_he).unwrap().start;
        let mate = body.mate(orbit_he).unwrap();
        body.get_half_edge_mut(mate).unwrap().parent_loop = crate::entity::LoopKey::default();
        let report = crate::surgery::tests::panic_message(std::panic::AssertUnwindSafe(|| {
            let _ = resolve(&body, vertex, orbit_he, band());
        }));
        let premise = format!(
            "{}'s parent_loop names {}, which does not resolve",
            EntityId::HalfEdge(mate),
            EntityId::Loop(crate::entity::LoopKey::default())
        );
        assert!(
            report.contains(&premise) && report.contains(crate::live::NAMES_ONLY_LIVE),
            "{report}"
        );
    }

    /// The charted arms read the base vertex, which the caller's walk
    /// resolved, so a vertex that does not resolve panics.
    #[test]
    fn a_charted_arm_panics_on_a_vertex_that_does_not_resolve() {
        let (body, face) = sphere_sided_prism();
        let he = body
            .get_loop(body.get_face(face).unwrap().outer)
            .and_then(|l| match l.boundary {
                crate::entity::LoopBoundary::Cycle { first } => Some(first),
                crate::entity::LoopBoundary::Empty { .. } => None,
            })
            .unwrap();
        // The sector CW-after `mate(he)` is this face's corner.
        let orbit_he = body.mate(he).unwrap();
        let bogus = crate::entity::VertexKey::default();
        let report = crate::surgery::tests::panic_message(std::panic::AssertUnwindSafe(|| {
            let _ = resolve(&body, bogus, orbit_he, band());
        }));
        assert!(
            report.contains(&format!(
                "{}, which this call resolved or read out of a record, does not resolve",
                EntityId::Vertex(bogus)
            )),
            "{report}"
        );
    }

    /// The sphere arm is WIRED here (the boolean lane executes it) and
    /// reports its carrier, so the lane without a sphere arm can refuse
    /// by name rather than by kind-matching a surface itself.
    #[test]
    fn the_sphere_arm_is_wired_and_names_its_carrier() {
        let (body, face) = sphere_sided_prism();
        let he = body
            .get_loop(body.get_face(face).unwrap().outer)
            .and_then(|l| match l.boundary {
                crate::entity::LoopBoundary::Cycle { first } => Some(first),
                crate::entity::LoopBoundary::Empty { .. } => None,
            })
            .unwrap();
        let orbit_he = body.mate(he).unwrap();
        let vertex = body.get_half_edge(orbit_he).unwrap().start;
        let resolved = resolve(&body, vertex, orbit_he, band()).expect("the sphere arm resolves");
        assert_eq!(resolved.carrier, SectorCarrier::Sphere);
        assert_eq!(resolved.face, face);
    }

    /// **A cone face reads its apex only at its apex.** A 2° needle cone
    /// is installed on a prism side face with one of the face's vertices
    /// on a generator: at the apex the vertex reads the apex's rulings;
    /// a hundred band widths along the generator, within the band of
    /// the axis but definitely off the apex, it reads its tangent plane
    /// (marked near the axis, which the Boolean refuses); a generator's
    /// length out it reads its tangent plane plainly.
    #[test]
    fn a_cone_reads_its_apex_only_at_the_apex() {
        let alpha = 2f64.to_radians();
        let g = Vec3::new(alpha.sin(), 0.0, alpha.cos());
        let eps = band().zero();
        for (along, want) in [(0.0, "apex"), (100.0 * eps, "near"), (1.0, "off")] {
            let p = raw_prism(3, Tol::witness());
            let face = p.face_side[0];
            let mut body = p.body;
            let he = body
                .get_loop(body.get_face(face).unwrap().outer)
                .and_then(|l| match l.boundary {
                    crate::entity::LoopBoundary::Cycle { first } => Some(first),
                    crate::entity::LoopBoundary::Empty { .. } => None,
                })
                .unwrap();
            let orbit_he = body.mate(he).unwrap();
            let vertex = body.get_half_edge(orbit_he).unwrap().start;
            let at = *body
                .get_point(body.get_vertex(vertex).unwrap().point)
                .unwrap();
            body.set_face_surface(
                face,
                crate::construction::FaceSurface::New {
                    surface: geom::Surface::Cone {
                        apex: at - g * along,
                        axis: Vec3::new(0.0, 0.0, 1.0),
                        half_angle: alpha,
                        u_ref: Vec3::new(1.0, 0.0, 0.0),
                    },
                    sense: true,
                },
            )
            .unwrap();
            let got = resolve(&body, vertex, orbit_he, band()).expect("the cone arm resolves");
            let read = match got.at_vertex {
                VertexFace::Apex(_) => "apex",
                VertexFace::NearAxis(_) => "near",
                VertexFace::Tangent(_) => "off",
            };
            assert_eq!(read, want, "{along} along the generator");
        }
    }

    /// **The torus arm answers the tube's outward normal at the base
    /// vertex.** A torus is installed on a prism side face so that one
    /// of the face's vertices sits on the tube at a general minor angle
    /// `v`; the chart normal there is `cos v·x̂ + sin v·ẑ` (the offset
    /// from the vertex's foot on the core circle, over `r`). The pose
    /// separates it from both wrong constructions a torus arm could
    /// fall into — the axis radial (`x̂`, a cylinder's) and the offset
    /// from the centre (a sphere's) — and the face's sense bit is
    /// folded exactly once.
    #[test]
    fn the_torus_arm_is_the_tube_normal_at_the_base_vertex() {
        let p = raw_prism(3, Tol::witness());
        let face = p.face_side[0];
        let mut body = p.body;
        let he = body
            .get_loop(body.get_face(face).unwrap().outer)
            .and_then(|l| match l.boundary {
                crate::entity::LoopBoundary::Cycle { first } => Some(first),
                crate::entity::LoopBoundary::Empty { .. } => None,
            })
            .unwrap();
        let orbit_he = body.mate(he).unwrap();
        let vertex = body.get_half_edge(orbit_he).unwrap().start;
        let at = *body
            .get_point(body.get_vertex(vertex).unwrap().point)
            .unwrap();
        let (big_r, r, v) = (2.0, 0.5, 2.3_f64);
        let (sv, cv) = v.sin_cos();
        let x = geom_core::Vec3::new(1.0, 0.0, 0.0);
        let z = geom_core::Vec3::new(0.0, 0.0, 1.0);
        let center = at - x * (big_r + r * cv) - z * (r * sv);
        body.set_face_surface(
            face,
            crate::construction::FaceSurface::New {
                surface: geom::Surface::Torus {
                    center,
                    axis: z,
                    major_radius: big_r,
                    minor_radius: r,
                    u_ref: x,
                },
                sense: true,
            },
        )
        .unwrap();
        let resolved = resolve(&body, vertex, orbit_he, band()).expect("the torus arm resolves");
        assert_eq!(resolved.carrier, SectorCarrier::Torus);
        assert_eq!(resolved.face, face);
        let tube = x * cv + z * sv;
        let want = OutwardNormal::from_chart(tube, body.get_face(face).unwrap().sense).vec();
        assert!(
            (resolved.at_vertex.tangent() - want).norm() < 1e-12,
            "{:?} is not the tube's outward normal {want:?}",
            resolved.at_vertex.tangent()
        );
    }

    /// The bilinear-in-`v` chart over the unit square at `z = 1` whose
    /// `u` columns stand at `xs` on the knots `ku`, `u` running along `x`
    /// and `v` along `y`: the plane `z = 1`, chart normal `+z`.
    pub(crate) fn planar_chart(
        ku: Vec<f64>,
        degree_u: usize,
        xs: &[f64],
    ) -> geom::NurbsSurface<f64> {
        use geom_core::spline::KnotVector;
        let ku = KnotVector::clamped(ku, degree_u).unwrap();
        let kv = KnotVector::clamped(vec![0.0, 0.0, 1.0, 1.0], 1).unwrap();
        let control: Vec<_> = xs
            .iter()
            .flat_map(|&x| [0.0, 1.0].map(|y| geom_core::Point3::new(x, y, 1.0)))
            .collect();
        let n = control.len();
        geom::NurbsSurface::new(ku, kv, control, vec![1.0; n]).unwrap()
    }

    /// The unit cube with its top face recarried on `chart` at `sense`,
    /// its pcurves minted, and the top face.
    fn spline_topped_cube(chart: geom::NurbsSurface<f64>, sense: bool) -> (Body<f64>, FaceKey) {
        let cube = crate::test_support_fixtures::prism::<f64>(
            &[(0.0, 0.0), (1.0, 0.0), (1.0, 1.0), (0.0, 1.0)],
            1.0,
            Tol::witness(),
        );
        let mut body = cube.body;
        body.set_face_surface_unvouched_for_tests(
            cube.top_face,
            crate::construction::FaceSurface::New {
                surface: geom::Surface::Nurbs(std::sync::Arc::new(chart)),
                sense,
            },
        )
        .unwrap();
        crate::pcurves::mint_pcurves(&mut body, Tol::witness()).unwrap();
        (body, cube.top_face)
    }

    /// [`spline_topped_cube`] with the top face's `y = 0` edge split at
    /// `x = ½`: the body, the top face and the new vertex.
    pub(crate) fn split_at_half(chart: geom::NurbsSurface<f64>) -> (Body<f64>, FaceKey, VertexKey) {
        let (mut body, top) = spline_topped_cube(chart, true);
        let edge = body
            .edges()
            .map(|(e, _)| e)
            .find(|&e| {
                let c = body.stored_curve(e).unwrap();
                let (t0, t1) = c.params();
                let (a, b) = (c.carrier().eval(t0), c.carrier().eval(t1));
                a.z == 1.0 && b.z == 1.0 && a.y == 0.0 && b.y == 0.0
            })
            .expect("the top face's y = 0 edge");
        let (t0, t1) = body.stored_curve(edge).unwrap().params();
        let mid = body
            .split_edge(edge, 0.5 * (t0 + t1), Tol::witness())
            .unwrap()
            .vertex;
        (body, top, mid)
    }

    /// The orbit half-edge at `vertex` whose sector is a corner of `face`.
    pub(crate) fn sector_of(body: &Body<f64>, vertex: VertexKey, face: FaceKey) -> HalfEdgeKey {
        body.vertex_orbit_linked(vertex)
            .into_iter()
            .find(|&he| face_of(body, vertex, he) == face)
            .expect("the vertex is a corner of the face")
    }

    /// The vertices of `face`'s outer loop.
    fn corners(body: &Body<f64>, face: FaceKey) -> Vec<VertexKey> {
        let crate::entity::LoopBoundary::Cycle { first } = body
            .get_loop(body.get_face(face).unwrap().outer)
            .unwrap()
            .boundary
        else {
            panic!("the face's outer loop is a cycle");
        };
        body.loop_cycle(first)
            .unwrap()
            .into_iter()
            .map(|he| body.get_half_edge(he).unwrap().start)
            .collect()
    }

    /// **A spline face's sector normal is its chart's `S_u × S_v` at the
    /// vertex, with the face's sense folded once.** The cube's top face on
    /// a planar spline chart reads `+z` at each corner whichever way the
    /// chart runs: on the chart along `(x, y)`, normal `+z`, sense true;
    /// and on its transpose, normal `−z`, sense false.
    #[test]
    fn a_spline_face_reads_its_outward_chart_normal_at_each_corner() {
        let chart = planar_chart(vec![0.0, 0.0, 1.0, 1.0], 1, &[0.0, 1.0]);
        for (chart, sense) in [(chart.clone(), true), (chart.transposed(), false)] {
            let (body, top) = spline_topped_cube(chart, sense);
            let vertices = corners(&body, top);
            assert_eq!(vertices.len(), 4, "the top face has four corners");
            for v in vertices {
                let resolved = resolve(&body, v, sector_of(&body, v, top), band())
                    .unwrap_or_else(|e| panic!("sense {sense}, corner {v:?}: {e:?}"));
                assert_eq!(resolved.face, top);
                assert_eq!(
                    resolved.carrier,
                    SectorCarrier::Spline(SurfaceKind::Nurbs),
                    "sense {sense}"
                );
                let n = resolved.at_vertex.tangent();
                assert!(
                    (n - geom_core::Vec3::new(0.0, 0.0, 1.0)).norm() < 1e-12,
                    "sense {sense}, corner {v:?}: {n:?} is not the outward +z"
                );
            }
        }
    }

    /// **A vertex on a crease of the chart refuses; the same vertex on a
    /// smooth knot reads.** The top face's `y = 0` edge is split at
    /// `x = ½`, where the chart's `u` knot `½` lies. On a degree-1 chart
    /// that knot is a crease run (multiplicity at least the degree), where
    /// the face has a normal on each side, so the vertex refuses typed. On
    /// a degree-2 chart the same simple knot is smooth, and the vertex
    /// reads `+z`.
    #[test]
    fn a_vertex_on_a_crease_of_the_chart_refuses_and_one_on_a_smooth_knot_reads() {
        let creased = planar_chart(vec![0.0, 0.0, 0.5, 1.0, 1.0], 1, &[0.0, 0.5, 1.0]);
        let smooth = planar_chart(
            vec![0.0, 0.0, 0.0, 0.5, 1.0, 1.0, 1.0],
            2,
            &[0.0, 0.25, 0.75, 1.0],
        );
        for (chart, crease) in [(creased, true), (smooth, false)] {
            let (body, top, mid) = split_at_half(chart);
            let got = resolve(&body, mid, sector_of(&body, mid, top), band());
            if crease {
                assert!(
                    matches!(got, Err(SectorFaceError::Crease { face, kind: SurfaceKind::Nurbs }) if face == top),
                    "a vertex on the crease refuses: {got:?}"
                );
            } else {
                let n = got
                    .expect("a vertex on a smooth knot reads")
                    .at_vertex
                    .tangent();
                assert!(
                    (n - geom_core::Vec3::new(0.0, 0.0, 1.0)).norm() < 1e-12,
                    "{n:?}"
                );
            }
        }
    }

    /// **A vertex at an irregular point of the chart refuses.** The top
    /// face of the prism over `(0, 0), (1, 0), (½, ½), (0, 1)` on the
    /// bilinear chart through those four corners: its Jacobian is
    /// `1 − (u + v)/2`, which vanishes at the corner `(½, ½)` alone, where
    /// `S_u` and `S_v` run back along the two collinear edges that meet
    /// there. `S_u × S_v` has no direction at that corner and the face
    /// no normal to read, though it is a plane; the other three corners
    /// read `+z`.
    #[test]
    fn a_vertex_at_an_irregular_point_of_the_chart_refuses() {
        use geom_core::Point3;
        use geom_core::spline::KnotVector;
        let lin = || KnotVector::clamped(vec![0.0, 0.0, 1.0, 1.0], 1).unwrap();
        let chart = geom::NurbsSurface::new(
            lin(),
            lin(),
            vec![
                Point3::new(0.0, 0.0, 1.0),
                Point3::new(0.0, 1.0, 1.0),
                Point3::new(1.0, 0.0, 1.0),
                Point3::new(0.5, 0.5, 1.0),
            ],
            vec![1.0; 4],
        )
        .unwrap();
        let prism = crate::test_support_fixtures::prism::<f64>(
            &[(0.0, 0.0), (1.0, 0.0), (0.5, 0.5), (0.0, 1.0)],
            1.0,
            Tol::witness(),
        );
        let (mut body, top) = (prism.body, prism.top_face);
        body.set_face_surface_unvouched_for_tests(
            top,
            crate::construction::FaceSurface::New {
                surface: geom::Surface::Nurbs(std::sync::Arc::new(chart)),
                sense: true,
            },
        )
        .unwrap();
        crate::pcurves::mint_pcurves(&mut body, Tol::witness()).unwrap();
        let vertices = corners(&body, top);
        assert_eq!(vertices.len(), 4, "the top face has four corners");
        for v in vertices {
            let at = *body.get_point(body.get_vertex(v).unwrap().point).unwrap();
            let got = resolve(&body, v, sector_of(&body, v, top), band());
            if at.x == 0.5 {
                assert!(
                    matches!(got, Err(SectorFaceError::Crease { face, kind: SurfaceKind::Nurbs }) if face == top),
                    "{at:?}: the irregular corner refuses: {got:?}"
                );
            } else {
                let n = got.expect("a regular corner reads").at_vertex.tangent();
                assert!(
                    (n - geom_core::Vec3::new(0.0, 0.0, 1.0)).norm() < 1e-12,
                    "{at:?}: {n:?}"
                );
            }
        }
    }
}

#[cfg(test)]
#[allow(clippy::unwrap_used, clippy::expect_used, clippy::panic)]
mod review_5802 {
    use super::*;
    use geom_core::spline::KnotVector;
    use geom_core::{Point3, Tol, Vec3};

    fn band() -> Band {
        Band::linear(Tol::witness()).unwrap()
    }

    /// The tensor chart over the unit square, `u` along `x` at columns
    /// `xs` on `ku`, `v` along `y` at rows `ys` on `kv`, height `z(i, j)`.
    fn chart(
        (ku, du, xs): (Vec<f64>, usize, &[f64]),
        (kv, dv, ys): (Vec<f64>, usize, &[f64]),
        z: impl Fn(usize, usize) -> f64,
    ) -> geom::NurbsSurface<f64> {
        let ku = KnotVector::clamped(ku, du).unwrap();
        let kv = KnotVector::clamped(kv, dv).unwrap();
        let mut control = Vec::new();
        for (i, &x) in xs.iter().enumerate() {
            for (j, &y) in ys.iter().enumerate() {
                control.push(Point3::new(x, y, z(i, j)));
            }
        }
        let n = control.len();
        geom::NurbsSurface::new(ku, kv, control, vec![1.0; n]).unwrap()
    }

    fn topped(chart: geom::NurbsSurface<f64>, sense: bool) -> (Body<f64>, FaceKey) {
        let cube = crate::test_support_fixtures::prism::<f64>(
            &[(0.0, 0.0), (1.0, 0.0), (1.0, 1.0), (0.0, 1.0)],
            1.0,
            Tol::witness(),
        );
        let mut body = cube.body;
        body.set_face_surface_unvouched_for_tests(
            cube.top_face,
            crate::construction::FaceSurface::New {
                surface: geom::Surface::Nurbs(std::sync::Arc::new(chart)),
                sense,
            },
        )
        .unwrap();
        crate::pcurves::mint_pcurves(&mut body, Tol::witness()).unwrap();
        (body, cube.top_face)
    }

    /// Splits the top edge whose ends satisfy `on` at its middle.
    fn split_top(body: &mut Body<f64>, on: impl Fn(Point3<f64>) -> bool) -> VertexKey {
        let edge = body
            .edges()
            .map(|(e, _)| e)
            .find(|&e| {
                let c = body.stored_curve(e).unwrap();
                let (t0, t1) = c.params();
                let (a, b) = (c.carrier().eval(t0), c.carrier().eval(t1));
                a.z == 1.0 && b.z == 1.0 && on(a) && on(b)
            })
            .expect("the top edge");
        let (t0, t1) = body.stored_curve(edge).unwrap().params();
        body.split_edge(edge, 0.5 * (t0 + t1), Tol::witness())
            .unwrap()
            .vertex
    }

    fn sector_of(body: &Body<f64>, vertex: VertexKey, face: FaceKey) -> HalfEdgeKey {
        body.vertex_orbit_linked(vertex)
            .into_iter()
            .find(|&he| face_of(body, vertex, he) == face)
            .expect("the vertex is a corner of the face")
    }

    /// The bump: biquadratic, boundary at `z = 1`, centre raised `h`. At
    /// the middle of the `y = 0` edge the chart has `S_u = (1, 0, 0)` and
    /// `S_v = (0, 1, h)`, so the outward normal is `(0, −h, 1)/‖·‖`,
    /// which no corner reads (each corner reads `+z`): a vertex end read
    /// at the wrong end of the edge reads `+z`.
    #[test]
    fn a_bumped_chart_reads_its_tilted_normal_at_an_edge_middle_both_ways_and_reverted() {
        let h = 0.3;
        let q = || (vec![0.0, 0.0, 0.0, 1.0, 1.0, 1.0], 2, &[0.0, 0.5, 1.0][..]);
        let bump = chart(
            q(),
            q(),
            |i, j| if (i, j) == (1, 1) { 1.0 + h } else { 1.0 },
        );
        let want = Vec3::new(0.0, -h, 1.0).normalize();
        for (chart, sense) in [(bump.clone(), true), (bump.transposed(), false)] {
            let (mut body, top) = topped(chart, sense);
            let mid = split_top(&mut body, |p| p.y == 0.0);
            let n = resolve(&body, mid, sector_of(&body, mid, top), band())
                .unwrap_or_else(|e| panic!("sense {sense}: {e:?}"))
                .at_vertex
                .tangent();
            assert!(
                (n - want).norm() < 1e-9,
                "sense {sense}: {n:?} not {want:?}"
            );
            let rev = body.revert().unwrap();
            let n = resolve(&rev, mid, sector_of(&rev, mid, top), band())
                .unwrap_or_else(|e| panic!("reverted, sense {sense}: {e:?}"))
                .at_vertex
                .tangent();
            assert!(
                (n + want).norm() < 1e-9,
                "reverted, sense {sense}: {n:?} not {:?}",
                -want
            );
        }
    }

    /// The same bump, every split edge: each edge middle tilts away from
    /// the bump, along the edge's own outward side.
    #[test]
    fn a_bumped_chart_reads_each_edge_middle() {
        let h = 0.3;
        let q = || (vec![0.0, 0.0, 0.0, 1.0, 1.0, 1.0], 2, &[0.0, 0.5, 1.0][..]);
        let bump = chart(
            q(),
            q(),
            |i, j| if (i, j) == (1, 1) { 1.0 + h } else { 1.0 },
        );
        type On = fn(Point3<f64>) -> bool;
        let rows: [(On, Vec3<f64>); 4] = [
            (|p| p.y == 0.0, Vec3::new(0.0, -h, 1.0)),
            (|p| p.y == 1.0, Vec3::new(0.0, h, 1.0)),
            (|p| p.x == 0.0, Vec3::new(-h, 0.0, 1.0)),
            (|p| p.x == 1.0, Vec3::new(h, 0.0, 1.0)),
        ];
        for (chart, sense) in [(bump.clone(), true), (bump.transposed(), false)] {
            for (on, want) in rows {
                let (mut body, top) = topped(chart.clone(), sense);
                let mid = split_top(&mut body, on);
                let n = resolve(&body, mid, sector_of(&body, mid, top), band())
                    .unwrap_or_else(|e| panic!("{e:?}"))
                    .at_vertex
                    .tangent();
                assert!(
                    (n - want.normalize()).norm() < 1e-9,
                    "sense {sense}: {n:?} not {want:?}"
                );
            }
        }
    }

    /// A kink with no knot: degree 2 in `u` on a simple knot `½`, columns
    /// 1 and 2 coincident at `x = ½`, so `S_u` vanishes along `u = ½`
    /// though no knot run reaches the degree. The body cannot be built
    /// (its `y = 0` edge's pcurve does not certify across the kink), so
    /// the arm's two checks are read at the point the walk would hand
    /// them: one of them refuses.
    #[test]
    fn a_coincident_control_row_kink_refuses() {
        let kinked = chart(
            (
                vec![0.0, 0.0, 0.0, 0.5, 1.0, 1.0, 1.0],
                2,
                &[0.0, 0.5, 0.5, 1.0],
            ),
            (vec![0.0, 0.0, 1.0, 1.0], 1, &[0.0, 1.0]),
            |_, _| 1.0,
        );
        for uv in [(0.5, 0.0), (0.5, 0.5)] {
            let read = geom_brep::regular_jet(&kinked, uv, 1.0, band())
                .and_then(|jet| geom_brep::chart_normal_at(&kinked, true, &jet, uv, 1.0, band()));
            assert!(read.is_err(), "{uv:?}: {:?}", read.map(|(n, _)| n.vec()));
        }
    }

    /// A crease in `v` (degree 1, knot `½` along `y`), the vertex at the
    /// middle of the `x = 0` edge, which runs along `v` across it; and the
    /// same crease in `u` reached through the transpose.
    #[test]
    fn a_crease_in_v_refuses() {
        let creased = chart(
            (vec![0.0, 0.0, 1.0, 1.0], 1, &[0.0, 1.0]),
            (vec![0.0, 0.0, 0.5, 1.0, 1.0], 1, &[0.0, 0.5, 1.0]),
            |_, _| 1.0,
        );
        for (chart, sense) in [(creased.clone(), true), (creased.transposed(), false)] {
            let (mut body, top) = topped(chart, sense);
            let mid = split_top(&mut body, |p| p.x == 0.0);
            let got = resolve(&body, mid, sector_of(&body, mid, top), band());
            assert!(
                matches!(got, Err(SectorFaceError::Crease { face, kind: SurfaceKind::Nurbs }) if face == top),
                "sense {sense}: {got:?}"
            );
        }
    }
}
