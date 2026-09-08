# EVAL-9 — every slot feeds its nominal beside its lane bits: the content key holds everything a reader of the node reads (spec)

**Program:** EVAL (`work/eval/plan.md`, unit 9). **Item:**
`work/eval/interval-content-key-hashes-bits-the-pre-pass-does-not-read.md`
(EVAL-7's review finding; pre-existing).
**Track:** E with a **correctness arm**: the unit changes every content
key (a format bump) and closes a memo hole; one style review and one
correctness review, fix pass, record-at-merge. No A/B draw.
**Branch:** `eval/9-nominal-in-the-key`. **Difficulty:** S–M.

## The claim

**A content key must fix every input a reader of the node reads, and at
the interval scalar it does not fix the nominal.** `content_key`
(`crates/editor-core/src/eval/mod.rs`, EVAL's) feeds each slot value at
the evaluation scalar through `ContentBits` (`eval/memo.rs`, DOCM's): at
f64 the bits ARE the nominal; at `Dual64` the value half is; at
`Interval` the feed is `(lo, hi, dec)` and the nominal is not a function
of them — a nominal edit under a compensating box leaves the bits equal.
Two readers of a frame node read the nominal: `wire::profile_plane_f64`
(the pre-pass, by design at f64) and the `Pinned` op (which embeds the f64
placement whole). So at `Interval` a `SetDocParamValue` under a
compensating `param_box` hits the memo and the reused profile carries
the OLD nominal's plane (the review's probe: `u = [1, p, 0]`, `p`
nominal `0.0` box `[-0.25, 0.25]` as prior, then `p = 0.5` box
`[-0.75, -0.25]`: `recomputed = 0`, keys equal, plane = identity
instead of `(0.894, 0.447, …)`). Pre-existing; reproduces on the base
before EVAL-7.

## What lands

1. **The rule, stated once at `tag::format::VERSION`'s doc and applied
   uniformly**: every slot value feeds its lane bits AND its nominal f64
   bits. The nominal is the slot's expression evaluated in the
   document's f64 parameter environment (`doc.param_env::<f64>()`, the
   environment the pre-pass already uses), fed as `write_f64_bits`
   immediately after the lane feed under one new word of the `program`
   or a new `slot` group in `mod tag` (EVAL-2's declaration module). At
   f64 the nominal duplicates the bits (harmless, and what makes the
   rule uniform rather than lane-conditional); at `Dual64` it duplicates
   the value half; at `Interval` it is the missing input.
2. **Format bump** `tag::format::VERSION` 6 → 7, with the one-sentence
   reason the rule at the declaration asks for (an existing node kind's
   stream gains a word every node writes into: every key moves, no
   pre-bump memo entry is reused).
3. **The red-first row**: the review's probe, adopted verbatim as a test
   in `crates/editor-core/tests/` (interval feature): at the head the
   second evaluation MISSES (`recomputed` counts the profile) and its
   plane equals the cold run's; the row is written to fail on the base
   (the PR body shows it red at the merge base, green at the head).
4. **Every key moves, and only by the bump**: a base-vs-head key dump
   (EVAL-2's shape: `(lane, doc, node, content_key, naming_key)`, corpus
   at f64, bumped f64, `Dual64`, interval) shows every key different
   (the bump) — and a SECOND dump with the head's format word forced to
   6 in a throwaway shows every f64 and `Dual64` key identical to the
   base (the nominal adds no information there) and only interval keys
   moved. That second dump is the proof the rule is uniform and the
   change is the interval one; put both row counts in the PR body.
5. **Memo behaviour preserved**: the standing memo rows
   (`switch_program_key`, `msolve4_mate_memo`, `asm_upd_pin_update`, the
   kstats hit rows) green; a parameter edit that does not move the
   nominal still hits (a row: `SetDocParamValue p = 0.0` to itself under
   a different box does NOT hit at interval — the box is a real input —
   and the same nominal under the same box does).
6. **`eval/memo.rs` is DOCM's** and this unit does not edit it: the
   nominal feed is written at `content_key`'s slot loop in `eval/mod.rs`
   (EVAL's). If the implementer finds the feed cannot be placed without
   touching `memo.rs`, STOP and report.

## Correctness claims (the correctness arm)

1. The probe row is red on the base and green on the head (the
   reviewer reproduces both).
2. The forced-version-6 dump: f64 and `Dual64` keys identical to base;
   only interval keys moved; count the moved rows.
3. Every memo row and hit row green at every lane/eps point.
4. Cost: one f64 evaluation per slot per node — measure it on the
   corpus evaluation wall time at f64 (the `perf` lane's rule: report,
   never gate) and put the number in the PR body.

## Sweep

`rg -n 'param_env::<f64>|profile_plane_f64|from_f64\(' crates/editor-core/src/eval` —
every reader of a nominal at the evaluator, each with the answer "its
input is in the key now" or a reason it need not be. Blind spot: a
nominal read through a helper that does not name `f64`.

## Review

Style lane per `docs/prompts/reviewer-style-lane.md`; correctness lane
takes claims 1–4 as MAJOR-class. Emphasis: Q4 — the file's hit-site
argument (EVAL-7's rewrite) names this hole; after this unit the
sentence must say the hole is closed and by what, and the item's
`## Closed` cites the row.

## Records at merge

`work/eval/log.md` entry with the DOCM note (`memo.rs`'s slot hashing
untouched; the feed lives at the key); the item `closed` with `pr:`;
this spec deleted per `docs/DOC-LEDGER.md`.
