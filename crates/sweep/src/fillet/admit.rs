//! **The surgery's front door, as types.**
//!
//! [`super::surgery::fillet_surgery`] admits a verdict one clause at a
//! time — this chain is a single convex plane–plane link; this corner
//! is trivalent with all three edges requested; this support face has
//! its entire outer cycle requested. Those clauses used to be carried
//! downward as PROSE: a helper two frames below refused a state its
//! caller had already excluded, and could say so only in a comment,
//! because the fact was not in anything it was handed.
//!
//! Each type here is one of those clauses. **Holding the value is the
//! fact**, so the helper that used to re-refuse takes the value
//! instead and has no branch left to write. Nothing here converts a
//! refusal into a panic: the refusal moves to the door that decides
//! it, in the plan phase, before any mutation.
//!
//! # How they are unforgeable
//!
//! Every field below is **private to this module**, and every function
//! that produces one either performs the check itself or takes a value
//! that already carries it. There is no `From`, no public field, no
//! `Default`, and no constructor that takes the underlying data on
//! faith. `crates/sweep/src/fillet/{build,surgery}.rs` are sibling
//! modules, not children, so field privacy holds against them.
//!
//! Two of the four carry their fact by SHAPE rather than by a check —
//! [`CornerLinks`] stores its first link in its own field and can
//! never be empty, and [`CornerFaces`] stores three faces in an array.
//! For those the constructor cannot lie because there is nothing to
//! lie about.
//!
//! # What these tokens do NOT claim
//!
//! They are statements about the **verdict and the source body as the
//! plan read them**, not about a body under mutation. [`ConvexOpen`]
//! borrows a link out of the verdict, which is immutable for the
//! surgery's whole run, so it cannot go stale. [`CornerFaces`] and
//! [`RequestedBoundary`] are read off the SOURCE body during the plan
//! phase, and the mutation phase runs on a clone — so a token can
//! describe a face the mutation has since carved. That is exactly what
//! the blank phase wants (it is carving along the boundary the plan
//! recorded), and it is why the rows are carried in the token rather
//! than re-walked afterwards.

use geom_core::{Bounds, Decide, Point3, Real};
use topo::{Body, EdgeKey, EntityId, FaceKey, HalfEdgeKey, VertexKey};

use super::battery::{Chain, Convexity, Link};
use super::blend::BlendArm;
use super::build::{face_cycle, outward_of, vertex_faces};
use super::surgery::{
    CORNER_SUPPORT_NOT_PLANAR, not_intact, unbuilt_chain, unbuilt_corner_config,
    unbuilt_geometry, unbuilt_run_out,
};
use super::{CornerConfig, FilletError};

/// **A chain admitted through the open-chain door**: exactly one link,
/// plane–plane supports, convex.
///
/// [`ConvexOpen::admit`] is the only way to obtain one, and it is the
/// door — the three refusals it raises are the surgery's own
/// open-chain frontier, unchanged. Everything downstream that used to
/// re-test one of those three properties takes this instead.
pub(super) struct ConvexOpen<'a, T: Real> {
    link: &'a Link<T>,
}

// Hand-written so the copy does not demand `T: Copy`: the value is one
// shared reference, and a proof used twice is the same proof.
impl<T: Real> Clone for ConvexOpen<'_, T> {
    fn clone(&self) -> Self {
        *self
    }
}
impl<T: Real> Copy for ConvexOpen<'_, T> {}

impl<'a, T: Real> ConvexOpen<'a, T> {
    /// The open-chain door: admit a chain the battery resolved, or
    /// refuse it through the frontier vocabulary.
    ///
    /// # Errors
    ///
    /// [`FilletError::UnsupportedChain`] when the chain has more than
    /// one link (junction carry-through), when its supports are not
    /// plane–plane, or when it is concave.
    pub(super) fn admit(chain: &'a Chain<T>) -> Result<Self, FilletError> {
        let link = chain.first();
        if !chain.rest().is_empty() {
            return Err(unbuilt_chain(
                link.edge,
                "an open chain with more than one link needs junction \
                 carry-through, which is not implemented",
            ));
        }
        if !matches!(link.arm, BlendArm::PlanePlaneCylinder) {
            return Err(unbuilt_chain(
                link.edge,
                "an open chain's supports are not plane–plane (the octant \
                 corner is the only termination built)",
            ));
        }
        if !matches!(link.convexity, Convexity::Convex) {
            return Err(unbuilt_chain(
                link.edge,
                "a concave chain adds material, which the surgery does not \
                 build — not implemented",
            ));
        }
        Ok(Self { link })
    }

    /// The admitted link.
    pub(super) fn link(&self) -> &'a Link<T> {
        self.link
    }

    /// The blended edge.
    pub(super) fn edge(&self) -> EdgeKey {
        self.link.edge
    }

    /// The link's convexity. **`Convex` for every admitted link** —
    /// [`ConvexOpen::admit`] refuses anything else — which is what
    /// lets a corner read its octant's orientation bit off any one of
    /// its incident links instead of testing that they agree.
    pub(super) fn convexity(&self) -> Convexity {
        self.link.convexity
    }
}

/// **The admitted open links incident to one corner vertex** — at
/// least one, every one convex.
///
/// Non-emptiness is the shape, not a check: the seed link lives in its
/// own field, so [`CornerLinks::first`] returns a link rather than an
/// `Option` and there is no "a corner has no requested incident link"
/// state to refuse. That refusal used to appear three times, in three
/// functions, each of which could only cite the loop one or two frames
/// up that built the corner list out of the links.
pub(super) struct CornerLinks<'a, T: Real> {
    vertex: VertexKey,
    first: ConvexOpen<'a, T>,
    rest: Vec<ConvexOpen<'a, T>>,
}

impl<'a, T: Real> CornerLinks<'a, T> {
    /// Start a corner's incidence list from the link that discovered
    /// it.
    pub(super) fn seed(vertex: VertexKey, first: ConvexOpen<'a, T>) -> Self {
        Self {
            vertex,
            first,
            rest: Vec::new(),
        }
    }

    /// Record another admitted link terminating at this corner.
    pub(super) fn also(&mut self, link: ConvexOpen<'a, T>) {
        self.rest.push(link);
    }

    /// The corner vertex.
    pub(super) fn vertex(&self) -> VertexKey {
        self.vertex
    }

    /// The link that discovered this corner — always present.
    pub(super) fn first(&self) -> ConvexOpen<'a, T> {
        self.first
    }

    /// Every incident admitted link. Never empty.
    pub(super) fn iter(&self) -> impl Iterator<Item = ConvexOpen<'a, T>> + Clone + '_ {
        core::iter::once(self.first).chain(self.rest.iter().copied())
    }

    /// The incident links, sorted by edge key — the deterministic
    /// order the corner fusion walks them in.
    pub(super) fn sorted(&self) -> Vec<ConvexOpen<'a, T>> {
        let mut all: Vec<ConvexOpen<'a, T>> = self.iter().collect();
        all.sort_by_key(ConvexOpen::edge);
        all
    }
}

/// **A trivalent corner's three distinct support faces**, in orbit
/// order.
///
/// The array is the claim: three faces, pairwise distinct. That is
/// what makes [`CornerFaces::third`] total — excluding two of three
/// distinct faces always leaves one — and it is the fact the octant's
/// chart pick used to fall off with a run-out refusal it could not
/// justify.
pub(super) struct CornerFaces {
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
    /// [`FilletError::BodyNotIntact`] when the orbit does not walk, or
    /// when it returns a face twice;
    /// [`FilletError::FilletCornerUnsupported`] when the corner is not
    /// trivalent.
    pub(super) fn admit<T: Decide>(
        body: &Body<T>,
        vertex: VertexKey,
    ) -> Result<Self, FilletError> {
        let faces = vertex_faces(body, vertex).ok_or_else(|| {
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
        // `vertex_faces` returns distinct faces; the array's whole
        // claim rests on that, so it is checked here rather than
        // inherited from the walk's implementation.
        if f0 == f1 || f1 == f2 || f0 == f2 {
            return Err(not_intact(
                EntityId::Vertex(vertex),
                "a corner's face orbit returned one face twice",
            ));
        }
        Ok(Self {
            faces: [f0, f1, f2],
        })
    }

    /// The three faces, in orbit order.
    pub(super) fn as_slice(&self) -> &[FaceKey] {
        &self.faces
    }

    /// Whether `face` is one of the corner's three.
    pub(super) fn contains(&self, face: FaceKey) -> bool {
        self.faces.contains(&face)
    }

    /// The corner's remaining support once `a` and `b` are excluded —
    /// **total**, because three distinct faces cannot all be excluded
    /// by two keys. When `a` and `b` are not both among the three the
    /// answer is still one of them, which is what the octant's scoring
    /// wants: a candidate axis, never a missing one.
    pub(super) fn third(&self, a: FaceKey, b: FaceKey) -> FaceKey {
        let [f0, f1, f2] = self.faces;
        if f0 != a && f0 != b {
            f0
        } else if f1 != a && f1 != b {
            f1
        } else {
            f2
        }
    }
}

/// One boundary station of an admitted support face: the half-edge the
/// strut is spun off, the vertex it stands on, the boundary edge that
/// leaves it, and the corner ball's foot on this face.
pub(super) struct BoundaryStation<T: Real> {
    /// The cycle half-edge whose start is [`BoundaryStation::vertex`].
    pub(super) half_edge: HalfEdgeKey,
    /// The boundary vertex the strut stands on.
    pub(super) vertex: VertexKey,
    /// The boundary edge leaving that vertex, in cycle order.
    pub(super) edge: EdgeKey,
    /// The corner ball's foot on this face — the strut's far point.
    pub(super) foot: Point3<T>,
}

/// **A support face whose ENTIRE outer cycle is requested**: every
/// boundary edge is an admitted open link, and every boundary vertex
/// is a planned corner that counts this face among its three.
///
/// The blank phase carves such a face into the shrunk face plus one
/// strip per edge, and its carving is only well-defined under exactly
/// that property. It used to re-derive the property mid-carve, one
/// refusal per clause, with nothing local to justify either — while
/// the source body it needed to read was already being mutated
/// alongside. Admission moves both refusals into the plan phase, where
/// they belong, and hands the carve the walk it checked.
pub(super) struct RequestedBoundary<T: Real> {
    face: FaceKey,
    stations: Vec<BoundaryStation<T>>,
}

impl<T: Decide + Bounds> RequestedBoundary<T> {
    /// Admit one support face of the plan.
    ///
    /// `corners` is `(vertex, its three faces, its ball centre)` for
    /// every planned corner.
    ///
    /// # Errors
    ///
    /// [`FilletError::BodyNotIntact`] when the face has no outer cycle
    /// that walks; [`FilletError::UnsupportedGeometry`] when the face
    /// is not a plane; [`FilletError::UnsupportedRunOut`] when a
    /// boundary edge is not requested, or a boundary vertex is not a
    /// planned corner of this face.
    pub(super) fn admit(
        body: &Body<T>,
        face: FaceKey,
        opens: &[ConvexOpen<'_, T>],
        corners: &[(VertexKey, &CornerFaces, Point3<T>)],
        radius: T,
    ) -> Result<Self, FilletError> {
        let normal = outward_of(body, face)
            .ok_or_else(|| unbuilt_geometry(EntityId::Face(face), CORNER_SUPPORT_NOT_PLANAR))?;
        let cycle = face_cycle(body, face).ok_or_else(|| {
            not_intact(
                EntityId::Face(face),
                "a support face has no outer cycle that walks",
            )
        })?;
        let mut stations = Vec::with_capacity(cycle.len());
        for he in cycle {
            // Every member of a returned cycle was resolved by the
            // bounded walk that returned it.
            let Some(h) = body.get_half_edge(he) else {
                unreachable!(
                    "support admission: cycle members are proven live by the bounded walk \
                     `face_cycle` just ran"
                )
            };
            // A face touched by an open link has its ENTIRE outer
            // cycle requested — each boundary vertex is a
            // fully-requested corner, so both its edges on this face
            // are. Both halves are CHECKED here rather than asserted
            // downstream.
            if !opens.iter().any(|o| o.edge() == h.edge) {
                return Err(unbuilt_run_out(
                    EntityId::Edge(h.edge),
                    "a support face's boundary carries an edge the request does not \
                     cover; run-outs at such corners are not implemented",
                ));
            }
            let Some(&(_, _, center)) = corners
                .iter()
                .find(|(v, faces, _)| *v == h.start && faces.contains(face))
            else {
                return Err(unbuilt_run_out(
                    EntityId::Vertex(h.start),
                    "a support face's boundary vertex is not a fully-requested corner of \
                     this face; run-outs at such corners are not implemented",
                ));
            };
            stations.push(BoundaryStation {
                half_edge: he,
                vertex: h.start,
                edge: h.edge,
                foot: center + normal * radius,
            });
        }
        Ok(Self { face, stations })
    }

    /// The admitted support face.
    pub(super) fn face(&self) -> FaceKey {
        self.face
    }

    /// Its boundary stations, in cycle order.
    pub(super) fn stations(&self) -> &[BoundaryStation<T>] {
        &self.stations
    }
}

#[cfg(test)]
#[allow(clippy::panic)]
mod tests {
    use topo::FaceKey;

    use super::CornerFaces;
    use crate::test_support::{L, all_links, cube};

    /// **What guards the unforgeability claim**, since nothing else
    /// can.
    ///
    /// Field privacy is the compiler's job and needs no test: no
    /// sibling module can name a field of any type here. What privacy
    /// does NOT guard is a second constructor added *inside* this
    /// module that skips the check — which is exactly how the sibling
    /// row's `Live` shipped two raw-construction sites while its own
    /// docs and its register row both said one, found by two reviewers
    /// independently rather than by anything mechanical.
    ///
    /// So the count is the claim: **four token types, four `Self`
    /// struct literals, one apiece**, each inside the door that checks.
    /// A fifth reddens this row and has to justify itself.
    ///
    /// **Its blind spot, stated:** this is a text scan of this file. A
    /// literal spelled `Self{…}` without the space, one written as
    /// `ConvexOpen { link }` by name, or a constructor built through
    /// `Default` would all escape it. It catches the accident it is
    /// aimed at — a convenience constructor added next to the door —
    /// not a determined evasion, and it cannot see whether the check a
    /// door performs is the RIGHT one. The rows below cover that half
    /// for the doors that have a decision to make.
    #[test]
    fn every_token_type_has_exactly_one_construction_site() {
        let source = include_str!("admit.rs");
        // Spelled in pieces so this row does not match itself.
        let lit = ["Self", " {"].concat();
        let returns = ["-> ", "Self", " {"].concat();
        let literals = source.matches(&lit).count() - source.matches(&returns).count();
        assert_eq!(
            literals, 4,
            "admit.rs must hold exactly one construction site per token type \
             (ConvexOpen, CornerLinks, CornerFaces, RequestedBoundary) — found {literals}"
        );
    }

    /// **The admission is what makes [`CornerFaces::third`] total**, so
    /// this row exercises both halves against a real corner: the door
    /// returns three distinct faces, and every exclusion pair over them
    /// — the three that are members and one that is not — still names a
    /// face.
    ///
    /// That is the whole reason the octant's chart pick no longer
    /// carries a run-out refusal it could not justify.
    #[test]
    fn admission_makes_the_third_support_total() {
        let body = cube(L);
        let vertex = all_links(&body)[0].start;
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
            let t = faces.third(a, b);
            assert!(t != a && t != b, "the third support excludes both");
            assert!(faces.contains(t), "and is one of the corner's own");
        }
        // A pair that is not both members: still an answer, never a
        // missing one.
        let stranger = FaceKey::default();
        assert!(faces.contains(faces.third(f0, stranger)));
    }
}
