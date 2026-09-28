//! **The reviewer bit-identity dump**: one body written out bit for bit,
//! and the `BITDUMP_DIR` channel that arms the rows that write it. A
//! base/head `diff` of the files is the verdict (`bitdump`'s own module
//! doc says how to take it). What a suite reads OFF a body it built, so
//! it routes here beside [`super::pcurve_rows`] ([`super`]'s routing
//! rule).
//!
//! **One home, not a copy per row.** A dump row that writes through a
//! second copy is silently blind to whatever that copy leaves out, so
//! every armed row writes through [`dump`].
//!
//! **Deliberately not absorbed**, and the whole of it:
//! `offd_r1_probes`' `dump`, which is the body's whole `Debug`, compared
//! for equality between two builds in one run rather than diffed as a
//! file.

use std::fmt::Write as _;

use geom_core::Tol;
use topo::readback::euler_counts;
use topo::{Body, LoopBoundary};

/// Dump one body, bit for bit, in key iteration order (identical
/// operation sequences produce identical key orders): the Euler counts,
/// every vertex, every edge's carrier, window and description, every
/// face with each of its loops' points in cycle order, and the mass
/// properties.
pub fn dump(body: &Body<f64>) -> String {
    let mut s = String::new();
    let counts = euler_counts(body);
    let _ = writeln!(
        s,
        "census V={} E={} F={} L={} S={} R={}",
        counts.v,
        counts.e,
        counts.f,
        body.loops().count(),
        counts.s,
        counts.r,
    );
    for (k, _) in body.vertices() {
        let p = body
            .get_vertex(k)
            .and_then(|v| body.get_point(v.point))
            .unwrap();
        let _ = writeln!(s, "V {k:?} ({:?}, {:?}, {:?})", p.x, p.y, p.z);
    }
    for (k, e) in body.edges() {
        let _ = write!(s, "E {k:?} he+={:?} he-={:?}", e.he_plus, e.he_minus);
        match body.get_curve_geom(e.curve).and_then(|g| g.certified()) {
            Some(c) => {
                let (t0, t1) = c.params();
                let _ = writeln!(
                    s,
                    " carrier={:?} params=({t0:?}, {t1:?}) desc={:?}",
                    c.carrier(),
                    c.description()
                );
            }
            None => {
                let _ = writeln!(s, " UNCERTIFIED");
            }
        }
    }
    for (k, _) in body.faces() {
        let fd = body.get_face(k).unwrap();
        let surf = body.get_surface(fd.surface).unwrap();
        let _ = writeln!(
            s,
            "F {k:?} sense={:?} rings={} surface={surf:?}",
            fd.sense,
            fd.rings.len()
        );
        for lk in core::iter::once(fd.outer).chain(fd.rings.iter().copied()) {
            let LoopBoundary::Cycle { first } = body.get_loop(lk).unwrap().boundary else {
                let _ = writeln!(s, "  L {lk:?} empty");
                continue;
            };
            let _ = write!(s, "  L {lk:?}");
            for he in body.loop_cycle(first).unwrap() {
                let start = body.get_half_edge(he).unwrap().start;
                let p = body
                    .get_vertex(start)
                    .and_then(|v| body.get_point(v.point))
                    .unwrap();
                let _ = write!(s, " ({:?}, {:?}, {:?})", p.x, p.y, p.z);
            }
            let _ = writeln!(s);
        }
    }
    let props = topo::mass_properties(body, Tol::witness()).unwrap();
    let _ = writeln!(
        s,
        "props volume={:?} pad={:?} area={:?} apad={:?}",
        props.volume, props.volume_pad, props.surface_area, props.area_pad
    );
    s
}

/// The dump directory, or `None` when the dump rows are not armed.
///
/// **Why an env read is admissible here, stated rather than assumed**
/// (`sweep`'s manifest warns that a suite rolling its own dial would be
/// a second `CAD_FUZZ`-style channel). The gate that bans ambient
/// environment scans `crates/*/src` and this is a `tests/` file, so no
/// shipped build can reach it — the same REACHABILITY argument that
/// allowlists `test-utils`' fuzz dial. And unlike a dial, this one
/// gates no assertion: armed, the rows write a file and assert nothing
/// about it; unarmed, they return before building anything. It selects
/// an artifact's destination, never a behaviour.
pub fn dump_dir() -> Option<String> {
    std::env::var("BITDUMP_DIR").ok().filter(|d| !d.is_empty())
}

/// Write `text` to `<dir>/<name>.txt`, creating `dir` if need be.
pub fn save(dir: &str, name: &str, text: &str) {
    std::fs::create_dir_all(dir).unwrap();
    std::fs::write(format!("{dir}/{name}.txt"), text).unwrap();
}
