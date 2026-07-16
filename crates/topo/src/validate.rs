//! The structural validation harness: [`validate`] and
//! [`ValidationError`].
//!
//! M0 validates **structural integrity only** — the referential soundness
//! of the arena store, checkable without any geometry evaluation. The
//! exact check set (each check names its error variant):
//!
//! 1. **Reference resolution.** Every topology key held by an entity
//!    resolves in its arena — each `ShellKey` in a solid, `FaceKey` in a
//!    shell, `LoopKey` in a face, `EdgeKey` in a loop, and `VertexKey` in
//!    an edge ([`ValidationError::DanglingTopology`]); and every geometry
//!    key resolves — a face's surface, an edge's curve, a vertex's point
//!    ([`ValidationError::DanglingGeometry`]).
//! 2. **Containment-spine ownership.** Every shell, face, and loop is
//!    referenced by its parent kind **exactly once**, counting
//!    multiplicity — zero is [`ValidationError::OrphanEntity`], two or
//!    more (whether two parents or one parent listing a child twice) is
//!    [`ValidationError::MultiplyOwned`]. Every edge is referenced by at
//!    least one loop and every vertex by at least one edge — zero is
//!    [`ValidationError::OrphanEntity`] (shared use is the point: edges
//!    and vertices sit on the spine's shared rim, and the manifold
//!    "exactly two loops per interior edge" pairing is M1 watertightness,
//!    not an M0 check). Spine *acyclicity* needs no check at all: the
//!    reference structure is stratified by types (solids only hold shell
//!    keys, shells only face keys, …), so a cycle cannot be expressed.
//! 3. **Orphan geometry.** Every point, curve, and surface is referenced
//!    by at least one entity; unreferenced geometry is
//!    [`ValidationError::OrphanGeometry`] — an **error**, not a warning:
//!    bodies are values built by operators, and nothing should leak.
//!
//! M1 extends both the function and the enum in place — Euler–Poincaré
//! counting, watertightness/orientation coherence, and the D4 ¶2
//! residual-certification check (`residual ≤ ε` for every derived cache)
//! plug in as new variants and new passes. The harness is deliberately a
//! plain function plus an error enum, **not a trait**: there is exactly
//! one notion of body validity per milestone, and speculative abstraction
//! would only blur it.
//!
//! # All failures, not the first
//!
//! [`validate`] collects **every** failure before returning: a validator
//! that stops at the first defect is a bad debugging tool, and fail-loud
//! (D4) means report everything. The `Err` vector is never empty.
//!
//! # Deterministic report order (D9)
//!
//! Errors arrive in a fixed, documented order — three passes, each
//! walking its arenas in slot-index order, checking an entity's
//! references in field-declaration order:
//!
//! 1. reference resolution, walking solids → shells → faces → loops →
//!    edges → vertices;
//! 2. ownership multiplicity, sweeping shells → faces → loops → edges →
//!    vertices;
//! 3. orphan geometry, sweeping points → curves → surfaces.

use core::fmt;

use geom_core::Real;
use slotmap::{Key, SecondaryMap};

use crate::body::Body;
use crate::entity::{EntityId, GeomRef};

/// A structural defect found by [`validate`]. Closed enum, D3 style:
/// M1 adds variants (Euler–Poincaré, watertightness, orientation
/// coherence, residual certification) as compiler-guided extensions —
/// every match site is forced to say what it does with the new failure
/// kinds, which is exactly the loudness the house style wants.
#[derive(Clone, Debug, PartialEq, Eq)]
pub enum ValidationError {
    /// An entity holds a topology key that does not resolve in its arena.
    /// Reported once per occurrence (a parent listing the same dangling
    /// key twice yields two errors).
    DanglingTopology {
        /// The entity holding the dangling reference.
        from: EntityId,
        /// The dangling key (wrapped with its kind; it resolves to
        /// nothing).
        to: EntityId,
    },
    /// An entity holds a geometry key that does not resolve in its arena.
    DanglingGeometry {
        /// The entity holding the dangling reference.
        from: EntityId,
        /// The dangling geometry key.
        to: GeomRef,
    },
    /// A spine entity nothing references: a shell in no solid, a face in
    /// no shell, a loop in no face, an edge in no loop, or a vertex in no
    /// edge.
    OrphanEntity {
        /// The unreferenced entity.
        entity: EntityId,
    },
    /// A shell, face, or loop referenced more than once by its parent
    /// kind — the containment spine is a tree of ownership down to loops,
    /// so exactly one parent may hold each child, once.
    MultiplyOwned {
        /// The multiply-referenced child.
        child: EntityId,
        /// How many owning references were found (≥ 2; multiplicity
        /// counts, so one parent listing a child twice reports 2).
        owners: usize,
    },
    /// A geometry-arena entry no entity references. An error, not a
    /// warning: bodies are values built by operators, and nothing should
    /// leak (see the [module docs](self)).
    OrphanGeometry {
        /// The unreferenced geometry entry.
        geometry: GeomRef,
    },
}

impl fmt::Display for ValidationError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::DanglingTopology { from, to } => {
                write!(f, "{from} references {to}, which does not resolve")
            }
            Self::DanglingGeometry { from, to } => {
                write!(f, "{from} references {to}, which does not resolve")
            }
            Self::OrphanEntity { entity } => {
                write!(f, "{entity} is referenced by no parent entity")
            }
            Self::MultiplyOwned { child, owners } => write!(
                f,
                "{child} has {owners} owning references (the containment \
                 spine requires exactly one)"
            ),
            Self::OrphanGeometry { geometry } => {
                write!(f, "{geometry} is referenced by no entity")
            }
        }
    }
}

impl std::error::Error for ValidationError {}

/// Counts one resolved reference to `key` (dangling keys never reach
/// here). `SecondaryMap` rather than a hash map: typed per key kind and
/// deterministic to sweep, per the crate's D9 iteration invariant.
fn count_ref<K: Key>(counts: &mut SecondaryMap<K, usize>, key: K) {
    let n = counts.get(key).copied().unwrap_or(0);
    counts.insert(key, n + 1);
}

/// Validates a body's structural integrity, collecting **all** failures.
///
/// The check set, the report order, and the M1 extension plan are
/// documented in the [module docs](self). The empty body validates
/// vacuously.
///
/// # Errors
///
/// A non-empty vector of every [`ValidationError`] found, in the
/// documented deterministic order.
pub fn validate<T: Real>(body: &Body<T>) -> Result<(), Vec<ValidationError>> {
    let mut errors = Vec::new();

    // Ownership / reference counters, filled by pass 1 and swept by
    // passes 2 and 3. Only keys that resolve are counted; dangling keys
    // are reported in pass 1 and count toward nothing.
    let mut shell_owners: SecondaryMap<_, usize> = SecondaryMap::new();
    let mut face_owners: SecondaryMap<_, usize> = SecondaryMap::new();
    let mut loop_owners: SecondaryMap<_, usize> = SecondaryMap::new();
    let mut edge_refs: SecondaryMap<_, usize> = SecondaryMap::new();
    let mut vertex_refs: SecondaryMap<_, usize> = SecondaryMap::new();
    let mut point_refs: SecondaryMap<_, usize> = SecondaryMap::new();
    let mut curve_refs: SecondaryMap<_, usize> = SecondaryMap::new();
    let mut surface_refs: SecondaryMap<_, usize> = SecondaryMap::new();

    // Pass 1: reference resolution, solids → shells → faces → loops →
    // edges → vertices; within an entity, field-declaration order.
    for (solid_key, solid) in body.solids.iter() {
        for &shell in &solid.shells {
            if body.shells.contains_key(shell) {
                count_ref(&mut shell_owners, shell);
            } else {
                errors.push(ValidationError::DanglingTopology {
                    from: EntityId::Solid(solid_key),
                    to: EntityId::Shell(shell),
                });
            }
        }
    }
    for (shell_key, shell) in body.shells.iter() {
        for &face in &shell.faces {
            if body.faces.contains_key(face) {
                count_ref(&mut face_owners, face);
            } else {
                errors.push(ValidationError::DanglingTopology {
                    from: EntityId::Shell(shell_key),
                    to: EntityId::Face(face),
                });
            }
        }
    }
    for (face_key, face) in body.faces.iter() {
        if body.surfaces.contains_key(face.surface) {
            count_ref(&mut surface_refs, face.surface);
        } else {
            errors.push(ValidationError::DanglingGeometry {
                from: EntityId::Face(face_key),
                to: GeomRef::Surface(face.surface),
            });
        }
        for &loop_ in &face.loops {
            if body.loops.contains_key(loop_) {
                count_ref(&mut loop_owners, loop_);
            } else {
                errors.push(ValidationError::DanglingTopology {
                    from: EntityId::Face(face_key),
                    to: EntityId::Loop(loop_),
                });
            }
        }
    }
    for (loop_key, loop_) in body.loops.iter() {
        for &edge in &loop_.edges {
            if body.edges.contains_key(edge) {
                count_ref(&mut edge_refs, edge);
            } else {
                errors.push(ValidationError::DanglingTopology {
                    from: EntityId::Loop(loop_key),
                    to: EntityId::Edge(edge),
                });
            }
        }
    }
    for (edge_key, edge) in body.edges.iter() {
        for endpoint in [edge.start, edge.end] {
            if body.vertices.contains_key(endpoint) {
                count_ref(&mut vertex_refs, endpoint);
            } else {
                errors.push(ValidationError::DanglingTopology {
                    from: EntityId::Edge(edge_key),
                    to: EntityId::Vertex(endpoint),
                });
            }
        }
        if body.curves.contains_key(edge.curve) {
            count_ref(&mut curve_refs, edge.curve);
        } else {
            errors.push(ValidationError::DanglingGeometry {
                from: EntityId::Edge(edge_key),
                to: GeomRef::Curve(edge.curve),
            });
        }
    }
    for (vertex_key, vertex) in body.vertices.iter() {
        if body.points.contains_key(vertex.point) {
            count_ref(&mut point_refs, vertex.point);
        } else {
            errors.push(ValidationError::DanglingGeometry {
                from: EntityId::Vertex(vertex_key),
                to: GeomRef::Point(vertex.point),
            });
        }
    }

    // Pass 2: ownership multiplicity, shells → faces → loops → edges →
    // vertices. Shells/faces/loops: exactly one owner; edges/vertices: at
    // least one referent.
    for (key, _) in body.shells.iter() {
        match shell_owners.get(key).copied().unwrap_or(0) {
            0 => errors.push(ValidationError::OrphanEntity {
                entity: EntityId::Shell(key),
            }),
            1 => {}
            owners => errors.push(ValidationError::MultiplyOwned {
                child: EntityId::Shell(key),
                owners,
            }),
        }
    }
    for (key, _) in body.faces.iter() {
        match face_owners.get(key).copied().unwrap_or(0) {
            0 => errors.push(ValidationError::OrphanEntity {
                entity: EntityId::Face(key),
            }),
            1 => {}
            owners => errors.push(ValidationError::MultiplyOwned {
                child: EntityId::Face(key),
                owners,
            }),
        }
    }
    for (key, _) in body.loops.iter() {
        match loop_owners.get(key).copied().unwrap_or(0) {
            0 => errors.push(ValidationError::OrphanEntity {
                entity: EntityId::Loop(key),
            }),
            1 => {}
            owners => errors.push(ValidationError::MultiplyOwned {
                child: EntityId::Loop(key),
                owners,
            }),
        }
    }
    for (key, _) in body.edges.iter() {
        if edge_refs.get(key).copied().unwrap_or(0) == 0 {
            errors.push(ValidationError::OrphanEntity {
                entity: EntityId::Edge(key),
            });
        }
    }
    for (key, _) in body.vertices.iter() {
        if vertex_refs.get(key).copied().unwrap_or(0) == 0 {
            errors.push(ValidationError::OrphanEntity {
                entity: EntityId::Vertex(key),
            });
        }
    }

    // Pass 3: orphan geometry, points → curves → surfaces.
    for (key, _) in body.points.iter() {
        if point_refs.get(key).copied().unwrap_or(0) == 0 {
            errors.push(ValidationError::OrphanGeometry {
                geometry: GeomRef::Point(key),
            });
        }
    }
    for (key, _) in body.curves.iter() {
        if curve_refs.get(key).copied().unwrap_or(0) == 0 {
            errors.push(ValidationError::OrphanGeometry {
                geometry: GeomRef::Curve(key),
            });
        }
    }
    for (key, _) in body.surfaces.iter() {
        if surface_refs.get(key).copied().unwrap_or(0) == 0 {
            errors.push(ValidationError::OrphanGeometry {
                geometry: GeomRef::Surface(key),
            });
        }
    }

    if errors.is_empty() {
        Ok(())
    } else {
        Err(errors)
    }
}

#[cfg(test)]
#[allow(clippy::unwrap_used, clippy::expect_used, clippy::panic)]
mod tests {
    use geom_core::Point3;
    use proptest::prelude::*;

    use super::*;
    use crate::entity::{Edge, Face, Loop, Shell, ShellKey, Solid, Vertex, VertexKey};
    use crate::geometry::{CurveGeom, PointKey, SurfaceGeom};
    use crate::provenance::Provenance;

    fn prov(op: &'static str) -> Provenance {
        Provenance::Primordial { op }
    }

    fn anchor() -> Point3<f64> {
        Point3::origin()
    }

    /// The keys of the canonical tiny well-formed body, for building
    /// malformed variations around.
    struct Tiny {
        body: Body<f64>,
        p0: PointKey,
        v0: VertexKey,
        sh: ShellKey,
    }

    /// The smallest structurally well-formed body these checks accept: one
    /// solid → one shell → one face (with surface) → one loop → two edges
    /// (a digon) over two vertices with points and curves. Geometry values
    /// are irrelevant to structural validation — everything sits at the
    /// origin.
    fn tiny() -> Tiny {
        let mut body = Body::<f64>::new();
        let p0 = body.add_point(anchor());
        let p1 = body.add_point(Point3::new(1.0, 0.0, 0.0));
        let v0 = body.add_vertex(Vertex { point: p0 }, prov("tiny"));
        let v1 = body.add_vertex(Vertex { point: p1 }, prov("tiny"));
        let c0 = body.add_curve(CurveGeom::Placeholder { anchor: anchor() });
        let c1 = body.add_curve(CurveGeom::Placeholder { anchor: anchor() });
        let e0 = body.add_edge(
            Edge {
                start: v0,
                end: v1,
                curve: c0,
            },
            prov("tiny"),
        );
        let e1 = body.add_edge(
            Edge {
                start: v1,
                end: v0,
                curve: c1,
            },
            prov("tiny"),
        );
        let lp = body.add_loop(
            Loop {
                edges: vec![e0, e1],
            },
            prov("tiny"),
        );
        let s = body.add_surface(SurfaceGeom::Placeholder { anchor: anchor() });
        let f = body.add_face(
            Face {
                surface: s,
                loops: vec![lp],
            },
            prov("tiny"),
        );
        let sh = body.add_shell(Shell { faces: vec![f] }, prov("tiny"));
        body.add_solid(Solid { shells: vec![sh] }, prov("tiny"));
        Tiny { body, p0, v0, sh }
    }

    #[test]
    fn empty_body_validates_vacuously() {
        assert_eq!(validate(&Body::<f64>::new()), Ok(()));
    }

    #[test]
    fn tiny_body_validates_cleanly() {
        assert_eq!(validate(&tiny().body), Ok(()));
    }

    // ------------------------------------------------------------------
    // One test per error variant, each asserting the EXACT error list —
    // the malformation is surgical, so nothing else may trip.
    //
    // Malformed bodies are built through the public raw-insertion API
    // plus direct pub(crate) arena access (removal has no public API at
    // M0), which is exactly what unit tests are for.
    // ------------------------------------------------------------------

    #[test]
    fn dangling_topology_is_reported() {
        let mut t = tiny();
        // Mint a key that can never resolve again: insert, then remove
        // (the slot's version is bumped; even reuse cannot revive the
        // key — see `stale_key_lookup_is_none_not_panic` in body.rs).
        let dead = t.body.add_shell(Shell { faces: vec![] }, prov("dead"));
        t.body.shells.remove(dead);
        let s2 = t.body.add_solid(Solid { shells: vec![dead] }, prov("bad"));
        assert_eq!(
            validate(&t.body),
            Err(vec![ValidationError::DanglingTopology {
                from: EntityId::Solid(s2),
                to: EntityId::Shell(dead),
            }])
        );
    }

    #[test]
    fn dangling_geometry_is_reported() {
        let mut t = tiny();
        // Repoint an existing vertex at a removed point: the vertex's
        // reference dangles, and the abandoned point becomes an orphan.
        let dead = t.body.add_point(anchor());
        t.body.points.remove(dead);
        t.body.vertices.get_mut(t.v0).unwrap().point = dead;
        assert_eq!(
            validate(&t.body),
            Err(vec![
                ValidationError::DanglingGeometry {
                    from: EntityId::Vertex(t.v0),
                    to: GeomRef::Point(dead),
                },
                ValidationError::OrphanGeometry {
                    geometry: GeomRef::Point(t.p0),
                },
            ])
        );
    }

    #[test]
    fn orphan_entity_is_reported() {
        let mut t = tiny();
        // A vertex no edge references. Its point is NOT an orphan —
        // geometry referenced by an orphan entity is still referenced.
        let p = t.body.add_point(anchor());
        let v = t.body.add_vertex(Vertex { point: p }, prov("orphan"));
        assert_eq!(
            validate(&t.body),
            Err(vec![ValidationError::OrphanEntity {
                entity: EntityId::Vertex(v),
            }])
        );
    }

    #[test]
    fn orphan_shell_face_loop_are_reported() {
        let mut t = tiny();
        let sh = t.body.add_shell(Shell { faces: vec![] }, prov("orphan"));
        assert_eq!(
            validate(&t.body),
            Err(vec![ValidationError::OrphanEntity {
                entity: EntityId::Shell(sh),
            }])
        );
    }

    #[test]
    fn multiply_owned_by_two_parents_is_reported() {
        let mut t = tiny();
        // A second solid claiming the same shell: owners = 2.
        t.body.add_solid(Solid { shells: vec![t.sh] }, prov("bad"));
        assert_eq!(
            validate(&t.body),
            Err(vec![ValidationError::MultiplyOwned {
                child: EntityId::Shell(t.sh),
                owners: 2,
            }])
        );
    }

    #[test]
    fn multiply_owned_within_one_parent_is_reported() {
        let mut t = tiny();
        // One solid listing the same (fresh, otherwise-valid) shell twice:
        // multiplicity counts, so owners = 2.
        let sh2 = t.body.add_shell(Shell { faces: vec![] }, prov("dup"));
        t.body.add_solid(
            Solid {
                shells: vec![sh2, sh2],
            },
            prov("dup"),
        );
        assert_eq!(
            validate(&t.body),
            Err(vec![ValidationError::MultiplyOwned {
                child: EntityId::Shell(sh2),
                owners: 2,
            }])
        );
    }

    #[test]
    fn orphan_geometry_is_reported_for_all_three_arenas() {
        let mut t = tiny();
        let p = t.body.add_point(anchor());
        let c = t
            .body
            .add_curve(CurveGeom::Placeholder { anchor: anchor() });
        let s = t
            .body
            .add_surface(SurfaceGeom::Placeholder { anchor: anchor() });
        assert_eq!(
            validate(&t.body),
            Err(vec![
                ValidationError::OrphanGeometry {
                    geometry: GeomRef::Point(p),
                },
                ValidationError::OrphanGeometry {
                    geometry: GeomRef::Curve(c),
                },
                ValidationError::OrphanGeometry {
                    geometry: GeomRef::Surface(s),
                },
            ])
        );
    }

    /// The validator collects every failure in one call, in the
    /// documented deterministic order (pass 1 danglings in spine order,
    /// pass 2 ownership sweeps, pass 3 geometry sweeps — each arena in
    /// slot-index order).
    #[test]
    fn multiple_errors_are_collected_in_documented_order() {
        let mut t = tiny();

        // Defect A: a solid referencing a dead shell (pass 1, solids).
        let dead_shell = t.body.add_shell(Shell { faces: vec![] }, prov("dead"));
        t.body.shells.remove(dead_shell);
        let s2 = t.body.add_solid(
            Solid {
                shells: vec![dead_shell],
            },
            prov("bad"),
        );
        // Defect B: v0 repointed at a dead point (pass 1, vertices) —
        // which also orphans p0 (pass 3, points).
        let dead_point = t.body.add_point(anchor());
        t.body.points.remove(dead_point);
        t.body.vertices.get_mut(t.v0).unwrap().point = dead_point;
        // Defect C: a third solid sharing the tiny body's shell (pass 2,
        // shells).
        t.body.add_solid(Solid { shells: vec![t.sh] }, prov("bad"));
        // Defect D: an orphan vertex (pass 2, vertices), whose fresh
        // point is referenced and therefore NOT an orphan.
        let p_ov = t.body.add_point(anchor());
        let ov = t.body.add_vertex(Vertex { point: p_ov }, prov("orphan"));
        // Defect E: an orphan point, in a slot after p0 (pass 3, points).
        let op = t.body.add_point(anchor());

        assert_eq!(
            validate(&t.body),
            Err(vec![
                // Pass 1: solids first (s2's dangling shell), vertices
                // last (v0's dangling point).
                ValidationError::DanglingTopology {
                    from: EntityId::Solid(s2),
                    to: EntityId::Shell(dead_shell),
                },
                ValidationError::DanglingGeometry {
                    from: EntityId::Vertex(t.v0),
                    to: GeomRef::Point(dead_point),
                },
                // Pass 2: shells (double ownership), then vertices
                // (orphan).
                ValidationError::MultiplyOwned {
                    child: EntityId::Shell(t.sh),
                    owners: 2,
                },
                ValidationError::OrphanEntity {
                    entity: EntityId::Vertex(ov),
                },
                // Pass 3: points in slot order — p0 (orphaned by the
                // repoint) precedes op (inserted later).
                ValidationError::OrphanGeometry {
                    geometry: GeomRef::Point(t.p0),
                },
                ValidationError::OrphanGeometry {
                    geometry: GeomRef::Point(op),
                },
            ])
        );
    }

    #[test]
    fn errors_display_without_panicking() {
        let mut t = tiny();
        t.body.add_point(anchor());
        let errors = validate(&t.body).expect_err("orphan point must fail");
        for e in &errors {
            // Display and Error are wired up; content is human-oriented.
            assert!(!e.to_string().is_empty());
            let _: &dyn std::error::Error = e;
        }
    }

    // ------------------------------------------------------------------
    // Property test: every random small well-formed containment tree
    // validates cleanly, as does its clone. The strategy is deterministic
    // (proptest's seeded RNG; no ambient state): a nested count shape
    // [solid][shell][face][loop] -> edges-per-loop is drawn, then the
    // body is built by one fixed traversal of that shape. Each loop with
    // k edges is a closed k-gon over k fresh vertices (k = 1 is a
    // self-loop edge), so vertices are genuinely shared between edges —
    // exercising the ≥ 1 (not exactly-1) multiplicity rule for the
    // spine's rim.
    // ------------------------------------------------------------------

    /// Builds a well-formed body from a nested count shape. Coordinates
    /// are derived from indices; their values are irrelevant to
    /// structural validation.
    fn build_tree(shape: &[Vec<Vec<Vec<usize>>>]) -> Body<f64> {
        let mut body = Body::<f64>::new();
        for solid_shape in shape {
            let mut shells = Vec::new();
            for shell_shape in solid_shape {
                let mut faces = Vec::new();
                for face_shape in shell_shape {
                    let surface = body.add_surface(SurfaceGeom::Placeholder { anchor: anchor() });
                    let mut loops = Vec::new();
                    for &edge_count in face_shape {
                        let vertices: Vec<VertexKey> = (0..edge_count)
                            .map(|_| {
                                let p = body.add_point(anchor());
                                body.add_vertex(Vertex { point: p }, prov("tree"))
                            })
                            .collect();
                        let edges = (0..edge_count)
                            .map(|i| {
                                let c = body.add_curve(CurveGeom::Placeholder { anchor: anchor() });
                                body.add_edge(
                                    Edge {
                                        start: vertices[i],
                                        end: vertices[(i + 1) % edge_count],
                                        curve: c,
                                    },
                                    prov("tree"),
                                )
                            })
                            .collect();
                        loops.push(body.add_loop(Loop { edges }, prov("tree")));
                    }
                    faces.push(body.add_face(Face { surface, loops }, prov("tree")));
                }
                shells.push(body.add_shell(Shell { faces }, prov("tree")));
            }
            body.add_solid(Solid { shells }, prov("tree"));
        }
        body
    }

    proptest! {
        #[test]
        fn well_formed_trees_validate_cleanly(
            shape in prop::collection::vec(          // solids
                prop::collection::vec(               // shells per solid
                    prop::collection::vec(           // faces per shell
                        prop::collection::vec(1usize..=4, 1..=2), // edges per loop, loops per face
                        1..=3,
                    ),
                    1..=2,
                ),
                1..=2,
            ),
        ) {
            let body = build_tree(&shape);
            prop_assert_eq!(validate(&body), Ok(()));
            // Clone independence, propertywise: the clone validates too,
            // and mutating it leaves the original clean.
            let mut cloned = body.clone();
            prop_assert_eq!(validate(&cloned), Ok(()));
            cloned.add_point(anchor()); // now malformed (orphan point)
            prop_assert!(validate(&cloned).is_err());
            prop_assert_eq!(validate(&body), Ok(()));
        }
    }
}
