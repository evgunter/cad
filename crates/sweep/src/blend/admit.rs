//! **The surgery's front door, as types.**
//!
//! [`super::surgery::blend_surgery`] admits a verdict one clause at a
//! time — this chain's links are plane–plane and meet only at joints
//! on one support pair; this corner is trivalent with all three edges
//! requested; every requested edge of this support face ends at a
//! planned corner, joint or cut-off of it. A cut-off — an end whose
//! edge alone is requested — has no token of its own: the verdict
//! classified it, its plan reads it off the source, and a support's
//! admission counts it among the stations.
//! Each type here is one of those
//! clauses, and **holding the value is the fact**: a helper handed one
//! has no branch left to write about it. A refusal belongs to the door
//! that decides it, in the plan phase, before any mutation — never to a
//! helper below that cannot justify it and never to a panic.
//!
//! # How they are unforgeable, and where that stops
//!
//! Every field is private to this module and every constructor either
//! checks or derives what it claims: no `From`, no `Default`, no public
//! field, no argument taken on faith.
//!
//! **The boundary is this module, not this file.** A child module
//! (`admit/…`) would sit inside it and could mint any of these without
//! a check. Nothing in the language prevents that, so
//! `tests::every_token_type_has_exactly_one_construction_site` asserts
//! there is no child to be inside it.
//!
//! # What these tokens do NOT claim
//!
//! They describe the verdict and the source body **as the plan read
//! them**. [`OpenBand`] and [`AdmittedOpen`] borrow out of the verdict, which is
//! immutable for the whole run, so it cannot go stale. [`CornerFaces`] and
//! [`RequestedBoundary`] are read off the SOURCE body and consumed
//! against a clone, so a token may describe a face the carve has since
//! split — which is what the blank phase wants, and why the walk rides
//! in the token rather than being re-derived after.

use geom_core::{Decide, Point3, Real};
use topo::{Body, EdgeKey, EntityId, FaceKey, HalfEdgeKey, VertexKey};

use super::battery::{Chain, Convexity, JointVerdict, Link, joint_verdict};
use super::build::{fan_at, outward_of};
use super::surgery::{
    CORNER_SUPPORT_NOT_PLANAR, not_intact, unbuilt_chain, unbuilt_corner_config, unbuilt_geometry,
};
use super::{BlendError, CornerConfig};

/// **One link of an admitted open band**, whose arm is plane–plane
/// (the band between trivalent corners) or ruled (the cylinder band
/// between plane caps).
///
/// [`OpenBand::admit`] is the only way to obtain one: the token is
/// minted for each link of a chain that door admitted, so holding one
/// is holding a link the open-chain door has passed. Everything
/// downstream that used to re-test one of the door's properties takes
/// this instead; which of the two bands a holder carves is read off
/// the link's arm.
///
/// The token does not name a convexity, because no admission clause
/// reads one: the chamfer's strip and flat corner patch carry no
/// convexity parameter, and the rolling ball's band and corner fold
/// the link's stored verdict at every site that needs its sign. A
/// holder that needs the SIGN reads [`AdmittedOpen::convexity`].
pub(super) struct AdmittedOpen<'a, T: Real> {
    link: &'a Link<T>,
}

// Hand-written so the copy does not demand `T: Copy`: the value is one
// shared reference, and a proof used twice is the same proof.
impl<T: Real> Clone for AdmittedOpen<'_, T> {
    fn clone(&self) -> Self {
        *self
    }
}
impl<T: Real> Copy for AdmittedOpen<'_, T> {}

impl<'a, T: Real> AdmittedOpen<'a, T> {
    /// The token for one link of a chain [`OpenBand::admit`] is
    /// admitting — called from that door only, after its arm check.
    fn admitted(link: &'a Link<T>) -> Self {
        Self { link }
    }

    /// The admitted link.
    pub(super) fn link(&self) -> &'a Link<T> {
        self.link
    }

    /// The blended edge.
    pub(super) fn edge(&self) -> EdgeKey {
        self.link.edge
    }

    /// The link's convexity — either sign, under either band.
    ///
    /// A corner (the fillet's octant; the chamfer's flat patch) reads
    /// its orientation bit off any ONE of its incident links rather
    /// than testing that they agree, and what makes that sound is the
    /// battery's corner-configuration predicate: a termination is
    /// admitted only where all three of its edges carry ONE convexity,
    /// so the three links cannot disagree by the time a corner is
    /// planned.
    pub(super) fn convexity(&self) -> Convexity {
        self.link.convexity
    }
}

/// **A joint of an admitted open band**: an interior valence-2
/// vertex of the chain where two consecutive plane–plane links meet on
/// the SAME two support faces.
///
/// Identical supports are what make it the trivial junction: the arm
/// is one function of the two supports, so it is the same on either
/// side, the two links' bands are one surface, and the joint only
/// splits it. The band is carved as ONE face across the joint, whose
/// trimlines carry the joint's two feet as vertices — the same shape
/// the supports keep, whose boundary carries the joint itself.
pub(super) struct Joint {
    vertex: VertexKey,
    faces: [FaceKey; 2],
    arriving: EdgeKey,
}

impl Joint {
    /// Admit the junction at `vertex` between `arriving` and `leaving`
    /// as a joint; `chain` is the edge the chain refuses under.
    ///
    /// # Errors
    ///
    /// [`BlendError::UnsupportedChain`] when the two links are not both
    /// plane–plane, or do not lie between the same two support faces;
    /// [`BlendError::BodyNotIntact`] when the vertex's orbit does not
    /// walk; [`BlendError::UnsupportedCorner`] when the vertex carries
    /// edges other than the two links.
    fn admit<T: Decide>(
        body: &Body<T>,
        vertex: VertexKey,
        arriving: &Link<T>,
        leaving: &Link<T>,
        chain: EdgeKey,
    ) -> Result<Self, BlendError> {
        // Only the plane–plane band mints the struts a joint is fused
        // across: the ruled band is cut off at plane caps and
        // mints none, and a torus arm on an open arc has no band.
        match joint_verdict(body, vertex, arriving, leaving) {
            JointVerdict::Joint => {}
            JointVerdict::NotPlanar => {
                return Err(unbuilt_chain(
                    chain,
                    "an open chain's links meet on supports other than two planes; that \
                     junction is not implemented",
                ));
            }
            JointVerdict::OtherFaces => {
                return Err(unbuilt_chain(
                    chain,
                    "an open chain's links meet on different support faces; that junction is \
                     not implemented",
                ));
            }
            JointVerdict::Valence(valence) => {
                return Err(unbuilt_corner_config(
                    vertex,
                    CornerConfig::NEdgeVertex { valence },
                ));
            }
            JointVerdict::OrbitBroken => {
                return Err(not_intact(
                    EntityId::Vertex(vertex),
                    "a joint's vertex orbit does not walk",
                ));
            }
        }
        Ok(Self {
            vertex,
            faces: [arriving.face_a, arriving.face_b],
            arriving: arriving.edge,
        })
    }

    /// The joint vertex.
    pub(super) fn vertex(&self) -> VertexKey {
        self.vertex
    }

    /// The two support faces both links lie between, in the arriving
    /// link's `(face_a, face_b)` order.
    pub(super) fn faces(&self) -> [FaceKey; 2] {
        self.faces
    }

    /// The link arriving at the joint in walk order — the one whose
    /// trimlines the joint's feet are read off.
    pub(super) fn arriving(&self) -> EdgeKey {
        self.arriving
    }
}

/// **A chain admitted through the open-chain door**: its links, in
/// walk order, every one plane–plane or ruled, and every junction
/// between them a [`Joint`].
///
/// A one-link chain is a band of one link and no joints. A chain of
/// several is admitted only where each consecutive pair shares BOTH
/// support faces — a rim a valence-2 vertex splits, which is what a
/// coplanar-face merge leaves of a subdivided wall — and only on the
/// plane–plane arm. Any other junction is general junction
/// carry-through, and refuses.
///
/// Non-emptiness is the shape, as on [`super::battery::Chain`]: the
/// first link lives in its own field.
pub(super) struct OpenBand<'a, T: Real> {
    first: AdmittedOpen<'a, T>,
    rest: Vec<AdmittedOpen<'a, T>>,
    joints: Vec<Joint>,
}

impl<'a, T: Decide> OpenBand<'a, T> {
    /// The open-chain door: admit a chain the battery resolved, or
    /// refuse it through the frontier vocabulary.
    ///
    /// # Errors
    ///
    /// [`Joint::admit`]'s refusals at any junction, and
    /// [`BlendError::UnsupportedChain`] when a link's arm is neither
    /// plane–plane nor ruled.
    pub(super) fn admit(body: &Body<T>, chain: &'a Chain<T>) -> Result<Self, BlendError> {
        // Two open bands are built, and the door admits exactly those:
        // the plane–plane link, terminating in trivalent corners the
        // corner patch fills, and the RULED link — a cylinder band
        // about a straight spine over supports sharing the ruling —
        // terminating in plane caps the band is cut off at. The
        // battery's predicate 6 has already classified each end as
        // the one its arm needs; a coaxial torus arm on an open arc has
        // neither termination and refuses here.
        //
        // No convexity clause, and no verb: neither band asks for
        // either. The ruled strip is minted from the supports' own
        // outward normals and its corner patch from three trimline
        // crossings; the rolling ball's cylinder, corner ball, feet
        // and octant chart each fold the link's stored convexity
        // verdict — one decision, derived at every site that needs
        // its sign, on either side.
        let links: Vec<&'a Link<T>> = chain.links().collect();
        let mut joints = Vec::with_capacity(chain.junctions.len());
        for j in &chain.junctions {
            let (Some(a), Some(b)) = (links.get(j.arriving()), links.get(j.leaving())) else {
                return Err(not_intact(
                    EntityId::Vertex(j.vertex),
                    "a chain junction names a link position the chain does not carry",
                ));
            };
            joints.push(Joint::admit(body, j.vertex, a, b, chain.first().edge)?);
        }
        for link in chain.links() {
            if !(link.arm.is_plane_plane() || link.arm.is_ruled()) {
                return Err(unbuilt_chain(
                    link.edge,
                    "an open chain's supports are neither a plane–plane nor a ruled cylinder pair",
                ));
            }
        }
        let first = AdmittedOpen::admitted(chain.first());
        let rest = chain.rest().iter().map(AdmittedOpen::admitted).collect();
        Ok(Self {
            first,
            rest,
            joints,
        })
    }

    /// The band's first link in walk order — always present.
    pub(super) fn first(&self) -> AdmittedOpen<'a, T> {
        self.first
    }

    /// Every link, in walk order. Never empty.
    pub(super) fn links(&self) -> impl Iterator<Item = AdmittedOpen<'a, T>> + '_ {
        core::iter::once(self.first).chain(self.rest.iter().copied())
    }

    /// The lowest-keyed link — the one the band's face is ordered and
    /// surfaced by.
    pub(super) fn lowest(&self) -> AdmittedOpen<'a, T> {
        self.rest
            .iter()
            .copied()
            .fold(self.first, |m, o| if o.edge() < m.edge() { o } else { m })
    }

    /// The joints between consecutive links — empty on a one-link band.
    pub(super) fn joints(&self) -> &[Joint] {
        &self.joints
    }
}

/// **The admitted open links incident to one corner vertex** — at
/// least one, every one through the open-chain door, every one
/// terminating at that vertex.
///
/// Non-emptiness is the shape: the seed link lives in its own field, so
/// [`CornerLinks::first`] returns a link rather than an `Option` and
/// there is no "a corner has no requested incident link" state left to
/// refuse. **Incidence is a check**, made by both constructors — the
/// vertex arrives as a separate argument, so it is the one thing here a
/// caller could get wrong.
pub(super) struct CornerLinks<'a, T: Real> {
    vertex: VertexKey,
    first: AdmittedOpen<'a, T>,
    rest: Vec<AdmittedOpen<'a, T>>,
}

impl<'a, T: Real> CornerLinks<'a, T> {
    /// A link terminates at `vertex`, or the plan's own data disagrees
    /// with itself.
    fn incident(vertex: VertexKey, link: AdmittedOpen<'a, T>) -> Result<(), BlendError> {
        let l = link.link();
        if l.start == vertex || l.end == vertex {
            return Ok(());
        }
        Err(not_intact(
            EntityId::Vertex(vertex),
            "a corner's incidence list was offered a link that does not terminate there",
        ))
    }

    /// Start a corner's incidence list from the link that discovered it.
    ///
    /// # Errors
    ///
    /// [`BlendError::BodyNotIntact`] when `first` does not terminate at
    /// `vertex`.
    pub(super) fn seed(vertex: VertexKey, first: AdmittedOpen<'a, T>) -> Result<Self, BlendError> {
        Self::incident(vertex, first)?;
        Ok(Self {
            vertex,
            first,
            rest: Vec::new(),
        })
    }

    /// Record another admitted link terminating at this corner.
    ///
    /// # Errors
    ///
    /// [`BlendError::BodyNotIntact`] when `link` does not terminate at
    /// this corner's vertex.
    pub(super) fn also(&mut self, link: AdmittedOpen<'a, T>) -> Result<(), BlendError> {
        Self::incident(self.vertex, link)?;
        self.rest.push(link);
        Ok(())
    }

    /// The corner vertex.
    pub(super) fn vertex(&self) -> VertexKey {
        self.vertex
    }

    /// The link that discovered this corner — always present.
    pub(super) fn first(&self) -> AdmittedOpen<'a, T> {
        self.first
    }

    /// The incident links after [`CornerLinks::first`].
    pub(super) fn rest(&self) -> &[AdmittedOpen<'a, T>] {
        &self.rest
    }

    /// The incident links in edge-key order — what the corner fusion
    /// walks, ordered here rather than by trusting the order the caller
    /// fed them in.
    ///
    /// **Seeded, in the shape this type's own fields have**: the
    /// lowest-keyed link in its own slot, the remainder behind it. The
    /// non-emptiness that makes [`CornerLinks::first`] total therefore
    /// survives the ordering, and a consumer that walks the links in
    /// order still holds one of them by the type.
    ///
    /// **The seed is the MINIMUM, not [`CornerLinks::first`]**, and the
    /// two are not interchangeable at a consumer even though today's
    /// one builder makes them equal: it seeds each corner from the
    /// first link that reaches it while walking the open links in
    /// ascending edge order, so the swap below never fires. Not
    /// depending on that walk is the whole reason this function
    /// exists — a consumer that needs the ordered walk's first element
    /// reads it HERE, and one that needs any single incident link (a
    /// convexity, a chart candidate) reads `first`, where order does
    /// not enter.
    pub(super) fn sorted(&self) -> (AdmittedOpen<'a, T>, Vec<AdmittedOpen<'a, T>>) {
        // The minimum is carried in a slot as the walk runs, never
        // searched for in a built `Vec` — so there is no empty case to
        // answer for and none to refuse.
        let mut first = self.first;
        let mut rest: Vec<AdmittedOpen<'a, T>> = Vec::with_capacity(self.rest.len());
        for &link in &self.rest {
            if link.edge() < first.edge() {
                rest.push(first);
                first = link;
            } else {
                rest.push(link);
            }
        }
        rest.sort_by_key(AdmittedOpen::edge);
        (first, rest)
    }
}

/// Pairwise distinctness of a corner's three support faces — the
/// property [`CornerFaces::third`]'s totality rests on. Free-standing
/// so it can be exercised directly: its one call site is unreachable by
/// input (see [`CornerFaces::admit`]).
fn distinct(f0: FaceKey, f1: FaceKey, f2: FaceKey) -> bool {
    f0 != f1 && f1 != f2 && f0 != f2
}

/// **A trivalent corner's three distinct support faces**, in orbit
/// order, and the VERTEX they were walked from.
///
/// The array is the claim: three faces, pairwise distinct. That is
/// what makes [`CornerFaces::third`] total **over this corner's own
/// pairs** — excluding two of three distinct faces always leaves one —
/// and it is the fact the octant's chart pick used to fall off with a
/// run-out refusal it could not justify.
///
/// **The vertex is kept because the faces alone cannot identify the
/// corner.** Two ends of one edge share both of its supports and
/// differ only in the third, so a consumer holding this token beside a
/// [`CornerLinks`] can check that the two describe the same corner —
/// and comparing the faces would not tell those two apart.
pub(super) struct CornerFaces {
    vertex: VertexKey,
    faces: [FaceKey; 3],
}

impl CornerFaces {
    /// Walk a corner's face orbit and admit it as a trivalent corner.
    ///
    /// The valence checked here is the FACE orbit's; on a manifold
    /// body it is the edge valence the caller checked, and a
    /// disagreement is itself the refusal.
    ///
    /// # Errors
    ///
    /// [`BlendError::BodyNotIntact`] when the orbit does not walk, or
    /// when it returns a face twice;
    /// [`BlendError::UnsupportedCorner`] when the corner is not
    /// trivalent.
    pub(super) fn admit<T: Decide>(body: &Body<T>, vertex: VertexKey) -> Result<Self, BlendError> {
        let faces = fan_at(body.faces_of_vertex(vertex)).ok_or_else(|| {
            not_intact(
                EntityId::Vertex(vertex),
                "a corner's face orbit does not walk",
            )
        })?;
        let [f0, f1, f2] = faces[..] else {
            return Err(unbuilt_corner_config(
                vertex,
                CornerConfig::NEdgeVertex {
                    valence: faces.len(),
                },
            ));
        };
        // Distinctness is what makes `third` total, so it is checked
        // here rather than inherited from `Body::faces_of_vertex`' dedup.
        // **This arm cannot fire today** — the walk already dedups, so
        // no input reaches it and no row can drive it; what is guarded
        // is the predicate, in `distinct_faces_is_pairwise`, and what
        // is unguarded is this one call to it.
        if !distinct(f0, f1, f2) {
            return Err(not_intact(
                EntityId::Vertex(vertex),
                "a corner's face orbit returned one face twice",
            ));
        }
        Ok(Self {
            vertex,
            faces: [f0, f1, f2],
        })
    }

    /// The vertex whose orbit these faces were walked from.
    pub(super) fn vertex(&self) -> VertexKey {
        self.vertex
    }

    /// The three faces, in orbit order.
    pub(super) fn as_slice(&self) -> &[FaceKey] {
        &self.faces
    }

    /// Whether `face` is one of the corner's three.
    pub(super) fn contains(&self, face: FaceKey) -> bool {
        self.faces.contains(&face)
    }

    /// Where `face` sits in orbit order — the index a corner's
    /// per-support rows (normals, feet) are keyed by, so a lookup
    /// cannot silently take another support's row.
    pub(super) fn slot_of(&self, face: FaceKey) -> Option<usize> {
        self.faces.iter().position(|f| *f == face)
    }

    /// The corner's remaining support once `a` and `b` are excluded —
    /// `Some` exactly when `a` and `b` are two DISTINCT supports of
    /// this corner, where excluding two of three distinct faces always
    /// leaves one.
    ///
    /// **`None` rather than a plausible answer**: excluding a face this
    /// corner does not hold leaves TWO, and naming either would be a
    /// support pair that is not this corner's, scored as if it were.
    /// The consumer derives a CHART from the answer, so a plausible one
    /// is worse than none. This is a NECESSARY condition and not the
    /// whole check — the two ends of one edge share both its supports,
    /// so identifying the corner is [`CornerFaces::vertex`]'s job.
    pub(super) fn third(&self, a: FaceKey, b: FaceKey) -> Option<FaceKey> {
        if a == b || !self.contains(a) || !self.contains(b) {
            return None;
        }
        let [f0, f1, f2] = self.faces;
        Some(if f0 != a && f0 != b {
            f0
        } else if f1 != a && f1 != b {
            f1
        } else {
            f2
        })
    }
}

/// One station of an admitted support face: a vertex of its boundary
/// where a requested edge ends, and the band's foot on this face there
/// (the fillet's ball rest or the chamfer's trimline crossing at a
/// corner, the trimline's foot at a joint or a cut-off — the plan
/// derives it, the door carries it).
pub(super) struct BoundaryStation<T: Real> {
    /// The cycle half-edge whose start is [`BoundaryStation::vertex`]:
    /// where a strut is spliced.
    pub(super) half_edge: HalfEdgeKey,
    /// The boundary vertex.
    pub(super) vertex: VertexKey,
    /// The band's foot on this face.
    pub(super) foot: Point3<T>,
    /// Which planned end or joint the station is.
    pub(super) kind: StationKind,
}

/// Which of the three planned shapes a station is — one, by admission.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub(super) enum StationKind {
    /// A corner patch's sharp vertex.
    Corner,
    /// A joint, where one band runs on through.
    Joint,
    /// A cut-off's old vertex.
    CutOff,
}

impl StationKind {
    /// Whether the carve spins a STRUT out to the foot — at a corner or
    /// a joint, whose foot lies inside the face — rather than finding
    /// it already on a split rim, at a cut-off.
    pub(super) fn spins_a_strut(self) -> bool {
        match self {
            Self::Corner | Self::Joint => true,
            Self::CutOff => false,
        }
    }
}

/// One requested edge of an admitted support face, as its boundary
/// traverses it: the carve's trimline chord runs from the foot at
/// `from` to the foot at `to`.
pub(super) struct BoundaryChord<T: Real> {
    /// The requested edge's half-edge in this face's cycle.
    pub(super) half_edge: HalfEdgeKey,
    /// The requested edge.
    pub(super) edge: EdgeKey,
    /// The station the half-edge starts at.
    pub(super) from: VertexKey,
    /// The station it ends at.
    pub(super) to: VertexKey,
    /// The band's feet on this face at `from` and at `to`, the two
    /// stations' own.
    pub(super) feet: [Point3<T>; 2],
}

/// One planned cut-off, as support admission reads it: the old vertex,
/// the band's two supports, and its foot on each in that order.
pub(super) type CutOffRow<T> = (VertexKey, [FaceKey; 2], [Point3<T>; 2]);

/// **A support face's requested boundary**: every requested edge in its
/// cycles, each ending at two stations, and every station a planned
/// corner or joint that counts this face among its supports, or a
/// planned cut-off whose band this face supports.
///
/// The blank phase carves such a face LOCALLY — one strip per requested
/// edge, the rest of the face shrunk and kept — and that carve is
/// well-defined under exactly this property. Admission checks it in the
/// plan phase, before any mutation, and hands the carve the walk it
/// checked plus each station's foot, so the carve reads no source
/// geometry of its own.
pub(super) struct RequestedBoundary<T: Real> {
    face: FaceKey,
    stations: Vec<BoundaryStation<T>>,
    chords: Vec<BoundaryChord<T>>,
}

// `Decide` alone: admission walks a cycle and folds a stored plane
// normal, and decides nothing that reads a bracket. The fillet seam's
// ratified compound `Decide + Bounds` bound (`geom-core/src/real.rs`,
// the `Bounds` scope rule) covers three files and this is not one.
impl<T: Decide> RequestedBoundary<T> {
    /// Admit one support face of the plan.
    ///
    /// `corners` is `(vertex, its three faces, its three FEET in those
    /// faces' orbit order)` for every planned corner, `joints` is
    /// `(joint, its foot on each of its two faces in that order)` for
    /// every planned joint, and `cut_offs` is `(vertex, the band's two
    /// supports, its foot on each in that order)` for every planned
    /// cut-off. The feet are the plan's, not this door's: where a band's
    /// trimlines meet a support is what the two verbs derive differently
    /// (the ball's foot; the two trimlines' crossing), and deriving it
    /// here would put that difference in the door instead of in the plan
    /// that owns it.
    ///
    /// # Errors
    ///
    /// [`BlendError::BodyNotIntact`] when a cycle of the face does not
    /// walk, or a requested edge ends at a vertex that is not exactly
    /// one planned station of this face; [`BlendError::UnsupportedGeometry`]
    /// when the face is not a plane.
    pub(super) fn admit(
        body: &Body<T>,
        face: FaceKey,
        opens: &[AdmittedOpen<'_, T>],
        corners: &[(VertexKey, &CornerFaces, [Point3<T>; 3])],
        joints: &[(&Joint, [Point3<T>; 2])],
        cut_offs: &[CutOffRow<T>],
    ) -> Result<Self, BlendError> {
        // Read once so a face that is not a plane refuses at this door
        // rather than deeper in the carve.
        outward_of(body, face)
            .ok_or_else(|| unbuilt_geometry(EntityId::Face(face), CORNER_SUPPORT_NOT_PLANAR))?;
        let fd = body
            .get_face(face)
            .ok_or_else(|| not_intact(EntityId::Face(face), "a support face"))?;
        let station = |he: HalfEdgeKey, v: VertexKey| -> Result<BoundaryStation<T>, BlendError> {
            let corner = corners
                .iter()
                .find(|(c, faces, _)| *c == v && faces.contains(face))
                .map(|(_, faces, feet)| faces.slot_of(face).map(|slot| feet[slot]));
            let joint = joints
                .iter()
                .find(|(j, _)| j.vertex() == v && j.faces().contains(&face))
                .map(|(j, feet)| {
                    if j.faces()[0] == face {
                        feet[0]
                    } else {
                        feet[1]
                    }
                });
            let cut = cut_offs
                .iter()
                .find(|(c, faces, _)| *c == v && faces.contains(&face))
                .map(|(_, faces, feet)| if faces[0] == face { feet[0] } else { feet[1] });
            // A corner and a joint END a band and run through it, and a
            // cut-off's vertex carries one requested edge where a corner
            // carries three — so a vertex is at most one of the three,
            // and a requested edge's end is at least one.
            let (foot, kind) = match (corner, joint, cut) {
                (Some(Some(foot)), None, None) => (foot, StationKind::Corner),
                (None, Some(foot), None) => (foot, StationKind::Joint),
                (None, None, Some(foot)) => (foot, StationKind::CutOff),
                _ => {
                    return Err(not_intact(
                        EntityId::Vertex(v),
                        "a requested edge of a support ends at a vertex that is not exactly one \
                         planned corner, joint or cut-off of that support",
                    ));
                }
            };
            Ok(BoundaryStation {
                half_edge: he,
                vertex: v,
                foot,
                kind,
            })
        };
        // A station's foot, admitting the station the first time one of
        // its requested edges reaches it.
        let footed = |stations: &mut Vec<BoundaryStation<T>>,
                      he: HalfEdgeKey,
                      v: VertexKey|
         -> Result<Point3<T>, BlendError> {
            if let Some(s) = stations.iter().find(|s| s.vertex == v) {
                return Ok(s.foot);
            }
            let s = station(he, v)?;
            let foot = s.foot;
            stations.push(s);
            Ok(foot)
        };
        let mut stations: Vec<BoundaryStation<T>> = Vec::new();
        let mut chords = Vec::new();
        for lp in core::iter::once(fd.outer).chain(fd.rings.iter().copied()) {
            let walk = loop_cycle_of(body, lp).ok_or_else(|| {
                not_intact(
                    EntityId::Loop(lp),
                    "a cycle of a support face does not walk",
                )
            })?;
            let n = walk.len();
            for (i, &he) in walk.iter().enumerate() {
                // Every member of a returned cycle was resolved by the
                // bounded walk that returned it.
                let (Some(h), Some(next)) = (
                    body.get_half_edge(he),
                    body.get_half_edge(walk[(i + 1) % n]),
                ) else {
                    unreachable!(
                        "support admission: cycle members are proven live by the bounded \
                         walk `loop_cycle_of` just ran"
                    )
                };
                if !opens.iter().any(|o| o.edge() == h.edge) {
                    continue;
                }
                let feet = [
                    footed(&mut stations, he, h.start)?,
                    footed(&mut stations, walk[(i + 1) % n], next.start)?,
                ];
                chords.push(BoundaryChord {
                    half_edge: he,
                    edge: h.edge,
                    from: h.start,
                    to: next.start,
                    feet,
                });
            }
        }
        Ok(Self {
            face,
            stations,
            chords,
        })
    }

    /// The admitted support face.
    pub(super) fn face(&self) -> FaceKey {
        self.face
    }

    /// Its stations, in cycle order.
    pub(super) fn stations(&self) -> &[BoundaryStation<T>] {
        &self.stations
    }

    /// Its requested edges, in cycle order.
    pub(super) fn chords(&self) -> &[BoundaryChord<T>] {
        &self.chords
    }
}

/// The half-edges of one loop's cycle, in `next` order: empty for a
/// lone-vertex loop, `None` where the loop or its cycle does not walk.
fn loop_cycle_of<T: Decide>(body: &Body<T>, lp: topo::LoopKey) -> Option<Vec<HalfEdgeKey>> {
    match body.get_loop(lp)?.boundary {
        topo::LoopBoundary::Cycle { first } => body.loop_cycle(first),
        topo::LoopBoundary::Empty { .. } => Some(Vec::new()),
    }
}

#[cfg(test)]
#[allow(clippy::expect_used, clippy::panic)]
mod tests {
    use geom_core::Tol;
    use topo::FaceKey;

    use super::super::BlendError;
    use super::super::battery::{Chain, ChainClosure, Link};
    use super::{AdmittedOpen, CornerFaces, CornerLinks, OpenBand};
    use crate::test_support::{L, all_links, cube};

    /// **What guards the unforgeability claim**, since nothing else
    /// can.
    ///
    /// Field privacy is the compiler's job. What privacy does NOT guard
    /// is a constructor added *inside* the boundary that skips the
    /// check, so this row asserts both halves of "inside":
    ///
    /// 1. **Six token types, six `Self` struct literals, one apiece**,
    ///    each inside the door that checks — the open link's inside
    ///    [`super::OpenBand::admit`]'s, through the one private helper
    ///    that door calls. A seventh reddens this row.
    /// 2. **No child module**, because `admit/…` would be inside the
    ///    privacy boundary *and* outside this file's text — the one
    ///    escape that defeats clause 1 silently.
    ///
    /// **The reader is the shared one** — `test_utils::source`, in its
    /// CODE view, comments and string literals blanked. That is what
    /// lets the needles below be spelled plainly: a needle written
    /// here is a string literal, and a literal is blanked, so the scan
    /// cannot match itself. What this file used to do instead was
    /// splice the needle out of pieces at run time, which buys the
    /// same non-self-matching and no lexing at all — a construction
    /// site quoted in a doc comment or a message counted as one, and a
    /// real site commented out went on counting. `test-utils`'s own
    /// census row — `reader_census::every_site_that_reads_rust_source_is_in_the_ledger`,
    /// in that crate's `tests/`, not this one's — is what keeps that
    /// choice from being made again silently.
    ///
    /// **Blind spot, stated:** clause 1 is a text scan over a lexed
    /// view, not a parse. `Self{…}` without the space, a literal
    /// written by type name, or a route through `Default` escapes it;
    /// it catches the accident it is aimed at, not a determined
    /// evasion, and it cannot judge whether a door's check is the
    /// RIGHT one. `distinct_faces_is_pairwise` and
    /// `admission_makes_the_third_support_total` cover that half where
    /// there is a decision to get wrong.
    #[test]
    fn every_token_type_has_exactly_one_construction_site() {
        let source = test_utils::source::code_only(include_str!("admit.rs"));
        let literals = source.matches("Self {").count() - source.matches("-> Self {").count();
        assert_eq!(
            literals, 6,
            "admit.rs must hold exactly one construction site per token type \
             (AdmittedOpen, Joint, OpenBand, CornerLinks, CornerFaces, RequestedBoundary) \
             — found {literals}"
        );
        let child = std::path::Path::new(env!("CARGO_MANIFEST_DIR")).join("src/blend/admit");
        assert!(
            !child.exists(),
            "a child module of `admit` is inside the privacy boundary and outside the scan \
             above, so it can mint any token here with no check and leave this row green: \
             {}",
            child.display()
        );
    }

    /// **The one thing a caller of [`CornerLinks`] can get wrong.** The
    /// vertex arrives as its own argument, so a link that terminates
    /// nowhere near it would be admitted on faith — which is the gap
    /// between "every constructor checks" and "every constructor exists
    /// for a reason". Both constructors refuse it.
    ///
    /// Unreachable at today's one call site, where the vertex is read
    /// off the link itself; the check is here so the type's claim does
    /// not depend on that staying true.
    #[test]
    fn corner_links_refuses_a_link_that_terminates_elsewhere() {
        let body = cube(L, Tol::witness());
        let links = all_links(&body, Tol::witness());
        let chains: Vec<Chain<f64>> = links.iter().cloned().map(open_chain).collect();
        let admitted: Vec<AdmittedOpen<'_, f64>> = chains
            .iter()
            .map(|c| {
                OpenBand::admit(&body, c)
                    .expect("a cube's links are plane–plane")
                    .first()
            })
            .collect();
        let vertex = links[0].start;
        let stranger = *admitted
            .iter()
            .find(|o| {
                let l = o.link();
                l.start != vertex && l.end != vertex
            })
            .expect("a cube has links touching neither end of a given vertex");
        assert!(
            matches!(
                CornerLinks::seed(vertex, stranger),
                Err(BlendError::BodyNotIntact { .. })
            ),
            "seed must refuse a link that does not terminate at the corner"
        );
        let seed = *admitted
            .iter()
            .find(|o| o.link().start == vertex || o.link().end == vertex)
            .expect("a link at this vertex");
        let mut c = CornerLinks::seed(vertex, seed).expect("the seed terminates here");
        assert!(
            matches!(c.also(stranger), Err(BlendError::BodyNotIntact { .. })),
            "also must refuse it too"
        );
    }

    /// One open chain per link, so the door has something to admit.
    fn open_chain(link: Link<f64>) -> Chain<f64> {
        let (head, tail) = (link.start, link.end);
        Chain::new(
            link,
            Vec::new(),
            Vec::new(),
            ChainClosure::Open { head, tail },
        )
    }

    /// The predicate [`super::CornerFaces::third`]'s totality rests on.
    /// Its one call site cannot fire (`Body::faces_of_vertex` already dedups), so
    /// the property is exercised here instead of being left as a guard
    /// nobody can tell is working.
    #[test]
    fn distinct_faces_is_pairwise() {
        let body = cube(L, Tol::witness());
        let vertex = all_links(&body, Tol::witness())[0].start;
        let faces = CornerFaces::admit(&body, vertex).expect("a cube corner is trivalent");
        let [f0, f1, f2] = match faces.as_slice() {
            [a, b, c] => [*a, *b, *c],
            other => panic!("three faces, got {other:?}"),
        };
        assert!(super::distinct(f0, f1, f2));
        assert!(!super::distinct(f0, f0, f2), "first pair");
        assert!(!super::distinct(f0, f1, f1), "second pair");
        assert!(!super::distinct(f0, f1, f0), "the wrap-around pair");
    }

    /// **The admission is what makes [`CornerFaces::third`] total over
    /// this corner's own pairs**, so this row exercises both halves
    /// against a real corner: the door returns three distinct faces,
    /// every exclusion pair drawn from them names the remaining one,
    /// and a pair drawn from anywhere else is refused rather than
    /// answered.
    ///
    /// The first half is why the octant's chart pick no longer carries
    /// a run-out refusal it could not justify. **The second is why it
    /// cannot be scored off a corner it does not belong to**: the
    /// answer to a stranger pair would be a face this corner holds and
    /// that pair does not exclude, which reads exactly like a right
    /// answer and produces a wrong chart.
    #[test]
    fn admission_makes_the_third_support_total() {
        let body = cube(L, Tol::witness());
        let vertex = all_links(&body, Tol::witness())[0].start;
        let faces = CornerFaces::admit(&body, vertex).expect("a cube corner is trivalent");
        let [f0, f1, f2] = match faces.as_slice() {
            [a, b, c] => [*a, *b, *c],
            other => panic!("admission must yield exactly three faces, got {other:?}"),
        };
        assert!(
            f0 != f1 && f1 != f2 && f0 != f2,
            "admission must yield three DISTINCT faces"
        );
        for (a, b) in [(f0, f1), (f1, f2), (f0, f2)] {
            let t = faces
                .third(a, b)
                .expect("a pair of this corner's own supports has a third");
            assert!(t != a && t != b, "the third support excludes both");
            assert!(faces.contains(t), "and is one of the corner's own");
        }
        // A pair that is not both members has no third: excluding a
        // stranger leaves TWO of the corner's faces, and naming either
        // is a chart scored off a support pair that is not this
        // corner's.
        let stranger = FaceKey::default();
        assert!(!faces.contains(stranger), "the stranger is not a member");
        assert!(faces.third(f0, stranger).is_none(), "one stranger");
        assert!(faces.third(stranger, f1).is_none(), "the other side");
        assert!(
            faces.third(stranger, stranger).is_none(),
            "two strangers, which excludes nothing at all"
        );
        // A member paired with ITSELF excludes one face and leaves
        // two, which is the same defect wearing a member's key.
        assert!(faces.third(f0, f0).is_none(), "a face against itself");
    }
}
