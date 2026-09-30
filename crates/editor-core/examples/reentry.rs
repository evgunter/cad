//! Review probe (namedepth-r1): the thread-local shallow mode under a
//! panic inside a walk, and under re-entrant walks from a hasher.
#![allow(clippy::all, missing_docs)]
use editor_core::names::*;
use editor_core::RecipeNodeId;
use std::hash::{Hash, Hasher};

fn chain(depth: usize, bottom: u64) -> StableName {
    let mut n = StableName { kind: EntityKind::Face, node: RecipeNodeId(bottom), path: vec![RoleSeg::Cap(CapEnd::End)] };
    for _ in 0..depth {
        n = StableName { kind: EntityKind::Face, node: RecipeNodeId(5), path: vec![RoleSeg::FromA(NameRef::new(n))] };
    }
    n
}

struct Panicking(usize);
impl Hasher for Panicking {
    fn finish(&self) -> u64 { 0 }
    fn write(&mut self, _: &[u8]) {
        self.0 += 1;
        if self.0 == 40 { panic!("hasher panics mid-walk"); }
    }
}

/// A hasher that, on its first write, renders and hashes ANOTHER name.
struct Nosy { log: Vec<u8>, other: StableName, shown: Option<String>, inner: Option<u64>, done: bool }
impl Hasher for Nosy {
    fn finish(&self) -> u64 { 0 }
    fn write(&mut self, b: &[u8]) {
        self.log.extend_from_slice(b);
        if !self.done {
            self.done = true;
            self.shown = Some(format!("{:?}", self.other));
            let mut h = std::collections::hash_map::DefaultHasher::new();
            self.other.hash(&mut h);
            self.inner = Some(h.finish());
        }
    }
}

fn main() {
    std::thread::Builder::new().stack_size(1 << 20).spawn(|| {
        for depth in [3usize, 100, 5000] {
            let (a, b) = (chain(depth, 1), chain(depth, 2));
            let r = std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| {
                let mut h = Panicking(0);
                a.hash(&mut h);
            }));
            println!("depth {depth}: hash panicked={}", r.is_err());
            // After the panic: every walk on this thread still answers right.
            println!("  after panic: a==b {} a<b {:?} a==a.clone {} dbg_has_bottom {}",
                a == b, a.cmp(&b), a == a.clone(), format!("{b:?}").contains("RecipeNodeId(2)"));
            let other = chain(depth.min(50), 7);
            let expect_shown = format!("{other:?}");
            let mut h0 = std::collections::hash_map::DefaultHasher::new();
            other.hash(&mut h0);
            let expect_inner = h0.finish();
            let mut nosy = Nosy { log: Vec::new(), other: other.clone(), shown: None, inner: None, done: false };
            a.hash(&mut nosy);
            println!("  re-entrant Debug inside a hash: complete={}", nosy.shown.as_deref() == Some(expect_shown.as_str()));
            println!("  re-entrant Hash inside a hash: same={} ", nosy.inner == Some(expect_inner));
        }
    }).unwrap().join().unwrap();
}
