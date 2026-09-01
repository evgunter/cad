//! **Issue 896, the import-door half of the fixture question.** The
//! issue named a STEP import as the plausible route to a boundary
//! junction inside the mesh walk's pole band (a non-pole vertex
//! within ε of an UNDECLARED chart pole). Measured: the door is shut,
//! and this row pins it shut, so the day it opens the guard's fixture
//! question is re-asked instead of the route silently appearing.
//!
//! The fixtures (`fixtures/poleguard/gen_poleguard.py`, committed so
//! the row does not depend on a Python interpreter) author the state
//! directly: an R = 10 mm sphere truncated 9e-8 rad below its north
//! pole, so the pole carries no vertex while the top rim's vertices
//! sit 0.9e-9 m from it. `polefrustum.step` is the halfcap family's
//! half-solid (its top rim ends carry the near-pole vertices);
//! `poleband.step` is the seam-authored full solid (top rim a closed
//! circle, one near-pole vertex).
//!
//! Why the door cannot admit them — and cannot admit ANY authoring of
//! the state: a junction within ε of a chart pole lies on a rim whose
//! radius is at most that distance, so some boundary feature measures
//! at most 2πε — under the ratified K = 10 band (and any K > 2π) its
//! certification cannot land clear of the indeterminate zone. The
//! measured refusals, per suite ε leg:
//!
//! * default 1e-9: `ParamSpan` escalation (`interval_span_forward`
//!   indeterminate — the top diameter LINE's 1.8e-9 m span for the
//!   frustum, the top CIRCLE's 5.65e-9 m arc span for the band).
//! * 1e-6: the same spans certify ZERO — `IntervalNotForward`.
//! * 1e-12: the spans certify positive, and the adoption lane refuses
//!   one level up — `NotTransverse` / `NotSecondOrderSeparated`: a
//!   rim 9e-8 rad below the pole meets the sphere nearly
//!   tangentially, and the curve–surface certification says so.
//!
//! Only the default band's shape is pinned by name; the other legs'
//! shapes belong to their own certification lanes and are recorded
//! here as measurements, with the row asserting refusal alone. The
//! construction-door measurement is `mesh/tests/issue896_pole_guard.rs`;
//! the guard these fixtures cannot reach is demonstrated firing by
//! `mesh::walk::tests::a_rim_junction_inside_the_pole_band_trips_the_guard`.
#![allow(clippy::unwrap_used, clippy::expect_used, clippy::panic)]

use geom_core::Tol;
use step_import::{ImportOptions, import_step};

fn fixture(name: &str) -> String {
    let p = format!(
        "{}/tests/fixtures/poleguard/{name}",
        env!("CARGO_MANIFEST_DIR")
    );
    std::fs::read_to_string(&p).unwrap_or_else(|e| panic!("reading {p}: {e}"))
}

#[test]
fn a_junction_inside_the_pole_band_cannot_enter_through_the_import_door() {
    let eps = Tol::witness().get().eps;
    for name in ["polefrustum.step", "poleband.step"] {
        let out = import_step(&fixture(name), &ImportOptions::default(), Tol::witness());
        let Err(e) = out else {
            panic!(
                "{name} imported at eps {eps:e}: the route to walk's pole guard is open \
                 and issue 896's fixture question must be re-asked — route the body to \
                 `mesh::tessellate` and demonstrate the guard, then re-pin this row"
            );
        };
        let shape = format!("{e:?}");
        // The default band's refusal is pinned by name; other bands
        // assert the refusal alone (module docs).
        #[allow(clippy::float_cmp)]
        if eps == 1e-9 {
            assert!(
                shape.contains("ParamSpan"),
                "{name}: at the default band the sub-band feature's span certification \
                 refuses (interval_span_forward indeterminate); got {shape}"
            );
        }
    }
}
