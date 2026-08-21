//! The **tessellation-budget lint** (issue #320): reads the per-face
//! budget CSV that `mesh::budget` MEASURES and `tools/tess-meter`
//! writes (`demo-tour tess-budget`), and answers two different
//! questions with it.
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
//! number. What guards the schedule itself is the per-triangle
//! certificate refusal, this gate's growth rules, and the committed
//! render cells — no column reports the lane's realisation of the
//! schedule, and `docs/TESS-BUDGET.md` ("Why there is no realisation
//! column") says why that is deliberate rather than owed.
//!
//! * **held** = `patch_cells / grid_cells` — the span gain TESS-SPAN
//!   holds over whole-patch sizing. A regression toward whole-patch
//!   sizing drives it toward 1.0 (and fires the gate through
//!   `recoverable`, below).
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
//! `tess_meter`'s module docs and docs/TESS-BUDGET.md.
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
//! **A measurement that could not be read is none of the three, and
//! must not be resolved into one.** All three rules fire on GROWTH
//! only, so any in-band fallback for an unreadable value is the
//! smallest movement expressible and passes by construction. The
//! sizing columns are therefore admitted or refused where they are
//! read (`Admissible`, private), per column, and a refused one leaves in the
//! harness voice — a sweep the lint cannot read is not a tessellation
//! that got better.
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

/// The NURBS lane's sizing columns (`tess_meter::NurbsColumns`).
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
    /// Per-cell sizing at the cheapest split.
    pub span_opt_cells: f64,
    /// Worst per-triangle certificate the face emitted.
    pub worst_cert: f64,
    /// Worst SAMPLED deviation, `None` when the sweep did not
    /// resample. The CSV spells that `NaN`; the absence is kept in the
    /// type rather than in a float, so no arithmetic can read it as a
    /// small number.
    pub worst_dev: Option<f64>,
}

impl Row {
    /// `patch_cells / grid_cells` — the held span gain, or `None` off
    /// the Hessian-sized lane.
    ///
    /// A plain division, and it is [`parse`] that makes it one: no
    /// cell count below one or off the finite line is admitted, so
    /// there is no broken reading here to resolve into a number.
    pub fn span_held(&self) -> Option<f64> {
        self.nurbs.map(|n| n.patch_cells / n.grid_cells)
    }

    /// `grid_cells / span_opt_cells` — the recoverable slack (the
    /// gate's per-face ratio).
    pub fn recoverable(&self) -> Option<f64> {
        self.nurbs.map(|n| n.grid_cells / n.span_opt_cells)
    }

    /// `delta / worst_dev` — the unspent deviation budget, `None`
    /// unless the sweep resampled.
    ///
    /// **A resampled face that attained EXACTLY zero deviation is not
    /// an absence**, and folding it back into `None` would undo, in
    /// the first caller, the distinction [`Nurbs::worst_dev`]'s type
    /// exists to draw: it spent none of its budget, which is
    /// `f64::INFINITY`, and [`totals`] reads that as the zero
    /// triangles the extrapolation says such a face needs.
    pub fn total_slack(&self) -> Option<f64> {
        self.nurbs
            .and_then(|n| n.worst_dev)
            .map(|dev| self.delta / dev)
    }
}

/// What a measured column may say — the distinction, PER COLUMN,
/// between a measurement that is ABSENT and one that is merely small.
///
/// The gate fires only on GROWTH, so any in-band fallback for a value
/// that could not be read is the smallest slack a ratio can report and
/// is therefore a guaranteed pass: an instrument whose failure mode is
/// its own pass condition reports nothing. No broken value is resolved
/// into a reading here. It is refused at the parse boundary and leaves
/// through `main.rs`'s harness voice, the same exit a renamed column
/// gets, because it is the same kind of event — the sweep and the lint
/// disagreeing about what the file says.
///
/// Absence is a real state for exactly one measured column, and it has
/// its own spelling: `worst_dev` is `NaN` on every `--sizing-only`
/// sweep, which is the CI gate's own, so it parses to `None` rather
/// than to a number.
///
/// **A cell count is never absent, and the mechanism differs by
/// column** — worth stating, because the floor is what the rest of
/// this argument rests on. `patch_cells` and `opt_cells` are products
/// and minima of `tess_meter`'s `divisions`, which floors at one.
/// `grid_cells` is `Σ nuc·nvc` over the bands the lane actually ran,
/// and `mesh::sizing::ceil_count` floors each factor at one over at
/// least one band. `span_opt_cells` is an accumulator that starts at
/// zero and skips analysis cells outside the trim box — so its floor
/// is not arithmetic but geometric: the cell grid tiles the patch
/// domain and the trim box is a non-degenerate sub-box of it, so some
/// cell overlaps, and a face whose box is degenerate has no triangles
/// and no row. **A zero there would therefore be drift, and refusing
/// it is the point**: a loud harness failure naming the column is the
/// outcome to prefer if the geometric argument ever turns out to have
/// a case in it.
#[derive(Clone, Copy, Debug)]
enum Admissible {
    /// A grid cell count: finite, at least one.
    CellCount,
    /// A tessellation target: finite, above zero.
    Target,
    /// A certificate: finite and non-negative (zero is a face whose
    /// triangles are exact).
    Certificate,
    /// A sampled deviation: finite and non-negative, or `NaN` for "the
    /// sweep did not resample".
    OptionalDeviation,
}

impl Admissible {
    /// Whether `v` is a reading of this kind of column.
    fn admits(self, v: f64) -> bool {
        match self {
            Self::CellCount => v.is_finite() && v >= 1.0,
            Self::Target => v.is_finite() && v > 0.0,
            Self::Certificate => v.is_finite() && v >= 0.0,
            Self::OptionalDeviation => v.is_nan() || (v.is_finite() && v >= 0.0),
        }
    }

    /// What this column may say, for the harness message.
    fn expects(self) -> &'static str {
        match self {
            Self::CellCount => "a cell count, finite and at least one",
            Self::Target => "a tessellation target, finite and above zero",
            Self::Certificate => "a certificate, finite and non-negative",
            Self::OptionalDeviation => {
                "a deviation, finite and non-negative, or NaN for an unresampled sweep"
            }
        }
    }
}

/// Where the sizing block starts in [`EXPECTED_HEADER`].
const SIZING_FIRST: usize = 15;

/// The sizing block — every column [`Nurbs`] is parsed from, in
/// [`EXPECTED_HEADER`]'s order, with what each may say. One table
/// rather than six hand-written checks so that the block the parser
/// polices and the block the header declares can be compared to each
/// other, which this module's tests do: a seventh sizing column added
/// without an entry here would otherwise reach the gate unpoliced.
const SIZING_COLUMNS: [(&str, Admissible); 6] = [
    ("grid_cells", Admissible::CellCount),
    ("patch_cells", Admissible::CellCount),
    ("opt_cells", Admissible::CellCount),
    ("span_opt_cells", Admissible::CellCount),
    ("worst_cert", Admissible::Certificate),
    ("worst_dev", Admissible::OptionalDeviation),
];

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
/// `tess_meter::CSV_HEADER`. Pinned HERE as well as there on
/// purpose: the two halves are separate cargo roots by design, so
/// there is no shared constant to import, and a drifting sweep must
/// fail as harness breakage rather than parse into wrong columns.
pub const EXPECTED_HEADER: &str = "scene,face,chart,delta,triangles,u0,u1,v0,v1,nu,nv,\
                                   muu,muv,mvv,cells,grid_cells,patch_cells,opt_cells,\
                                   span_opt_cells,worst_cert,worst_dev,dev_samples";

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
        // The measured columns go through here, so that a broken
        // measurement is harness breakage rather than a reading (see
        // `Admissible`). The counted ones — `face`, `triangles` — go
        // through `idx` and cannot arrive non-finite or negative at
        // all.
        let admit = |col: usize, name: &str, kind: Admissible| -> Result<f64, ParseError> {
            let v = num(col, name)?;
            if kind.admits(v) {
                Ok(v)
            } else {
                Err(ParseError {
                    line: n,
                    text: format!("{name}: {v:e} is not {} (sweep drift?)", kind.expects()),
                })
            }
        };
        let idx = |col: usize, name: &str| -> Result<usize, ParseError> {
            f[col].parse::<usize>().map_err(|e| ParseError {
                line: n,
                text: format!("{name}: {e} ({:?})", f[col]),
            })
        };
        // The sizing columns are empty on every non-NURBS chart. All
        // present or all absent — a half-filled row is drift.
        let sizing: Vec<&str> = (0..SIZING_COLUMNS.len())
            .map(|k| f[SIZING_FIRST + k])
            .collect();
        let nurbs = if sizing.iter().all(|s| s.is_empty()) {
            None
        } else if sizing.iter().any(|s| s.is_empty()) {
            return Err(ParseError {
                line: n,
                text: "partially filled sizing columns".into(),
            });
        } else {
            let mut read = [0.0f64; SIZING_COLUMNS.len()];
            for (k, (name, kind)) in SIZING_COLUMNS.iter().enumerate() {
                read[k] = admit(SIZING_FIRST + k, name, *kind)?;
            }
            let [
                grid_cells,
                patch_cells,
                opt_cells,
                span_opt_cells,
                worst_cert,
                worst_dev,
            ] = read;
            Some(Nurbs {
                grid_cells,
                patch_cells,
                opt_cells,
                span_opt_cells,
                worst_cert,
                worst_dev: worst_dev.is_finite().then_some(worst_dev),
            })
        };
        rows.push(Row {
            scene: f[0].to_string(),
            face: idx(1, "face")?,
            chart: f[2].to_string(),
            delta: admit(3, "delta", Admissible::Target)?,
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
    /// per-cell sizing at the cheapest split), or `None` for a scene
    /// with no Hessian-sized face.
    ///
    /// A scene with no sizing has no sizing factor, and the sums say
    /// which case this is without a second counter: [`parse`] admits
    /// no cell count below one, so both are above zero exactly when
    /// some face contributed to them.
    pub fn recoverable(&self) -> Option<f64> {
        (self.span_opt_cells > 0.0).then(|| self.grid_cells / self.span_opt_cells)
    }

    /// The scene's held span gain (the whole-patch counterfactual
    /// against the shipped grid), `None` on a scene with no
    /// Hessian-sized face.
    pub fn span_held(&self) -> Option<f64> {
        (self.grid_cells > 0.0).then(|| self.patch_cells / self.grid_cells)
    }

    /// Adds one face's row.
    ///
    /// The ONE accumulator: [`totals`] folds a scene's rows through it
    /// and the CLI folds the whole sweep through it, so the two cannot
    /// disagree about what a total is or re-derive one of these
    /// factors by hand under a guard of its own.
    pub fn add(&mut self, r: &Row) {
        self.faces += 1;
        self.triangles += r.triangles;
        if let Some(n) = r.nurbs {
            self.nurbs_triangles += r.triangles;
            self.grid_cells += n.grid_cells;
            self.patch_cells += n.patch_cells;
            self.opt_cells += n.opt_cells;
            self.span_opt_cells += n.span_opt_cells;
        }
        if let Some(slack) = r.total_slack() {
            #[allow(clippy::cast_precision_loss)]
            {
                self.measured_triangles += r.triangles;
                self.extrapolated_triangles += r.triangles as f64 / slack;
            }
        }
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
        // `None` means "nothing was resampled", and only that. A scene
        // whose resampled faces were all exact extrapolates to zero
        // triangles and so reports INFINITE unspent budget — a reading,
        // not an absence, and the same distinction `Row::total_slack`
        // draws one level down.
        #[allow(clippy::cast_precision_loss)]
        (self.measured_triangles > 0)
            .then(|| self.measured_triangles as f64 / self.extrapolated_triangles)
    }
}

/// Per-scene totals, in first-seen order (which is tour order — the
/// sweep writes rows as it walks the tour).
pub fn totals(rows: &[Row]) -> Vec<(String, SceneTotals)> {
    let mut order: Vec<String> = Vec::new();
    let mut map: std::collections::HashMap<String, SceneTotals> = std::collections::HashMap::new();
    for r in rows {
        map.entry(r.scene.clone())
            .or_insert_with(|| {
                order.push(r.scene.clone());
                SceneTotals::default()
            })
            .add(r);
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
///
/// **Boxed from both sides by this module's tests, because the
/// tempting move on a red gate is to widen it.** A scene 4% larger
/// must stay clean and a scene 6% larger must fire, and the same pair
/// is asserted on the slack rule — so the constant cannot leave
/// `[1.04, 1.06)` without a test going red, on either rule, whether it
/// is widened or split in two. Widening it then costs a diff that says
/// so, which is the difference between a threshold and a knob.
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
            // The comment below holds only when the whole scene is
            // gone. This join is POSITIONAL, so a face ordinal that
            // merely moved drops its face out of the comparison in
            // silence, and `Vanished` is scene-granular. Filed as
            // issue #746 and deliberately not closed here.
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
    ///
    /// Twinned in `tests/cli_contract.rs`, deliberately: an
    /// integration test cannot see a `#[cfg(test)]` item, so the two
    /// cannot share one. Keep them in step.
    fn csv(tris: usize, span_opt: f64) -> String {
        format!(
            "{EXPECTED_HEADER}\n\
             s/b,0,plane,2e-3,4,,,,,,,,,,,,,,,,,\n\
             s/b,1,nurbs,2e-3,{tris},0e0,1e0,0e0,1e0,1e1,2e1,1e0,1e0,1e0,4,\
             1e2,2e2,5e1,{span_opt:e},1e-4,5e-5,99\n"
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
        // 200 / 100, 100 / 25, and delta / worst_dev.
        assert!((rows[1].span_held().unwrap() - 2.0).abs() < 1e-9);
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
             s/b,1,nurbs,2e-3,9,0e0,1e0,0e0,1e0,1e1,2e1,1e0,1e0,1e0,4,1e2,,5e1,2.5e1,\
             1e-4,5e-5,99\n"
        );
        let e = parse(&bad).unwrap_err();
        assert!(e.text.contains("partially filled"), "{}", e.text);
    }

    /// Sets one ABSOLUTE column of the fixture's Hessian-sized row.
    ///
    /// The one way this module breaks a fixture: positional, so a test
    /// says which column it is breaking rather than which byte
    /// sequence happens to spell it — a `replace` on a literal is
    /// coupled to the caller's arguments and silently does nothing
    /// when they change.
    fn with_field(text: &str, col: usize, value: &str) -> String {
        let mut lines: Vec<String> = text.lines().map(str::to_string).collect();
        let mut f: Vec<String> = lines[2].split(',').map(str::to_string).collect();
        f[col] = value.to_string();
        lines[2] = f.join(",");
        format!("{}\n", lines.join("\n"))
    }

    /// [`with_field`] addressed within the sizing block.
    fn with_column(k: usize, value: &str) -> String {
        with_field(&csv(100, 2.5e1), SIZING_FIRST + k, value)
    }

    /// The shape of the sweep CI actually gates on: `--sizing-only`
    /// resamples nothing, so `worst_dev` is `NaN` and `dev_samples` is
    /// zero.
    fn sizing_only(tris: usize, span_opt: f64) -> Vec<Row> {
        let text = with_field(&csv(tris, span_opt), SIZING_FIRST + 5, "NaN");
        let text = with_field(&text, SIZING_FIRST + SIZING_COLUMNS.len(), "0");
        parse(&text).unwrap()
    }

    /// The question this parser exists to answer. Every rule fires on
    /// GROWTH, so a broken value resolved into a reading is the
    /// smallest movement expressible and passes by construction — the
    /// instrument's failure mode would be its own pass condition. So
    /// every column a ratio touches refuses one.
    ///
    /// The expectations are written out rather than derived from
    /// [`SIZING_COLUMNS`]: a test that reads the policy it is checking
    /// asserts nothing. The array's width is the guard against the
    /// NEXT column — a seventh entry in the table with no row here
    /// does not compile, so a column cannot arrive unpoliced by being
    /// added quietly.
    #[test]
    fn every_sizing_column_refuses_the_values_that_would_read_as_a_pass() {
        // `5e-1` is here because the other four cannot tell a CELL
        // COUNT from any other positive-finite policy, and the count's
        // floor of one is what the argument above rests on: without a
        // fractional row, relaxing `CellCount` to "finite and above
        // zero" passes this whole suite.
        const BAD: [&str; 5] = ["0e0", "-1e0", "inf", "NaN", "5e-1"];
        const ADMITTED: [(&str, [bool; 5]); SIZING_COLUMNS.len()] = [
            ("grid_cells", [false, false, false, false, false]),
            ("patch_cells", [false, false, false, false, false]),
            ("opt_cells", [false, false, false, false, false]),
            ("span_opt_cells", [false, false, false, false, false]),
            // A face whose triangles are exact certifies at zero, and
            // a certificate is a length, not a count.
            ("worst_cert", [true, false, false, false, true]),
            // The one absence with a spelling: NaN is "not resampled".
            ("worst_dev", [true, false, false, true, true]),
        ];
        for (k, (name, admitted)) in ADMITTED.iter().enumerate() {
            assert_eq!(*name, SIZING_COLUMNS[k].0, "column {k} of the table");
            for (b, bad) in BAD.iter().enumerate() {
                let got = parse(&with_column(k, bad)).is_ok();
                assert_eq!(got, admitted[b], "{name} = {bad}: admitted = {got}");
            }
        }
    }

    /// The block the parser polices is the header's own, bracketed on
    /// both sides: a column inserted into the sizing run would slide
    /// every measurement under the wrong policy, and the drifting
    /// header this file already refuses is the same failure one step
    /// earlier.
    #[test]
    fn the_policed_block_is_the_headers_sizing_block() {
        let cols: Vec<&str> = EXPECTED_HEADER.split(',').collect();
        assert_eq!(cols[SIZING_FIRST - 1], "cells", "the block starts too late");
        for (k, (name, _)) in SIZING_COLUMNS.iter().enumerate() {
            assert_eq!(cols[SIZING_FIRST + k], *name, "column {k}");
        }
        assert_eq!(
            cols[SIZING_FIRST + SIZING_COLUMNS.len()],
            "dev_samples",
            "the block ends too early"
        );
    }

    /// A denominator that could not be read is harness breakage, and
    /// the message says which column — the same voice a renamed column
    /// gets, because it is the same event.
    #[test]
    fn an_unreadable_denominator_is_harness_breakage() {
        let e = parse(&with_column(3, "0e0")).unwrap_err();
        assert_eq!(e.line, 3);
        assert!(e.text.contains("span_opt_cells"), "{}", e.text);
        assert!(e.text.contains("cell count"), "{}", e.text);
    }

    /// The positive control the refusal above is worthless without: a
    /// denominator that genuinely collapses is a real measurement and
    /// FIRES. The pair is the whole point — the gate reads a real
    /// collapse and refuses an unreadable one, and neither of them is
    /// a face that improved.
    #[test]
    fn a_collapsed_denominator_fires_rather_than_passing() {
        let base = parse(&csv(100, 2.5e1)).unwrap();
        let fresh = parse(&csv(100, 1.0)).unwrap();
        let f = compare(&base, &fresh);
        assert_eq!(f.len(), 1, "{f:?}");
        assert_eq!(f[0].kind, Kind::Slack);
        assert!((f[0].now - 100.0).abs() < 1e-9);
    }

    /// The absence that is NOT breakage, and the reason the refusals
    /// above are per column: `worst_dev` is `NaN` on every
    /// `--sizing-only` sweep, which is the sweep CI gates on. It
    /// parses, it reports no total slack, and both cell-count rules
    /// still run over it.
    #[test]
    fn a_sizing_only_sweep_still_gates() {
        let base = sizing_only(100, 2.5e1);
        assert_eq!(base[1].nurbs.unwrap().worst_dev, None);
        assert_eq!(base[1].total_slack(), None);
        assert_eq!(compare(&base, &sizing_only(100, 2.5e1)), Vec::new());
        let f = compare(&base, &sizing_only(200, 2.5e1));
        assert_eq!(f.len(), 1, "{f:?}");
        assert_eq!(f[0].kind, Kind::Triangles);
        // The SLACK rule is the one this finding is about, and an
        // equality-to-empty cannot say it still runs: a `recoverable`
        // that went absent along with `worst_dev` would satisfy every
        // line above.
        assert_eq!(base[1].recoverable(), Some(4.0));
        let f = compare(&base, &sizing_only(100, 1.0e1));
        assert_eq!(f.len(), 1, "{f:?}");
        assert_eq!(f[0].kind, Kind::Slack);
    }

    /// A scene with no Hessian-sized face has no sizing factor and
    /// says so. Reporting 1.0 would be a reading of a grid nobody
    /// sized, in the column where 1.0 means "as good as it gets".
    #[test]
    fn a_scene_with_no_sized_face_reports_no_factor() {
        let planes = parse(&format!(
            "{EXPECTED_HEADER}\ns/b,0,plane,2e-3,4,,,,,,,,,,,,,,,,,\n"
        ))
        .unwrap();
        let t = &totals(&planes)[0].1;
        assert_eq!(t.recoverable(), None);
        assert_eq!(t.span_held(), None);
    }

    /// `delta` is admitted for the same reason the cell counts are,
    /// and the reason is one level downstream: [`totals`] divides a
    /// face's triangles by its `total_slack` = `delta / worst_dev`, so
    /// a zero or non-finite δ extrapolates every resampled face to
    /// zero triangles and the scene's total column then reports
    /// **absent**. A broken value manufacturing an absence is this
    /// finding's own shape with the sign flipped, and the report is
    /// where it would be read.
    #[test]
    fn a_broken_delta_is_harness_breakage() {
        for bad in ["0e0", "-2e-3", "NaN", "inf"] {
            let e = parse(&with_field(&csv(100, 2.5e1), 3, bad)).unwrap_err();
            assert!(e.text.contains("delta"), "{bad}: {}", e.text);
            assert!(e.text.contains("tessellation target"), "{bad}: {}", e.text);
        }
        assert!(parse(&with_field(&csv(100, 2.5e1), 3, "2e-3")).is_ok());
    }

    /// A resampled face that attained EXACTLY zero deviation spent
    /// none of its budget: infinite slack, and a reading. `None` still
    /// means "not resampled" and only that — collapsing the two is
    /// what the `Option` exists to prevent, and the first caller is
    /// where that collapse would happen.
    #[test]
    fn an_exact_face_reports_infinite_slack_not_an_absence() {
        let rows = parse(&with_column(5, "0e0")).unwrap();
        assert_eq!(rows[1].nurbs.unwrap().worst_dev, Some(0.0));
        assert_eq!(rows[1].total_slack(), Some(f64::INFINITY));
        assert_eq!(totals(&rows)[0].1.total_slack(), Some(f64::INFINITY));
    }

    /// [`GROWTH_TOLERANCE`] boxed from BELOW on the triangle rule: a
    /// scene exactly 4% larger (96 + 4 planar = 100 against 104) stays
    /// clean, so the constant cannot be cut under 1.04.
    #[test]
    fn a_four_percent_scene_is_inside_the_tolerance() {
        let base = parse(&csv(96, 2.5e1)).unwrap();
        let fresh = parse(&csv(100, 2.5e1)).unwrap();
        assert_eq!(compare(&base, &fresh), Vec::new());
    }

    /// …and from ABOVE: 6% is a finding, so the constant cannot be
    /// widened to 1.06. This is the side that matters — the move a red
    /// gate tempts is to raise the tolerance until it goes quiet, and
    /// the pair leaves a 2-point window to raise it into.
    #[test]
    fn a_six_percent_scene_is_a_finding() {
        let base = parse(&csv(96, 2.5e1)).unwrap();
        let fresh = parse(&csv(102, 2.5e1)).unwrap();
        let f = compare(&base, &fresh);
        assert_eq!(f.len(), 1, "{f:?}");
        assert_eq!(f[0].kind, Kind::Triangles);
    }

    /// The same box on the SLACK rule, which shares the constant: the
    /// ratio of the two baselines' `span_opt_cells` is the growth, so
    /// 26 → 25 is exactly 1.04 and stays clean.
    #[test]
    fn a_four_percent_slack_growth_is_inside_the_tolerance() {
        let base = parse(&csv(100, 2.6e1)).unwrap();
        let fresh = parse(&csv(100, 2.5e1)).unwrap();
        assert_eq!(compare(&base, &fresh), Vec::new());
    }

    /// …and 26.5 → 25 is exactly 1.06 and fires. Boxing both rules
    /// rather than one keeps the box intact if the constant is ever
    /// split in two: a second threshold with no box would red here.
    #[test]
    fn a_six_percent_slack_growth_is_a_finding() {
        let base = parse(&csv(100, 2.65e1)).unwrap();
        let fresh = parse(&csv(100, 2.5e1)).unwrap();
        let f = compare(&base, &fresh);
        assert_eq!(f.len(), 1, "{f:?}");
        assert_eq!(f[0].kind, Kind::Slack);
    }

    #[test]
    fn a_sweep_without_deviation_has_no_total_slack() {
        let rows = sizing_only(100, 2.5e1);
        assert_eq!(rows[1].total_slack(), None);
        // …and the cell-count factors are unaffected: they never
        // needed the resampling pass.
        assert!((rows[1].recoverable().unwrap() - 4.0).abs() < 1e-9);
    }
}
