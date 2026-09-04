//! Adversarial e2e review artifact for M1 PR 4 (2026-07-16), promoted
//! into the shipped suite per the standing convention
//! (`memories/review-and-dependency-policy.md`): reviewers write and run
//! real consumer programs against the API under review, and the
//! programs are kept.
//!
//! Unlike PR 3's artifact this one lives in `src/` (cfg(test)) rather
//! than `tests/`: several probes attack the pub(crate) test-support
//! machinery itself — the isomorphism oracle ([`crate::iso`]) and the
//! sequence generator ([`crate::seqgen`]) — which integration tests
//! cannot reach.
//!
//! The derivations (splice geometry, the single-op re-make taxonomy,
//! the shell-component formula, orbit orders) are **independent
//! re-derivations by the reviewer — do not "simplify" them to match the
//! implementation's comments**; their value is exactly that they were
//! computed from the pinned PR 1/2/3 surgeries without reading the
//! kill-op code. Highlights: the both-ends asymmetric valence-5 kev
//! (the strongest direction pin in the tree), the falsification of the
//! first-draft "mate-alone kef is irreversible" claim (it IS one-op
//! re-makeable when the survivor is a bare outer — the fix pass encoded
//! the corrected taxonomy), exhaustive single-op-search proofs for the
//! two genuinely irreversible kill subcases, the shell-component
//! Euler–Poincaré formula that PR 5's validator now implements as
//! pass 11 (this file's derivation was its spec seed), oracle
//! automorphism/degeneracy attacks, a genus-2 teardown, and
//! release-mode torn-body batteries.
//!
//! Only lint fixes and the promotion of the corrected re-make taxonomy
//! were applied at promotion; the probes are otherwise verbatim.

#![allow(clippy::unwrap_used, clippy::expect_used, clippy::panic)]

test_utils::gated_to![
    "crates/topo/src/euler.rs",
    "crates/topo/src/euler_ring.rs",
    "crates/topo/src/euler_kill.rs",
    "crates/topo/src/body.rs",
    "crates/topo/src/entity.rs",
    "crates/topo/src/validate.rs",
    "crates/topo/src/iso.rs",
    "crates/topo/src/seqgen.rs",
    "crates/topo/src/fixtures.rs",
];

use geom_core::Point3;

use crate::body::Body;
use crate::entity::{EntityId, FaceKey, HalfEdgeKey, LoopBoundary, LoopKey, VertexKey};
use crate::euler::{EulerOpError, MefSite, MevCreated, MevSite, MvfsCreated};
use crate::euler_ring::MekrSite;
use crate::fixtures::{deep_snapshot, ops_cube};
use crate::iso::{canonical_form, isomorphic};
use crate::seqgen;
use crate::validate::validate;
use geom_core::Tol;

fn pt(x: f64, y: f64, z: f64) -> Point3<f64> {
    Point3::new(x, y, z)
}

fn p(x: f64) -> Point3<f64> {
    pt(x, 0.0, 0.0)
}

/// mvfs + mev(Lone): the segment body.
fn segment(tol: Tol) -> (Body<f64>, MvfsCreated, MevCreated) {
    let mut body = Body::<f64>::new();
    let seed = body.mvfs(p(0.0)).unwrap();
    let seg = body
        .mev_line(
            MevSite::Lone {
                r#loop: seed.r#loop,
            },
            p(1.0),
            tol,
        )
        .unwrap();
    (body, seed, seg)
}

/// Five-spoke star: central vertex with clockwise orbit
/// [a+, b+, c+, d+, e+], asymmetric (all tips at distinct coords).
fn five_spoke_star(tol: Tol) -> (Body<f64>, MvfsCreated, [MevCreated; 5]) {
    let mut body = Body::<f64>::new();
    let seed = body.mvfs(p(0.0)).unwrap();
    let a = body
        .mev_line(
            MevSite::Lone {
                r#loop: seed.r#loop,
            },
            p(1.0),
            tol,
        )
        .unwrap();
    let strut_at = |body: &mut Body<f64>, x: f64| {
        body.mev_line(
            MevSite::Fan {
                he1: a.he_plus,
                he2: a.he_plus,
            },
            p(x),
            tol,
        )
        .unwrap()
    };
    let b = strut_at(&mut body, 2.0);
    let c = strut_at(&mut body, 3.0);
    let d = strut_at(&mut body, 4.0);
    let e = strut_at(&mut body, 5.0);
    assert_eq!(
        body.vertex_orbit(a.he_plus),
        Some(vec![a.he_plus, b.he_plus, c.he_plus, d.he_plus, e.he_plus])
    );
    (body, seed, [a, b, c, d, e])
}

// =====================================================================
// 1. Independent splice checks: make ∘ kill for EVERY site case,
//    canonical identity (deep identity where anchors genuinely restore).
// =====================================================================

#[test]
fn mk_kill_roundtrip_every_mev_site_case() {
    let tol = Tol::witness();
    // Lone (segment): DEEP identity — the pre-state had emanating None
    // and an Empty loop, which the segment kill restores exactly, and
    // the balanced pair leaves survivor keys untouched.
    let mut b3 = Body::<f64>::new();
    let seed3 = b3.mvfs(p(0.0)).unwrap();
    let deep3 = deep_snapshot(&b3);
    let created = b3
        .mev_line(
            MevSite::Lone {
                r#loop: seed3.r#loop,
            },
            p(1.0),
            tol,
        )
        .unwrap();
    b3.kev(created.he_plus).unwrap();
    assert_eq!(deep_snapshot(&b3), deep3, "Lone mev∘kev deep identity");

    // Fan strut (he1 == he2): canonical identity.
    let (mut body, _seed, [a, _b, _c, _d, _e]) = five_spoke_star(tol);
    let before = canonical_form(&body);
    let strut = body
        .mev_line(
            MevSite::Fan {
                he1: a.he_plus,
                he2: a.he_plus,
            },
            p(9.0),
            tol,
        )
        .unwrap();
    body.kev(strut.he_plus).unwrap();
    assert_eq!(validate(&body), Ok(()));
    assert_eq!(canonical_form(&body), before, "strut mev∘kev");

    // Fan asymmetric valence-5, non-wrapping run [b .. d) = {b, c}.
    let (mut body, _seed, [_a, b, _c, d, _e]) = five_spoke_star(tol);
    let before = canonical_form(&body);
    let split = body
        .mev_line(
            MevSite::Fan {
                he1: b.he_plus,
                he2: d.he_plus,
            },
            p(9.0),
            tol,
        )
        .unwrap();
    body.kev(split.he_plus).unwrap();
    assert_eq!(validate(&body), Ok(()));
    assert_eq!(canonical_form(&body), before, "v5 fan mev∘kev");

    // Fan WRAPPING run [d .. b) = {d, e, a} (wraps past the orbit
    // anchor).
    let (mut body, _seed, [_a, b, _c, d, _e]) = five_spoke_star(tol);
    let before = canonical_form(&body);
    let split = body
        .mev_line(
            MevSite::Fan {
                he1: d.he_plus,
                he2: b.he_plus,
            },
            p(9.0),
            tol,
        )
        .unwrap();
    body.kev(split.he_plus).unwrap();
    assert_eq!(validate(&body), Ok(()));
    assert_eq!(canonical_form(&body), before, "v5 wrapping fan mev∘kev");

    // Fan cross-loop: he1/he2 in different loops (digon pillow seed
    // vertex).
    let (mut body, _seed, seg) = segment(tol);
    let split_faces = body
        .mef_chord(
            MefSite::Chords {
                he1: seg.he_plus,
                he2: seg.he_minus,
            },
            tol,
        )
        .unwrap();
    let before = canonical_form(&body);
    let fan = body
        .mev_line(
            MevSite::Fan {
                he1: seg.he_plus,
                he2: split_faces.he_plus,
            },
            p(9.0),
            tol,
        )
        .unwrap();
    body.kev(fan.he_plus).unwrap();
    assert_eq!(validate(&body), Ok(()));
    assert_eq!(canonical_form(&body), before, "cross-loop fan mev∘kev");
}

#[test]
fn mk_kill_roundtrip_every_mef_site_case() {
    let tol = Tol::witness();
    // Chords general (valence > minimal: cut a cube face corner to
    // corner).
    let t = ops_cube(tol);
    let mut body = t.body;
    let before = canonical_form(&body);
    // two half-edges of the top face's loop, two apart
    let top_loop = body.get_face(t.mefs[1].face).unwrap().outer;
    let LoopBoundary::Cycle { first } = body.get_loop(top_loop).unwrap().boundary else {
        panic!("cycle");
    };
    let cycle = body.loop_cycle(first).unwrap();
    let cut = body
        .mef_chord(
            MefSite::Chords {
                he1: cycle[0],
                he2: cycle[2],
            },
            tol,
        )
        .unwrap();
    body.kef(cut.he_minus).unwrap();
    assert_eq!(validate(&body), Ok(()));
    assert_eq!(canonical_form(&body), before, "Chords general mef∘kef");

    // Chords he1 == he2 (circular one-edge face).
    let (mut body, _seed, seg) = segment(tol);
    let before = canonical_form(&body);
    let circ = body
        .mef_chord(
            MefSite::Chords {
                he1: seg.he_plus,
                he2: seg.he_plus,
            },
            tol,
        )
        .unwrap();
    body.kef(circ.he_minus).unwrap();
    assert_eq!(validate(&body), Ok(()));
    assert_eq!(canonical_form(&body), before, "circular mef∘kef");

    // Lone (self-loop pair at the lone vertex).
    let mut body = Body::<f64>::new();
    let seed = body.mvfs(p(0.0)).unwrap();
    let before = deep_snapshot(&body);
    let circ = body
        .mef_chord(
            MefSite::Lone {
                r#loop: seed.r#loop,
            },
            tol,
        )
        .unwrap();
    body.kef(circ.he_minus).unwrap();
    assert_eq!(validate(&body), Ok(()));
    assert_eq!(deep_snapshot(&body), before, "Lone mef∘kef deep identity");

    // Ring-loop split: mef with both chords in a RING loop; the new
    // face's outer is he1's side; kef(he_minus) undoes it.
    let (mut body, ring) = body_with_cycle_ring(tol);
    let before = canonical_form(&body);
    let LoopBoundary::Cycle { first } = body.get_loop(ring).unwrap().boundary else {
        panic!("cycle ring");
    };
    let rc = body.loop_cycle(first).unwrap();
    assert!(rc.len() >= 2);
    let cut = body
        .mef_chord(
            MefSite::Chords {
                he1: rc[0],
                he2: rc[1],
            },
            tol,
        )
        .unwrap();
    body.kef(cut.he_minus).unwrap();
    assert_eq!(validate(&body), Ok(()));
    assert_eq!(canonical_form(&body), before, "ring-split mef∘kef");
}

/// Pillow with a CYCLE ring: plant an empty ring, then grow it into a
/// two-half segment cycle with mev(Lone).
fn body_with_cycle_ring(tol: Tol) -> (Body<f64>, LoopKey) {
    let (mut body, _seed, seg) = segment(tol);
    let _faces = body
        .mef_chord(
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
    let kill = body.kemr(strut.he_plus, strut.he_minus).unwrap();
    body.mev_line(MevSite::Lone { r#loop: kill.ring }, p(3.0), tol)
        .unwrap();
    assert_eq!(validate(&body), Ok(()));
    (body, kill.ring)
}

// =====================================================================
// 2. Orbit-direction slip hunt: asymmetric valence-5 configuration
//    killed from BOTH ends of the edge.
// =====================================================================

#[test]
fn kev_from_both_ends_of_an_asymmetric_valence_five_split() {
    let tol = Tol::witness();
    // Split the 5-star fan [a,b,c,d,e] at Fan{b, d}: run {b, c} moves to
    // the new vertex w. Killing from he_plus returns {b,c} to v;
    // killing from he_minus instead kills v and must deliver the
    // REMAINING fan {d, e, a} to w in unchanged clockwise cyclic order.
    let (mut body, seed, [a, b, c, d, e]) = five_spoke_star(tol);
    let before = canonical_form(&body);
    let split = body
        .mev_line(
            MevSite::Fan {
                he1: b.he_plus,
                he2: d.he_plus,
            },
            p(9.0),
            tol,
        )
        .unwrap();
    // End 1: kill the new vertex.
    {
        let mut probe = body.clone();
        let result = probe.kev(split.he_plus).unwrap();
        assert_eq!(validate(&probe), Ok(()));
        assert_eq!(result.killed_vertex, split.vertex);
        assert_eq!(canonical_form(&probe), before);
    }
    // End 2: kill the OLD center v; w inherits everything.
    let result = body.kev(split.he_minus).unwrap();
    assert_eq!(validate(&body), Ok(()));
    assert_eq!(result.killed_vertex, seed.vertex);
    let start = |body: &Body<f64>, he| body.get_half_edge(he).unwrap().start;
    for spoke in [a.he_plus, b.he_plus, c.he_plus, d.he_plus, e.he_plus] {
        assert_eq!(start(&body, spoke), split.vertex, "spoke moved to w");
    }
    // THE direction pin: the clockwise cyclic order of the five spokes
    // at w must be the original [a, b, c, d, e] (as a cyclic sequence).
    // A direction slip in the fan merge would scramble it (e.g.
    // reversed or with {d,e,a} spliced before {b,c}).
    let orbit = body.vertex_orbit(b.he_plus).unwrap();
    assert_eq!(orbit.len(), 5);
    let names: Vec<&str> = orbit
        .iter()
        .map(|&he| {
            if he == a.he_plus {
                "a"
            } else if he == b.he_plus {
                "b"
            } else if he == c.he_plus {
                "c"
            } else if he == d.he_plus {
                "d"
            } else if he == e.he_plus {
                "e"
            } else {
                "?"
            }
        })
        .collect();
    assert_eq!(names, vec!["b", "c", "d", "e", "a"], "cyclic order kept");
}

// =====================================================================
// 3. Falsifying the "no single-op re-make" claim.
// =====================================================================

/// Enumerates EVERY single-op body one op away from `body` (all mev/mef
/// sites with the given candidate coordinates, all mekr sites, all
/// mfkrh rings, mvfs at the coords) and returns true iff any of them is
/// isomorphic to `target`.
fn some_single_op_reaches(
    body: &Body<f64>,
    target: &str,
    coords: &[Point3<f64>],
    tol: Tol,
) -> bool {
    let mut candidates: Vec<Body<f64>> = Vec::new();
    // mev Fan (all orbit pairs incl. he1==he2), all candidate coords.
    let halves: Vec<HalfEdgeKey> = body.half_edges().map(|(k, _)| k).collect();
    for &he1 in &halves {
        let orbit = body.vertex_orbit(he1).unwrap();
        for &he2 in &orbit {
            for &c in coords {
                let mut probe = body.clone();
                if probe.mev_line(MevSite::Fan { he1, he2 }, c, tol).is_ok() {
                    candidates.push(probe);
                }
            }
        }
    }
    // mev Lone / mef Lone on every empty loop.
    let empties: Vec<LoopKey> = body
        .loops()
        .filter(|(_, l)| matches!(l.boundary, LoopBoundary::Empty { .. }))
        .map(|(k, _)| k)
        .collect();
    for &l in &empties {
        for &c in coords {
            let mut probe = body.clone();
            if probe.mev_line(MevSite::Lone { r#loop: l }, c, tol).is_ok() {
                candidates.push(probe);
            }
        }
        let mut probe = body.clone();
        if probe.mef_chord(MefSite::Lone { r#loop: l }, tol).is_ok() {
            candidates.push(probe);
        }
    }
    // mef Chords: all same-loop pairs incl. he1==he2.
    for (_, loop_data) in body.loops() {
        let LoopBoundary::Cycle { first } = loop_data.boundary else {
            continue;
        };
        let cycle = body.loop_cycle(first).unwrap();
        for &he1 in &cycle {
            for &he2 in &cycle {
                let mut probe = body.clone();
                if probe.mef_chord(MefSite::Chords { he1, he2 }, tol).is_ok() {
                    candidates.push(probe);
                }
            }
        }
    }
    // mekr: all four site shapes over all loop pairs of each face.
    for (_, face) in body.faces() {
        let loops: Vec<LoopKey> = core::iter::once(face.outer)
            .chain(face.rings.iter().copied())
            .collect();
        for &target_l in &loops {
            for &ring in &face.rings {
                if target_l == ring {
                    continue;
                }
                let tb = body.get_loop(target_l).unwrap().boundary;
                let rb = body.get_loop(ring).unwrap().boundary;
                let sites: Vec<MekrSite> = match (tb, rb) {
                    (LoopBoundary::Cycle { first: tf }, LoopBoundary::Cycle { first: rf }) => {
                        let tc = body.loop_cycle(tf).unwrap();
                        let rc = body.loop_cycle(rf).unwrap();
                        tc.iter()
                            .flat_map(|&t| {
                                rc.iter()
                                    .map(move |&r| MekrSite::Cycles { target: t, ring: r })
                            })
                            .collect()
                    }
                    (LoopBoundary::Cycle { first: tf }, LoopBoundary::Empty { .. }) => body
                        .loop_cycle(tf)
                        .unwrap()
                        .into_iter()
                        .map(|t| MekrSite::EmptyRing { target: t, ring })
                        .collect(),
                    (LoopBoundary::Empty { .. }, LoopBoundary::Cycle { first: rf }) => body
                        .loop_cycle(rf)
                        .unwrap()
                        .into_iter()
                        .map(|r| MekrSite::EmptyTarget {
                            target: target_l,
                            ring: r,
                        })
                        .collect(),
                    (LoopBoundary::Empty { .. }, LoopBoundary::Empty { .. }) => {
                        vec![MekrSite::BothEmpty {
                            target: target_l,
                            ring,
                        }]
                    }
                };
                for site in sites {
                    let mut probe = body.clone();
                    if probe.mekr_chord(site, tol).is_ok() {
                        candidates.push(probe);
                    }
                }
            }
        }
    }
    // mfkrh on every ring; kfmrh on every face pair; mvfs.
    let rings: Vec<LoopKey> = body.faces().flat_map(|(_, f)| f.rings.clone()).collect();
    for ring in rings {
        let mut probe = body.clone();
        if probe.mfkrh_plug(ring).is_ok() {
            candidates.push(probe);
        }
    }
    let faces: Vec<FaceKey> = body.faces().map(|(k, _)| k).collect();
    for &f1 in &faces {
        for &f2 in &faces {
            let mut probe = body.clone();
            if probe.kfmrh(f1, f2).is_ok() {
                candidates.push(probe);
            }
        }
    }
    for &c in coords {
        let mut probe = body.clone();
        if probe.mvfs(c).is_ok() {
            candidates.push(probe);
        }
    }
    candidates
        .iter()
        .any(|candidate| canonical_form(candidate) == target)
}

#[test]
fn kef_mate_alone_has_a_single_op_remake_when_survivor_is_bare_outer() {
    let tol = Tol::witness();
    // THE FALSIFICATION ATTEMPT for skip case (b). Circular-edge config:
    // face A = big loop [seg+, circ+, seg−], face B outer = [circ−]
    // alone. kef(circ.he_plus) kills the BIG side (mate's loop [m]
    // alone). Claim under test: no single op re-makes the pre-kill
    // body. Candidate counter-move: mef(Chords{b, b}) with
    // b = next(killed he) — it should splice a new plus half into the
    // exact gap and hang a fresh one-half circular face off it, which
    // is the pre-kill structure up to face identity (oracle-equal).
    let (mut body, _seed, seg) = segment(tol);
    let circ = body
        .mef_chord(
            MefSite::Chords {
                he1: seg.he_minus,
                he2: seg.he_minus,
            },
            tol,
        )
        .unwrap();
    let before = canonical_form(&body);
    let b = body.get_half_edge(circ.he_plus).unwrap().next;
    body.kef(circ.he_plus).unwrap();
    assert_eq!(validate(&body), Ok(()));
    // The single-op re-make:
    let remake = body
        .mef_chord(MefSite::Chords { he1: b, he2: b }, tol)
        .unwrap();
    assert_eq!(validate(&body), Ok(()));
    assert_eq!(
        canonical_form(&body),
        before,
        "single mef re-made the 'irreversible' mate-alone kef kill"
    );
    let _ = remake;
}

#[test]
fn kef_mate_alone_with_ring_on_survivor_has_no_single_op_remake() {
    let tol = Tol::witness();
    // The subcase where the claim DOES hold: the surviving singleton
    // loop's face carries a ring, so the re-make puts the big loop on
    // the ringed face — wrong body. Exhaustive single-op search.
    let (mut body, _seed, seg) = segment(tol);
    let circ = body
        .mef_chord(
            MefSite::Chords {
                he1: seg.he_minus,
                he2: seg.he_minus,
            },
            tol,
        )
        .unwrap();
    // Plant an empty ring on the circular face B (via strut + kemr in
    // B's loop [circ−]).
    let strut = body
        .mev_line(
            MevSite::Fan {
                he1: circ.he_minus,
                he2: circ.he_minus,
            },
            p(5.0),
            tol,
        )
        .unwrap();
    body.kemr(strut.he_plus, strut.he_minus).unwrap();
    assert_eq!(validate(&body), Ok(()));
    let before = canonical_form(&body);
    let mut after = body.clone();
    after.kef(circ.he_plus).unwrap();
    assert_eq!(validate(&after), Ok(()));
    let coords = [p(0.0), p(1.0), p(5.0)];
    assert!(
        !some_single_op_reaches(&after, &before, &coords, tol),
        "found an unexpected single-op re-make"
    );
}

#[test]
fn kev_mirror_has_no_single_op_remake() {
    let tol = Tol::witness();
    // Skip case (a): kev from the valence-1 side of an edge whose far
    // vertex carries a fan. Exhaustive single-op search over the
    // post-kill body, trying every site and every plausibly-relevant
    // coordinate (including the killed vertex's).
    let (mut body, seed, seg) = segment(tol);
    let strut = body
        .mev_line(
            MevSite::Fan {
                he1: seg.he_minus,
                he2: seg.he_minus,
            },
            p(2.0),
            tol,
        )
        .unwrap();
    // seg.vertex carries fan [seg−, strut+]; strut.he_minus starts at
    // the valence-1 tip and points at it — the mirror kill.
    let before = canonical_form(&body);
    body.kev(strut.he_minus).unwrap();
    assert_eq!(validate(&body), Ok(()));
    let coords = [p(0.0), p(1.0), p(2.0)];
    assert!(
        !some_single_op_reaches(&body, &before, &coords, tol),
        "found an unexpected single-op re-make of the mirror kev"
    );
    let _ = seed;
}

// =====================================================================
// 4. mfkrh on a non-handle ring: the negative-genus finding, derived
//    precisely (per-shell component-aware Euler–Poincaré).
// =====================================================================

/// Connected components of a shell's incidence complex: faces glue all
/// their loops (outer + rings); a cycle loop glues its darts' start
/// vertices and (via mate) its edges' other sides; an empty loop glues
/// its lone vertex. Components are maximal under "shares a vertex, edge,
/// or face".
fn shell_components(body: &Body<f64>, shell: crate::entity::ShellKey) -> Vec<ShellComponent> {
    use std::collections::{HashMap, HashSet, VecDeque};
    let shell_data = body.get_shell(shell).unwrap();
    let faces: Vec<FaceKey> = shell_data.faces.clone();
    // face -> vertices it touches
    let mut face_vertices: HashMap<FaceKey, Vec<VertexKey>> = HashMap::new();
    for &face in &faces {
        let face_data = body.get_face(face).unwrap();
        let mut vs = Vec::new();
        for l in core::iter::once(face_data.outer).chain(face_data.rings.iter().copied()) {
            match body.get_loop(l).unwrap().boundary {
                LoopBoundary::Empty { vertex } => vs.push(vertex),
                LoopBoundary::Cycle { first } => {
                    for he in body.loop_cycle(first).unwrap() {
                        vs.push(body.get_half_edge(he).unwrap().start);
                    }
                }
            }
        }
        face_vertices.insert(face, vs);
    }
    // vertex -> faces
    let mut vertex_faces: HashMap<VertexKey, Vec<FaceKey>> = HashMap::new();
    for (&face, vs) in &face_vertices {
        for &v in vs {
            vertex_faces.entry(v).or_default().push(face);
        }
    }
    let mut seen: HashSet<FaceKey> = HashSet::new();
    let mut components = Vec::new();
    for &start in &faces {
        if seen.contains(&start) {
            continue;
        }
        let mut comp_faces: HashSet<FaceKey> = HashSet::new();
        let mut comp_vertices: HashSet<VertexKey> = HashSet::new();
        let mut queue = VecDeque::from([start]);
        seen.insert(start);
        while let Some(face) = queue.pop_front() {
            comp_faces.insert(face);
            for &v in &face_vertices[&face] {
                comp_vertices.insert(v);
                for &next_face in &vertex_faces[&v] {
                    if seen.insert(next_face) {
                        queue.push_back(next_face);
                    }
                }
            }
        }
        // Count v, e, f, r inside the component.
        let mut comp_edges: HashSet<crate::entity::EdgeKey> = HashSet::new();
        let mut rings = 0usize;
        for &face in &comp_faces {
            let face_data = body.get_face(face).unwrap();
            rings += face_data.rings.len();
            for l in core::iter::once(face_data.outer).chain(face_data.rings.iter().copied()) {
                if let LoopBoundary::Cycle { first } = body.get_loop(l).unwrap().boundary {
                    for he in body.loop_cycle(first).unwrap() {
                        comp_edges.insert(body.get_half_edge(he).unwrap().edge);
                    }
                }
            }
        }
        components.push(ShellComponent {
            v: comp_vertices.len() as i64,
            e: comp_edges.len() as i64,
            f: comp_faces.len() as i64,
            r: rings as i64,
        });
    }
    components
}

struct ShellComponent {
    v: i64,
    e: i64,
    f: i64,
    r: i64,
}

impl ShellComponent {
    /// v − e + f − r = 2(1 − g) per connected component of a shell;
    /// returns g and asserts it is a non-negative integer.
    fn genus(&self) -> i64 {
        let chi = self.v - self.e + self.f - self.r;
        assert_eq!(chi.rem_euclid(2), 0, "component χ parity");
        let g = (2 - chi) / 2;
        assert!(g >= 0, "component genus must be ≥ 0, got {g}");
        g
    }
}

#[test]
fn mfkrh_on_a_planted_ring_disconnects_the_shell_not_negative_genus() {
    let tol = Tol::witness();
    // Pillow + planted empty ring: v3 e2 f2 r1, h = 0.
    let (mut body, _seed, seg) = segment(tol);
    let _faces = body
        .mef_chord(
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
    let kill = body.kemr(strut.he_plus, strut.he_minus).unwrap();
    let shell = body.shells().next().unwrap().0;
    assert_eq!(shell_components(&body, shell).len(), 1, "still connected");

    // Promote the planted (non-handle) ring: the naive per-body genus
    // 2h = 2s − (v − e + f − r) goes NEGATIVE...
    body.mfkrh_plug(kill.ring).unwrap();
    assert_eq!(validate(&body), Ok(()), "tier-1 accepts the result");
    let v = body.vertices().count() as i64;
    let e = body.edges().count() as i64;
    let f = body.faces().count() as i64;
    let r: i64 = body.faces().map(|(_, fd)| fd.rings.len() as i64).sum();
    let s = body.solids().count() as i64;
    let twice_h = 2 * s - (v - e + f - r);
    assert_eq!(twice_h, -2, "naive derived h = -1 (the finding)");

    // ...because the SHELL's surface disconnected: two components, each
    // a genus-0 closed piece (the pillow, and the lone-vertex+disk
    // point-sphere), one shell entity. The component-aware statement
    // v − e + f − r = 2(c − Σg) holds exactly.
    let components = shell_components(&body, shell);
    assert_eq!(components.len(), 2, "the dart/incidence graph split");
    let chi_sum: i64 = components.iter().map(|c| c.v - c.e + c.f - c.r).sum();
    let genus_sum: i64 = components.iter().map(ShellComponent::genus).sum();
    assert_eq!(
        chi_sum,
        2 * (components.len() as i64 - genus_sum),
        "per-shell component-aware Euler–Poincaré"
    );
    assert_eq!(v - e + f - r, chi_sum, "shell totals = component sums");
    assert_eq!(genus_sum, 0);
}

#[test]
fn component_formula_holds_on_reference_bodies() {
    let tol = Tol::witness();
    // Cube: c = 1, g = 0.
    let t = ops_cube(tol);
    let shell = t.body.shells().next().unwrap().0;
    let comps = shell_components(&t.body, shell);
    assert_eq!(comps.len(), 1);
    assert_eq!(comps[0].genus(), 0);
    // Holed box: c = 1, g = 1 (the rings are ATTACHED via the tube, so
    // mfkrh on the plug ring keeps the shell connected — the h drop is
    // genuine genus there).
    let t = crate::fixtures::ops_holed_box(tol);
    let shell = t.body.shells().next().unwrap().0;
    let comps = shell_components(&t.body, shell);
    assert_eq!(comps.len(), 1);
    assert_eq!(comps[0].genus(), 1);
}

// =====================================================================
// 5. Oracle attacks.
// =====================================================================

/// An n-gon pillow (two faces sharing an n-cycle): mvfs at pts[0], grow
/// a chain of n−1 mevs, close with one mef.
fn ngon_pillow(pts: &[Point3<f64>], tol: Tol) -> Body<f64> {
    let mut body = Body::<f64>::new();
    let seed = body.mvfs(pts[0]).unwrap();
    let first = body
        .mev_line(
            MevSite::Lone {
                r#loop: seed.r#loop,
            },
            pts[1],
            tol,
        )
        .unwrap();
    let mut last = first;
    for &pt in &pts[2..] {
        last = body
            .mev_line(
                MevSite::Fan {
                    he1: last.he_minus,
                    he2: last.he_minus,
                },
                pt,
                tol,
            )
            .unwrap();
    }
    body.mef_chord(
        MefSite::Chords {
            he1: first.he_plus,
            he2: last.he_minus,
        },
        tol,
    )
    .unwrap();
    assert_eq!(validate(&body), Ok(()));
    body
}

#[test]
fn hexagon_pillow_oracle_is_deterministic_and_rotation_blind() {
    let tol = Tol::witness();
    // Adversarial symmetric body: the hexagon pillow has a dihedral
    // automorphism group of the underlying structure. Build it starting
    // at each of the six rotations of the SAME six coordinates: all
    // must be isomorphic to each other (min-over-roots must defeat the
    // anchor/rotation sensitivity), and repeated canonicalization must
    // be byte-stable.
    let hexagon: Vec<Point3<f64>> = (0..6).map(|i| pt(f64::from(i), 0.0, 0.0)).collect();
    let reference = ngon_pillow(&hexagon, tol);
    let reference_form = canonical_form(&reference);
    assert_eq!(canonical_form(&reference), reference_form, "stable");
    for shift in 1..6 {
        let rotated: Vec<Point3<f64>> = (0..6).map(|i| hexagon[(i + shift) % 6]).collect();
        let body = ngon_pillow(&rotated, tol);
        assert!(
            isomorphic(&reference, &body),
            "rotation {shift} must be isomorphic"
        );
    }
    // Reflected coordinates: the pillow is ALSO isomorphic to its
    // reversal (flip the pillow over: an orientation-preserving map of
    // the whole closed surface exists). The oracle must find it.
    let reversed: Vec<Point3<f64>> = (0..6).map(|i| hexagon[(6 - i) % 6]).collect();
    let body = ngon_pillow(&reversed, tol);
    assert!(isomorphic(&reference, &body), "reversal isomorphic");
}

#[test]
fn hexagon_pillow_with_fully_degenerate_coordinates_is_stable() {
    let tol = Tol::witness();
    // All six vertices at ONE point: the automorphism group now acts
    // with genuinely tied candidates. Determinism + termination + self
    // consistency (two identical builds compare equal; two rotated
    // builds still compare equal since every candidate ties).
    //
    // M2 PR 3 revision: the chord-line SUGAR now refuses coincident
    // endpoints loudly (a zero-length chord has no certifiable line
    // carrier — the attachment gate at work), so the degenerate build
    // attaches explicit full-period circle carriers through the shared
    // point instead: every edge certifies honestly (its carrier truly
    // closes at that point), and the ORACLE claim under test — topology
    // and canonicalization are stable under coordinate ties — is
    // unchanged.
    let p7 = pt(7.0, 7.0, 7.0);
    let degenerate_pillow = || {
        let mut body = Body::<f64>::new();
        let seed = body.mvfs(p7).unwrap();
        let circle = || geom_brep::EdgeCurveSpec::self_loop_circle_at(p7);
        let first = body
            .mev(
                MevSite::Lone {
                    r#loop: seed.r#loop,
                },
                p7,
                circle(),
                tol,
            )
            .unwrap();
        let mut last = first;
        for _ in 2..6 {
            last = body
                .mev(
                    MevSite::Fan {
                        he1: last.he_minus,
                        he2: last.he_minus,
                    },
                    p7,
                    circle(),
                    tol,
                )
                .unwrap();
        }
        body.mef(
            MefSite::Chords {
                he1: first.he_plus,
                he2: last.he_minus,
            },
            circle(),
            crate::FaceSurface::Inherit,
            tol,
        )
        .unwrap();
        assert_eq!(validate(&body), Ok(()));
        body
    };
    // The sugar path refuses the same degenerate build, typed (the
    // fail-loud half of the revision).
    let mut refused = Body::<f64>::new();
    let seed = refused.mvfs(p7).unwrap();
    assert!(matches!(
        refused.mev_line(
            MevSite::Lone {
                r#loop: seed.r#loop
            },
            p7,
            tol,
        ),
        Err(EulerOpError::Certification { .. })
    ));
    let a = degenerate_pillow();
    let b = degenerate_pillow();
    assert!(isomorphic(&a, &b));
    let form = canonical_form(&a);
    assert_eq!(canonical_form(&a), form, "byte-stable on ties");
}

#[test]
fn oracle_solid_order_false_negative_is_real_and_documented() {
    // Documented limit probe: same two solids, opposite creation order.
    // The bodies are genuinely isomorphic (solid order is not
    // structure), but the oracle compares positionally.
    let build = |first: f64, second: f64| {
        let mut body = Body::<f64>::new();
        body.mvfs(p(first)).unwrap();
        body.mvfs(p(second)).unwrap();
        body
    };
    let ab = build(0.0, 1.0);
    let ba = build(1.0, 0.0);
    assert!(
        !isomorphic(&ab, &ba),
        "positional solid comparison (documented false negative)"
    );
}

#[test]
fn oracle_distinguishes_ring_attachment_even_at_shared_coordinates() {
    let tol = Tol::witness();
    // Attack on the "twin" blind spot from the attachment side: two
    // empty rings at IDENTICAL coordinates, together on one face vs
    // split across the pillow's two faces. The empty-ring tokens tie;
    // the face emission must still tell the bodies apart.
    let build = |split: bool| {
        let (mut body, _seed, seg) = segment(tol);
        let faces = body
            .mef_chord(
                MefSite::Chords {
                    he1: seg.he_plus,
                    he2: seg.he_minus,
                },
                tol,
            )
            .unwrap();
        let plant = |body: &mut Body<f64>, at: HalfEdgeKey| {
            let strut = body
                .mev_line(MevSite::Fan { he1: at, he2: at }, pt(9.0, 9.0, 9.0), tol)
                .unwrap();
            body.kemr(strut.he_plus, strut.he_minus).unwrap()
        };
        let r1 = plant(&mut body, seg.he_plus);
        let _r2 = plant(&mut body, seg.he_plus);
        if split {
            let other = body.get_half_edge(seg.he_minus).unwrap().parent_loop;
            let other = body.get_loop(other).unwrap().face;
            assert_ne!(other, faces.face);
            body.ring_move(r1.ring, other).unwrap();
        }
        assert_eq!(validate(&body), Ok(()));
        body
    };
    let together = build(false);
    let split = build(true);
    assert!(!isomorphic(&together, &split));
}

// =====================================================================
// 6. Teardown: the genus-2 double-hole body.
// =====================================================================

/// Carves an n-gon hole from `f_from` through to `f_to` (PR 3 review's
/// recipe, compacted; no ledger asserts — the seqgen Ledger is checked
/// where it matters).
fn carve_hole(
    body: &mut Body<f64>,
    at: HalfEdgeKey,
    _f_from: FaceKey, // reviewer signature kept; the anchor `at` already sits on it
    f_to: FaceKey,
    rim_pts: &[Point3<f64>],
    drop_pts: &[Point3<f64>],
    tol: Tol,
) {
    let strut = body
        .mev_line(MevSite::Fan { he1: at, he2: at }, rim_pts[0], tol)
        .unwrap();
    let kill = body.kemr(strut.he_plus, strut.he_minus).unwrap();
    let mut rim: Vec<MevCreated> = vec![
        body.mev_line(MevSite::Lone { r#loop: kill.ring }, rim_pts[1], tol)
            .unwrap(),
    ];
    for rp in &rim_pts[2..] {
        let prev = rim.last().unwrap().he_minus;
        rim.push(
            body.mev_line(
                MevSite::Fan {
                    he1: prev,
                    he2: prev,
                },
                *rp,
                tol,
            )
            .unwrap(),
        );
    }
    let membrane = body
        .mef_chord(
            MefSite::Chords {
                he1: rim[0].he_plus,
                he2: rim.last().unwrap().he_minus,
            },
            tol,
        )
        .unwrap();
    let mut drops: Vec<MevCreated> = Vec::new();
    for (i, dp) in drop_pts.iter().enumerate() {
        let anchor = if i < rim.len() {
            rim[i].he_plus
        } else {
            membrane.he_minus
        };
        drops.push(
            body.mev_line(
                MevSite::Fan {
                    he1: anchor,
                    he2: anchor,
                },
                *dp,
                tol,
            )
            .unwrap(),
        );
    }
    let mut walls: Vec<crate::euler::MefCreated> = Vec::new();
    for i in 0..drops.len() - 1 {
        walls.push(
            body.mef_chord(
                MefSite::Chords {
                    he1: drops[i].he_minus,
                    he2: drops[i + 1].he_minus,
                },
                tol,
            )
            .unwrap(),
        );
    }
    let he_first_far = body
        .find_half_edge(membrane.face, drops[0].vertex, drops[1].vertex)
        .unwrap();
    body.mef_chord(
        MefSite::Chords {
            he1: drops.last().unwrap().he_minus,
            he2: he_first_far,
        },
        tol,
    )
    .unwrap();
    body.kfmrh(f_to, membrane.face).unwrap();
    assert_eq!(validate(body), Ok(()));
}

#[test]
fn genus_two_double_hole_body_tears_down_to_nothing() {
    let tol = Tol::witness();
    // Rebuild the PR 3 milestone body via ops: cube + square hole
    // top→bottom (the ops_holed_box recipe) + triangular hole
    // front→back. Genus 2. Then drive seqgen::teardown — it must
    // unwind handles (mfkrh), rings, and all 30+ edges to empty arenas
    // AND empty provenance maps AND fully reaped geometry (teardown
    // asserts all of that internally).
    let t = crate::fixtures::ops_holed_box(tol);
    let mut body = t.body;
    // Second hole: triangular, through the front face to the back face.
    let f_front = t.box_mefs[1].face;
    let f_back = t.box_mefs[3].face;
    let front_outer = body.get_face(f_front).unwrap().outer;
    let LoopBoundary::Cycle { first } = body.get_loop(front_outer).unwrap().boundary else {
        panic!("front outer is a cycle");
    };
    let at = first;
    carve_hole(
        &mut body,
        at,
        f_front,
        f_back,
        &[pt(0.3, 0.0, 0.3), pt(0.7, 0.0, 0.3), pt(0.5, 0.0, 0.7)],
        &[pt(0.3, 1.0, 0.3), pt(0.7, 1.0, 0.3), pt(0.5, 1.0, 0.7)],
        tol,
    );
    // Genus 2 checkpoint: v − e + f − r = 2(1 − 2) = −2.
    let v = body.vertices().count() as i64;
    let e = body.edges().count() as i64;
    let f = body.faces().count() as i64;
    let r: i64 = body.faces().map(|(_, fd)| fd.rings.len() as i64).sum();
    assert_eq!(v - e + f - r, -2, "genus 2");
    seqgen::teardown(&mut body, tol);
}

// =====================================================================
// 7. Key purity under failing kill calls + generation safety.
// =====================================================================

#[test]
fn failing_kill_calls_consume_no_keys_between_kills() {
    let tol = Tol::witness();
    let run = |with_failures: bool| {
        let (mut body, seed, seg) = segment(tol);
        if with_failures {
            // A battery of failing calls between every real op.
            assert!(body.kvfs(seed.solid).is_err()); // grown solid
            assert!(body.kev(HalfEdgeKey::default()).is_err()); // stale
        }
        let split = body
            .mef_chord(
                MefSite::Chords {
                    he1: seg.he_plus,
                    he2: seg.he_minus,
                },
                tol,
            )
            .unwrap();
        if with_failures {
            assert!(body.kef(HalfEdgeKey::default()).is_err()); // stale
            assert!(body.mfkrh_plug(LoopKey::default()).is_err()); // stale
        }
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
        body.kev(strut.he_plus).unwrap();
        if with_failures {
            assert!(body.mfkrh_plug(seed.r#loop).is_err()); // outer, RingIsOuter
            assert!(body.kvfs(seed.solid).is_err());
        }
        let cut = body
            .mef_chord(
                MefSite::Chords {
                    he1: split.he_minus,
                    he2: seg.he_plus,
                },
                tol,
            )
            .unwrap();
        body.kef(cut.he_minus).unwrap();
        body
    };
    let clean = run(false);
    let noisy = run(true);
    assert_eq!(
        deep_snapshot(&clean),
        deep_snapshot(&noisy),
        "failing calls must consume zero key slots"
    );
}

#[test]
fn kvfs_slot_recycling_is_generation_safe() {
    let _tol = Tol::witness();
    let mut body = Body::<f64>::new();
    let first = body.mvfs(p(0.0)).unwrap();
    body.kvfs(first.solid).unwrap();
    let second = body.mvfs(p(1.0)).unwrap();
    // Recycled slots, bumped generations: every old key is dead and
    // DISTINCT from the new one.
    assert_ne!(first.solid, second.solid);
    assert_ne!(first.vertex, second.vertex);
    assert!(body.get_solid(first.solid).is_none());
    assert!(body.get_shell(first.shell).is_none());
    assert!(body.get_face(first.face).is_none());
    assert!(body.get_loop(first.r#loop).is_none());
    assert!(body.get_vertex(first.vertex).is_none());
    assert_eq!(body.provenance(EntityId::Solid(first.solid)), None);
    assert_eq!(body.provenance(EntityId::Vertex(first.vertex)), None);
    // Ops fed the stale keys give typed errors, not aliasing.
    assert_eq!(
        body.kvfs(first.solid).unwrap_err(),
        EulerOpError::StaleKey {
            key: EntityId::Solid(first.solid)
        }
    );
    assert_eq!(
        body.mfkrh_plug(first.r#loop).unwrap_err(),
        EulerOpError::StaleKey {
            key: EntityId::Loop(first.r#loop)
        }
    );
    // And the recycled solid still works.
    body.kvfs(second.solid).unwrap();
    assert_eq!(body.solids().count(), 0);
}

// =====================================================================
// 8. kef's UnclaimedHalfEdge (the kev-only shipped test's sibling).
// =====================================================================

#[test]
fn kef_rejects_a_corrupt_edge_bijection() {
    let tol = Tol::witness();
    let (mut body, _seed, seg) = segment(tol);
    let split = body
        .mef_chord(
            MefSite::Chords {
                he1: seg.he_plus,
                he2: seg.he_minus,
            },
            tol,
        )
        .unwrap();
    // Corrupt: seg.edge no longer claims seg.he_plus.
    body.get_edge_mut(seg.edge).unwrap().he_plus = split.he_plus;
    body.get_edge_mut(seg.edge).unwrap().he_minus = split.he_minus;
    let err = body.kef(seg.he_plus).unwrap_err();
    assert_eq!(
        err,
        EulerOpError::UnclaimedHalfEdge {
            he: seg.he_plus,
            edge: seg.edge,
        }
    );
}

// =====================================================================
// 9. Release-mode garbage-in. The surviving half of the contract is
//    "never a hang; every traversal is bounded" (D9's footnote as
//    amended by the D2 addendum, which retired the garbage-out half).
//    These rows also run in debug, where the errors are
//    precondition-caught, on every code-tier run. The release side is the
//    `corrupt input (release profile)` job, which runs once a night in
//    .github/workflows/nightly.yml (and on every local gate) rather than
//    per PR: the argument that a persistence-detector may sit on a cadence
//    is at the job. It greps that job name out of this comment, so a
//    rename is loud rather than quietly falsifying this sentence.
// =====================================================================

#[test]
fn kill_ops_survive_torn_bodies_without_panicking() {
    let tol = Tol::witness();
    use crate::entity::{HalfEdge, Vertex};
    // A big strut chain, then tear next/prev links and edge bijections
    // at scale; hammer all four ops on every half-edge/solid/loop.
    let mut body = Body::<f64>::new();
    let seed = body.mvfs(p(0.0)).unwrap();
    let seg = body
        .mev_line(
            MevSite::Lone {
                r#loop: seed.r#loop,
            },
            p(1.0),
            tol,
        )
        .unwrap();
    let mut anchor = seg.he_minus;
    for i in 0..300 {
        let strut = body
            .mev_line(
                MevSite::Fan {
                    he1: anchor,
                    he2: anchor,
                },
                p(2.0 + f64::from(i)),
                tol,
            )
            .unwrap();
        anchor = strut.he_minus;
    }
    // Tear: point half the next links at a foreign key and half the
    // edges' plus slots at the wrong half.
    let halves: Vec<HalfEdgeKey> = body.half_edges().map(|(k, _)| k).collect();
    let foreign = {
        let mut other = Body::<f64>::new();
        let s = other.mvfs(p(0.0)).unwrap();
        let sg = other
            .mev_line(MevSite::Lone { r#loop: s.r#loop }, p(1.0), tol)
            .unwrap();
        sg.he_plus
    };
    for (i, &he) in halves.iter().enumerate() {
        if i % 2 == 0 {
            body.get_half_edge_mut(he).unwrap().next = foreign;
        }
        if i % 3 == 0 {
            let edge = body.get_half_edge(he).unwrap().edge;
            if let Some(e) = body.get_edge_mut(edge) {
                e.he_plus = he;
                e.he_minus = he;
            }
        }
    }
    // Also a vertex whose point is gone and a loop pointing at a dead
    // face.
    let vpoint = body.add_point(p(999.0));
    let stray = body.add_vertex(
        Vertex {
            point: vpoint,
            emanating: None,
        },
        crate::fixtures::prov(),
    );
    body.points.remove(vpoint);
    let _ = stray;
    let started = std::time::Instant::now();
    for &he in &halves {
        let _ = body.clone().kev(he);
        let _ = body.clone().kef(he);
    }
    let solids: Vec<_> = body.solids().map(|(k, _)| k).collect();
    for s in solids {
        let _ = body.clone().kvfs(s);
    }
    let loops: Vec<_> = body.loops().map(|(k, _)| k).collect();
    for l in loops {
        let _ = body.clone().mfkrh_plug(l);
    }
    assert!(
        started.elapsed() < std::time::Duration::from_secs(30),
        "bounded walks must keep this fast"
    );
    let _ = HalfEdge {
        edge: crate::entity::EdgeKey::default(),
        start: VertexKey::default(),
        parent_loop: LoopKey::default(),
        next: HalfEdgeKey::default(),
        prev: HalfEdgeKey::default(),
    };
}

// =====================================================================
// 10. seqgen reusability probe: coverage of all op kinds over a fixed
//     decision sweep (detects silently-never-generated ops).
// =====================================================================

#[test]
fn seqgen_generates_every_op_kind_and_every_site_shape() {
    let tol = Tol::witness();
    use crate::seqgen::{OpChoice, apply, choose_op};
    use std::collections::BTreeSet;
    use test_utils::fuzz;
    let expected: BTreeSet<&'static str> = [
        "mvfs",
        "mev_lone",
        "mev_fan",
        "mev_fan_strut",
        "mef_chords",
        "mef_chords_self",
        "mef_lone",
        "kemr",
        "mekr_cycles",
        "mekr_empty_ring",
        "mekr_empty_target",
        "mekr_both_empty",
        "kfmrh",
        "mfkrh",
        "ring_move",
        "split_edge",
        "split_edge_strut",
        "split_edge_self_loop",
        "kev",
        "kef",
        "kvfs",
    ]
    .into_iter()
    .collect();
    let mut seen: BTreeSet<&'static str> = BTreeSet::new();
    // COVERAGE ROW, not a sweep: the claim is that every op kind and site
    // shape IS generated, and the rarest label (`kvfs`, ~0.1% of steps)
    // sets the floor. So the walk ACCUMULATES until coverage is complete
    // and the counts below are a CAP, not a target — a run that reaches
    // the cap without full coverage fails loudly, and a run that covers
    // early stops there (the usual cost is a small fraction of the cap).
    // The cap rides `CAD_FUZZ_EFFORT` like every other count, upward.
    //
    // The budget is split into MANY SHORT walks rather than three long
    // ones. AUDIT FINDING (2026-08-13): the rarest label, `kvfs`, needs a
    // SKELETAL solid to exist, and that is trajectory-dependent — once a
    // body has grown, skeletal solids stop appearing (this file's sibling
    // row records ~3% of steps, "0 on some seeds"). A fresh `Body` starts
    // empty, so each RESTART manufactures the window that long walks stop
    // offering.
    //
    // THE SEED IS PINNED, and that is correct here rather than an
    // exception to the no-fixed-seeds rule. This row is shape 3 in
    // `test_utils::fuzz`'s taxonomy: the claim IS coverage, and "a walk
    // that reaches every op kind and every site shape" is not something
    // you can write down as a fixture. A coverage claim on a VARYING
    // seed is a witness search that flakes — measured at ~1 run in 150
    // before this pin — and the honest fix is determinism, not
    // over-provisioning the budget to survive bad luck. It clears the
    // accident bar comfortably: every op kind AND every site shape must
    // land at once, so a seed that satisfies this is a good seed rather
    // than a lucky one. `CAD_FUZZ_SEED` still overrides, which is how
    // you check the claim is not specific to this draw.
    let restarts = fuzz::scaled(12);
    let steps = fuzz::scaled(1_500);
    let mut rng = fuzz::pinned(
        "review_m1_pr4::seqgen_op_kind_coverage",
        0x243F_6A88_85A3_08D3,
    );
    let mut walked = 0usize;
    'walk: for _restart in 0..restarts {
        let mut body = Body::<f64>::new();
        let mut counter = 0u32;
        let step = |r: &mut fuzz::Rng| (r.next_u64() >> 33) as u32;
        for _ in 0..steps {
            let d1 = step(&mut rng);
            let d2 = step(&mut rng);
            let Some(choice) = choose_op(&body, d1, d2, tol) else {
                panic!("no applicable op — {}", fuzz::replay());
            };
            walked += 1;
            seen.insert(match choice {
                OpChoice::Mvfs => "mvfs",
                OpChoice::MevLone(_) => "mev_lone",
                OpChoice::MevFan(h1, h2) => {
                    if h1 == h2 {
                        "mev_fan_strut"
                    } else {
                        "mev_fan"
                    }
                }
                OpChoice::MefChords(h1, h2) => {
                    if h1 == h2 {
                        "mef_chords_self"
                    } else {
                        "mef_chords"
                    }
                }
                OpChoice::MefLone(_) => "mef_lone",
                OpChoice::Kemr(..) => "kemr",
                OpChoice::Mekr(MekrSite::Cycles { .. }) => "mekr_cycles",
                OpChoice::Mekr(MekrSite::EmptyRing { .. }) => "mekr_empty_ring",
                OpChoice::Mekr(MekrSite::EmptyTarget { .. }) => "mekr_empty_target",
                OpChoice::Mekr(MekrSite::BothEmpty { .. }) => "mekr_both_empty",
                OpChoice::Kfmrh(..) => "kfmrh",
                OpChoice::Mfkrh(_) => "mfkrh",
                OpChoice::Kev(_) => "kev",
                OpChoice::Kef(_) => "kef",
                OpChoice::Kvfs(_) => "kvfs",
                OpChoice::RingMove(..) => "ring_move",
                // Split by SITE SHAPE, like the other multi-shape ops:
                // "split_edge fired at least once" is not the claim the
                // fuzz row exists to support — `split.rs`'s surgery
                // note calls the strut and self-loop coincidences the
                // delicate cases, and those are what a randomised lane
                // is for. A single label would prove neither was
                // reached.
                OpChoice::SplitEdge(e) => {
                    let ed = body.get_edge(e).expect("a split candidate resolves");
                    let (hp, hm) = (ed.he_plus, ed.he_minus);
                    let plus = body.get_half_edge(hp).expect("a half resolves");
                    if plus.start == body.half_edge_end(hp).expect("an end resolves") {
                        "split_edge_self_loop"
                    } else if plus.next == hm {
                        "split_edge_strut"
                    } else {
                        "split_edge"
                    }
                }
            });
            apply(&mut body, choice, &mut counter, tol);
            assert_eq!(
                validate(&body),
                Ok(()),
                "seqgen produced an invalid body — {}",
                fuzz::replay()
            );
            if expected.is_subset(&seen) {
                break 'walk;
            }
        }
    }
    println!("[seqgen coverage] complete after {walked} steps (cap {restarts}x{steps})");
    let missing: Vec<_> = expected.difference(&seen).collect();
    assert!(
        missing.is_empty(),
        "op kinds/sites never generated within the {restarts}x{steps} cap: \
         {missing:?} (seen: {seen:?}) — {}",
        fuzz::replay()
    );
}

#[test]
fn seqgen_kvfs_availability_instrumented() {
    let tol = Tol::witness();
    use crate::seqgen::{OpChoice, apply, choose_op};
    use test_utils::fuzz;
    let mut body = Body::<f64>::new();
    let mut counter = 0u32;
    // Pinned for the same reason as the coverage row above: this is an
    // EXISTENCE claim (shape 3), so a varying seed would make it a
    // witness search that can flake, and the trajectory that produces a
    // skeletal solid is not something you can write down as a fixture.
    let mut rng = fuzz::pinned(
        "review_m1_pr4::seqgen_kvfs_availability",
        0x1234_5678_9ABC_DEF0,
    );
    let step = |r: &mut fuzz::Rng| (r.next_u64() >> 33) as u32;
    let mut available = 0usize;
    let mut chosen = 0usize;
    let mut skeletal_present = 0usize;
    // EXISTENCE claim: the count below is the CAP on the walk, not the
    // work done (the loop breaks the moment a skeletal solid appears, in
    // ~30 steps at the recorded ~3% density). Cutting the cap would only
    // make a thin trajectory fail, so it stays at the recorded bound and
    // rides EFFORT upward.
    for _ in 0..fuzz::scaled(6_000) {
        let skeletal = body
            .solids()
            .filter(|&(s, _)| {
                let solid_data = body.get_solid(s).unwrap();
                let [shell] = solid_data.shells[..] else {
                    return false;
                };
                let [face] = body.get_shell(shell).unwrap().faces[..] else {
                    return false;
                };
                let face_data = body.get_face(face).unwrap();
                face_data.rings.is_empty()
                    && matches!(
                        body.get_loop(face_data.outer).unwrap().boundary,
                        LoopBoundary::Empty { .. }
                    )
            })
            .count();
        if skeletal > 0 {
            skeletal_present += 1;
        }
        let d1 = step(&mut rng);
        let d2 = step(&mut rng);
        let choice = choose_op(&body, d1, d2, tol).unwrap();
        if matches!(choice, OpChoice::Kvfs(_)) {
            chosen += 1;
        }
        let _ = &mut available;
        apply(&mut body, choice, &mut counter, tol);
        // The claim below is an EXISTENCE claim, so the walk stops the
        // moment it is witnessed. INVARIANT: 6000 stays the UPPER
        // bound, not the work done — a trajectory on which skeletal
        // solids never appear still walks all 6000 steps and then fails
        // loudly on the assertion. At the ~3% density recorded below,
        // the witness lands in ~30 steps in expectation.
        if skeletal_present > 0 {
            break;
        }
    }
    // Finding (recorded in the review report): kvfs availability is
    // trajectory-dependent and thin (~3% of steps see a skeletal
    // solid; kvfs itself is chosen in ~0.1% of steps, 0 on some
    // seeds). Coverage exists but is thin; teardown() exercises kvfs
    // deterministically every proptest case.
    assert!(
        skeletal_present > 0,
        "skeletal solids never appear — {}",
        fuzz::replay()
    );
    let _ = chosen;
}

// =====================================================================
// 11. The SameFace error guidance: an edge joining two loops of ONE
//     face (kfmrh on adjacent faces makes it). kef must refuse with
//     SameFace; kev must be able to kill it when endpoints are
//     distinct (the Display text's advice); the self-loop variant has
//     NO direct one-op killer (mfkrh+kef is the escape).
// =====================================================================

#[test]
fn same_face_bridge_edge_kef_refuses_and_kev_kills() {
    let tol = Tol::witness();
    // Cube; kfmrh(top, front): front's outer becomes a ring of top;
    // the shared top/front edge now has one half in top's outer, the
    // other in top's ring.
    let t = ops_cube(tol);
    let mut body = t.body;
    let top = t.seed.face;
    let front = t.mefs[1].face;
    body.kfmrh(top, front).unwrap();
    assert_eq!(validate(&body), Ok(()));
    // Find a half-edge whose mate is in a different loop of the SAME
    // face.
    let bridge = body
        .half_edges()
        .map(|(k, _)| k)
        .find(|&he| {
            let l1 = body.get_half_edge(he).unwrap().parent_loop;
            let m = body.mate(he).unwrap();
            let l2 = body.get_half_edge(m).unwrap().parent_loop;
            l1 != l2 && body.get_loop(l1).unwrap().face == body.get_loop(l2).unwrap().face
        })
        .expect("kfmrh on adjacent faces creates a bridge edge");
    // kef: SameFace, atomic.
    let before = deep_snapshot(&body);
    let err = body.kef(bridge).unwrap_err();
    assert!(matches!(err, EulerOpError::SameFace { .. }));
    assert_eq!(deep_snapshot(&body), before);
    // kev (the error text's advice): endpoints are distinct cube
    // corners, so it kills the edge (and the far vertex, fan merged).
    body.kev(bridge).unwrap();
    assert_eq!(validate(&body), Ok(()));
}

#[test]
fn same_face_self_loop_bridge_has_no_direct_killer_but_mfkrh_frees_it() {
    let tol = Tol::witness();
    // The self-loop variant: mef(Lone) then kfmrh folds the circular
    // face into its parent — a self-loop edge whose halves sit in the
    // outer and a ring of ONE face. kev: SelfLoopEdge. kef: SameFace.
    // kemr: NotSameLoop. mfkrh(ring) re-splits, then kef works.
    let mut body = Body::<f64>::new();
    let seed = body.mvfs(p(0.0)).unwrap();
    let circ = body
        .mef_chord(
            MefSite::Lone {
                r#loop: seed.r#loop,
            },
            tol,
        )
        .unwrap();
    body.kfmrh(seed.face, circ.face).unwrap();
    assert_eq!(validate(&body), Ok(()));
    assert!(matches!(
        body.kev(circ.he_plus).unwrap_err(),
        EulerOpError::SelfLoopEdge { .. }
    ));
    assert!(matches!(
        body.kef(circ.he_plus).unwrap_err(),
        EulerOpError::SameFace { .. }
    ));
    assert!(matches!(
        body.kemr(circ.he_plus, circ.he_minus).unwrap_err(),
        EulerOpError::NotSameLoop { .. }
    ));
    // Escape hatch: promote the ring back to a face, then kef.
    let promoted = body.mfkrh_plug(circ.r#loop).unwrap();
    let _ = promoted;
    body.kef(circ.he_minus).unwrap();
    assert_eq!(validate(&body), Ok(()));
    assert_eq!(
        body.get_loop(seed.r#loop).unwrap().boundary,
        LoopBoundary::Empty {
            vertex: seed.vertex
        }
    );
}
