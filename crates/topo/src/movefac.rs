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

use geom_core::Decide;

use crate::body::Body;
use crate::entity::{EntityId, FaceKey, LoopBoundary, Shell, ShellKey};
#[cfg(debug_assertions)]
use crate::euler::ArenaDelta;
use crate::euler::EulerOpError;
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
        if !self.solids.contains_key(solid) {
            return Err(EulerOpError::StaleKey {
                key: EntityId::Solid(solid),
            });
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
                        if !self.faces.contains_key(neighbor) {
                            return Err(EulerOpError::StaleKey {
                                key: EntityId::Face(neighbor),
                            });
                        }
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
                    "movefac: `solid` proven live by the plan phase's `contains_key` and \
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
}

#[cfg(test)]
#[allow(clippy::unwrap_used, clippy::expect_used, clippy::panic)]
mod tests {
    use geom_core::Tol;
    use geom_core::Point3;

    use super::*;
    use crate::euler::{MefSite, MevSite};
    use crate::fixtures::{deep_snapshot, ops_cube};
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
        body.mef_chord(MefSite::Chords {
            he1: seg.he_plus,
            he2: seg.he_minus,
        }, Tol::witness())
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
        body.mef_chord(MefSite::Chords {
            he1: grow.he_plus,
            he2: grow.he_minus,
        }, Tol::witness())
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
        let cube = ops_cube(Tol::witness());
        let mut body = cube.body;
        let before = deep_snapshot(&body);
        let shells = body.movefac(cube.seed.shell).unwrap();
        assert_eq!(shells, vec![cube.seed.shell]);
        assert_eq!(deep_snapshot(&body), before);
    }

    /// Stale shell: typed error, body untouched.
    #[test]
    fn movefac_stale_shell_is_typed() {
        let cube = ops_cube(Tol::witness());
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
