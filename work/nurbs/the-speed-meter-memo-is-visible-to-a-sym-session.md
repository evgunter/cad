---
id: the-speed-meter-memo-is-visible-to-a-sym-session
kind: issue
title: The speed-meter memo is not invisible in Sym: a meter first read before a session freezes in it
status: open
opened: 2026-10-12
priority: P3
cost: E
---

Found by CLEAVE's main-red lane while it traced the sym `frozen` digest
move that PR 4814 re-cut.

## What

PR 4803 (`nurbs/speed-meter-memo`) caches
`NurbsCurve*::speed_lower_bound` on the curve on first read
(`crates/geom/src/curves/nurbs.rs`). The loft build reads the meter
before any symbolic session exists, so the cached `InfSpeed<Sym>` is
minted outside a session and, per `geom-core/src/sym.rs` ("No session,
no tier"), freezes to an unknown of its own in every later session.
Measured at `880e5d2508`: `validate_geometric` alone on a freshly built
body reads `frozen` 1065 / 1181 (`sym_arc_loft` / `sym_thin_strip`),
against 1060 / 1177 at its first parent, and so does a second
validation of the same body. This is what moved the digest; PR 4814's
re-cut says "which freeze cause each added count is, is not measured".

No decision moved, and freezing loses no soundness: the meter is now a
build-time value of the carrier, like its control points. But the test
`the_meter_memo_is_invisible_and_never_inherited` claims invisibility,
and in `Sym` whether the meter was first read before a session changes
that session's frozen set, which could in principle lose a cancellation
through the meter's structure.

## Fix

Either state the Sym exception at the memo and in that test (the memo
is a build-time value, frozen like the control points), or key the
memo so a session-minted read is not served a pre-session value. Say
which in the PR, and correct 4814's "is not measured" note.
