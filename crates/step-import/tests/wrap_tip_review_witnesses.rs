//! **The PR 7774 review's witnesses** (`fixtures/wrap-tip-review/`): three
//! files the review of import's wrap tip found, each held at what it
//! should do, and pinned red at what it does today where that is short
//! of it, against the row that owns the gap.
#![allow(clippy::unwrap_used, clippy::expect_used, clippy::panic)]

use geom_core::Tol;
use step_import::{ImportOptions, StepImport, import_step};

fn fixture(name: &str) -> String {
    let p = format!(
        "{}/tests/fixtures/wrap-tip-review/{name}",
        env!("CARGO_MANIFEST_DIR")
    );
    std::fs::read_to_string(&p).unwrap_or_else(|e| panic!("reading {p}: {e}"))
}

/// The file imported as a solid that passes tier 3, and its volume.
fn imported(name: &str, tol: Tol) -> (topo::Body<f64>, f64) {
    let body = match import_step(&fixture(name), &ImportOptions::default(), tol) {
        Ok(StepImport::Solid { body, .. }) => body.into_body(),
        Ok(other) => panic!("{name}: imported as something other than a solid: {other:?}"),
        Err(e) => panic!("{name}: {}", e.render()),
    };
    assert_eq!(
        topo::validate_geometric(&body, tol),
        Ok(()),
        "{name}: tier 3"
    );
    let volume = topo::test_support::pinned_volume(
        name,
        &topo::test_support::finished(name, body.clone(), tol),
        tol,
    )
    .volume;
    (body, volume)
}

/// **A ball cut on a tilted chart imports at its closed form.** Pinned
/// red at every ε: a NaN distance reaches the `step_built_sense` band
/// read (`work/importgate/a-nan-distance-reaches-a-band-read-on-a-tilted-axis-sphere.md`).
#[test]
fn a_hole_on_a_tilted_axis_sphere_imports_at_its_closed_form() {
    let tol = Tol::witness();
    test_utils::pinned_red::row(
        tol.eps(),
        &test_utils::pinned_red::EVERY_ROW,
        "the tilted-axis sphere's cap cut",
        &["the distance NaN m (step_built_sense)"],
        || {
            let (_, volume) = imported("hole_on_a_tilted_axis_sphere.step", tol);
            let want = 4.143_631_343_316_944e-6;
            assert!(
                (volume - want).abs() <= 1e-9 * want,
                "volume {volume} vs {want}"
            );
        },
    );
}

/// **A cone whose face walks two struts to its apex refuses typed**: one
/// wrap edge reaches a singularity, and only one (DESIGN `[topology]`),
/// so a file stating two is refused naming that, never as a broken
/// invariant. Pinned red at every ε: the build refuses it `internal:`
/// (`work/importgate/a-two-strut-cone-apex-reports-an-internal-message-a-file-reaches.md`).
#[test]
fn a_cone_with_two_struts_to_its_apex_refuses_typed() {
    let tol = Tol::witness();
    test_utils::pinned_red::row(
        tol.eps(),
        &test_utils::pinned_red::EVERY_ROW,
        "the two-strut cone",
        &["internal: a built vertex with no built fan anchor"],
        || match import_step(
            &fixture("cone_with_two_struts_to_its_apex.step"),
            &ImportOptions::default(),
            tol,
        ) {
            Err(e) => {
                let text = e.render();
                assert!(!text.contains("internal:"), "a typed refusal: {text}");
            }
            Ok(_) => panic!("the two-strut cone imports"),
        },
    );
}

/// How far the hole passes from the north pole (m): the file's cap is
/// `1e-4` of angular radius short of it on a 10 mm sphere.
const GRAZE: f64 = 1e-6;

/// Whether the run's band cannot tell the graze from nothing: the wrap
/// edge to the pole is `GRAZE` long, and where that is inside the band's
/// escalation its pcurve's map residual is too.
fn graze_in_band(tol: Tol) -> bool {
    GRAZE < geom_core::Band::linear(tol).unwrap().escalate()
}

/// Where [`graze_in_band`], the import refuses typed at its at-rest
/// gate: the 1 µm wrap edge's pcurve map residual escalates in the band.
fn refuses_at_the_band(tol: Tol) {
    match import_step(
        &fixture("hole_grazing_the_pole.step"),
        &ImportOptions::default(),
        tol,
    ) {
        Err(e @ step_import::StepImportError::TierInvalid { .. }) => {
            let read = format!("{e:?}");
            assert!(
                read.contains("Escalated") && read.contains("MapResidual"),
                "the wrap edge's map residual escalates at ε = {:e}: {read}",
                tol.eps()
            );
        }
        Err(e) => panic!("a different refusal at ε = {:e}: {}", tol.eps(), e.render()),
        Ok(_) => panic!("imports at ε = {:e}, where the graze is in band", tol.eps()),
    }
}

/// **A sphere face whose hole passes a micrometre from its pole**: the
/// pole is reached by a 1 µm wrap edge, and the body passes tier 3 at
/// its closed form. At an ε whose band holds the graze, the import
/// refuses it typed instead ([`refuses_at_the_band`]).
#[test]
fn a_sphere_face_whose_hole_grazes_its_pole_imports_at_its_closed_form() {
    let tol = Tol::witness();
    if graze_in_band(tol) {
        refuses_at_the_band(tol);
        return;
    }
    let (_, volume) = imported("hole_grazing_the_pole.step", tol);
    let want = 4.182_624_669_233_374e-6;
    assert!(
        (volume - want).abs() <= 1e-9 * want,
        "volume {volume} vs {want}"
    );
}

/// **That body meshes.** Pinned red where it imports: the curved lane
/// folds a triangle on the sphere face
/// (`work/tesslane/a-sphere-face-whose-hole-grazes-its-pole-folds-at-the-mesher.md`).
/// At an ε whose band holds the graze there is no body to mesh: the
/// import's typed refusal is the outcome ([`refuses_at_the_band`]).
#[test]
fn a_sphere_face_whose_hole_grazes_its_pole_meshes() {
    let tol = Tol::witness();
    if graze_in_band(tol) {
        refuses_at_the_band(tol);
        return;
    }
    test_utils::pinned_red::row(
        tol.eps(),
        &test_utils::pinned_red::EVERY_ROW,
        "the pole-grazing hole's mesh",
        &["FoldedTriangle"],
        || {
            let (body, _) = imported("hole_grazing_the_pole.step", tol);
            assert_eq!(
                mesh::tessellate(&body, 1e-3, tol).map(|_| ()),
                Ok(()),
                "the pole-grazing hole meshes"
            );
        },
    );
}
