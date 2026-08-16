//! The **tessellation-budget lint** (issue #320): reads the per-face
//! budget CSV `mesh::budget` records (`demo-tour tess-budget`) and
//! answers two different questions with it.
//!
//! # 1. The report: where does the mesh actually go, and why
//!
//! Per scene and per face: triangles, and the factors that say how
//! many of them the deviation budget actually needed. The factors are
//! ratios of GRID CELL COUNTS, all counted over the same trim box with
//! the same `ceil` discipline, so they are directly comparable.
//!
//! **Re-derived at TESS-SPAN** (the #320 span promotion): the shipped
//! grid is per-knot-span-cell-sized now, recorded as `grid_cells`;
//! the retired whole-patch-sup schedule rides along as the
//! COUNTERFACTUAL `patch_cells` column so the held gain stays a
//! number; `span_cells` is the meter's independent per-cell
//! prediction of the shipped schedule.
//!
//! * **held** = `patch_cells / grid_cells` — the span gain TESS-SPAN
//!   holds over whole-patch sizing. A regression toward whole-patch
//!   sizing drives it toward 1.0 (and fires the gate through
//!   `recoverable`, below).
//! * **agree** = `grid_cells / span_cells` — ~1.00 by construction:
//!   the lane's actual schedule against the meter's independent
//!   prediction. Drift means the shipped sizing and the analysis no
//!   longer describe the same grid.
//! * **split** = `grid_cells / span_opt_cells` — what is still
//!   recoverable by picking a cheaper point on each cell's
//!   constraint ellipse `muu·h_u² + 2·muv·h_u·h_v + mvv·h_v² ≤ δ_s`;
//!   the shipped schedule still reaches it through the decoupling
//!   `2·a_u·a_v ≤ a_u² + a_v²` (the split unit's open question).
//!   Anisotropic walls (a ruled direction: `muu ≈ 0` with `muv > 0`)
//!   pay the most.
//! * **total** = `delta / worst_dev` — the deviation budget that went
//!   unspent, when the sweep ran with `--deviation`. A softer number
//!   than the ones above: `worst_dev` is sampled (so it under-reports
//!   deviation and over-reports slack) and the `h² ↔ 1/h²` scaling that
//!   turns it into a triangle count is a first-order extrapolation.
//!
//! `held` and `split` are **realizable without weakening any
//! certificate** (held IS realized — the shipped lane holds it):
//! each counts a grid whose every cell satisfies the same
//! per-triangle bound the shipped lane checks.
//!
//! One caveat on `split`, because the number is otherwise too
//! flattering: the cheapest point on the constraint curve is a STRIP
//! on a ruled wall (one division across the flat direction, thousands
//! along the curved one). It certifies, but it is an upper bound on
//! what an aspect-respecting schedule would recover — see
//! `mesh::budget`'s module docs and docs/TESS-BUDGET.md.
//!
//! # 2. The gate: has the budget regressed?
//!
//! With `--baseline`, findings are DIFFERENCES against a committed
//! sweep, never absolute thresholds — because at the head where this
//! tool was written the absolute factors are large and *known* (the
//! report says so, loudly, and #320 tracks the fix). An absolute
//! threshold would therefore have to be set above the current state to
//! be green, which makes it a threshold that certifies nothing. What a
//! baseline comparison catches is the thing nobody notices by hand:
//!
//! 1. **Triangle-count growth** — a scene's mesh grew by more than
//!    [`GROWTH_TOLERANCE`]. Tessellation cost is invisible in a diff.
//! 2. **Slack growth** — a face's recoverable slack
//!    (`grid_cells / span_opt_cells`) grew by more than
//!    [`GROWTH_TOLERANCE`]: the sizing schedule got MORE wasteful,
//!    which a triangle count alone can hide (a smaller, flatter face
//!    can regress in slack while shrinking). Since TESS-SPAN this is
//!    also the tripwire for a silent revert to whole-patch sizing —
//!    `grid_cells` would jump by the held span factor.
//! 3. **Scene disappeared** — a baseline scene the fresh sweep has no
//!    row for. Silent coverage loss reads as an improvement in every
//!    total, so it is a finding, not a footnote.
//!
//! A scene the FRESH sweep adds is not a finding (new scenes are
//! normal); it is reported, so the baseline's staleness stays visible.
//!
//! # Reading a firing gate
//!
//! Same discipline as `k-lint`, and for the same reason: a fired lint
//! is evidence that the budget DISTRIBUTION moved. Growth can be
//! entirely legitimate (a scene got a genuinely more curved wall) — the
//! recourse is then to re-cut the baseline and say why in the commit,
//! never to coarsen δ or simplify geometry to get the number down.

/// One face's row, as parsed. Every ratio this lint reports is derived
/// here rather than stored, so the CSV stays measurements only.
#[derive(Clone, Debug, PartialEq)]
pub struct Row {
    /// `<stop>/<body>`.
    pub scene: String,
    /// Face ordinal within its body.
    pub face: usize,
    /// Chart tag (`nurbs`, `plane`, …).
    pub chart: String,
    /// The δ the sweep tessellated at.
    pub delta: f64,
    /// Triangles this face contributed.
    pub triangles: usize,
    /// The Hessian-sized lane's columns; `None` on other charts.
    pub nurbs: Option<Nurbs>,
}

/// The NURBS lane's sizing columns (`mesh::budget::NurbsBudget`).
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct Nurbs {
    /// The grid the lane actually built (TESS-SPAN: per-cell-sized),
    /// as a cell count.
    pub grid_cells: f64,
    /// The retired whole-patch-sup schedule's cell count — the
    /// counterfactual column.
    pub patch_cells: f64,
    /// Cheapest uniform grid the same whole-patch bound admits.
    pub opt_cells: f64,
    /// The meter's independent per-cell prediction of the shipped
    /// schedule.
    pub span_cells: f64,
    /// Per-cell sizing at the cheapest split.
    pub span_opt_cells: f64,
    /// Worst per-triangle certificate the face emitted.
    pub worst_cert: f64,
    /// Worst SAMPLED deviation, `NaN` when the sweep did not resample.
    pub worst_dev: f64,
}

impl Row {
    /// `patch_cells / grid_cells` — the held span gain, or `None` off
    /// the Hessian-sized lane.
    pub fn span_held(&self) -> Option<f64> {
        self.nurbs.map(|n| ratio(n.patch_cells, n.grid_cells))
    }

    /// `grid_cells / span_cells` — the lane-vs-meter agreement,
    /// ~1.0 by construction.
    pub fn agreement(&self) -> Option<f64> {
        self.nurbs.map(|n| ratio(n.grid_cells, n.span_cells))
    }

    /// `grid_cells / span_opt_cells` — the recoverable slack (the
    /// gate's per-face ratio).
    pub fn recoverable(&self) -> Option<f64> {
        self.nurbs.map(|n| ratio(n.grid_cells, n.span_opt_cells))
    }

    /// `delta / worst_dev` — the unspent deviation budget, `None`
    /// unless the sweep resampled.
    pub fn total_slack(&self) -> Option<f64> {
        self.nurbs
            .filter(|n| n.worst_dev.is_finite() && n.worst_dev > 0.0)
            .map(|n| self.delta / n.worst_dev)
    }
}

/// A ratio that answers 1.0 rather than a NaN/∞ when the denominator
/// is absent — "no slack measured" is the honest reading of a face
/// whose comparison grid could not be counted, and it is the reading
/// that cannot manufacture a finding.
fn ratio(num: f64, den: f64) -> f64 {
    if den > 0.0 && num.is_finite() {
        num / den
    } else {
        1.0
    }
}

/// A malformed input row: the lint could not run, which is not a
/// statement about tessellation (`main.rs` gives it its own exit).
#[derive(Debug, PartialEq, Eq)]
pub struct ParseError {
    /// 1-based line number in the file.
    pub line: usize,
    /// What was wrong.
    pub text: String,
}

/// The column order [`parse`] requires, byte for byte
/// `mesh::budget::CSV_HEADER`. Pinned HERE as well as there on
/// purpose: the two halves are separate cargo roots by design, so
/// there is no shared constant to import, and a drifting sweep must
/// fail as harness breakage rather than parse into wrong columns.
pub const EXPECTED_HEADER: &str = "scene,face,chart,delta,triangles,u0,u1,v0,v1,nu,nv,\
                                   muu,muv,mvv,cells,grid_cells,patch_cells,opt_cells,\
                                   span_cells,span_opt_cells,worst_cert,worst_dev,dev_samples";

/// Parses a budget CSV.
///
/// # Errors
///
/// [`ParseError`] on a missing/renamed header, a short row, or a field
/// that does not parse — all harness breakage.
pub fn parse(text: &str) -> Result<Vec<Row>, ParseError> {
    let mut lines = text.lines().enumerate();
    let (_, header) = lines.next().ok_or(ParseError {
        line: 0,
        text: "empty file".into(),
    })?;
    if header.trim() != EXPECTED_HEADER {
        return Err(ParseError {
            line: 1,
            text: format!("unexpected header (sweep format drift?): {header}"),
        });
    }
    let expected = EXPECTED_HEADER.split(',').count();
    let mut rows = Vec::new();
    for (i, line) in lines {
        let n = i + 1;
        if line.trim().is_empty() {
            continue;
        }
        let f: Vec<&str> = line.split(',').collect();
        if f.len() != expected {
            return Err(ParseError {
                line: n,
                text: format!("{} fields, expected {expected}", f.len()),
            });
        }
        let num = |col: usize, name: &str| -> Result<f64, ParseError> {
            f[col].parse::<f64>().map_err(|e| ParseError {
                line: n,
                text: format!("{name}: {e} ({:?})", f[col]),
            })
        };
        let idx = |col: usize, name: &str| -> Result<usize, ParseError> {
            f[col].parse::<usize>().map_err(|e| ParseError {
                line: n,
                text: format!("{name}: {e} ({:?})", f[col]),
            })
        };
        // The sizing columns are empty on every non-NURBS chart. All
        // present or all absent — a half-filled row is drift.
        let sizing: Vec<&str> = vec![f[15], f[16], f[17], f[18], f[19], f[20], f[21]];
        let nurbs = if sizing.iter().all(|s| s.is_empty()) {
            None
        } else if sizing.iter().any(|s| s.is_empty()) {
            return Err(ParseError {
                line: n,
                text: "partially filled sizing columns".into(),
            });
        } else {
            Some(Nurbs {
                grid_cells: num(15, "grid_cells")?,
                patch_cells: num(16, "patch_cells")?,
                opt_cells: num(17, "opt_cells")?,
                span_cells: num(18, "span_cells")?,
                span_opt_cells: num(19, "span_opt_cells")?,
                worst_cert: num(20, "worst_cert")?,
                worst_dev: num(21, "worst_dev")?,
            })
        };
        rows.push(Row {
            scene: f[0].to_string(),
            face: idx(1, "face")?,
            chart: f[2].to_string(),
            delta: num(3, "delta")?,
            triangles: idx(4, "triangles")?,
            nurbs,
        });
    }
    Ok(rows)
}

/// One scene's totals — the unit the gate compares, because a face
/// ordinal is only meaningful within its body.
#[derive(Clone, Debug, Default, PartialEq)]
pub struct SceneTotals {
    /// Faces in the scene.
    pub faces: usize,
    /// Triangles over all its faces.
    pub triangles: usize,
    /// Triangles on Hessian-sized faces only.
    pub nurbs_triangles: usize,
    /// Grid cells the shipped (per-cell) sizing used, summed.
    pub grid_cells: f64,
    /// The whole-patch counterfactual's cells, summed.
    pub patch_cells: f64,
    /// Cheapest same-bound uniform grids, summed.
    pub opt_cells: f64,
    /// The meter's per-cell predictions, summed.
    pub span_cells: f64,
    /// Per-cell-sized grids at the cheapest split, summed.
    pub span_opt_cells: f64,
    /// Triangles on faces the sweep actually resampled.
    pub measured_triangles: usize,
    /// What those faces would cost if each were sized to the deviation
    /// it ATTAINED (`triangles · worst_dev / delta`, module docs — a
    /// first-order extrapolation, not a bound).
    pub extrapolated_triangles: f64,
}

impl SceneTotals {
    /// The scene's recoverable factor (the shipped grid against
    /// per-cell sizing at the cheapest split).
    pub fn recoverable(&self) -> f64 {
        ratio(self.grid_cells, self.span_opt_cells)
    }

    /// The scene's held span gain (the whole-patch counterfactual
    /// against the shipped grid).
    pub fn span_held(&self) -> f64 {
        ratio(self.patch_cells, self.grid_cells)
    }

    /// The scene's lane-vs-meter agreement (~1.0 by construction).
    pub fn agreement(&self) -> f64 {
        ratio(self.grid_cells, self.span_cells)
    }

    /// The scene's total slack: its resampled triangles against the
    /// extrapolation of what their attained deviation needed. `None`
    /// unless the sweep resampled.
    ///
    /// TRIANGLE-WEIGHTED, deliberately. The obvious alternative — the
    /// worst face's `delta / worst_dev` — is dominated by whichever
    /// face happens to be flattest, and a 110-triangle wall that is
    /// exactly planar reports an astronomical ratio while saying
    /// nothing about where the scene's mesh went.
    pub fn total_slack(&self) -> Option<f64> {
        #[allow(clippy::cast_precision_loss)]
        (self.measured_triangles > 0 && self.extrapolated_triangles > 0.0)
            .then(|| self.measured_triangles as f64 / self.extrapolated_triangles)
    }
}

/// Per-scene totals, in first-seen order (which is tour order — the
/// sweep writes rows as it walks the tour).
pub fn totals(rows: &[Row]) -> Vec<(String, SceneTotals)> {
    let mut order: Vec<String> = Vec::new();
    let mut map: std::collections::HashMap<String, SceneTotals> = std::collections::HashMap::new();
    for r in rows {
        let t = map.entry(r.scene.clone()).or_insert_with(|| {
            order.push(r.scene.clone());
            SceneTotals::default()
        });
        t.faces += 1;
        t.triangles += r.triangles;
        if let Some(n) = r.nurbs {
            t.nurbs_triangles += r.triangles;
            t.grid_cells += n.grid_cells;
            t.patch_cells += n.patch_cells;
            t.opt_cells += n.opt_cells;
            t.span_cells += n.span_cells;
            t.span_opt_cells += n.span_opt_cells;
        }
        if let Some(s) = r.total_slack() {
            #[allow(clippy::cast_precision_loss)]
            {
                t.measured_triangles += r.triangles;
                t.extrapolated_triangles += r.triangles as f64 / s;
            }
        }
    }
    order
        .into_iter()
        .map(|s| {
            let t = map.remove(&s).unwrap_or_default();
            (s, t)
        })
        .collect()
}

/// How much a scene's triangle count or a face's recoverable slack may
/// grow against the baseline before it is a finding.
///
/// 5%: the sweep is deterministic (D9 — same body, same δ, same mesh),
/// so a change of any size is real and zero tolerance would be
/// defensible. The margin exists for the honest small mover — a face
/// gaining one grid row because a trim box shifted in the last ulp —
/// not for noise, of which there is none.
pub const GROWTH_TOLERANCE: f64 = 1.05;

/// What kind of movement a finding reports.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Kind {
    /// A scene's triangle count grew.
    Triangles,
    /// A face's recoverable slack grew (the sizing got wastefuller).
    Slack,
    /// A baseline scene has no fresh row at all.
    Vanished,
}

/// One gate finding.
#[derive(Clone, Debug, PartialEq)]
pub struct Finding {
    /// What moved.
    pub kind: Kind,
    /// The scene it moved in.
    pub scene: String,
    /// The face, for [`Kind::Slack`].
    pub face: Option<usize>,
    /// Baseline value.
    pub was: f64,
    /// Fresh value.
    pub now: f64,
}

impl Finding {
    /// `now / was`, the growth factor (0 baseline reads as ∞).
    pub fn factor(&self) -> f64 {
        if self.was > 0.0 {
            self.now / self.was
        } else {
            f64::INFINITY
        }
    }
}

/// The gate: fresh against baseline, per the three rules in the module
/// docs. Scenes only in the fresh sweep are NOT findings — they are
/// new coverage; `main.rs` names them so the baseline's staleness is
/// visible.
pub fn compare(baseline: &[Row], fresh: &[Row]) -> Vec<Finding> {
    let mut findings = Vec::new();
    let base_totals = totals(baseline);
    let fresh_totals: std::collections::HashMap<String, SceneTotals> =
        totals(fresh).into_iter().collect();
    for (scene, was) in &base_totals {
        let Some(now) = fresh_totals.get(scene) else {
            findings.push(Finding {
                kind: Kind::Vanished,
                scene: scene.clone(),
                face: None,
                #[allow(clippy::cast_precision_loss)]
                was: was.triangles as f64,
                now: 0.0,
            });
            continue;
        };
        #[allow(clippy::cast_precision_loss)]
        let (w, n) = (was.triangles as f64, now.triangles as f64);
        if n > w * GROWTH_TOLERANCE {
            findings.push(Finding {
                kind: Kind::Triangles,
                scene: scene.clone(),
                face: None,
                was: w,
                now: n,
            });
        }
    }
    // Slack is per FACE: a scene total would let one face's regression
    // hide behind another's improvement.
    let key = |r: &Row| (r.scene.clone(), r.face);
    let fresh_faces: std::collections::HashMap<(String, usize), f64> = fresh
        .iter()
        .filter_map(|r| r.recoverable().map(|s| (key(r), s)))
        .collect();
    for r in baseline {
        let Some(was) = r.recoverable() else { continue };
        let Some(&now) = fresh_faces.get(&key(r)) else {
            continue; // the scene's absence is already a Vanished finding
        };
        if now > was * GROWTH_TOLERANCE {
            findings.push(Finding {
                kind: Kind::Slack,
                scene: r.scene.clone(),
                face: Some(r.face),
                was,
                now,
            });
        }
    }
    findings
}

#[cfg(test)]
mod tests {
    use super::*;

    /// A two-face fixture: one plane (empty sizing columns), one NURBS.
    fn csv(tris: usize, span_opt: f64) -> String {
        format!(
            "{EXPECTED_HEADER}\n\
             s/b,0,plane,2e-3,4,,,,,,,,,,,,,,,,,,\n\
             s/b,1,nurbs,2e-3,{tris},0e0,1e0,0e0,1e0,1e1,2e1,1e0,1e0,1e0,4,\
             1e2,2e2,5e1,1e2,{span_opt:e},1e-4,5e-5,99\n"
        )
    }

    #[test]
    fn parses_both_chart_shapes() {
        let rows = parse(&csv(100, 2.5e1)).unwrap();
        assert_eq!(rows.len(), 2);
        assert!(rows[0].nurbs.is_none(), "a plane row carries no sizing");
        let n = rows[1].nurbs.unwrap();
        assert!((n.grid_cells - 100.0).abs() < 1e-9);
        assert!((n.patch_cells - 200.0).abs() < 1e-9);
        // 200 / 100, 100 / 100, 100 / 25, and delta / worst_dev.
        assert!((rows[1].span_held().unwrap() - 2.0).abs() < 1e-9);
        assert!((rows[1].agreement().unwrap() - 1.0).abs() < 1e-9);
        assert!((rows[1].recoverable().unwrap() - 4.0).abs() < 1e-9);
        assert!((rows[1].total_slack().unwrap() - 40.0).abs() < 1e-9);
    }

    #[test]
    fn a_renamed_column_is_harness_breakage_not_a_finding() {
        let drifted = csv(100, 2.5e1).replacen("span_opt_cells", "span_best_cells", 1);
        let e = parse(&drifted).unwrap_err();
        assert_eq!(e.line, 1);
        assert!(e.text.contains("unexpected header"), "{}", e.text);
    }

    #[test]
    fn a_short_row_is_harness_breakage() {
        let e = parse(&format!("{EXPECTED_HEADER}\ns/b,0,plane,2e-3,4\n")).unwrap_err();
        assert_eq!(e.line, 2);
        assert!(e.text.contains("expected"), "{}", e.text);
    }

    #[test]
    fn an_unmoved_sweep_is_clean() {
        let base = parse(&csv(100, 2.5e1)).unwrap();
        assert_eq!(compare(&base, &base), Vec::new());
    }

    #[test]
    fn growth_inside_the_tolerance_is_not_a_finding() {
        let base = parse(&csv(100, 2.5e1)).unwrap();
        let fresh = parse(&csv(104, 2.5e1)).unwrap();
        assert_eq!(compare(&base, &fresh), Vec::new());
    }

    #[test]
    fn triangle_growth_fires() {
        let base = parse(&csv(100, 2.5e1)).unwrap();
        let fresh = parse(&csv(200, 2.5e1)).unwrap();
        let f = compare(&base, &fresh);
        assert_eq!(f.len(), 1);
        assert_eq!(f[0].kind, Kind::Triangles);
        // The plane's 4 triangles ride along in the scene total.
        assert!((f[0].factor() - 204.0 / 104.0).abs() < 1e-9);
    }

    /// The rule that a triangle count alone cannot express: the mesh
    /// got SMALLER while the sizing schedule got wastefuller.
    #[test]
    fn slack_growth_fires_even_as_the_mesh_shrinks() {
        let base = parse(&csv(100, 2.5e1)).unwrap();
        let fresh = parse(&csv(50, 1.0e1)).unwrap();
        let f = compare(&base, &fresh);
        assert_eq!(f.len(), 1, "{f:?}");
        assert_eq!(f[0].kind, Kind::Slack);
        assert_eq!(f[0].face, Some(1));
        assert!((f[0].was - 4.0).abs() < 1e-9 && (f[0].now - 10.0).abs() < 1e-9);
    }

    #[test]
    fn a_vanished_scene_is_a_finding_not_an_improvement() {
        let base = parse(&csv(100, 2.5e1)).unwrap();
        let fresh = parse(EXPECTED_HEADER).unwrap();
        let f = compare(&base, &fresh);
        assert_eq!(f.len(), 1);
        assert_eq!(f[0].kind, Kind::Vanished);
        assert_eq!(f[0].now, 0.0);
    }

    #[test]
    fn a_new_scene_is_not_a_finding() {
        let base = parse(EXPECTED_HEADER).unwrap();
        let fresh = parse(&csv(100, 2.5e1)).unwrap();
        assert_eq!(compare(&base, &fresh), Vec::new());
    }

    /// Sizing columns are all-or-nothing: a half-filled row means the
    /// sweep and the lint disagree about the schema.
    #[test]
    fn a_half_filled_sizing_row_is_harness_breakage() {
        let bad = format!(
            "{EXPECTED_HEADER}\n\
             s/b,1,nurbs,2e-3,9,0e0,1e0,0e0,1e0,1e1,2e1,1e0,1e0,1e0,4,1e2,,5e1,1e2,2.5e1,\
             1e-4,5e-5,99\n"
        );
        let e = parse(&bad).unwrap_err();
        assert!(e.text.contains("partially filled"), "{}", e.text);
    }

    #[test]
    fn a_sweep_without_deviation_has_no_total_slack() {
        let no_dev = csv(100, 2.5e1).replace(",1e-4,5e-5,99", ",1e-4,NaN,0");
        let rows = parse(&no_dev).unwrap();
        assert_eq!(rows[1].total_slack(), None);
        // …and the cell-count factors are unaffected: they never
        // needed the resampling pass.
        assert!((rows[1].recoverable().unwrap() - 4.0).abs() < 1e-9);
    }
}
