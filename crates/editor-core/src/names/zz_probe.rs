//! namedepth-r2 differential probe: the same corpus, built identically
//! on main (derived impls) and on the head (hand-written), every answer
//! written to a file for a byte diff.
#![allow(clippy::all, clippy::pedantic, clippy::nursery, clippy::restriction)]
#![allow(unused)]

use super::role::{
    CapEnd, EntityKind, MeridianEnd, NameRef, PieceRole, ProfileEdgeRef, ProfileVertexRef,
    Qualifier, RimSupport, RoleSeg, SectionCircle, SideVerdict, SplitHalf, StableName,
};
use crate::node::{RecipeNodeId, StepId};
use core::cmp::Ordering;
use std::fmt::Write as _;

struct Rng(u64);
impl Rng {
    fn next(&mut self) -> u64 {
        self.0 = self.0.wrapping_mul(6364136223846793005).wrapping_add(1442695040888963407);
        self.0 >> 33
    }
    fn below(&mut self, n: u64) -> u64 {
        self.next() % n
    }
}

fn leaf(kind: EntityKind, node: u64, cap: CapEnd) -> StableName {
    StableName { kind, node: RecipeNodeId(node), path: vec![RoleSeg::Cap(cap)] }
}

const VARIANTS: u64 = 47;

/// Segment of variant `v`; `ns` supplies held names (cycled), `share`
/// says whether the held handles for one variant share one Arc.
fn seg(v: u64, ns: &[StableName], rng: &mut Rng) -> RoleSeg {
    use RoleSeg as R;
    let mut i = 0usize;
    let mut next = || {
        let n = ns[i % ns.len()].clone();
        i += 1;
        n
    };
    let step = StepId(rng.below(9));
    let e = ProfileEdgeRef::Piece { step, role: PieceRole::Piece(rng.below(4) as u32) };
    let e2 = ProfileEdgeRef::Section { circle: SectionCircle::Bore, role: PieceRole::Arc };
    let vx = ProfileVertexRef::Piece { step, role: PieceRole::Leg };
    let side = if rng.below(2) == 0 { SplitHalf::Below } else { SplitHalf::Above };
    let many = |next: &mut dyn FnMut() -> StableName, k: usize| (0..k).map(|_| next()).collect::<Vec<_>>();
    match v {
        0 => R::OutputBody,
        1 => R::Cap(CapEnd::Start),
        2 => R::Lateral(e),
        3 => R::RimEdge(CapEnd::End, e2),
        4 => R::LateralEdge(vx),
        5 => R::CapVertex(CapEnd::Start, vx),
        6 => R::LoftWall(vec![e, e2]),
        7 => R::LoftSeam(vec![vx]),
        8 => R::Band(e),
        9 => R::BandRim(vx),
        10 => R::BandRimPi(vx),
        11 => R::BandPi(e),
        12 => R::Meridian(MeridianEnd::Seam, e),
        13 => R::MeridianVertex(MeridianEnd::Pi, vx),
        14 => R::RevolveCap(MeridianEnd::End),
        15 => R::Pole(vx),
        16 => R::AxisEdge(e2),
        17 => R::FromA(NameRef::new(next())),
        18 => R::FromB(NameRef::new(next())),
        19 => R::FromMember { member: RecipeNodeId(rng.below(5)), of: NameRef::new(next()) },
        20 => R::Seam { a: NameRef::new(next()), b: NameRef::new(next()) },
        21 => {
            let k = 1 + rng.below(5) as usize;
            R::Merged(many(&mut next, k))
        }
        22 => {
            let k = 1 + rng.below(4) as usize;
            let v = many(&mut next, k)
                .into_iter()
                .enumerate()
                .map(|(j, n)| (n, if j % 2 == 0 { SideVerdict::On } else { SideVerdict::Mixed }))
                .collect();
            R::Fragment(Qualifier::SideOf(v))
        }
        23 => R::Fragment(Qualifier::OrderAlong { rank: rng.below(3) as u32, of: 3 }),
        24 => R::SplitBody(side),
        25 => R::SectionFace { side, section: rng.below(3) as u32 },
        26 => R::SectionEdge { side, face: NameRef::new(next()) },
        27 => R::SplitFragment { side, parent: NameRef::new(next()) },
        28 => R::CrossingVertex { side, edge: NameRef::new(next()) },
        29 => R::OnToolVertex { side, of: NameRef::new(next()) },
        30 => R::FromTarget(NameRef::new(next())),
        31 => R::BlendFace(NameRef::new(next())),
        32 => R::CornerFace(NameRef::new(next())),
        33 => R::TrimEdge { edge: NameRef::new(next()), support: NameRef::new(next()) },
        34 => R::FootVertex { vertex: NameRef::new(next()), support: NameRef::new(next()) },
        35 => R::EndArc { vertex: NameRef::new(next()), edge: NameRef::new(next()) },
        36 => {
            let k = 1 + rng.below(4) as usize;
            R::BandFace(many(&mut next, k))
        }
        37 => R::BandTrim { edge: NameRef::new(next()), support: RimSupport::Mate },
        38 => R::BandFoot(NameRef::new(next())),
        39 => {
            let edge = NameRef::new(next());
            let k = rng.below(3) as usize;
            R::BandCross { edge, band: many(&mut next, k) }
        }
        40 => R::BandCut(NameRef::new(next())),
        41 => {
            let edge = NameRef::new(next());
            let k = rng.below(3) as usize;
            R::BandSlit { edge, band: many(&mut next, k) }
        }
        42 => R::Inner(NameRef::new(next())),
        43 => R::Rim(NameRef::new(next())),
        44 => R::HoleRim { of: NameRef::new(next()), hole: rng.below(3) as u32 },
        45 => R::InPart { of: NameRef::new(next()) },
        _ => R::Instance { i: rng.below(3) as u32, of: NameRef::new(next()) },
    }
}

fn kind(rng: &mut Rng) -> EntityKind {
    match rng.below(4) {
        0 => EntityKind::Face,
        1 => EntityKind::Edge,
        2 => EntityKind::Vertex,
        _ => EntityKind::Body,
    }
}

/// A random tree of depth up to `d`.
fn tree(rng: &mut Rng, d: u32) -> StableName {
    if d == 0 || rng.below(4) == 0 {
        let cap = if rng.below(2) == 0 { CapEnd::Start } else { CapEnd::End };
        return leaf(kind(rng), rng.below(4), cap);
    }
    let segs = 1 + rng.below(3);
    let mut path = Vec::new();
    for _ in 0..segs {
        let v = rng.below(VARIANTS);
        let ns: Vec<StableName> = (0..5).map(|_| tree(rng, d - 1)).collect();
        path.push(seg(v, &ns, rng));
    }
    StableName { kind: kind(rng), node: RecipeNodeId(rng.below(6)), path }
}

/// A chain `depth` deep over `bottom`, cycling through the held-name
/// shapes, each level optionally with a trailing own segment.
fn chain(bottom: StableName, depth: usize, tail: Option<u32>) -> StableName {
    let side = leaf(EntityKind::Face, 9, CapEnd::Start);
    let mut n = bottom;
    for level in 0..depth {
        let r = NameRef::new(n.clone());
        let s = match level % 8 {
            0 => RoleSeg::FromA(r),
            1 => RoleSeg::Instance { i: 1, of: r },
            2 => RoleSeg::InPart { of: r },
            3 => RoleSeg::Merged(vec![side.clone(), n]),
            4 => RoleSeg::Fragment(Qualifier::SideOf(vec![(n, SideVerdict::On)])),
            5 => RoleSeg::Seam { a: NameRef::new(side.clone()), b: r },
            6 => RoleSeg::BandCross { edge: NameRef::new(side.clone()), band: vec![n] },
            _ => RoleSeg::FromB(r),
        };
        let mut path = vec![s];
        if let Some(t) = tail {
            path.push(RoleSeg::Fragment(Qualifier::OrderAlong { rank: t, of: 3 }));
        }
        n = StableName { kind: EntityKind::Face, node: RecipeNodeId(level as u64 + 20), path };
    }
    n
}

fn corpus() -> Vec<StableName> {
    let mut rng = Rng(0x5eed_1234);
    let mut out = Vec::new();
    // Leaves.
    for k in 0..4 {
        for node in 0..3 {
            out.push(leaf(
                [EntityKind::Face, EntityKind::Edge, EntityKind::Vertex, EntityKind::Body][k],
                node,
                CapEnd::End,
            ));
        }
    }
    // Every variant, one deep, over leaf sets, and again two deep.
    let a = leaf(EntityKind::Face, 1, CapEnd::End);
    let b = leaf(EntityKind::Face, 2, CapEnd::End);
    let c = leaf(EntityKind::Edge, 1, CapEnd::Start);
    for v in 0..VARIANTS {
        for ns in [vec![a.clone(), b.clone(), c.clone()], vec![b.clone(), a.clone()], vec![a.clone()]] {
            out.push(StableName { kind: EntityKind::Face, node: RecipeNodeId(10), path: vec![seg(v, &ns, &mut rng)] });
        }
    }
    let one: Vec<StableName> = out.clone();
    for v in 0..VARIANTS {
        let i = rng.below(one.len() as u64) as usize;
        let j = rng.below(one.len() as u64) as usize;
        let ns = vec![one[i].clone(), one[j].clone(), one[(i + 1) % one.len()].clone()];
        out.push(StableName { kind: EntityKind::Edge, node: RecipeNodeId(11), path: vec![seg(v, &ns, &mut rng), RoleSeg::Fragment(Qualifier::OrderAlong { rank: 1, of: 2 })] });
    }
    // Random trees.
    for _ in 0..150 {
        out.push(tree(&mut rng, 4));
    }
    // Merged with several members, members differing deep.
    for k in [2usize, 3, 5, 8] {
        let ms: Vec<StableName> = (0..k).map(|j| chain(leaf(EntityKind::Face, j as u64, CapEnd::End), 3, None)).collect();
        out.push(StableName { kind: EntityKind::Face, node: RecipeNodeId(4), path: vec![RoleSeg::Merged(ms.clone())] });
        let mut ms2 = ms.clone();
        ms2.reverse();
        out.push(StableName { kind: EntityKind::Face, node: RecipeNodeId(4), path: vec![RoleSeg::Merged(ms2)] });
        ms2 = ms.clone();
        ms2.pop();
        out.push(StableName { kind: EntityKind::Face, node: RecipeNodeId(4), path: vec![RoleSeg::Merged(ms2)] });
    }
    // Deep chains either side of 64, differing only at the bottom (or
    // only at level 64/65 via tail rank).
    for depth in [1usize, 62, 63, 64, 65, 66, 100, 129, 200] {
        for bottom in [1u64, 2] {
            out.push(chain(leaf(EntityKind::Face, bottom, CapEnd::End), depth, None));
            out.push(chain(leaf(EntityKind::Face, bottom, CapEnd::End), depth, Some(0)));
        }
        // Differing only in the bottom's cap.
        out.push(chain(leaf(EntityKind::Face, 1, CapEnd::Start), depth, None));
        // A shared-Arc twin: same bottom, built from one shared handle.
        let shared = NameRef::new(chain(leaf(EntityKind::Face, 1, CapEnd::End), depth, None));
        out.push(StableName { kind: EntityKind::Edge, node: RecipeNodeId(3), path: vec![RoleSeg::Seam { a: shared.clone(), b: shared.clone() }] });
        // Unshared equal twin.
        out.push(StableName { kind: EntityKind::Edge, node: RecipeNodeId(3), path: vec![RoleSeg::Seam { a: NameRef::new(chain(leaf(EntityKind::Face, 1, CapEnd::End), depth, None)), b: shared.clone() }] });
        out.push(StableName { kind: EntityKind::Edge, node: RecipeNodeId(3), path: vec![RoleSeg::Seam { a: shared.clone(), b: NameRef::new(chain(leaf(EntityKind::Face, 2, CapEnd::End), depth, None)) }] });
    }
    // Names differing only at one level k in a 130-deep chain: put the
    // difference as the rank of level k's tail.
    for k in [10usize, 63, 64, 65, 66, 120] {
        let mut n = leaf(EntityKind::Face, 1, CapEnd::End);
        for level in 0..130usize {
            let rank = if 130 - level == k { 1 } else { 0 };
            n = StableName { kind: EntityKind::Face, node: RecipeNodeId(7), path: vec![RoleSeg::FromA(NameRef::new(n)), RoleSeg::Fragment(Qualifier::OrderAlong { rank, of: 2 })] };
        }
        out.push(n);
    }
    out
}

fn sip(n: &StableName) -> u64 {
    use core::hash::{Hash, Hasher};
    let mut h = std::collections::hash_map::DefaultHasher::new();
    n.hash(&mut h);
    h.finish()
}

fn sip_any<T: core::hash::Hash>(n: &T) -> u64 {
    use core::hash::Hasher;
    let mut h = std::collections::hash_map::DefaultHasher::new();
    n.hash(&mut h);
    h.finish()
}

fn run_probe() {
    let out_path = std::env::var("PROBE_OUT").expect("PROBE_OUT");
    let names = corpus();
    // Stamped: seal a subset in structural order, then add copies of
    // a few stamped handles in seams, so NameRef::cmp settles by stamp.
    let mut sorted_idx: Vec<usize> = (0..names.len()).collect();
    sorted_idx.sort_by(|&i, &j| names[i].cmp(&names[j]));
    let refs: Vec<NameRef> = names.iter().map(|n| NameRef::new(n.clone())).collect();
    let epoch = super::role::next_epoch().unwrap();
    for (pos, &i) in sorted_idx.iter().enumerate() {
        refs[i].stamp(epoch, pos as u32);
    }
    let mut stamped = Vec::new();
    for k in (0..names.len()).step_by(7) {
        let other = (k * 31 + 5) % names.len();
        stamped.push(StableName { kind: EntityKind::Edge, node: RecipeNodeId(99), path: vec![RoleSeg::Seam { a: refs[k].clone(), b: refs[other].clone() }] });
        stamped.push(StableName { kind: EntityKind::Edge, node: RecipeNodeId(99), path: vec![RoleSeg::Seam { a: NameRef::new(names[k].clone()), b: refs[other].clone() }] });
    }
    let mut all = names;
    all.extend(stamped);
    let mut s = String::new();
    for (i, n) in all.iter().enumerate() {
        let c = n.clone();
        writeln!(s, "#{i} dbg {:?}", n).unwrap();
        writeln!(s, "#{i} alt {:#?}", n).unwrap();
        writeln!(s, "#{i} clone_dbg_eq {} clone_eq {} clone_cmp {:?}", format!("{c:?}") == format!("{n:?}"), c == *n, c.cmp(n)).unwrap();
        writeln!(s, "#{i} hash {}", sip(n)).unwrap();
        writeln!(s, "#{i} json {}", serde_json::to_string(n).unwrap()).unwrap();
        writeln!(s, "#{i} pretty {}", serde_json::to_string_pretty(n).unwrap()).unwrap();
        let back: Result<StableName, _> = serde_json::from_str(&serde_json::to_string(n).unwrap());
        writeln!(s, "#{i} back {}", match back { Ok(b) => format!("eq={}", b == *n), Err(e) => format!("err={e}") }).unwrap();
        // Formatter flags beyond alternate.
        writeln!(s, "#{i} hex {:x?}", n).unwrap();
        writeln!(s, "#{i} width [{:6?}]", n).unwrap();
        writeln!(s, "#{i} pathdbg {:?}", n.path).unwrap();
        writeln!(s, "#{i} pathhash {}", sip_any(&n.path)).unwrap();
    }
    // Pairwise.
    let mut eqm = String::new();
    let mut cmpm = String::new();
    for a in &all {
        for b in &all {
            eqm.push(if a == b { '1' } else { '0' });
            cmpm.push(match a.cmp(b) { Ordering::Less => '<', Ordering::Equal => '=', Ordering::Greater => '>' });
            // partial_cmp and ne agree.
            assert_eq!(a.partial_cmp(b), Some(a.cmp(b)));
            assert_eq!(a != b, !(a == b));
        }
        eqm.push('\n');
        cmpm.push('\n');
    }
    writeln!(s, "EQ\n{eqm}CMP\n{cmpm}").unwrap();
    // Full order via sort, and a BTreeSet.
    let mut idx: Vec<usize> = (0..all.len()).collect();
    idx.sort_by(|&i, &j| all[i].cmp(&all[j]));
    writeln!(s, "SORT {idx:?}").unwrap();
    let set: std::collections::BTreeSet<StableName> = all.iter().cloned().collect();
    writeln!(s, "SETLEN {}", set.len()).unwrap();
    let hs: std::collections::HashSet<StableName> = all.iter().cloned().collect();
    writeln!(s, "HSETLEN {}", hs.len()).unwrap();
    // Hash-Eq consistency on this build.
    for a in &all {
        for b in &all {
            if a == b {
                assert_eq!(sip(a), sip(b), "Hash agrees with Eq");
            }
        }
    }
    std::fs::write(out_path, s).unwrap();
}

#[test]
fn zz_probe_differential() {
    std::thread::Builder::new()
        .stack_size(512 << 20)
        .spawn(run_probe)
        .unwrap()
        .join()
        .unwrap();
}
