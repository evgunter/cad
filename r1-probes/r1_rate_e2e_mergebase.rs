//! R1 (scalar-rate-r1) end-to-end exercise for RATE-PAIR.
//!
//! A user-shaped program: build a lofted body with NURBS chart walls,
//! validate it geometrically (the pcurve certify path reaches
//! `param_rate`, `trim_containment` and the iso lane), run the
//! loop-continuity validator (`v_meter` through `metered_sup`), and
//! read `PatchRegularity` off every NURBS wall (the `SupSpeed` fields,
//! `speed_lever`, `thinness`, `offset_normal_floor`).
//!
//! Everything numeric is printed as BITS so the merge-base run and the
//! head run can be compared byte for byte.

use geom_core::{Affine3, Point2, Tol, Vec3};
use profile::{ProfileLoop, RawLoop};
use sweep::{Section, loft_body};

fn quad(pts: [(f64, f64); 4]) -> Section {
    vec![ProfileLoop::polygon(
        pts.iter().map(|&(x, y)| Point2::new(x, y)),
    )]
}

/// Five non-affine sections with a scale ramp and off-axis stations —
/// the walls are genuine NURBS charts, not planes.
fn sections() -> (Vec<Section>, Vec<Affine3<f64>>) {
    let s = |k: f64| {
        quad([
            (-k, -k),
            (1.0 * k, -0.6 * k),
            (0.8 * k, 1.2 * k),
            (-1.3 * k, 0.9 * k),
        ])
    };
    (
        vec![s(1.0), s(2.0), s(0.5), s(4.0), s(1.0)],
        vec![
            Affine3::identity(),
            Affine3::translation(Vec3::new(0.3, -0.2, 1.0)),
            Affine3::translation(Vec3::new(-0.4, 0.5, 2.0)),
            Affine3::translation(Vec3::new(0.1, 0.1, 3.0)),
            Affine3::translation(Vec3::new(0.0, -0.3, 4.0)),
        ],
    )
}

fn b(x: f64) -> String {
    format!("{:016x}", x.to_bits())
}

fn main() {
    let tol = Tol::witness();
    let (sections, places) = sections();

    // ---- 1. Build, through the public door. ----
    let lofted = loft_body::<f64>(&sections, &places, 2, tol).expect("the loft builds");
    let body = &lofted.body;
    println!(
        "E2E build: faces={} edges={} surfaces={}",
        body.faces().count(),
        body.edges().count(),
        body.surfaces().count()
    );

    // ---- 2. validate_geometric: the pcurve certify path. ----
    match topo::validate_geometric(body, tol) {
        Ok(()) => println!("E2E validate_geometric: OK"),
        Err(errs) => {
            println!("E2E validate_geometric: {} errors", errs.len());
            for e in errs.iter().take(6) {
                println!("  {e:?}");
            }
        }
    }

    // ---- 3. Loop continuity (v_meter -> metered_sup). ----
    let band = geom_core::Band::linear(tol).expect("band");
    let pc = topo::pcurves::validate_pcurves(body, band);
    println!("E2E validate_pcurves: {} findings", pc.len());
    for e in pc.iter().take(6) {
        println!("  {e:?}");
    }

    // ---- 4. chart_boundary on every face: the closure-height door. ----
    let mut boundaries = 0usize;
    let mut refusals = 0usize;
    let keys: Vec<_> = body.faces().map(|(k, f)| (k, f.surface)).collect();
    for (fk, sk) in keys {
        let chart = body.get_surface(sk).expect("a face has a surface");
        match topo::chart_boundary(body, fk, chart, band) {
            Ok(_) => boundaries += 1,
            Err(_) => refusals += 1,
        }
    }
    println!("E2E chart_boundary: {boundaries} ok, {refusals} refused");

    // ---- 5. PatchRegularity on every NURBS wall. ----
    let mut rows: Vec<String> = Vec::new();
    for (sk, surface) in body.surfaces() {
        let geom::Surface::Nurbs(ref payload) = *surface else {
            continue;
        };
        if payload.is_placeholder() {
            continue;
        }
        let Ok(cells) =
            geom_brep::patch_bound::patch_cells_refined(payload, geom_brep::offset_meters::OFFSET_METER_LADDER[0])
        else {
            continue;
        };
        let reg = geom_brep::offset_meters::patch_regularity(&cells);
        let floor = geom_brep::offset_meters::offset_normal_floor(&reg, band);
        rows.push(format!(
            "  surface {sk:?} floor={} sup={} su={} sv={} lever={} thin={} sine={} cells={} normal_floor={}",
            b(reg.floor),
            b(reg.sup),
            b(reg.speed_u),
            b(reg.speed_v),
            b(reg.speed_lever()),
            b(reg.thinness()),
            b(reg.sine_floor),
            reg.cells,
            match floor {
                Ok(()) => "ok".to_string(),
                Err(e) => format!("{e:?}"),
            }
        ));
    }
    rows.sort();
    println!("E2E PatchRegularity rows: {}", rows.len());
    for r in &rows {
        println!("{r}");
    }

    // ---- 6. chart_stretch_sup / chart_stretch_inf on every chart. ----
    let mut sup_rows: Vec<String> = Vec::new();
    for (sk, surface) in body.surfaces() {
        let (su, sv) = geom_brep::chart_stretch_sup(surface);
        sup_rows.push(format!(
            "  chart {sk:?} sup=({},{})",
            b(su),
            b(sv)
        ));
    }
    sup_rows.sort();
    for r in &sup_rows {
        println!("{r}");
    }

    // ---- 7. Mass properties: the whole-body number. ----
    match topo::props::mass_properties(body, tol) {
        Ok(m) => println!(
            "E2E mass: vol={} volpad={} area={} areapad={}",
            b(m.volume),
            b(m.volume_pad),
            b(m.surface_area),
            b(m.area_pad)
        ),
        Err(e) => println!("E2E mass: {e:?}"),
    }
}
