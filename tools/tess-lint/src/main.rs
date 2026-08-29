//! The tessellation-budget lint CLI (issue #320):
//! `tess-lint <fresh.csv> [--baseline <base.csv>] [--top N]`.
//!
//! Two modes, one tool:
//!
//! * **report** (no `--baseline`) — prints where the mesh goes and how
//!   much of it the deviation budget needed. Always exit 0: a report
//!   is not a verdict.
//! * **gate** (`--baseline`) — additionally compares against a
//!   committed sweep; findings exit [`EXIT_FINDINGS`].
//!
//! Harness breakage (no input, unreadable file, malformed CSV) exits
//! [`EXIT_HARNESS`] in its own voice — `k-lint`'s three-voice split,
//! and for its reason: a sweep-format drift must never read as a
//! geometry finding.

use tess_lint::{Kind, Observation, Rekey, Row, SceneTotals, compare, parse, totals};

/// The gate ran and the budget distribution moved.
const EXIT_FINDINGS: i32 = 2;

/// The lint could not run: no inputs, unreadable file, malformed CSV.
const EXIT_HARNESS: i32 = 1;

/// Reads a sweep or exits in the harness voice.
fn read(path: &str) -> Vec<Row> {
    let text = std::fs::read_to_string(path).unwrap_or_else(|e| {
        eprintln!("tess-lint: cannot read {path}: {e}");
        std::process::exit(EXIT_HARNESS);
    });
    parse(&text).unwrap_or_else(|e| {
        eprintln!(
            "tess-lint: {path}:{}: malformed budget row (harness breakage): {}",
            e.line, e.text
        );
        std::process::exit(EXIT_HARNESS);
    })
}

/// The failure message. Leads with the interpretation discipline, as
/// `k-lint`'s does, because the tempting wrong move is a real one:
/// this lint's numbers can be made to fall by coarsening δ or by
/// simplifying the geometry, and either destroys the evidence.
fn discipline(findings: usize) -> String {
    format!(
        "\ntess-lint: GATE FAILED — the tessellation budget moved: {findings} finding(s).\n\
         \n\
         A fired gate is evidence ABOUT THE BUDGET DISTRIBUTION. Do NOT coarsen delta\n\
         and do NOT simplify a demo's geometry to get the number down — both destroy\n\
         exactly the measurement this gate exists to keep.\n\
         \n\
         Recourse, in order:\n\
         \x20 1. Find what changed. A triangle-count growth with unchanged slack is a\n\
         \x20    GEOMETRY change (a scene got more curved); a slack growth is a SIZING\n\
         \x20    change (the schedule got wastefuller) and belongs to mesh::budget's\n\
         \x20    lane, not to the scene.\n\
         \x20 2. If the growth is intended, re-cut the baseline with\n\
         \x20    `demo-tour tess-budget <out.csv>` and say WHY in the commit — the\n\
         \x20    baseline is a record of a deliberate state, not a high-water mark.\n\
         \x20 3. A `vanished` finding is never re-baselined without reading it: a scene\n\
         \x20    the sweep stopped covering improves every total it used to appear in.\n\
         \x20 4. A re-keyed face is the join refusing to call one ordinal one face.\n\
         \x20    The line names the face, the column that disagreed and both\n\
         \x20    readings; that face and every face above it went uncompared, which\n\
         \x20    is what makes it a finding. Establish what changed in the MODEL\n\
         \x20    first — a face genuinely replaced is a geometry change, a face\n\
         \x20    merely renumbered is not — because a re-cut taken before that\n\
         \x20    reading commits whatever the uncompared faces were doing.\n"
    )
}

/// One observation, as a line. The prefix says whether it fails the
/// row, so a note and a finding can never be read for each other.
fn line(prefix: &str, o: &Observation) -> String {
    let scene = &o.scene;
    match &o.kind {
        Kind::Triangles { was, now } => {
            let factor = if *was > 0.0 { now / was } else { f64::INFINITY };
            format!("{prefix} {scene}: triangles {was:.0} -> {now:.0} ({factor:.2}x)")
        }
        Kind::Slack { face, was, now } => format!(
            "{prefix} {scene} face {face}: recoverable slack {was:.1}x -> {now:.1}x — the \
             sizing schedule got wastefuller"
        ),
        Kind::Vanished { was_triangles } => format!(
            "{prefix} {scene}: in the baseline ({was_triangles:.0} triangles), absent from \
             this sweep"
        ),
        // Never "the sizing got wastefuller" and never a face count:
        // this is the join refusing to call two rows one face, so the
        // line is the column that disagreed and both its readings.
        Kind::Rekeyed { face, how } => {
            let what = match how {
                Rekey::Absent { in_baseline: true } => {
                    "in the baseline, absent from this sweep".to_string()
                }
                Rekey::Absent { in_baseline: false } => {
                    "in this sweep, absent from the baseline".to_string()
                }
                Rekey::Column { name, was, now } => {
                    format!("a different face: {name} {was} -> {now}")
                }
            };
            format!(
                "{prefix} {scene} face {face}: {what} — the per-face join is by ORDINAL, so \
                 this face and every face above it went uncompared"
            )
        }
        Kind::NewScene { triangles } => format!(
            "{prefix} {scene}: in this sweep ({triangles:.0} triangles), not in the baseline \
             — new coverage, so no face in it was compared against anything"
        ),
    }
}

fn main() {
    let args: Vec<String> = std::env::args().skip(1).collect();
    let mut fresh_path: Option<String> = None;
    let mut baseline_path: Option<String> = None;
    let mut top = 12usize;
    let mut i = 0;
    while i < args.len() {
        match args[i].as_str() {
            "--baseline" | "--top" => {
                let Some(v) = args.get(i + 1) else {
                    eprintln!("tess-lint: {} needs a value", args[i]);
                    std::process::exit(EXIT_HARNESS);
                };
                if args[i] == "--baseline" {
                    baseline_path = Some(v.clone());
                } else {
                    top = v.parse().unwrap_or_else(|e| {
                        eprintln!("tess-lint: --top {v}: {e}");
                        std::process::exit(EXIT_HARNESS);
                    });
                }
                i += 2;
            }
            other if other.starts_with("--") => {
                eprintln!("tess-lint: unknown flag {other}");
                std::process::exit(EXIT_HARNESS);
            }
            other => {
                fresh_path = Some(other.to_string());
                i += 1;
            }
        }
    }
    let Some(fresh_path) = fresh_path else {
        eprintln!(
            "tess-lint: usage: tess-lint <budget-csv> [--baseline <csv>] [--top N]\n\
             \x20 produce the CSV with: cd demos/tour && cargo run --release -- \\\n\
             \x20   tess-budget <out.csv> [--deviation]"
        );
        std::process::exit(EXIT_HARNESS);
    };
    let rows = read(&fresh_path);

    // --- report -------------------------------------------------
    let faces = rows.len();
    let tris: usize = rows.iter().map(|r| r.triangles).sum();
    let nurbs: Vec<&Row> = rows.iter().filter(|r| r.nurbs.is_some()).collect();
    let ntris: usize = nurbs.iter().map(|r| r.triangles).sum();
    println!("tess-lint: {fresh_path}: {faces} faces, {tris} triangles");
    #[allow(clippy::cast_precision_loss)]
    if faces > 0 && tris > 0 {
        println!(
            "  Hessian-sized (NURBS) faces: {} ({:.1}% of faces) carrying {ntris} triangles \
             ({:.1}% of the mesh)",
            nurbs.len(),
            100.0 * nurbs.len() as f64 / faces as f64,
            100.0 * ntris as f64 / tris as f64
        );
    }
    // Totals over the whole sweep: what the shipped per-cell schedule
    // holds, and what the same certificates would still allow. Cell
    // counts, not triangles — the triangle count of a trimmed face is
    // not its grid.
    //
    // Through `SceneTotals`, deliberately: it is the same fold the
    // per-scene table uses and it answers `None` where there is
    // nothing to divide, so this report has one spelling of a total
    // rather than a second one guarded by a comment.
    let mut sweep = SceneTotals::default();
    for r in &rows {
        sweep.add(r);
    }
    if let (Some(held), Some(recoverable)) = (sweep.span_held(), sweep.recoverable()) {
        println!(
            "  grid cells over all Hessian-sized faces: {:.0} used (per-knot-span-cell, \
             TESS-SPAN); whole-patch counterfactual {:.0} ({held:.1}x held), {:.0} at the \
             cheapest split per cell ({recoverable:.1}x still recoverable)",
            sweep.grid_cells, sweep.patch_cells, sweep.span_opt_cells
        );
        println!(
            "  every one of those grids satisfies the SAME per-triangle certificate the \
             shipped lane checks — this is sizing slack, not tolerance slack"
        );
    }
    // The constraint-activity indicator (TESS-SPLIT): which constraint
    // bound the schedule where the split ratio sits above 1.0, and the
    // worst realized lattice aspect (reported; the sliver-safe line is
    // read at mesh::nurbs_cert::SAFE_ASPECT, never from a copy here).
    {
        let (mut bands, mut cap, mut snap) = (0.0f64, 0.0f64, 0.0f64);
        let mut worst: Option<(f64, &Row)> = None;
        for r in &nurbs {
            if let Some(n) = r.nurbs {
                bands += n.bands;
                cap += n.cap_bands;
                snap += n.snap_bands;
                if worst.is_none_or(|(a, _)| n.realized_aspect > a) {
                    worst = Some((n.realized_aspect, r));
                }
            }
        }
        if let Some((aspect, r)) = worst {
            println!(
                "  constraint activity: {cap:.0} A-cap-bound band(s), {snap:.0} snap-projected \
                 band(s) of {bands:.0}; worst realized s_u/s_v {aspect:.2} ({} face {})",
                r.scene, r.face
            );
        }
    }
    let scenes = totals(&rows);
    let mut ranked: Vec<_> = scenes
        .iter()
        .filter(|(_, t)| t.nurbs_triangles > 0)
        .collect();
    ranked.sort_by_key(|(_, t)| std::cmp::Reverse(t.triangles));
    if !ranked.is_empty() {
        println!(
            "\n  {:<34} {:>9} {:>9} {:>7} {:>7} {:>8}",
            "scene (Hessian-sized faces)", "tris", "nurbs", "held", "split", "total"
        );
        for (scene, t) in ranked.iter().take(top) {
            // A column with nothing behind it prints as nothing: the
            // report never spells an absent measurement as a number,
            // for the same reason the gate refuses to read one.
            let factor = |v: Option<f64>| v.map_or_else(|| "-".to_string(), |x| format!("{x:.1}x"));
            println!(
                "  {scene:<34} {:>9} {:>9} {:>7} {:>7} {:>8}",
                t.triangles,
                t.nurbs_triangles,
                factor(t.span_held()),
                factor(t.recoverable()),
                factor(t.total_slack())
            );
        }
        if ranked.len() > top {
            println!("  … {} more scenes (--top {})", ranked.len() - top, top);
        }
        println!(
            "\n  held = the whole-patch-sup counterfactual against the shipped per-cell grid \
             (the TESS-SPAN gain);\n  split = what a cheaper split point per cell \
             would still recover (a strip-shaped upper bound);\n  total = triangles against \
             what their ATTAINED deviation needed (an estimate: a sampled sup,\n  extrapolated \
             through deviation ~ h^2 — the others are counted grids)"
        );
    }

    // --- gate ---------------------------------------------------
    let Some(baseline_path) = baseline_path else {
        println!("\ntess-lint: report only (no --baseline) — no gate ran");
        return;
    };
    let base = read(&baseline_path);
    let report = compare(&base, &rows);
    // Notes first, and on stdout only: they are coverage the gate did
    // not get to compare, and they never make the row red.
    for note in &report.notes {
        println!("{}", line("\nnote:", note));
    }
    println!(
        "\ntess-lint: gate vs {baseline_path}: {} finding(s)",
        report.findings.len()
    );
    for f in &report.findings {
        println!("{}", line("  FINDING", f));
    }
    if !report.findings.is_empty() {
        // stderr, and stderr only: this verdict must survive a
        // redirected stdout — it is the reason the row is red.
        eprint!("{}", discipline(report.findings.len()));
        std::process::exit(EXIT_FINDINGS);
    }
    println!("tess-lint: clean — no scene grew and no face's sizing got wastefuller");
}
