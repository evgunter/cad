//! Reviewer probes (ATREST R1, PR 5746) on `gap`'s two new decisions,
//! at margins inside their bands. Each row asserts what the head DOES,
//! and its doc says why that is the finding.

#![allow(clippy::expect_used, clippy::panic)]

use super::*;

fn band() -> Band {
    Band::linear(geom_core::Tol::witness()).expect("the witness tolerance is a band")
}

fn eps() -> f64 {
    geom_core::Tol::witness().eps()
}

fn p(x: f64, y: f64, z: f64) -> Point3<f64> {
    Point3::new(x, y, z)
}

fn v(x: f64, y: f64, z: f64) -> Vec3<f64> {
    Vec3::new(x, y, z)
}

fn ridge() -> Wedge<f64> {
    Wedge {
        along: v(0.0, 2.0, 0.0),
        plus: v(0.0, 0.0, 1.0),
        minus: v(1.0, 0.0, 0.0),
    }
}

fn line(start: Point3<f64>, end: Point3<f64>, wedge: Option<Wedge<f64>>) -> Carrier<f64> {
    Carrier::Line {
        dir: (end - start) / (end - start).norm(),
        ends: [start, end],
        wedge,
    }
}

/// **`gap_wedge_crossing` escalates AT CONTACT.** An edge lying on the
/// ridge's top face (`z = 0`), parallel to the ridge and `s` in from it,
/// rests on the wedge: its gap is exactly 0 at every `s`, and the
/// parallel edge has no crossing to find. But the guard decides
/// `|D₀| + |D₁| = 2s`, so at `2s` inside the sliver the measure refuses
/// `Escalated` for a resting edge. Contradicts the PR body's "neither
/// escalates near contact". Green = the escalation is there.
#[test]
fn probe_wedge_crossing_escalates_for_a_resting_parallel_edge() {
    let outer = line(p(0.0, -1.0, 0.0), p(0.0, 1.0, 0.0), Some(ridge()));
    // 2s = 2ε: past the coincidence threshold, inside K·ε for any K > 2.
    let s = eps();
    let rest = line(p(-s, -0.5, 0.0), p(-s, 0.5, 0.0), None);
    let got = gap(&outer, &rest, band());
    eprintln!("eps {}: {got:?}", eps());
    assert!(
        matches!(
            got,
            Err(PrimitiveRefusal::Escalated {
                predicate: "gap_wedge_crossing",
                ..
            })
        ),
        "a resting edge 1ε from the ridge: {got:?}"
    );
    // Far from the sliver it reads 0, as it should.
    let far = line(p(-0.1, -0.5, 0.0), p(-0.1, 0.5, 0.0), None);
    assert_eq!(gap(&outer, &far, band()).expect("reads"), 0.0);
}

/// **A short, gently concave crease reads as flat, and its saddle
/// reads a number with the wrong sign.** A corner on a top face (`+z`)
/// and two side faces `n₂ = +x` and `n₃` (turned δ about `z`), the
/// `n₂`/`n₃` crease CONCAVE, length `L = 1e-3`, `δ = ε/(2L)`, the two
/// `z` rims convex: a saddle. `gap_edge_convexity` levers δ by the
/// EDGE's length (δ·L = ε/2: Zero, "flat"), where `geom_brep::dihedral`
/// says a wedge is levered by its faces' reach, never the edge's. The
/// fold comes out `Max`; a point inside `n₃`'s half-space by `0.005δ`
/// (2.5ε at ε = 1e-6) reads `+0.005δ` clearance. Green = it reads.
#[test]
fn probe_short_concave_crease_reads_a_saddle_as_convex() {
    let length = 1e-3;
    let delta = 0.5 * eps() / length;
    let z = v(0.0, 0.0, 1.0);
    let n2 = v(1.0, 0.0, 0.0);
    let n3 = v(delta.cos(), delta.sin(), 0.0);
    let corner = Corner {
        faces: vec![z, n2, n3],
        edges: vec![
            // Convex rims: along·(plus × minus) > 0.
            Wedge {
                along: v(0.0, 1.0, 0.0),
                plus: z,
                minus: n2,
            },
            Wedge {
                along: v(-delta.sin(), delta.cos(), 0.0) * -1.0,
                plus: n3,
                minus: z,
            },
            // The concave crease: along·(n₂ × n₃) = −sin δ.
            Wedge {
                along: v(0.0, 0.0, -length),
                plus: n2,
                minus: n3,
            },
        ],
    };
    assert_eq!(
        Fold::of(&corner.edges, band()).expect("decides"),
        Some(Fold::Max),
        "the crease reads flat, so the saddle reads convex"
    );
    let a = 0.005 * delta;
    let y = -0.01;
    let x = (-a - y * delta.sin()) / delta.cos();
    // n₂·q = x ≈ 0.005δ... solve n₃·q = −a for x at y = −0.01.
    let q = p(x, y, -1e-3);
    let off2 = n2.dot(q - p(0.0, 0.0, 0.0));
    let off3 = n3.dot(q - p(0.0, 0.0, 0.0));
    let outer = Carrier::Point {
        at: p(0.0, 0.0, 0.0),
        corner: Some(corner),
    };
    let got = gap(
        &outer,
        &Carrier::Point {
            at: q,
            corner: None,
        },
        band(),
    )
    .expect("reads");
    eprintln!("eps {}: off2 {off2}, off3 {off3}, gap {got}", eps());
    assert!(
        off3 < -eps() && got > eps(),
        "inside n3 by {off3}, read {got}"
    );
}
