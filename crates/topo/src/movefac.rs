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
use crate::entity::{EntityId, FaceKey, LoopBoundary, LoopKey, Shell, ShellKey, Solid, SolidKey};
#[cfg(debug_assertions)]
use crate::euler::ArenaDelta;
use crate::euler::{EulerOpError, RunExtent};
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
    /// [`EulerOpError::NotOwned`] if a face the walk reaches is not the
    /// shell's (its `shell` is another, or the shell does not list it),
    /// or a loop a face lists names another face;
    /// [`EulerOpError::LoopCycleBroken`] naming the loop if a cycle walk
    /// fails to close or is not the loop's whole cycle: a member claims
    /// another loop, or a half-edge that claims the loop is not walked;
    /// [`EulerOpError::UnclaimedHalfEdge`] if a member's edge does not
    /// claim it, and [`EulerOpError::NotSameEdge`] if the mate that edge
    /// gives names another edge. All checks precede any mutation
    /// (atomic).
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
        // How many half-edges claim each loop. A walk is its loop's whole
        // cycle when every member claims the loop (`require_run_of`) and
        // it is as long as the loop's claim count: `RunExtent::Whole`'s
        // proof, with the arena read once rather than once per loop.
        let mut claims: slotmap::SecondaryMap<LoopKey, usize> = slotmap::SecondaryMap::new();
        for (_, half) in &self.half_edges {
            match claims.get_mut(half.parent_loop) {
                Some(count) => *count += 1,
                None => {
                    claims.insert(half.parent_loop, 1);
                }
            }
        }
        let mut listed: slotmap::SecondaryMap<FaceKey, ()> = slotmap::SecondaryMap::new();
        for &face in &shell_data.faces {
            listed.insert(face, ());
        }
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
                // Every face labelled is the shell's, in both directions:
                // the move builds its lists from the shell's, so a face
                // the walk glued on from outside them would join two
                // components through a face it does not move.
                if face.shell != shell || !listed.contains_key(face_key) {
                    return Err(EulerOpError::NotOwned {
                        child: EntityId::Face(face_key),
                        owner: EntityId::Shell(shell),
                    });
                }
                for loop_key in core::iter::once(face.outer).chain(face.rings.iter().copied()) {
                    let loop_data = self.get_loop(loop_key).ok_or(EulerOpError::StaleKey {
                        key: EntityId::Loop(loop_key),
                    })?;
                    if loop_data.face != face_key {
                        return Err(EulerOpError::NotOwned {
                            child: EntityId::Loop(loop_key),
                            owner: EntityId::Face(face_key),
                        });
                    }
                    let LoopBoundary::Cycle { first } = loop_data.boundary else {
                        continue; // empty loop: glues only its vertex
                    };
                    let broken = || EulerOpError::LoopCycleBroken { r#loop: loop_key };
                    let cycle = self.loop_cycle(first).ok_or_else(broken)?;
                    let run =
                        self.require_run_of(cycle.iter().copied(), loop_key, RunExtent::Part, &[])?;
                    if claims.get(loop_key) != Some(&run.len()) {
                        return Err(broken());
                    }
                    for member in cycle {
                        let mate_data = self.proven_mate(member)?.mate_data;
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
    /// is their `solid` back-pointer and the two solids' shell lists.
    ///
    /// **Determinism (D9)**: the new solid's shell list is `shells` in
    /// the order given; the source solid keeps its remaining shells in
    /// their relative order. **Minting order** (exact): the one solid,
    /// recorded as [`Provenance::MoveShells`] naming the source — nothing
    /// else is minted or killed.
    ///
    /// Tier-1 preservation: shells move **whole**, so every edge's two
    /// faces stay in one shell and every shell's complex is untouched;
    /// both solids keep at least one shell (pass 9's floor).
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

        // ---- Mutation (infallible from here on). ----
        let new_solid = self.add_solid(
            Solid {
                shells: shells.to_vec(),
            },
            Provenance::MoveShells { solid: source },
        );
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

#[cfg(test)]
#[allow(clippy::unwrap_used, clippy::expect_used, clippy::panic)]
mod tests {
    use geom_core::Point3;
    use geom_core::Tol;

    use super::*;
    use crate::entity::{EdgeKey, LoopKey};
    use crate::euler::{MefSite, MevSite};
    use crate::fixtures::{deep_snapshot, detached_digons, ops_strut_cube};
    use crate::test_support_fixtures::declined_cube;
    use crate::validate::{ValidationError, validate, validate_closed};
    use slotmap::SecondaryMap;

    fn p(x: f64) -> Point3<f64> {
        Point3::new(x, 0.0, 0.0)
    }

    /// The PR 4 detached-digon transient: pillow + a digon hanging on a
    /// promoted ring — one shell entity, two closed surface components
    /// (the validator suite's construction). Returns
    /// (body, shell, pillow_face_of_ring, promoted_face).
    fn detached_digon() -> (Body<f64>, ShellKey, FaceKey, FaceKey) {
        let (body, shell, seed_face, promoted) = detached_digons(1);
        (body, shell, seed_face, promoted[0])
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
        let other = body.mvfs(p(9.0), true).unwrap().shell;
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
        let a = body.mvfs(p(0.0), true).unwrap();
        let b = body.mvfs(p(1.0), true).unwrap();
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

    /// `shell`'s components under the claims its records make, not the
    /// links a walk steps: a half-edge lies in the face its
    /// `parent_loop` names, and the halves that name one edge glue their
    /// faces. On a valid body that is `movefac`'s relation; on a torn one
    /// it is the partition the records still hold. Labels follow the
    /// shell's face order.
    pub(super) fn claimed_components(
        body: &Body<f64>,
        shell: ShellKey,
    ) -> SecondaryMap<FaceKey, usize> {
        let mut by_edge: SecondaryMap<EdgeKey, Vec<FaceKey>> = SecondaryMap::new();
        for (_, half) in body.half_edges() {
            let Some(face) = body.get_loop(half.parent_loop).map(|l| l.face) else {
                continue;
            };
            match by_edge.get_mut(half.edge) {
                Some(faces) => faces.push(face),
                None => {
                    by_edge.insert(half.edge, vec![face]);
                }
            }
        }
        let mut glued: SecondaryMap<FaceKey, Vec<FaceKey>> = SecondaryMap::new();
        for (_, faces) in &by_edge {
            for pair in faces.windows(2) {
                for (a, b) in [(pair[0], pair[1]), (pair[1], pair[0])] {
                    match glued.get_mut(a) {
                        Some(near) => near.push(b),
                        None => {
                            glued.insert(a, vec![b]);
                        }
                    }
                }
            }
        }
        let mut label = SecondaryMap::new();
        let mut count = 0;
        for &seed in &body.get_shell(shell).unwrap().faces {
            if label.contains_key(seed) {
                continue;
            }
            label.insert(seed, count);
            let mut pending = vec![seed];
            while let Some(face) = pending.pop() {
                for &near in glued.get(face).into_iter().flatten() {
                    if !label.contains_key(near) {
                        label.insert(near, count);
                        pending.push(near);
                    }
                }
            }
            count += 1;
        }
        label
    }

    /// The number of components [`claimed_components`] finds among
    /// `shell`'s own faces.
    pub(super) fn claimed_count(body: &Body<f64>, shell: ShellKey) -> usize {
        let truth = claimed_components(body, shell);
        let faces = &body.get_shell(shell).unwrap().faces;
        let mut labels: Vec<usize> = faces.iter().map(|&f| truth[f]).collect();
        labels.sort_unstable();
        labels.dedup();
        labels.len()
    }

    /// How the partition `shells` hold misreads `truth`: whether one
    /// shell holds faces of two components (joined), and whether one
    /// component's faces lie in two shells (split).
    pub(super) fn misread(
        body: &Body<f64>,
        shells: &[ShellKey],
        truth: &SecondaryMap<FaceKey, usize>,
    ) -> (bool, bool) {
        let mut home: std::collections::BTreeMap<usize, ShellKey> = Default::default();
        let (mut joined, mut split) = (false, false);
        for &shell in shells {
            let mut own = None;
            for &face in &body.get_shell(shell).unwrap().faces {
                let label = truth[face];
                joined |= *own.get_or_insert(label) != label;
                split |= *home.entry(label).or_insert(shell) != shell;
            }
        }
        (joined, split)
    }

    /// `movefac(shell)` on a torn `body` refuses `expected` with the body
    /// deep-unchanged. It runs in a surgery scope, so an `Ok` answers with
    /// its partition rather than with the tier-1 postcondition a torn
    /// input fails whatever the operator writes.
    fn assert_refuses_torn(body: &mut Body<f64>, shell: ShellKey, expected: &EulerOpError) {
        let truth = claimed_components(body, shell);
        let before = deep_snapshot(body);
        let mut scope = body.begin_surgery();
        let outcome = scope.movefac(shell);
        drop(scope);
        match outcome {
            Ok(shells) => {
                let (joined, split) = misread(body, &shells, &truth);
                panic!(
                    "movefac partitioned a torn body: Ok({shells:?}), \
                     joined {joined}, split {split}"
                );
            }
            Err(err) => {
                assert_eq!(&err, expected);
                assert_eq!(deep_snapshot(body), before, "movefac atomicity on Err");
            }
        }
    }

    /// [`assert_refuses_torn`] for [`EulerOpError::NotOwned`], with the
    /// rendered refusal read for its subject: `child` taken as `owner`'s,
    /// and both directions the ownership can fail in, at every raise
    /// site whichever direction failed there.
    fn assert_refuses_not_owned(
        body: &mut Body<f64>,
        shell: ShellKey,
        child: EntityId,
        owner: EntityId,
    ) {
        let expected = EulerOpError::NotOwned { child, owner };
        assert_refuses_torn(body, shell, &expected);
        let text = expected.to_string();
        for subject in [
            format!("movefac took {child} as {owner}'s"),
            format!("{owner} does not list {child}, or {child} does not name {owner}"),
        ] {
            assert!(
                text.contains(&subject),
                "NotOwned renders `{subject}`: {text}"
            );
        }
    }

    /// The first member of `l`'s cycle.
    fn first_of(body: &Body<f64>, l: LoopKey) -> crate::entity::HalfEdgeKey {
        match body.get_loop(l).unwrap().boundary {
            LoopBoundary::Cycle { first } => first,
            LoopBoundary::Empty { .. } => panic!("{l:?} is a cycle loop"),
        }
    }

    /// **A diverted walk would join two components.** Two torn `next`
    /// links route the seed face's outer walk through the digon's outer
    /// loop and back, so the walk closes over both loops' members and
    /// reads the digon's mates: the labelling would find one component
    /// where the records hold two, and return the shell as connected.
    #[test]
    fn movefac_refuses_a_walk_diverted_through_another_components_loop() {
        let (mut body, shell, seed_face, promoted) = detached_digons(1);
        let walked = body.get_face(seed_face).unwrap().outer;
        let other = body.get_face(promoted[0]).unwrap().outer;
        let h = first_of(&body, walked);
        let h_next = body.get_half_edge(h).unwrap().next;
        let x = first_of(&body, other);
        let y = *body.loop_cycle(x).unwrap().last().unwrap();
        body.get_half_edge_mut(h).unwrap().next = x;
        body.get_half_edge_mut(y).unwrap().next = h_next;
        let diverted = body.loop_cycle(h).expect("the diverted walk closes");
        assert!(
            diverted.contains(&x),
            "the walk crosses into the digon's loop"
        );
        assert_eq!(claimed_count(&body, shell), 2, "the records still hold two");
        assert_refuses_torn(
            &mut body,
            shell,
            &EulerOpError::LoopCycleBroken { r#loop: walked },
        );
    }

    /// **A short walk would split one component.** The strut cube's top
    /// loop, anchored at the strut's out half with the tip's return torn
    /// back onto it, walks `[s+, s−]` and closes: both members claim the
    /// loop, and so do four more. The top face is the shell's first, so
    /// the labelling would seed it, reach only itself across the strut,
    /// and label the other five faces a second component.
    #[test]
    fn movefac_refuses_a_walk_closed_short_of_its_loop() {
        let t = ops_strut_cube(Tol::witness());
        let mut body = t.body;
        let top = body.get_loop(t.outer).unwrap().face;
        let shell = body.get_face(top).unwrap().shell;
        assert_eq!(
            body.get_shell(shell).unwrap().faces[0],
            top,
            "the top face seeds"
        );
        body.get_loop_mut(t.outer).unwrap().boundary = LoopBoundary::Cycle {
            first: t.strut.he_plus,
        };
        body.get_half_edge_mut(t.strut.he_minus).unwrap().next = t.strut.he_plus;
        assert_eq!(
            body.loop_cycle(t.strut.he_plus),
            Some(vec![t.strut.he_plus, t.strut.he_minus]),
            "the short walk closes"
        );
        assert_eq!(claimed_count(&body, shell), 1, "the records still hold one");
        assert_refuses_torn(
            &mut body,
            shell,
            &EulerOpError::LoopCycleBroken { r#loop: t.outer },
        );
    }

    /// The first member of a face's outer loop.
    fn outer_first(body: &Body<f64>, face: FaceKey) -> crate::entity::HalfEdgeKey {
        first_of(body, body.get_face(face).unwrap().outer)
    }

    /// **A mate whose own edge is another would join two components.**
    /// The detached digon's shell before the partition, and a circle in
    /// another shell: the seed face's first half-edge torn to name the
    /// circle's edge, which is torn to claim it with a half-edge of the
    /// digon as its mate. The circle's two halves, which the edge no
    /// longer claims, lie in the other shell, so no walk of this one
    /// reaches them; the mate hop would glue the seed face to the digon.
    #[test]
    fn movefac_refuses_a_mate_whose_stranded_half_lies_in_another_shell() {
        let (mut body, shell, seed_face, promoted) = detached_digons(1);
        let seed = body.mvfs(p(9.0), true).unwrap();
        let circle = body
            .mef_chord(
                MefSite::Lone {
                    r#loop: seed.r#loop,
                },
                Tol::witness(),
            )
            .unwrap();
        let x = outer_first(&body, seed_face);
        let y = outer_first(&body, promoted[0]);
        body.get_half_edge_mut(x).unwrap().edge = circle.edge;
        let torn = body.get_edge_mut(circle.edge).unwrap();
        (torn.he_plus, torn.he_minus) = (x, y);
        assert_eq!(claimed_count(&body, shell), 2, "the records still hold two");
        assert_refuses_torn(
            &mut body,
            shell,
            &EulerOpError::NotSameEdge { he1: x, he2: y },
        );
    }

    /// `a`'s mate and `b`'s mate traded: `a`'s edge claims `a` with `b`'s
    /// old mate, `b`'s edge claims `b` with `a`'s, and each traded half
    /// names its new edge. The bijection stays whole.
    fn trade_mates(
        body: &mut Body<f64>,
        a: crate::entity::HalfEdgeKey,
        b: crate::entity::HalfEdgeKey,
    ) {
        let (ea, eb) = (
            body.get_half_edge(a).unwrap().edge,
            body.get_half_edge(b).unwrap().edge,
        );
        let (a2, b2) = (body.mate(a).unwrap(), body.mate(b).unwrap());
        for (edge, old, new) in [(ea, a2, b2), (eb, b2, a2)] {
            let data = body.get_edge_mut(edge).unwrap();
            if data.he_plus == old {
                data.he_plus = new;
            } else {
                data.he_minus = new;
            }
        }
        body.get_half_edge_mut(a2).unwrap().edge = eb;
        body.get_half_edge_mut(b2).unwrap().edge = ea;
    }

    /// The detached digon's shell before the partition, and a pillow in
    /// another shell traded one edge's mates with the seed face and the
    /// other's with the digon: each mate hop names its edge, and the
    /// walk reaches the digon only through the pillow. Returns the body,
    /// the shell, the pillow's shell and faces, and the pillow face the
    /// seed face's walk reaches last, which the worklist pops first.
    fn bridged_through_a_pillow() -> (Body<f64>, ShellKey, ShellKey, [FaceKey; 2], FaceKey) {
        let (mut body, shell, seed_face, promoted) = detached_digons(1);
        let seed = body.mvfs(p(9.0), true).unwrap();
        let seg = body
            .mev_line(
                MevSite::Lone {
                    r#loop: seed.r#loop,
                },
                p(10.0),
                Tol::witness(),
            )
            .unwrap();
        let chord = body
            .mef_chord(
                MefSite::Chords {
                    he1: seg.he_plus,
                    he2: seg.he_minus,
                },
                Tol::witness(),
            )
            .unwrap();
        let x = outer_first(&body, seed_face);
        let y = outer_first(&body, promoted[0]);
        trade_mates(&mut body, x, seg.he_plus);
        trade_mates(&mut body, y, chord.he_plus);
        let face_of = |he| {
            let l = body.get_half_edge(he).unwrap().parent_loop;
            body.get_loop(l).unwrap().face
        };
        let reached = face_of(seg.he_plus);
        (body, shell, seed.shell, [seed.face, chord.face], reached)
    }

    /// **A neighbour in another shell would join two components**: the
    /// walk would reach the digon through the pillow ([`bridged_through_a_pillow`]),
    /// which the shell does not list, and leave the two in one shell.
    /// First the pillow's faces as its own shell has them, then with
    /// their `shell` torn to the walked one, which still does not list
    /// them.
    #[test]
    fn movefac_refuses_a_neighbour_in_another_shell() {
        for torn_back in [false, true] {
            let (mut body, shell, _, pillow, reached) = bridged_through_a_pillow();
            if torn_back {
                for face in pillow {
                    body.get_face_mut(face).unwrap().shell = shell;
                }
            }
            assert_refuses_not_owned(
                &mut body,
                shell,
                EntityId::Face(reached),
                EntityId::Shell(shell),
            );
        }
    }

    /// A face the shell lists whose `shell` is torn to another: the walk
    /// reaches it from the seed face, and the partition would leave it
    /// naming the other shell.
    #[test]
    fn movefac_refuses_a_face_it_lists_that_names_another_shell() {
        let (mut body, shell, seed_face, _) = detached_digons(1);
        let other = body.mvfs(p(9.0), true).unwrap();
        let x = outer_first(&body, seed_face);
        let mate = body.mate(x).unwrap();
        let l = body.get_half_edge(mate).unwrap().parent_loop;
        let neighbour = body.get_loop(l).unwrap().face;
        assert_ne!(neighbour, seed_face);
        body.get_face_mut(neighbour).unwrap().shell = other.shell;
        assert_refuses_not_owned(
            &mut body,
            shell,
            EntityId::Face(neighbour),
            EntityId::Shell(shell),
        );
    }

    /// **A loop of another face would join two components.** The seed
    /// face of the detached digon's shell torn to list the digon's outer
    /// loop as a ring: its walk would glue the seed face to the digon's
    /// mates.
    #[test]
    fn movefac_refuses_a_loop_that_names_another_face() {
        let (mut body, shell, seed_face, promoted) = detached_digons(1);
        let foreign = body.get_face(promoted[0]).unwrap().outer;
        body.get_face_mut(seed_face).unwrap().rings.push(foreign);
        assert_eq!(claimed_count(&body, shell), 2, "the records still hold two");
        assert_refuses_not_owned(
            &mut body,
            shell,
            EntityId::Loop(foreign),
            EntityId::Face(seed_face),
        );
    }

    /// No over-refusal: on every valid body here — one to three
    /// components, rings, struts, genus — `movefac` partitions each shell
    /// exactly as its records do. An enumeration, not a sample.
    #[test]
    fn valid_fixtures_partition_as_their_records_do() {
        use crate::fixtures::{
            mvfs_state, ngon_pillow, ops_genus2, ops_holed_box, ops_ring_bridge, ops_strutted,
            pillow, raw_prism,
        };
        use crate::test_support_fixtures::geometric_cube;
        let tol = Tol::witness();
        let bodies: [(&str, Body<f64>); 13] = [
            ("declined_cube", declined_cube(tol).body),
            ("geometric_cube", geometric_cube(tol).body),
            ("ops_strut_cube", ops_strut_cube(tol).body),
            ("ops_holed_box", ops_holed_box(tol).body),
            ("ops_genus2", ops_genus2(tol)),
            ("ops_ring_bridge", ops_ring_bridge(tol).body),
            ("ops_strutted", ops_strutted(tol).0),
            ("pillow", pillow(tol).body),
            ("ngon_pillow(5)", ngon_pillow(5, tol).body),
            ("raw_prism(3)", raw_prism(3, tol).body),
            ("mvfs_state", mvfs_state().body),
            ("one detached digon", detached_digons(1).0),
            ("two detached digons", detached_digons(2).0),
        ];
        let mut most = 0;
        for (fixture, body) in bodies {
            assert_eq!(validate(&body), Ok(()), "{fixture} is valid");
            for (shell, _) in body.shells() {
                let truth = claimed_components(&body, shell);
                let components = claimed_count(&body, shell);
                most = most.max(components);
                let mut trial = body.clone();
                let shells = trial
                    .movefac(shell)
                    .unwrap_or_else(|e| panic!("the valid {fixture} refuses {e:?}"));
                assert_eq!(
                    shells.len(),
                    components,
                    "{fixture}: one shell per component"
                );
                assert_eq!(
                    misread(&trial, &shells, &truth),
                    (false, false),
                    "{fixture}: the partition is the records'"
                );
                assert_eq!(validate(&trial), Ok(()), "{fixture}: tier 1 after movefac");
            }
        }
        assert_eq!(
            most, 3,
            "a shell of three components reaches the c − 1 > 1 arm"
        );
    }

    #[cfg(test)]
    mod tears;
}
