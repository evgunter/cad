//! Pre-GUI visual demo tour: builds a handful of highlight bodies
//! through the kernel's public profile/sweep APIs, narrates each
//! (operations, topology census, genus, validation tiers, exact vs
//! meshed mass properties), and exports a binary STL per body for the
//! render step (`demos/render.sh`).
//!
//! Usage: `cargo run --release -- <outdir>` (from `demos/tour/`).

#![allow(clippy::unwrap_used, clippy::expect_used, clippy::panic)]

mod bodies;
mod bool_bodies;
mod booleans;

use mesh::validate::{check_mesh, signed_volume, triangle_count};
use topo::Body;

/// One tour stop: a named body, the story of how it was built, the
/// tessellation chord tolerance for its STL export, whether the body
/// came out of a boolean op (seam edges carry chord-line descriptions,
/// so tier 3 runs through the upgrade posture), and an optional
/// build-time narration note.
struct Stop {
    name: &'static str,
    story: &'static str,
    ops: &'static str,
    delta: f64,
    seamed: bool,
    note: Option<String>,
    body: Body<f64>,
}

/// Topology census + genus via the Euler–Poincaré identity
/// `v − e + f − r = 2(s − g)` (s = shells; g summed over shells —
/// the boolean leg's void box is legitimately two-shell; the identity
/// is narration, tier 2 is the checker).
fn census(body: &Body<f64>) -> (usize, usize, usize, usize, usize, i64) {
    let v = body.vertices().count();
    let e = body.edges().count();
    let f = body.faces().count();
    let r: usize = body.faces().map(|(_, face)| face.rings.len()).sum();
    let s = body.shells().count();
    let genus = s as i64 - (v as i64 - e as i64 + f as i64 - r as i64) / 2;
    (v, e, f, r, s, genus)
}

fn run_stop(stop: &Stop, outdir: &str) {
    println!("\n== {} ==", stop.name);
    println!("   {}", stop.story);
    println!("   built by: {}", stop.ops);
    if let Some(note) = &stop.note {
        println!("   note: {note}");
    }

    // The tiered validators: tier 1 (structural), tier 2 (closed-solid
    // census: mates, orientation, Euler–Poincaré), tier 3 (geometric:
    // carrier residuals, cap planarity, the +V orientation invariant).
    topo::validate(&stop.body)
        .unwrap_or_else(|e| panic!("{}: tier-1 structural validation failed: {e:?}", stop.name));
    topo::validate_closed(&stop.body).unwrap_or_else(|e| {
        panic!(
            "{}: tier-2 closed-solid validation failed: {e:?}",
            stop.name
        )
    });
    if stop.seamed {
        // Boolean outputs carry chord-line descriptions on seam edges
        // (the documented PR 3 description gap), so tier 3 runs on a
        // clone upgraded to Intersection descriptions — the honest
        // upgrade op is a PR 6 obligation (booleans module docs).
        let mut upgraded = stop.body.clone();
        booleans::upgrade_edges_to_intersections(&mut upgraded);
        topo::validate_geometric(&upgraded).unwrap_or_else(|e| {
            panic!(
                "{}: tier-3 geometric validation (upgraded clone) failed: {e:?}",
                stop.name
            )
        });
    } else {
        topo::validate_geometric(&stop.body)
            .unwrap_or_else(|e| panic!("{}: tier-3 geometric validation failed: {e:?}", stop.name));
    }

    let (v, e, f, r, s, genus) = census(&stop.body);
    println!(
        "   topology: {v} vertices, {e} edges, {f} faces, {r} rings, {s} shell(s) -> genus {genus}"
    );
    if stop.seamed {
        println!(
            "   validation: tiers 1-2 on the result; tier 3 on the Intersection-upgraded clone \
             (seam edges carry chord descriptions until PR 6's upgrade op)"
        );
    } else {
        println!("   validation: tiers 1-3 passed (structural, closed-solid census, geometric/+V)");
    }

    // Exact B-rep mass properties (divergence theorem over the exact
    // faces — not the mesh).
    let props = topo::mass_properties(&stop.body).expect("mass properties");

    // Tessellate, self-check the mesh, and compare its signed volume
    // against the exact one as an end-to-end sanity ribbon.
    let mesh = mesh::tessellate(&stop.body, stop.delta).expect("tessellate");
    check_mesh(&mesh).unwrap_or_else(|e| panic!("{}: check_mesh failed: {e:?}", stop.name));
    let v_mesh = signed_volume(&mesh);
    assert!(
        v_mesh > 0.0,
        "{}: mesh signed volume must be positive",
        stop.name
    );
    let rel = ((v_mesh - props.volume) / props.volume).abs();
    println!(
        "   mass properties (exact B-rep): V = {:.6} m^3, A = {:.6} m^2",
        props.volume, props.surface_area
    );
    println!(
        "   mesh (delta = {:.0e}): {} triangles, V_mesh = {v_mesh:.6} ({:.3}% off exact — chordal, inscribed)",
        stop.delta,
        triangle_count(&mesh),
        rel * 100.0
    );

    let path = format!("{outdir}/{}.stl", stop.name);
    let mut file = std::fs::File::create(&path).expect("create stl file");
    stl::write_binary(&mesh, &mut file).expect("write stl");
    println!("   exported {path}");
}

fn main() {
    let outdir = std::env::args().nth(1).expect("usage: demo-tour <outdir>");
    std::fs::create_dir_all(&outdir).expect("create outdir");

    println!("B-rep kernel demo tour — sweeps, then the M3 boolean ops");
    println!("=========================================================");
    for stop in bodies::stops() {
        run_stop(&stop, &outdir);
    }
    println!("\n-- the boolean leg (M3 PR 5: union / subtract, planar-only) --");
    println!(
        "   operand posture: extruded boxes get their edges re-described as chord lines\n\
         \x20  first — the ops' carve/merge stages currently dangle extrude's\n\
         \x20  Intersection edge descriptions (a typed Merge refusal; PR 5/6 feedback)"
    );
    for stop in bool_bodies::stops() {
        run_stop(&stop, &outdir);
    }
    bodies::finale_fail_loud();
    println!("\ntour complete: STLs in {outdir}/ — render with demos/render.sh");
}
