---
id: kprobe-is-a-rung-under-drive-config-on-the-analysis-list
kind: issue
title: KProbe is a payload rung under DriveConfig, now that the sweep reads the analysis list
status: open
opened: 2026-09-09
---



`crates/pncad/src/analysis.rs` joined `scripts/payload-rung-sweep.py`'s
curated lists at LIB-MC, and the uncurated column gained one row that
is not argued anywhere in the tree:

| payload | at | carrier | carrier on | carrier at |
| --- | --- | --- | --- | --- |
| `KProbe` | `crates/editor-core/src/drive.rs:319` | `DriveConfig` | analysis | `crates/editor-core/src/drive.rs:141` |

Reproduce with `python3 scripts/payload-rung-sweep.py`, the NARROWED
table.

The other new uncurated row, `PairingViolation`, is already argued —
`crates/pncad/tests/all.rs`'s `NOT_CARRIED` lists it as the analysis
lane's interior residue — and its disposition says so. This one is not
on that roster and has no counterpart argument at the site.

## Why it is a question and not obviously a defect

`KProbe` is `DriveConfig`'s `probe` dial: whether the E6 driver replays
each certified leaf into the `k_stats` funnel. Both the field and the
type are behind `#[cfg(feature = "probe")]`, and `pncad` HAS a `probe`
feature (`crates/pncad/Cargo.toml`) that forwards it, so a
`pncad --features interval,probe` consumer holds a `DriveConfig` with a
field whose type `pncad::analysis` does not carry. `..DriveConfig::default()`
still compiles; SETTING the dial does not.

Two readings, and the sweep cannot choose between them:

1. The dial's only consumer is the K experiment's harness, which is a
   workspace member and reaches `editor_core` directly. Then the
   non-carriage is right and what is missing is a sentence saying so,
   next to the `interval`-gated `pub use` list in
   `crates/pncad/src/analysis.rs`.
2. A curated carrier owes its payload, the rule
   `crates/pncad/src/document.rs` states for `VerbKind`/`Arity`. Then
   `KProbe` belongs on the analysis list beside `DriveConfig`, gated on
   `probe` the way its neighbours are gated on `interval` — which would
   be the façade's first `probe`-conditional door and is a decision of
   its own.

## What a unit closing it would decide

Which reading holds, and it is a curation act either way: a name joins
the list, or the argument is written where the gate is. It also settles
the general question the analysis list newly raises — whether a curated
list owes payloads that exist only in a feature unification the shipped
artifact is not built with. `crates/pncad-py` is that artifact and it
answers no for `interval`; nothing has answered for `probe`.
