//! **FILLET-T review probes (lane r2, PR 1943).** Two rows the unit's
//! own suites do not carry.
//!
//! - `bitdump_ruled_band` — the corpus row `work/fillet/ruled-band-has-no-bit-identity-corpus-row.md`
//!   says is missing. Two of the eight `kef` sites FILLET-T re-routes
//!   through `kef_minted` are `ruled_phase`'s, and no armed dump row
//!   reaches that phase, so C1's "every existing carve is bit-identical
//!   to the merge base" is taken over a corpus that never executes it.
//!   The fixture is the module docs' own rod with a flat milled along
//!   it (`test_support::rod_with_flat`), filleted at both creases.
//!   Armed by `BITDUMP_DIR` exactly as `bitdump.rs` is — clean skip
//!   unarmed — and dumped in that file's format INCLUDING the mass
//!   properties line (`review_arms2_r1_probes.rs`'s copy of `dump`
//!   omits it, so its row is blind to a volume/area move).
//!   **Adopt into `bitdump.rs` beside the other rows**; it lives here
//!   only because a review lane may not edit the suite it is
//!   differentiating.
//!
//! - `corner_arcs_are_minted_in_seeded_edge_order` — an always-on fence
//!   on what `D325` actually changed. `CornerLinks::sorted` now returns
//!   `(first, rest)`, and the corner fusion mints the SEED's arc first
//!   and the rest after; the birth record's `arcs` rows are that mint
//!   order, seen from outside the crate. The row goes red if the seed
//!   ever stops being the lowest-keyed incident link or if `rest` stops
//!   being ordered — the two properties the three call sites read and
//!   that nothing else asserts. It passes at the merge base too (the
//!   `Vec` was ascending there), so it is a fence, not a differential.

#![allow(clippy::unwrap_used, clippy::expect_used, clippy::panic)]

use std::collections::BTreeMap;
use std::fmt::Write as _;

use geom_core::Tol;
use sweep::blend::build::fillet_edges;
use sweep::test_support::{ROD_FILLET, cube, rod_creases, rod_with_flat};
use topo::{Body, EdgeKey, VertexKey, query};

fn tol() -> Tol {
    Tol::witness()
}

/// `bitdump.rs`'s dump, in its format: shortest-roundtrip `f64` on
/// every stored number, in key iteration order.
fn dump(body: &Body<f64>) -> String {
    let mut s = String::new();
    let _ = writeln!(
        s,
        "census V={} E={} F={}",
        body.vertices().count(),
        body.edges().count(),
        body.faces().count()
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
    }
    let props = topo::mass_properties(body, tol()).unwrap();
    let _ = writeln!(
        s,
        "props volume={:?} pad={:?} area={:?} apad={:?}",
        props.volume, props.volume_pad, props.surface_area, props.area_pad
    );
    s
}

/// The RULED band's dump row: the rod with a flat, both creases carved
/// in one call at `ROD_FILLET`.
#[test]
fn bitdump_ruled_band() {
    // Explicit CLEAN SKIP when unarmed, as every other dump row: never
    // a red (a panicking env read) and never a silent green.
    let Some(dir) = std::env::var("BITDUMP_DIR").ok().filter(|d| !d.is_empty()) else {
        return;
    };
    let source = rod_with_flat(tol());
    let creases = rod_creases(&source);
    assert_eq!(creases.len(), 2, "the milled rod has two creases");
    let out = fillet_edges(&source, &creases, ROD_FILLET, tol()).unwrap();
    let mut text = dump(&out.body);
    let _ = writeln!(
        text,
        "blend={:?} corner={:?} band={:?}",
        out.blend_faces, out.corner_faces, out.band_faces
    );
    std::fs::create_dir_all(&dir).unwrap();
    std::fs::write(format!("{dir}/ruled_band.txt"), text).unwrap();
}

/// **The corner fusion mints one arc per incident link, seed first, and
/// the seed is the lowest-keyed link.** Read from outside the crate
/// through the birth record: `naming.arcs` rows are pushed in mint
/// order, each naming the source link edge its arc belongs to, so per
/// corner vertex those edges must come out strictly ascending.
#[test]
fn corner_arcs_are_minted_in_seeded_edge_order() {
    let body = cube(1.0, tol());
    let out = fillet_edges(&body, &query::all_edges(&body), 0.15, tol()).unwrap();
    let rec = out.naming.as_ref().expect("the surgery records its births");
    let mut per_corner: BTreeMap<VertexKey, Vec<EdgeKey>> = BTreeMap::new();
    for (_, vertex, source_edge) in &rec.arcs {
        per_corner.entry(*vertex).or_default().push(*source_edge);
    }
    assert_eq!(
        per_corner.len(),
        8,
        "the die's eight corners each fuse from their own incidence list"
    );
    for (vertex, edges) in &per_corner {
        assert_eq!(
            edges.len(),
            3,
            "a trivalent corner mints one arc per incident link at {vertex:?}"
        );
        let mut ascending = edges.clone();
        ascending.sort_unstable();
        assert_eq!(
            edges, &ascending,
            "the arcs at {vertex:?} are minted in ascending source-edge order — the seed \
             (lowest key) first, then the rest"
        );
        assert_eq!(
            edges
                .iter()
                .collect::<std::collections::BTreeSet<_>>()
                .len(),
            edges.len(),
            "no incident link is walked twice at {vertex:?}"
        );
    }
}
