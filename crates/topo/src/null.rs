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
//!   every other scaffolding shape — empty loops, struts).
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
use geom_core::Tol;

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
    /// vertex **coincident with the site's old vertex** (its point is a
    /// bitwise copy — structural coincidence, no comparison) joined by a
    /// **null edge** whose curve entry is [`CurveGeom::NullScaffold`]
    /// carrying the F9 side attribute (`new_side` declares which side
    /// the new vertex faces; the old vertex takes the other).
    ///
    /// Serves ch. 14 `splitclassify` null-edge insertion and ch. 15
    /// null-edge pairs (M3 PRs 2 and 4). Site semantics — fan split,
    /// strut (`he1 == he2`, ch. 15's dangling null edge), lone — and
    /// the surgery are exactly [`Body::mev`]'s; only the geometry lane
    /// differs (no certification gate: there is no carrier to certify,
    /// by type — the ratified F9 shape, module docs). Tier 1 accepts
    /// the result; tier 2 refuses it at rest
    /// ([`crate::ValidationError::NullEdgeAtRest`]).
    ///
    /// Euler vector: `(v +1, e +1, f 0, h 0, r 0, s 0)` — identical to
    /// `mev` (a null edge is an edge).
    ///
    /// **Minting order** (D9, exact — deviates from `mev`'s): point,
    /// **vertex**, curve entry (the F9 attribute names the new vertex,
    /// so the vertex must exist first), edge, `he_plus`, `he_minus`.
    /// Emanating rule and splice positions: as [`Body::mev`].
    ///
    /// # Errors
    ///
    /// The site preconditions, exactly as [`Body::mev`] minus the
    /// certification gate; the body is untouched on `Err`.
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
                let point = plan.p_old; // bitwise coincident copy
                self.mev_fan_execute(
                    plan,
                    point,
                    crate::euler::MevCurveMint::Null(new_side),
                    provenance,
                )
            }
            MevSite::Lone { r#loop } => {
                let (v, p_old) = self.mev_lone_plan(r#loop)?;
                self.mev_lone_execute(
                    r#loop,
                    v,
                    p_old, // bitwise coincident copy
                    crate::euler::MevCurveMint::Null(new_side),
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
    /// Structural preconditions only: the face resolves, both role
    /// loops resolve, and they are distinct. Whether the loops belong
    /// to the face is deliberately **not** checked here or at tier 1 —
    /// Euler surgery legitimately re-homes loops mid-sequence, and the
    /// minting/consuming ops of PRs 2–5 own the record's currency
    /// (module docs). Tier 2 refuses marked faces at rest
    /// ([`crate::ValidationError::NullFaceAtRest`]).
    ///
    /// # Errors
    ///
    /// [`EulerOpError::StaleKey`] if the face or a role loop does not
    /// resolve; [`EulerOpError::SameLoop`] if the two role loops are
    /// one loop. The body is untouched on `Err`.
    pub fn set_null_face_pair(
        &mut self,
        face: FaceKey,
        pair: NullFacePair,
    ) -> Result<(), EulerOpError> {
        if !self.faces.contains_key(face) {
            return Err(EulerOpError::StaleKey {
                key: EntityId::Face(face),
            });
        }
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
    use geom_core::Tol;
    use super::*;
    use crate::fixtures::{deep_snapshot, ops_cube};
    use crate::validate::{ValidationError, validate, validate_closed};

    /// A null strut (`he1 == he2`) on a cube vertex: coincident point
    /// copy, F9 attribute recorded per side, tier 1 accepts, tier 2
    /// refuses by name, and the scaffolding is killable by `kev`.
    #[test]
    fn mev_null_strut_lifecycle() {
        let cube = ops_cube(Tol::witness());
        let mut body = cube.body;
        let he = body
            .get_vertex(cube.seed.vertex)
            .unwrap()
            .emanating
            .unwrap();
        let strut = crate::MevSite::Fan { he1: he, he2: he };
        let created = body.mev_null(strut, NewVertexSide::Above).unwrap();
        // Coincident copy, bitwise.
        let p_new = *body.get_point(created.point).unwrap();
        let p_old = *body
            .get_point(body.get_vertex(cube.seed.vertex).unwrap().point)
            .unwrap();
        assert_eq!(
            (p_new.x.to_bits(), p_new.y.to_bits(), p_new.z.to_bits()),
            (p_old.x.to_bits(), p_old.y.to_bits(), p_old.z.to_bits()),
        );
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
        body.kev(created.he_plus).unwrap();
        assert_eq!(validate_closed(&body), Ok(()));
    }

    /// `Below` puts the new vertex on the below side.
    #[test]
    fn mev_null_below_side_attribute() {
        let cube = ops_cube(Tol::witness());
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
        let cube = ops_cube(Tol::witness());
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
    /// structural preconditions, and kill-op hygiene through `kfmrh`.
    #[test]
    fn null_face_record_lifecycle() {
        let cube = ops_cube(Tol::witness());
        let mut body = cube.body;
        let (f1, f2) = (cube.mefs[0].face, cube.mefs[1].face);
        let outer1 = body.get_face(f1).unwrap().outer;
        let outer2 = body.get_face(f2).unwrap().outer;
        // Distinctness is checked.
        assert_eq!(
            body.set_null_face_pair(
                f1,
                NullFacePair::Split {
                    above_loop: outer1,
                    below_loop: outer1,
                },
            ),
            Err(EulerOpError::SameLoop { r#loop: outer1 })
        );
        // A valid record: tier 1 fine, tier 2 refuses by name.
        body.set_null_face_pair(
            f1,
            NullFacePair::Boolean {
                in_copy: outer1,
                out_copy: outer2,
            },
        )
        .unwrap();
        assert_eq!(validate(&body), Ok(()));
        assert_eq!(
            validate_closed(&body),
            Err(vec![ValidationError::NullFaceAtRest { face: f1 }])
        );
        assert_eq!(
            body.null_face_pair(f1),
            Some(&NullFacePair::Boolean {
                in_copy: outer1,
                out_copy: outer2,
            })
        );
        // Clearing restores tier 2.
        assert!(body.clear_null_face_pair(f1).is_some());
        assert_eq!(validate_closed(&body), Ok(()));
        // Kill-op hygiene: a record on kfmrh's dying face is removed
        // with it (no LeakedNullFaceRecord).
        body.set_null_face_pair(
            f2,
            NullFacePair::Split {
                above_loop: outer1,
                below_loop: outer2,
            },
        )
        .unwrap();
        body.kfmrh(f1, f2).unwrap();
        assert_eq!(body.null_face_pair(f2), None);
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
        let cube = ops_cube(Tol::witness());
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
}
