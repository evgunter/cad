---
id: LIB-MC
kind: unit
title: the advisory Monte Carlo lane gets its Python door and the census reads analysis.rs
status: review
opened: 2026-09-09
branch: lib/mc
refs: [advisory-monte-carlo-lane-has-no-python-door]
---



Closes `advisory-monte-carlo-lane-has-no-python-door` under Ev's
ruling (A): the E11.1 advisory estimator crosses as a free function
with frozen value classes and a typed refusal, `sample_offset` beside
it, and `crates/pncad/src/analysis.rs` joins the files the binding
census reads.

## Delivered

- **`monte_carlo(doc, analyzed, config=None) -> McReport`**, a module
  function on the analysis surface (`crates/pncad-py/src/py/analysis.rs`).
  `analyzed` is REQUIRED and not defaulted: which parameters vary and
  what counts as outside is the analysis's knob (E2), so the box is the
  caller's explicit choice rather than one hidden inside the run. The
  GIL is released for the run — one full document evaluation per
  sample is the lane's honest price.
- **`McConfig(samples=None, seed=None, parallel=None)`**, frozen,
  comparable and hashable, each field defaulting to the kernel's own
  (`DEFAULT_SAMPLES`, `DEFAULT_SEED`, parallel). Both constants are on
  the module. A zero sample count is NOT refused at the constructor:
  the kernel refuses the RUN, and pre-checking here would move a
  refusal off the door that owns it.
- **`McReport` / `McMeasure` / `McAssertion`**, frozen, projecting
  every kernel field; `violation_fraction` is a property answering
  `Optional[float]`. `McReport.render()` crosses too — the label
  discipline E11.1 requires is structural in the kernel (the count and
  the seed on every rendered line) and stopping it at the language
  boundary would have left a Python caller printing unlabeled numbers.
  `McMeasure` and `McAssertion` hash, folding `-0.0` through
  `py::doc::fold_zero` (made `pub(crate)` for it) so the hash cannot
  split what the IEEE equality calls the same.
- **`McRefusal`** as a typed exception under `PncadError`
  (`ErrorClass::Mc`), with `variant` from an exhaustive
  `tags::mc_refusal_tag` and `param`/`node`/`cause` present on every
  arm, `None` off-arm — the projected shape `py/readback.rs` states.
  The band arm's tag DELEGATES to `measure_unavailable_tag`, because it
  carries that very refusal: two words for one fault would let a caller
  who already branches on `band_has_no_measure` miss it here.
- **`sample_offset(param, dist, u) -> Length | Angle | float`.** The
  kernel answers an `f64` offset in the parameter's dimension; the
  dimension crosses off the `Distribution` wrapper, which borrowed it
  from the parameter at construction, so a `Length` annotation's offset
  is a `Length`. `param` is the NAME alone — it names the parameter in
  the band refusal and decides nothing else, so this free door has no
  mispairing to make, which is why it crosses free where `tail_mass`
  and `box_mass` cross as methods on the box.
- **Census.** `crates/pncad/src/analysis.rs` joined `FACADE_FILES`.
  Curated names went 425 -> 486: the file introduces 63, of which two
  (`AssertionVerdict`, `UnevaluatedReason`) were already curated on
  another list, so 61 are new. All 63 accounted: 16 by rule 1 (the
  seven already bound plus the nine this unit binds), 6 in `BOUND_AS`
  — the two above, plus `tail_mass`/`box_mass` as methods on the box
  and `ParamBoxError`/`SeedError` as `EvaluationError.kind`, the two
  payloads the façade carries unconditionally — and 41 in `NOT_BOUND`
  as `different-shape`: `OffsetInterval`, whose Python shape is the
  `(lo, hi)` pair, and the 40 names behind
  `#[cfg(feature = "interval")]`, which are absent from the default
  build the wheel is made from. Six members joined the member rosters:
  `AnalyzedParam::dim` -> `AnalyzedParam.dimension`, and five arms as
  `variant` words (`MeasureUnavailable`'s one,
  `AnalysisPolicyError`'s one, `McRefusal`'s three). No `gap:` row and
  no `FAMILIES` charter: nothing here is owed work.
- **The census's blind-spot statement** is now on the file rather than
  in a closed family's comment: the reader strips comments and reads
  `pub use` statements, so a `cfg` attribute above one is invisible to
  it and the forty gated names look exactly like the ungated ones
  beside them. Stated at `FACADE_FILES`' docstring and argued at the
  roster entry.
- **The sweep took the same file.** `analysis.rs` joined
  `scripts/payload-rung-sweep.py`'s `ALL_LISTS`, because the census
  SHARES that script's resolver: a census over five lists and a sweep
  over four would be the "no two runs agree on a number" defect one
  layer up. Its `--selftest` fixture gained a fifth list and a witness
  for the new blind spot (j) — a `cfg`-gated `pub use` read as curated
  unconditionally — with the counts re-derived. Six new rungs, all
  dispositioned: `PairingViolation` argued (`NOT_CARRIED`, the analysis
  lane's interior residue), `KProbe` filed as
  `kprobe-is-a-rung-under-drive-config-on-the-analysis-list`, and four
  cross-list rows over three names ARGUED under the rule LIB-CUR8
  ratified while this branch was open — a payload whose vocabulary a
  curated list owns is spelled once on THAT list and the carrier's
  list points at it. `Dimension`, `Distribution` and
  `MeasureUnavailableAt` are the document layer's vocabulary under
  analysis-list carriers, which is that rule with the two lists
  swapped, and it is the clearest case the rule has:
  `crates/pncad/src/analysis.rs`'s own head says the split is the
  DESIGN's, document state against what is derived from it. The
  ratified prose in `document.rs` and `select.rs` said "the four
  curated lists" and now says the rule without the count.
- **Prose.** `py/analysis.rs`'s header gained the advisory lane and
  lost its stale `crates/pncad/src/analysis.rs:NN` citations;
  `docs/GUIDE.md`'s distributions rung gained the fourth door as a
  paragraph, and its closing sentence now says why the lane is un-gated
  rather than only what does not cross. `tests/test_monte_carlo.py` is
  the positive form, `ty_fixtures/legal.py` and `illegal.py` carry the
  typed shape.

## Deviations

- **`McReport.render()` is not in the brief** and is bound anyway; the
  argument is above. Nothing else on the report's Rust surface crosses
  — `serialize` and `content_key` are the goldening and cache-key
  forms, which have no Python consumer.
- **`test_north_star.py` gained nothing.** Its rosters are `dir(Node)`,
  `dir(DocEdit)` and a list of module-level names asserted ABSENT; it
  lists no module-level functions, so two new ones move no row there.
  `tests/test_stubs.py` is what pins the new names to the module.
