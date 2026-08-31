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
//!   since TESS-SPLIT the shipped schedule IS the cell minimizer
//!   under the ratified A = 16 aspect cap, so this reads ~1.0 where
//!   no constraint is active and the residue is the indicated price
//!   of the cap and the sliver snap (`cap_bands` / `snap_bands`).
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
//! 4. **A re-keyed face** — an ordinal whose two rows are not one
//!    face, so rule 2 has nothing it can compare there. Announced
//!    with the column that disagreed, never resolved into a
//!    comparison.
//! 5. **An uncovered scene** — a scene the fresh sweep has and the
//!    baseline does not, so every face in it was swept, measured and
//!    compared against nothing.
//!
//! ## Rule 5: corpus growth re-cuts the baseline in the PR that grows
//!
//! A scene the fresh sweep adds FAILS the gate. It reads as the
//! opposite of a finding — coverage went up — and that reading is what
//! let a comparison gate decay in silence: a scene nobody folded is
//! swept, measured and reported forever while the verdict stays green
//! by not looking (#1038's measurement: five scenes, 146 face rows).
//!
//! The **genuinely new** scene and the scene the baseline **outgrew**
//! are the same case here, deliberately. Both are a gate with nothing
//! to compare, both are fixed by the same three steps — re-run the
//! sweep, fold the rows, commit — and both belong in the PR that grew
//! the corpus, which is where the author who knows what the scene is
//! supposed to measure is standing. That is the panic-on-move
//! discipline the rest of this tree runs on, one instrument over: the
//! cost of growth is paid by the growth.
//!
//! What the baseline's [`Cut`] adds is the READING, not the verdict.
//! A scene added after the cut has been uncovered for one PR; a scene
//! the cut already predated has been uncovered for however long that
//! is, and the fold it needs restores comparison from now on without
//! recovering the window — `docs/TESS-BUDGET.md`'s standing sentence,
//! *"restores coverage, it does not verify it"*. Without a recorded
//! cut those two are one undifferentiated "absent", which is why the
//! sweep writes one and the CLI prints it.
//!
//! **The lint PRINTS the cut and stops there; it does not classify a
//! scene as added-after or predated-by.** That is deliberate and it is
//! less than it sounds. Deciding which of the two a scene is needs the
//! scene's own first appearance, and the CSV carries no such column —
//! the sweep records what the corpus HAS, never when it got it — so
//! the lint would have to reach outside its inputs, into git, over a
//! `<stop>/<body>` name that is not a path and often not a file. A
//! reader who knows when the scene landed can date the window from
//! the printed cut in one step; a lint that guessed at it would be
//! stating as a reading something it inferred. Closing the gap
//! properly means a first-appearance column, which is the sweep's
//! half of the contract, not this one's.
//!
//! ## Rule 2 joins on the face ORDINAL, and rule 4 is its precondition
//!
//! `face` is the sweep's only per-face NAME, and any added, dropped or
//! rerouted face renumbers every face above it — so what a positional
//! key needs, that both sides number the same faces, is CHECKED at
//! each ordinal rather than assumed. The check reads
//! [`IDENTITY_COLUMNS`], whose doc carries the principle that chose
//! them: **the precondition is over the columns the comparison does
//! not read, and the comparison is over the rest.**
//!
//! It is POINTWISE, so a value shared by two faces is harmless: the
//! question is whether ordinal *i* is one face on both sides, not
//! whether it is unique. And ordinals BELOW the first disagreement are
//! still compared — they are provably aligned, and suppressing them
//! would hide a regression behind a re-key, which is rule 3's own
//! *"silent coverage loss reads as an improvement"* one level down.
//! From the first disagreement up the walk stops, because an inserted
//! or dropped face shifts everything above it.
//!
//! **What it cannot see, measured rather than shaped:** two faces of
//! one scene agreeing on every identity column and swapping ordinals.
//! Counted over the SIZED rows — an unsized swap costs rule 2 nothing,
//! and that restriction is doing the work — the committed baseline has
//! **8 such pairs, 16 of its 64 sized rows, across 6 of the 12 scenes
//! carrying a sized face** (`lily_leaf_b` ×2, `lily_leaf_c` ×2,
//! `lily_sepal_c`, `loft_prism`, `nonuniform_loft`, `s_duct`), each two
//! walls of one body. Taken across all 1327 rows it is **22,545**.
//!
//! And among the sized rows the list is mostly constant, which is the
//! honest reading of that 8: `chart` is `nurbs` on all 64 and the trim
//! box is `0e0,1e0,0e0,1e0` on all 64, so **five of the eight entries
//! discriminate nothing there and the live pair is `nu`/`nv` alone**.
//! Corpus-wide `chart` and the sizing block do discriminate — six
//! charts appear, and the reroute they catch is a named case — but a
//! reader sizing up the hole should size up `(nu, nv)`. Closing it
//! needs a face identity in a column of the sweep's own, which is
//! `tess_meter`'s half of the contract.
//!
//! **A re-key is a finding where it can cost a measurement**, judged
//! per SCENE: does either side carry a sized face at all. That is
//! coarser than asking whether THIS re-key cost a comparison, and
//! deliberately so — it errs toward the finding. Elsewhere it is a
//! note: rule 1 still runs over the scene's total, so nothing was
//! lost, and a gate that reds where it gates nothing is one people
//! learn to route around. Rule 3 only looks like a counter-example: a
//! vanished scene loses rule 1 with it.
//!
//! **A measurement that could not be read is none of the five, and
//! must not be resolved into one.** Rules 1 and 2 fire on GROWTH
//! only, so any in-band fallback for an unreadable value is the
//! smallest movement expressible and passes by construction. The
//! sizing columns are therefore admitted or refused where they are
//! read (`Admissible`, private), per column, and a refused one leaves in the
//! harness voice — a sweep the lint cannot read is not a tessellation
//! that got better. Rules 3, 4 and 5 say the same thing one level up:
//! a comparison that stopped HAPPENING — or never started — is not
//! growth of any size.
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
    /// The Hessian-sized lane's columns, `None` on a face that lane
    /// did not size.
    ///
    /// Not the same question as `chart`, and the two move
    /// independently: the kernel's lane split (`mesh::trimmed::Lane`)
    /// puts `Surface::Approx` on the sized lane too, so an `approx`
    /// row carries this block — while a `nurbs` face that reroutes
    /// off the lane keeps its chart and loses it. `IDENTITY_COLUMNS`
    /// reads both for that reason.
    pub nurbs: Option<Nurbs>,
}

/// The Hessian-sized lane's columns (`tess_meter::NurbsColumns`).
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct Nurbs {
    /// The trim box the grid spans, `u` low. Read by rule 4 and by no
    /// other rule — see [`IDENTITY_COLUMNS`].
    pub u0: f64,
    /// The trim box, `u` high.
    pub u1: f64,
    /// The trim box, `v` low.
    pub v0: f64,
    /// The trim box, `v` high.
    pub v1: f64,
    /// The whole-patch counterfactual's `u` divisions.
    pub nu: f64,
    /// The whole-patch counterfactual's `v` divisions.
    pub nv: f64,
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
    /// Bands the shipped schedule emitted.
    pub bands: f64,
    /// The constraint-activity indicator (TESS-SPLIT): bands whose
    /// step selection the 3-D aspect cap clamped.
    pub cap_bands: f64,
    /// Bands the malign-band snap projected onto the patch column
    /// schedule with changed counts (either direction).
    pub snap_bands: f64,
    /// Max over bands of the emitted lattice's post-`ceil` spacing
    /// ratio `s_u/s_v`. Reported, not judged here: the sliver-safe
    /// line lives at `mesh::nurbs_cert::SAFE_ASPECT` and is read
    /// there, never from a copy in this crate.
    pub realized_aspect: f64,
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
    /// A constraint-activity count: finite, non-negative (zero is an
    /// inactive constraint, which is a reading).
    Count,
    /// A realized lattice aspect: finite and above zero (every band
    /// has nonempty extents over counts floored at one).
    Aspect,
    /// A trim-box edge in parameter space: finite, and nothing more —
    /// the box's own non-degeneracy is a cross-column property this
    /// per-column table cannot state.
    Extent,
}

impl Admissible {
    /// Whether `v` is a reading of this kind of column.
    fn admits(self, v: f64) -> bool {
        match self {
            Self::CellCount => v.is_finite() && v >= 1.0,
            Self::Target => v.is_finite() && v > 0.0,
            Self::Certificate => v.is_finite() && v >= 0.0,
            Self::OptionalDeviation => v.is_nan() || (v.is_finite() && v >= 0.0),
            Self::Count => v.is_finite() && v >= 0.0,
            Self::Aspect => v.is_finite() && v > 0.0,
            Self::Extent => v.is_finite(),
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
            Self::Count => "a constraint-activity count, finite and non-negative",
            Self::Aspect => "a realized aspect, finite and above zero",
            Self::Extent => "a trim-box edge, finite",
        }
    }
}

/// Where the identity block starts in [`EXPECTED_HEADER`] — the first
/// column after `triangles`, which is the last one every row fills.
const IDENTITY_FIRST: usize = 5;

/// The columns [`identity`] reads, policed exactly as the sizing block
/// is and for the added reason that they are gate INPUTS now: rule 4
/// decides from them whether an ordinal is one face, so a value the
/// lint cannot read must leave in the harness voice rather than
/// manufacturing a re-key.
const IDENTITY_MEASURES: [(&str, Admissible); 6] = [
    ("u0", Admissible::Extent),
    ("u1", Admissible::Extent),
    ("v0", Admissible::Extent),
    ("v1", Admissible::Extent),
    ("nu", Admissible::CellCount),
    ("nv", Admissible::CellCount),
];

/// Where the sizing block starts in [`EXPECTED_HEADER`].
const SIZING_FIRST: usize = 17;

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

/// Where the constraint-activity indicator block starts in
/// [`EXPECTED_HEADER`] (TESS-SPLIT D-3), after `dev_samples`.
const INDICATOR_FIRST: usize = 24;

/// The indicator block, policed exactly as [`SIZING_COLUMNS`] is —
/// NURBS-only, all present or all absent with the sizing block.
const INDICATOR_COLUMNS: [(&str, Admissible); 4] = [
    ("bands", Admissible::CellCount),
    ("cap_bands", Admissible::Count),
    ("snap_bands", Admissible::Count),
    ("realized_aspect", Admissible::Aspect),
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
                                   muu,muv,mvv,mu1,mv1,cells,grid_cells,patch_cells,\
                                   opt_cells,span_opt_cells,worst_cert,worst_dev,\
                                   dev_samples,bands,cap_bands,snap_bands,realized_aspect";

/// Every tag `tess_meter::Chart::tag` emits, restated HERE because
/// `chart` is a column the gate JOINS on: a renamed tag must fail as
/// harness breakage rather than re-key every scene that carries it.
///
/// **Weaker than [`EXPECTED_HEADER`]'s pin, and the difference is
/// worth knowing.** That constant is checked against the meter's own
/// source from the other cargo root; this roster is checked only by
/// this crate's test, so it catches a tag the meter renames — the row
/// then reads as drift, which is the point — but a tag the meter ADDS
/// arrives here as harness breakage on every row carrying it, and
/// nothing on the meter's side says so. Closing that is `tess-meter`'s
/// ground, not this crate's.
pub const CHART_TAGS: [&str; 7] = [
    "plane", "cylinder", "cone", "sphere", "torus", "nurbs", "approx",
];

/// The provenance line `scripts/tess_budget_sweep.sh` writes above
/// [`EXPECTED_HEADER`]: `# tess-budget-cut: <commit> <date>`.
///
/// It exists so that a scene the baseline does not cover can be told
/// apart from a scene the baseline never could have covered. Without
/// it both read as "absent", and the one that matters — a scene the
/// corpus gained before this cut and nobody folded — is
/// indistinguishable from the one that does not.
pub const CUT_PREFIX: &str = "# tess-budget-cut:";

/// The tree a sweep was taken from, as the sweep script recorded it.
///
/// Both fields are the script's readings, kept as text: this crate
/// reports the cut, it does not resolve it. The commit carries a
/// `-dirty` suffix when the sweep ran over uncommitted changes, which
/// is exactly when the pair is least trustworthy and most worth
/// printing.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Cut {
    /// The commit the sweep tree was at, abbreviated, `-dirty` when
    /// the tree carried uncommitted changes.
    pub commit: String,
    /// That commit's committer date, ISO-8601.
    pub date: String,
}

impl std::fmt::Display for Cut {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{} ({})", self.commit, self.date)
    }
}

/// Splits an optional leading [`CUT_PREFIX`] line off a sweep, so that
/// [`parse`] and [`cut`] agree about what line 1 is by construction
/// rather than by two readings kept in step.
///
/// A `#` line that is not a well-formed cut is harness breakage, not a
/// missing cut: the sweep and this lint would then disagree about the
/// provenance format, and a silent `None` there is exactly the
/// unreadable-measurement-as-absence shape the sizing columns already
/// refuse one level down.
fn split_cut(text: &str) -> Result<(Option<Cut>, usize), ParseError> {
    let Some(first) = text.lines().next() else {
        return Ok((None, 0));
    };
    if !first.starts_with('#') {
        return Ok((None, 0));
    }
    let rest = first.strip_prefix(CUT_PREFIX).ok_or_else(|| ParseError {
        line: 1,
        text: format!("comment line is not a `{CUT_PREFIX} <commit> <date>` record: {first}"),
    })?;
    let fields: Vec<&str> = rest.split_whitespace().collect();
    let [commit, date] = fields[..] else {
        return Err(ParseError {
            line: 1,
            text: format!("expected `{CUT_PREFIX} <commit> <date>`, got: {first}"),
        });
    };
    // Enough of a shape check that a placeholder cannot pass as a
    // reading: a commit is hex (plus the dirty marker) and a date
    // starts with its calendar day.
    let hex = commit.strip_suffix("-dirty").unwrap_or(commit);
    let commit_ok = hex.len() >= 7 && hex.chars().all(|c| c.is_ascii_hexdigit());
    let date_ok = date.len() >= 10
        && date.as_bytes()[..10].iter().enumerate().all(|(i, &b)| {
            if i == 4 || i == 7 {
                b == b'-'
            } else {
                b.is_ascii_digit()
            }
        });
    if !commit_ok || !date_ok {
        return Err(ParseError {
            line: 1,
            text: format!("cut record is not <hex commit> <YYYY-MM-DD…>: {first}"),
        });
    }
    Ok((
        Some(Cut {
            commit: commit.to_string(),
            date: date.to_string(),
        }),
        1,
    ))
}

/// The tree a sweep was cut from, or `None` when it records none.
///
/// # Errors
///
/// [`ParseError`] when the provenance line is present but malformed —
/// harness breakage, for the reason [`split_cut`] gives.
pub fn cut(text: &str) -> Result<Option<Cut>, ParseError> {
    split_cut(text).map(|(c, _)| c)
}

/// Parses a budget CSV.
///
/// # Errors
///
/// [`ParseError`] on a malformed [`CUT_PREFIX`] line, a
/// missing/renamed header, a short row, or a field that does not
/// parse — all harness breakage.
pub fn parse(text: &str) -> Result<Vec<Row>, ParseError> {
    let (_, skip) = split_cut(text)?;
    let mut lines = text.lines().enumerate().skip(skip);
    let (i, header) = lines.next().ok_or(ParseError {
        line: skip,
        text: "empty file".into(),
    })?;
    if header.trim() != EXPECTED_HEADER {
        return Err(ParseError {
            line: i + 1,
            text: format!("unexpected header (sweep format drift?): {header}"),
        });
    }
    let expected = EXPECTED_HEADER.split(',').count();
    let mut rows = Vec::new();
    // `(scene, face)` is the gate's per-face join key and the sweep
    // writes one row per face, so a repeat is two faces wearing one
    // name — two tour bodies sharing a `<stop>/<body>` spelling, say.
    // Refused HERE, because every downstream index by that key would
    // otherwise resolve the collision by keeping whichever row it saw
    // last, which is a mis-join dressed as a reading.
    let mut seen: std::collections::HashSet<(&str, usize)> = std::collections::HashSet::new();
    for (i, line) in lines {
        let n = i + 1;
        // A blank line is not a row, and skipping it is the one place
        // in this file where a `continue` discards input. Stated
        // because the module docs name an uncommented skip as this
        // gate's own blind-spot shape: what is dropped here carries no
        // measurement at all — a trailing newline, or the blank a
        // hand-edit leaves — while every line with a field on it goes
        // to the width check below and is refused if it is short.
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
        // Every column after `triangles` is the Hessian-sized lane's,
        // and `tess_meter::FaceRow::csv_row` writes them from ONE
        // `Option`: all present, or all absent on a face the lane did
        // not size. Checked over the whole run — identity columns
        // included, since rule 4 reads those — because a half-filled
        // row is the sweep and the lint disagreeing about the file,
        // and reading part of it would let a re-key be manufactured
        // out of drift.
        let measured: Vec<&str> = f[IDENTITY_FIRST..].to_vec();
        let nurbs = if measured.iter().all(|s| s.is_empty()) {
            None
        } else if measured.iter().any(|s| s.is_empty()) {
            return Err(ParseError {
                line: n,
                text: "partially filled NURBS columns".into(),
            });
        } else {
            let mut ident = [0.0f64; IDENTITY_MEASURES.len()];
            for (k, (name, kind)) in IDENTITY_MEASURES.iter().enumerate() {
                ident[k] = admit(IDENTITY_FIRST + k, name, *kind)?;
            }
            let [u0, u1, v0, v1, nu, nv] = ident;
            let mut read = [0.0f64; SIZING_COLUMNS.len()];
            for (k, (name, kind)) in SIZING_COLUMNS.iter().enumerate() {
                read[k] = admit(SIZING_FIRST + k, name, *kind)?;
            }
            let mut ind = [0.0f64; INDICATOR_COLUMNS.len()];
            for (k, (name, kind)) in INDICATOR_COLUMNS.iter().enumerate() {
                ind[k] = admit(INDICATOR_FIRST + k, name, *kind)?;
            }
            let [
                grid_cells,
                patch_cells,
                opt_cells,
                span_opt_cells,
                worst_cert,
                worst_dev,
            ] = read;
            let [bands, cap_bands, snap_bands, realized_aspect] = ind;
            Some(Nurbs {
                u0,
                u1,
                v0,
                v1,
                nu,
                nv,
                grid_cells,
                patch_cells,
                opt_cells,
                span_opt_cells,
                worst_cert,
                worst_dev: worst_dev.is_finite().then_some(worst_dev),
                bands,
                cap_bands,
                snap_bands,
                realized_aspect,
            })
        };
        if !CHART_TAGS.contains(&f[2]) {
            // `chart` is a gate INPUT (rule 4 joins on it), so an
            // unknown tag is sweep-format drift and leaves in the
            // harness voice — the same treatment, and the same reason,
            // as a renamed column in `EXPECTED_HEADER`. A tag renamed
            // in `tess_meter::Chart::tag` must never arrive here as a
            // re-key on every scene that carries it.
            return Err(ParseError {
                line: n,
                text: format!(
                    "chart: {:?} is not one of {} (sweep drift?)",
                    f[2],
                    CHART_TAGS.join(", ")
                ),
            });
        }
        let face = idx(1, "face")?;
        if !seen.insert((f[0], face)) {
            return Err(ParseError {
                line: n,
                text: format!(
                    "second row for scene {:?} face {face}: the per-face join needs one row per face",
                    f[0]
                ),
            });
        }
        rows.push(Row {
            scene: f[0].to_string(),
            face,
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

/// Rows grouped by scene and indexed by face ordinal, with the scenes
/// in first-seen order (tour order — the sweep writes rows as it walks
/// the tour).
///
/// The ONE group-by: [`totals`] and the per-face rules fold the same
/// index, so they cannot disagree about which rows are a scene's.
/// [`parse`] refuses a repeated `(scene, face)`, so nothing is dropped
/// building it — a silent overwrite here would be the mis-join rule 4
/// exists to announce, one level down.
fn by_scene(rows: &[Row]) -> (Vec<&str>, std::collections::HashMap<&str, FaceIndex<'_>>) {
    let mut order: Vec<&str> = Vec::new();
    let mut map: std::collections::HashMap<&str, FaceIndex> = std::collections::HashMap::new();
    for r in rows {
        map.entry(r.scene.as_str())
            .or_insert_with(|| {
                order.push(r.scene.as_str());
                FaceIndex::new()
            })
            .insert(r.face, r);
    }
    (order, map)
}

/// A scene's rows, indexed by face ordinal.
type FaceIndex<'a> = std::collections::BTreeMap<usize, &'a Row>;

/// Per-scene totals, in first-seen order (which is tour order — the
/// sweep writes rows as it walks the tour). Within a scene the fold is
/// by ascending face ordinal, which is what [`by_scene`] indexes by,
/// so a total does not depend on the order rows happen to sit in the
/// file.
pub fn totals(rows: &[Row]) -> Vec<(String, SceneTotals)> {
    let (order, map) = by_scene(rows);
    order
        .into_iter()
        .map(|scene| {
            let mut t = SceneTotals::default();
            // `order` and `map` come from ONE `by_scene` call over one
            // slice, so the miss this `flat_map` folds away cannot
            // happen: every name in the order has an index. It is
            // spelled as an empty fold rather than an `expect` because
            // an absent scene and an empty scene total to the same
            // thing here, and there is no reading to lose — unlike the
            // sizing columns, where an unreadable value becomes a
            // passing one and is refused at the read instead.
            for r in map.get(scene).into_iter().flat_map(FaceIndex::values) {
                t.add(r);
            }
            (scene.to_string(), t)
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

/// The columns rule 4 reads, in [`EXPECTED_HEADER`] order, and the
/// order a disagreement is reported in.
///
/// Every one of them is a column rules 1 and 2 do NOT compare, which
/// is what keeps the precondition from being circular: rule 1 reads
/// `triangles` and rule 2 reads `grid_cells / span_opt_cells`, and a
/// face whose SIZING moved does not move any name below. `nu`/`nv` are
/// the whole-patch counterfactual's divisions and reach the report
/// only through `patch_cells`, which no rule gates on.
///
/// "the sizing block" is whether the row carries the Hessian-sized
/// columns at all — the reroute case, where a face keeps its surface
/// description and leaves the sized lane.
const IDENTITY_COLUMNS: [&str; 8] = [
    "chart",
    "the sizing block",
    "u0",
    "u1",
    "v0",
    "v1",
    "nu",
    "nv",
];

/// One column's reading, as the precondition compares it.
///
/// Numbers are compared as NUMBERS and rendered only for the message.
/// `-0e0` and `0e0` are the same trim-box edge, and `{:?}` renders
/// them differently — comparing the renderings would announce a
/// re-key over a sign bit and stop a scene's comparison from that
/// ordinal up. `f64`'s `==` is the right test here: [`parse`] admits
/// nothing non-finite into these columns, so there is no `NaN` to
/// make it non-reflexive.
#[derive(Clone, Copy, Debug, PartialEq)]
enum Reading<'a> {
    /// A tag, compared as text.
    Tag(&'a str),
    /// A measured column.
    Number(f64),
    /// The row does not carry this column.
    Absent,
}

impl Reading<'_> {
    /// This reading as the message spells it.
    fn show(self) -> String {
        match self {
            Self::Tag(s) => s.to_string(),
            Self::Number(v) => format!("{v:?}"),
            Self::Absent => "absent".to_string(),
        }
    }
}

/// One row's reading of [`IDENTITY_COLUMNS`], index for index.
fn identity(r: &Row) -> [Reading<'_>; IDENTITY_COLUMNS.len()] {
    let col = |f: fn(&Nurbs) -> f64| r.nurbs.map_or(Reading::Absent, |n| Reading::Number(f(&n)));
    [
        Reading::Tag(r.chart.as_str()),
        Reading::Tag(if r.nurbs.is_some() {
            "present"
        } else {
            "absent"
        }),
        col(|n| n.u0),
        col(|n| n.u1),
        col(|n| n.v0),
        col(|n| n.v1),
        col(|n| n.nu),
        col(|n| n.nv),
    ]
}

/// Why an ordinal's two rows are not the same face.
///
/// A re-key is announced WITH its evidence: "the roster moved" and
/// nothing else is a verdict with no reading under it, and the two
/// cases a reader must tell apart — a face that left, and a face that
/// was replaced — look identical without one.
#[derive(Clone, Debug, PartialEq, Eq)]
pub enum Rekey {
    /// Only one side has a row at this ordinal; `in_baseline` says
    /// which side has it.
    Absent { in_baseline: bool },
    /// Both sides have a row and this [`IDENTITY_COLUMNS`] entry
    /// disagrees — the first one, with both readings.
    Column {
        /// The column that disagreed.
        name: &'static str,
        /// The baseline's reading.
        was: String,
        /// The fresh sweep's reading.
        now: String,
    },
}

/// What a comparison observed. The numbers ride with the kind that
/// gives them a unit, so nothing has to remember whether a `was` is
/// triangles, a ratio or a face count.
#[derive(Clone, Debug, PartialEq)]
pub enum Kind {
    /// A scene's triangle count grew (rule 1).
    Triangles {
        /// Baseline triangles.
        was: f64,
        /// Fresh triangles.
        now: f64,
    },
    /// A face's recoverable slack grew — the sizing got wastefuller
    /// (rule 2).
    Slack {
        /// The face's ordinal.
        face: usize,
        /// Baseline `grid_cells / span_opt_cells`.
        was: f64,
        /// The same ratio now.
        now: f64,
    },
    /// A baseline scene the fresh sweep has no row for (rule 3).
    Vanished {
        /// What the scene carried in the baseline.
        was_triangles: f64,
    },
    /// The per-face join's precondition failed at this ordinal, so it
    /// and every ordinal above it went uncompared (rule 4).
    Rekeyed {
        /// The lowest ordinal whose two rows are not one face.
        face: usize,
        /// What disagreed there.
        how: Rekey,
    },
    /// A scene the fresh sweep has and the baseline does not, so the
    /// gate had nothing to compare any of its faces against (rule 5).
    ///
    /// The baseline's own cut is what tells its two readings apart —
    /// a scene the corpus gained after the cut, and a scene the cut
    /// already predated — so the CLI prints [`Cut`] beside it. Both
    /// readings have the same recourse, which is why one variant
    /// carries both.
    Uncovered {
        /// What the uncovered scene carries.
        triangles: f64,
    },
}

/// One observation and the scene it is about.
#[derive(Clone, Debug, PartialEq)]
pub struct Observation {
    /// `<stop>/<body>`.
    pub scene: String,
    /// What was observed.
    pub kind: Kind,
}

/// What [`compare`] answers: what fails the row, and what is reported
/// without failing it.
///
/// The split is the gate's own and is decided by whether an
/// observation can cost a MEASUREMENT — never by how alarming it
/// looks. A gate that reds where it gates nothing is trained away, and
/// this tree has already paid for that once.
#[derive(Clone, Debug, Default, PartialEq)]
pub struct Report {
    /// Movements that fail the gate.
    pub findings: Vec<Observation>,
    /// Observations that cost the comparison nothing — reported, never
    /// red.
    pub notes: Vec<Observation>,
}

/// The gate: fresh against baseline, per the five rules in the module
/// docs.
///
/// Both slices must come from [`parse`], which is what makes the
/// per-face index total: it refuses a repeated `(scene, face)`, and a
/// hand-built pair of rows sharing one would be indexed by whichever
/// came last.
pub fn compare(baseline: &[Row], fresh: &[Row]) -> Report {
    let mut out = Report::default();
    let base_totals = totals(baseline);
    // ONE fold of the fresh side: the same totals answer both the
    // per-scene rules below and the new-scene notes.
    let fresh_scenes = totals(fresh);
    let fresh_totals: std::collections::HashMap<&str, &SceneTotals> = fresh_scenes
        .iter()
        .map(|(scene, t)| (scene.as_str(), t))
        .collect();
    for (scene, was) in &base_totals {
        #[allow(clippy::cast_precision_loss)]
        let w = was.triangles as f64;
        let Some(now) = fresh_totals.get(scene.as_str()) else {
            out.findings.push(Observation {
                scene: scene.clone(),
                kind: Kind::Vanished { was_triangles: w },
            });
            continue;
        };
        #[allow(clippy::cast_precision_loss)]
        let n = now.triangles as f64;
        if n > w * GROWTH_TOLERANCE {
            out.findings.push(Observation {
                scene: scene.clone(),
                kind: Kind::Triangles { was: w, now: n },
            });
        }
    }
    // A scene only the FRESH sweep has is a hole in the very
    // comparison the gate is making: every face in it was swept,
    // measured and compared against nothing. The gate cannot compare
    // what the baseline lacks, so it says so and fails rather than
    // reporting clean over a scene it never looked at.
    //
    // Disjoint from the `Vanished` walk above by construction — that
    // one ranges over the baseline's scenes, this one over the
    // fresh sweep's, and a scene either side lacks is in exactly one
    // of them — so neither direction can shadow the other.
    let base_scenes: std::collections::HashSet<&str> =
        baseline.iter().map(|r| r.scene.as_str()).collect();
    for (scene, t) in &fresh_scenes {
        if !base_scenes.contains(scene.as_str()) {
            #[allow(clippy::cast_precision_loss)]
            let triangles = t.triangles as f64;
            out.findings.push(Observation {
                scene: scene.clone(),
                kind: Kind::Uncovered { triangles },
            });
        }
    }
    // Rule 2 is per FACE — a scene total would let one face's
    // regression hide behind another's improvement — and rule 4 is its
    // precondition, both per module docs.
    let (base_order, base_by_scene) = by_scene(baseline);
    let (_, fresh_by_scene) = by_scene(fresh);
    for scene in base_order {
        let (Some(base_faces), Some(fresh_faces)) =
            (base_by_scene.get(scene), fresh_by_scene.get(scene))
        else {
            // `scene` came from the baseline's own order, so the side
            // that can be missing is the fresh one — already a
            // `Vanished` finding above, at the granularity that claim
            // is true at.
            continue;
        };
        // Whether rule 2 has anything to lose in this scene, which is
        // what puts a re-key on the findings side or the notes side.
        let gated = base_faces
            .values()
            .chain(fresh_faces.values())
            .any(|r| r.recoverable().is_some());
        let ordinals: std::collections::BTreeSet<usize> = base_faces
            .keys()
            .chain(fresh_faces.keys())
            .copied()
            .collect();
        for face in ordinals {
            let how = match (base_faces.get(&face), fresh_faces.get(&face)) {
                (Some(b), Some(f)) => match first_disagreement(b, f) {
                    Some(how) => how,
                    None => {
                        // Every identity column agrees, so this ordinal
                        // is one face on both sides and rule 2 runs on
                        // it. Agreement includes "the sizing block", so
                        // the ratio is present on both sides or on
                        // neither and a skip here is a face with no
                        // slack to compare, never a comparison dropped.
                        if let (Some(was), Some(now)) = (b.recoverable(), f.recoverable())
                            && now > was * GROWTH_TOLERANCE
                        {
                            out.findings.push(Observation {
                                scene: scene.to_string(),
                                kind: Kind::Slack { face, was, now },
                            });
                        }
                        continue;
                    }
                },
                (Some(_), None) => Rekey::Absent { in_baseline: true },
                // An ordinal in neither index cannot be in the union
                // above, so this arm is the fresh side's alone.
                (None, _) => Rekey::Absent { in_baseline: false },
            };
            let seen = Observation {
                scene: scene.to_string(),
                kind: Kind::Rekeyed { face, how },
            };
            if gated {
                out.findings.push(seen);
            } else {
                out.notes.push(seen);
            }
            // Ordinals BELOW this one are provably still aligned and
            // have already been compared. From here up they are not:
            // an added or dropped face shifts every ordinal above it,
            // so the walk stops rather than comparing shifted pairs.
            break;
        }
    }
    out
}

/// The first [`IDENTITY_COLUMNS`] entry at which two rows are not one
/// face, or `None` when every one agrees.
///
/// Both readings come from [`identity`], so index `i` names the same
/// column on both sides by construction rather than by a pairing that
/// could drift.
fn first_disagreement(base: &Row, fresh: &Row) -> Option<Rekey> {
    let (was, now) = (identity(base), identity(fresh));
    IDENTITY_COLUMNS
        .iter()
        .enumerate()
        .find(|&(i, _)| was[i] != now[i])
        .map(|(i, &name)| Rekey::Column {
            name,
            was: was[i].show(),
            now: now[i].show(),
        })
}

#[cfg(test)]
mod tests {
    use super::*;

    /// A two-face fixture: one plane (empty NURBS columns), one NURBS
    /// wall at ordinal 1.
    ///
    /// Twinned in `tests/cli_contract.rs`, deliberately: an
    /// integration test cannot see a `#[cfg(test)]` item, so the two
    /// cannot share one. Keep them in step.
    fn csv(tris: usize, span_opt: f64) -> String {
        format!(
            "{EXPECTED_HEADER}\n{}\
             s/b,1,nurbs,2e-3,{tris},0e0,1e0,0e0,1e0,1e1,2e1,1e0,1e0,1e0,2e0,3e0,4,\
             1e2,2e2,5e1,{span_opt:e},1e-4,5e-5,99,2,1,0,3e0\n",
            unsized_row(0, "plane", 4)
        )
    }

    /// A row on a lane that sizes nothing, at a chosen ordinal and
    /// chart — enough to move a scene's roster without moving a
    /// triangle. The empty tail is COUNTED from the header, at the
    /// column [`IDENTITY_FIRST`] names, so a schema change cannot turn
    /// these fixtures into short rows that fail for the wrong reason.
    fn unsized_row(face: usize, chart: &str, tris: usize) -> String {
        let blanks = ",".repeat(EXPECTED_HEADER.split(',').count() - IDENTITY_FIRST);
        format!("s/b,{face},{chart},2e-3,{tris}{blanks}\n")
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

    /// The kinds the gate reported, findings only — most tests are
    /// about which rule spoke, not about which scene it was in.
    fn fired(base: &[Row], fresh: &[Row]) -> Vec<Kind> {
        compare(base, fresh)
            .findings
            .into_iter()
            .map(|o| o.kind)
            .collect()
    }

    /// The same for the notes, which never fail the row.
    fn noted(base: &[Row], fresh: &[Row]) -> Vec<Kind> {
        compare(base, fresh)
            .notes
            .into_iter()
            .map(|o| o.kind)
            .collect()
    }

    #[test]
    fn an_unmoved_sweep_is_clean() {
        let base = parse(&csv(100, 2.5e1)).unwrap();
        assert_eq!(compare(&base, &base), Report::default());
    }

    #[test]
    fn growth_inside_the_tolerance_is_not_a_finding() {
        let base = parse(&csv(100, 2.5e1)).unwrap();
        let fresh = parse(&csv(104, 2.5e1)).unwrap();
        assert_eq!(compare(&base, &fresh), Report::default());
    }

    #[test]
    fn triangle_growth_fires() {
        let base = parse(&csv(100, 2.5e1)).unwrap();
        let fresh = parse(&csv(200, 2.5e1)).unwrap();
        // The plane's 4 triangles ride along in the scene total.
        assert_eq!(
            fired(&base, &fresh),
            vec![Kind::Triangles {
                was: 104.0,
                now: 204.0
            }]
        );
    }

    /// The rule that a triangle count alone cannot express: the mesh
    /// got SMALLER while the sizing schedule got wastefuller.
    #[test]
    fn slack_growth_fires_even_as_the_mesh_shrinks() {
        let base = parse(&csv(100, 2.5e1)).unwrap();
        let fresh = parse(&csv(50, 1.0e1)).unwrap();
        assert_eq!(
            fired(&base, &fresh),
            vec![Kind::Slack {
                face: 1,
                was: 4.0,
                now: 10.0
            }]
        );
    }

    #[test]
    fn a_vanished_scene_is_a_finding_not_an_improvement() {
        let base = parse(&csv(100, 2.5e1)).unwrap();
        let fresh = parse(EXPECTED_HEADER).unwrap();
        assert_eq!(
            fired(&base, &fresh),
            vec![Kind::Vanished {
                was_triangles: 104.0
            }]
        );
    }

    /// Rule 5. The scene the baseline does not cover is the one the
    /// gate never looked at, so it fails the row rather than riding
    /// along as good news.
    #[test]
    fn an_uncovered_scene_is_a_finding_not_a_note() {
        let base = parse(EXPECTED_HEADER).unwrap();
        let fresh = parse(&csv(100, 2.5e1)).unwrap();
        assert_eq!(
            fired(&base, &fresh),
            vec![Kind::Uncovered { triangles: 104.0 }]
        );
        assert_eq!(noted(&base, &fresh), vec![]);
    }

    /// Rule 5 must not eat rule 3. The two walks range over different
    /// sides, so a sweep that drops one scene and adds another owes
    /// BOTH findings — the fold that answers the second one must not
    /// be allowed to bury the first.
    #[test]
    fn an_uncovered_scene_does_not_shadow_a_vanished_one() {
        let base = parse(&csv(100, 2.5e1).replace("s/b", "gone/gone")).unwrap();
        let fresh = parse(&csv(100, 2.5e1)).unwrap();
        assert_eq!(
            fired(&base, &fresh),
            vec![
                Kind::Vanished {
                    was_triangles: 104.0
                },
                Kind::Uncovered { triangles: 104.0 }
            ]
        );
    }

    #[test]
    fn a_sweep_records_the_tree_it_was_cut_from() {
        let text = format!(
            "{CUT_PREFIX} 1a2b3c4d5e6f 2026-08-30T12:00:00+00:00\n{}",
            csv(100, 2.5e1)
        );
        assert_eq!(
            cut(&text).unwrap(),
            Some(Cut {
                commit: "1a2b3c4d5e6f".into(),
                date: "2026-08-30T12:00:00+00:00".into()
            })
        );
        // And the rows behind it parse exactly as they do without one.
        assert_eq!(parse(&text).unwrap(), parse(&csv(100, 2.5e1)).unwrap());
    }

    #[test]
    fn a_dirty_cut_says_so_rather_than_reading_as_clean() {
        let text = format!(
            "{CUT_PREFIX} 1a2b3c4d-dirty 2026-08-30\n{}",
            csv(100, 2.5e1)
        );
        assert_eq!(cut(&text).unwrap().unwrap().commit, "1a2b3c4d-dirty");
    }

    #[test]
    fn a_sweep_without_a_cut_line_records_none() {
        assert_eq!(cut(&csv(100, 2.5e1)).unwrap(), None);
    }

    /// A provenance line the lint cannot read is the sweep and the
    /// lint disagreeing about the format — harness breakage, never a
    /// silently absent cut, for the reason the sizing columns refuse
    /// an unreadable value one level down.
    #[test]
    fn a_malformed_cut_line_is_harness_breakage_not_an_absent_cut() {
        for bad in [
            "# tess-budget-cut: 1a2b3c4d5e6f",
            "# tess-budget-cut: not-hex 2026-08-30",
            "# tess-budget-cut: 1a2b3c 2026-08-30",
            "# tess-budget-cut: 1a2b3c4d5e6f yesterday",
            "# swept at 1a2b3c4d5e6f 2026-08-30",
        ] {
            let text = format!("{bad}\n{}", csv(100, 2.5e1));
            let e = cut(&text).unwrap_err();
            assert_eq!(e.line, 1, "{bad}");
            assert!(parse(&text).is_err(), "parse admitted {bad}");
        }
    }

    /// The cut line shifts the physical lines below it, and a parse
    /// error must name the line the reader will find in the file.
    #[test]
    fn a_cut_line_shifts_the_reported_line_numbers() {
        let text = format!(
            "{CUT_PREFIX} 1a2b3c4d5e6f 2026-08-30\n{EXPECTED_HEADER}\ns/b,0,plane,2e-3,4\n"
        );
        let e = parse(&text).unwrap_err();
        assert_eq!(e.line, 3);
    }

    /// Sizing columns are all-or-nothing: a half-filled row means the
    /// sweep and the lint disagree about the schema.
    #[test]
    fn a_half_filled_sizing_row_is_harness_breakage() {
        let bad = format!(
            "{EXPECTED_HEADER}\n\
             s/b,1,nurbs,2e-3,9,0e0,1e0,0e0,1e0,1e1,2e1,1e0,1e0,1e0,2e0,3e0,4,1e2,,5e1,2.5e1,\
             1e-4,5e-5,99,2,1,0,3e0\n"
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

    /// The identity block's policing, in the same shape as the other
    /// two: an unreadable identity column must be harness breakage,
    /// because rule 4 decides FROM these values and a broken one would
    /// manufacture a re-key — a scene's comparison stopped by drift
    /// rather than by geometry.
    ///
    /// The expectations are written out rather than derived from
    /// [`IDENTITY_MEASURES`]: a test that reads the policy it is
    /// checking asserts nothing. The array's width is the guard
    /// against the next column.
    #[test]
    fn every_identity_column_refuses_the_values_that_would_manufacture_a_re_key() {
        // `5e-1` separates a DIVISION COUNT, floored at one by
        // `tess_meter::divisions`, from any merely positive policy;
        // `inf`/`NaN` separate a finite trim-box edge from no policy at
        // all; `0e0`/`-1e0` separate an edge, which is a signed
        // parameter value, from a count.
        const BAD: [&str; 5] = ["0e0", "-1e0", "inf", "NaN", "5e-1"];
        const ADMITTED: [(&str, [bool; 5]); IDENTITY_MEASURES.len()] = [
            ("u0", [true, true, false, false, true]),
            ("u1", [true, true, false, false, true]),
            ("v0", [true, true, false, false, true]),
            ("v1", [true, true, false, false, true]),
            ("nu", [false, false, false, false, false]),
            ("nv", [false, false, false, false, false]),
        ];
        for (k, (name, admitted)) in ADMITTED.iter().enumerate() {
            assert_eq!(*name, IDENTITY_MEASURES[k].0, "column {k} of the table");
            for (b, bad) in BAD.iter().enumerate() {
                let got = parse(&with_field(&csv(100, 2.5e1), IDENTITY_FIRST + k, bad)).is_ok();
                assert_eq!(got, admitted[b], "{name} = {bad}: admitted = {got}");
            }
        }
    }

    /// A trim-box edge that reads `-0e0` where the baseline read `0e0`
    /// is the SAME EDGE, and announcing a re-key over it would stop the
    /// scene's comparison from that ordinal up. Identity is compared as
    /// numbers for this reason; `{:?}` renders the two differently.
    #[test]
    fn a_signed_zero_extent_is_not_a_re_key() {
        let base = parse(&csv(100, 2.5e1)).unwrap();
        let fresh = parse(&with_field(&csv(100, 2.5e1), IDENTITY_FIRST, "-0e0")).unwrap();
        assert_eq!(fresh[1].nurbs.unwrap().u0, 0.0, "and it parsed as an edge");
        assert!(
            fresh[1].nurbs.unwrap().u0.is_sign_negative(),
            "the sign bit really is there to be tripped over"
        );
        assert_eq!(compare(&base, &fresh), Report::default());
    }

    /// The block the parser polices is the header's own, bracketed on
    /// both sides: a column inserted into the sizing run would slide
    /// every measurement under the wrong policy, and the drifting
    /// header this file already refuses is the same failure one step
    /// earlier.
    #[test]
    fn the_policed_block_is_the_headers_sizing_block() {
        let cols: Vec<&str> = EXPECTED_HEADER.split(',').collect();
        // The identity block first, bracketed the same way: a column
        // inserted at its head would slide every identity reading
        // under the wrong policy AND re-key every scene at once.
        assert_eq!(
            cols[IDENTITY_FIRST - 1],
            "triangles",
            "the identity block starts too late"
        );
        for (k, (name, _)) in IDENTITY_MEASURES.iter().enumerate() {
            assert_eq!(cols[IDENTITY_FIRST + k], *name, "identity column {k}");
        }
        assert_eq!(
            cols[IDENTITY_FIRST + IDENTITY_MEASURES.len()],
            "muu",
            "the identity block ends too late"
        );
        assert_eq!(cols[SIZING_FIRST - 1], "cells", "the block starts too late");
        for (k, (name, _)) in SIZING_COLUMNS.iter().enumerate() {
            assert_eq!(cols[SIZING_FIRST + k], *name, "column {k}");
        }
        assert_eq!(
            cols[SIZING_FIRST + SIZING_COLUMNS.len()],
            "dev_samples",
            "the block ends too early"
        );
        // …and the indicator block is bracketed the same way: after
        // `dev_samples`, through to the end of the header.
        assert_eq!(
            cols[INDICATOR_FIRST - 1],
            "dev_samples",
            "the indicator block starts too late"
        );
        for (k, (name, _)) in INDICATOR_COLUMNS.iter().enumerate() {
            assert_eq!(cols[INDICATOR_FIRST + k], *name, "indicator column {k}");
        }
        assert_eq!(
            cols.len(),
            INDICATOR_FIRST + INDICATOR_COLUMNS.len(),
            "the indicator block ends before the header does"
        );
    }

    /// The indicator block's policing, in the same shape as the sizing
    /// block's: written out, not derived from the table it checks.
    #[test]
    fn every_indicator_column_refuses_the_values_that_would_hide_a_broken_reading() {
        const BAD: [&str; 5] = ["0e0", "-1e0", "inf", "NaN", "5e-1"];
        const ADMITTED: [(&str, [bool; 5]); INDICATOR_COLUMNS.len()] = [
            // A schedule always has at least one band.
            ("bands", [false, false, false, false, false]),
            // Zero is "constraint inactive", a reading.
            ("cap_bands", [true, false, false, false, true]),
            ("snap_bands", [true, false, false, false, true]),
            // An aspect is a positive finite ratio.
            ("realized_aspect", [false, false, false, false, true]),
        ];
        for (k, (name, admitted)) in ADMITTED.iter().enumerate() {
            assert_eq!(*name, INDICATOR_COLUMNS[k].0, "column {k} of the table");
            for (b, bad) in BAD.iter().enumerate() {
                let got = parse(&with_field(&csv(100, 2.5e1), INDICATOR_FIRST + k, bad)).is_ok();
                assert_eq!(got, admitted[b], "{name} = {bad}: admitted = {got}");
            }
        }
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
        assert_eq!(
            fired(&base, &fresh),
            vec![Kind::Slack {
                face: 1,
                was: 4.0,
                now: 100.0
            }]
        );
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
        assert_eq!(compare(&base, &sizing_only(100, 2.5e1)), Report::default());
        assert_eq!(
            fired(&base, &sizing_only(200, 2.5e1)),
            vec![Kind::Triangles {
                was: 104.0,
                now: 204.0
            }]
        );
        // The SLACK rule is the one this finding is about, and an
        // equality-to-empty cannot say it still runs: a `recoverable`
        // that went absent along with `worst_dev` would satisfy every
        // line above.
        assert_eq!(base[1].recoverable(), Some(4.0));
        assert_eq!(
            fired(&base, &sizing_only(100, 1.0e1)),
            vec![Kind::Slack {
                face: 1,
                was: 4.0,
                now: 10.0
            }]
        );
    }

    /// A scene with no Hessian-sized face has no sizing factor and
    /// says so. Reporting 1.0 would be a reading of a grid nobody
    /// sized, in the column where 1.0 means "as good as it gets".
    #[test]
    fn a_scene_with_no_sized_face_reports_no_factor() {
        let planes = parse(&format!(
            "{EXPECTED_HEADER}\n{}",
            unsized_row(0, "plane", 4)
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
        assert_eq!(compare(&base, &fresh), Report::default());
    }

    /// …and from ABOVE: 6% is a finding, so the constant cannot be
    /// widened to 1.06. This is the side that matters — the move a red
    /// gate tempts is to raise the tolerance until it goes quiet, and
    /// the pair leaves a 2-point window to raise it into.
    #[test]
    fn a_six_percent_scene_is_a_finding() {
        let base = parse(&csv(96, 2.5e1)).unwrap();
        let fresh = parse(&csv(102, 2.5e1)).unwrap();
        assert!(
            matches!(fired(&base, &fresh)[..], [Kind::Triangles { .. }]),
            "{:?}",
            fired(&base, &fresh)
        );
    }

    /// The same box on the SLACK rule, which shares the constant: the
    /// ratio of the two baselines' `span_opt_cells` is the growth, so
    /// 26 → 25 is exactly 1.04 and stays clean.
    #[test]
    fn a_four_percent_slack_growth_is_inside_the_tolerance() {
        let base = parse(&csv(100, 2.6e1)).unwrap();
        let fresh = parse(&csv(100, 2.5e1)).unwrap();
        assert_eq!(compare(&base, &fresh), Report::default());
    }

    /// …and 26.5 → 25 is exactly 1.06 and fires. Boxing both rules
    /// rather than one keeps the box intact if the constant is ever
    /// split in two: a second threshold with no box would red here.
    #[test]
    fn a_six_percent_slack_growth_is_a_finding() {
        let base = parse(&csv(100, 2.65e1)).unwrap();
        let fresh = parse(&csv(100, 2.5e1)).unwrap();
        assert!(
            matches!(fired(&base, &fresh)[..], [Kind::Slack { .. }]),
            "{:?}",
            fired(&base, &fresh)
        );
    }

    /// Rule 4's headline case, and the one the ordinal join used to
    /// lose in silence: the scene is still there, so `Vanished` says
    /// nothing, and the face the slack rule was watching is gone.
    #[test]
    fn a_face_missing_from_a_surviving_scene_is_a_finding() {
        let base = parse(&csv(100, 2.5e1)).unwrap();
        let fresh = parse(&format!(
            "{EXPECTED_HEADER}\n{}",
            unsized_row(0, "plane", 4)
        ))
        .unwrap();
        assert_eq!(
            fired(&base, &fresh),
            vec![Kind::Rekeyed {
                face: 1,
                how: Rekey::Absent { in_baseline: true }
            }]
        );
    }

    /// The same rule from the other side: a face only the FRESH sweep
    /// has.
    #[test]
    fn a_face_only_the_fresh_sweep_has_is_a_finding() {
        let base = parse(&csv(100, 2.5e1)).unwrap();
        let fresh = parse(&format!(
            "{}{}",
            csv(100, 2.5e1),
            unsized_row(2, "plane", 0)
        ))
        .unwrap();
        assert_eq!(
            fired(&base, &fresh),
            vec![Kind::Rekeyed {
                face: 2,
                how: Rekey::Absent { in_baseline: false }
            }]
        );
    }

    /// A NURBS face that reroutes off the sized lane keeps its ordinal
    /// and its triangle count, so nothing scene-granular moves — and
    /// dropping it quietly is the coverage loss the module docs call a
    /// finding rather than a footnote. The message must say the SIZING
    /// BLOCK went, not that a schedule got wastefuller.
    #[test]
    fn a_face_that_leaves_the_sized_lane_names_the_sizing_block() {
        let base = parse(&csv(100, 2.5e1)).unwrap();
        // Same chart, no sizing columns: `Chart::of` reads the
        // surface and the block records the LANE, so these two move
        // independently and the reroute is exactly that case.
        let fresh = parse(&format!(
            "{EXPECTED_HEADER}\n{}{}",
            unsized_row(0, "plane", 4),
            unsized_row(1, "nurbs", 100)
        ))
        .unwrap();
        assert_eq!(
            fired(&base, &fresh),
            vec![Kind::Rekeyed {
                face: 1,
                how: Rekey::Column {
                    name: "the sizing block",
                    was: "present".into(),
                    now: "absent".into()
                }
            }]
        );
    }

    /// Every identity column re-keys ON ITS OWN, so none can leave
    /// [`IDENTITY_COLUMNS`] without a row going red — the mutation
    /// that motivated this test dropped one half of the old two-part
    /// key and the whole suite stayed green.
    ///
    /// Written out rather than derived from the table it checks, and
    /// the array's width is the guard against the NEXT column: an
    /// eighth entry with no row here does not compile.
    #[test]
    fn every_identity_column_re_keys_on_its_own() {
        // `approx` is a real tag on the SIZED lane, so the chart row
        // moves chart and nothing else — a mutation that dropped
        // `chart` from the list would pass without it.
        const MOVED: [(&str, usize, &str); IDENTITY_COLUMNS.len() - 1] = [
            ("chart", 2, "approx"),
            ("u0", IDENTITY_FIRST, "5e-1"),
            ("u1", IDENTITY_FIRST + 1, "9e-1"),
            ("v0", IDENTITY_FIRST + 2, "5e-1"),
            ("v1", IDENTITY_FIRST + 3, "9e-1"),
            ("nu", IDENTITY_FIRST + 4, "9e0"),
            ("nv", IDENTITY_FIRST + 5, "9e0"),
        ];
        let base = parse(&csv(100, 2.5e1)).unwrap();
        for (name, col, value) in MOVED {
            let fresh = parse(&with_field(&csv(100, 2.5e1), col, value)).unwrap();
            let got = fired(&base, &fresh);
            let [
                Kind::Rekeyed {
                    face: 1,
                    how: Rekey::Column { name: got_name, .. },
                },
            ] = got[..]
            else {
                panic!("{name}: expected one re-key, got {got:?}");
            };
            assert_eq!(got_name, name, "the column the message names");
        }
        // …and one case pinned with both readings, so the message's
        // evidence is asserted somewhere and not just its shape.
        let fresh = parse(&with_field(&csv(100, 2.5e1), IDENTITY_FIRST + 4, "9e0")).unwrap();
        assert_eq!(
            fired(&base, &fresh),
            vec![Kind::Rekeyed {
                face: 1,
                how: Rekey::Column {
                    name: "nu",
                    was: "10.0".into(),
                    now: "9.0".into()
                }
            }]
        );
    }

    /// The precondition is POINTWISE, so it must not fire on a scene
    /// whose faces merely moved their MEASUREMENTS: a face that got
    /// bigger is rule 1's and rule 2's business, and re-keying it
    /// would suppress the very comparison this gate exists to make.
    #[test]
    fn a_face_whose_measurements_moved_is_still_the_same_face() {
        let base = parse(&csv(100, 2.5e1)).unwrap();
        let fresh = parse(&csv(100, 1.0e1)).unwrap();
        assert!(
            matches!(fired(&base, &fresh)[..], [Kind::Slack { .. }]),
            "{:?}",
            fired(&base, &fresh)
        );
    }

    /// Ordinals BELOW the first disagreement are still compared. The
    /// alternative hides a regression behind a re-key whose printed
    /// recourse is "re-cut", which commits the regression unnamed.
    #[test]
    fn a_re_key_does_not_mask_a_regression_below_it() {
        let base = parse(&csv(100, 2.5e1)).unwrap();
        // Face 1 regresses; face 2 is added above it.
        let fresh = parse(&format!(
            "{}{}",
            csv(100, 1.0e1),
            unsized_row(2, "plane", 0)
        ))
        .unwrap();
        assert_eq!(
            fired(&base, &fresh),
            vec![
                Kind::Slack {
                    face: 1,
                    was: 4.0,
                    now: 10.0
                },
                Kind::Rekeyed {
                    face: 2,
                    how: Rekey::Absent { in_baseline: false }
                },
            ]
        );
    }

    /// …and from the first disagreement UP nothing is compared, so a
    /// shifted face is never measured against the one that took its
    /// ordinal. Here the wall moves from ordinal 1 to 2 and its slack
    /// quadruples; reporting that would report the inserted plane's
    /// ordinal carrying the wall's numbers.
    #[test]
    fn a_shifted_face_above_the_disagreement_is_not_compared() {
        let base = parse(&csv(100, 2.5e1)).unwrap();
        let shifted = with_field(&csv(100, 1.0e1), 1, "2");
        let fresh = parse(&format!("{}{}", shifted, unsized_row(1, "plane", 0))).unwrap();
        assert_eq!(
            fired(&base, &fresh),
            vec![Kind::Rekeyed {
                face: 1,
                how: Rekey::Column {
                    name: "chart",
                    was: "nurbs".into(),
                    now: "plane".into()
                }
            }]
        );
    }

    /// A re-key is a FINDING where it can cost a measurement…
    #[test]
    fn a_re_key_in_a_scene_with_a_sized_face_reds_the_row() {
        let base = parse(&csv(100, 2.5e1)).unwrap();
        let fresh = parse(&format!(
            "{EXPECTED_HEADER}\n{}",
            unsized_row(0, "plane", 4)
        ))
        .unwrap();
        assert_eq!(noted(&base, &fresh), vec![], "nothing to report quietly");
        assert_eq!(fired(&base, &fresh).len(), 1);
    }

    /// …and a NOTE where it cannot. Rule 1 still runs over this
    /// scene's total, so no comparison was lost; reddening here is how
    /// a gate teaches people to route around it. 58 of the committed
    /// baseline's 70 scenes are this shape.
    #[test]
    fn a_re_key_in_a_scene_with_no_sized_face_is_a_note() {
        let base = parse(&format!(
            "{EXPECTED_HEADER}\n{}{}",
            unsized_row(0, "plane", 4),
            unsized_row(1, "cylinder", 10)
        ))
        .unwrap();
        let fresh = parse(&format!(
            "{EXPECTED_HEADER}\n{}",
            unsized_row(0, "plane", 4)
        ))
        .unwrap();
        assert_eq!(fired(&base, &fresh), vec![], "nothing was lost here");
        assert_eq!(
            noted(&base, &fresh),
            vec![Kind::Rekeyed {
                face: 1,
                how: Rekey::Absent { in_baseline: true }
            }]
        );
    }

    /// The MIGRATION DIRECTION, and the half of the finding/note test
    /// that a fixture set can miss: the scene carries no sized face in
    /// the baseline and gains one in the fresh sweep. Rule 2 has
    /// something to lose here — it is about to start measuring this
    /// scene — so the re-key is a FINDING, not the note the baseline
    /// side alone would make it.
    #[test]
    fn a_scene_that_gains_its_first_sized_face_reds_rather_than_notes() {
        let base = parse(&format!(
            "{EXPECTED_HEADER}\n{}{}",
            unsized_row(0, "plane", 4),
            unsized_row(1, "cylinder", 10)
        ))
        .unwrap();
        // The same 14 triangles, so rule 1 cannot speak for it.
        let fresh = parse(&csv(10, 2.5e1)).unwrap();
        assert_eq!(noted(&base, &fresh), vec![], "nothing was reported quietly");
        assert_eq!(
            fired(&base, &fresh),
            vec![Kind::Rekeyed {
                face: 1,
                how: Rekey::Column {
                    name: "chart",
                    was: "cylinder".into(),
                    now: "nurbs".into()
                }
            }]
        );
    }

    /// Two rows for one `(scene, face)` are two faces wearing one
    /// name, and every index by that key would keep whichever came
    /// last. Refused at the parse boundary, in the harness voice.
    #[test]
    fn a_repeated_face_row_is_harness_breakage() {
        let e = parse(&format!(
            "{}{}",
            csv(100, 2.5e1),
            unsized_row(1, "plane", 7)
        ))
        .unwrap_err();
        assert_eq!(e.line, 4);
        assert!(e.text.contains("second row"), "{}", e.text);
        assert!(e.text.contains("face 1"), "{}", e.text);
    }

    /// `chart` is a column the gate JOINS on, so a tag this crate does
    /// not know is sweep-format drift and leaves in the harness voice
    /// — the same treatment `EXPECTED_HEADER` gets, and for the same
    /// reason. Renaming a tag in `tess_meter::Chart::tag` must never
    /// arrive here as a re-key on every scene that carries it.
    #[test]
    fn an_unknown_chart_tag_is_harness_breakage_not_a_re_key() {
        let e = parse(&with_field(&csv(100, 2.5e1), 2, "hessian")).unwrap_err();
        assert_eq!(e.line, 3);
        assert!(e.text.contains("chart"), "{}", e.text);
        assert!(e.text.contains("hessian"), "{}", e.text);
        // …and the roster itself, written out rather than iterated:
        // reading `CHART_TAGS` to check `CHART_TAGS` would let a tag be
        // dropped from it in silence, and a dropped tag turns every
        // scene carrying that chart into harness breakage.
        assert_eq!(
            CHART_TAGS,
            [
                "plane", "cylinder", "cone", "sphere", "torus", "nurbs", "approx"
            ],
            "the tags `tess_meter::Chart::tag` emits"
        );
        for tag in CHART_TAGS {
            assert!(
                parse(&with_field(&csv(100, 2.5e1), 2, tag)).is_ok(),
                "{tag} is a tag the sweep writes"
            );
        }
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
