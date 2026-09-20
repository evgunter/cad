---
id: suite-headers-instruct-on-ignored-rows-they-no-longer-have
kind: issue
title: Suite headers carry standing instructions about ignored rows the file no longer has
status: open
opened: 2026-09-20
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
- **Eleven more files match the shape and were NOT read**:
  `crates/editor-core/tests/m10_10_pins_interval.rs`,
  `m10_6_r1_probes_interval.rs`, `m10_8_pins_interval.rs`,
  `m10_9_pins_interval.rs`; `crates/geom-core/src/linalg/mat.rs`,
  `crates/geom-core/tests/review_m5_pr2_scratch.rs`,
  `crates/geom-core/tests/ring_interval_fuzz.rs`;
  `crates/step-import/tests/probe_knot.rs`,
  `crates/test-utils/src/roster.rs`,
  `interval-transcendentals/tests/review_fuzz_exact.rs`,
  `tools/k-lint/src/lib.rs`.
- **Importance**: medium, and the instrument is why this is a row
  rather than a sweep. It cannot tell a stale positive from a true
  negative: a file saying *"nothing here is `#[ignore]`d"* and a file
  saying *"two rows here are `#[ignore]`d"* match identically when the
  file has no attribute. Every one of the eleven has to be read.
- **Confidence**: sure about the two members; the eleven are
  candidates, not findings.
- **Instrument**: over every tracked `*.rs`, count `^\s*#\[ignore`
  attributes and `^\s*//[/!].*#\[ignore` prose mentions, and name the
  files with prose and no attribute. Blind spots: a prose mention
  spelled without the brackets ("the ignored row below"), an attribute
  written on the same line as the `fn`, and a header whose claim is
  about a DIFFERENT file's rows.
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
