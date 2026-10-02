//! Null-entity scaffolding: typed attributes and the null-edge lane
//! (M3 PR 1, fork F9).
//!
//! Ch. 14/15's splitting and boolean pipelines manufacture **null
//! entities** — zero-length edges and two-loop "null faces" holding
//! section-polygon copies — as mandatory mid-operation scaffolding. The
//! book encodes which side such an entity faces in half-edge slot
//! position (`he1`/`he2`) and list position (`floops`); both notes flag
//! that encoding as a mirror-bug farm. F9's ratified answer: **which
//! side a null entity faces is DATA** — typed attributes
//! ([`NullEdge`], [`NullFacePair`]), correspondence by explicit keys,
//! never index coincidence.
//!
//! # The null-edge representation (ratified shape)
//!
//! A null edge is a **distinct scaffolding representation, not a
//! relaxed certification**: its curve-arena entry is
//! [`CurveGeom::NullScaffold`] — carrying the F9 attribute and *no
//! carrier at all* — rather than a certified
//! [`geom_brep::EdgeCurve`] with a degenerate interval. The
//! forward-span certification gate (M2 PR 3: a certified interval's
//! arc length is definitely positive) is untouched; zero length is
//! representable only *by type*, and the type is transient:
//!
//! - **Tier 1 accepts** null entities (mid-op states are legal, as for
//!   every other scaffolding shape — empty loops, struts), checking
//!   only that a null-face record's loops are live and its face's own.
//! - **Tier 2 refuses** them at rest, by name
//!   ([`crate::ValidationError::NullEdgeAtRest`] /
//!   [`crate::ValidationError::NullFaceAtRest`]): a body carrying null
//!   entities is mid-surgery and never crosses an API boundary at rest.
//! - Every consumer that needs a real carrier meets the sum type
//!   [`CurveGeom`] and must handle the scaffolding variant explicitly
//!   (fail-loud at the type level; there is no accessor that silently
//!   converts a null edge into geometry).
//!
//! # Null faces
//!
//! A null face (the completed section polygon: one face, two coincident
//! loops) is an ordinary [`Face`](crate::Face) — its two loops and its
//! surface slot are real topology — so its null-ness is a typed
//! **annotation**, stored in a side table on the body
//! ([`crate::Body::null_face_pair`]) and maintained by the same
//! kill-op hygiene as provenance records (a record never outlives its
//! face). The asymmetry with edges is deliberate: an edge's null-ness
//! *replaces* its geometry (no carrier exists, by type), while a
//! face's null-ness *annotates* loop roles on an otherwise complete
//! face.
//!
//! A record's two loops are therefore its face's own: the door refuses
//! any other ([`crate::Body::set_null_face_pair`]), and an op that
//! removes a loop, or moves it off its face, drops each record naming
//! it, since that face no longer holds the two loops the record
//! describes. Every such op stays `Ok`.
//!
//! # Consumers (one line each, per the M3 doc convention)
//!
//! - [`Body::mev_null`](crate::Body::mev_null) serves `splitclassify` /
//!   `separ1`-`separ2` null-edge insertion (M3 PRs 2 and 4).
//! - [`NullFacePair`] serves `splitconnect`'s completed section
//!   polygons and `setopfinish`'s in/out copies (M3 PRs 3 and 5).

use geom_brep::EdgeCurve;
use geom_core::Real;

use crate::body::Body;
use crate::entity::{EntityId, FaceKey, LoopKey, VertexKey};
#[cfg(debug_assertions)]
use crate::euler::ArenaDelta;
use crate::euler::{EulerOpError, MevCreated, MevSite};
use crate::provenance::Provenance;

/// The F9 typed attribute of a null (zero-length) edge: **which side
/// each end faces is data**. The two end vertices are geometrically
/// coincident copies; `below_end` belongs to the below/IN side of the
/// splitting surface, `above_end` to the above/OUT side (the boolean
/// lane reads below ≙ IN, above ≙ OUT — one attribute, two readings,
/// documented at the PR 4 call sites).
///
/// Stored inside the edge's curve-arena entry
/// ([`CurveGeom::NullScaffold`]) — a null edge has no carrier, and the
/// attribute is what it has instead. Coherence contract (deliberately
/// *not* a tier-1 check: Euler surgery on a neighborhood legitimately
/// rewires half-edge starts mid-sequence, and the minting/consuming
/// ops of PRs 2–5 own the attribute's currency): `{below_end,
/// above_end}` name the edge's two end vertices as minted.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct NullEdge {
    /// The end vertex on the below (splitting) / IN (boolean) side.
    pub below_end: VertexKey,
    /// The end vertex on the above (splitting) / OUT (boolean) side.
    pub above_end: VertexKey,
}

/// The F9 typed attribute of a null face: which of its two loops plays
/// which role — never derived from `outer`-vs-ring designation or list
/// position.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum NullFacePair {
    /// Ch. 14 splitting: the two copies of a section polygon.
    Split {
        /// The loop bounding the above part.
        above_loop: LoopKey,
        /// The loop bounding the below part.
        below_loop: LoopKey,
    },
    /// Ch. 15 booleans: the IN/OUT copies of a seam polygon.
    Boolean {
        /// The loop of the IN component's copy.
        in_copy: LoopKey,
        /// The loop of the OUT component's copy.
        out_copy: LoopKey,
    },
}

impl NullFacePair {
    /// The two role loops in declaration order (above/in first).
    pub fn loops(self) -> [LoopKey; 2] {
        match self {
            Self::Split {
                above_loop,
                below_loop,
            } => [above_loop, below_loop],
            Self::Boolean { in_copy, out_copy } => [in_copy, out_copy],
        }
    }
}

/// Which side the **new** vertex of a [`Body::mev_null`] call faces —
/// the caller's declaration, recorded into the minted [`NullEdge`]
/// attribute (the old vertex takes the other side).
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum NewVertexSide {
    /// The new vertex is the above/OUT copy; the old vertex is
    /// below/IN.
    Above,
    /// The new vertex is the below/IN copy; the old vertex is
    /// above/OUT.
    Below,
}

/// A curve-arena element: a certified carrier, or the typed null-edge
/// scaffolding state (module docs — the F9/forward-span design point).
///
/// The sum lives at the arena so that *no carrier at all* is
/// representable without weakening [`EdgeCurve`]'s
/// certified-only-constructible invariant: consumers that need real
/// geometry match on this type and handle the scaffolding variant
/// explicitly (typically by refusing with a typed error — tier 2 has
/// already banned null entities from every at-rest body they should
/// legitimately see).
#[derive(Clone, Debug)]
pub enum CurveGeom<T: Real> {
    /// A certified edge carrier (the only at-rest state; D4 ¶2).
    Certified(EdgeCurve<T>),
    /// M3 null-edge scaffolding: no carrier **by type**; the payload is
    /// the F9 side attribute. Transient — tier 2 refuses it at rest.
    NullScaffold(NullEdge),
}

impl<T: Real> CurveGeom<T> {
    /// The certified carrier, if this entry is one (`None` for null
    /// scaffolding — callers decide loudly what that means for them).
    pub fn certified(&self) -> Option<&EdgeCurve<T>> {
        match self {
            Self::Certified(curve) => Some(curve),
            Self::NullScaffold(_) => None,
        }
    }

    /// The null-edge attribute, if this entry is scaffolding.
    pub fn null_scaffold(&self) -> Option<&NullEdge> {
        match self {
            Self::Certified(_) => None,
            Self::NullScaffold(attr) => Some(attr),
        }
    }
}

impl<T: geom_core::Decide> Body<T> {
    /// MEV, null-edge form — *make (null) edge, vertex*: the zero-length
    /// `lmev` idiom of ch. 14/15 (`separ1`/`separ2`), minting a new
    /// vertex **on the site's old vertex's point** (the same
    /// `PointKey` — structural coincidence, no comparison; D1 tier 3′
    /// reads two vertices on one point as structural sharing) joined by a
    /// **null edge** whose curve entry is [`CurveGeom::NullScaffold`]
    /// carrying the F9 side attribute (`new_side` declares which side
    /// the new vertex faces; the old vertex takes the other).
    ///
    /// Serves ch. 14 `splitclassify` null-edge insertion and ch. 15
    /// null-edge pairs (M3 PRs 2 and 4). Site semantics — fan split,
    /// strut (`he1 == he2`, ch. 15's dangling null edge), lone — and
    /// the surgery are exactly [`Body::mev`]'s; only the geometry lane
    /// differs, and BOTH of `mev`'s geometry gates are skipped rather
    /// than passed:
    ///
    /// - the new edge's own certification, because there is no carrier
    ///   to certify, by type (the ratified F9 shape, module docs);
    /// - the re-basing gate over a fan site's moved run
    ///   ([`Body::certify_rebased_run`]), because the new vertex's
    ///   point is the old one's, so no re-based edge's
    ///   endpoint moves and every certificate is the one it had. A
    ///   structural coincidence, not a comparison — which is why this
    ///   door needs no `Tol`.
    ///
    /// Tier 1 accepts the result; tier 2 refuses it at rest
    /// ([`crate::ValidationError::NullEdgeAtRest`]).
    ///
    /// **Pcurve rows** ([`crate::pcurves`]): the null edge has no
    /// carrier to derive its halves' rows from, so the loop it joins is
    /// held open, missing those two, and the site mint cannot walk it.
    /// The door that releases the loop mints it whole, with the rows of
    /// any half an operator added to it meanwhile: an Euler operator
    /// that rewires the loop out from under the edge — the boolean's and
    /// the splitting lane's joins, whose chord `mef`s cut the section's
    /// null halves off the faces they cross — or the edge's first
    /// description ([`Body::set_edge_curve`]); on a spline chart either
    /// leaves the face as found. A kill that releases the loop leaves
    /// those rows missing
    /// (`work/topo/a-kill-that-releases-a-loop-from-its-last-null-edge-leaves-its-gaps`).
    ///
    /// Euler vector: `(v +1, e +1, f 0, h 0, r 0, s 0)` — identical to
    /// `mev` (a null edge is an edge).
    ///
    /// **Minting order** (D9, exact — deviates from `mev`'s): no point,
    /// then **vertex**, curve entry (the F9 attribute names the new vertex,
    /// so the vertex must exist first), edge, `he_plus`, `he_minus`.
    /// Emanating rule and splice positions: as [`Body::mev`].
    ///
    /// # Precondition check order
    ///
    /// [`Body::mev`]'s site list, and nothing after it.
    ///
    /// # Errors
    ///
    /// The first failing precondition above; the body is untouched on
    /// `Err`.
    pub fn mev_null(
        &mut self,
        site: MevSite,
        new_side: NewVertexSide,
    ) -> Result<MevCreated, EulerOpError> {
        #[cfg(debug_assertions)]
        let before = self.arena_counts();

        let provenance = Provenance::MevNull { site, new_side };
        let created = match site {
            MevSite::Fan { he1, he2 } => {
                let plan = self.mev_fan_plan(he1, he2)?;
                let point = plan.p_old_key;
                self.mev_fan_execute(
                    plan,
                    point,
                    crate::euler::MevCurveMint::Null(new_side),
                    // A null edge has no carrier to derive a row from.
                    Vec::new(),
                    provenance,
                )
            }
            MevSite::Lone { r#loop } => {
                let (v, p_old_key, _) = self.mev_lone_plan(r#loop)?;
                self.mev_lone_execute(
                    r#loop,
                    v,
                    p_old_key,
                    crate::euler::MevCurveMint::Null(new_side),
                    Vec::new(),
                    provenance,
                )
            }
        };

        #[cfg(debug_assertions)]
        self.assert_euler_postcondition(
            before,
            ArenaDelta {
                half_edges: 2,
                edges: 1,
                vertices: 1,
                ..ArenaDelta::ZERO
            },
            "mev_null",
        );
        Ok(created)
    }
}

// The marker setters make no geometric decision, so they stay at the
// `Real` bound (the tiers' posture: structural bookkeeping never
// classifies).
impl<T: Real> Body<T> {
    /// Marks `face` as a null face with the given F9 loop-role
    /// attribute (replacing any existing mark — the record is data the
    /// splitting/boolean pipeline maintains as it builds the pair).
    ///
    /// Preconditions: the face resolves, both role loops resolve, they
    /// are distinct, and both are the face's own — its outer loop or
    /// one of its rings. A null face is one face's two coincident loops
    /// (module docs), so a record naming another face's loop describes
    /// none, and the door refuses it before it writes. The type cannot
    /// say it instead: the record names its loops by explicit key
    /// (F9), not by outer-vs-ring designation.
    ///
    /// Every op that removes a named loop, or moves it off its face,
    /// drops the record rather than refusing (module docs), so no op
    /// leaves a record naming a loop its face does not hold, and tier 1
    /// checks ownership too
    /// ([`crate::ValidationError::StaleNullFaceOwnership`]). Tier 2
    /// refuses marked faces at rest
    /// ([`crate::ValidationError::NullFaceAtRest`]).
    ///
    /// # Errors
    ///
    /// [`EulerOpError::StaleKey`] if the face or a role loop does not
    /// resolve; [`EulerOpError::SameLoop`] if the two role loops are
    /// one loop; [`EulerOpError::NullPairForeignLoop`] naming the first
    /// role loop, in declaration order, that is not the face's own. The
    /// body is untouched on `Err`.
    pub fn set_null_face_pair(
        &mut self,
        face: FaceKey,
        pair: NullFacePair,
    ) -> Result<(), EulerOpError> {
        let Some(face_data) = self.faces.get(face) else {
            return Err(EulerOpError::StaleKey {
                key: EntityId::Face(face),
            });
        };
        let [a, b] = pair.loops();
        for l in [a, b] {
            if !self.loops.contains_key(l) {
                return Err(EulerOpError::StaleKey {
                    key: EntityId::Loop(l),
                });
            }
        }
        if a == b {
            return Err(EulerOpError::SameLoop { r#loop: a });
        }
        for l in [a, b] {
            if face_data.outer != l && !face_data.rings.contains(&l) {
                return Err(EulerOpError::NullPairForeignLoop { face, r#loop: l });
            }
        }
        self.null_faces.insert(face, pair);
        Ok(())
    }

    /// Removes `face`'s null-face mark, returning it (or `None` if the
    /// face was not marked — total, like the accessor: clearing is the
    /// consuming pipeline's bookkeeping, not a checked surgery).
    pub fn clear_null_face_pair(&mut self, face: FaceKey) -> Option<NullFacePair> {
        self.null_faces.remove(face)
    }
}

#[cfg(test)]
#[allow(clippy::unwrap_used, clippy::expect_used, clippy::panic)]
mod tests {
    use super::*;
    use crate::fixtures::deep_snapshot;
    use crate::test_support_fixtures::declined_cube;
    use crate::validate::{ValidationError, validate, validate_closed};
    use geom_core::Tol;

    /// A null strut (`he1 == he2`) on a cube vertex: the new vertex on
    /// the old one's point, F9 attribute recorded per side, tier 1
    /// accepts, tier 2 refuses by name, and the scaffolding is killable
    /// by `kev`, which leaves the point to the vertex still on it.
    #[test]
    fn mev_null_strut_lifecycle() {
        let cube = declined_cube::<f64>(Tol::witness());
        let mut body = cube.body;
        let he = body
            .get_vertex(cube.seed.vertex)
            .unwrap()
            .emanating
            .unwrap();
        let strut = crate::MevSite::Fan { he1: he, he2: he };
        let created = body.mev_null(strut, NewVertexSide::Above).unwrap();
        let old_point = body.get_vertex(cube.seed.vertex).unwrap().point;
        assert_eq!(created.point, old_point, "the copy shares the point");
        assert_eq!(body.get_vertex(created.vertex).unwrap().point, old_point);
        // The F9 attribute names old-below / new-above.
        let attr = *body
            .get_curve_geom(created.curve)
            .unwrap()
            .null_scaffold()
            .unwrap();
        assert_eq!(
            attr,
            NullEdge {
                below_end: cube.seed.vertex,
                above_end: created.vertex,
            }
        );
        // Tier 1 accepts; tier 2 refuses by name (strut + null edge).
        assert_eq!(validate(&body), Ok(()));
        let errs = validate_closed(&body).unwrap_err();
        assert!(errs.contains(&ValidationError::NullEdgeAtRest { edge: created.edge }));
        assert!(errs.contains(&ValidationError::ScaffoldingStrutVertex {
            vertex: created.vertex
        }));
        // Provenance is the typed MevNull record.
        assert_eq!(
            body.provenance(crate::EntityId::Edge(created.edge)),
            Some(&crate::Provenance::MevNull {
                site: strut,
                new_side: NewVertexSide::Above,
            })
        );
        // Consumed by kev like any other edge; the scaffolding entry
        // dies with it and tier 2 is restored.
        let killed = body.kev(created.he_plus).unwrap();
        assert_eq!(killed.killed_point, None, "the old vertex still sits on it");
        assert!(body.get_point(old_point).is_some());
        assert_eq!(validate_closed(&body), Ok(()));
    }

    /// `Below` puts the new vertex on the below side.
    #[test]
    fn mev_null_below_side_attribute() {
        let cube = declined_cube::<f64>(Tol::witness());
        let mut body = cube.body;
        let he = body
            .get_vertex(cube.seed.vertex)
            .unwrap()
            .emanating
            .unwrap();
        let created = body
            .mev_null(
                crate::MevSite::Fan { he1: he, he2: he },
                NewVertexSide::Below,
            )
            .unwrap();
        let attr = *body
            .get_curve_geom(created.curve)
            .unwrap()
            .null_scaffold()
            .unwrap();
        assert_eq!(attr.below_end, created.vertex);
        assert_eq!(attr.above_end, cube.seed.vertex);
    }

    /// Precondition failures leave the body deeply untouched (the mev
    /// error paths, exercised through the null lane).
    #[test]
    fn mev_null_atomic_on_error() {
        let cube = declined_cube::<f64>(Tol::witness());
        let mut body = cube.body;
        let before = deep_snapshot(&body);
        let err = body
            .mev_null(
                crate::MevSite::Fan {
                    he1: crate::HalfEdgeKey::default(),
                    he2: crate::HalfEdgeKey::default(),
                },
                NewVertexSide::Above,
            )
            .unwrap_err();
        assert!(matches!(err, EulerOpError::StaleKey { .. }));
        assert_eq!(deep_snapshot(&body), before);
    }

    /// Null-face records: set/clear roundtrip, tier-2 refusal by name,
    /// structural preconditions, and a kill that leaves both of a
    /// record's loops on its face keeps the record.
    #[test]
    fn null_face_record_lifecycle() {
        let mut body = crate::fixtures::ops_holed_box(Tol::witness()).body;
        let (face, outer, ring) = ringed_face(&body);
        // Distinctness is checked.
        assert_eq!(
            body.set_null_face_pair(
                face,
                NullFacePair::Split {
                    above_loop: outer,
                    below_loop: outer,
                },
            ),
            Err(EulerOpError::SameLoop { r#loop: outer })
        );
        // A valid record: tier 1 fine, tier 2 refuses by name.
        let pair = NullFacePair::Boolean {
            in_copy: outer,
            out_copy: ring,
        };
        body.set_null_face_pair(face, pair).unwrap();
        assert_eq!(validate(&body), Ok(()));
        assert_eq!(
            validate_closed(&body),
            Err(vec![ValidationError::NullFaceAtRest { face }])
        );
        assert_eq!(body.null_face_pair(face), Some(&pair));
        // Clearing restores tier 2.
        assert_eq!(body.clear_null_face_pair(face), Some(pair));
        assert_eq!(validate_closed(&body), Ok(()));
        // `kfmrh` demoting another face's outer into the marked face
        // keeps both named loops there, so the record stands.
        body.set_null_face_pair(face, pair).unwrap();
        let shell = body.get_face(face).unwrap().shell;
        let other = body
            .faces()
            .find(|&(f, data)| f != face && data.shell == shell && data.rings.is_empty())
            .unwrap()
            .0;
        body.kfmrh(face, other).unwrap();
        assert_eq!(body.null_face_pair(face), Some(&pair));
        assert_eq!(validate(&body), Ok(()));
    }

    /// Pass 13's loop-key resolution (review flag c): a null-face
    /// record naming a killed loop is reported typed
    /// (`StaleNullFaceLoop`), not passed silently. The stale record is
    /// constructed through the crate-internal map (the public setter
    /// refuses dead keys — asserted), modeling an op that killed a
    /// named loop after the record was minted.
    #[test]
    fn stale_null_face_loop_record_reported() {
        let cube = declined_cube::<f64>(Tol::witness());
        let mut body = cube.body;
        let (f1, f2) = (cube.mefs[0].face, cube.mefs[1].face);
        let outer1 = body.get_face(f1).unwrap().outer;
        let outer2 = body.get_face(f2).unwrap().outer;
        // Kill f2's outer loop legitimately (kef of one of its edges:
        // the argument half's face and loop die, the mate's survive).
        let crate::LoopBoundary::Cycle { first } = body.get_loop(outer2).unwrap().boundary else {
            panic!("cube outer loops are cycles");
        };
        let killed = body.kef(first).unwrap();
        assert_eq!(killed.killed_loop, outer2);
        assert_eq!(validate(&body), Ok(()));
        // The public door refuses the dead key...
        assert_eq!(
            body.set_null_face_pair(
                f1,
                NullFacePair::Split {
                    above_loop: outer2,
                    below_loop: outer1,
                },
            ),
            Err(EulerOpError::StaleKey {
                key: EntityId::Loop(outer2),
            })
        );
        // ...so the stale state is built directly on the arena map.
        body.null_faces.insert(
            f1,
            NullFacePair::Split {
                above_loop: outer2,
                below_loop: outer1,
            },
        );
        assert_eq!(
            validate(&body),
            Err(vec![ValidationError::StaleNullFaceLoop {
                face: f1,
                named_loop: outer2,
            }])
        );
    }

    /// Pass 13's ownership check: a record naming a live loop its face
    /// does not hold is reported typed (`StaleNullFaceOwnership`),
    /// naming each such loop and not the face's own: another face's
    /// outer loop, and another face's ring. The door refuses the record
    /// ([`the_door_refuses_a_record_naming_a_loop_not_the_faces_own`]),
    /// so it is built on the crate-internal map, modelling an op that
    /// moved a named loop off its face and kept the record.
    #[test]
    fn stale_null_face_ownership_reported() {
        let mut body = crate::fixtures::ops_holed_box(Tol::witness()).body;
        let (face, outer, ring) = ringed_face(&body);
        let (other, other_outer) = body
            .faces()
            .find(|&(f, _)| f != face)
            .map(|(f, data)| (f, data.outer))
            .unwrap();
        for (marked, pair, named_loop) in [
            (
                face,
                NullFacePair::Boolean {
                    in_copy: outer,
                    out_copy: other_outer,
                },
                other_outer,
            ),
            (
                other,
                NullFacePair::Split {
                    above_loop: other_outer,
                    below_loop: ring,
                },
                ring,
            ),
        ] {
            let mut body = body.clone();
            body.null_faces.insert(marked, pair);
            assert_eq!(
                validate(&body),
                Err(vec![ValidationError::StaleNullFaceOwnership {
                    face: marked,
                    named_loop,
                }]),
                "{pair:?} on {marked:?}"
            );
        }
        body.null_faces.insert(
            face,
            NullFacePair::Split {
                above_loop: outer,
                below_loop: ring,
            },
        );
        assert_eq!(validate(&body), Ok(()), "the face's own two loops");
    }

    /// The first face with exactly one ring, its outer loop and the
    /// ring.
    fn ringed_face(body: &Body<f64>) -> (FaceKey, LoopKey, LoopKey) {
        body.faces()
            .find_map(|(f, data)| match data.rings[..] {
                [ring] => Some((f, data.outer, ring)),
                _ => None,
            })
            .expect("the fixture has a face with one ring")
    }

    /// The `first` half-edge of a cycle loop.
    fn first_of(body: &Body<f64>, l: LoopKey) -> crate::entity::HalfEdgeKey {
        match body.get_loop(l).unwrap().boundary {
            crate::LoopBoundary::Cycle { first } => first,
            crate::LoopBoundary::Empty { .. } => panic!("{l:?} is a cycle"),
        }
    }

    /// **A kill drops each record naming the loop it removes, and stays
    /// `Ok`.** A null face is one face's two coincident loops (module
    /// docs), so a record whose loop dies no longer describes one. Here
    /// the declined cube's seventh half-edge is killed, a third loop's
    /// face naming its loop.
    ///
    /// `kef`'s dying loop is its face's only loop, so with the door's
    /// ownership check no reachable record but that face's own names
    /// it, and the face's own dies with the face. This row and the
    /// `kvfs` and `kfmrh` ones therefore build the record on the
    /// crate-internal map, a state no public door builds and tier 1
    /// refuses ([`stale_null_face_ownership_reported`]), and stay as
    /// defence in depth: the drop reads every face's record, not only
    /// the dying loop's face's.
    #[test]
    fn kef_drops_each_record_naming_the_loop_it_removes() {
        let mut body = declined_cube::<f64>(Tol::witness()).body;
        let he = body.half_edges().nth(6).unwrap().0;
        let l1 = body.get_half_edge(he).unwrap().parent_loop;
        let l2 = body
            .get_half_edge(body.mate(he).unwrap())
            .unwrap()
            .parent_loop;
        let (third, marked) = body
            .loops()
            .find(|&(l, _)| l != l1 && l != l2)
            .map(|(l, data)| (l, data.face))
            .unwrap();
        body.null_faces.insert(
            marked,
            NullFacePair::Split {
                above_loop: third,
                below_loop: l1,
            },
        );
        assert_eq!(body.kef(he).unwrap().killed_loop, l1);
        assert_eq!(body.null_face_pair(marked), None, "kef drops the record");
        assert_eq!(validate(&body), Ok(()), "kef");
    }

    /// [`kef_drops_each_record_naming_the_loop_it_removes`]'s `kvfs`
    /// row: a segment's face marked naming a lone solid's loop. The
    /// lone loop is its face's only loop, so this is defence in depth on
    /// a state only the crate-internal map builds, as the `kef` row is.
    #[test]
    fn kvfs_drops_each_record_naming_the_loop_it_removes() {
        let tol = Tol::witness();
        let mut body = Body::<f64>::new();
        let p = |x: f64| geom_core::Point3::new(x, 0.0, 0.0);
        let seg = body.mvfs(p(0.0), true).unwrap();
        let site = MevSite::Lone { r#loop: seg.r#loop };
        body.mev_line(site, p(1.0), tol).unwrap();
        let lone = body.mvfs(p(5.0), true).unwrap();
        body.null_faces.insert(
            seg.face,
            NullFacePair::Split {
                above_loop: seg.r#loop,
                below_loop: lone.r#loop,
            },
        );
        body.kvfs(lone.solid).unwrap();
        assert_eq!(body.null_face_pair(seg.face), None, "kvfs drops the record");
        assert_eq!(validate(&body), Ok(()), "kvfs");
    }

    /// [`kef_drops_each_record_naming_the_loop_it_removes`]'s `mekr`
    /// row: the holed box's ringed face marked with its own outer and
    /// ring, through the door, the ring joined into the outer.
    #[test]
    fn mekr_drops_each_record_naming_the_loop_it_removes() {
        let tol = Tol::witness();
        let mut body = crate::fixtures::ops_holed_box(tol).body;
        let (face, outer, ring) = ringed_face(&body);
        body.set_null_face_pair(
            face,
            NullFacePair::Split {
                above_loop: outer,
                below_loop: ring,
            },
        )
        .unwrap();
        let site = crate::MekrSite::Cycles {
            target: first_of(&body, outer),
            ring: first_of(&body, ring),
        };
        body.mekr_chord(site, tol).unwrap();
        assert_eq!(body.null_face_pair(face), None, "mekr drops the record");
        assert_eq!(validate(&body), Ok(()), "mekr");
    }

    /// The holed box with its ringed face marked by its own two loops,
    /// through the door: the face, and its ring.
    fn marked_holed_box() -> (Body<f64>, FaceKey, LoopKey) {
        let mut body = crate::fixtures::ops_holed_box(Tol::witness()).body;
        let (face, outer, ring) = ringed_face(&body);
        let pair = NullFacePair::Boolean {
            in_copy: ring,
            out_copy: outer,
        };
        body.set_null_face_pair(face, pair).unwrap();
        (body, face, ring)
    }

    /// **An op that moves a loop off its face drops the record naming
    /// it.** The marked face no longer has that loop, so the record no
    /// longer describes one face's two coincident loops: here `mfkrh`
    /// promotes the holed box's ring to a face of its own.
    #[test]
    fn mfkrh_drops_the_record_naming_the_ring_it_promotes() {
        let (mut body, face, ring) = marked_holed_box();
        body.mfkrh_plug(ring, true).unwrap();
        assert_eq!(body.null_face_pair(face), None, "mfkrh drops the record");
        assert_eq!(validate(&body), Ok(()), "mfkrh");
    }

    /// [`mfkrh_drops_the_record_naming_the_ring_it_promotes`]'s
    /// `ring_move` row: the ring moved to another face of its shell.
    #[test]
    fn ring_move_drops_the_record_naming_the_ring_it_moves() {
        let (mut body, face, ring) = marked_holed_box();
        let shell = body.get_face(face).unwrap().shell;
        let to = body
            .faces()
            .find(|&(f, data)| f != face && data.shell == shell)
            .unwrap()
            .0;
        body.ring_move(ring, to).unwrap();
        assert_eq!(
            body.null_face_pair(face),
            None,
            "ring_move drops the record"
        );
        assert_eq!(validate(&body), Ok(()), "ring_move");
    }

    /// **A drop is keyed on the loop, not the face.** A ring of the
    /// marked face that the record does not name dies by `mekr`, or
    /// leaves by `mfkrh` or `ring_move`, and the record stands: its two
    /// loops are still the face's own. From the PR 3618 review's probe.
    #[test]
    fn ops_on_an_unnamed_ring_of_the_marked_face_keep_the_record() {
        let tol = Tol::witness();
        let crate::fixtures::OpsTwoRingFace {
            body: base,
            face,
            outer,
            hole,
            tip,
        } = crate::fixtures::ops_two_ring_face(tol);
        let pair = NullFacePair::Split {
            above_loop: outer,
            below_loop: hole,
        };
        let mut marked = base;
        marked.set_null_face_pair(face, pair).unwrap();
        let mut body = marked.clone();
        let site = crate::MekrSite::EmptyRing {
            target: first_of(&body, outer),
            ring: tip,
        };
        body.mekr_chord(site, tol).unwrap();
        assert_eq!(body.null_face_pair(face), Some(&pair), "mekr keeps");
        assert_eq!(validate(&body), Ok(()), "mekr");
        let mut body = marked.clone();
        body.mfkrh_plug(tip, true).unwrap();
        assert_eq!(body.null_face_pair(face), Some(&pair), "mfkrh keeps");
        assert_eq!(validate(&body), Ok(()), "mfkrh");
        let mut body = marked;
        let shell = body.get_face(face).unwrap().shell;
        let to = body
            .faces()
            .find(|&(f, data)| f != face && data.shell == shell)
            .unwrap()
            .0;
        body.ring_move(tip, to).unwrap();
        assert_eq!(body.null_face_pair(face), Some(&pair), "ring_move keeps");
        assert_eq!(validate(&body), Ok(()), "ring_move");
    }

    /// **A `ring_move` within one face stays a deep no-op**, the record
    /// naming the ring included: the ring does not leave its face. From
    /// the PR 3618 review's probe.
    #[test]
    fn ring_move_within_one_face_is_a_deep_no_op() {
        let crate::fixtures::OpsTwoRingFace {
            mut body,
            face,
            outer,
            hole,
            ..
        } = crate::fixtures::ops_two_ring_face(Tol::witness());
        let pair = NullFacePair::Split {
            above_loop: outer,
            below_loop: hole,
        };
        body.set_null_face_pair(face, pair).unwrap();
        let before = deep_snapshot(&body);
        body.ring_move(hole, face).unwrap();
        assert_eq!(deep_snapshot(&body), before, "ring_move within one face");
    }

    /// [`mfkrh_drops_the_record_naming_the_ring_it_promotes`]'s `kfmrh`
    /// row: `f2`'s outer, re-homed onto `f1`, named by a third face's
    /// record built on the crate-internal map (a ring-free `f2` holds
    /// one loop, so no record the door accepts names it). Defence in
    /// depth on a state no public door builds, as the `kef` row is.
    #[test]
    fn kfmrh_drops_the_record_naming_the_loop_it_demotes() {
        let cube = declined_cube::<f64>(Tol::witness());
        let mut body = cube.body;
        let (f1, f2) = (cube.mefs[0].face, cube.mefs[1].face);
        let outer2 = body.get_face(f2).unwrap().outer;
        let (third, third_outer) = body
            .faces()
            .find(|&(f, _)| f != f1 && f != f2)
            .map(|(f, data)| (f, data.outer))
            .unwrap();
        body.null_faces.insert(
            third,
            NullFacePair::Split {
                above_loop: third_outer,
                below_loop: outer2,
            },
        );
        body.kfmrh(f1, f2).unwrap();
        assert_eq!(body.null_face_pair(third), None, "kfmrh drops the record");
        assert_eq!(validate(&body), Ok(()), "kfmrh");
    }

    /// **The door refuses a record naming a loop that is not the face's
    /// own**, naming the first such role loop in declaration order, and
    /// leaves the body deep-unchanged: another face's outer in either
    /// role, and the ringed face's loops named on a face that holds
    /// neither.
    #[test]
    fn the_door_refuses_a_record_naming_a_loop_not_the_faces_own() {
        let mut body = crate::fixtures::ops_holed_box(Tol::witness()).body;
        let (face, outer, ring) = ringed_face(&body);
        let (other, other_outer) = body
            .faces()
            .find(|&(f, _)| f != face)
            .map(|(f, data)| (f, data.outer))
            .unwrap();
        let split = |above_loop, below_loop| NullFacePair::Split {
            above_loop,
            below_loop,
        };
        let boolean = |in_copy, out_copy| NullFacePair::Boolean { in_copy, out_copy };
        let cases = [
            (face, split(other_outer, ring), other_outer),
            (face, boolean(outer, other_outer), other_outer),
            (other, split(other_outer, ring), ring),
            (other, boolean(outer, ring), outer),
        ];
        for (marked, pair, named) in cases {
            let expected = EulerOpError::NullPairForeignLoop {
                face: marked,
                r#loop: named,
            };
            crate::fixtures::assert_err_deep_unchanged(&mut body, &expected, |b| {
                b.set_null_face_pair(marked, pair).unwrap_err()
            });
        }
    }

    /// **A refusal inside the re-mint leaves the body untouched.** A
    /// null strut on a minted cylinder wall, described by the wall's
    /// own circle through its vertex: the description re-mints the
    /// wall, and a half-edge of the loop it walks whose curve entry is
    /// gone is tier 1's corruption, refused typed before the door
    /// writes — the null edge keeps its scaffolding entry and every
    /// row stays where it was.
    #[test]
    fn a_refused_null_description_leaves_the_body_untouched() {
        use crate::test_support_fixtures::{CylFrame, cyl_wall_sheet};
        let tol = Tol::witness();
        let mut body = Body::<f64>::new();
        let frame = CylFrame::canonical(1.0);
        let face = cyl_wall_sheet(&mut body, frame, None, (0.2, 1.4), (0.0, 1.0), tol);
        let outer = body.get_face(face).unwrap().outer;
        let crate::LoopBoundary::Cycle { first } = body.get_loop(outer).unwrap().boundary else {
            panic!("the wall is bounded by a cycle")
        };
        let v = body.get_half_edge(first).unwrap().start;
        let p = *body.get_point(body.get_vertex(v).unwrap().point).unwrap();
        let null = body
            .mev_null(
                crate::MevSite::Fan {
                    he1: first,
                    he2: first,
                },
                NewVertexSide::Above,
            )
            .unwrap();
        let t0 = p.y.atan2(p.x);
        let circle = geom::Curve3::Circle {
            center: frame.origin + frame.axis * p.z,
            axis: frame.axis,
            radius: frame.radius,
            u_ref: frame.u_ref,
        };
        let spec = || {
            geom_brep::EdgeCurveSpec::arc_of_circle(circle.clone(), t0, t0 + core::f64::consts::TAU)
                .unwrap()
        };
        // The control: on the intact body the description mints.
        let mut control = body.clone();
        control.set_edge_curve(null.edge, spec(), tol).unwrap();
        assert!(control.pcurve(null.he_plus).is_some());
        assert!(control.pcurve(null.he_minus).is_some());

        let torn = body
            .get_edge(body.get_half_edge(first).unwrap().edge)
            .unwrap()
            .curve;
        body.curves.remove(torn).unwrap();
        let before = deep_snapshot(&body);
        let err = body.set_edge_curve(null.edge, spec(), tol).unwrap_err();
        assert_eq!(
            err,
            EulerOpError::PcurveMint {
                face,
                refusal: crate::pcurves::SiteRowRefusal::Corrupt,
            }
        );
        assert_eq!(
            deep_snapshot(&body),
            before,
            "the body is untouched, every pcurve row included"
        );
    }

    /// A null edge between the two faces of the minted wall sheet — the
    /// wall and its seed face, both on the cylinder — at a vertex they
    /// share, with a site-minted spur on the seed face at another
    /// vertex, and the description that closes it: the cylinder's
    /// circle through that vertex, once round.
    struct TwoFaced {
        body: Body<f64>,
        wall: FaceKey,
        seed: FaceKey,
        null: crate::MevCreated,
        spur: crate::MevCreated,
        spec: geom_brep::EdgeCurveSpec<f64>,
    }

    fn two_faced_null_edge() -> TwoFaced {
        use crate::test_support_fixtures::{CylFrame, cyl_wall_sheet};
        let tol = Tol::witness();
        let mut body = Body::<f64>::new();
        let frame = CylFrame::canonical(1.0);
        let wall = cyl_wall_sheet(&mut body, frame, None, (0.2, 1.4), (0.0, 1.0), tol);
        let seed = body.faces().map(|(k, _)| k).find(|&k| k != wall).unwrap();
        let cycle = |body: &Body<f64>, face: FaceKey| {
            let outer = body.get_face(face).unwrap().outer;
            let crate::LoopBoundary::Cycle { first } = body.get_loop(outer).unwrap().boundary
            else {
                panic!("the sheet's faces are bounded by cycles")
            };
            body.loop_cycle(first).unwrap()
        };
        let wall_leaving = cycle(&body, wall)[0];
        let v = body.get_half_edge(wall_leaving).unwrap().start;
        let start = |h: &crate::entity::HalfEdgeKey| body.get_half_edge(*h).unwrap().start;
        let seed_cycle = cycle(&body, seed);
        let seed_leaving = *seed_cycle.iter().find(|h| start(h) == v).unwrap();
        let elsewhere = *seed_cycle.iter().find(|h| start(h) != v).unwrap();
        let w = start(&elsewhere);
        let pw = *body.get_point(body.get_vertex(w).unwrap().point).unwrap();
        let rise = if pw.z > 0.5 { -0.3 } else { 0.3 };
        let spur = body
            .mev_line(
                crate::MevSite::Fan {
                    he1: elsewhere,
                    he2: elsewhere,
                },
                pw + geom_core::Vec3::new(0.0, 0.0, rise),
                tol,
            )
            .unwrap();
        let null = body
            .mev_null(
                crate::MevSite::Fan {
                    he1: wall_leaving,
                    he2: seed_leaving,
                },
                NewVertexSide::Above,
            )
            .unwrap();
        let p = *body.get_point(body.get_vertex(v).unwrap().point).unwrap();
        let t0 = p.y.atan2(p.x);
        let circle = geom::Curve3::Circle {
            center: frame.origin + frame.axis * p.z,
            axis: frame.axis,
            radius: frame.radius,
            u_ref: frame.u_ref,
        };
        let spec = geom_brep::EdgeCurveSpec::arc_of_circle(circle, t0, t0 + core::f64::consts::TAU)
            .unwrap();
        TwoFaced {
            body,
            wall,
            seed,
            null,
            spur,
            spec,
        }
    }

    fn face_of(body: &Body<f64>, he: crate::entity::HalfEdgeKey) -> FaceKey {
        body.get_loop(body.get_half_edge(he).unwrap().parent_loop)
            .unwrap()
            .face
    }

    /// Every stored row with its interval, image and certificate, sorted.
    fn rows_deep(body: &Body<f64>) -> Vec<String> {
        let mut out: Vec<String> = body
            .pcurves()
            .map(|(he, c)| {
                format!(
                    "{he:?} {:?} {:?} {:?}",
                    c.params(),
                    c.pcurve(),
                    c.certificate()
                )
            })
            .collect();
        out.sort();
        out
    }

    fn findings(body: &Body<f64>) -> Vec<crate::pcurves::PcurveMintError> {
        let band = geom_core::Band::linear(Tol::witness()).unwrap();
        crate::pcurves::validate_pcurves(body, band)
    }

    /// **A null edge between two faces completes both.** Each half is on
    /// its own face's chart, so the two rows differ, and each lands on
    /// its own key: the faces leave complete, with the minting pass's
    /// rows. (Adopted from the review's probe P3.)
    #[test]
    fn a_two_faced_null_edges_description_mints_both_faces() {
        let TwoFaced {
            mut body,
            wall,
            seed,
            null,
            spec,
            ..
        } = two_faced_null_edge();
        assert_eq!(face_of(&body, null.he_plus), wall);
        assert_eq!(face_of(&body, null.he_minus), seed);
        let missing = |he| crate::pcurves::PcurveMintError::MissingCache { half_edge: he };
        let mut want = vec![missing(null.he_plus), missing(null.he_minus)];
        let mut found = findings(&body);
        let order = |e: &crate::pcurves::PcurveMintError| format!("{e:?}");
        want.sort_by_key(order);
        found.sort_by_key(order);
        assert_eq!(
            found, want,
            "mev_null leaves each face missing its half's row"
        );
        body.set_edge_curve(null.edge, spec, Tol::witness())
            .unwrap();
        assert_eq!(findings(&body), vec![], "both faces are complete");
        let described = rows_deep(&body);
        crate::pcurves::mint_pcurves(&mut body, Tol::witness()).unwrap();
        assert_eq!(rows_deep(&body), described, "the rows are the pass's");
    }

    /// **A refusal on the second face leaves both untouched.** The wall
    /// is planned first and plans fine; the seed face's spur has lost
    /// its curve entry, so its re-mint refuses — and the wall's plan is
    /// not written either.
    #[test]
    fn a_refusal_on_the_second_face_leaves_the_body_untouched() {
        let TwoFaced {
            mut body,
            wall,
            seed,
            null,
            spur,
            spec,
        } = two_faced_null_edge();
        assert_eq!(
            face_of(&body, null.he_plus),
            wall,
            "the wall is planned first"
        );
        let torn = body.get_edge(spur.edge).unwrap().curve;
        body.curves.remove(torn).unwrap();
        let before = deep_snapshot(&body);
        let err = body
            .set_edge_curve(null.edge, spec, Tol::witness())
            .unwrap_err();
        assert_eq!(
            err,
            EulerOpError::PcurveMint {
                face: seed,
                refusal: crate::pcurves::SiteRowRefusal::Corrupt,
            }
        );
        assert_eq!(
            deep_snapshot(&body),
            before,
            "the body is untouched, every pcurve row included"
        );
    }

    /// The vertex `split_edge` puts on the wall's rim at height `z`,
    /// where the ruling `u` meets it. The split carries the rows.
    fn rim_vertex_on_the_ruling(body: &mut Body<f64>, u: f64, z: f64) -> crate::entity::VertexKey {
        use crate::test_support_fixtures::CylFrame;
        let want: geom_core::Point3<f64> = CylFrame::canonical(1.0).at(u, z);
        let (rim, t) = body
            .edges()
            .find_map(|(e, data)| {
                let c = body.get_curve_geom(data.curve)?.certified()?;
                let geom::Curve3::Circle {
                    center,
                    axis,
                    u_ref,
                    ..
                } = *c.carrier()
                else {
                    return None;
                };
                if center.z != z {
                    return None;
                }
                let d = want - center;
                let angle = d.dot(axis.cross(u_ref)).atan2(d.dot(u_ref));
                let (t0, t1) = c.params();
                let (lo, hi) = (t0.min(t1), t0.max(t1));
                let tau = core::f64::consts::TAU;
                let t = angle + tau * ((lo - angle) / tau).ceil();
                (t < hi).then_some((e, t))
            })
            .unwrap();
        body.split_edge(rim, t, Tol::witness()).unwrap().vertex
    }

    /// The minted wall sheet with both rims split on the ruling
    /// `u = 0.8`, a null strut hung at a corner — so a null edge holds
    /// the wall's one loop open and its only missing rows are the
    /// strut's two — and the `mef` along the ruling that cuts the wall
    /// in two, the strut on one side.
    struct RulingCut {
        body: Body<f64>,
        wall: FaceKey,
        null: crate::MevCreated,
        site: crate::MefSite,
        chord: geom_brep::EdgeCurveSpec<f64>,
    }

    fn ruling_cut() -> RulingCut {
        use crate::test_support_fixtures::{CylFrame, cyl_wall_sheet};
        let tol = Tol::witness();
        let mut body = Body::<f64>::new();
        let wall = cyl_wall_sheet(
            &mut body,
            CylFrame::canonical(1.0),
            None,
            (0.2, 1.4),
            (0.0, 1.0),
            tol,
        );
        let bottom = rim_vertex_on_the_ruling(&mut body, 0.8, 0.0);
        let top = rim_vertex_on_the_ruling(&mut body, 0.8, 1.0);
        let cycle = |body: &Body<f64>| {
            let outer = body.get_face(wall).unwrap().outer;
            let crate::LoopBoundary::Cycle { first } = body.get_loop(outer).unwrap().boundary
            else {
                panic!("the wall is bounded by a cycle")
            };
            body.loop_cycle(first).unwrap()
        };
        let start = |body: &Body<f64>, he: crate::entity::HalfEdgeKey| {
            body.get_half_edge(he).unwrap().start
        };
        let corner = *cycle(&body)
            .iter()
            .find(|&&he| ![bottom, top].contains(&start(&body, he)))
            .unwrap();
        let null = body
            .mev_null(
                crate::MevSite::Fan {
                    he1: corner,
                    he2: corner,
                },
                NewVertexSide::Above,
            )
            .unwrap();
        let leaving = |v| {
            *cycle(&body)
                .iter()
                .find(|&&he| start(&body, he) == v)
                .unwrap()
        };
        let site = crate::MefSite::Chords {
            he1: leaving(bottom),
            he2: leaving(top),
        };
        let point = |v: crate::entity::VertexKey| {
            *body.get_point(body.get_vertex(v).unwrap().point).unwrap()
        };
        let chord = geom_brep::EdgeCurveSpec::line_between(point(bottom), point(top));
        RulingCut {
            body,
            wall,
            null,
            site,
            chord,
        }
    }

    /// The half-edges `validate_pcurves` names as missing a row, sorted.
    fn missing_rows(body: &Body<f64>) -> Vec<crate::entity::HalfEdgeKey> {
        let mut out: Vec<_> = findings(body)
            .into_iter()
            .map(|f| match f {
                crate::pcurves::PcurveMintError::MissingCache { half_edge } => half_edge,
                other => panic!("only missing rows are reported, got {other:?}"),
            })
            .collect();
        out.sort();
        out
    }

    /// Every stored row of `face`'s loops, sorted.
    fn face_rows(body: &Body<f64>, face: FaceKey) -> Vec<String> {
        let f = body.get_face(face).unwrap();
        let mut out: Vec<String> = core::iter::once(f.outer)
            .chain(f.rings.iter().copied())
            .filter_map(|lk| match body.get_loop(lk).unwrap().boundary {
                crate::LoopBoundary::Cycle { first } => Some(body.loop_cycle(first).unwrap()),
                crate::LoopBoundary::Empty { .. } => None,
            })
            .flatten()
            .filter_map(|he| {
                let c = body.pcurve(he)?;
                Some(format!(
                    "{he:?} {:?} {:?} {:?}",
                    c.params(),
                    c.pcurve(),
                    c.certificate()
                ))
            })
            .collect();
        out.sort();
        out
    }

    /// **An operator that cuts a null edge off a loop mints the loop it
    /// releases.** The `mef` along the ruling leaves the strut on one
    /// piece and the other piece's loop running through no null edge:
    /// that piece leaves complete, with the minting pass's rows byte for
    /// byte, and the piece the strut still holds open misses exactly the
    /// strut's two rows and the chord half on its loop. Before the site
    /// mint read a face held open by a null edge, the `mef` left the
    /// wall as found and both pieces missed their chord halves' rows.
    #[test]
    fn a_mef_that_cuts_a_null_strut_off_mints_the_piece_it_releases() {
        let RulingCut {
            mut body,
            wall,
            null,
            site,
            chord,
        } = ruling_cut();
        let mut held_open = vec![null.he_plus, null.he_minus];
        held_open.sort();
        assert_eq!(
            missing_rows(&body),
            held_open,
            "the null strut holds the wall open"
        );
        let made = body
            .mef(site, chord, crate::FaceSurface::Inherit, Tol::witness())
            .unwrap();
        let held = face_of(&body, null.he_plus);
        let released = if held == wall { made.face } else { wall };
        let chord_on_held = if face_of(&body, made.he_plus) == held {
            made.he_plus
        } else {
            made.he_minus
        };
        held_open.push(chord_on_held);
        held_open.sort();
        assert_eq!(
            missing_rows(&body),
            held_open,
            "only the loop the strut holds open misses rows"
        );
        let released_rows = face_rows(&body, released);
        let mut pass = body.clone();
        crate::pcurves::mint_pcurves_of(&mut pass, &[released], Tol::witness()).unwrap();
        assert_eq!(
            released_rows,
            face_rows(&pass, released),
            "the released piece's rows are the minting pass's, byte for byte"
        );
    }

    /// **A half of the cut wall that does not resolve refuses typed,
    /// and leaves the body untouched.** The same cut, with the curve
    /// entry of a half-edge gone — tier 1's corruption — on either side
    /// of the chord: on the side the cut releases, whose loop the site
    /// mint walks, and on the side the strut still holds open, whose
    /// loop it never walks but reads for the null edge. Either way the
    /// `mef` refuses naming the wall, before it mutates.
    #[test]
    fn a_torn_half_on_either_side_of_the_cut_refuses_before_the_mef_mutates() {
        for side in ["released", "held"] {
            let RulingCut {
                mut body,
                wall,
                null,
                site,
                chord,
            } = ruling_cut();
            let mut control = body.clone();
            let made = control
                .mef(
                    site,
                    chord.clone(),
                    crate::FaceSurface::Inherit,
                    Tol::witness(),
                )
                .unwrap();
            let held = face_of(&control, null.he_plus);
            let released = if held == wall { made.face } else { wall };
            let piece = if side == "released" { released } else { held };
            let outer = control.get_face(piece).unwrap().outer;
            let crate::LoopBoundary::Cycle { first } = control.get_loop(outer).unwrap().boundary
            else {
                panic!("the {side} piece is bounded by a cycle")
            };
            let scaffolding = [made.he_plus, made.he_minus, null.he_plus, null.he_minus];
            let torn_half = *control
                .loop_cycle(first)
                .unwrap()
                .iter()
                .find(|he| !scaffolding.contains(he))
                .unwrap();
            let torn = body
                .get_edge(body.get_half_edge(torn_half).unwrap().edge)
                .unwrap()
                .curve;
            body.curves.remove(torn).unwrap();
            let before = deep_snapshot(&body);
            let err = body
                .mef(site, chord, crate::FaceSurface::Inherit, Tol::witness())
                .unwrap_err();
            assert_eq!(
                err,
                EulerOpError::PcurveMint {
                    face: wall,
                    refusal: crate::pcurves::SiteRowRefusal::Corrupt,
                },
                "torn on the {side} side"
            );
            assert_eq!(
                deep_snapshot(&body),
                before,
                "{side}: the body is untouched, every pcurve row included"
            );
        }
    }

    /// The minted wall sheet with both rims split on the ruling
    /// `u = 0.8`, a two-half ring standing on the ruling `u = 0.5` — a
    /// strut up it from the bottom rim, a second strut from its tip, the
    /// first cut away with `kemr` — and a null strut hung at a corner
    /// of the run the `mef` along `u = 0.8` moves onto its new face. So
    /// that cut leaves the strut on the new face and the ring on the
    /// wall. Returns the cut and the ring.
    fn ringed_ruling_cut() -> (RulingCut, crate::entity::LoopKey) {
        use crate::test_support_fixtures::{CylFrame, cyl_wall_sheet};
        let tol = Tol::witness();
        let frame = CylFrame::canonical(1.0);
        let mut body = Body::<f64>::new();
        let wall = cyl_wall_sheet(&mut body, frame, None, (0.2, 1.4), (0.0, 1.0), tol);
        let bottom = rim_vertex_on_the_ruling(&mut body, 0.8, 0.0);
        let top = rim_vertex_on_the_ruling(&mut body, 0.8, 1.0);
        let foot = rim_vertex_on_the_ruling(&mut body, 0.5, 0.0);
        let cycle = |body: &Body<f64>| {
            let outer = body.get_face(wall).unwrap().outer;
            let crate::LoopBoundary::Cycle { first } = body.get_loop(outer).unwrap().boundary
            else {
                panic!("the wall is bounded by a cycle")
            };
            body.loop_cycle(first).unwrap()
        };
        let leaving = |body: &Body<f64>, v| {
            *cycle(body)
                .iter()
                .find(|&&he| body.get_half_edge(he).unwrap().start == v)
                .unwrap()
        };
        let from_foot = leaving(&body, foot);
        let lower = body
            .mev_line(
                crate::MevSite::Fan {
                    he1: from_foot,
                    he2: from_foot,
                },
                frame.at(0.5, 0.3),
                tol,
            )
            .unwrap();
        body.mev_line(
            crate::MevSite::Fan {
                he1: lower.he_minus,
                he2: lower.he_minus,
            },
            frame.at(0.5, 0.6),
            tol,
        )
        .unwrap();
        let ring = body.kemr(lower.he_plus, lower.he_minus).unwrap().ring;
        assert_eq!(findings(&body), vec![], "the ringed wall is complete");
        let (he1, he2) = (leaving(&body, bottom), leaving(&body, top));
        let corner = body.get_half_edge(he1).unwrap().next;
        let null = body
            .mev_null(
                crate::MevSite::Fan {
                    he1: corner,
                    he2: corner,
                },
                NewVertexSide::Above,
            )
            .unwrap();
        let point = |v: crate::entity::VertexKey| {
            *body.get_point(body.get_vertex(v).unwrap().point).unwrap()
        };
        let chord = geom_brep::EdgeCurveSpec::line_between(point(bottom), point(top));
        let cut = RulingCut {
            body,
            wall,
            null,
            site: crate::MefSite::Chords { he1, he2 },
            chord,
        };
        (cut, ring)
    }

    /// **The piece the `mef` takes the last null edge off is re-minted
    /// whatever else it misses.** On the ringed wall, one ring half's
    /// row detached — a gap no null edge holds — the cut moves the strut
    /// onto the new face and leaves the wall running through no null
    /// edge. The site mint re-mints the loop it rewires on the wall, so
    /// the wall misses only the ring's detached row, on the loop it
    /// keeps, and its other rows are the minting pass's, byte for byte;
    /// the new face misses the strut's two rows and its chord half. At
    /// this unit's first review head the wall, found with a gap no null
    /// edge holds, was left as found and missed its chord half too.
    #[test]
    fn a_mef_that_takes_the_last_null_edge_off_a_face_with_another_gap_mints_its_loop() {
        let (
            RulingCut {
                mut body,
                wall,
                null,
                site,
                chord,
            },
            ring,
        ) = ringed_ruling_cut();
        let crate::LoopBoundary::Cycle { first: on_ring } = body.get_loop(ring).unwrap().boundary
        else {
            panic!("the ring is a cycle")
        };
        assert!(body.detach_pcurve(on_ring).is_some());
        let made = body
            .mef(site, chord, crate::FaceSurface::Inherit, Tol::witness())
            .unwrap();
        assert_eq!(face_of(&body, null.he_plus), made.face, "the strut moves");
        assert_eq!(face_of(&body, on_ring), wall, "the ring stays");
        let chord_on_new = if face_of(&body, made.he_plus) == made.face {
            made.he_plus
        } else {
            made.he_minus
        };
        let mut want = vec![on_ring, null.he_plus, null.he_minus, chord_on_new];
        want.sort();
        assert_eq!(
            missing_rows(&body),
            want,
            "the wall misses only its ring's detached row"
        );
        let mut pass = body.clone();
        crate::pcurves::mint_pcurves_of(&mut pass, &[wall], Tol::witness()).unwrap();
        let ring_row = format!("{on_ring:?} ");
        let passs: Vec<String> = face_rows(&pass, wall)
            .into_iter()
            .filter(|row| !row.starts_with(&ring_row))
            .collect();
        assert_eq!(
            face_rows(&body, wall),
            passs,
            "the wall's other rows are the minting pass's, byte for byte"
        );
    }

    /// **A face with a loop that does not walk is not the site mint's.**
    /// The ringed wall, held open by its null strut, is read further by
    /// the site mint. With its ring's loop record gone — tier 1's
    /// corruption — it is not, even though the strut's release would
    /// otherwise take it whatever it misses.
    #[test]
    fn a_held_open_face_with_a_loop_that_does_not_walk_is_left_as_found() {
        let (RulingCut { mut body, wall, .. }, ring) = ringed_ruling_cut();
        let read_further = |body: &Body<f64>| {
            let face = body.get_face(wall).unwrap();
            let surface = body.get_surface(face.surface).unwrap();
            crate::pcurves::site_rows_from(body, face, surface)
                .unwrap()
                .is_some()
        };
        assert!(read_further(&body), "the held-open wall is read further");
        body.loops.remove(ring).unwrap();
        assert!(
            !read_further(&body),
            "a loop that does not walk keeps the wall from the site mint"
        );
    }
}
