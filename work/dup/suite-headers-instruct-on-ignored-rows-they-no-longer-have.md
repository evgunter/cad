---
id: suite-headers-instruct-on-ignored-rows-they-no-longer-have
kind: issue
title: Suite headers carry standing instructions about ignored rows the file no longer has
status: open
opened: 2026-09-20
priority: P4
cost: E
---


## Finding

- **The shape**: a suite's module header states that one or more of its
  rows are `#[ignore]`d and tells a future lane what to do about it
  ("un-ignore it when the fix lands"), while the file carries no
  `#[ignore]` attribute at all. The rows were promoted; the standing
  instruction was not deleted. It is a sentence nothing holds, in the
  grammar of an instruction, which is the worst combination: a reader
  acts on it.
- **Two members, read and confirmed false**, both fixed by the unit
  that found this (`crates/viewer/tests/review_gui0_r1.rs` — the header
  said `framing_at_an_extreme_aspect_should_contain_or_refuse` is
  `#[ignore]`d and RED while that row's own doc comment, ten lines
  above the `#[test]`, said it was UN-IGNORED in a fix pass and now
  gates; `crates/viewer/tests/review_gui3_r2.rs` — a whole `# Two rows
  are #[ignore]d` section over a file with none).
- **A third member, found by a differently-shaped instrument and NOT
  in this row's first candidate set**:
  `crates/sweep/tests/p1b_r1_probes.rs` — *"the probe is adopted,
  ignored against the issue, and preserved as the reproduction … **Un-
  ignore it when #1152 lands**"*, present tense, on
  `coplanar_split_products_carry_no_scaffold_at_rest`, a `#[test]` with
  no `#[ignore]` attribute. The file carries no `gated_to!` marker, so
  it runs on every code-tier run, and it is green — so either #1152
  landed and the prose rotted, or the row was never ignored. Not
  touched here: `crates/sweep/tests` is another lane's live tree.
- **The candidate set is re-taken, and most of the first one was
  benign.** The original eleven came from a line-shaped,
  bracket-requiring instrument and were published unclassified. Read at
  `ab086f8c1`, the shape splits three ways, and only the first way is
  this row:
  - **stale standing instruction** (this row): `review_gui0_r1.rs` and
    `review_gui3_r2.rs`, both fixed; `sweep/tests/p1b_r1_probes.rs`,
    open.
  - **true, past tense** — a completed promotion correctly described,
    nothing owed: `bvh/tests/ray_r2.rs`, `editor-core/tests/
    m10_6_r1_probes_interval.rs`, `step-import/tests/probe_knot.rs`,
    `geom-core/tests/review_m5_pr2_scratch.rs`,
    `geom-core/tests/ring_interval_fuzz.rs`,
    `interval-transcendentals/tests/review_fuzz_exact.rs`, and
    `review_gui0_r1.rs`'s surviving row-level sentence.
  - **a claim about ANOTHER file's ignored row**, which this row does
    not cover and no instrument here verifies:
    `editor-core/tests/m10_{8,9,10}_pins_interval.rs`,
    `geom-core/src/linalg/mat.rs`, `test-utils/src/roster.rs`,
    `tools/k-lint/src/lib.rs`.
- **Importance**: medium, and no instrument closes it. None can tell a
  stale positive from a true negative — a file saying *"nothing here is
  `#[ignore]`d"* and one saying *"two rows here are `#[ignore]`d"* match
  identically when the file carries no attribute — so every candidate
  has to be read, which is why this row's deliverable is a
  classification and not a sweep.
- **Confidence**: sure about all three members, each read. The fourteen
  candidates above are fully classified; what is NOT settled is the
  third bucket, whose members make claims about other files' rows that
  nothing here checks.
- **Instruments, and why it takes two.** (1) Line-shaped: count
  `^\s*#\[ignore` attributes against `^\s*//[/!].*#\[ignore` prose
  per file. Cheap, and it MISSES every mention that wraps or omits the
  brackets — which is how `p1b_r1_probes.rs` was missed, since its
  sentence says "ignored against the issue" with no brackets at all.
  (2) Comment-joined: concatenate each file's `//`/`//!` lines into one
  blob so a wrapped sentence is a single unit, then match
  `un-?ignore | #\[ignore | ignored (against|because|until|pending)`
  case-insensitively, restricted to files containing `#[test]` and no
  `#[ignore]`. That returns 14 files, all read. A third, matching the
  bare word `ignor(e|ed|ing)`, returns 76 and is useless: most are the
  ordinary English verb.
  **Neither can tell a stale positive from a true negative** — a file
  saying *"nothing here is ignored"* matches exactly like one saying
  *"two rows here are ignored"*. Every hit has to be read, which is why
  this row's deliverable is a classification and not a sweep. Remaining
  blind spots: a sentence using neither "ignore" nor the attribute
  ("the probe below does not gate"), and an `#[ignore]` written on the
  `fn`'s own line.
- **Raised by**: the S-DUP lane closing
  `viewer-review-suite-fixtures-have-no-oracle-role`, 2026-09-20,
  measured at `b29fe8bd1`, while restating that unit's headers.

## Why this sits on S-DUP's slate

It is not a duplication and it is not S-DUP's charter; it is method
item 12 — stale modality, text whose scope was true when written and
reads later as though it had none — found by a S-DUP unit reading
headers. It spans `crates/editor-core`, `crates/geom-core`,
`crates/step-import`, `crates/test-utils`, `crates/viewer`,
`interval-transcendentals` and `tools/k-lint`, so no one program's
ground holds it. It sits here because this program found it and is the
program whose method names the class; S-TINT is the likelier long-term
owner, and may claim it by `git mv`.
