---
id: cross-crate-inline-expr-literal-sites-outside-the-viewer-suites
kind: issue
title: The inline Expr::literal class runs past crates/viewer into sixty-three files
status: closed
opened: 2026-09-20
priority: P4
cost: H
closed: 2026-09-27
branch: dup/b7-b
---


## Finding

The row `viewer-tests-bypass-the-shared-literal-doors` named *"every
other crate's suites"* as its instrument's blind spot and cited
*"forty-five files"* as evidence that the class runs wider. That
citation was published as a caveat rather than run; this is the row it
owed.

- **The measurement, re-taken at `cd9fdfd6b`** over every tracked file
  with no path argument: `git grep -l 'Dimension::Length).expect\|
  Dimension::Scalar).expect\|Dimension::Angle).expect' -- crates/`,
  minus `crates/viewer/`, names **63 files** — not forty-five. The
  bulk is `crates/editor-core/`, split between `src/` (the kernel's
  own doors) and `tests/corpus/` (the shared document corpus), with
  `crates/pncad/` and `crates/profile/` behind them.
- **What makes this a different question from the viewer's**, and why
  it is a row rather than a bigger sweep: `crates/viewer/tests/`'s
  members bypassed a door that **already existed in the same binary**
  (`common::{len, scl, ang}`), so the fold was a substitution. Outside
  that crate there is no such home, and the sites are split between
  `src/` — where an `Expr::literal` call is the kernel doing its job,
  not a test restating a fixture — and three separate `tests/` trees
  that would each need their own. **Deciding which of the 63 files
  hold members at all is the first instrument**, and a count of
  members should not be published before it runs.
- **Importance**: low. No oracle: a hand-written
  `Expr::literal(v, Dimension::Length).expect(..)` and a door over it
  are the same call.
- **Instrument, and its blind spot**: the `-l` grep above is
  **line-shaped and `.expect`-shaped**. It misses a call whose
  argument list wraps (the viewer unit found one such site, in
  `frame_policy.rs`, written across four lines — so the shape exists
  in this tree and this census certainly undercounts), a site using
  `.ok()`, `?` or a match instead of `.expect`, and
  `Expr::literal_with_unit`, which is a different door (its
  `crates/viewer/tests/` members are folded onto `common::len_mm`).
  It is a FILE count, not a site count, and the two are not
  interchangeable.
- **Raised by**: the S-DUP lane closing the four viewer-suite door
  rows, in its fix pass, 2026-09-20.

## Why this sits on S-DUP's slate

The ground is three crates' `tests/` trees plus one `src/` test module,
each claimed by several programs, so there is no single ground-owner to
file it with, and one construction spelled more than once is S-DUP's
charter. Any claimant may take it by `git mv`.


## Closed 2026-09-27 — re-taken at `360eb7320`, every member folded

### The census, and which files hold members

**The row's own instrument, re-run with no path claim beyond its
own:** `git grep -l 'Dimension::Length).expect\|…Scalar…\|…Angle…'
-- crates/` minus `crates/viewer/` names **63 files** at `360eb7320`
— the row's figure held: `editor-core/tests` 61, `pncad/tests/all.rs`,
`pncad-py/src/tests.rs`. The same grep with NO path argument adds
`demos/tour` 9, `docs/` 3 and two `work/` rows. `crates/profile/`,
which the row put third, holds **none** at this base, by this grep or
the one below.

**The instrument that decided membership** is denominator-first
(plan item 7): every textual `literal(` or `literal_with_unit(` call in
every tracked `.rs` outside `crates/viewer/`, its argument list
balanced across lines (the row's wrapped-list blind spot), the call
that consumes its `Result` read off after the closing paren (its
`.ok()` / `?` / match blind spot), `literal_with_unit` in the same
pass. **424 calls**. Every one is dispositioned:

| calls | where | disposition |
| --- | --- | --- |
| 344 in 95 files | `editor-core/tests` 307 (suites, `corpus/`, `fixture/mod.rs`), `editor-core/src` unit-test modules 10 (`edit.rs` 6 wrapped path-qualified inline, `mate/member.rs` 3, `param_source.rs` 1), `pncad/tests/all.rs` 17, `pncad-py/src/tests.rs` 10 | **folded**: 114 private wrappers deleted (fns and closures named `len`, `lit`, `lit_len`, `scalar`, `sca`, `scl2`, `angle`, …) and 230 inline calls rewritten |
| 7 | `u8a_parse.rs` 3, `switch_display_units.rs` 4 | **kept by judgement** (plan item 6): each row's subject is what the bare constructor records — the canonical unit a plain `Expr::literal` carries, set against `literal_with_unit` and the parser beside it — so the constructor is the thing the row names |
| 15 | `editor-core/src` library code: `expr.rs` 3 (the constructor), `parse.rs` 3, `persist/wire.rs` 1, `measure.rs` 1, `program.rs` 5 (the `Result`-returning authoring helpers and two `?`), `param_source.rs` 2 (`literal_with_unit` in mm and cm, the unit being the subject) | **not members**: the kernel doing its job |
| 6 | `pncad-py/src/py/` | **not members**: the FFI door building an `Expr` from Python input |
| 31 (`editor-core/tests` 22, `pncad-py/src/tests.rs` 7, `pncad/tests/all.rs` 2) | refusal probes (`NaN`, `±∞`, `Count`, `f64::from_bits(u64::MAX)`) with `unwrap_err` / `expect_err` / `is_err` / no tail, a runtime dimension (`m4_pr6_review_probes`, `switch_program_vocabulary`, `pncad-py` `1511`), `literal_with_unit` rows about notation (`pirad_wire`, `wire_rv_bytes`, `switch_display_units`), two format-string prose hits in `pncad-py/src/tests.rs`, and `pncad/tests/all.rs`'s two | **not members**: the door expects, and these rows are about the refusal or the unit |
| 21 | `demos/tour` | **never converted** (the program's `keep_out`) |

Outside `.rs`: `docs/` (keep_out), the Python suites' `Expr.literal(q)`
(a one-argument public door with nothing to fold), and `work/` prose.

**The homes.** `editor_core::test_support` gains `len` / `ang` / `scl`
(behind the existing `test-support` feature; no `Tol` anywhere near
them, and `witness-not-ambient.sh` passes); `tests/fixture` re-exports
them, so the suites, the corpus, `viewer`'s binary that mounts both
trees, and the `src` unit-test modules read ONE definition. `fixture`'s
own `xform` spelled the angle longhand beside `ang` and is folded too.
`pncad/tests/all.rs` and `pncad-py/src/tests.rs` each get one
file-level pair: neither crate has a test home, and giving one is the
design question filed as
`facade-box-document-fixture-spelled-in-pncad-and-pncad-py`. The two
nested façade modules in `all.rs` import the pair through `super`,
which the file's own use-root guard now admits: `super` names this
file's items and so no crate, and a kernel path behind it is still the
guard's second check.

**Gated suites.** `m4_pr1_eval` now reads the doors through `fixture`,
so its `gated_to!` marker names `tests/fixture/` (the gate required
it) and `src/test_support.rs`; six more gated suites that already named
`tests/fixture/` and read the doors through it name
`src/test_support.rs` too.

### The proof

Local, opt-level 1 (the hosted archive's setting). Baseline **2459
passed**: `editor-core` lib 233, `editor-core::all` 2033 (94 skipped:
`#[ignore]`d evidence rows), `pncad::all` 67, `pncad-py` lib 126. Every
row below sums to its binaries' baseline; each plant restored its
file's bytes and was checked against `git diff HEAD`.

| plant | direction | passed / failed | per binary |
| --- | --- | --- | --- |
| `test_support::len(m) → m + 1` | grow: every length gains a metre | 1762 / 504 | lib 233/0, all 1529/504 |
| `len → panic!` | control: reach | 520 / 1746 | lib 220/13, all 300/1733 |
| `ang(r) → r + 1` | grow — but a rigid rotation of every `Transform` and a rotation of every tube window, so rotation-invariant rows cannot see it | 2066 / 200 | lib 233/0, all 1833/200 |
| `ang → panic!` | control: reach | 1725 / 541 | lib 225/8, all 1500/533 |
| `scl(v) → v + 1` | grow: a unit axis stops being one, a zero component becomes one | 1784 / 482 | lib 233/0, all 1551/480, **2 killed as hung** (`m10_4_stackup_interval::a_band_contributor_…` at 1200 s against 33 s baseline, `m10_sym_drive_memo_interval::a_memo_from_one_drive_…` at 870 s against 153 s) |
| `scl → panic!` | control: reach | 566 / 1700 | lib 225/8, all 341/1692 |
| `pncad` `len → m + 1` | grow | 57 / 10 | both nested modules red |
| `pncad` `scl → v + 1` | grow | 62 / 5 | |
| `pncad-py` `len → m + 1` | grow | 126 / 0 | dark: see the control |
| `pncad-py` `len → panic!` | control: reach | 115 / 11 | all four folded fixtures' rows |
| `pncad-py` `scl → v + 1` | grow | 125 / 1 | `expression_evaluation_tags_are_stable` — the pole's divisor stops being zero |

**Reached but value-dark**, read by pairing each value plant's green
suites against its panic control: 17 folded suites under `len`, 18
under `ang` and 12 under `scl` stay green under the value plant and red
under the panic, and so do the three `src` unit-test modules (`edit`
2, `mate::member` 8, `param_source` 3 under the `len` panic). Called
and unasserted at those values — the rows are about tags, structure,
refusals or rotation-invariant measures — which is what the folded
sites were before the fold too. The `corpus/` modules hold no rows of
their own; their reach is read through the suites that build their
documents (`m4_pr8_corpus`, `m5_pr11_corpus_curved`,
`m10_di_dual_corpus`, `cert_m2r1_corpus`, … — red under every panic
control). **Not run** by any plant: the
`#[ignore]`d evidence suites (`m10_10_r1`, `m10_10_r2`, `m10_8_r1`,
most of `m10_9_r1`) and the `probe`-gated ones (`m10_3_driver_k_probe`,
`m10_7_r1_census_probe`, and `m10_7_plate`, whose only consumer is the
latter); the `probe` build was clippy'd clean, not run.

### What this found that it did not fold

- `editor-core-suites-hand-build-the-world-xy-frame` — 37 longhand
  world-xy frames in 23 suites beside `fixture::xy_frame`, whose
  literals this unit folded and whose frames it left.
- `viewer-test-support-restates-editor-core-literal-doors` — `viewer`
  now defines the same three doors a second time; `crates/viewer/` was
  outside this unit's fence.
- `facade-box-document-fixture-spelled-in-pncad-and-pncad-py` — the
  one-box document fixture written line for line in two crates with no
  home either can reach.
