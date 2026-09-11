---
id: LIB-SWEEP
kind: unit
title: the payload-rung sweep as a committed script over all four curated lists
status: closed
opened: 2026-09-09
branch: lib/sweep
refs: [payload-rung-sweep-is-prose-and-a-third-run-disagrees, cross-list-payload-rungs-under-document-only-carriers]
pr: 2245
closed: 2026-09-09
---


Closes `payload-rung-sweep-is-prose-and-a-third-run-disagrees`: the
sweep is a committed script, run rather than re-derived, reading all
four curated façade lists and saying which list each side of a row is
on.

## Delivered

- **`scripts/payload-rung-sweep.py`** — the pattern implemented once.
  Four counts (curated / declared / raw / narrowed), the narrowed
  table with `file:line` per row in a deterministic order, a `--json`
  form, a `--check` pin and a `--selftest` fixture battery. Crate-aware
  through the `pub use` statement's own root, so a carrier resolves to
  the declaration in the crate the list named.
- **The fourth list read, and the two states separated.** The curated
  set is the union of `document`, `select`, `prelude` and `profile`,
  and a row records which list(s) carry the payload and which carry
  the carrier. Three categories fall out where a three-list run had
  one: curated beside its carrier (not a rung, dropped), UNCURATED (on
  no list at all), and CROSS-LIST (curated, on no list that carries its
  carrier). `--lists` reproduces the three-list definition, which is
  how the earlier runs' numbers are bounded.
- **Blind spots (a)–(i)** stated as comments beside the code that has
  each, and indexed in the module docstring: (a), (f), (g) CLOSED here,
  (e) narrowed to macro-MINTED names, (b), (c), (d), (h) open, and a
  new (i) — registry dependencies are outside the path closure.
- **The disposition tables as data**, `DISPOSITIONS` and
  `CROSS_LIST_DISPOSITIONS`, each name citing the home of its argument.
  `--check` requires the narrowed names to equal the tables' keys in
  both directions, so a new rung reds the PR that lands it and a stale
  row reds too.
- **The CI row**, in `discipline` on both halves, running the selftest
  and then `--check`. Sited there because the sweep's inputs are the
  crates' sources and the script, which is the change set a
  `run_build` job runs on; mirrored in `local-scripts/ci-local.sh`
  under a `HOSTED MIRROR` marker, which
  `scripts/check-ci-mirror-parity.py` claims 1 and 4 require.
- **One residue filed**:
  `cross-list-payload-rungs-under-document-only-carriers` — the four
  cross-list rows over `EntityKind` and `SplitHalf`, which no
  three-list run could see. The uncurated column had no new row.

## Decisions and deviations

- **The refusal filter is a NAME test**, `*Error` / `*Refusal` /
  `*Fault`, not a `Display`-impl test. Stated in the code with its
  measurement: every enum in the dependency set with one of those
  suffixes is a refusal and none is spelled otherwise, while five
  discriminants a curation pass CARRIED have `Display` impls, so a
  `Display` test does not separate the two. Earlier runs made this
  judgement by hand and by class; this makes it mechanical.
  DEVIATION FROM THE BRIEF'S ARGUED LIST, in the same direction: the
  suffix set admits `LeverRefusal` and `MintRefusal`, which the
  earlier runs disposed of as the `Display`-carrying class and which
  the brief's list therefore does not name. They are dropped by the
  filter, not by a table row.
- **Blind spot (b) is left OPEN**, as the brief directs, so
  `CarrierRelation` stays in `DISPOSITIONS` as a `false-positive` row
  citing the alias. The alias arm is mechanically closable — a
  `pub use path::X as Y` in the dependency set says `Y` names `X`'s
  declaration — and the code says so where the blind spot is stated.
  Not done here: it changes the narrowed set, which is a decision for
  a unit that means to make it.
- **`--check` pins the four-list sweep only.** `--lists` is for
  reproducing a historical definition, and a pin over a definition
  nothing else uses would be a second baseline to keep.
- Sources are read `#[cfg]`-gated code included: a payload behind
  `#[cfg(feature = "interval")]` is a real payload in the lane that
  builds it, and dropping gated code would make the sweep's answer
  depend on a feature selection it does not take.
