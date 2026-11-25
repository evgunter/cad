//! MEASUREMENT ONLY (`embedwalk/split-ground-instrument`, not for merge):
//! every split of the injectivity walk recorded with its reason, its
//! parent and its time, written per ask where `CAD_SPLIT_GROUND=<dir>` is
//! set. No verdict changes.
#![allow(missing_docs, clippy::pedantic, clippy::nursery)]

use std::cell::{Cell, RefCell};
use std::fmt::Write as _;
use std::io::Write as _;

use crate::FaceKey;

#[derive(Default)]
struct Node {
    parent: u32,
    depth: u32,
    k: usize,
    reason: u32,
    secs: f64,
    stop: &'static str,
    lo: f64,
    hi: f64,
}

#[derive(Default)]
struct Rec {
    nodes: Vec<Node>,
    faces: Vec<(FaceKey, FaceKey, &'static str)>,
    refusal: Option<(u32, String, f64)>,
    start: Option<std::time::Instant>,
}

thread_local! {
    static ON: Cell<bool> = const { Cell::new(false) };
    static REC: RefCell<Rec> = RefCell::new(Rec::default());
    static CODES: RefCell<Vec<String>> = const { RefCell::new(Vec::new()) };
    static REASON: RefCell<Option<String>> = const { RefCell::new(None) };
    static LOCAL: Cell<u32> = const { Cell::new(0) };
    static MARGIN: Cell<Option<&'static str>> = const { Cell::new(None) };
    static CUR: Cell<u32> = const { Cell::new(0) };
    static DOOR: Cell<&'static str> = const { Cell::new("direct") };
    static SEQ: Cell<u32> = const { Cell::new(0) };
}

pub fn on() -> bool {
    ON.with(Cell::get)
}

fn dir() -> Option<std::ffi::OsString> {
    std::env::var_os("CAD_SPLIT_GROUND")
}

pub fn gate_enabled() -> bool {
    dir().is_some()
}

pub fn door(d: &'static str) {
    DOOR.with(|x| x.set(d));
}

/// Starts recording an ask. Node 0 is "no node".
pub fn start() {
    if dir().is_none() {
        return;
    }
    ON.with(|o| o.set(true));
    CUR.with(|c| c.set(0));
    REC.with(|r| {
        *r.borrow_mut() = Rec {
            nodes: vec![Node::default()],
            start: Some(std::time::Instant::now()),
            ..Rec::default()
        }
    });
    CODES.with(|c| c.borrow_mut().clear());
}

fn code(s: String) -> u32 {
    CODES.with(|c| {
        let mut c = c.borrow_mut();
        match c.iter().position(|k| *k == s) {
            Some(i) => i as u32 + 1,
            None => {
                c.push(s);
                c.len() as u32
            }
        }
    })
}

pub fn root(k: usize, a: FaceKey, b: FaceKey, share: &'static str) -> u32 {
    if !on() {
        return 0;
    }
    REC.with(|r| {
        let mut r = r.borrow_mut();
        if r.faces.len() <= k {
            r.faces.resize(k + 1, (a, b, "?"));
        }
        r.faces[k] = (a, b, share);
        r.nodes.push(Node {
            k,
            ..Node::default()
        });
        (r.nodes.len() - 1) as u32
    })
}

// ---- reasons, stashed where they are minted ----

pub fn local(line: u32) {
    if on() {
        LOCAL.with(|l| l.set(line));
    }
}
pub fn take_local() -> u32 {
    LOCAL.with(|l| l.replace(0))
}
pub fn margin(r: &Result<geom_core::Sign, geom_core::Indeterminate>) {
    if !on() {
        return;
    }
    let class = match r {
        Ok(geom_core::Sign::Zero) => "decided-zero",
        Err(e) if e.terminal_sliver => "wholly-in-band",
        Err(_) => "straddles",
        Ok(_) => return,
    };
    MARGIN.with(|m| m.set(Some(class)));
}
pub fn clear() {
    if on() {
        REASON.with(|r| *r.borrow_mut() = None);
        LOCAL.with(|l| l.set(0));
        MARGIN.with(|m| m.set(None));
    }
}
pub fn take_margin() -> &'static str {
    MARGIN
        .with(|m| m.take())
        .unwrap_or("no-undecided-margin")
}
pub fn reason(s: String) {
    if on() {
        REASON.with(|r| *r.borrow_mut() = Some(s));
    }
}
pub fn take_reason() -> Option<String> {
    REASON.with(|r| r.borrow_mut().take())
}

/// The reason the next split (or stop) of the current node carries.
pub fn pending(s: String) {
    if on() {
        let c = code(s);
        let cur = CUR.with(Cell::get);
        REC.with(|r| {
            if let Some(n) = r.borrow_mut().nodes.get_mut(cur as usize) {
                n.reason = c;
            }
        });
    }
}

pub fn sep(lo: f64, hi: f64) {
    let cur = CUR.with(Cell::get);
    REC.with(|r| {
        if let Some(n) = r.borrow_mut().nodes.get_mut(cur as usize) {
            n.lo = lo;
            n.hi = hi;
        }
    });
}

pub fn current(node: u32) {
    CUR.with(|c| c.set(node));
}

/// The current node split into `n` children; their ids.
pub fn split(node: u32, n: usize) -> Vec<u32> {
    if !on() || node == 0 {
        return vec![0; n];
    }
    REC.with(|r| {
        let mut r = r.borrow_mut();
        let (depth, k) = r
            .nodes
            .get(node as usize)
            .map_or((0, 0), |p| (p.depth, p.k));
        (0..n)
            .map(|_| {
                r.nodes.push(Node {
                    parent: node,
                    depth: depth + 1,
                    k,
                    ..Node::default()
                });
                (r.nodes.len() - 1) as u32
            })
            .collect()
    })
}

pub fn stopped(node: u32, why: &'static str) {
    if on() {
        REC.with(|r| {
            if let Some(n) = r.borrow_mut().nodes.get_mut(node as usize) {
                n.stop = why;
            }
        });
    }
}

pub fn time(node: u32, secs: f64) {
    if on() {
        REC.with(|r| {
            if let Some(n) = r.borrow_mut().nodes.get_mut(node as usize) {
                n.secs += secs;
            }
        });
    }
}

/// The ask's first lower-bound refusal, at the current node.
pub fn refused(what: String) {
    if !on() {
        return;
    }
    let cur = CUR.with(Cell::get);
    REC.with(|r| {
        let mut r = r.borrow_mut();
        if r.refusal.is_none() {
            let t = r.start.map_or(0.0, |s| s.elapsed().as_secs_f64());
            let why = REASON.with(|x| x.borrow().clone()).unwrap_or_default();
            r.refusal = Some((cur, format!("{what} [{why}]"), t));
        }
    });
}

fn test_name() -> String {
    std::thread::current()
        .name()
        .unwrap_or("?")
        .to_owned()
}

/// Ends the ask's record and writes it.
pub fn finish(verdict: &str, candidates: usize, splits: usize) {
    if !on() {
        return;
    }
    ON.with(|o| o.set(false));
    let Some(d) = dir() else { return };
    let rec = REC.with(|r| std::mem::take(&mut *r.borrow_mut()));
    let secs = rec.start.map_or(0.0, |s| s.elapsed().as_secs_f64());
    let seq = SEQ.with(|s| {
        s.set(s.get() + 1);
        s.get()
    });
    let ask = format!("{}:{seq}", std::process::id());
    let test = test_name();
    let door = DOOR.with(Cell::get);
    let mut out = String::new();
    let (rnode, rwhat, rt) = rec
        .refusal
        .clone()
        .unwrap_or((0, String::new(), f64::NAN));
    let rk = rec.nodes.get(rnode as usize).map(|n| n.k);
    let rdepth = rec.nodes.get(rnode as usize).map_or(0, |n| n.depth);
    let rfaces = rk
        .and_then(|k| rec.faces.get(k))
        .map(|(a, b, s)| format!("{a:?}/{b:?}/{s}"))
        .unwrap_or_default();
    let _ = writeln!(
        out,
        "ASK\t{ask}\t{test}\t{door}\t{verdict}\t{candidates}\t{splits}\t{secs:.6}\t{rnode}\t{rdepth}\t{}\t{rfaces}\t{rwhat}\t{rt:.6}\t{}",
        rk.map_or(-1, |k| k as i64),
        rec.nodes.len() - 1,
    );
    if verdict != "Embedded" {
        CODES.with(|c| {
            for (i, s) in c.borrow().iter().enumerate() {
                let _ = writeln!(out, "CODE\t{ask}\t{}\t{s}", i + 1);
            }
        });
        for (k, (a, b, s)) in rec.faces.iter().enumerate() {
            let _ = writeln!(out, "PAIR\t{ask}\t{k}\t{a:?}\t{b:?}\t{s}");
        }
        for (i, n) in rec.nodes.iter().enumerate().skip(1) {
            let _ = writeln!(
                out,
                "N\t{ask}\t{i}\t{}\t{}\t{}\t{}\t{:.7}\t{}\t{:.3e}\t{:.3e}",
                n.parent, n.depth, n.k, n.reason, n.secs, n.stop, n.lo, n.hi
            );
        }
    }
    let path = std::path::Path::new(&d).join(format!("{}.log", std::process::id()));
    if let Ok(mut f) = std::fs::OpenOptions::new()
        .create(true)
        .append(true)
        .open(path)
    {
        let _ = f.write_all(out.as_bytes());
    }
}
