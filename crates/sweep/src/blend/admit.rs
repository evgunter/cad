//! **The surgery's front door, as types.**
//!
//! [`super::surgery::blend_surgery`] admits a verdict one clause at a
//! time — this chain is a single plane–plane link whose convexity the
//! requested band carves; this corner is trivalent with all three edges
//! requested; this support face has its entire outer cycle requested.
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
//! them**. [`AdmittedOpen`] borrows out of the verdict, which is
//! immutable for the whole run, so it cannot go stale. [`CornerFaces`] and
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
use super::{BlendError, BlendKind, CornerConfig};

/// **A chain admitted through the open-chain door**: exactly one link,
/// plane–plane supports, and a convexity the requesting VERB carves.
///
/// [`AdmittedOpen::admit`] is the only way to obtain one, and it is the
/// door — the three refusals it raises are the surgery's own
/// open-chain frontier. Everything downstream that used to re-test one
/// of those three properties takes this instead.
///
/// The third clause is the verb's, not the token's, which is why the
/// token does not name a convexity: the chamfer's strip and flat
/// corner patch carry no convexity parameter and carve either side,
/// while the rolling ball's band and octant are derived convex-only.
/// A holder that needs the SIGN reads [`AdmittedOpen::convexity`].
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
    /// The open-chain door: admit a chain the battery resolved, or
    /// refuse it through the frontier vocabulary.
    ///
    /// # Errors
    ///
    /// [`BlendError::UnsupportedChain`] when the chain has more than
    /// one link (junction carry-through), when its supports are not
    /// plane–plane, or when it is concave and the band asked for is the
    /// rolling ball's.
    pub(super) fn admit(chain: &'a Chain<T>, kind: BlendKind) -> Result<Self, BlendError> {
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
        // The convexity clause is the BAND's. A ruled strip is minted
        // from the supports' own outward normals and its corner patch
        // from three trimline crossings, so it sits in a wedge of
        // material as readily as in a wedge of air; the rolling ball's
        // cylinder and octant are derived on the convex side, and a
        // concave request there would ask a half-derived construction
        // for the other one.
        // Publicly unreachable today, and kept anyway: a fillet whose
        // chain is concave refuses one door earlier, at the battery's
        // corner predicate, so no caller of `fillet_edges` reads this
        // sentence. It is the door's own clause rather than a message
        // for a user — the day a concave chain reaches here with its
        // ends admitted, this is what refuses it instead of a
        // half-derived band being built.
        if matches!(kind, BlendKind::Fillet) && !matches!(link.convexity, Convexity::Convex) {
            return Err(unbuilt_chain(
                link.edge,
                "a concave chain adds material, which the rolling ball's band \
                 does not build — not implemented",
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

    /// The link's convexity — `Convex` for every link admitted under
    /// the rolling ball's band, either sign under a ruled strip's.
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
    pub(super) fn sorted(&self) -> Vec<AdmittedOpen<'a, T>> {
        let mut all: Vec<AdmittedOpen<'a, T>> = core::iter::once(self.first)
            .chain(self.rest.iter().copied())
            .collect();
        all.sort_by_key(AdmittedOpen::edge);
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
        opens: &[AdmittedOpen<'_, T>],
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

    use super::super::battery::{Chain, ChainClosure, Link};
    use super::super::{BlendError, BlendKind};
    use super::{AdmittedOpen, CornerFaces, CornerLinks};
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
    /// **The reader is the shared one** — `test_utils::source`, in its
    /// CODE view, comments and string literals blanked. That is what
    /// lets the needles below be spelled plainly: a needle written
    /// here is a string literal, and a literal is blanked, so the scan
    /// cannot match itself. What this file used to do instead was
    /// splice the needle out of pieces at run time, which buys the
    /// same non-self-matching and no lexing at all — a construction
    /// site quoted in a doc comment or a message counted as one, and a
    /// real site commented out went on counting. The census at
    /// `crates/test-utils/tests/reader_census` is what keeps that
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
            literals, 4,
            "admit.rs must hold exactly one construction site per token type \
             (AdmittedOpen, CornerLinks, CornerFaces, RequestedBoundary) — found {literals}"
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
                AdmittedOpen::admit(c, BlendKind::Fillet)
                    .expect("a cube's links are convex plane–plane")
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
