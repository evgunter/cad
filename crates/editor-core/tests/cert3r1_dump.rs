//! **The fence's diff instrument.** When an `m10_p_fence` digest
//! moves, the digest says only THAT the corpus's evaluation moved;
//! this file says WHAT moved. It runs the fence's own walk
//! (`m10_p_fence::walk`) and prints every observable the digest hashes
//! instead of folding it: one `RSTRUCT` line per fixture program and
//! per node (its outcome), and one `RCOORD` line per coordinate, bits
//! in hex. Run it on the tree whose digest moved and on the tree it
//! moved from, keep the `RSTRUCT`/`RCOORD` lines, and diff them. The
//! fence header's re-derivation paragraphs quote measurements taken
//! this way.
//!
//! `#[ignore]`d: it asserts nothing and gates nothing, so it runs only
//! when asked. Run it with
//!
//! ```text
//! cargo test -p editor-core --test all \
//!     -- --ignored --nocapture cert3r1_dump
//! ```

#![allow(clippy::unwrap_used, clippy::expect_used, clippy::panic)]

use crate::m10_p_fence::{Outcome, Seen, walk};

/// Print the fence's walk at `T`, tagging every line with `lane`.
/// `coord` prints one body point, `scalar` one fixture scalar.
fn dump<T: editor_core::EvalScalar>(
    lane: &str,
    coord: impl Fn(&str, &geom_core::Point3<T>),
    scalar: impl Fn(&str, T),
) {
    walk::<T>(|seen| match seen {
        Seen::Fixture(_) | Seen::Document(_) => {}
        Seen::FixtureLoop { i, vertices } => {
            println!("RSTRUCT {lane} fixture{i} ok {vertices}");
        }
        Seen::FixtureVertex { i, j, x, y, bulge } => {
            scalar(&format!("{lane} fixture{i} v{j} x"), x);
            scalar(&format!("{lane} fixture{i} v{j} y"), y);
            scalar(&format!("{lane} fixture{i} v{j} b"), bulge);
        }
        Seen::FixtureRefused(i) => println!("RSTRUCT {lane} fixture{i} refused"),
        Seen::Node { doc, id, outcome } => match outcome {
            Outcome::Poisoned { through } => {
                println!("RSTRUCT {lane} {doc} {id} poisoned {through}");
            }
            Outcome::Failed => println!("RSTRUCT {lane} {doc} {id} failed"),
            Outcome::Ok { kind } => println!("RSTRUCT {lane} {doc} {id} ok {kind}"),
        },
        Seen::Point { doc, id, i, p, .. } => coord(&format!("{lane} {doc} {id} {i}"), p),
    });
}

#[test]
#[ignore = "the fence's diff instrument; asserts nothing, run with --ignored"]
fn r1_dump_f64() {
    dump::<f64>(
        "f64",
        |tag, p| {
            for (a, c) in [("x", p.x), ("y", p.y), ("z", p.z)] {
                println!("RCOORD {tag} {a} {:016x}", c.to_bits());
            }
        },
        |tag, v: f64| println!("RCOORD {tag} {:016x}", v.to_bits()),
    );
}

#[test]
#[ignore = "the fence's diff instrument; asserts nothing, run with --ignored"]
fn r1_dump_interval() {
    use geom_core::{Bounds, Interval};
    dump::<Interval>(
        "iv",
        |tag, p| {
            for (a, c) in [("x", p.x), ("y", p.y), ("z", p.z)] {
                println!(
                    "RCOORD {tag} {a} {:016x} {:016x}",
                    c.lo().to_bits(),
                    c.hi().to_bits()
                );
            }
        },
        |tag, v: Interval| {
            println!(
                "RCOORD {tag} {:016x} {:016x}",
                v.lo().to_bits(),
                v.hi().to_bits()
            );
        },
    );
}
