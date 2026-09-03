//! The rows, end to end: a real body tessellated with the meter armed,
//! turned into the rows a sweep writes.
//!
//! What is checked is what this crate is responsible for — that EVERY
//! face gets a row (the question "which face IS the scene's cost" is
//! unanswerable if only the Hessian-sized lane reports), that the rows
//! account for every triangle in the mesh, and that the sizing columns
//! belong to the Hessian-sized lane and to no other.

#![allow(clippy::unwrap_used, clippy::expect_used, clippy::panic)]

use geom_core::{Affine3, Point2, Tol, Vec3};
use mesh::budget::{self, FaceMeasure, Mode};
use profile::{ProfileLoop, RawLoop as _};
use sweep::loft_body;
use tess_meter::{Bound, Chart, best_split_cells, divisions, face_rows};
use test_utils::vacuity::Exposure;
use topo::Body;

/// The `loft_prism` corpus body (#212): squares at z = 0 and 2, the
/// non-affine trapezoid at z = 1, v-degree 2 — NURBS walls and planar
/// caps in one body, which is the mix the row rules are about.
fn loft_prism(tol: Tol) -> Body<f64> {
    let quad = |pts: [(f64, f64); 4]| -> sweep::Section {
        vec![ProfileLoop::polygon(
            pts.iter().map(|&(x, y)| Point2::new(x, y)),
        )]
    };
    let sections = vec![
        quad([(-1.0, -1.0), (1.0, -1.0), (1.0, 1.0), (-1.0, 1.0)]),
        quad([(-1.375, -1.0), (1.375, -1.0), (1.0, 1.0), (-1.0, 1.0)]),
        quad([(-1.0, -1.0), (1.0, -1.0), (1.0, 1.0), (-1.0, 1.0)]),
    ];
    let places: Vec<Affine3<f64>> = [0.0, 1.0, 2.0]
        .iter()
        .map(|z| Affine3::translation(Vec3::new(0.0, 0.0, *z)))
        .collect();
    loft_body::<f64>(&sections, &places, 2, tol)
        .expect("the corpus loft builds")
        .body
}

#[test]
fn every_face_gets_a_row_and_only_nurbs_faces_get_sizing() {
    let tol = Tol::witness();
    let body = loft_prism(tol);
    budget::arm(Mode::Sizing);
    let mesh = mesh::tessellate(&body, 6e-3, tol).expect("tessellates");
    let measures = budget::take();
    let rows = face_rows(6e-3, &body, &mesh, &measures);

    assert_eq!(
        rows.len(),
        body.faces().count(),
        "one row per face, planar caps included"
    );
    assert_eq!(
        rows.iter().map(|r| r.triangles).sum::<usize>(),
        mesh.patches
            .iter()
            .map(|p| p.triangles.len())
            .sum::<usize>(),
        "the rows account for every triangle in the mesh"
    );
    for r in &rows {
        assert_eq!(
            r.nurbs.is_some(),
            r.chart == Chart::Nurbs,
            "sizing columns belong to the Hessian-sized lane and to no other"
        );
    }

    let walls: Vec<_> = rows.iter().filter_map(|r| r.nurbs).collect();
    assert!(!walls.is_empty(), "the loft's walls are NURBS faces");
    for n in &walls {
        assert!(n.grid_cells > 0.0 && n.span_opt_cells > 0.0, "{n:?}");
        // (Positivity is about the PARTS. The composition they are
        // parts of — the split derivation at the call's own sizing
        // target — is asserted in
        // `the_reported_cells_are_the_split_derivation_at_the_calls_sizing_target`.)
        //
        // (No ordering assertion between `span_opt_cells` and either
        // cell count. Both candidates are vacuous HERE, for one
        // mechanism: `best_split_cells` seeds its running minimum with
        // the lane's own steps, so an inequality against the schedule
        // those steps produced holds by construction of the loop, not
        // by anything about the answer. `opt_cells <= patch_cells` is
        // vacuous for every body; `span_opt_cells <= grid_cells`
        // separates from its seed only when a face has more than one
        // knot-span cell, so that the BAND takes a max the per-cell
        // ideal does not — and every NURBS face of this fixture has
        // `cells = 1`. Multi-cell faces are not rare (56 of the
        // committed tour baseline's 64 NURBS rows), but a fixture
        // built from one would buy a guard rather than a detector:
        // over those 56 the ratio `span_opt_cells / grid_cells` runs
        // 0.16 to 0.73, and the one mechanism that could invert it is
        // the per-cell `ceil` named just below.)
        //
        // (THE 56, THE 64 AND THAT RANGE ARE A READING OF THE COMMITTED
        // BASELINE, NOT OF THIS FIXTURE, AND NOTHING RE-TAKES THEM.
        // `docs/tess-budget-data/` is re-cut deliberately, by
        // `scripts/tess_budget_sweep.sh`, and every re-cut moves these
        // three — a scene added to the tour changes them without
        // touching this file, and no run compares them against anything.
        // They are here to say why the missing assertion is a judgement
        // about REACH rather than an oversight, and that argument holds
        // at any distribution in which multi-cell faces are common. A
        // reader who needs the current figures reads the baseline, which
        // carries its own `# tess-budget-cut:` line naming the tree it
        // came from; that stamp is the guard on the reading's age, and
        // it is a guard this comment does not have.)
        //
        // (No `grid_cells <= patch_cells` assertion either: the
        // per-cell schedule pays a `ceil` per cell, and a face with
        // many near-empty cells can honestly cost a few cells MORE
        // than the whole-patch grid — #547 measured exactly that on
        // the swept blades, span 0.9x.)
        assert!(
            n.worst_cert.is_finite() && n.worst_cert > 0.0,
            "the face's worst certificate is recorded: {n:?}"
        );
        assert!(
            n.worst_dev.is_nan() && n.dev_samples == 0,
            "Mode::Sizing does not resample: {n:?}"
        );
    }
}

/// A face the meter said nothing about still gets a row, and its row
/// is the empty-tailed shape — a zero in a sizing column would read as
/// a measured zero.
#[test]
fn a_face_with_no_measurements_gets_an_empty_tailed_row() {
    let tol = Tol::witness();
    let body = loft_prism(tol);
    let mesh = mesh::tessellate(&body, 6e-3, tol).expect("tessellates");
    let rows = face_rows(6e-3, &body, &mesh, &[]);
    assert_eq!(rows.len(), body.faces().count());
    assert!(rows.iter().all(|r| r.nurbs.is_none()));
    let cols = tess_meter::CSV_HEADER.split(',').count();
    for r in &rows {
        assert_eq!(r.csv_row("s/b").split(',').count(), cols);
    }
}

/// The chordal tolerance every row in this file is measured at.
const ROW_DELTA: f64 = 6e-3;

/// The sizing target a tessellation at chordal tolerance `delta` sizes
/// against — `mesh::sizing::sizing_target`'s documented halving, which
/// is a kernel rule and not a meter one.
///
/// **This is the independent handle the composition assertion below
/// rests on.** It restates the kernel's rule rather than reading
/// `FaceMeasure::delta_s`, so the recomposition's tolerance comes from
/// the tolerance THIS FILE asked for and not from anything
/// `columns()` touched.
fn sizing_target(delta: f64) -> f64 {
    delta * 0.5
}

/// `columns()`' two split derivations, spelled here as the composition
/// they are meant to be: the whole-patch bound over the trim box, and
/// the per-cell bounds over their clipped sub-boxes, each at
/// `delta_s`.
///
/// The per-cell sum accumulates in `cells` order because the quantity
/// it reproduces does, and the comparison is bit for bit.
fn recomposed_cells(m: &FaceMeasure, delta_s: f64) -> (f64, f64) {
    let (du, dv) = (m.u.1 - m.u.0, m.v.1 - m.v.0);
    let patch = Bound {
        muu: m.muu,
        muv: m.muv,
        mvv: m.mvv,
        steps: m.patch_steps,
    };
    let mut span_opt = 0.0;
    for c in &m.cells {
        let cdu = c.u.1.min(m.u.1) - c.u.0.max(m.u.0);
        let cdv = c.v.1.min(m.v.1) - c.v.0.max(m.v.0);
        if cdu <= 0.0 || cdv <= 0.0 {
            continue; // cell outside the trim box
        }
        span_opt += best_split_cells(Bound::from(c), cdu, cdv, delta_s);
    }
    (best_split_cells(patch, du, dv, delta_s), span_opt)
}

/// **The reported cell counts ARE the derivation at the call's own
/// sizing target** — asserted on the COMPOSITION, because the parts
/// being positive leaves the call site free.
///
/// `columns()` hands `FaceMeasure::delta_s` to `best_split_cells` and
/// to the per-cell sum, and nothing about either of those functions
/// says which tolerance reaches them. A retune there — a factor on
/// `δ_s`, a target of its own — leaves every column finite and
/// positive, so the pre-existing row above stays green while the
/// reported cell count moves by up to +100%.
///
/// So this row spells the intended composition and compares bit for
/// bit, at a `δ_s` it derives from [`ROW_DELTA`] through
/// [`sizing_target`] rather than reading off the measurement. The
/// recomposition supplies the SHAPE — which bound, which box, which
/// per-cell clip — and the kernel's halving rule supplies the
/// tolerance, which is the half a call-site retune moves.
///
/// The tally is the tightness check the equality needs: an equality
/// that would hold at any `δ_s` says nothing about the one the call
/// site passed, so each face is also recomposed at a different target
/// and the answers are required to separate.
#[test]
fn the_reported_cells_are_the_split_derivation_at_the_calls_sizing_target() {
    let tol = Tol::witness();
    let body = loft_prism(tol);
    budget::arm(Mode::Sizing);
    let mesh = mesh::tessellate(&body, ROW_DELTA, tol).expect("tessellates");
    let measures = budget::take();
    let rows = face_rows(ROW_DELTA, &body, &mesh, &measures);
    let delta_s = sizing_target(ROW_DELTA);

    let mut seen = Exposure::new("δ_s sensitivity");
    let mut nurbs_rows = 0usize;
    for (ordinal, patch) in mesh.patches.iter().enumerate() {
        let Some(n) = rows[ordinal].nurbs else {
            continue;
        };
        nurbs_rows += 1;
        let m = measures
            .iter()
            .find(|m| m.face == patch.face)
            .expect("a filled sizing column came from a measurement");
        assert_eq!(
            m.delta_s.to_bits(),
            delta_s.to_bits(),
            "the lane sized face {ordinal} against {} and not against δ/2 = {delta_s}",
            m.delta_s
        );

        let (du, dv) = (m.u.1 - m.u.0, m.v.1 - m.v.0);
        let (opt, span_opt) = recomposed_cells(m, delta_s);
        assert_eq!(
            (
                n.opt_cells.to_bits(),
                n.span_opt_cells.to_bits(),
                n.grid_cells.to_bits(),
                n.nu.to_bits(),
                n.nv.to_bits(),
                n.patch_cells.to_bits()
            ),
            (
                opt.to_bits(),
                span_opt.to_bits(),
                (m.grid_cells as f64).to_bits(),
                divisions(du, m.patch_steps.0).to_bits(),
                divisions(dv, m.patch_steps.1).to_bits(),
                (divisions(du, m.patch_steps.0) * divisions(dv, m.patch_steps.1)).to_bits()
            ),
            "face {ordinal}'s columns are no longer the shipped derivation: {n:?}"
        );

        let (other_opt, other_span) = recomposed_cells(m, delta_s * 0.5);
        if other_opt != opt {
            seen.note("opt_cells moves with δ_s");
        }
        if other_span != span_opt {
            seen.note("span_opt_cells moves with δ_s");
        }
    }
    assert!(nurbs_rows > 0, "the loft's walls are NURBS faces");
    seen.report();
    seen.require_each(
        &["opt_cells moves with δ_s", "span_opt_cells moves with δ_s"],
        1,
        "an equality that holds at every sizing target cannot see a retune of the one \
         `columns()` passes",
    );
}
