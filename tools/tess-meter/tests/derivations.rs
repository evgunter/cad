//! The derivations this crate owns, checked where they are: the split
//! optimizer's answer against the certificate it claims to satisfy,
//! its determinism, the ruled-wall case #320 is about, and the CSV's
//! two row shapes agreeing about their width.

#![allow(clippy::unwrap_used, clippy::expect_used, clippy::panic)]

use tess_meter::{
    Bound, CSV_HEADER, Chart, FaceRow, NurbsColumns, best_split_cells, best_split_steps, divisions,
};
use test_utils::fuzz;

/// A synthetic [`Bound`] with a plausible seed in its `steps`.
///
/// **This is a FIXTURE, not a second copy of the lane's schedule.**
/// `best_split_steps` treats `steps` as an opaque starting point for
/// its running minimum — it never asks where the pair came from — so
/// the properties asserted below (the answer certifies; the reported
/// count matches the reported steps; a ruled wall improves) hold for
/// any seed, and this one is chosen only because it is the shape the
/// kernel actually reports. The KERNEL-supplied steps are exercised
/// end to end in `rows.rs`, which tessellates a real body and reads
/// `mesh::budget`'s own `patch_steps` / `CellMeasure::steps`.
fn bound(muu: f64, muv: f64, mvv: f64, delta_s: f64) -> Bound {
    let step = |group: f64| {
        if group > 0.0 {
            (delta_s / (2.0 * group)).sqrt()
        } else {
            f64::INFINITY
        }
    };
    Bound {
        muu,
        muv,
        mvv,
        steps: (step(muu + muv), step(mvv + muv)),
    }
}

/// The claim the whole "split slack" column rests on: the grid
/// [`best_split_steps`] picks satisfies the SAME certificate the lane
/// checks, `muu·h_u² + 2·muv·h_u·h_v + mvv·h_v² ≤ δ_s`.
///
/// That inequality is the per-triangle bound `Q/4` at the lane's own
/// two-cells-per-axis budgeting (`a_u ≤ 2h_u`, `a_v ≤ 2h_v`), so a
/// grid satisfying it certifies exactly as the shipped one does.
/// Asserted on the ANSWER over random bounds — including the
/// degenerate corners (a ruled direction's `muu = 0`, a zero cross
/// term) where the closed-form optimum does not exist.
#[test]
fn the_cheapest_split_still_satisfies_the_certificate() {
    let mut rng = fuzz::start("tess_meter::cheapest_split");
    // Log-uniform magnitudes, with a zero in each slot often enough to
    // exercise the degenerate corners (a ruled direction's muu = 0, a
    // dead cross term).
    fn mag(r: &mut fuzz::Rng) -> f64 {
        if r.unit() < 0.2 {
            0.0
        } else {
            10.0f64.powf(r.range(-6.0, 4.0))
        }
    }
    for _ in 0..fuzz::scaled(400) {
        let delta_s = 10.0f64.powf(rng.range(-6.0, -1.0));
        let b = bound(mag(&mut rng), mag(&mut rng), mag(&mut rng), delta_s);
        let (du, dv) = (
            10.0f64.powf(rng.range(-2.0, 2.0)),
            10.0f64.powf(rng.range(-2.0, 2.0)),
        );
        let (cells, hu, hv) = best_split_steps(b, du, dv, delta_s);
        // An infinite step means "one division", which is the whole
        // extent — that is what the certificate must be checked at,
        // not at ∞.
        let (hu, hv) = (hu.min(du), hv.min(dv));
        let q = b.muu * hu.powi(2) + 2.0 * b.muv * hu * hv + b.mvv * hv.powi(2);
        assert!(
            q <= delta_s * (1.0 + 1e-9),
            "cheapest split violates the certificate: q={q:e} > delta_s={delta_s:e} \
             at h=({hu:e},{hv:e}) for {b:?} — {}",
            fuzz::replay()
        );
        // …and the count it reports is the count its OWN steps give.
        //
        // NOT `cells <= divisions(du, b.steps.0) * divisions(dv,
        // b.steps.1)`, which was here and could not fail: the scan
        // SEEDS its running minimum with exactly that product
        // (`best_split_steps`), so "never loses to the lane" is a
        // property of the seed, not of the search. What can fail is
        // the tuple going out of step with itself — a count updated
        // without its steps, or vice versa — and that is what a
        // consumer of `(cells, h_u, h_v)` actually relies on. The
        // search's own claim is the certificate assertion above, and
        // that it is STRICTLY better on the anisotropic case, which
        // `a_ruled_wall_pays_for_its_flat_direction` pins.
        let (_, rhu, rhv) = best_split_steps(b, du, dv, delta_s);
        assert_eq!(
            cells.to_bits(),
            (divisions(du, rhu) * divisions(dv, rhv)).to_bits(),
            "the reported cell count is not the count its own steps give, \
             for {b:?} — {}",
            fuzz::replay()
        );
    }
}

/// The scan is a fixed grid of aspect ratios (D9: structure, never a
/// data-dependent iteration), so the answer is reproducible.
#[test]
fn the_split_scan_is_deterministic() {
    let b = bound(0.0, 2.4, 51.3, 1e-3);
    let x = best_split_cells(b, 1.0, 1.0, 1e-3);
    let y = best_split_cells(b, 1.0, 1.0, 1e-3);
    assert_eq!(x.to_bits(), y.to_bits());
}

/// A ruled wall is the case #320 is about: `muu = 0` with a live cross
/// term. The lane's symmetric split charges the cross term to BOTH
/// directions and grids the flat one anyway; the cheapest split spends
/// its divisions where the curvature is.
#[test]
fn a_ruled_wall_pays_for_its_flat_direction() {
    let b = bound(0.0, 2.4, 51.3, 1e-3);
    let lane = divisions(1.0, b.steps.0) * divisions(1.0, b.steps.1);
    let (best, hu, _) = best_split_steps(b, 1.0, 1.0, 1e-3);
    assert!(
        lane / best > 3.0,
        "expected the ruled wall's split slack to be several-fold, got {:.2}x",
        lane / best
    );
    // Asserted on the DIVISIONS, not on `h_u` itself. The objective is
    // a step function of the aspect ratio, so which sample wins is a
    // property of the scan's lattice: refining the scan moves the
    // winning `h_u` between 1.02 and 0.34 while the answer improves,
    // and an `h_u >= 1.0` line therefore reds on scans that are
    // strictly better. What is stable — and what the finding is about
    // — is that the cheapest split spends an order of magnitude fewer
    // divisions on the flat direction than the lane does.
    let (best_u, lane_u) = (divisions(1.0, hu), divisions(1.0, b.steps.0));
    assert!(
        best_u * 10.0 <= lane_u,
        "the cheapest split should barely divide the FLAT direction: \
         {best_u} divisions against the lane's {lane_u}"
    );
}

/// The empty-tail arm and the filled arm must agree about the row's
/// width, or every consumer's column indices are off by the
/// difference.
#[test]
fn both_row_shapes_have_the_headers_width() {
    let cols = CSV_HEADER.split(',').count();
    let plane = FaceRow {
        face: 0,
        chart: Chart::Plane,
        delta: 1e-3,
        triangles: 2,
        nurbs: None,
    };
    assert_eq!(plane.csv_row("s/b").split(',').count(), cols);
    let nurbs = FaceRow {
        chart: Chart::Nurbs,
        nurbs: Some(NurbsColumns {
            u: (0.0, 1.0),
            v: (0.0, 1.0),
            nu: 4.0,
            nv: 5.0,
            muu: 1.0,
            muv: 2.0,
            mvv: 3.0,
            cells: 6,
            grid_cells: 12.0,
            patch_cells: 20.0,
            opt_cells: 10.0,
            span_opt_cells: 8.0,
            worst_cert: 1e-4,
            worst_dev: 5e-5,
            dev_samples: 7,
        }),
        ..plane
    };
    assert_eq!(nurbs.csv_row("s/b").split(',').count(), cols);
}

/// The header this crate writes and the one `tools/tess-lint` parses
/// are the same bytes.
///
/// **Why source text and not a shared constant.** The obvious answer —
/// a schema-only crate both depend on — would break the property
/// `tess-lint`'s manifest states as its design: *"It has no
/// dependencies at all: it reads the CSV … and the kernel does not
/// appear even as a dev-dependency, its fixtures are CSV text, which
/// is the whole contract between the two halves."* A consumer that
/// shares a constant with its producer can no longer fail as a
/// PARSER when the producer's schema moves, which is the failure mode
/// the lint wants. So the two declarations stay independent and this
/// test reads the other one's source.
///
/// It is a real pin and it is ugly: it parses Rust string
/// continuations out of a sibling crate's `lib.rs`, and it breaks if
/// that declaration is reformatted rather than changed. It replaces a
/// comment asking a human to check, which could not break at all.
#[test]
fn the_lints_expected_header_is_this_one() {
    let lint = include_str!("../../tess-lint/src/lib.rs");
    let quoted = lint
        .split("pub const EXPECTED_HEADER: &str = ")
        .nth(1)
        .expect("tess-lint declares EXPECTED_HEADER");
    let end = quoted.find(';').expect("the declaration ends");
    // Rust string continuations: drop the backslash-newline-indent runs.
    let mut header = String::new();
    let mut chars = quoted[..end].chars().peekable();
    while let Some(c) = chars.next() {
        match c {
            '"' => {}
            '\\' => {
                while chars.peek().is_some_and(|c| c.is_whitespace()) {
                    chars.next();
                }
            }
            c => header.push(c),
        }
    }
    assert_eq!(header, CSV_HEADER);
}
