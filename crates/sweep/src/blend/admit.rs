//! **The surgery's front door, as types.**
//!
//! [`super::surgery::blend_surgery`] admits a verdict one clause at a
//! time — this chain is a single convex plane–plane link; this corner
//! is trivalent with all three edges requested; this support face has
//! its entire outer cycle requested. Each type here is one of those
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
//! them**. [`ConvexOpen`] borrows out of the verdict, which is immutable
//! for the whole run, so it cannot go stale. [`CornerFaces`] and
//! [`RequestedBoundary`] are read off the SOURCE body and consumed
//! against a clone, so a token may describe a face the carve has since
//! split — which is what the blank phase wants, and why the walk rides
//! in the token rather than being re-derived after.

use geom_core::{Decide, Point3, Real};
use topo::{Body, EdgeKey, EntityId, FaceKey, HalfEdgeKey, VertexKey};

use super::battery::{Chain, Convexity, Link};
use super::build::{face_cycle, outward_of, vertex_faces};
use super::surgery::{
    CORNER_SUPPORT_NOT_PLANAR, not_intact, unbuilt_chain, unbuilt_corner_config, unbuilt_geometry,
    unbuilt_run_out,
};
use super::{BlendError, CornerConfig};

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
    /// [`BlendError::UnsupportedChain`] when the chain has more than
    /// one link (junction carry-through), when its supports are not
    /// plane–plane, or when it is concave.
    pub(super) fn admit(chain: &'a Chain<T>) -> Result<Self, BlendError> {
        let link = chain.first();
        if !chain.rest().is_empty() {
            return Err(unbuilt_chain(
                link.edge,
                "an open chain with more than one link needs junction \
                 carry-through, which is not implemented",
            ));
        }
        // The two RULED arms reach here with an exact cylinder band and
        // straight trimlines, and still refuse: what is missing is the
        // carve, whose terminations are OQ6's reserved run-out taxonomy
        // (tracked as #987), not the arm.
        if !link.arm.is_plane_plane() {
            return Err(unbuilt_chain(
                link.edge,
                "an open chain's supports are not plane–plane (the trivalent \
                 corner patch is the only termination built)",
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
    /// lets a corner (the fillet's octant; the chamfer's flat patch)
    /// read its orientation bit off any one of its incident links
    /// instead of testing that they agree.
    pub(super) fn convexity(&self) -> Convexity {
        self.link.convexity
    }
}

/// **The admitted open links incident to one corner vertex** — at
/// least one, every one convex, every one terminating at that vertex.
///
/// Non-emptiness is the shape: the seed link lives in its own field, so
/// [`CornerLinks::first`] returns a link rather than an `Option` and
/// there is no "a corner has no requested incident link" state left to
/// refuse. **Incidence is a check**, made by both constructors — the
/// vertex arrives as a separate argument, so it is the one thing here a
/// caller could get wrong.
pub(super) struct CornerLinks<'a, T: Real> {
    vertex: VertexKey,
    first: ConvexOpen<'a, T>,
    rest: Vec<ConvexOpen<'a, T>>,
}

impl<'a, T: Real> CornerLinks<'a, T> {
    /// A link terminates at `vertex`, or the plan's own data disagrees
    /// with itself.
    fn incident(vertex: VertexKey, link: ConvexOpen<'a, T>) -> Result<(), BlendError> {
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
    pub(super) fn seed(vertex: VertexKey, first: ConvexOpen<'a, T>) -> Result<Self, BlendError> {
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
    pub(super) fn also(&mut self, link: ConvexOpen<'a, T>) -> Result<(), BlendError> {
        Self::incident(self.vertex, link)?;
        self.rest.push(link);
        Ok(())
    }

    /// The corner vertex.
    pub(super) fn vertex(&self) -> VertexKey {
        self.vertex
    }

    /// The link that discovered this corner — always present.
    pub(super) fn first(&self) -> ConvexOpen<'a, T> {
        self.first
    }

    /// The incident links after [`CornerLinks::first`].
    pub(super) fn rest(&self) -> &[ConvexOpen<'a, T>] {
        &self.rest
    }

    /// The incident links in edge-key order — what the corner fusion
    /// walks, ordered here rather than by trusting the order the caller
    /// fed them in.
    pub(super) fn sorted(&self) -> Vec<ConvexOpen<'a, T>> {
        let mut all: Vec<ConvexOpen<'a, T>> = core::iter::once(self.first)
            .chain(self.rest.iter().copied())
            .collect();
        all.sort_by_key(ConvexOpen::edge);
        all
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
    /// [`BlendError::BodyNotIntact`] when the orbit does not walk, or
    /// when it returns a face twice;
    /// [`BlendError::UnsupportedCorner`] when the corner is not
    /// trivalent.
    pub(super) fn admit<T: Decide>(body: &Body<T>, vertex: VertexKey) -> Result<Self, BlendError> {
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
        // Distinctness is what makes `third` total, so it is checked
        // here rather than inherited from `vertex_faces`' dedup.
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

    /// Where `face` sits in orbit order — the index a corner's
    /// per-support rows (normals, feet) are keyed by, so a lookup
    /// cannot silently take another support's row.
    pub(super) fn slot_of(&self, face: FaceKey) -> Option<usize> {
        self.faces.iter().position(|f| *f == face)
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
/// leaves it, and the corner's foot on this face (the fillet's ball
/// rest; the chamfer's trimline crossing — the plan derives it, the
/// door carries it).
pub(super) struct BoundaryStation<T: Real> {
    /// The cycle half-edge whose start is [`BoundaryStation::vertex`].
    pub(super) half_edge: HalfEdgeKey,
    /// The boundary vertex the strut stands on.
    pub(super) vertex: VertexKey,
    /// The boundary edge leaving that vertex, in cycle order.
    pub(super) edge: EdgeKey,
    /// The corner's foot on this face — the strut's far point.
    pub(super) foot: Point3<T>,
}

/// **A support face whose ENTIRE outer cycle is requested**: every
/// boundary edge is an admitted open link, and every boundary vertex
/// is a planned corner that counts this face among its three.
///
/// The blank phase carves such a face into the shrunk face plus one
/// strip per edge, and that carve is well-defined only under exactly
/// this property. Admission checks it in the plan phase, before any
/// mutation, and hands the carve the walk it checked plus each
/// station's foot — so the carve reads no source geometry of its own.
pub(super) struct RequestedBoundary<T: Real> {
    face: FaceKey,
    stations: Vec<BoundaryStation<T>>,
}

// `Decide` alone: admission walks a cycle and folds a stored plane
// normal, and decides nothing that reads a bracket. The fillet seam's
// ratified compound `Decide + Bounds` bound (`geom-core/src/real.rs`,
// the `Bounds` scope rule) covers three files and this is not one.
impl<T: Decide> RequestedBoundary<T> {
    /// Admit one support face of the plan.
    ///
    /// `corners` is `(vertex, its three faces, its three FEET in those
    /// faces' orbit order)` for every planned corner. The feet are the
    /// plan's, not this door's: where a band's trimlines meet on a
    /// support is the one thing the two verbs derive differently (the
    /// ball's foot; the two trimlines' crossing), and deriving it here
    /// would put that difference in the door instead of in the plan
    /// that owns it.
    ///
    /// # Errors
    ///
    /// [`BlendError::BodyNotIntact`] when the face has no outer cycle
    /// that walks, or a planned corner does not carry a foot on this
    /// face; [`BlendError::UnsupportedGeometry`] when the face is not
    /// a plane; [`BlendError::UnsupportedRunOut`] when a boundary edge
    /// is not requested, or a boundary vertex is not a planned corner
    /// of this face.
    pub(super) fn admit(
        body: &Body<T>,
        face: FaceKey,
        opens: &[ConvexOpen<'_, T>],
        corners: &[(VertexKey, &CornerFaces, [Point3<T>; 3])],
    ) -> Result<Self, BlendError> {
        // Read once so a face that is not a plane refuses at this door
        // rather than deeper in the carve.
        outward_of(body, face)
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
            let Some((_, faces, feet)) = corners
                .iter()
                .find(|(v, faces, _)| *v == h.start && faces.contains(face))
            else {
                return Err(unbuilt_run_out(
                    EntityId::Vertex(h.start),
                    "a support face's boundary vertex is not a fully-requested corner of \
                     this face; run-outs at such corners are not implemented",
                ));
            };
            // `contains` above passed, so the slot is present; keyed
            // rather than positional so the row cannot be another
            // support's.
            let Some(slot) = faces.slot_of(face) else {
                return Err(not_intact(
                    EntityId::Face(face),
                    "a planned corner carries this face but has no foot on it",
                ));
            };
            stations.push(BoundaryStation {
                half_edge: he,
                vertex: h.start,
                edge: h.edge,
                foot: feet[slot],
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
#[allow(clippy::expect_used, clippy::panic)]
mod tests {
    use geom_core::Tol;
    use topo::FaceKey;

    use super::super::BlendError;
    use super::super::battery::{Chain, ChainClosure, Link};
    use super::{ConvexOpen, CornerFaces, CornerLinks};
    use crate::test_support::{L, all_links, cube};

    /// **What guards the unforgeability claim**, since nothing else
    /// can.
    ///
    /// Field privacy is the compiler's job. What privacy does NOT guard
    /// is a constructor added *inside* the boundary that skips the
    /// check, so this row asserts both halves of "inside":
    ///
    /// 1. **Four token types, four `Self` struct literals, one apiece**,
    ///    each inside the door that checks. A fifth reddens this row.
    /// 2. **No child module**, because `admit/…` would be inside the
    ///    privacy boundary *and* outside this file's text — the one
    ///    escape that defeats clause 1 silently.
    ///
    /// **Blind spot, stated:** clause 1 is a text scan. `Self{…}`
    /// without the space, a literal written by type name, or a route
    /// through `Default` escapes it; it catches the accident it is
    /// aimed at, not a determined evasion, and it cannot judge whether
    /// a door's check is the RIGHT one. `distinct_faces_is_pairwise`
    /// and `admission_makes_the_third_support_total` cover that half
    /// where there is a decision to get wrong.
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
        let admitted: Vec<ConvexOpen<'_, f64>> = chains
            .iter()
            .map(|c| ConvexOpen::admit(c).expect("a cube's links are convex plane–plane"))
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
    /// Its one call site cannot fire (`vertex_faces` already dedups), so
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
