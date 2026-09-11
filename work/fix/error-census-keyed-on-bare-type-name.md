---
id: error-census-keyed-on-bare-type-name
kind: issue
title: The error-type census is keyed on the bare type name, which is ambiguous at seven names
status: open
opened: 2026-09-04
refs: [1111, 1741]
---


Filed from #1741 (the viewer cut of
`error-types-with-no-display-class`). **No live defect found** — this
is a finding about the METHOD, not about a rendering.

## Why it matters

The list in #1111 was produced by a census keyed on the bare type
name, and so was the re-sweep in #1741 that refuted most of it. The
same key produced both the claim and its correction, so its failure
modes are worth stating before it is used a third time.

At seven names the bare key is ambiguous, in two opposite directions.

**One key, two distinct types** (a false merge — a `Display` found on
either satisfies the census for both):

| name | declared at |
|---|---|
| `BlendError` | `sweep/src/blend/mod.rs`, `viewer/src/blend.rs` |
| `ComposeError` | `geom-core/src/spline/compose.rs`, `geom/src/curves/compose.rs` |
| `LiftRefusal` | `editor-core/src/stackup.rs`, `profile/src/lift.rs` |
| `ReplayError` | `profile/src/path/program.rs`, `viewer/src/history.rs` |
| `SplitError` | `editor-core/src/refactor.rs`, `topo/src/splitting/mod.rs` |

`ReplayError` is on #1111's own list, and that entry resolves to two
distinct types — which is exactly the hazard, since only one of them
was ever the subject.

**One type, two public paths** (a false duplicate — the same type
counted twice, inflating a hit list):

| name | reachable at |
|---|---|
| `PathError` | `profile`'s root and `pncad::prelude` |
| `Refusal` | `viewer::session` and `viewer`'s root re-export |

## The fix

Re-run the census keyed on `crate::path::Type` rather than on the bare
identifier, resolving re-exports to the declaring path. Both directions
close at once: distinct types stop sharing a key, and one type stops
appearing under two.

Worth doing before the next sweep of this class, not urgently — the
known consumers of the old key have both been checked by hand.
