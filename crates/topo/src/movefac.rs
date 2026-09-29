//! `movefac` — worklist shell partition: split a shell whose incidence
//! complex has fallen into several connected components (the post-mfkrh
//! state) into one shell per component (M3 PR 1).
//!
//! Ch. 14's `splitfinish` and ch. 15's `setopfinish` end with a
//! distribution step: after the section faces are promoted (`mfkrh` on
//! the null-face rings) a single shell entity holds ≥ 2 disconnected
//! closed surfaces, and the pieces must become real shells before the
//! result can rest (tier 2 requires c = 1 per shell). GWB's `movefac`
//! walks faces recursively; ours is the worklist form (F12: no
//! unbounded recursion) over the same edge-adjacency relation the
//! validator's pass-11 component enumeration uses.
//!
//! Like [`Body::ring_move`], `movefac` is **not an Euler operator**: it
//! re-partitions ownership. Unlike `ring_move` it mints entities (the
//! new shells), so it records [`Provenance::Movefac`] birth records for
//! them; the moved faces keep their own birth records (re-homing is not
//! a re-birth). Serves ch. 14 `splitfinish` / ch. 15 `setopfinish`
//! component distribution (M3 PRs 3 and 5).
//!
//! [`Body::move_shells_to_new_solid`] is the same kind of door one
//! level up: shells re-homed into a new solid, minted with
//! [`Provenance::MoveShells`]; nothing about a shell's faces changes.

use geom_core::Decide;

use crate::body::Body;
use geom_brep::EdgeDescription;
use slotmap::SecondaryMap;

use crate::entity::{
    EdgeKey, EntityId, FaceKey, GeomRef, LoopBoundary, Shell, ShellKey, Solid, SolidKey,
};
use crate::geometry::{CurveKey, SurfaceKey};
use crate::null::CurveGeom;
#[cfg(debug_assertions)]
use crate::euler::ArenaDelta;
use crate::euler::EulerOpError;
use crate::live::require_key;
use crate::provenance::Provenance;

impl<T: Decide> Body<T> {
    /// Partitions `shell`'s faces into connected components of the
    /// incidence complex (the validator's pass-11 relation: a face
    /// glues all its loops; a cycle loop glues across each edge via
    /// mate; an empty-loop face is its own dartless component) and
    /// re-homes every component after the first into a **new shell of
    /// the same solid**.
    ///
    /// Returns the component → shell map, in component order:
    /// `result[0]` is always `shell` (which keeps the first
    /// component), `result[i]` for i ≥ 1 are the minted shells. A
    /// connected shell returns `vec![shell]` with the body untouched
    /// (deterministic no-op, like `ring_move`'s).
    ///
    /// **Determinism (D9)**: components are seeded in the shell's
    /// face-list order; the worklist expands loops in (outer, rings)
    /// list order and cycles in `next` order; each new shell's face
    /// list preserves the original list's relative order; new shells
    /// are appended to the solid's shell list in component order.
    /// **Minting order** (exact): the new shells, in component order —
    /// nothing else is minted or killed.
    ///
    /// Tier-1 preservation: components move **whole**, so every edge's
    /// two faces stay in one shell (pass 10) and each new shell's
    /// complex is exactly one component with its old per-component
    /// Euler–Poincaré count (pass 11) — the partition is re-labeled,
    /// never re-cut.
    ///
    /// # Errors
    ///
    /// [`EulerOpError::StaleKey`] if `shell`, its solid, or a
    /// face/loop/half-edge/edge reached by the walk does not resolve;
    /// [`EulerOpError::LoopCycleBroken`] if a cycle walk fails to
    /// close. All checks precede any mutation (atomic).
    pub fn movefac(&mut self, shell: ShellKey) -> Result<Vec<ShellKey>, EulerOpError> {
        #[cfg(debug_assertions)]
        let before = self.arena_counts();

        // ---- Preconditions + read-only component labeling. ----
        let shell_data = self
            .get_shell(shell)
            .cloned()
            .ok_or(EulerOpError::StaleKey {
                key: EntityId::Shell(shell),
            })?;
        let solid = shell_data.solid;
        require_key(&self.solids, solid, EntityId::Solid)?;
        let mut component: slotmap::SecondaryMap<FaceKey, usize> = slotmap::SecondaryMap::new();
        let mut count = 0_usize;
        for &seed in &shell_data.faces {
            if component.contains_key(seed) {
                continue;
            }
            let label = count;
            count += 1;
            let mut pending = vec![seed];
            component.insert(seed, label);
            while let Some(face_key) = pending.pop() {
                let face = self
                    .get_face(face_key)
                    .cloned()
                    .ok_or(EulerOpError::StaleKey {
                        key: EntityId::Face(face_key),
                    })?;
                for loop_key in core::iter::once(face.outer).chain(face.rings.iter().copied()) {
                    let loop_data = self.get_loop(loop_key).ok_or(EulerOpError::StaleKey {
                        key: EntityId::Loop(loop_key),
                    })?;
                    let LoopBoundary::Cycle { first } = loop_data.boundary else {
                        continue; // empty loop: glues only its vertex
                    };
                    let cycle = self
                        .loop_cycle(first)
                        .ok_or(EulerOpError::LoopCycleBroken { r#loop: loop_key })?;
                    for member in cycle {
                        let mate = self.mate(member).ok_or(EulerOpError::StaleKey {
                            key: EntityId::HalfEdge(member),
                        })?;
                        let mate_data = self.resolve_half_edge(mate)?;
                        let mate_loop =
                            self.get_loop(mate_data.parent_loop)
                                .ok_or(EulerOpError::StaleKey {
                                    key: EntityId::Loop(mate_data.parent_loop),
                                })?;
                        let neighbor = mate_loop.face;
                        require_key(&self.faces, neighbor, EntityId::Face)?;
                        if !component.contains_key(neighbor) {
                            component.insert(neighbor, label);
                            pending.push(neighbor);
                        }
                    }
                }
            }
        }

        // ---- Mutation (infallible from here on). ----
        if count <= 1 {
            #[cfg(debug_assertions)]
            self.assert_euler_postcondition(before, ArenaDelta::ZERO, "movefac");
            return Ok(vec![shell]);
        }
        // Per-component face lists, preserving original relative order.
        let mut lists: Vec<Vec<FaceKey>> = vec![Vec::new(); count];
        for &face in &shell_data.faces {
            let Some(&label) = component.get(face) else {
                unreachable!(
                    "movefac: every face of `shell_data.faces` is labelled by the \
                     component walk above"
                )
            };
            lists[label].push(face);
        }
        let mut result = vec![shell];
        let mut lists = lists.into_iter();
        let first = lists.next().unwrap_or_default();
        let Some(shell_data) = self.get_shell_mut(shell) else {
            unreachable!("movefac: `shell` resolved in the plan phase and this op kills no shell")
        };
        shell_data.faces = first;
        for faces in lists {
            let new_shell = self.add_shell(
                Shell {
                    faces: faces.clone(),
                    solid,
                },
                Provenance::Movefac { shell },
            );
            for face in faces {
                let Some(face_data) = self.get_face_mut(face) else {
                    unreachable!(
                        "movefac: every labelled face was resolved by the component walk \
                         above and this op kills no face"
                    )
                };
                face_data.shell = new_shell;
            }
            let Some(solid_data) = self.get_solid_mut(solid) else {
                unreachable!(
                    "movefac: `solid` proven live by the plan phase's `require_key` and \
                     this op kills no solid"
                )
            };
            solid_data.shells.push(new_shell);
            result.push(new_shell);
        }

        #[cfg(debug_assertions)]
        {
            let minted = isize::try_from(count - 1).unwrap_or(isize::MAX);
            self.assert_euler_postcondition(
                before,
                ArenaDelta {
                    shells: minted,
                    ..ArenaDelta::ZERO
                },
                "movefac",
            );
        }
        Ok(result)
    }

    /// Moves `shells` — some, not all, of ONE solid's shells — into a
    /// **new solid** and returns it. Not an Euler operator: it
    /// re-partitions ownership, like [`Body::movefac`] one level up.
    /// The moved shells keep their keys and their faces; what changes
    /// is their `solid` back-pointer, the two solids' shell lists, and
    /// the charts the moved faces share with faces staying behind.
    ///
    /// **A chart lives in one solid** (a [`Body`] invariant, tier 1's
    /// pass 14), so a surface key worn both by a moved face and by a
    /// face staying behind is **re-minted**: the moved faces wear a
    /// bitwise copy, the stayers keep the original. The copy carries
    /// its source's [`crate::GeomOrigin`] row and per-field
    /// `ParamSource` rows verbatim — a copy is the same description,
    /// so a recipe source seen on it is the same source. A moved
    /// edge's curve description that names a re-minted chart is
    /// re-pointed at the copy (handles only, certificate verbatim, as
    /// a disjoint graft carries it); a curve a staying edge also wears
    /// is copied first, with its origin row. Pcurve caches are per
    /// half-edge and name no chart, so they ride along untouched.
    ///
    /// **Determinism (D9)**: the new solid's shell list is `shells` in
    /// the order given; the source solid keeps its remaining shells in
    /// their relative order. **Minting order** (exact): the one solid,
    /// recorded as [`Provenance::MoveShells`] naming the source; then
    /// one surface copy per shared chart, in order of first wear
    /// walking `shells` in the order given and each shell's face list;
    /// then one curve copy per shared curve a moved edge re-points, in
    /// edge-arena order. Nothing is killed.
    ///
    /// Tier-1 preservation: shells move **whole**, so every edge's two
    /// faces stay in one shell and every shell's complex is untouched;
    /// both solids keep at least one shell (pass 9's floor); every
    /// original chart and curve keeps a wearer on the stayers' side and
    /// every copy has one on the moved side (pass 8).
    ///
    /// **Ownership, not material coherence.** Nothing here reads which
    /// shell is an outer boundary and which a cavity: moving a lone
    /// void mints a solid with no outer shell, and tier 3 accepts it
    /// (`shell5_r2_probes::r2_the_new_door_mints_a_solid_with_no_outer_shell`).
    /// A caller owns the pairing it moves.
    ///
    /// # Errors
    ///
    /// All checks precede any mutation (atomic).
    /// [`EulerOpError::NoShellsNamed`] on an empty list;
    /// [`EulerOpError::ShellRepeated`] when a shell is named twice;
    /// [`EulerOpError::StaleKey`] if a shell, its solid, or the solid's
    /// own listing of it does not resolve;
    /// [`EulerOpError::ShellsAcrossSolids`] if the shells are not all in
    /// one solid; [`EulerOpError::SolidWouldEmpty`] if the list is every
    /// shell of that solid.
    pub fn move_shells_to_new_solid(
        &mut self,
        shells: &[ShellKey],
    ) -> Result<SolidKey, EulerOpError> {
        #[cfg(debug_assertions)]
        let before = self.arena_counts();

        // ---- Preconditions, read-only. ----
        let &[first, ..] = shells else {
            return Err(EulerOpError::NoShellsNamed);
        };
        let owner_of = |body: &Self, shell: ShellKey| -> Result<SolidKey, EulerOpError> {
            Ok(body
                .get_shell(shell)
                .ok_or(EulerOpError::StaleKey {
                    key: EntityId::Shell(shell),
                })?
                .solid)
        };
        let source = owner_of(self, first)?;
        for (i, &shell) in shells.iter().enumerate() {
            if shells[..i].contains(&shell) {
                return Err(EulerOpError::ShellRepeated { shell });
            }
            if owner_of(self, shell)? != source {
                return Err(EulerOpError::ShellsAcrossSolids {
                    shell: first,
                    other: shell,
                });
            }
        }
        let listed = self
            .shells_of_solid(source)
            .ok_or(EulerOpError::StaleKey {
                key: EntityId::Solid(source),
            })?
            .to_vec();
        // A shell whose back-pointer names `source` but which `source`
        // does not list is an ownership desync (tier 1's pass 7); the
        // op refuses rather than building on it.
        for &shell in shells {
            if !listed.contains(&shell) {
                return Err(EulerOpError::StaleKey {
                    key: EntityId::Shell(shell),
                });
            }
        }
        if listed.iter().all(|s| shells.contains(s)) {
            return Err(EulerOpError::SolidWouldEmpty { solid: source });
        }

        let plan = self.plan_chart_remint(shells)?;

        // ---- Mutation (infallible from here on). ----
        let new_solid = self.add_solid(
            Solid {
                shells: shells.to_vec(),
            },
            Provenance::MoveShells { solid: source },
        );
        self.remint_charts(&plan);
        for &shell in shells {
            let Some(shell_data) = self.get_shell_mut(shell) else {
                unreachable!(
                    "move_shells_to_new_solid: every named shell resolved in the plan phase \
                     and this op kills no shell"
                )
            };
            shell_data.solid = new_solid;
        }
        let Some(source_data) = self.get_solid_mut(source) else {
            unreachable!(
                "move_shells_to_new_solid: `source` resolved in the plan phase and this op \
                 kills no solid"
            )
        };
        source_data.shells.retain(|s| !shells.contains(s));

        #[cfg(debug_assertions)]
        self.assert_euler_postcondition(
            before,
            ArenaDelta {
                solids: 1,
                ..ArenaDelta::ZERO
            },
            "move_shells_to_new_solid",
        );
        Ok(new_solid)
    }
}

/// What [`Body::move_shells_to_new_solid`] re-mints so that a chart
/// lives in one solid, planned read-only before any mutation.
#[derive(Default)]
struct ChartRemint {
    /// Charts a moved face shares with a staying face, in minting
    /// order (first wear, walking the moved shells as named).
    surfaces: Vec<SurfaceKey>,
    /// The moved faces wearing one of `surfaces`.
    faces: Vec<FaceKey>,
    /// Moved edges whose curve description names one of `surfaces`,
    /// in edge-arena order.
    edges: Vec<EdgeKey>,
    /// Curves of `edges` a staying edge also wears: these are copied,
    /// the rest re-pointed in place.
    shared_curves: SecondaryMap<CurveKey, ()>,
}

/// The chart keys an edge description names.
fn named_charts<T: geom_core::Real>(description: &EdgeDescription<T>) -> [Option<SurfaceKey>; 2] {
    match description {
        EdgeDescription::Intersection { s1, s2, .. }
        | EdgeDescription::TangentIntersection { s1, s2, .. } => [Some(*s1), Some(*s2)],
        EdgeDescription::Chart(chart) => [Some(chart.surface), None],
        EdgeDescription::Scaffold(_) => [None, None],
    }
}

impl<T: Decide> Body<T> {
    /// The read-only half of the re-mint: which charts, faces, edges
    /// and curves a move of `shells` touches. `shells` are live and
    /// listed (the caller's plan phase established it).
    fn plan_chart_remint(&self, shells: &[ShellKey]) -> Result<ChartRemint, EulerOpError> {
        let mut staying: SecondaryMap<SurfaceKey, ()> = SecondaryMap::new();
        for (_, face) in self.faces.iter() {
            if !shells.contains(&face.shell) {
                staying.insert(face.surface, ());
            }
        }
        let mut plan = ChartRemint::default();
        let mut shared: SecondaryMap<SurfaceKey, ()> = SecondaryMap::new();
        for &shell in shells {
            let listed = self.get_shell(shell).ok_or(EulerOpError::StaleKey {
                key: EntityId::Shell(shell),
            })?;
            for &face in &listed.faces {
                let surface = self
                    .get_face(face)
                    .ok_or(EulerOpError::StaleKey {
                        key: EntityId::Face(face),
                    })?
                    .surface;
                if !staying.contains_key(surface) {
                    continue;
                }
                if shared.insert(surface, ()).is_none() {
                    plan.surfaces.push(surface);
                }
                plan.faces.push(face);
            }
        }
        if plan.surfaces.is_empty() {
            return Ok(plan);
        }
        let mut staying_curves: SecondaryMap<CurveKey, ()> = SecondaryMap::new();
        for (edge_key, edge) in self.edges.iter() {
            let face = self
                .face_of_half_edge(edge.he_plus)
                .and_then(|face| self.get_face(face))
                .ok_or(EulerOpError::StaleKey {
                    key: EntityId::HalfEdge(edge.he_plus),
                })?;
            if !shells.contains(&face.shell) {
                staying_curves.insert(edge.curve, ());
                continue;
            }
            let curve = self
                .curves
                .get(edge.curve)
                .ok_or(EulerOpError::StaleGeometry {
                    key: GeomRef::Curve(edge.curve),
                })?;
            let CurveGeom::Certified(curve) = curve else {
                continue; // null scaffolding names no chart
            };
            if named_charts(curve.description())
                .into_iter()
                .flatten()
                .any(|k| shared.contains_key(k))
            {
                plan.edges.push(edge_key);
            }
        }
        for &edge in &plan.edges {
            let curve = self.edges[edge].curve;
            if staying_curves.contains_key(curve) {
                plan.shared_curves.insert(curve, ());
            }
        }
        Ok(plan)
    }

    /// The mutating half of the re-mint (infallible: `plan` was read
    /// off this body, and nothing has been killed since).
    fn remint_charts(&mut self, plan: &ChartRemint) {
        let mut copies: SecondaryMap<SurfaceKey, SurfaceKey> = SecondaryMap::new();
        for &surface in &plan.surfaces {
            let (Some(description), Some(origin)) = (
                self.surfaces.get(surface).cloned(),
                self.surface_origins.get(surface).cloned(),
            ) else {
                unreachable!(
                    "chart re-mint: {surface:?} is worn by a live face and the origin map is \
                     total over live keys (kernel bug)"
                )
            };
            let fields = self.surface_field_sources.get(surface).cloned();
            let copy = self.add_surface(description);
            self.surface_origins.insert(copy, origin);
            if let Some(fields) = fields {
                self.surface_field_sources.insert(copy, fields);
            }
            copies.insert(surface, copy);
        }
        for &face in &plan.faces {
            let Some(data) = self.faces.get_mut(face) else {
                unreachable!("chart re-mint: {face:?} resolved in the plan phase")
            };
            let Some(&copy) = copies.get(data.surface) else {
                unreachable!("chart re-mint: the plan lists {face:?} for a chart it copies")
            };
            data.surface = copy;
        }
        let mut curve_copies: SecondaryMap<CurveKey, CurveKey> = SecondaryMap::new();
        for &edge in &plan.edges {
            let curve = self.edges[edge].curve;
            if let Some(&copy) = curve_copies.get(curve) {
                self.edges[edge].curve = copy;
                continue;
            }
            let Some(CurveGeom::Certified(certified)) = self.curves.get(curve) else {
                unreachable!("chart re-mint: the plan lists {edge:?} for a certified curve")
            };
            let Some(repointed) =
                certified.with_remapped_surfaces(|k| Some(copies.get(k).copied().unwrap_or(k)))
            else {
                unreachable!("chart re-mint: the remap answers every key")
            };
            if plan.shared_curves.contains_key(curve) {
                let Some(origin) = self.curve_origins.get(curve).cloned() else {
                    unreachable!(
                        "chart re-mint: {curve:?} is live and the origin map is total over \
                         live keys (kernel bug)"
                    )
                };
                let copy = self.add_curve(repointed);
                self.curve_origins.insert(copy, origin);
                curve_copies.insert(curve, copy);
                self.edges[edge].curve = copy;
            } else {
                self.curves[curve] = CurveGeom::Certified(repointed);
            }
        }
    }
}

#[cfg(test)]
#[allow(clippy::unwrap_used, clippy::expect_used, clippy::panic)]
mod tests {
    use geom_core::Point3;
    use geom_core::Tol;

    use super::*;
    use crate::euler::{MefSite, MevSite};
    use crate::fixtures::deep_snapshot;
    use crate::test_support_fixtures::declined_cube;
    use crate::validate::{ValidationError, validate, validate_closed};

    fn p(x: f64) -> Point3<f64> {
        Point3::new(x, 0.0, 0.0)
    }

    /// The PR 4 detached-digon transient: pillow + a digon hanging on a
    /// promoted ring — one shell entity, two closed surface components
    /// (the validator suite's construction). Returns
    /// (body, shell, pillow_face_of_ring, promoted_face).
    fn detached_digon() -> (Body<f64>, ShellKey, FaceKey, FaceKey) {
        let mut body = Body::<f64>::new();
        let seed = body.mvfs(p(0.0)).unwrap();
        let seg = body
            .mev_line(
                MevSite::Lone {
                    r#loop: seed.r#loop,
                },
                p(1.0),
                Tol::witness(),
            )
            .unwrap();
        body.mef_chord(
            MefSite::Chords {
                he1: seg.he_plus,
                he2: seg.he_minus,
            },
            Tol::witness(),
        )
        .unwrap();
        let strut = body
            .mev_line(
                MevSite::Fan {
                    he1: seg.he_plus,
                    he2: seg.he_plus,
                },
                p(2.0),
                Tol::witness(),
            )
            .unwrap();
        let kill = body.kemr(strut.he_plus, strut.he_minus).unwrap();
        let grow = body
            .mev_line(MevSite::Lone { r#loop: kill.ring }, p(3.0), Tol::witness())
            .unwrap();
        body.mef_chord(
            MefSite::Chords {
                he1: grow.he_plus,
                he2: grow.he_minus,
            },
            Tol::witness(),
        )
        .unwrap();
        let promoted = body.mfkrh_plug(kill.ring).unwrap();
        (body, seed.shell, seed.face, promoted.face)
    }

    /// The distribution primitive: the two-component shell splits into
    /// two connected shells of one solid; tier 2 is restored; face
    /// lists preserve relative order; re-homed faces keep their birth
    /// provenance and the new shell records `Provenance::Movefac`.
    #[test]
    fn movefac_distributes_the_detached_component() {
        let (mut body, shell, seed_face, promoted_face) = detached_digon();
        assert!(matches!(
            validate_closed(&body).unwrap_err()[..],
            [ValidationError::ShellDisconnected { .. }]
        ));
        let shells = body.movefac(shell).unwrap();
        assert_eq!(shells.len(), 2);
        assert_eq!(shells[0], shell);
        assert_eq!(validate_closed(&body), Ok(()));
        // One solid, two shells.
        let (_, solid) = body.solids().next().unwrap();
        assert_eq!(solid.shells, shells);
        // The seed component stayed; the digon moved.
        assert_eq!(body.get_face(seed_face).unwrap().shell, shells[0]);
        assert_eq!(body.get_face(promoted_face).unwrap().shell, shells[1]);
        // Birth records: moved faces keep theirs; the new shell is a
        // Movefac mint.
        assert_eq!(
            body.provenance(crate::EntityId::Shell(shells[1])),
            Some(&Provenance::Movefac { shell })
        );
    }

    /// A connected shell is a deterministic no-op.
    #[test]
    fn movefac_connected_shell_is_a_noop() {
        let cube = declined_cube::<f64>(Tol::witness());
        let mut body = cube.body;
        let before = deep_snapshot(&body);
        let shells = body.movefac(cube.seed.shell).unwrap();
        assert_eq!(shells, vec![cube.seed.shell]);
        assert_eq!(deep_snapshot(&body), before);
    }

    /// Stale shell: typed error, body untouched.
    #[test]
    fn movefac_stale_shell_is_typed() {
        let cube = declined_cube::<f64>(Tol::witness());
        let mut body = cube.body;
        let before = deep_snapshot(&body);
        let err = body.movefac(ShellKey::default()).unwrap_err();
        assert_eq!(
            err,
            EulerOpError::StaleKey {
                key: EntityId::Shell(ShellKey::default()),
            }
        );
        assert_eq!(deep_snapshot(&body), before);
    }

    /// Determinism (D9): replaying the identical history (including
    /// movefac) yields byte-identical bodies.
    #[test]
    fn movefac_replay_is_byte_identical() {
        let build = || {
            let (mut body, shell, _, _) = detached_digon();
            body.movefac(shell).unwrap();
            body
        };
        assert_eq!(deep_snapshot(&build()), deep_snapshot(&build()));
    }

    /// Cross-shell kfmrh fuses the distributed shells back: the digon
    /// face becomes a ring of the pillow face, the second shell dies,
    /// its faces re-home, and the shell's complex is one component
    /// again (tier 1; tier 2 modulo nothing — the fused body is
    /// closed). Genus bookkeeping: connected sum of two genus-0
    /// components stays genus 0 with one ring.
    #[test]
    fn cross_shell_kfmrh_fuses_shells() {
        let (mut body, shell, seed_face, promoted_face) = detached_digon();
        let shells = body.movefac(shell).unwrap();
        assert_eq!(validate_closed(&body), Ok(()));
        let result = body.kfmrh(seed_face, promoted_face).unwrap();
        assert_eq!(result.killed_shell, Some(shells[1]));
        assert!(!body.shells().any(|(k, _)| k == shells[1]));
        assert_eq!(validate(&body), Ok(()));
        // Back to the pre-movefac shape: one shell, disconnected? No —
        // fusion re-glues through the demoted ring: ONE component.
        assert_eq!(validate_closed(&body), Ok(()));
        assert_eq!(body.get_face(seed_face).unwrap().rings, vec![result.ring]);
        // The re-homed digon face points at the surviving shell.
        let digon_partner = body
            .faces()
            .find(|&(k, _)| k != seed_face && k != promoted_face)
            .map(|(_, f)| f.shell);
        assert_eq!(digon_partner, Some(shell));
    }

    /// The solid re-partition: the distributed digon shell moves into
    /// a solid of its own; keys, faces and the other shell are
    /// untouched; the new solid records `Provenance::MoveShells`
    /// naming the source; tier 1 holds on both solids.
    #[test]
    fn move_shells_to_new_solid_splits_one_solid_in_two() {
        let (mut body, shell, seed_face, promoted_face) = detached_digon();
        let shells = body.movefac(shell).unwrap();
        let (source, _) = body.solids().next().unwrap();
        let moved = body.move_shells_to_new_solid(&[shells[1]]).unwrap();
        assert_ne!(moved, source);
        assert_eq!(body.solids().count(), 2);
        assert_eq!(body.get_solid(source).unwrap().shells, vec![shells[0]]);
        assert_eq!(body.get_solid(moved).unwrap().shells, vec![shells[1]]);
        assert_eq!(body.get_shell(shells[0]).unwrap().solid, source);
        assert_eq!(body.get_shell(shells[1]).unwrap().solid, moved);
        assert_eq!(body.get_face(seed_face).unwrap().shell, shells[0]);
        assert_eq!(body.get_face(promoted_face).unwrap().shell, shells[1]);
        assert_eq!(
            body.provenance(crate::EntityId::Solid(moved)),
            Some(&Provenance::MoveShells { solid: source })
        );
        assert_eq!(validate_closed(&body), Ok(()));
    }

    /// Every precondition refuses typed with the body untouched: an
    /// empty list, a stale shell, a repeated shell, shells of two
    /// solids, and a list that is every shell of its solid.
    #[test]
    fn move_shells_to_new_solid_refuses_typed_at_each_precondition() {
        let cube = declined_cube::<f64>(Tol::witness());
        let mut body = cube.body;
        let only = cube.seed.shell;
        let other = body.mvfs(p(9.0)).unwrap().shell;
        let before = deep_snapshot(&body);
        let rows: [(&[ShellKey], EulerOpError); 5] = [
            (&[], EulerOpError::NoShellsNamed),
            (
                &[ShellKey::default()],
                EulerOpError::StaleKey {
                    key: EntityId::Shell(ShellKey::default()),
                },
            ),
            (&[only, only], EulerOpError::ShellRepeated { shell: only }),
            (
                &[only, other],
                EulerOpError::ShellsAcrossSolids { shell: only, other },
            ),
            (
                &[only],
                EulerOpError::SolidWouldEmpty {
                    solid: body.get_shell(only).unwrap().solid,
                },
            ),
        ];
        for (shells, want) in rows {
            let err = body.move_shells_to_new_solid(shells).unwrap_err();
            assert_eq!(err, want, "shells {shells:?}");
            assert_eq!(
                deep_snapshot(&body),
                before,
                "shells {shells:?}: body untouched"
            );
        }
    }

    /// Determinism (D9): replaying the identical history (including
    /// the solid re-partition) yields byte-identical bodies.
    #[test]
    fn move_shells_to_new_solid_replay_is_byte_identical() {
        let build = || {
            let (mut body, shell, _, _) = detached_digon();
            let shells = body.movefac(shell).unwrap();
            body.move_shells_to_new_solid(&[shells[1]]).unwrap();
            body
        };
        assert_eq!(deep_snapshot(&build()), deep_snapshot(&build()));
    }

    /// Cross-solid kfmrh stays a typed error (two mvfs seeds in one
    /// body are two solids).
    #[test]
    fn cross_solid_kfmrh_is_typed() {
        let mut body = Body::<f64>::new();
        let a = body.mvfs(p(0.0)).unwrap();
        let b = body.mvfs(p(1.0)).unwrap();
        let before = deep_snapshot(&body);
        let err = body.kfmrh(a.face, b.face).unwrap_err();
        assert_eq!(
            err,
            EulerOpError::CrossSolid {
                f1: a.face,
                f2: b.face,
            }
        );
        assert_eq!(deep_snapshot(&body), before);
    }
}
