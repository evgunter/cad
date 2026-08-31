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
//! Harness breakage (no input, unreadable file, malformed CSV, an
//! unreadable `# tess-budget-cut:` line) exits [`EXIT_HARNESS`] in its
//! own voice — `k-lint`'s three-voice split, and for its reason: a
//! sweep-format drift must never read as a geometry finding.
//!
//! **Three EXIT voices, five finding KINDS, and the two do not line
//! up one to one.** Rule 5 — a scene the baseline has no rows for — is
//! the case that makes the distinction worth stating: it speaks in the
//! harness-breakage register, because nothing about that scene's
//! budget was read, and it still exits [`EXIT_FINDINGS`], because the
//! sweep and the lint agree perfectly about the format and the thing
//! that is missing is a REFERENCE, which the author supplies by
//! folding. Reading it as exit 1 would file corpus growth as a broken
//! instrument.

use tess_lint::{Cut, Kind, Observation, Rekey, Row, SceneTotals, compare, cut, parse, totals};

/// The gate ran and the budget distribution moved.
const EXIT_FINDINGS: i32 = 2;

/// The lint could not run: no inputs, unreadable file, malformed CSV.
const EXIT_HARNESS: i32 = 1;

/// Reads a sweep and the tree it was cut from, or exits in the
/// harness voice.
fn read(path: &str) -> (Vec<Row>, Option<Cut>) {
    let text = std::fs::read_to_string(path).unwrap_or_else(|e| {
        eprintln!("tess-lint: cannot read {path}: {e}");
        std::process::exit(EXIT_HARNESS);
    });
    let bail = |e: tess_lint::ParseError| -> ! {
        eprintln!(
            "tess-lint: {path}:{}: malformed budget row (harness breakage): {}",
            e.line, e.text
        );
        std::process::exit(EXIT_HARNESS);
    };
    let cut = cut(&text).unwrap_or_else(|e| bail(e));
    (parse(&text).unwrap_or_else(|e| bail(e)), cut)
}

/// How the gate names the baseline's cut. An absent one is SAID, not
/// left blank: rule 5's two readings are told apart by this line, so a
/// baseline that records none has to announce that it cannot tell them
/// apart.
fn provenance(cut: Option<&Cut>) -> String {
    cut.map_or_else(
        || {
            "no recorded cut — re-cut with scripts/tess_budget_sweep.sh, which records one"
                .to_string()
        },
        |c| format!("cut at {c}"),
    )
}

/// The failure message. Leads with the interpretation discipline, as
/// `k-lint`'s does, because the tempting wrong move is a real one:
/// this lint's numbers can be made to fall by coarsening δ or by
/// simplifying the geometry, and either destroys the evidence.
///
/// WHICH discipline it leads with is decided by what fired, because
/// the two are not the same kind of event. A budget that MOVED is a
/// measurement, and the tempting wrong move is to move it back. A
/// scene the baseline does not cover is a comparison that never
/// happened, and telling its author not to coarsen delta would be
/// advice about a number nobody read. **Both print when both fire**,
/// in that order; neither is an `else` for the other, because a sweep
/// can easily carry one of each and dropping either lead would leave
/// half the findings unaddressed.
///
/// **The split is by RECOURSE, not by whether a comparison happened**,
/// and rule 3 is where that distinction earns its keep. A vanished
/// scene is also a stopped comparison, and it leads with the
/// measurement discipline anyway — because what its author must do is
/// read WHY the scene left before touching the baseline, which is the
/// same "do not move the number back" instruction one level up, and is
/// the opposite of rule 5's *"Nothing about the SCENE is at fault"*.
/// Rule 4 sits with rule 3 for the same reason: recourse item 4 says
/// establish what changed in the MODEL first. Rule 5 is alone on its
/// lead because it is the only finding here whose fix is mechanical.
fn discipline(findings: &[Observation], cut: Option<&Cut>) -> String {
    let provenance = provenance(cut);
    let uncovered = findings
        .iter()
        .filter(|o| matches!(o.kind, Kind::Uncovered { .. }))
        .count();
    let mut lead = String::new();
    if uncovered > 0 {
        lead.push_str(&format!(
            "The gate could not COMPARE {uncovered} scene(s): this sweep has them and the\n\
             baseline has no rows for them, so not one of their faces was measured\n\
             against anything. That is the comparison breaking, not the budget moving —\n\
             the fix is to fold those scenes into the baseline (5, below), and it belongs\n\
             in the PR that grew the corpus. Nothing about the SCENE is at fault.\n\
             \n"
        ));
    }
    if findings.len() > uncovered {
        lead.push_str(
            "A fired gate is evidence ABOUT THE BUDGET DISTRIBUTION. Do NOT coarsen delta\n\
             and do NOT simplify a demo's geometry to get the number down — both destroy\n\
             exactly the measurement this gate exists to keep.\n\
             \n",
        );
    }
    let count = findings.len();
    format!(
        "\ntess-lint: GATE FAILED — {count} finding(s).\n\
         \n\
         {lead}\
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
         \x20    readings; that face and every face above it went uncompared. It is\n\
         \x20    a FINDING because the scene carries a Hessian-sized face, so the\n\
         \x20    slack rule lost comparisons it would otherwise have made; the same\n\
         \x20    event in a scene with no sized face is printed as a `note:` above\n\
         \x20    and exits 0, because rule 1 still runs over that scene's total.\n\
         \x20    Establish what changed in the MODEL first — a face genuinely\n\
         \x20    replaced is a geometry change, a face merely renumbered is not —\n\
         \x20    because a re-cut taken before that reading commits whatever the\n\
         \x20    uncompared faces were doing.\n\
         \x20 5. An `uncovered` finding is a scene the baseline has no rows for, so\n\
         \x20    the gate could not compare a single face in it. This is the one\n\
         \x20    finding whose fix is mechanical, and it belongs in the PR that grew\n\
         \x20    the corpus:\n\
         \x20      a. scripts/tess_budget_sweep.sh \\\n\
         \x20           docs/tess-budget-data/tess-budget-baseline.csv\n\
         \x20      b. check the diff is ADDITIVE — new rows only. A row that MOVED is\n\
         \x20         a separate finding this one was hiding, and it is read, not\n\
         \x20         folded.\n\
         \x20      c. commit the baseline with the scene, saying what the scene is.\n\
         \x20    The baseline you are folding into: {provenance}.\n\
         \x20    A scene older than that cut has been outside the gate ever since —\n\
         \x20    swept, measured and compared against nothing — and the fold buys\n\
         \x20    comparison FROM NOW ON only. It cannot audit the window, so the\n\
         \x20    values it blesses are current-state, not verified-optimal —\n\
         \x20    docs/TESS-BUDGET.md, `restores coverage, it does not verify it`.\n"
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
        // The harness voice, not a measurement's: nothing about this
        // scene's budget was READ, so the line reports a comparison
        // that did not happen rather than a number that moved.
        Kind::Uncovered { triangles } => format!(
            "{prefix} {scene}: in this sweep ({triangles:.0} triangles), not in the baseline \
             — the gate cannot compare what the baseline lacks, so no face in it was \
             compared against anything. Fold it: re-run the sweep into the baseline, check \
             the diff is additive, commit it with the scene"
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
    let (rows, _) = read(&fresh_path);

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
    let (base, base_cut) = read(&baseline_path);
    let report = compare(&base, &rows);
    // Notes first, and on stdout only: they are comparisons the gate
    // did not make where it had nothing to lose by not making them,
    // and they never redden the row. COUNTED, because an uncounted
    // channel is where findings go to be forgotten.
    if !report.notes.is_empty() {
        println!(
            "\ntess-lint: {} note(s) — a comparison the gate did not make where it had none to \
             lose, not a finding:",
            report.notes.len()
        );
        for note in &report.notes {
            println!("{}", line(" ", note));
        }
    }
    // The baseline's cut rides with every gate line, clean or not: it
    // is the reference point every finding below is measured from, and
    // for rule 5 it is what separates a scene added this PR from one
    // the baseline outgrew.
    println!(
        "\ntess-lint: gate vs {baseline_path} ({}): {} finding(s)",
        provenance(base_cut.as_ref()),
        report.findings.len()
    );
    for f in &report.findings {
        println!("{}", line("  FINDING", f));
    }
    if !report.findings.is_empty() {
        // stderr, and stderr only: this verdict must survive a
        // redirected stdout — it is the reason the row is red.
        eprint!("{}", discipline(&report.findings, base_cut.as_ref()));
        std::process::exit(EXIT_FINDINGS);
    }
    println!("tess-lint: clean — no scene grew and no face's sizing got wastefuller");
}
