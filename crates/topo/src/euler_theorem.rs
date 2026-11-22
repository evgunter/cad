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
                for loop_key in face.loop_keys() {
                    let loop_ = body.get_loop(loop_key).expect("a face's loop");
                    if let LoopBoundary::Empty { vertex } = loop_.boundary {
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
