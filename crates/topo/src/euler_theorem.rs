//! **The per-component Euler–Poincaré theorem, pinned.** On a body
//! that passes the validator's checks 1–7 and 10, every connected
//! component of a shell's incidence complex is a closed, connected,
//! orientable surface, so its census read with `s = 1` has an even
//! `v − e + f − r` and `h() ≥ 0` — its genus (`[topology.tiers]`).
//! The validator checks no such thing; this module is the test pin that
//! the checklist guarantees it, run over `movefac`'s fixture enumeration
//! and on every `seqgen` step.
//!
//! The component walk is this module's own, read straight off the
//! records: two faces of the shell glue when one edge has a half in a
//! loop of each. A component's census counts the distinct `start`
//! vertices and edges of its faces' half-edges, the lone vertex of each
//! empty loop, its faces, and its faces' rings.

#![allow(clippy::expect_used)]

use slotmap::SecondaryMap;

use crate::body::Body;
use crate::entity::{EdgeKey, FaceKey, LoopBoundary, ShellKey, VertexKey};
use crate::readback::EulerCounts;

/// Each shell's connected components, as `(shell, seed, census)` with
/// `s = 1`: shells in arena order, a component seeded by its first face
/// in the shell's face list.
pub(crate) fn component_censuses(body: &Body<f64>) -> Vec<(ShellKey, FaceKey, EulerCounts)> {
    let mut faces_of_edge: SecondaryMap<EdgeKey, Vec<FaceKey>> = SecondaryMap::new();
    for (_, half) in body.half_edges() {
        let face = body.get_loop(half.parent_loop).expect("a half's loop").face;
        match faces_of_edge.get_mut(half.edge) {
            Some(faces) => faces.push(face),
            None => {
                faces_of_edge.insert(half.edge, vec![face]);
            }
        }
    }
    let mut out = Vec::new();
    let mut seen: SecondaryMap<FaceKey, ()> = SecondaryMap::new();
    for (shell_key, shell) in body.shells() {
        for &seed in &shell.faces {
            if seen.insert(seed, ()).is_some() {
                continue;
            }
            let mut vertices: SecondaryMap<VertexKey, ()> = SecondaryMap::new();
            let mut edges: SecondaryMap<EdgeKey, ()> = SecondaryMap::new();
            let (mut f, mut r) = (0, 0);
            let mut pending = vec![seed];
            while let Some(face_key) = pending.pop() {
                let face = body.get_face(face_key).expect("a listed face");
                f += 1;
                r += face.rings.len() as i64;
                for (_, loop_) in body.loops() {
                    if let LoopBoundary::Empty { vertex } = loop_.boundary
                        && loop_.face == face_key
                    {
                        vertices.insert(vertex, ());
                    }
                }
                for (_, half) in body.half_edges() {
                    if body.get_loop(half.parent_loop).expect("a half's loop").face != face_key {
                        continue;
                    }
                    vertices.insert(half.start, ());
                    edges.insert(half.edge, ());
                    for &near in &faces_of_edge[half.edge] {
                        let in_shell =
                            body.get_face(near).expect("a glued face").shell == shell_key;
                        if in_shell && seen.insert(near, ()).is_none() {
                            pending.push(near);
                        }
                    }
                }
            }
            let census = EulerCounts {
                v: vertices.len() as i64,
                e: edges.len() as i64,
                f,
                r,
                s: 1,
            };
            out.push((shell_key, seed, census));
        }
    }
    out
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

#[cfg(test)]
#[allow(clippy::unwrap_used, clippy::expect_used)]
mod review_probe {
    //! Reviewer probe (PR 7430): loop re-assignment outside operator
    //! order. Each step moves a loop between faces of one shell, swaps a
    //! face's outer with a ring, promotes a ring to a fresh face, or
    //! absorbs a face's outer into another face as a ring (deleting the
    //! face). A mutant is kept only when passes 1–7 and 10 stay clean
    //! (8, 9, 12 errors are tolerated); on each kept body the theorem
    //! must hold.
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
            self.0 = self.0.wrapping_mul(6364136223846793005).wrapping_add(1442695040888963407);
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
        let peers: Vec<FaceKey> = body.shells().find(|(k, _)| *k == shell).unwrap().1.faces.clone();
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
                let r = face.rings[i];
                face.rings[i] = face.outer;
                face.outer = r;
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
                for l in std::iter::once(f.outer).chain(f.rings.iter().copied()) {
                    body.get_loop_mut(l).unwrap().face = b;
                    body.get_face_mut(b).unwrap().rings.push(l);
                }
                body.get_shell_mut(shell).unwrap().faces.retain(|&k| k != a);
                body.faces.remove(a);
            }
        }
        true
    }

    #[test]
    fn loop_reassignment_never_breaks_the_theorem() {
        let tol = Tol::witness();
        let makers: Vec<(&str, Box<dyn Fn() -> Body<f64>>)> = vec![
            ("declined_cube", Box::new(move || declined_cube(tol).body)),
            ("ops_strut_cube", Box::new(move || ops_strut_cube(tol).body)),
            ("ops_holed_box", Box::new(move || ops_holed_box(tol).body)),
            ("ops_genus2", Box::new(move || ops_genus2(tol))),
            ("ops_ring_bridge", Box::new(move || ops_ring_bridge(tol).body)),
            ("ops_strutted", Box::new(move || ops_strutted(tol).0)),
            ("pillow", Box::new(move || pillow(tol).body)),
            ("ngon_pillow5", Box::new(move || ngon_pillow(5, tol).body)),
            ("raw_prism3", Box::new(move || raw_prism(3, tol).body)),
            ("mvfs_state", Box::new(|| mvfs_state().body)),
            ("ops_two_ring_face", Box::new(move || ops_two_ring_face(tol).body)),
            ("two detached digons", Box::new(|| detached_digons(2).0)),
        ];
        let (mut kept, mut tried, mut multi) = (0, 0, 0);
        for (name, make) in &makers {
            for seed in 0..300_u64 {
                let mut rng = Lcg(seed.wrapping_mul(0x9E3779B97F4A7C15) ^ 0xD1B54A32D192ED03);
                let mut body = make();
                for step in 0..12 {
                    let mut trial = body.clone();
                    if !mutate(&mut trial, &mut rng) {
                        continue;
                    }
                    tried += 1;
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
        eprintln!("probe: tried {tried}, kept {kept}, multi-component {multi}");
        assert!(kept > 100, "the probe exercised something: {kept}");
    }
}

#[cfg(test)]
#[allow(clippy::unwrap_used, clippy::expect_used)]
mod review_probe_gate {
    //! Reviewer probe (PR 7430, claim 2): a shell that is genuinely
    //! disconnected (c = 3) beside an unrelated pass-10 defect.
    use super::*;
    use crate::entity::Shell;
    use crate::fixtures::{detached_digons, prov};
    use crate::validate::{ValidationError as E, validate_closed};

    #[test]
    fn a_disconnected_shell_beside_a_cross_shell_edge() {
        let (mut body, shell, _seed, promoted) = detached_digons(2);
        let solid = body.get_shell(shell).unwrap().solid;
        let moved = promoted[1];
        let shell2 = body.add_shell(Shell { faces: vec![], solid }, prov());
        body.get_solid_mut(solid).unwrap().shells.push(shell2);
        body.get_face_mut(moved).unwrap().shell = shell2;
        body.get_shell_mut(shell).unwrap().faces.retain(|&k| k != moved);
        body.get_shell_mut(shell2).unwrap().faces.push(moved);
        let in_shell = component_censuses(&body)
            .iter()
            .filter(|(s, _, _)| *s == shell)
            .count();
        let errs = validate_closed(&body).unwrap_err();
        eprintln!("probe: components in the old shell = {in_shell}; tier 2 = {errs:?}");
        let disconnected = errs.iter().any(|e| matches!(e, E::ShellDisconnected { .. }));
        assert!(in_shell >= 2, "the old shell is genuinely disconnected");
        assert!(disconnected, "tier 2 names the disconnected shell");
    }
}
