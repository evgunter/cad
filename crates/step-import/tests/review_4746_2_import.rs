//! Second-review probes for PR 4746 on the side-cut equator-split ball.
#![allow(clippy::unwrap_used, clippy::expect_used, clippy::panic)]

use geom::Surface;
use geom_core::Tol;
use step_import::{ImportOptions, StepImport, import_step};

fn body(tol: Tol) -> topo::Body<f64> {
    let path = format!(
        "{}/tests/fixtures/seam-region/side_cut_equator_split.step",
        env!("CARGO_MANIFEST_DIR")
    );
    let text = std::fs::read_to_string(&path).unwrap();
    match import_step(&text, &ImportOptions::default(), tol) {
        Ok(StepImport::Solid { body, .. }) => body,
        other => panic!("not a solid: {other:?}"),
    }
}

/// Every flip of the side-cut ball's two sphere faces, at three ε.
#[test]
fn review_4746_2_side_cut_flips() {
    {
        let tol = Tol::witness();
        let b = body(tol);
        let truth = topo::mass_properties(&b, tol).unwrap().volume;
        let sph: Vec<_> = b
            .faces()
            .filter(|(_, f)| matches!(b.get_surface(f.surface), Some(Surface::Sphere { .. })))
            .map(|(k, _)| k)
            .collect();
        eprintln!("ε={tol:?} sphere faces {} truth {truth}", sph.len());
        for set in [vec![sph[0]], vec![sph[1]], sph.clone()] {
            let fb = set
                .iter()
                .fold(b.clone(), |b, &k| b.flipped_face_sense_for_tests(k).unwrap());
            eprintln!(
                "  flip {set:?}: tier3 {:?} volume {:?}",
                topo::validate_geometric(&fb, tol).map_err(|e| e.into_iter().map(|e| format!("{e:?}").chars().take(60).collect::<String>()).collect::<Vec<_>>()),
                topo::mass_properties(&fb, tol).map(|m| m.volume)
            );
        }
    }
}

fn load(name: &str, tol: Tol) -> topo::Body<f64> {
    let path = format!("{}/tests/fixtures/review-4746-2/{name}", env!("CARGO_MANIFEST_DIR"));
    let text = std::fs::read_to_string(&path).unwrap();
    match import_step(&text, &ImportOptions::default(), tol) {
        Ok(StepImport::Solid { body, .. }) => body,
        other => panic!("{name}: not a solid: {other:?}"),
    }
}

fn flips(label: &str, b: &topo::Body<f64>, tol: Tol) -> usize {
    let truth = topo::mass_properties(b, tol).map(|m| m.volume);
    let sph: Vec<_> = b
        .faces()
        .filter(|(_, f)| matches!(b.get_surface(f.surface), Some(Surface::Sphere { .. })))
        .map(|(k, _)| k)
        .collect();
    let n = sph.len();
    let mut pass = 0;
    for mask in 1..(1u32 << n) {
        let set: Vec<_> = (0..n).filter(|i| mask & (1 << i) != 0).map(|i| sph[i]).collect();
        let fb = set.iter().fold(b.clone(), |b, &k| b.flipped_face_sense_for_tests(k).unwrap());
        if topo::validate_geometric(&fb, tol).is_ok() {
            pass += 1;
            eprintln!("  {label}: flip {set:?} PASSES, volume {:?} vs {truth:?}", topo::mass_properties(&fb, tol).map(|m| m.volume));
        }
    }
    eprintln!("{label}: {} flips, {pass} pass", (1u32 << n) - 1);
    pass
}

/// A whole sphere of six faces: quarter-lunes Q0, Q2 whole, Q1 and Q3
/// split at the equator. In the two-entity file the north pieces Q1n,
/// Q3n lie on a second, coincident SPHERICAL_SURFACE, so Q0's seam
/// region {Q0, Q1s, Q2, Q3s} touches itself at the north pole (a
/// bow-tie) and holds the south pole on interior edges.
#[test]
fn review_4746_2_bowtie() {
    let tol = Tol::witness();
    let exact = 4.0 / 3.0 * core::f64::consts::PI * 1e-6;
    for name in ["bowtie_one_entity.step", "bowtie_two_entities.step"] {
        let b = load(name, tol);
        let surfaces: std::collections::BTreeSet<_> = b.faces().map(|(_, f)| f.surface).collect();
        let r = topo::validate_geometric(&b, tol);
        let vol = topo::mass_properties(&b, tol).map(|m| m.volume);
        eprintln!("{name}: {} faces on {} surfaces; tier3 {r:?}; volume {vol:?} exact {exact}", b.faces().count(), surfaces.len());
        assert!(r.is_ok(), "{name}: the valid body is refused: {r:?}");
        assert!((vol.unwrap() - exact).abs() <= 1e-9 * exact);
        flips(name, &b, tol);
    }
}
