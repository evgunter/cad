//! namedepth-r2 (head only): the thread-local mode under panic, error,
//! re-entrancy and threads; the JSON door off its text route.
#![allow(clippy::all, clippy::pedantic, clippy::nursery, clippy::restriction)]
#![allow(unused)]

use super::nest;
use super::role::{CapEnd, EntityKind, NameRef, Qualifier, RoleSeg, StableName};
use crate::node::RecipeNodeId;
use core::hash::{Hash, Hasher};
use std::cell::RefCell;

fn leaf(node: u64) -> StableName {
    StableName { kind: EntityKind::Face, node: RecipeNodeId(node), path: vec![RoleSeg::Cap(CapEnd::End)] }
}

fn tower(depth: usize, bottom: u64) -> StableName {
    (0..depth).fold(leaf(bottom), |n, i| StableName {
        kind: EntityKind::Face,
        node: RecipeNodeId(7),
        path: vec![if i % 2 == 0 { RoleSeg::FromA(NameRef::new(n)) } else { RoleSeg::Merged(vec![leaf(3), n]) }],
    })
}

fn sip(n: &StableName) -> u64 {
    let mut h = std::collections::hash_map::DefaultHasher::new();
    n.hash(&mut h);
    h.finish()
}

/// Everything a clean thread answers, for `depth`.
fn fingerprint(depth: usize) -> (bool, bool, core::cmp::Ordering, u64, usize, String) {
    let (a, b, a2) = (tower(depth, 1), tower(depth, 2), tower(depth, 1));
    (
        a == b,
        a == a2,
        a.cmp(&b),
        sip(&a),
        format!("{a:?}").matches("StableName {").count(),
        a.to_json().unwrap(),
    )
}

struct PanicAfter(usize);
impl Hasher for PanicAfter {
    fn finish(&self) -> u64 {
        0
    }
    fn write(&mut self, _: &[u8]) {
        if self.0 == 0 {
            panic!("planted hasher panic");
        }
        self.0 -= 1;
    }
}

struct FailAfter(usize);
impl core::fmt::Write for FailAfter {
    fn write_str(&mut self, s: &str) -> core::fmt::Result {
        if self.0 < s.len() {
            return Err(core::fmt::Error);
        }
        self.0 -= s.len();
        Ok(())
    }
}

#[test]
fn zz_r2_mode_after_panic_error_and_reentry() {
    std::thread::Builder::new()
        .stack_size(1 << 20)
        .spawn(|| {
            for depth in [3usize, 64, 65, 300] {
                let clean = std::thread::Builder::new()
                    .stack_size(1 << 20)
                    .spawn(move || fingerprint(depth))
                    .unwrap()
                    .join()
                    .unwrap();
                // A hasher that panics mid-walk, at several points.
                for k in [0usize, 1, 5, 40, 200, 700] {
                    let n = tower(depth, 1);
                    let r = std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| {
                        let mut h = PanicAfter(k);
                        n.hash(&mut h);
                    }));
                    let _ = r;
                    assert_eq!(fingerprint(depth), clean, "after a hasher panic at write {k}, depth {depth}");
                }
                // A writer that fails mid-Debug.
                for k in [0usize, 10, 100, 1000] {
                    let n = tower(depth, 1);
                    let _ = core::fmt::write(&mut FailAfter(k), format_args!("{n:?}"));
                    let _ = core::fmt::write(&mut FailAfter(k), format_args!("{n:#?}"));
                    assert_eq!(fingerprint(depth), clean, "after a failed Debug write at {k}, depth {depth}");
                }
                // A malformed door read that fails mid-level.
                let bad = tower(depth, 1).to_json().unwrap().replacen("\"Cap\"", "\"Cop\"", 1);
                assert!(StableName::from_json(&bad).is_err());
                assert_eq!(fingerprint(depth), clean, "after a failed door read, depth {depth}");
            }
            eprintln!("R2 mode: panics, fmt errors and failed reads leave no mode behind");
        })
        .unwrap()
        .join()
        .unwrap();
}

/// A hasher whose `write` does name work: re-entrancy inside a hash
/// level.
struct Reentrant<'a> {
    other: &'a StableName,
    seen: RefCell<Vec<String>>,
    inner: std::collections::hash_map::DefaultHasher,
}
impl Hasher for Reentrant<'_> {
    fn finish(&self) -> u64 {
        self.inner.finish()
    }
    fn write(&mut self, b: &[u8]) {
        self.inner.write(b);
        if self.seen.borrow().is_empty() {
            let o = self.other;
            let c = o.clone();
            self.seen.borrow_mut().push(format!(
                "hash={} dbg_levels={} eq_self={} eq_other_bottom={} cmp={:?} clone_eq={} json_len={}",
                sip(o),
                format!("{o:?}").matches("StableName {").count(),
                *o == tower(5, 1),
                *o == tower(5, 2),
                o.cmp(&tower(5, 2)),
                c == *o,
                o.to_json().map(|t| t.len()).unwrap_or(0),
            ));
        }
    }
}

#[test]
fn zz_r2_reentry_inside_a_hash_level() {
    let other = tower(5, 1);
    let c = other.clone();
    let expect = format!(
        "hash={} dbg_levels={} eq_self={} eq_other_bottom={} cmp={:?} clone_eq={} json_len={}",
        sip(&other),
        format!("{other:?}").matches("StableName {").count(),
        other == tower(5, 1),
        other == tower(5, 2),
        other.cmp(&tower(5, 2)),
        c == other,
        other.to_json().unwrap().len(),
    );
    // Hash a name that holds names, so write() runs inside a shallow
    // hash level (the first level's own fields).
    let n = tower(3, 9);
    let mut h = Reentrant { other: &other, seen: RefCell::new(vec![]), inner: Default::default() };
    n.hash(&mut h);
    let got = h.seen.borrow()[0].clone();
    eprintln!("R2 reentry expect {expect}");
    eprintln!("R2 reentry got    {got}");
}

#[test]
fn zz_r2_door_off_the_text_route() {
    let n = tower(4, 1);
    let v = serde_json::to_value(&n).unwrap();
    let from_value = nest::json_door(|| serde_json::from_value::<StableName>(v.clone()));
    eprintln!("R2 door from_value: {:?}", from_value.as_ref().map(|x| *x == n).map_err(|e| e.to_string()));
    let to_value = nest::json_door(|| serde_json::to_value(&n));
    eprintln!("R2 door to_value eq: {:?}", to_value.as_ref().map(|x| *x == v).map_err(|e| e.to_string()));
    let deep = tower(300, 1);
    let to_value_deep = nest::json_door(|| serde_json::to_value(&deep));
    eprintln!("R2 door to_value deep: {:?}", to_value_deep.as_ref().map(|_| "ok").map_err(|e| e.to_string()));
    let pretty = nest::json_door(|| serde_json::to_string_pretty(&n)).unwrap();
    eprintln!("R2 door pretty == derived pretty: {}", pretty == serde_json::to_string_pretty(&n).unwrap());
    // Error text/position: a nested bad variant, door vs derived.
    let bad = n.to_json().unwrap().replacen("\"Cap\"", "\"Cop\"", 1);
    let derived = serde_json::from_str::<StableName>(&bad).unwrap_err().to_string();
    let door = StableName::from_json(&bad).unwrap_err().to_string();
    eprintln!("R2 door err: derived=[{derived}] door=[{door}]");
    // Whitespace before the root, and a root that is not an object.
    let spaced = format!("  \n {}", n.to_json().unwrap());
    eprintln!("R2 door leading ws: {:?}", StableName::from_json(&spaced).map(|x| x == n).map_err(|e| e.to_string()));
    eprintln!("R2 door null: door=[{:?}] derived=[{:?}]", StableName::from_json("null").map_err(|e| e.to_string()).err(), serde_json::from_str::<StableName>("null").map_err(|e| e.to_string()).err());
    // A key named "kind" inside a segment's object: derived refuses as
    // unknown field; what does the door say?
    let odd = r#"{"kind":"Face","node":1,"path":[{"Lateral":{"Piece":{"step":1,"role":"Leg","kind":"Face"}}}]}"#;
    eprintln!("R2 door odd key: door=[{:?}] derived=[{:?}]", StableName::from_json(odd).map_err(|e| e.to_string()).err(), serde_json::from_str::<StableName>(odd).map_err(|e| e.to_string()).err());
    // A doc-level struct inside the door holding a name through a
    // sequence and an Option.
    #[derive(serde::Serialize, serde::Deserialize, PartialEq, Debug)]
    struct Holder {
        a: Option<StableName>,
        b: Vec<(u8, StableName)>,
    }
    let h = Holder { a: Some(tower(200, 1)), b: vec![(1, tower(150, 2)), (2, leaf(3))] };
    let text = nest::json_door(|| serde_json::to_string(&h)).unwrap();
    let back: Holder = nest::json_door(|| serde_json::from_str(&text)).unwrap();
    eprintln!("R2 door holder round trip: {}", back == h);
}

#[test]
fn zz_r2_threads() {
    use std::sync::Arc;
    let names: Arc<Vec<StableName>> = Arc::new((0..64).map(|i| tower(100 + (i % 5), i as u64 % 3)).collect());
    let serial: Vec<(u64, bool, core::cmp::Ordering)> =
        names.iter().zip(names.iter().rev()).map(|(a, b)| (sip(a), a == b, a.cmp(b))).collect();
    let hs: Vec<_> = (0..8)
        .map(|t| {
            let names = names.clone();
            std::thread::Builder::new()
                .stack_size(1 << 20)
                .spawn(move || {
                    (0..50)
                        .map(|_| {
                            names
                                .iter()
                                .zip(names.iter().rev())
                                .map(|(a, b)| (sip(a), a == b, a.cmp(b)))
                                .collect::<Vec<_>>()
                        })
                        .collect::<Vec<_>>()
                })
                .unwrap()
        })
        .collect();
    for h in hs {
        for run in h.join().unwrap() {
            assert_eq!(run, serial);
        }
    }
    eprintln!("R2 threads: 8 threads x 50 runs agree with the serial answers");
}

/// The smallest thread stack on which the native routes (64 levels of
/// Eq, Ord, Hash, then heap) survive a worst-case pair, measured by
/// re-running this test in a child with a given stack.
#[test]
fn zz_r2_native_stack() {
    if let Ok(size) = std::env::var("ZZ_STACK") {
        let size: usize = size.parse().unwrap();
        let which = std::env::var("ZZ_WHICH").unwrap();
        std::thread::Builder::new()
            .stack_size(size)
            .spawn(move || {
                let (a, b) = (tower(200, 1), tower(200, 2));
                match which.as_str() {
                    "eq" => assert!(a != b),
                    "ord" => assert!(a < b),
                    _ => {
                        let _ = sip(&a);
                    }
                }
            })
            .unwrap()
            .join()
            .unwrap();
        return;
    }
    let exe = std::env::current_exe().unwrap();
    for which in ["eq", "ord", "hash"] {
        for kib in [16usize, 32, 48, 64, 96, 128, 192, 256, 384, 512] {
            let st = std::process::Command::new(&exe)
                .args(["--exact", "names::zz_probe2::zz_r2_native_stack", "--test-threads=1"])
                .env("ZZ_STACK", (kib * 1024).to_string())
                .env("ZZ_WHICH", which)
                .output()
                .unwrap();
            eprintln!("R2 native stack {which} {kib} KiB: {}", st.status.success());
        }
    }
}
