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

use tess_lint::{Kind, Row, SceneTotals, compare, parse, totals};

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
         \x20    the sweep stopped covering improves every total it used to appear in.\n"
    )
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
    let findings = compare(&base, &rows);
    let base_scenes: std::collections::HashSet<&str> =
        base.iter().map(|r| r.scene.as_str()).collect();
    let added: std::collections::BTreeSet<&str> = rows
        .iter()
        .map(|r| r.scene.as_str())
        .filter(|s| !base_scenes.contains(s))
        .collect();
    if !added.is_empty() {
        // Not a finding — but never silent: an uncovered scene is a
        // hole in the very comparison the gate is making.
        println!(
            "\ntess-lint: {} scene(s) in the fresh sweep are NOT in the baseline \
             (new coverage, not a finding): {}",
            added.len(),
            added.into_iter().collect::<Vec<_>>().join(", ")
        );
    }
    println!(
        "\ntess-lint: gate vs {baseline_path}: {} finding(s)",
        findings.len()
    );
    for f in &findings {
        match f.kind {
            Kind::Triangles => println!(
                "  FINDING {}: triangles {:.0} -> {:.0} ({:.2}x)",
                f.scene,
                f.was,
                f.now,
                f.factor()
            ),
            Kind::Slack => println!(
                "  FINDING {} face {}: recoverable slack {:.1}x -> {:.1}x — the sizing \
                 schedule got wastefuller",
                f.scene,
                f.face.unwrap_or_default(),
                f.was,
                f.now
            ),
            Kind::Vanished => println!(
                "  FINDING {}: in the baseline ({:.0} triangles), absent from this sweep",
                f.scene, f.was
            ),
        }
    }
    if !findings.is_empty() {
        // stderr, and stderr only: this verdict must survive a
        // redirected stdout — it is the reason the row is red.
        eprint!("{}", discipline(findings.len()));
        std::process::exit(EXIT_FINDINGS);
    }
    println!("tess-lint: clean — no scene grew and no face's sizing got wastefuller");
}
