//! **R2 review probes for MESH-11 (issue 1571): the branch door at the
//! band, and the shared pole-membership helper on saturated spans.**
//! Reviewer's rows; every offset is derived from the run's own `Band`
//! so the file means the same on every eps row and on the interval
//! lane. Printed outcomes are the evidence; the assertions pin what
//! the unit's own rows leave between them.
#![allow(
    clippy::unwrap_used,
    clippy::expect_used,
    clippy::panic,
    clippy::print_stdout
)]

use crate::shared::point::{p3, v3};
use crate::shared::surf;
use crate::shared::tol::band;
use crate::shared::topo;
use crate::shared::topo::edge;
use geom::Curve3;
use geom::Surface;
use geom_brep::props::{
    LoopEdge, PropsError, curved_face, require_iso_rectangle, require_one_chart_branch,
};
use geom_core::Tol;

const PI: f64 = core::f64::consts::PI;
const RS: f64 = 0.010;

fn sphere() -> Surface<f64> {
    surf::sphere(RS)
}
fn rim(v: f64, u0: f64, u1: f64, a: u32, b: u32) -> LoopEdge<f64> {
    topo::sphere_rim(RS, v, u0, u1, a, b)
}
/// Same meridian helper as `cert1_sphere_polar.rs` / `mesh11_arc_branch.rs`:
/// `t = π/2` is the north pole on the `u` side.
fn great(u: f64, t0: f64, t1: f64, a: u32, b: u32) -> LoopEdge<f64> {
    topo::sphere_great(RS, u, t0, t1, a, b)
}

fn split_half_cap(b: f64, split: f64) -> Vec<LoopEdge<f64>> {
    vec![
        rim(b, 0.0, PI, 0, 1),
        great(0.0, PI - b, split, 1, 2),
        great(0.0, split, b, 2, 0),
    ]
}

fn verdict(r: &Result<(), PropsError>) -> String {
    match r {
        Ok(()) => "ADMIT".into(),
        Err(PropsError::NotOneChartBranch { edge, .. }) => format!("REFUSE(edge {edge})"),
        Err(e) => format!("Err({e:?})"),
    }
}

/// **Claim 3 / claim 8: the door's floor, pinned from both sides
/// against the run's band.** The pole sits `d` inside edge 1's span
/// end (`split = π/2 − d`), with `d·R` a stated fraction of the
/// run's `zero` and `escalate` thresholds. The unit's own rows bracket
/// the threshold at `[0.5·zero, 10·escalate]`; this row closes the
/// bracket to `[0.99·escalate, 1.01·escalate]` and shows the fold is
/// exact on every row (one margin, one sign apart).
#[test]
fn r2_the_floor_is_the_escalation_threshold_from_both_sides() {
    let bd = band();
    let b: f64 = 0.5;
    let exact = RS * RS * PI * (1.0 - b.sin());
    let rows: [(&str, f64); 9] = [
        ("0.25·zero", 0.25 * bd.zero()),
        ("1·zero", bd.zero()),
        ("4·zero", 4.0 * bd.zero()),
        ("0.5·escalate", 0.5 * bd.escalate()),
        ("0.99·escalate", 0.99 * bd.escalate()),
        ("1.01·escalate", 1.01 * bd.escalate()),
        ("4·escalate", 4.0 * bd.escalate()),
        ("10·escalate", 10.0 * bd.escalate()),
        ("100·escalate", 100.0 * bd.escalate()),
    ];
    for (name, metres) in rows {
        let d = metres / RS;
        let face = split_half_cap(b, PI / 2.0 - d);
        let door = require_one_chart_branch(&sphere(), &face, bd);
        let shape = require_iso_rectangle(&sphere(), &face, bd);
        let fc = curved_face(&sphere(), &face, 1.0, bd);
        let area_rel = fc
            .as_ref()
            .map(|f| (f.area - exact).abs() / exact)
            .map_err(|e| format!("{e:?}"));
        println!(
            "R2-FLOOR eps={:e} zero={:e} escalate={:e} pole {name} inside ({metres:e} m): door={} shape={} area_rel={:?}",
            Tol::witness().eps(),
            bd.zero(),
            bd.escalate(),
            verdict(&door),
            verdict(&shape),
            area_rel
        );
        assert_eq!(shape, Ok(()), "{name}: the shape door is unmoved");
        assert!(
            matches!(area_rel, Ok(r) if r < 1e-9),
            "{name}: the fold measures the half-cap exactly on every row: {area_rel:?}"
        );
        let expect_refuse = metres > bd.escalate();
        assert_eq!(
            matches!(door, Err(PropsError::NotOneChartBranch { edge: 1, .. })),
            expect_refuse,
            "{name}: the refusal begins exactly at the escalation threshold; got {door:?}"
        );
    }
    // The other side: the pole `d` OUTSIDE edge 1's span end sits inside
    // edge 2's; the same fractions, the same verdict shape, edge 2.
    for (name, metres) in rows {
        let d = metres / RS;
        let face = split_half_cap(b, PI / 2.0 + d);
        let door = require_one_chart_branch(&sphere(), &face, bd);
        println!(
            "R2-FLOOR-MIRROR pole {name} past the split on edge 2: {}",
            verdict(&door)
        );
        assert_eq!(
            matches!(door, Err(PropsError::NotOneChartBranch { edge: 2, .. })),
            metres > bd.escalate(),
            "{name}: mirror; got {door:?}"
        );
    }
}

/// **The cone arm's floor, the same way.** The apex `d` inside a
/// generator's span end; a line's `t` is metres, so `d` is the margin.
#[test]
fn r2_the_cone_floor_is_the_escalation_threshold_from_both_sides() {
    let bd = band();
    let s = core::f64::consts::FRAC_1_SQRT_2;
    let cone = Surface::Cone {
        apex: p3(0.0, 0.0, 0.0),
        axis: v3(0.0, 0.0, 1.0),
        half_angle: core::f64::consts::FRAC_PI_4,
        u_ref: v3(1.0, 0.0, 0.0),
    };
    let cone_rim = |v: f64, u0: f64, u1: f64, a, b| {
        edge(
            Curve3::Circle {
                center: p3(0.0, 0.0, v * s),
                axis: v3(0.0, 0.0, 1.0),
                radius: (v * s).abs(),
                u_ref: v3(1.0, 0.0, 0.0),
            },
            u0,
            u1,
            a,
            b,
        )
    };
    let generator = |u: f64, v0: f64, v1: f64, a, b| {
        edge(
            Curve3::Line {
                origin: p3(0.0, 0.0, 0.0),
                dir: v3(u.cos() * s, u.sin() * s, s),
            },
            v0,
            v1,
            a,
            b,
        )
    };
    let rows: [(&str, f64); 6] = [
        ("0.25·zero", 0.25 * bd.zero()),
        ("1·zero", bd.zero()),
        ("4·zero", 4.0 * bd.zero()),
        ("0.99·escalate", 0.99 * bd.escalate()),
        ("1.01·escalate", 1.01 * bd.escalate()),
        ("10·escalate", 10.0 * bd.escalate()),
    ];
    for (name, d) in rows {
        // The cap whose seam generator overshoots the apex by `d` onto
        // the mirror nappe: rim at slant 1, generators [−d, 1].
        let face = vec![
            cone_rim(1.0, 0.0, PI, 0, 1),
            generator(PI, -d, 1.0, 1, 2),
            generator(0.0, 1.0, -d, 2, 0),
        ];
        let door = require_one_chart_branch(&cone, &face, bd);
        println!(
            "R2-CONE-FLOOR apex {name} inside ({d:e} m): {}",
            verdict(&door)
        );
        assert_eq!(
            matches!(door, Err(PropsError::NotOneChartBranch { edge: 1, .. })),
            d > bd.escalate(),
            "{name}: got {door:?}"
        );
    }
}

/// **The shared helper on a SATURATED span (`dt > 2π`), where the
/// membership test's zero set is not empty.** The helper clamps the
/// membership edge at a half-turn, so for `dt ≥ 2π` the sign is
/// `f = ⟨P, M⟩ + 1`, which is ≥ 0 and vanishes exactly when the pole
/// direction is antipodal to the span's midpoint direction — an
/// INTERIOR point of the span, `δ` past `t0` for a span `2π + 2δ`.
/// The chord to the nearer endpoint is then `≈ δ`, not ≈ 0, so the
/// sign of a rounding residual decides between a definite `Positive`
/// (door refuses, fold folds) and a definite `Negative` (door ADMITS
/// a pole-crossing arc, fold SKIPS the pole and the area is short).
/// One arc per `δ`; the rimless pair closes with the complementary
/// `2π − 2δ` arc so `curved_face` can be asked (hemisphere = 2πR²).
#[test]
fn r2_a_saturated_span_with_the_pole_antipodal_to_its_midpoint() {
    let bd = band();
    let exact = 2.0 * PI * RS * RS;
    let mut admitted = Vec::new();
    let mut short = Vec::new();
    let mut n = 0;
    for k in 1..=400 {
        let delta = 0.001 * f64::from(k) + 1e-7 * f64::from(k * k);
        if delta >= 1.0 {
            break;
        }
        n += 1;
        let t0 = PI / 2.0 - delta;
        let t1 = t0 + 2.0 * PI + 2.0 * delta;
        let pair = vec![
            great(0.0, t0, t1, 0, 1),
            great(0.0, t1, t0 + 4.0 * PI, 1, 0),
        ];
        let door = require_one_chart_branch(&sphere(), &pair, bd);
        let fc = curved_face(&sphere(), &pair, 1.0, bd);
        let area_rel = fc
            .as_ref()
            .map(|f| (f.area - exact) / exact)
            .map_err(|e| format!("{e:?}"));
        if door.is_ok() {
            admitted.push(delta);
        }
        if !matches!(area_rel, Ok(r) if r.abs() < 1e-9) {
            short.push((delta, area_rel.clone()));
        }
        if k <= 3 || door.is_ok() || !matches!(area_rel, Ok(r) if r.abs() < 1e-9) {
            println!(
                "R2-SATURATED delta={delta:.6} span=2π+{:.6}: door={} area_rel={area_rel:?}",
                2.0 * delta,
                verdict(&door)
            );
        }
    }
    println!(
        "R2-SATURATED {n} spans: door admitted {} pole-crossing arcs, fold measured short on {}",
        admitted.len(),
        short.len()
    );
    assert!(
        admitted.is_empty(),
        "the branch door admitted a saturated span whose pole is δ inside it at δ = {admitted:?}"
    );
    // The DOOR holds on every one of these spans (the assertion
    // above), which is this probe's claim about MESH-11. The FOLD does
    // not, and that is issue 1601 — pre-existing on the base, the flux
    // lane's, and outside this unit's fence. Pinned as a limitation
    // with its direction, exactly as `mesh11r2_base_probes.rs` pins it
    // on the merge-base behaviour.
    assert!(
        !short.is_empty(),
        "issue 1601 is pinned here as a limitation: if the fold now measures every \
         saturated span exactly, assert `short.is_empty()` again and delete this note"
    );
    for (d, r) in &short {
        assert!(
            matches!(r, Ok(rel) if *rel < 0.0),
            "the fold may only measure SHORT, never long and never refuse; δ {d}: {r:?}"
        );
    }
}

/// **Issue 1598's L-shaped complement at the predicate.** The same two
/// edges traversed opposite ways: the shape door admits it, the flux
/// lane's contribution is the NEGATIVE of the half-cap's (equal and
/// opposite, which is the mechanism the issue records), and the branch
/// door refuses it naming the meridian.
#[test]
fn r2_the_l_shaped_complement_at_the_predicate() {
    let bd = band();
    let b: f64 = 0.5;
    let cap = vec![rim(b, 0.0, PI, 0, 1), great(0.0, PI - b, b, 1, 0)];
    let ell = vec![rim(b, PI, 0.0, 1, 0), great(0.0, b, PI - b, 0, 1)];
    assert_eq!(require_iso_rectangle(&sphere(), &ell, bd), Ok(()));
    let fc_cap = curved_face(&sphere(), &cap, 1.0, bd).expect("cap");
    let fc_ell = curved_face(&sphere(), &ell, 1.0, bd).expect("L");
    println!(
        "R2-L cap flux={} area={} | L flux={} area={}",
        fc_cap.flux, fc_cap.area, fc_ell.flux, fc_ell.area
    );
    assert_eq!(
        fc_cap.area, fc_ell.area,
        "one parse, the same levels for both faces"
    );
    assert_eq!(fc_cap.flux, -fc_ell.flux, "equal and opposite (issue 1598)");
    assert!(matches!(
        require_one_chart_branch(&sphere(), &ell, bd),
        Err(PropsError::NotOneChartBranch { edge: 1, .. })
    ));
}

/// **A full 2π meridian circle is refused; the same circle as a rim is
/// admitted.** The sphere arm's rim/meridian split decides which
/// arithmetic runs; both circles have span 2π.
#[test]
fn r2_a_full_meridian_circle_is_refused_and_a_full_rim_is_not() {
    let bd = band();
    let full_meridian = vec![great(0.0, 0.0, 2.0 * PI, 0, 0)];
    let full_rim = vec![rim(0.3, 0.0, 2.0 * PI, 0, 0)];
    let m = require_one_chart_branch(&sphere(), &full_meridian, bd);
    let r = require_one_chart_branch(&sphere(), &full_rim, bd);
    println!("R2-FULL meridian={} rim={}", verdict(&m), verdict(&r));
    assert!(matches!(
        m,
        Err(PropsError::NotOneChartBranch { edge: 0, .. })
    ));
    assert_eq!(r, Ok(()));
}

/// **The mechanism behind the short spans above, replicated in f64.**
/// The same direction arithmetic as `sphere_meridian_pole_margins`,
/// on the north pole of the `2π + 2δ` arc: `f = ⟨P, M⟩ − c_edge` with
/// `c_edge = cos(min(dt/2, π)) = −1`, so `f = ⟨P, M⟩ + 1` with
/// `P = −M` mathematically. Whether the fold keeps the pole is the sign
/// of a one-ulp residual, and the chord it is copied onto is `≈ δ`,
/// far outside any band.
#[test]
fn r2_the_saturated_span_sign_is_a_rounding_residual() {
    let bd = band();
    let exact = 2.0 * PI * RS * RS;
    let axis = v3(0.0, 0.0, 1.0);
    let mut mismatched = 0;
    let mut neg = 0;
    for k in 1..=400 {
        let delta = 0.001 * f64::from(k) + 1e-7 * f64::from(k * k);
        if delta >= 1.0 {
            break;
        }
        let t0 = PI / 2.0 - delta;
        let t1 = t0 + 2.0 * PI + 2.0 * delta;
        let e = great(0.0, t0, t1, 0, 1);
        let Curve3::Circle { axis: n_c, .. } = e.carrier else {
            unreachable!()
        };
        let w0 = e.carrier.eval(e.t0) - p3(0.0, 0.0, 0.0);
        let sa = w0.dot(axis) / RS;
        let ca = n_c.cross(w0).dot(axis) / RS;
        let dt = e.t1 - e.t0;
        let (sd2, cd2) = (dt * 0.5).sin_cos();
        let (_, c_edge) = (dt * 0.5).min(PI).sin_cos();
        let f = sa * cd2 + ca * sd2 - c_edge;
        let pair = vec![e, great(0.0, t1, t0 + 4.0 * PI, 1, 0)];
        let area = curved_face(&sphere(), &pair, 1.0, bd).map(|f| f.area);
        let short = !matches!(area, Ok(a) if ((a - exact) / exact).abs() < 1e-9);
        if f < 0.0 {
            neg += 1;
            println!(
                "R2-RESIDUAL delta={delta:.6} f={f:e} (chord≈{:.4}, {:e} m at R) short={short}",
                2.0 * (delta / 2.0).sin(),
                2.0 * (delta / 2.0).sin() * RS
            );
        }
        if (f < 0.0) != short {
            mismatched += 1;
        }
    }
    println!("R2-RESIDUAL f<0 on {neg} spans; fold-short disagreed with the sign on {mismatched}");
    assert_eq!(mismatched, 0, "the short spans are exactly the f<0 spans");
}
