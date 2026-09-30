//! Differential harness (review namedepth-r1): prints, for a generated
//! corpus of names, every derived-impl answer; run on main and on the
//! head and diff the output.
#![allow(clippy::all, clippy::pedantic, clippy::nursery, missing_docs)]
#![allow(unused)]

use editor_core::names::*;
use editor_core::{RecipeNodeId, StepId};
use sha2::{Digest, Sha256};
use std::cmp::Ordering;
use std::fmt::Write as _;
use std::hash::BuildHasher;

struct Rng(u64, u64, Option<u64>);
impl Rng {
    fn next(&mut self) -> u64 {
        let mut x = self.0;
        x ^= x << 13;
        x ^= x >> 7;
        x ^= x << 17;
        self.0 = x;
        self.1 += 1;
        if Some(self.1) == self.2 {
            return x.wrapping_add(1);
        }
        x
    }
    fn below(&mut self, n: u64) -> u64 {
        self.next() % n
    }
}

fn dig(s: &str) -> String {
    let h = Sha256::digest(s.as_bytes());
    h.iter().take(8).map(|b| format!("{b:02x}")).collect()
}

struct Gen {
    rng: Rng,
    pool: Vec<NameRef>,
}

fn piece_role(r: &mut Rng) -> PieceRole {
    match r.below(5) {
        0 => PieceRole::Leg,
        1 => PieceRole::RunIn,
        2 => PieceRole::Arc,
        3 => PieceRole::RunOut,
        _ => PieceRole::Piece(r.below(3) as u32),
    }
}
fn edge(r: &mut Rng) -> ProfileEdgeRef {
    if r.below(2) == 0 {
        ProfileEdgeRef::Piece { step: StepId(r.below(2)), role: piece_role(r) }
    } else {
        ProfileEdgeRef::Section {
            circle: if r.below(2) == 0 { SectionCircle::Outer } else { SectionCircle::Bore },
            role: piece_role(r),
        }
    }
}
fn vertex(r: &mut Rng) -> ProfileVertexRef {
    if r.below(2) == 0 {
        ProfileVertexRef::Piece { step: StepId(r.below(2)), role: piece_role(r) }
    } else {
        ProfileVertexRef::Section {
            circle: if r.below(2) == 0 { SectionCircle::Outer } else { SectionCircle::Bore },
            role: piece_role(r),
        }
    }
}
fn cap(r: &mut Rng) -> CapEnd {
    if r.below(2) == 0 { CapEnd::Start } else { CapEnd::End }
}
fn mer(r: &mut Rng) -> MeridianEnd {
    [MeridianEnd::Start, MeridianEnd::End, MeridianEnd::Seam, MeridianEnd::Pi][r.below(4) as usize]
}
fn half(r: &mut Rng) -> SplitHalf {
    if r.below(2) == 0 { SplitHalf::Above } else { SplitHalf::Below }
}
fn verdict(r: &mut Rng) -> SideVerdict {
    [SideVerdict::Positive, SideVerdict::Negative, SideVerdict::Mixed, SideVerdict::On][r.below(4) as usize]
}

const VARIANTS: u64 = 49;

impl Gen {
    fn name(&mut self, budget: u32) -> StableName {
        let wide = if self.rng.below(4) == 0 { 4 } else { 2 };
        let kind = [EntityKind::Face, EntityKind::Edge, EntityKind::Vertex, EntityKind::Body]
            [self.rng.below(wide) as usize];
        let node = RecipeNodeId(self.rng.below(3));
        let len = match self.rng.below(8) {
            0 => 0,
            1 | 2 | 3 | 4 => 1,
            5 | 6 => 2,
            _ => 3,
        };
        let path = (0..len).map(|_| self.seg(budget)).collect();
        StableName { kind, node, path }
    }
    fn held(&mut self, budget: u32) -> StableName {
        if budget == 0 {
            StableName { kind: EntityKind::Face, node: RecipeNodeId(self.rng.below(2)), path: vec![RoleSeg::Cap(cap(&mut self.rng))] }
        } else {
            self.name(budget - 1)
        }
    }
    fn r(&mut self, budget: u32) -> NameRef {
        match self.rng.below(4) {
            0 if !self.pool.is_empty() => {
                let i = self.rng.below(self.pool.len() as u64) as usize;
                self.pool[i].clone()
            }
            1 if !self.pool.is_empty() => {
                let i = self.rng.below(self.pool.len() as u64) as usize;
                NameRef::new(self.pool[i].name().clone())
            }
            _ => {
                let n = NameRef::new(self.held(budget));
                self.pool.push(n.clone());
                n
            }
        }
    }
    fn set(&mut self, budget: u32) -> Vec<StableName> {
        let k = self.rng.below(4);
        (0..k).map(|_| self.held(budget)).collect()
    }
    fn seg(&mut self, budget: u32) -> RoleSeg {
        use RoleSeg as R;
        let v = self.rng.below(VARIANTS);
        match v {
            0 => R::OutputBody,
            1 => R::Cap(cap(&mut self.rng)),
            2 => R::Lateral(edge(&mut self.rng)),
            3 => R::RimEdge(cap(&mut self.rng), edge(&mut self.rng)),
            4 => R::LateralEdge(vertex(&mut self.rng)),
            5 => R::CapVertex(cap(&mut self.rng), vertex(&mut self.rng)),
            6 => { let k = self.rng.below(3); R::LoftWall((0..k).map(|_| edge(&mut self.rng)).collect()) }
            7 => { let k = self.rng.below(3); R::LoftSeam((0..k).map(|_| vertex(&mut self.rng)).collect()) }
            8 => R::Band(edge(&mut self.rng)),
            9 => R::BandRim(vertex(&mut self.rng)),
            10 => R::BandRimPi(vertex(&mut self.rng)),
            11 => R::BandPi(edge(&mut self.rng)),
            12 => R::Meridian(mer(&mut self.rng), edge(&mut self.rng)),
            13 => R::MeridianVertex(mer(&mut self.rng), vertex(&mut self.rng)),
            14 => R::RevolveCap(mer(&mut self.rng)),
            15 => R::Pole(vertex(&mut self.rng)),
            16 => R::AxisEdge(edge(&mut self.rng)),
            17 => R::FromA(self.r(budget)),
            18 => R::FromB(self.r(budget)),
            19 => { let member = RecipeNodeId(self.rng.below(2)); R::FromMember { member, of: self.r(budget) } }
            20 => { let a = self.r(budget); R::Seam { a, b: self.r(budget) } }
            21 => R::Merged(self.set(budget)),
            22 => {
                let k = self.rng.below(3);
                let v = (0..k).map(|_| { let n = self.held(budget); (n, verdict(&mut self.rng)) }).collect();
                R::Fragment(Qualifier::SideOf(v))
            }
            23 => R::Fragment(Qualifier::OrderAlong { rank: self.rng.below(3) as u32, of: self.rng.below(3) as u32 }),
            24 => R::SplitBody(half(&mut self.rng)),
            25 => R::SectionFace { side: half(&mut self.rng), section: self.rng.below(2) as u32 },
            26 => { let side = half(&mut self.rng); R::SectionEdge { side, face: self.r(budget) } }
            27 => { let side = half(&mut self.rng); R::SplitFragment { side, parent: self.r(budget) } }
            28 => { let side = half(&mut self.rng); R::CrossingVertex { side, edge: self.r(budget) } }
            29 => { let side = half(&mut self.rng); R::OnToolVertex { side, of: self.r(budget) } }
            30 => R::FromTarget(self.r(budget)),
            31 => R::BlendFace(self.r(budget)),
            32 => R::CornerFace(self.r(budget)),
            33 => { let edge = self.r(budget); R::TrimEdge { edge, support: self.r(budget) } }
            34 => { let vertex = self.r(budget); R::FootVertex { vertex, support: self.r(budget) } }
            35 => { let vertex = self.r(budget); R::EndArc { vertex, edge: self.r(budget) } }
            36 => R::BandFace(self.set(budget)),
            37 => { let edge = self.r(budget); R::BandTrim { edge, support: if self.rng.below(2) == 0 { RimSupport::Host } else { RimSupport::Mate } } }
            38 => R::BandFoot(self.r(budget)),
            39 => { let edge = self.r(budget); R::BandCross { edge, band: self.set(budget) } }
            40 => R::BandCut(self.r(budget)),
            41 => { let edge = self.r(budget); R::BandSlit { edge, band: self.set(budget) } }
            42 => R::Inner(self.r(budget)),
            43 => R::Rim(self.r(budget)),
            44 => { let of = self.r(budget); R::HoleRim { of, hole: self.rng.below(2) as u32 } }
            45 => R::InPart { of: self.r(budget) },
            46 => { let i = self.rng.below(2) as u32; R::Instance { i, of: self.r(budget) } }
            _ => R::Cap(cap(&mut self.rng)),
        }
    }
}

/// One level wrapping `n`, chosen by `w`, with sides from `side`.
fn wrap(n: StableName, w: u64, side: &StableName, node: u64) -> StableName {
    use RoleSeg as R;
    let r = NameRef::new(n.clone());
    let s = || NameRef::new(side.clone());
    let seg = match w % 12 {
        0 => R::FromA(r),
        1 => R::Instance { i: 1, of: r },
        2 => R::InPart { of: r },
        3 => R::Merged(vec![side.clone(), n]),
        4 => R::Fragment(Qualifier::SideOf(vec![(n, SideVerdict::On), (side.clone(), SideVerdict::Mixed)])),
        5 => R::Seam { a: r, b: s() },
        6 => R::Seam { a: s(), b: r },
        7 => R::BandCross { edge: s(), band: vec![n] },
        8 => R::FromMember { member: RecipeNodeId(2), of: r },
        9 => R::TrimEdge { edge: r, support: s() },
        10 => R::BandSlit { edge: r, band: vec![side.clone()] },
        _ => R::FromB(r),
    };
    let tail = if w % 5 == 0 { vec![seg, R::Fragment(Qualifier::OrderAlong { rank: 0, of: 1 })] } else { vec![seg] };
    StableName { kind: EntityKind::Face, node: RecipeNodeId(node), path: tail }
}

fn chain(depth: usize, seed: u64, bottom: StableName, side: &StableName, perturb_level: Option<usize>) -> StableName {
    let mut rng = Rng(seed | 1, 0, None);
    let mut n = bottom;
    for level in 0..depth {
        let w = rng.below(12);
        let node = if perturb_level == Some(level) { 99 } else { 5 };
        n = wrap(n, w, side, node);
    }
    n
}

fn leaf(node: u64, c: CapEnd) -> StableName {
    StableName { kind: EntityKind::Face, node: RecipeNodeId(node), path: vec![RoleSeg::Cap(c)] }
}

fn corpus() -> Vec<StableName> {
    let mut out = Vec::new();
    let mut g = Gen { rng: Rng(0x9e3779b97f4a7c15, 0, None), pool: Vec::new() };
    for i in 0..500u32 {
        out.push(g.name(i % 4));
    }
    // Near-twins: the same generation with one draw perturbed.
    for seed in 1..=150u64 {
        let mut a = Gen { rng: Rng(seed * 7919 + 13, 0, None), pool: Vec::new() };
        let na = a.name(3);
        let draws = a.rng.1;
        let k = 1 + (seed * 31) % draws.max(1);
        let mut b = Gen { rng: Rng(seed * 7919 + 13, 0, Some(k)), pool: Vec::new() };
        let nb = b.name(3);
        out.push(na);
        out.push(nb);
    }
    // Deep chains around the native budget.
    let side = leaf(9, CapEnd::Start);
    let side2 = leaf(9, CapEnd::End);
    for depth in [1usize, 2, 3, 10, 40, 62, 63, 64, 65, 66, 67, 70, 128, 130] {
        for seed in [3u64, 11, 17] {
            out.push(chain(depth, seed, leaf(1, CapEnd::End), &side, None));
            out.push(chain(depth, seed, leaf(2, CapEnd::End), &side, None));
            out.push(chain(depth, seed, leaf(1, CapEnd::Start), &side, None));
            out.push(chain(depth, seed, leaf(1, CapEnd::End), &side2, None));
            for p in [0usize, 1, depth / 2, depth.saturating_sub(64), depth.saturating_sub(65), depth.saturating_sub(63), depth - 1] {
                out.push(chain(depth, seed, leaf(1, CapEnd::End), &side, Some(p)));
            }
        }
    }
    out
}

fn main() {
    let h = std::thread::Builder::new().stack_size(1 << 30).spawn(run).unwrap();
    h.join().unwrap();
}

fn hash(n: &impl std::hash::Hash) -> u64 {
    std::hash::BuildHasherDefault::<std::collections::hash_map::DefaultHasher>::default().hash_one(n)
}

fn depth_of(n: &StableName) -> usize {
    // json nesting estimate: count '{' depth via text
    let t = serde_json::to_string(n).unwrap();
    let mut d = 0i64;
    let mut m = 0i64;
    let mut in_s = false;
    let mut esc = false;
    for c in t.chars() {
        if in_s {
            if esc { esc = false } else if c == '\\' { esc = true } else if c == '"' { in_s = false }
            continue;
        }
        match c {
            '"' => in_s = true,
            '{' | '[' => { d += 1; m = m.max(d) }
            '}' | ']' => d -= 1,
            _ => {}
        }
    }
    m as usize
}

fn run() {
    let names = corpus();
    let n = names.len();
    let mut out = String::new();
    writeln!(out, "names {n}").unwrap();
    for (i, a) in names.iter().enumerate() {
        let dbg = format!("{a:?}");
        let pretty = format!("{a:#?}");
        let hex = format!("{a:x?}");
        let width = format!("{a:>3?}");
        let path_dbg = format!("{:?}", a.path);
        let json = serde_json::to_string(a).unwrap();
        let json_pretty = serde_json::to_string_pretty(a).unwrap();
        let c = a.clone();
        let ceq = c == *a;
        let cdbg = format!("{c:?}") == dbg;
        let back = if depth_of(a) < 120 {
            match serde_json::from_str::<StableName>(&json) {
                Ok(b) => format!("{} {}", b == *a, dig(&format!("{b:?}"))),
                Err(e) => format!("ERR {e}"),
            }
        } else {
            "skip".into()
        };
        let back_pretty = if depth_of(a) < 120 {
            match serde_json::from_str::<StableName>(&json_pretty) {
                Ok(b) => format!("{}", b == *a),
                Err(e) => format!("ERR {e}"),
            }
        } else {
            "skip".into()
        };
        writeln!(
            out,
            "N{i} dbg={} pretty={} hex={} width={} pathdbg={} json={} jsonp={} clone={ceq}/{cdbg} back={back} backp={back_pretty} hash={:016x} pathhash={:016x}",
            dig(&dbg), dig(&pretty), dig(&hex), dig(&width), dig(&path_dbg), dig(&json), dig(&json_pretty), hash(a), hash(&a.path)
        )
        .unwrap();
    }
    // Eq/Ord matrices.
    let mut all_eq = String::new();
    let mut all_ord = String::new();
    let mut hash_violations = 0;
    let mut ord_asym = 0;
    for (i, a) in names.iter().enumerate() {
        let mut row_eq = String::with_capacity(n);
        let mut row_ord = String::with_capacity(n);
        for b in &names {
            let e = a == b;
            row_eq.push(if e { '1' } else { '0' });
            let o = a.cmp(b);
            row_ord.push(match o { Ordering::Less => '<', Ordering::Equal => '=', Ordering::Greater => '>' });
            if e && hash(a) != hash(b) {
                hash_violations += 1;
            }
            if (o == Ordering::Equal) != e {
                ord_asym += 1;
            }
            if a.partial_cmp(b) != Some(o) || (a < b) != (o == Ordering::Less) {
                ord_asym += 1;
            }
            if b.cmp(a) != o.reverse() {
                ord_asym += 1;
            }
        }
        writeln!(out, "E{i} {}", dig(&row_eq)).unwrap();
        writeln!(out, "O{i} {}", dig(&row_ord)).unwrap();
        all_eq.push_str(&row_eq);
        all_ord.push_str(&row_ord);
    }
    writeln!(out, "EQ-ALL {}", dig(&all_eq)).unwrap();
    writeln!(out, "ORD-ALL {}", dig(&all_ord)).unwrap();
    writeln!(out, "hash-eq-violations {hash_violations} ord-inconsistent {ord_asym}").unwrap();
    // Paths as whole: eq/cmp through RoleSeg's derived impl, top level.
    let mut pe = String::new();
    for a in names.iter().take(400) {
        for b in names.iter().take(400) {
            pe.push(match a.path.cmp(&b.path) { Ordering::Less => '<', Ordering::Equal => '=', Ordering::Greater => '>' });
            pe.push(if a.path == b.path { '1' } else { '0' });
        }
    }
    writeln!(out, "PATH-ALL {}", dig(&pe)).unwrap();
    // Sorted order and BTreeSet.
    let mut idx: Vec<usize> = (0..n).collect();
    idx.sort_by(|&x, &y| names[x].cmp(&names[y]));
    writeln!(out, "SORT {}", dig(&format!("{idx:?}"))).unwrap();
    let set: std::collections::BTreeSet<StableName> = names.iter().cloned().collect();
    let order: Vec<String> = set.iter().map(|x| dig(&format!("{x:?}"))).collect();
    writeln!(out, "BTREESET {} {}", set.len(), dig(&order.join(","))).unwrap();
    // NameRef-level compare.
    let refs: Vec<NameRef> = names.iter().map(|x| NameRef::new(x.clone())).collect();
    let mut rr = String::new();
    for a in refs.iter().take(300) {
        for b in refs.iter().take(300) {
            rr.push(match a.cmp(b) { Ordering::Less => '<', Ordering::Equal => '=', Ordering::Greater => '>' });
        }
    }
    writeln!(out, "REF-ALL {}", dig(&rr)).unwrap();
    print!("{out}");
}
