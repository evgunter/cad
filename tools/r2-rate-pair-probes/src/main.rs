//! R2 e2e probe for RATE-PAIR (PR 2657, frozen head 711236057): a swept
//! elbow and a non-uniform prism loft (NURBS walls), through
//! validate_geometric, validate_pcurves (loop continuity on a spline
//! chart), shell, and the offset meters (PatchRegularity,
//! offset_normal_floor, fit_offset) — every number printed as bits so
//! base and head can be diffed textually. Build with `--features head`
//! against the unit branch, without it against the merge base
//! (6dd5983ba); the two 44-line outputs were byte-identical, verdict
//! counts 1054 / 1059 included.
//!
//! Mutation record on `crates/geom-core/tests/rate_pair_doors.rs`
//! (default features, `cargo nextest run -p geom-core rate_pair_doors`):
//! - `SupSpeed::to_meters` re-associated as `(span + span) * s * 0.5`:
//!   reds `sup_to_meters_is_the_bare_product` (the `MAX` sample overflows).
//! - `SupSpeed::to_meters` commuted to `s * span`: NO row reds — IEEE
//!   multiplication is commutative on the whole sample, so the rows
//!   cannot see operand order (which is fine: the bits are the same).
//! - `SupSpeed::to_param` sanitized as `m / s.max(MIN_POSITIVE)`: reds
//!   `a_poisoned_or_collapsed_rate_passes_straight_through` (rs:145)
//!   and `sup_to_param_is_the_bare_quotient` (rs:105).
#![allow(clippy::unwrap_used, clippy::expect_used)]

use geom::Surface;
use geom_brep::offset_meters::{OFFSET_METER_LADDER, offset_normal_floor, patch_regularity};
use geom_brep::patch_bound::patch_cells_refined;
use geom_core::k_stats::Bracket;
use geom_core::{Affine3, Band, Tol, Vec3};
use profile::RawLoop;
use sweep::test_support::{elbow_section, swept_elbow_lofted};
use sweep::{ProfileLoop, loft_body};
use topo::Body;

#[cfg(feature = "head")]
fn speeds(reg: &geom_brep::offset_meters::PatchRegularity) -> (f64, f64, f64) {
    (reg.speed_u.get(), reg.speed_v.get(), reg.speed_lever().get())
}
#[cfg(not(feature = "head"))]
fn speeds(reg: &geom_brep::offset_meters::PatchRegularity) -> (f64, f64, f64) {
    (reg.speed_u, reg.speed_v, reg.speed_lever())
}

#[cfg(feature = "head")]
fn stretch(s: &Surface<f64>) -> (f64, f64) {
    let (a, b) = geom_brep::chart_stretch_sup(s);
    (a.get(), b.get())
}
#[cfg(not(feature = "head"))]
fn stretch(s: &Surface<f64>) -> (f64, f64) {
    geom_brep::chart_stretch_sup(s)
}

fn bits(x: f64) -> String {
    format!("{x:e}[{:016x}]", x.to_bits())
}

fn quad(pts: [(f64, f64); 4]) -> sweep::Section {
    vec![ProfileLoop::new(
        pts.iter()
            .map(|&(x, y)| sweep::ProfileVertex::new(geom_core::Point2::new(x, y), 0.0))
            .collect(),
    )]
}

fn exercise(name: &str, body: &Body<f64>, tol: Tol) {
    let band = Band::linear(tol).unwrap();
    let bracket = Bracket::open();
    println!("== {name}: faces={} surfaces={}", body.faces().count(), body.surfaces().count());
    println!("validate_geometric: {:?}", topo::validate_geometric(body, tol));
    println!("validate_pcurves: {:?}", topo::pcurves::validate_pcurves(body, band));
    match topo::mass_properties(body, tol) {
        Ok(p) => println!("volume: {}", bits(p.volume)),
        Err(e) => println!("mass_properties refused: {e:?}"),
    }
    match topo::shell(body, 0.02, tol) {
        Ok(s) => println!(
            "shell: ok faces={} validate={:?}",
            s.body.faces().count(),
            topo::validate_geometric(&s.body, tol)
        ),
        Err(e) => println!("shell refused: {e}"),
    }
    let mut keys: Vec<_> = body.surfaces().collect();
    keys.sort_by_key(|(k, _)| format!("{k:?}"));
    for (k, s) in keys {
        let Surface::Nurbs(n) = s else { continue };
        if n.is_placeholder() {
            continue;
        }
        let (su, sv) = stretch(s);
        println!("{k:?} chart_stretch_sup: u={} v={}", bits(su), bits(sv));
        for splits in OFFSET_METER_LADDER {
            let cells = patch_cells_refined(n, splits).expect("cells");
            let reg = patch_regularity(&cells);
            let (u, v, lever) = speeds(&reg);
            println!(
                "{k:?} splits={splits} cells={} floor={} sup={} speed_u={} speed_v={} lever={} thinness={} sine_floor={} normal_floor={:?}",
                reg.cells,
                bits(reg.floor),
                bits(reg.sup),
                bits(u),
                bits(v),
                bits(lever),
                bits(reg.thinness()),
                bits(reg.sine_floor),
                offset_normal_floor(&reg, band)
            );
        }
        match geom_brep::fit_offset(n, 0.02, tol, band) {
            Ok((fit, cert)) => println!(
                "{k:?} fit_offset: ok control={} cells={} rounds={} hull_sup={} normal_floor={} reach={}",
                fit.control().len(),
                cert.cells,
                cert.rounds,
                bits(cert.hull_sup),
                bits(cert.normal_floor),
                bits(cert.curvature_reach)
            ),
            Err(e) => println!("{k:?} fit_offset refused: {e}"),
        }
    }
    let rec = bracket.finish();
    println!("{name}: verdicts={}", rec.verdicts.len());
}

fn main() {
    let tol = Tol::witness();
    let elbow = swept_elbow_lofted(tol);
    exercise("swept elbow", &elbow.body, tol);
    let _ = elbow_section();

    let sections = vec![
        quad([(-1.0, -1.0), (1.0, -1.0), (1.0, 1.0), (-1.0, 1.0)]),
        quad([(-1.375, -1.0), (1.375, -1.0), (1.0, 1.0), (-1.0, 1.0)]),
        quad([(-1.0, -1.0), (1.0, -1.0), (1.0, 1.0), (-1.0, 1.0)]),
    ];
    let places: Vec<Affine3<f64>> = [0.0, 0.7, 2.0]
        .iter()
        .map(|z| Affine3::translation(Vec3::new(0.0, 0.0, *z)))
        .collect();
    let lofted = loft_body::<f64>(&sections, &places, 2, tol).expect("loft builds");
    exercise("nonuniform prism loft", &lofted.body, tol);
}
