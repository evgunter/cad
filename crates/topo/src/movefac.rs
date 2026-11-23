//! `movefac` — worklist shell partition: split a shell whose incidence
//! complex has fallen into several connected components (the post-mfkrh
//! state) into one shell per component (M3 PR 1).
//!
//! Ch. 14's `splitfinish` and ch. 15's `setopfinish` end with a
//! distribution step: after the section faces are promoted (`mfkrh` on
//! the null-face rings) a single shell entity holds ≥ 2 disconnected
//! closed surfaces, and the pieces must become real shells before the
//! result can rest (tier 2 requires c = 1 per shell). GWB's `movefac`
//! walks faces recursively; ours reads the components from
//! [`crate::glue::face_set_glue`], a worklist walk (F12: no unbounded
//! recursion) of the relation the validator's pass 11 enumerates.
//!
//! Like [`Body::ring_move`](crate::construction::Construct::ring_move), `movefac` is **not an Euler operator**: it
//! re-partitions ownership. Unlike `ring_move` it mints entities (the
//! new shells), so it records [`Provenance::Movefac`] birth records for
//! them; the moved faces keep their own birth records (re-homing is not
//! a re-birth). Serves ch. 14 `splitfinish` / ch. 15 `setopfinish`
//! component distribution (M3 PRs 3 and 5).
//!
//! [`Body::move_shells_to_new_solid`](crate::construction::Construct::move_shells_to_new_solid) is the same kind of door one
//! level up: shells re-homed into a new solid, minted with
//! [`Provenance::MoveShells`]; nothing about a shell's faces changes.

use geom_core::Decide;

use crate::body::Body;
use crate::entity::{EntityId, Shell, ShellKey, Solid, SolidKey};
#[cfg(debug_assertions)]
use crate::euler::ArenaDelta;
use crate::euler::EulerOpError;
use crate::glue::{LoopClaims, face_set_glue};
use crate::live::{Arg, linked, lookup};
use crate::provenance::Provenance;

impl<T: Decide> Body<T> {
    /// The door [`Construct::movefac`](crate::construction::Construct::movefac) forwards to;
    /// documented there.
    pub(crate) fn movefac(&mut self, shell: ShellKey) -> Result<Vec<ShellKey>, EulerOpError> {
        #[cfg(debug_assertions)]
        let before = self.arena_counts();

        // ---- Preconditions + read-only component labeling. ----
        let shell_data = lookup(&self.shells, shell, EntityId::Shell, Arg("shell"))?.clone();
        let solid = shell_data.solid;
        linked(
            &self.solids,
            solid,
            EntityId::Solid,
            EntityId::Shell(shell),
            "solid",
        );
        // The move builds its lists from the shell's, so a listed face
        // naming another shell would be left naming it.
        for &face in &shell_data.faces {
            let owner = linked(
                &self.faces,
                face,
                EntityId::Face,
                EntityId::Shell(shell),
                "faces",
            )
            .shell;
            assert!(
                owner == shell,
                "movefac: shell {shell:?} lists {face:?}, which names shell {owner:?}: on a \
                 tier-1-valid body a face's shell lists it"
            );
        }
        let glue = face_set_glue(
            self,
            &LoopClaims::of(self),
            &shell_data.faces,
            EntityId::Shell(shell),
        );
        // A face the walk glued on from outside the list would join two
        // components through a face the move does not move.
        if let Some(hop) = glue.leaving.first() {
            unreachable!(
                "movefac reached {:?}, which is not shell {shell:?}'s, across {:?} from {:?}: \
                 the shell does not list it, and on a tier-1-valid body an edge's two faces \
                 share a shell and a face's shell lists it",
                hop.to, hop.member, hop.from
            );
        }
        let count = glue.count;

        // ---- Mutation (infallible from here on). ----
        if count <= 1 {
            #[cfg(debug_assertions)]
            self.assert_euler_postcondition(before, ArenaDelta::ZERO, "movefac");
            return Ok(vec![shell]);
        }
        let mut result = vec![shell];
        let mut lists = glue.components(&shell_data.faces).into_iter();
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
                        "movefac: every listed face was resolved by the plan's owner pre-pass \
                         and this op kills no face"
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

    /// The door [`Construct::move_shells_to_new_solid`](crate::construction::Construct::move_shells_to_new_solid) forwards to;
    /// documented there.
    pub(crate) fn move_shells_to_new_solid(
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
            Ok(lookup(&body.shells, shell, EntityId::Shell, Arg("shells"))?.solid)
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
        let listed = linked(
            &self.solids,
            source,
            EntityId::Solid,
            EntityId::Shell(first),
            "solid",
        )
        .shells
        .clone();
        for &shell in shells {
            assert!(
                listed.contains(&shell),
                "shell {shell:?} names solid {source:?}, which does not list it: on a \
                 tier-1-valid body a shell's solid lists it"
            );
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

    /// Refiles every shell of `donor` under `keeper` — appended to
    /// `keeper`'s shell list in `donor`'s order, each back-pointer moved —
    /// and removes `donor`: [`Body::move_shells_to_new_solid`](crate::construction::Construct::move_shells_to_new_solid) run
    /// backwards, and the one home of "these shells are one solid's".
    /// Not an Euler operator; ownership moves and nothing else, every
    /// shell keeping its key and faces. The emptied donor is REMOVED, not
    /// left standing (a shell-less solid is `SolidWithoutShells`), and
    /// its arena removal is paired with its provenance removal the way
    /// `kvfs` pairs them (a removal that leaves the record behind is
    /// `LeakedProvenance`). Folding a solid into itself changes nothing.
    ///
    /// The result is construction state: several pieces of material
    /// under one solid, which tier 3's check 10 refuses until the verb
    /// that made it sorts it ([`crate::pieces`]).
    ///
    /// # Errors
    ///
    /// [`BadArgument::Stale`](crate::euler::BadArgument::Stale) if `donor` or `keeper` does not resolve.
    /// All checks precede any mutation (atomic).
    ///
    /// # Panics
    ///
    /// Where a shell `donor` lists does not resolve: on a tier-1-valid
    /// body a solid's shells are live.
    pub(crate) fn fold_solid_into(
        &mut self,
        donor: SolidKey,
        keeper: SolidKey,
    ) -> Result<(), EulerOpError> {
        let moved = lookup(&self.solids, donor, EntityId::Solid, Arg("donor"))?
            .shells
            .clone();
        lookup(&self.solids, keeper, EntityId::Solid, Arg("keeper"))?;
        if donor == keeper {
            return Ok(());
        }
        for &shell in &moved {
            linked(
                &self.shells,
                shell,
                EntityId::Shell,
                EntityId::Solid(donor),
                "shells",
            );
        }

        // ---- Mutation (infallible from here on). ----
        #[cfg(debug_assertions)]
        let before = self.arena_counts();
        for &shell in &moved {
            let Some(data) = self.get_shell_mut(shell) else {
                unreachable!("fold_solid_into: every moved shell resolved in the plan phase")
            };
            data.solid = keeper;
        }
        let Some(k) = self.get_solid_mut(keeper) else {
            unreachable!("fold_solid_into: `keeper` resolved in the plan phase")
        };
        k.shells.extend(moved);
        self.solids.remove(donor);
        self.solid_provenance.remove(donor);
        #[cfg(debug_assertions)]
        self.assert_euler_postcondition(
            before,
            ArenaDelta {
                solids: -1,
                ..ArenaDelta::ZERO
            },
            "fold_solid_into",
        );
        Ok(())
    }

    /// Folds every solid of the body into its first (in arena order)
    /// ([`Body::fold_solid_into`]) and returns the keeper, or `None` for
    /// a body with no solid.
    ///
    /// A multi-shell solid is the shape the boolean's and the split's
    /// pipelines read an operand in: they classify, keep and graft
    /// SHELLS, and the result sort ([`crate::pieces`]) files them back
    /// into pieces at the exit.
    ///
    /// # Errors
    ///
    /// [`Body::fold_solid_into`]'s.
    pub(crate) fn merge_all_solids(&mut self) -> Result<Option<SolidKey>, EulerOpError> {
        let solids: Vec<SolidKey> = self.solids().map(|(k, _)| k).collect();
        let Some((&keeper, donors)) = solids.split_first() else {
            return Ok(None);
        };
        for &donor in donors {
            self.fold_solid_into(donor, keeper)?;
        }
        Ok(Some(keeper))
    }

    /// **Failure-injection door** (`sweep-testing` only): a clone of
    /// this body with every shell filed under its first solid
    /// ([`Body::merge_all_solids`]) — several pieces of material under one
    /// solid, the state the verbs sort out of their results and tier 3's
    /// check 10 refuses (`ValidationError::SolidOuterShells`). No verb
    /// produces it; it exists to be refused, and to stand for a body
    /// built before solids were one piece each.
    #[cfg(feature = "sweep-testing")]
    #[doc(hidden)]
    #[must_use]
    pub fn with_solids_merged_for_tests(&self) -> Self {
        let mut out = self.clone();
        if let Err(e) = out.merge_all_solids() {
            unreachable!("a body's own solid and shell keys resolve: {e:?}")
        }
        out
    }
}

#[cfg(test)]
#[allow(clippy::unwrap_used, clippy::expect_used, clippy::panic)]
mod tests {
    use geom_core::Point3;
    use geom_core::Tol;

    use super::*;
    use crate::entity::{EdgeKey, FaceKey, LoopBoundary, LoopKey};
    use crate::euler::{BadArgument, MefSite, MevSite};
    use crate::fixtures::{deep_snapshot, detached_digons, ops_strut_cube};
    use crate::test_support_fixtures::declined_cube;
    use crate::validate::{ValidationError, validate, validate_closed};
    use slotmap::SecondaryMap;

    fn p(x: f64) -> Point3<f64> {
        Point3::new(x, 0.0, 0.0)
    }

    /// **movefac panics on a ring link that does not resolve**, where
    /// the loop walk reads it.
    #[test]
    fn movefac_panics_on_a_torn_ring_link() {
        use crate::live::OPERATORS_KEEP_LINKS;
        use crate::review_d18::{ROW_FOUR, assert_torn_op_panics, tear_ring};
        let mut body = declined_cube::<f64>(Tol::witness()).body;
        let (face, shell) = body.faces().next().map(|(k, f)| (k, f.shell)).unwrap();
        let named = tear_ring(&mut body, face);
        assert_torn_op_panics(
            "movefac",
            &mut body,
            &[&named, ROW_FOUR, OPERATORS_KEEP_LINKS],
            |b| b.movefac(shell),
        );
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
            EulerOpError::Argument(BadArgument::Stale {
                role: "shell",
                key: EntityId::Shell(ShellKey::default()),
            })
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
                EulerOpError::Argument(BadArgument::Stale {
                    role: "shells",
                    key: EntityId::Shell(ShellKey::default()),
                }),
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

    /// `movefac(shell)` on a torn `body` panics with a message containing
    /// `want`, before it mutates the body; an `Ok` answers with its
    /// partition.
    #[track_caller]
    fn assert_panics_torn(body: &mut Body<f64>, shell: ShellKey, want: &str) {
        let truth = claimed_components(body, shell);
        crate::review_d18::assert_torn_op_panics("movefac", body, &[want], |b| {
            b.movefac(shell).map(|shells| {
                let (joined, split) = misread(b, &shells, &truth);
                let counts: Vec<usize> = shells
                    .iter()
                    .map(|&s| b.get_shell(s).unwrap().faces.len())
                    .collect();
                format!("{shells:?}, face counts {counts:?}, joined {joined}, split {split}")
            })
        });
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
    fn movefac_panics_on_a_walk_diverted_through_another_components_loop() {
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
        assert_panics_torn(&mut body, shell, &format!("leaving loop {walked:?}"));
    }

    /// **A short walk would split one component.** The strut cube's top
    /// loop, anchored at the strut's out half with the tip's return torn
    /// back onto it, walks `[s+, s−]` and closes: both members claim the
    /// loop, and so do four more. The top face is the shell's first, so
    /// the labelling would seed it, reach only itself across the strut,
    /// and label the other five faces a second component.
    #[test]
    fn movefac_panics_on_a_walk_closed_short_of_its_loop() {
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
        assert_panics_torn(
            &mut body,
            shell,
            &format!(
                "loop {:?}'s cycle walk is not every half-edge that claims it",
                t.outer
            ),
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
    fn movefac_panics_on_a_mate_whose_stranded_half_lies_in_another_shell() {
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
        assert_panics_torn(
            &mut body,
            shell,
            &format!(
                "{x:?} and {y:?}, read as edge {:?}'s halves, are not",
                circle.edge
            ),
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
    /// the shell, the pillow's shell and its faces.
    fn bridged_through_a_pillow() -> (Body<f64>, ShellKey, ShellKey, [FaceKey; 2]) {
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
        (body, shell, seed.shell, [seed.face, chord.face])
    }

    /// **A neighbour in another shell would join two components**: the
    /// walk would reach the digon through the pillow ([`bridged_through_a_pillow`]),
    /// which the shell does not list, and leave the two in one shell.
    /// First the pillow's faces as its own shell has them, then with
    /// their `shell` torn to the walked one, which still does not list
    /// them. The panic names the pillow face the walk reached first.
    #[test]
    fn movefac_panics_on_a_neighbour_in_another_shell() {
        for torn_back in [false, true] {
            let (mut body, shell, _, pillow) = bridged_through_a_pillow();
            if torn_back {
                for face in pillow {
                    body.get_face_mut(face).unwrap().shell = shell;
                }
            }
            let truth = claimed_components(&body, shell);
            let report = crate::review_d18::assert_torn_op_panics(
                "movefac",
                &mut body,
                &[&format!("which is not shell {shell:?}'s")],
                |b| b.movefac(shell).map(|shells| misread(b, &shells, &truth)),
            );
            assert!(
                pillow
                    .iter()
                    .any(|f| report.contains(&format!("movefac reached {f:?},"))),
                "the panic names a pillow face {pillow:?}: {report}"
            );
        }
    }

    /// A face the shell lists whose `shell` is torn to another: the
    /// partition would leave it naming the other shell.
    #[test]
    fn movefac_panics_on_a_face_it_lists_that_names_another_shell() {
        let (mut body, shell, seed_face, _) = detached_digons(1);
        let other = body.mvfs(p(9.0), true).unwrap();
        let x = outer_first(&body, seed_face);
        let mate = body.mate(x).unwrap();
        let l = body.get_half_edge(mate).unwrap().parent_loop;
        let neighbour = body.get_loop(l).unwrap().face;
        assert_ne!(neighbour, seed_face);
        body.get_face_mut(neighbour).unwrap().shell = other.shell;
        assert_panics_torn(
            &mut body,
            shell,
            &format!(
                "shell {shell:?} lists {neighbour:?}, which names shell {:?}",
                other.shell
            ),
        );
    }

    /// **A loop of another face would join two components.** The seed
    /// face of the detached digon's shell torn to list the digon's outer
    /// loop as a ring: its walk would glue the seed face to the digon's
    /// mates.
    #[test]
    fn movefac_panics_on_a_loop_that_names_another_face() {
        let (mut body, shell, seed_face, promoted) = detached_digons(1);
        let foreign = body.get_face(promoted[0]).unwrap().outer;
        body.get_face_mut(seed_face).unwrap().rings.push(foreign);
        assert_eq!(claimed_count(&body, shell), 2, "the records still hold two");
        assert_panics_torn(
            &mut body,
            shell,
            &format!("{seed_face:?} lists loop {foreign:?}, which names face"),
        );
    }

    /// `detached_digons(1)`'s digon faces in the shell's list order, and
    /// the second's outer loop: the loop the first's mates lie in.
    fn digon_faces(body: &Body<f64>, shell: ShellKey) -> (FaceKey, FaceKey, LoopKey) {
        let faces = &body.get_shell(shell).unwrap().faces;
        let d1 = faces[2];
        let mate = body.mate(outer_first(body, d1)).unwrap();
        let ld2 = body.get_half_edge(mate).unwrap().parent_loop;
        let d2 = body.get_loop(ld2).unwrap().face;
        assert_eq!(faces[3], d2, "the shell lists the digon's faces last");
        (d1, d2, ld2)
    }

    /// **A mate's loop its face does not list would join two components
    /// and carry a third face.** The digon's second face's outer loop,
    /// which the first face's mates lie in, torn to name the seed face,
    /// and the shell torn to stop listing the second face: the first
    /// face's hop would land on the seed face through a loop the seed
    /// face does not list, and nothing would ever walk the second face.
    #[test]
    fn movefac_panics_on_a_mates_loop_its_face_does_not_list() {
        let (mut body, shell, seed_face, _) = detached_digons(1);
        let (_, d2, ld2) = digon_faces(&body, shell);
        body.get_loop_mut(ld2).unwrap().face = seed_face;
        body.get_shell_mut(shell)
            .unwrap()
            .faces
            .retain(|&f| f != d2);
        assert_panics_torn(
            &mut body,
            shell,
            &format!("loop {ld2:?} names face {seed_face:?}, which does not list it"),
        );
    }

    /// The first of those two tears alone, the second face still listed,
    /// in either order of the digon's faces: listed first, the first
    /// face's hop finds the seed face does not list the loop; swapped,
    /// the second face's own loop names another face.
    #[test]
    fn movefac_panics_on_a_mates_loop_naming_another_face_in_either_order() {
        for swapped in [false, true] {
            let (mut body, shell, seed_face, _) = detached_digons(1);
            let (_, d2, ld2) = digon_faces(&body, shell);
            body.get_loop_mut(ld2).unwrap().face = seed_face;
            if swapped {
                body.get_shell_mut(shell).unwrap().faces.swap(2, 3);
            }
            let want = if swapped {
                format!("{d2:?} lists loop {ld2:?}, which names face {seed_face:?}")
            } else {
                format!("loop {ld2:?} names face {seed_face:?}, which does not list it")
            };
            assert_panics_torn(&mut body, shell, &want);
        }
    }

    /// **A loop torn empty under the half-edges that claim it would let
    /// the partition follow the shell's face order.** The digon's first
    /// face's outer loop torn `Empty`: its two halves still claim it, and
    /// their mates lie in the second face. The second face's hops reach
    /// the first; nothing walks back. Listed first, the first face would
    /// be labelled alone and the second face's hop dropped on it, the
    /// digon left in two shells; swapped, the two would share one. Both
    /// orders panic: the loop's whole cycle, its claimants, is not walked.
    #[test]
    fn movefac_panics_on_an_empty_loop_half_edges_claim_in_either_order() {
        for swapped in [false, true] {
            let (mut body, shell, _, _) = detached_digons(1);
            let (d1, _, _) = digon_faces(&body, shell);
            let emptied = body.get_face(d1).unwrap().outer;
            let vertex = body.get_half_edge(first_of(&body, emptied)).unwrap().start;
            body.get_loop_mut(emptied).unwrap().boundary = LoopBoundary::Empty { vertex };
            if swapped {
                body.get_shell_mut(shell).unwrap().faces.swap(2, 3);
            }
            assert_eq!(claimed_count(&body, shell), 2, "the records still hold two");
            assert_panics_torn(
                &mut body,
                shell,
                &format!("empty loop {emptied:?} is claimed by a half-edge"),
            );
        }
    }

    /// A pillow with an empty ring planted on one face and promoted by
    /// `mfkrh`: one shell, two components, the second a dartless face
    /// (an `Empty` outer loop) listed after the pillow's faces, so it
    /// seeds after the first component.
    fn pillow_with_a_promoted_empty_ring() -> Body<f64> {
        let tol = Tol::witness();
        let mut body = Body::<f64>::new();
        let seed = body.mvfs(p(0.0), true).unwrap();
        let seg = body
            .mev_line(
                MevSite::Lone {
                    r#loop: seed.r#loop,
                },
                p(1.0),
                tol,
            )
            .unwrap();
        body.mef_chord(
            MefSite::Chords {
                he1: seg.he_plus,
                he2: seg.he_minus,
            },
            tol,
        )
        .unwrap();
        let strut = body
            .mev_line(
                MevSite::Fan {
                    he1: seg.he_plus,
                    he2: seg.he_plus,
                },
                p(2.0),
                tol,
            )
            .unwrap();
        let planted = body.kemr(strut.he_plus, strut.he_minus).unwrap();
        let promoted = body.mfkrh_plug(planted.ring, true).unwrap();
        let shell = body.get_face(promoted.face).unwrap().shell;
        assert_eq!(
            body.get_shell(shell).unwrap().faces.last(),
            Some(&promoted.face),
            "the dartless face is listed last"
        );
        body
    }

    /// The valid bodies the labelling is measured on: one to three
    /// components, rings, struts, genus, and `Empty` loops with no
    /// half-edge in the shell (`mvfs_state`), beside half-edges that walk
    /// other loops of the same face (`ops_two_ring_face`'s tip ring,
    /// which the empty-loop proof must let through), and as the outer
    /// loop of a dartless face that seeds after another component.
    fn valid_fixtures() -> [(&'static str, Body<f64>); 15] {
        use crate::fixtures::{
            mvfs_state, ngon_pillow, ops_genus2, ops_holed_box, ops_ring_bridge, ops_strutted,
            ops_two_ring_face, pillow, raw_prism,
        };
        use crate::test_support_fixtures::geometric_cube;
        let tol = Tol::witness();
        [
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
            ("ops_two_ring_face", ops_two_ring_face(tol).body),
            ("one detached digon", detached_digons(1).0),
            ("two detached digons", detached_digons(2).0),
            (
                "pillow with a promoted empty ring",
                pillow_with_a_promoted_empty_ring(),
            ),
        ]
    }

    /// No over-refusal, and the shared walk's labels: on every valid
    /// body here ([`valid_fixtures`]), from each shell's face list and
    /// that list reversed, [`crate::glue::face_set_glue`] gives each face
    /// the label [`claimed_components`] reads off the half-edges' `edge`
    /// and `parent_loop` claims, numbered the same way, with no hop
    /// leaving the shell, and `movefac` partitions the shell exactly as
    /// those records do. An enumeration, not a sample.
    #[test]
    fn valid_fixtures_partition_as_their_records_do() {
        let mut most = 0;
        for (fixture, body) in valid_fixtures() {
            assert_eq!(validate(&body), Ok(()), "{fixture} is valid");
            for (shell, _) in body.shells() {
                let components = claimed_count(&body, shell);
                most = most.max(components);
                for reversed in [false, true] {
                    let mut trial = body.clone();
                    if reversed {
                        trial.get_shell_mut(shell).unwrap().faces.reverse();
                    }
                    let truth = claimed_components(&trial, shell);
                    let faces = trial.get_shell(shell).unwrap().faces.clone();
                    let glue = crate::glue::face_set_glue(
                        &trial,
                        &crate::glue::LoopClaims::of(&trial),
                        &faces,
                        EntityId::Shell(shell),
                    );
                    assert_eq!(
                        glue.leaving,
                        [],
                        "{fixture}: no hop leaves the shell (reversed {reversed})"
                    );
                    let labels: Vec<usize> = faces.iter().map(|&f| glue.labels[f]).collect();
                    let claimed: Vec<usize> = faces.iter().map(|&f| truth[f]).collect();
                    assert_eq!(
                        (labels, glue.count),
                        (claimed, components),
                        "{fixture}: each face's component label and the count (reversed \
                         {reversed})"
                    );
                    let shells = trial.movefac(shell).unwrap_or_else(|e| {
                        panic!("the valid {fixture} refuses {e:?} (reversed {reversed})")
                    });
                    assert_eq!(
                        shells.len(),
                        components,
                        "{fixture}: one shell per component (reversed {reversed})"
                    );
                    assert_eq!(
                        misread(&trial, &shells, &truth),
                        (false, false),
                        "{fixture}: the partition is the records' (reversed {reversed})"
                    );
                    assert_eq!(
                        validate(&trial),
                        Ok(()),
                        "{fixture}: tier 1 after movefac (reversed {reversed})"
                    );
                }
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
