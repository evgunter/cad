//! Review probe (namedepth-r1): `remap_name` over one level holding N
//! carried names (a merged set of N members, each a boolean survivor),
//! timed for growing N.
#![allow(clippy::all, missing_docs)]
use editor_core::names::*;
use editor_core::{RecipeNodeId, remap_name};

fn main() {
    let map = (0..=3u64).map(|n| (RecipeNodeId(n), RecipeNodeId(n + 100))).collect();
    for n in [250usize, 500, 1000, 2000, 4000] {
        let member = |i: usize| StableName {
            kind: EntityKind::Face,
            node: RecipeNodeId(2),
            path: vec![RoleSeg::FromA(NameRef::new(StableName {
                kind: EntityKind::Face,
                node: RecipeNodeId(1),
                path: vec![RoleSeg::Instance { i: i as u32, of: NameRef::new(StableName {
                    kind: EntityKind::Face, node: RecipeNodeId(0), path: vec![RoleSeg::Cap(CapEnd::End)] }) }],
            }))],
        };
        let name = StableName {
            kind: EntityKind::Face,
            node: RecipeNodeId(3),
            path: vec![RoleSeg::Merged((0..n).map(member).collect())],
        };
        let t = std::time::Instant::now();
        let out = remap_name(&name, &map, &Default::default()).unwrap();
        println!("N={n}: {:?} (node {})", t.elapsed(), out.node.0);
    }
}
