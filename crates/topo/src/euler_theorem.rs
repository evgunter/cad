//! **The per-component Euler–Poincaré theorem, pinned.** On a body
//! that passes the validator's checks 1–7 and 10, every connected
//! component of a shell's incidence complex is a closed, connected,
//! orientable surface, so its census read with `s = 1` has an even
//! `v − e + f − r` and `h() ≥ 0`: its genus (`[topology.tiers]`). No
//! check repeats it; this module pins it, on every `seqgen` step,
//! over `movefac`'s fixture enumeration and over off-order loop
//! re-assignments.
//!
//! Why it holds. Check 4 makes a half-edge's end its `next`'s start,
//! so a vertex orbit carries one vertex; check 6 makes each vertex one
//! orbit, so its link is a circle. Checks 3 and 4 pair every half-edge
//! with an antiparallel mate, a closed, orientable gluing. A face with
//! `r` rings is a planar domain, `χ = 1 − r`; an empty loop is a point
//! (its vertex counted, its ring paid). Check 10 keeps each piece in
//! one shell, and faces glued by edge are connected exactly when the
//! surface is. So each component is a closed orientable surface:
//! `χ = 2 − 2g`, even and at most 2.
//!
//! The pin runs only in those tests, not inside every `validate()`; the χ content of `tests/review_m1_pr5.rs`'s
//! public-mutator sweep (every accepted mutation tier-1 valid) rests on
//! this theorem.
//!
//! Components are the shared glue relation's ([`face_set_glue`]); a
//! component's census counts the distinct `start` vertices and edges of
//! its faces' half-edges, its empty loops' vertices, its faces and its
//! faces' rings.

#![allow(clippy::expect_used)]

use slotmap::SecondaryMap;

use crate::body::Body;
use crate::entity::{EdgeKey, EntityId, FaceKey, LoopBoundary, ShellKey, VertexKey};
use crate::glue::{LoopClaims, face_set_glue};
use crate::readback::EulerCounts;

/// Each shell's connected components, as `(shell, seed, census)` with
/// `s = 1`: shells in arena order, a component seeded by its first face
/// in the shell's face list.
pub(crate) fn component_censuses(body: &Body<f64>) -> Vec<(ShellKey, FaceKey, EulerCounts)> {
    let mut vertices_of: SecondaryMap<FaceKey, Vec<VertexKey>> = SecondaryMap::new();
    let mut edges_of: SecondaryMap<FaceKey, Vec<EdgeKey>> = SecondaryMap::new();
    for (_, half) in body.half_edges() {
        let face = body.get_loop(half.parent_loop).expect("a half's loop").face;
        push(&mut vertices_of, face, half.start);
        push(&mut edges_of, face, half.edge);
    }
    for (_, loop_) in body.loops() {
        if let LoopBoundary::Empty { vertex } = loop_.boundary {
            push(&mut vertices_of, loop_.face, vertex);
        }
    }
    let claims = LoopClaims::of(body);
    let mut out = Vec::new();
    for (shell_key, shell) in body.shells() {
        let glue = face_set_glue(body, &claims, &shell.faces, EntityId::Shell(shell_key));
        for component in glue.components(&shell.faces) {
            let mut vertices: SecondaryMap<VertexKey, ()> = SecondaryMap::new();
            let mut edges: SecondaryMap<EdgeKey, ()> = SecondaryMap::new();
            let mut r = 0;
            for &face in &component {
                for &v in vertices_of.get(face).into_iter().flatten() {
                    vertices.insert(v, ());
                }
                for &e in edges_of.get(face).into_iter().flatten() {
                    edges.insert(e, ());
                }
                r += body.get_face(face).expect("a listed face").rings.len() as i64;
            }
            let census = EulerCounts {
                v: vertices.len() as i64,
                e: edges.len() as i64,
                f: component.len() as i64,
                r,
                s: 1,
            };
            out.push((shell_key, component[0], census));
        }
    }
    out
}

fn push<K: slotmap::Key, V>(map: &mut SecondaryMap<K, Vec<V>>, key: K, value: V) {
    match map.get_mut(key) {
        Some(list) => list.push(value),
        None => {
            map.insert(key, vec![value]);
        }
    }
}

/// The theorem on `body`: every component's census has even
/// `v − e + f − r` and `h() ≥ 0`. The `Err` names the first component
/// that breaks it.
pub(crate) fn check_component_theorem(body: &Body<f64>) -> Result<(), String> {
    for (shell, seed, census) in component_censuses(body) {
        match census.h() {
            Ok(h) if h >= 0 => {}
            Ok(h) => {
                return Err(format!(
                    "shell {shell:?}'s component at {seed:?} reads h = {h} < 0: {census:?}"
                ));
            }
            Err(refusal) => {
                return Err(format!(
                    "shell {shell:?}'s component at {seed:?}: {refusal}"
                ));
            }
        }
    }
    Ok(())
}

#[allow(clippy::unwrap_used)]
mod tests {
    use geom_core::Tol;

    use super::*;
    use crate::entity::Shell;
    use crate::fixtures::prov;
    use crate::test_support_fixtures::declined_cube;
    use crate::validate::{ValidationError, validate};

    /// The pin goes red where its premise fails: a cube with one side
    /// face moved into a second shell passes checks 1–7 and fails only
    /// check 10 (four `EdgeAcrossShells`), and each shell's piece then
    /// reads an odd `v − e + f − r` (8 − 12 + 5 on the five faces left,
    /// 4 − 4 + 1 on the moved one).
    #[test]
    fn a_face_moved_across_shells_reads_red() {
        let mut body = declined_cube(Tol::witness()).body;
        let (shell, solid, moved) = {
            let (key, shell) = body.shells().next().unwrap();
            (key, shell.solid, shell.faces[1])
        };
        let shell2 = body.add_shell(
            Shell {
                faces: vec![],
                solid,
            },
            prov(),
        );
        body.get_solid_mut(solid).unwrap().shells.push(shell2);
        body.get_face_mut(moved).unwrap().shell = shell2;
        body.get_shell_mut(shell)
            .unwrap()
            .faces
            .retain(|&k| k != moved);
        body.get_shell_mut(shell2).unwrap().faces.push(moved);
        let errors = validate(&body).unwrap_err();
        let crossings = errors
            .iter()
            .filter(|e| matches!(e, ValidationError::EdgeAcrossShells { .. }))
            .count();
        assert_eq!(crossings, 4, "the planted body breaks check 10");
        assert_eq!(errors.len(), 4, "and only check 10: {errors:?}");
        let chis: Vec<i64> = component_censuses(&body)
            .iter()
            .map(|(_, _, n)| n.v - n.e + n.f - n.r)
            .collect();
        assert_eq!(chis, vec![1, 1], "both pieces read odd");
        assert!(
            check_component_theorem(&body).is_err(),
            "the pin reads the planted body red"
        );
    }
}

#[allow(clippy::unwrap_used)]
mod reassignment {
    //! Loop re-assignment outside operator order: each step moves a ring
    //! between faces of one shell, swaps a face's outer with a ring,
    //! promotes a ring to a fresh face, or absorbs a face's loops into
    //! another face as rings (deleting the face). A mutant is kept only
    //! when the theorem's premise holds (checks 1–7 and 10 clean; 8, 9
    //! and 12 do not bear on it), and on each kept body the theorem must
    //! hold. The seeds are fixed: an enumeration of off-order bodies.
    use geom_core::Tol;

    use super::*;
    use crate::fixtures::{
        detached_digons, mvfs_state, ngon_pillow, ops_genus2, ops_holed_box, ops_ring_bridge,
        ops_strut_cube, ops_strutted, ops_two_ring_face, pillow, prov, raw_prism,
    };
    use crate::test_support_fixtures::declined_cube;
    use crate::validate::{ValidationError as E, validate};

    fn premise_holds(body: &Body<f64>) -> bool {
        match validate(body) {
            Ok(()) => true,
            Err(errs) => errs.iter().all(|e| {
                matches!(
                    e,
                    E::OrphanGeometry { .. }
                        | E::SolidWithoutShells { .. }
                        | E::ShellWithoutFaces { .. }
                        | E::MissingProvenance { .. }
                        | E::LeakedProvenance { .. }
                )
            }),
        }
    }

    struct Lcg(u64);
    impl Lcg {
        fn next(&mut self, n: usize) -> usize {
            self.0 = self
                .0
                .wrapping_mul(6364136223846793005)
                .wrapping_add(1442695040888963407);
            ((self.0 >> 33) as usize) % n.max(1)
        }
    }

    fn mutate(body: &mut Body<f64>, rng: &mut Lcg) -> bool {
        let faces: Vec<FaceKey> = body.faces().map(|(k, _)| k).collect();
        if faces.is_empty() {
            return false;
        }
        let a = faces[rng.next(faces.len())];
        let shell = body.get_face(a).unwrap().shell;
        let peers: Vec<FaceKey> = body
            .shells()
            .find(|(k, _)| *k == shell)
            .unwrap()
            .1
            .faces
            .clone();
        let b = peers[rng.next(peers.len())];
        match rng.next(4) {
            0 => {
                // move a ring of a to b
                let rings = body.get_face(a).unwrap().rings.clone();
                if rings.is_empty() || a == b {
                    return false;
                }
                let r = rings[rng.next(rings.len())];
                body.get_face_mut(a).unwrap().rings.retain(|&k| k != r);
                body.get_face_mut(b).unwrap().rings.push(r);
                body.get_loop_mut(r).unwrap().face = b;
            }
            1 => {
                // swap outer with a ring
                let f = body.get_face(a).unwrap().clone();
                if f.rings.is_empty() {
                    return false;
                }
                let i = rng.next(f.rings.len());
                let face = body.get_face_mut(a).unwrap();
                std::mem::swap(&mut face.rings[i], &mut face.outer);
            }
            2 => {
                // promote a ring of a to a fresh face in the shell
                let f = body.get_face(a).unwrap().clone();
                if f.rings.is_empty() {
                    return false;
                }
                let r = f.rings[rng.next(f.rings.len())];
                body.get_face_mut(a).unwrap().rings.retain(|&k| k != r);
                let mut g = f.clone();
                g.outer = r;
                g.rings = vec![];
                let gk = body.add_face(g, prov());
                body.get_loop_mut(r).unwrap().face = gk;
                body.get_shell_mut(shell).unwrap().faces.push(gk);
            }
            _ => {
                // absorb a: its outer and rings become rings of b; delete a
                if a == b {
                    return false;
                }
                let f = body.get_face(a).unwrap().clone();
                let mut absorbed = f.rings.clone();
                absorbed.push(f.outer);
                for l in absorbed {
                    body.get_loop_mut(l).unwrap().face = b;
                    body.get_face_mut(b).unwrap().rings.push(l);
                }
                body.get_shell_mut(shell).unwrap().faces.retain(|&k| k != a);
                body.faces.remove(a);
            }
        }
        true
    }

    type Maker = fn() -> Body<f64>;

    #[test]
    fn loop_reassignment_never_breaks_the_theorem() {
        let makers: [(&str, Maker); 12] = [
            ("declined_cube", || declined_cube(Tol::witness()).body),
            ("ops_strut_cube", || ops_strut_cube(Tol::witness()).body),
            ("ops_holed_box", || ops_holed_box(Tol::witness()).body),
            ("ops_genus2", || ops_genus2(Tol::witness())),
            ("ops_ring_bridge", || ops_ring_bridge(Tol::witness()).body),
            ("ops_strutted", || ops_strutted(Tol::witness()).0),
            ("pillow", || pillow(Tol::witness()).body),
            ("ngon_pillow5", || ngon_pillow(5, Tol::witness()).body),
            ("raw_prism3", || raw_prism(3, Tol::witness()).body),
            ("mvfs_state", || mvfs_state().body),
            ("ops_two_ring_face", || {
                ops_two_ring_face(Tol::witness()).body
            }),
            ("two detached digons", || detached_digons(2).0),
        ];
        let (mut kept, mut multi) = (0, 0);
        for (name, make) in &makers {
            for seed in 0..300_u64 {
                let mut rng = Lcg(seed.wrapping_mul(0x9E3779B97F4A7C15) ^ 0xD1B54A32D192ED03);
                let mut body = make();
                for step in 0..12 {
                    let mut trial = body.clone();
                    if !mutate(&mut trial, &mut rng) {
                        continue;
                    }
                    if !premise_holds(&trial) {
                        continue;
                    }
                    kept += 1;
                    body = trial;
                    let comps = component_censuses(&body);
                    if comps.len() > body.shells().count() {
                        multi += 1;
                    }
                    assert_eq!(
                        check_component_theorem(&body),
                        Ok(()),
                        "{name} seed {seed} step {step}: {comps:?}"
                    );
                }
            }
        }
        assert!(
            kept >= 10_000 && multi >= 100,
            "the enumeration keeps {kept} mutants, {multi} with a shell in pieces"
        );
    }
}

#[allow(clippy::unwrap_used)]
mod gate {
    //! Pass 11 counts components behind checks 1–7 only, so tier 2 still
    //! names a shell left in pieces beside an unrelated check-10 defect.
    use super::*;
    use crate::entity::Shell;
    use crate::fixtures::{detached_digons, prov};
    use crate::validate::{ValidationError as E, validate_closed};

    #[test]
    fn a_disconnected_shell_beside_a_cross_shell_edge() {
        let (mut body, shell, _seed, promoted) = detached_digons(2);
        let solid = body.get_shell(shell).unwrap().solid;
        let moved = promoted[1];
        let shell2 = body.add_shell(
            Shell {
                faces: vec![],
                solid,
            },
            prov(),
        );
        body.get_solid_mut(solid).unwrap().shells.push(shell2);
        body.get_face_mut(moved).unwrap().shell = shell2;
        body.get_shell_mut(shell)
            .unwrap()
            .faces
            .retain(|&k| k != moved);
        body.get_shell_mut(shell2).unwrap().faces.push(moved);
        let in_shell = component_censuses(&body)
            .iter()
            .filter(|(s, _, _)| *s == shell)
            .count();
        let errs = validate_closed(&body).unwrap_err();
        assert_eq!(in_shell, 3, "the old shell is in three pieces");
        assert!(
            errs.iter()
                .any(|e| matches!(e, E::ShellDisconnected { shell: s, .. } if *s == shell)),
            "tier 2 names the shell in pieces: {errs:?}"
        );
    }
}
