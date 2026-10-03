//! The m10-p fence's corpus walk as tab-separated lines: one per node
//! (`POISONED`, `FAILED` or `OK` with its kind) and one per body point
//! (`PT`, its key, its bits and its decimal value), then a `SUMMARY`
//! line counting nodes and coordinates. It prints the corpus half of
//! `m10_p_fence::walk` and skips the arc-carrier fixture; the
//! `RSTRUCT`/`RCOORD` form in `cert3r1_dump` prints both halves at
//! both scalars.
//!
//! `#[ignore]`d: it asserts nothing and gates nothing. Run it with
//!
//! ```text
//! cargo test -p editor-core --test all \
//!     -- --ignored --nocapture r2_cert3_coord_dump
//! ```

#![allow(clippy::unwrap_used, clippy::expect_used, clippy::panic)]

use crate::m10_p_fence::{Outcome, Seen, walk};

/// Dump every corpus observable the fence digests, in the same order.
fn dump<T: editor_core::EvalScalar>(tag: &str, bits: impl Fn(&geom_core::Point3<T>) -> String) {
    let mut coords = 0usize;
    let mut nodes = 0usize;
    walk::<T>(|seen| match seen {
        Seen::Fixture(_)
        | Seen::FixtureLoop { .. }
        | Seen::FixtureVertex { .. }
        | Seen::FixtureRefused(_)
        | Seen::Document(_) => {}
        Seen::Node { doc, id, outcome } => {
            nodes += 1;
            match outcome {
                Outcome::Poisoned { through } => {
                    println!("{tag}\t{doc}\t{id}\tPOISONED\t{through}");
                }
                Outcome::Failed => println!("{tag}\t{doc}\t{id}\tFAILED"),
                Outcome::Ok { kind } => println!("{tag}\t{doc}\t{id}\tOK\t{kind}"),
            }
        }
        Seen::Point {
            doc, id, key, p, ..
        } => {
            coords += 3;
            println!("{tag}\t{doc}\t{id}\tPT\t{key:?}\t{}", bits(p));
        }
    });
    println!("{tag}\tSUMMARY\tnodes={nodes}\tcoords={coords}");
}

#[test]
#[ignore = "a diff instrument over the fence's walk; asserts nothing, run with --ignored"]
fn r2_dump_corpus_coordinates_f64() {
    dump::<f64>("f64", |p| {
        format!(
            "{:016x} {:016x} {:016x} | {:?} {:?} {:?}",
            p.x.to_bits(),
            p.y.to_bits(),
            p.z.to_bits(),
            p.x,
            p.y,
            p.z
        )
    });
}
