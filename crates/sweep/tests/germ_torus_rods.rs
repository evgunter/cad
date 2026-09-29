//! **Random rods through the donut's tube under ∖ and ∩** — a
//! counterexample search for a wrong body. Each rod passes through a
//! point `p` drawn inside the tube, so `p` lies in both operands: a
//! body `donut ∩ rod` answers must hold `p`, and a body either
//! subtract answers must not. Any body answered must also be valid at
//! tier 3, and the volumes of `A ∖ B` and `A ∩ B` must sum to `A`'s.
//!
//! Most rods refuse (the sagitta charge, the pierce door, or an
//! escalation, depending on the pose and the band); the row prints the
//! count of each refusal, which is a report and not a gate.

#![allow(clippy::unwrap_used, clippy::expect_used, clippy::panic)]

test_utils::gated_to![
    "crates/topo/src/boolean/",
    "crates/topo/src/chart_region.rs",
    "crates/topo/src/face_normal.rs",
    "crates/geom-brep/src/implicit.rs",
    "crates/geom-brep/src/intersect.rs",
    "crates/sweep/tests/common/",
    "crates/sweep/tests/revolve_common/",
];

use std::collections::BTreeMap;

use crate::common::operands::framed_bar;
use crate::common::revert_ops::subtract_both_orders_and_intersect;
use crate::revolve_common::{self, axis_y, validated};

use geom_core::{Band, Point3, Tol, Vec3};
use sweep::{Revolution, revolve};
use test_utils::fuzz;
use topo::{Body, BooleanDeclarations, BooleanResult};

fn volume(b: &Body<f64>) -> f64 {
    topo::mass_properties(b, Tol::witness())
        .expect("the volume integrates")
        .volume
}

#[test]
fn random_rods_through_the_tube_never_answer_a_wrong_body() {
    let mut rng = fuzz::start("germ_torus_rods");
    let band = Band::linear(Tol::witness()).expect("the run's band");
    let donut = revolve(
        &validated(vec![revolve_common::donut_profile()]),
        axis_y(),
        Revolution::Full,
        Tol::witness(),
    )
    .expect("the donut revolves")
    .body;
    let vol_donut = volume(&donut);
    let mut counts: BTreeMap<String, usize> = BTreeMap::new();
    for i in 0..fuzz::scaled(60) {
        let theta = rng.range(0.0, std::f64::consts::TAU);
        let phi = rng.range(0.0, std::f64::consts::TAU);
        let s = 0.45 * rng.unit().sqrt();
        let rho = 2.0 + s * phi.cos();
        let p = Point3::new(rho * theta.cos(), s * phi.sin(), rho * theta.sin());
        let z = rng.range(-1.0, 1.0);
        let az = rng.range(0.0, std::f64::consts::TAU);
        let q = (1.0 - z * z).sqrt();
        let mut d = Vec3::new(q * az.cos(), z, q * az.sin());
        if d.cross(Vec3::new(0.0, 1.0, 0.0)).norm() < 0.05 {
            // `framed_bar` frames its square off `ŷ`.
            d = Vec3::new(1.0, 0.1, 0.0);
        }
        let w = rng.range(1e-3, 0.1);
        let rod = framed_bar(p, d, -4.0, 4.0, w);
        let what = |op: &str| {
            format!(
                "rod {i} (p {p:?}, d {d:?}, w {w}), {op}; {}",
                fuzz::replay()
            )
        };
        let mut vols = [None; 3];
        for (k, (op, r)) in
            subtract_both_orders_and_intersect(&donut, &rod, &BooleanDeclarations::none())
                .into_iter()
                .enumerate()
        {
            let want_p = op == "A ∩ B";
            match r {
                Err(e) => {
                    let name = format!("{e:?}");
                    let name = name.split([' ', '{', '(']).next().unwrap_or("").to_string();
                    *counts.entry(name).or_default() += 1;
                }
                Ok(BooleanResult::Empty) => {
                    assert!(!want_p, "{}: empty, but p is in both operands", what(op));
                    vols[k] = Some(0.0);
                    *counts.entry("Empty".into()).or_default() += 1;
                }
                Ok(BooleanResult::Body(b)) => {
                    assert_eq!(
                        topo::validate_geometric(&b.body, Tol::witness()),
                        Ok(()),
                        "{}",
                        what(op)
                    );
                    let at_p = topo::point_in_solid(&b.body, p, band, Tol::witness());
                    assert!(
                        matches!(
                            (at_p, want_p),
                            (Ok(topo::SolidContainment::In), true)
                                | (Ok(topo::SolidContainment::Out), false)
                        ),
                        "{}: p must be {} the result",
                        what(op),
                        if want_p { "in" } else { "out of" }
                    );
                    vols[k] = Some(volume(&b.body));
                    *counts.entry("Body".into()).or_default() += 1;
                }
            }
        }
        if let [Some(a_minus_b), Some(b_minus_a), Some(a_and_b)] = vols {
            let vol_rod = volume(&rod);
            assert!(
                (a_minus_b + a_and_b - vol_donut).abs() <= 1e-8 * vol_donut,
                "{}",
                what("vol(A ∖ B) + vol(A ∩ B) = vol(A)")
            );
            assert!(
                (b_minus_a + a_and_b - vol_rod).abs() <= 1e-8 * vol_donut,
                "{}",
                what("vol(B ∖ A) + vol(A ∩ B) = vol(B)")
            );
        }
    }
    println!("germ_torus_rods: outcomes over every rod and op: {counts:?}");
}
