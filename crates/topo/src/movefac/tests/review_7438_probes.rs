//! Review probes for PR 7438: `face_set_glue` against the base's walks.

use super::*;
use crate::glue::face_set_glue;

/// The base's walk, restricted to `faces`: seeded in order, a face's
/// neighbours are the faces its cycles' members' mates' loops name
/// (base `seqgen::shell_component_faces` over a full shell, and base
/// `shell::count_components` / `face_neighbours` over a subset), a hop
/// out of the set not followed. Returns labels in set order, the
/// count, and the leaving hops as a sorted (from, to) list.
fn base_walk(body: &Body<f64>, faces: &[FaceKey]) -> (Vec<usize>, usize, Vec<(FaceKey, FaceKey)>) {
    let mut label: slotmap::SecondaryMap<FaceKey, usize> = slotmap::SecondaryMap::new();
    let mut leaving = Vec::new();
    let mut count = 0;
    for &seed in faces {
        if label.contains_key(seed) {
            continue;
        }
        label.insert(seed, count);
        let mut pending = vec![seed];
        while let Some(face) = pending.pop() {
            for he in body.proven_face(face).half_edges() {
                let mate = body.proven_mate(he, crate::live::link(EntityId::Face(face), "cycle"));
                let near = body.face_of_linked(mate.mate);
                if !faces.contains(&near) {
                    leaving.push((face, near));
                    continue;
                }
                if !label.contains_key(near) {
                    label.insert(near, count);
                    pending.push(near);
                }
            }
        }
        count += 1;
    }
    leaving.sort_unstable();
    (faces.iter().map(|&f| label[f]).collect(), count, leaving)
}

fn glue_walk(
    body: &Body<f64>,
    shell: ShellKey,
    faces: &[FaceKey],
) -> (Vec<usize>, usize, Vec<(FaceKey, FaceKey)>) {
    let g = face_set_glue(body, faces, EntityId::Shell(shell))
        .unwrap_or_else(|e| panic!("valid body torn: {e}"));
    let mut leaving: Vec<_> = g.leaving.iter().map(|h| (h.from, h.to)).collect();
    leaving.sort_unstable();
    (
        faces.iter().map(|&f| g.labels[f]).collect(),
        g.count,
        leaving,
    )
}

/// Subsets of `faces` (all when small, else a deterministic sample),
/// each in its own order, reversed, and rotated.
fn subsets(faces: &[FaceKey], salt: u64) -> Vec<Vec<FaceKey>> {
    let n = faces.len();
    let mut masks: Vec<u64> = Vec::new();
    if n <= 10 {
        masks.extend(1..(1_u64 << n));
    } else {
        let mut x = 0x9e37_79b9_7f4a_7c15_u64 ^ salt;
        for _ in 0..400 {
            x ^= x << 13;
            x ^= x >> 7;
            x ^= x << 17;
            masks.push(x | 1);
        }
        // every leave-one-out
        let full = if n >= 64 { u64::MAX } else { (1_u64 << n) - 1 };
        masks.push(full);
        for i in 0..n.min(64) {
            masks.push(full & !(1 << i));
        }
    }
    let mut out = Vec::new();
    for m in masks {
        let sub: Vec<FaceKey> = faces
            .iter()
            .enumerate()
            .filter(|(i, _)| *i >= 64 || m & (1 << i) != 0)
            .map(|(_, &f)| f)
            .collect();
        if sub.is_empty() {
            continue;
        }
        let mut rev = sub.clone();
        rev.reverse();
        let mut rot = sub.clone();
        rot.rotate_left(sub.len() / 2);
        out.extend([sub, rev, rot]);
    }
    out
}

fn compare(body: &Body<f64>, what: &str, salt: u64) -> usize {
    let mut checked = 0;
    for (shell, data) in body.shells() {
        for sub in subsets(&data.faces, salt) {
            let base = base_walk(body, &sub);
            let new = glue_walk(body, shell, &sub);
            assert_eq!(new, base, "{what}: shell {shell:?} subset {sub:?}");
            checked += 1;
        }
    }
    checked
}

/// Claims 1 and 2: on every valid fixture, every shell, every face
/// subset (all of them up to 10 faces) in three orders, the shared walk
/// labels, counts and records leaving hops exactly as the base walk.
#[test]
fn probe_glue_equals_base_walk_on_every_fixture_subset() {
    let mut checked = 0;
    for (i, (fixture, body)) in valid_fixtures().into_iter().enumerate() {
        checked += compare(&body, fixture, i as u64);
    }
    eprintln!("probe: {checked} subset comparisons");
    assert!(checked > 1000);
}

/// Claims 1 and 2 over the sequence generator's own bodies: 48 seeded
/// 40-step walks, every shell after every step, every subset.
#[test]
fn probe_glue_equals_base_walk_on_seqgen_states() {
    use crate::seqgen::{apply, choose_op};
    let mut checked = 0;
    let mut multi = 0;
    for seed in 0..48_u32 {
        let mut x = seed.wrapping_mul(2_654_435_761).wrapping_add(777);
        let mut roll = || {
            x = x.wrapping_mul(1_664_525).wrapping_add(1_013_904_223);
            x
        };
        let mut body = Body::<f64>::new();
        let mut counter = 0_u32;
        for step in 0..40 {
            let (d1, d2) = (roll(), roll());
            let Some(choice) = choose_op(&body, d1, d2, Tol::witness()) else {
                continue;
            };
            apply(&mut body, choice, &mut counter, Tol::witness());
            for (shell, data) in body.shells() {
                let full = glue_walk(&body, shell, &data.faces);
                if full.1 >= 2 {
                    multi += 1;
                }
            }
            checked += compare(&body, &format!("seed {seed} step {step}"), u64::from(seed));
        }
    }
    eprintln!("probe: {checked} seqgen subset comparisons, {multi} multi-component shells");
    assert!(multi > 0, "the walks reach a multi-component shell");
}
