//! R2 review probes for MESH-6 (PR 1545). ADDITIVE, review-only.
//!
//! Attacks the deliverable-2 refutation at `curved::pole_columns`,
//! which says the full-2π seam "actually lives" on `grid_counts`'
//! cylinder arm, where `nv == 1` empties the interior grid. The torus
//! arm also carries full-2π seams (the PR's own face census records
//! the donut's torus patches as having BOTH meridians `Seam`), and
//! there the interior grid is large. This probe separates the two by
//! counting interior grid vertices per patch through the public API.
//!
//! Id minting order is public contract (`types::Mesh::positions`):
//! topology vertices, then per-edge chord points, then per-face
//! interior grid points. So the first id above every boundary
//! polyline's ids is the same mark `tessellate`'s census calls
//! `shared_below`, and anything at or above it is a patch's private
//! interior grid point.
#![allow(
    clippy::unwrap_used,
    clippy::expect_used,
    clippy::panic,
    clippy::print_stdout
)]

mod common;
use common::*;
use geom_core::Tol;
use topo::Body;

/// Interior-grid vertex count per patch, and the shared mark.
fn interior_census(m: &mesh::Mesh) -> (u32, Vec<(usize, usize, usize)>) {
    let mut mark = 0u32;
    for pl in &m.boundaries {
        for i in &pl.points {
            mark = mark.max(*i + 1);
        }
    }
    let per: Vec<(usize, usize, usize)> = m
        .patches
        .iter()
        .enumerate()
        .map(|(k, p)| {
            let mut ids: std::collections::HashSet<u32> = std::collections::HashSet::new();
            for t in &p.triangles {
                for i in t {
                    if *i >= mark {
                        ids.insert(*i);
                    }
                }
            }
            (k, p.triangles.len(), ids.len())
        })
        .collect();
    (mark, per)
}

#[test]
fn r2_interior_grid_census_over_the_corpus() {
    let bodies: Vec<(&str, Body<f64>)> = vec![
        ("ball", ball()),
        ("cone", cone()),
        ("l_prism", l_prism()),
        ("washer", washer()),
        ("donut", donut()),
        ("sphere_wedge", sphere_wedge(2.0)),
        ("wedge", wedge()),
    ];
    for (name, b) in &bodies {
        for d in [0.1f64, 0.004] {
            let m = mesh::tessellate(b, d, Tol::witness()).unwrap();
            let (mark, per) = interior_census(&m);
            let tot: usize = per.iter().map(|(_, _, n)| n).sum();
            let maxp = per.iter().map(|(_, _, n)| *n).max().unwrap_or(0);
            println!(
                "R2-GRID {name} d={d} positions={} mark={mark} interior_total={tot} \
                 max_per_patch={maxp} patches={}",
                m.positions.len(),
                m.patches.len()
            );
            for (k, nt, ni) in &per {
                println!("R2-GRID   patch {k}: tris={nt} interior_grid_ids={ni}");
            }
        }
    }
}

/// Per-chord-segment use counts, split by how many DISTINCT patches
/// supply the uses. A segment with two uses from ONE patch is a face
/// that traversed its own seam both ways — the full-2π seam class, in
/// the emission, observable from outside the crate. This is also an
/// independent re-derivation of `tessellate::unpaired_chord_segment`'s
/// verdict over the whole corpus.
#[test]
fn r2_seam_and_pairing_census_over_the_corpus() {
    let bodies: Vec<(&str, Body<f64>)> = vec![
        ("ball", ball()),
        ("cone", cone()),
        ("l_prism", l_prism()),
        ("washer", washer()),
        ("donut", donut()),
        ("sphere_wedge", sphere_wedge(2.0)),
        ("wedge", wedge()),
        ("cone_wedge", cone_wedge(0.05, 0.5)),
    ];
    for (name, b) in &bodies {
        for d in [0.1f64, 0.004] {
            let m = mesh::tessellate(b, d, Tol::witness()).unwrap();
            let mut mark = 0u32;
            for pl in &m.boundaries {
                for i in &pl.points {
                    mark = mark.max(*i + 1);
                }
            }
            // segment -> (total uses, uses from a single patch max, patches touching)
            let mut seg: std::collections::HashMap<(u32, u32), Vec<usize>> =
                std::collections::HashMap::new();
            for pl in &m.boundaries {
                for w in pl.points.windows(2) {
                    seg.entry((w[0].min(w[1]), w[0].max(w[1]))).or_default();
                }
            }
            for (pk, p) in m.patches.iter().enumerate() {
                for t in &p.triangles {
                    for k in 0..3 {
                        let (a, bb) = (t[k], t[(k + 1) % 3]);
                        if a < mark && bb < mark {
                            if let Some(v) = seg.get_mut(&(a.min(bb), a.max(bb))) {
                                v.push(pk);
                            }
                        }
                    }
                }
            }
            let mut hist: std::collections::BTreeMap<usize, usize> =
                std::collections::BTreeMap::new();
            let mut seam_segs = 0usize;
            let mut one_patch_pairs = 0usize;
            for uses in seg.values() {
                *hist.entry(uses.len()).or_insert(0) += 1;
                let distinct: std::collections::HashSet<usize> = uses.iter().copied().collect();
                if uses.len() == 2 && distinct.len() == 1 {
                    seam_segs += 1;
                    one_patch_pairs += 1;
                }
            }
            println!(
                "R2-SEAM {name} d={d} segments={} use_histogram={hist:?} \
                 seam_segments_paired_within_one_patch={seam_segs} (={one_patch_pairs})",
                seg.len()
            );
        }
    }
}

/// Falsification attempt for `unpaired_chord_segment`: a body that is
/// tier-1 Euler-valid but NOT a closed solid (topo's own documented
/// scaffolding strut). `tessellate`'s docs say it triangulates "every
/// face of a closed body", but nothing in `tessellate` enforces that —
/// it never calls `validate_closed`. A strut edge carries a chord
/// polyline that no face triangle can use twice, so the new census's
/// `n != 2` should fire on caller data rather than on a kernel bug.
#[test]
fn r2_scaffold_strut_body_through_tessellate() {
    use geom_core::Point3;
    use topo::{Body, MefSite, MevSite};
    let tol = Tol::witness();
    let pt = Point3::new;
    let mut body = Body::<f64>::new();
    let seed = body.mvfs(pt(0.0, 0.0, 0.0)).unwrap();
    let e_ab = body
        .mev_line(
            MevSite::Lone {
                r#loop: seed.r#loop,
            },
            pt(1.0, 0.0, 0.0),
            tol,
        )
        .unwrap();
    let strut = |he| MevSite::Fan { he1: he, he2: he };
    let e_bc = body
        .mev_line(strut(e_ab.he_minus), pt(1.0, 1.0, 0.0), tol)
        .unwrap();
    let e_cd = body
        .mev_line(strut(e_bc.he_minus), pt(0.0, 1.0, 0.0), tol)
        .unwrap();
    let he_dc = body
        .find_half_edge(seed.face, e_cd.vertex, e_bc.vertex)
        .unwrap();
    let f_bot = body
        .mef_chord(
            MefSite::Chords {
                he1: he_dc,
                he2: e_ab.he_plus,
            },
            tol,
        )
        .unwrap();
    let e_aa = body
        .mev_line(strut(e_ab.he_plus), pt(0.0, 0.0, 1.0), tol)
        .unwrap();
    let e_bb = body
        .mev_line(strut(e_bc.he_plus), pt(1.0, 0.0, 1.0), tol)
        .unwrap();
    let e_cc = body
        .mev_line(strut(e_cd.he_plus), pt(1.0, 1.0, 1.0), tol)
        .unwrap();
    let e_dd = body
        .mev_line(strut(f_bot.he_plus), pt(0.0, 1.0, 1.0), tol)
        .unwrap();
    let chord = |he1, he2| MefSite::Chords { he1, he2 };
    let f_front = body
        .mef_chord(chord(e_aa.he_minus, e_bb.he_minus), tol)
        .unwrap();
    body.mef_chord(chord(e_bb.he_minus, e_cc.he_minus), tol)
        .unwrap();
    body.mef_chord(chord(e_cc.he_minus, e_dd.he_minus), tol)
        .unwrap();
    body.mef_chord(chord(e_dd.he_minus, f_front.he_plus), tol)
        .unwrap();
    println!(
        "R2-SCAF closed cube: validate_closed = {:?}",
        topo::validate_closed(&body).is_ok()
    );
    println!(
        "R2-SCAF closed cube tessellate = {:?}",
        mesh::tessellate(&body, 0.1, tol).map(|m| m.patches.len())
    );

    // Now the scaffolding strut: tier-1 legal, tier-2 invalid.
    let scaffold = body
        .mev_line(strut(e_ab.he_plus), pt(2.0, 0.0, 0.0), tol)
        .unwrap();
    let _ = scaffold;
    println!(
        "R2-SCAF with strut: validate      = {:?}",
        topo::validate(&body).is_ok()
    );
    println!(
        "R2-SCAF with strut: validate_closed = {:?}",
        topo::validate_closed(&body).is_err()
    );
    let r = std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| {
        mesh::tessellate(&body, 0.1, tol).map(|m| m.patches.len())
    }));
    match r {
        Ok(v) => println!("R2-SCAF with strut: tessellate = {v:?}  (NO PANIC)"),
        Err(_) => println!("R2-SCAF with strut: tessellate PANICKED (the new census fired)"),
    }
}
