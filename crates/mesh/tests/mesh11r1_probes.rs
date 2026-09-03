//! **Reviewer probes: the two witnesses, before and after** (MESH-11
//! review). One row prints the raw outcome of each witness body at
//! every δ the unit's own suite uses, through the public `tessellate`,
//! catching panics and reading `check_mesh` on any mesh that comes
//! back. It names no post-unit type, so the SAME row compiles and runs
//! with `crates/{geom-brep,mesh}/src` checked out at the merge base —
//! which is how the "before" column is taken.
#![allow(clippy::unwrap_used, clippy::expect_used, clippy::panic)]

mod common;

use common::witness_bodies::{apex_crossing_bowtie, pole_crossing_half_cap};
use geom_core::Tol;
use topo::Body;

fn outcome(body: &Body<f64>, delta: f64) -> String {
    let tol = Tol::witness();
    let hook = std::panic::take_hook();
    std::panic::set_hook(Box::new(|_| {}));
    let out = std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| {
        mesh::tessellate(body, delta, tol)
    }));
    std::panic::set_hook(hook);
    match out {
        Ok(Ok(m)) => format!(
            "Ok({} tris, watertight={})",
            m.patches.iter().map(|p| p.triangles.len()).sum::<usize>(),
            mesh::validate::check_mesh(&m).is_ok()
        ),
        Ok(Err(e)) => format!("Err({e:?})"),
        Err(p) => {
            let msg = p
                .downcast_ref::<String>()
                .cloned()
                .or_else(|| p.downcast_ref::<&str>().map(|s| (*s).to_string()))
                .unwrap_or_default();
            format!("PANIC({})", msg.lines().next().unwrap_or(""))
        }
    }
}

/// The two π-rad witnesses at every δ, printed. Asserted only that
/// nothing PANICS and nothing returns a NON-watertight mesh — the two
/// outcomes the unit claims are no longer reachable. Under merge-base
/// sources this row goes red, which is the "before".
#[test]
fn r1_both_witnesses_at_every_delta() {
    let (sphere_body, _, _) = pole_crossing_half_cap();
    let (cone_body, _, _) = apex_crossing_bowtie();
    println!(
        "R1 debug_assertions={} eps={:e}",
        cfg!(debug_assertions),
        Tol::witness().get().eps
    );
    let mut rows = Vec::new();
    for (name, body) in [("half-cap", &sphere_body), ("bow-tie", &cone_body)] {
        for delta in [0.5, 0.3, 0.2, 0.1, 0.05, 0.02] {
            let got = outcome(body, delta);
            println!("R1 {name} delta={delta}: {got}");
            rows.push((name, delta, got));
        }
    }
    for (name, delta, got) in &rows {
        assert!(
            !got.starts_with("PANIC"),
            "{name} at delta={delta} panicked: {got}"
        );
        assert!(
            !got.contains("watertight=false"),
            "{name} at delta={delta} returned a non-watertight mesh: {got}"
        );
    }
    // The volume the closed sphere measures, printed either way (issue
    // 1598 asserts 0.0 at head; the same number is the "before").
    println!(
        "R1 half-cap mass_properties volume = {:?}",
        topo::mass_properties(&sphere_body, Tol::witness()).map(|m| m.volume)
    );
}
