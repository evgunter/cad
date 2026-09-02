//! **Issue 896: the route argument's single home.** The mesh walk's
//! pole guard (`mesh::walk::loop_polygon`, beside #895's) asserts
//! that no junction emitted `pole: false` lies within ε of a chart
//! pole. This file holds the measured claim that no minting door in
//! this build can put that state in front of `mesh::tessellate` —
//! the other sites (`walk.rs`'s REACHABILITY comment,
//! `tier_gate.rs`'s poleguard section) cross-reference here.
//!
//! # The argument, both firing branches
//!
//! The guard fires on a junction × pole pair the classification
//! passes over. Either branch forces a sub-band boundary feature:
//!
//! * **Rim branch**: a rim-traversal junction within ε of a pole
//!   lies on a rim of radius ρ ≤ ε (its pole distance is at least
//!   ρ), so that rim's metre-certified parameter span is at most
//!   2πρ ≤ 2πε.
//! * **Second-pole branch**: a meridian junction within ε of a pole
//!   beyond the one it is identified with requires two poles within
//!   2ε of each other; `Chart::poles` places a sphere's poles 2r
//!   apart (cones have one pole, cylinders and tori none), so this
//!   forces sphere r ≤ ε — and then EVERY edge of that sphere spans
//!   at most 2πr ≤ 2πε.
//!
//! Under the ratified K = 10 ambiguity band, a span of at most 2πε
//! cannot certify clear: it is either ≤ ε (certified zero — refused
//! `IntervalNotForward`) or inside (ε, Kε) (escalated), because
//! **K > 2π**. That premise is mechanical below
//! (`the_span_route_argument_assumes_k_above_two_pi`); at a legal
//! K = 3 the span argument voids, and the door was MEASURED to hold
//! anyway at the adoption transversality bar (`NotTransverse` /
//! second-order refusals) — shut for a different reason, so the
//! pins here state their K assumption rather than pretending the
//! derivation alone closes the door.
//!
//! # The fixtures, and the three bands
//!
//! `fixtures/poleguard/gen_poleguard.py` (committed, so the rows do
//! not depend on a Python interpreter) authors the state directly on
//! an R = 10 mm sphere: `polefrustum.step` / `poleband.step` truncate
//! 9e-8 rad below the north pole (near-pole vertices 0.9e-9 m from
//! an UNDECLARED pole — inside the default band), and
//! `poleband_eps12.step` truncates 9e-11 rad below it (0.9e-12 m —
//! inside the 1e-12 band), so every suite band has a fixture whose
//! near-pole feature is inside it. Measured refusals, pinned cell by
//! cell in `tier_gate.rs`'s `EPS_ROWS`: span escalation where the
//! span lands in the band, `IntervalNotForward` where it certifies
//! zero, and — for the 9e-8 twins at 1e-12, where their spans
//! certify — the rim/sphere near-tangency refusing one level up at
//! adoption (`NotTransverse`, and the tangency's second-order margin
//! exactly zero: the fragment `tier_gate.rs` pins).
//!
//! # The door enumeration, honest
//!
//! Measured shut: **import** (this file + `tier_gate.rs`, all three
//! bands); **revolve** and the **profile** door
//! (`mesh/tests/issue896_pole_guard.rs`: `NonManifoldAxisContact` at
//! 0.9ε, `SliverRadius` at 5ε, `DegenerateSegment`/`NonSimple` at
//! the profile, the pcurve lane far beyond); the **plane split** and
//! **boolean** doors (review rows `r2_split_door.rs` /
//! `r2_bool_door.rs` / `r1_probe_bool_route.rs`: every sphere-face
//! cut refuses typed at every probed height — as
//! `CurvedBooleanUnsupported` / `CurvedPierceUnsupported` at the
//! default band, and at earlier profile/adoption escalations on the
//! coarser bands; R1's eleven near-tangent plane×sphere
//! configurations all refuse typed). Reasoned or measured shut by
//! review: `transform_rigid` (an isometry moves junctions and poles
//! together), `split_edge` (its interiority gate is metred against
//! the same band). **Named unmeasured**: blend/fillet, `shell`,
//! `offset_charts_together`, `loft_body`/`sweep_body`,
//! `merge_faces` — and the door no certification fronts at all,
//! direct Euler-operator assembly, which is why the guard exists and
//! runs in release builds.
//!
//! # The in-tree premise, corrected
//!
//! Issue 896 recorded "no in-tree body puts a non-pole vertex within
//! any suite's ε of a pole" (lane I-e's zero-lever trace). That
//! premise is now FALSE at the 1e-6 band: `halfcap_eps7.step`
//! (landed with CERT-1, after the issue) carries a vertex
//! 1.000e-9 m from its sphere face's undeclared north pole — inside
//! 1e-6 by three orders. The no-route conclusion survives — the
//! vertex lands on the IDENTIFIED half the guard deliberately does
//! not assert (`pole_v` substitutes the pole and the body meshes
//! watertight, guard quiet) — but the premise does not, and
//! `the_halfcap_eps7_witness_is_band_shaped` below pins the
//! corrected three-band shape. Recorded as MESH-4 substrate on
//! issue 881 (the ε-type port inherits the read this witness
//! exercises).
#![allow(clippy::unwrap_used, clippy::expect_used, clippy::panic)]

use geom_core::Tol;
use step_import::{ImportOptions, StepImport, import_step};

fn fixture(name: &str) -> String {
    let p = format!(
        "{}/tests/fixtures/poleguard/{name}",
        env!("CARGO_MANIFEST_DIR")
    );
    std::fs::read_to_string(&p).unwrap_or_else(|e| panic!("reading {p}: {e}"))
}

/// The span argument's premise, mechanical: `CAD_AMBIGUITY_K` is a
/// legal per-run knob with floor 1.0, and every span-based pin in
/// this file and in `tier_gate.rs`'s poleguard cells assumes the
/// ratified K = 10 > 2π. A run below 2π is a deliberate experiment;
/// this row is what tells the experimenter the pins' premise moved
/// (the door itself was measured to hold at K = 3, at the adoption
/// transversality bar — the PINS shift, not the verdict).
#[test]
fn the_span_route_argument_assumes_k_above_two_pi() {
    let k = Tol::witness().get().k;
    assert!(
        k > 2.0 * core::f64::consts::PI,
        "ambient K = {k} <= 2π: the poleguard span pins (here and in tier_gate.rs) \
         assume K > 2π and must be re-derived for this run's band; the import door \
         itself was measured shut at K = 3 at the adoption transversality bar"
    );
}

/// The import door cannot admit a body carrying a junction inside the
/// pole band — every fixture refuses at every ambient band. The
/// default-band ParamSpan shape is additionally pinned by name when
/// the live K covers both twins' margins (2ρ = 1.8ε and 2πρ = 5.65ε
/// need K > 5.7 to land in the band rather than clear it).
#[test]
fn a_junction_inside_the_pole_band_cannot_enter_through_the_import_door() {
    let tol = Tol::witness().get();
    let (eps, k) = (tol.eps, tol.k);
    for name in ["polefrustum.step", "poleband.step", "poleband_eps12.step"] {
        let out = import_step(&fixture(name), &ImportOptions::default(), Tol::witness());
        let Err(e) = out else {
            panic!(
                "{name} imported at eps {eps:e}: the route to walk's pole guard is open \
                 and issue 896's fixture question must be re-asked — route the body to \
                 `mesh::tessellate` and demonstrate the guard, then re-pin this row"
            );
        };
        let shape = format!("{e:?}");
        #[allow(clippy::float_cmp)]
        if eps == 1e-9 && k >= 6.0 && name != "poleband_eps12.step" {
            assert!(
                shape.contains("ParamSpan"),
                "{name}: at the default band the sub-band feature's span certification \
                 refuses (interval_span_forward indeterminate); got {shape}"
            );
        }
    }
}

/// **The halfcap_eps7 witness, band-shaped** (the corrected en-route
/// finding, and the in-tree falsifier of the issue's premise): the
/// one committed body with a non-pole vertex inside a suite band's
/// reach of an undeclared chart pole. At 1e-6 the vertex is
/// identified with the pole and the body TESSELLATES WATERTIGHT,
/// guard quiet — the identified half the guard deliberately does not
/// assert, live in tree.
///
/// At 1e-9 and 1e-12 the vertex clears the band, and the file's
/// pole-crossing meridian arc — the walk's one-azimuth-per-meridian
/// model meeting an arc that lies on two chart meridians — shows up
/// TWICE, in the two places that own the two halves of it:
///
/// - `mesh::tessellate` REFUSES the face typed
///   (`CertificateExceeded`), because the chord certificate cannot
///   be met on a face whose UV polygon the arc premise breaks. It is a
///   refusal, not a panic and not a wrong mesh.
/// - `topo::examine_chart_coherence` REPORTS the arc, naming the
///   half-turn: the carrier's mid-parameter azimuth against its own
///   endpoint (`MeridianClosure`), and the file's two sub-edges of
///   that one meridian column against each other
///   (`MeridianContinuation`). Both at this vertex's own lever arm —
///   1.0e-9 m from the axis, so a half-turn there opens 3.14 nm of
///   arc, which is over the band at 1e-9 and 1e-12 and UNDER it at
///   1e-6. That is the same three-band shape this row already had,
///   measured by the door that measures rather than by an assertion
///   that panics.
///
/// Issue 1571 owns FIXING the arc premise; this row owns seeing it,
/// and it is the only row in tree that sees it through the door
/// defective coordinates actually arrive at.
#[test]
fn the_halfcap_eps7_witness_is_band_shaped() {
    let eps = Tol::witness().get().eps;
    let p = format!(
        "{}/tests/fixtures/halfcap/halfcap_eps7.step",
        env!("CARGO_MANIFEST_DIR")
    );
    let text = std::fs::read_to_string(&p).unwrap();
    let Ok(StepImport::Solid { body, .. }) =
        import_step(&text, &ImportOptions::default(), Tol::witness())
    else {
        panic!("halfcap_eps7 must import at eps {eps:e}");
    };
    // The file's own geometry, asserted band-independently: its
    // sphere face's undeclared north pole has a declared vertex
    // ~1.000e-9 m away.
    let (mut pole, mut radius) = (None, 0.0f64);
    for (_, f) in body.faces() {
        if let Some(geom::Surface::Sphere {
            center,
            radius: r,
            axis,
            ..
        }) = body.get_surface(f.surface)
        {
            pole = Some(*center + *axis * *r);
            radius = *r;
        }
    }
    let pole = pole.expect("a sphere face");
    assert!(radius > 0.0);
    let nearest = body
        .vertices()
        .map(|(_, v)| (*body.get_point(v.point).unwrap() - pole).norm())
        .fold(f64::INFINITY, f64::min);
    assert!(
        (0.9e-9..=1.1e-9).contains(&nearest),
        "the witness vertex sits ~1e-9 m from the undeclared pole; measured {nearest:e}"
    );

    let hook = std::panic::take_hook();
    std::panic::set_hook(Box::new(|_| {}));
    let out = std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| {
        mesh::tessellate(&body, 1.0e-3, Tol::witness())
    }));
    std::panic::set_hook(hook);
    let out = out.unwrap_or_else(|_| {
        panic!("at eps {eps:e} the witness must not panic — nothing here asserts")
    });
    let report = topo::examine_chart_coherence(&body, Tol::witness());
    assert!(report.unexamined.is_empty(), "{:?}", report.unexamined);
    if (0.99e-6..=1.01e-6).contains(&eps) {
        let mesh = out.expect("tessellates");
        mesh::validate::check_mesh(&mesh).expect("watertight");
        assert!(
            nearest <= eps,
            "the vertex is inside this band, on the identified half"
        );
        assert!(
            report.findings.is_empty(),
            "a half-turn at a lever arm of {nearest:e} m opens {:e} m of arc, which this band calls noise: {:?}",
            core::f64::consts::PI * nearest,
            report.findings
        );
    } else if (0.99e-9..=1.01e-9).contains(&eps) || (0.99e-12..=1.01e-12).contains(&eps) {
        let err = out.expect_err("the arc premise breaks the chord certificate");
        assert!(
            matches!(err, mesh::TessellateError::CertificateExceeded { .. }),
            "a typed refusal, not a panic and not a mesh; got {err:?}"
        );
        let kinds: Vec<_> = report.findings.iter().map(|f| f.condition).collect();
        assert!(
            kinds
                .iter()
                .any(|c| matches!(c, topo::CoherenceCondition::MeridianClosure { .. })),
            "the arc's carrier sits a half-turn from its own endpoint: {kinds:?}"
        );
        assert!(
            kinds
                .iter()
                .any(|c| matches!(c, topo::CoherenceCondition::MeridianContinuation { .. })),
            "and the file's two sub-edges of that column disagree by the same half-turn: {kinds:?}"
        );
        for f in &report.findings {
            assert!(
                (f.gap - core::f64::consts::PI).abs() < 1e-12,
                "a half-turn, got {} rad",
                f.gap
            );
            assert!(
                f.metres >= eps,
                "every finding clears its own band: {} m over {eps:e}",
                f.metres
            );
        }
    }
    // Other ambient bands: the import-Pass and vertex-distance
    // assertions above still ran; the walk outcome is unpinned there.
}
