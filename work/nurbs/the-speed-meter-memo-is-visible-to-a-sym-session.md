---
id: the-speed-meter-memo-is-visible-to-a-sym-session
kind: issue
title: The speed-meter memo is not invisible in Sym: a meter first read before a session freezes in it
status: closed
opened: 2026-10-12
priority: P3
cost: E
branch: nurbs/memo-sym-invisible
closed: 2026-10-12
pr: 4869
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

## Closed (2026-10-12)

The second fix: the memo is keyed to "no session". While a symbolic
session is installed, `speed_lower_bound` computes the meter afresh and
neither reads nor fills the memo. A session never reads a value minted
before it, and nothing a session mints outlives it. The test is
`geom/tests/curves/swaying_corner_meter.rs`'s
`a_meter_read_before_a_session_is_not_served_inside_it`. It decides over
the meter of two equal `Sym` curves, one of them read before the
session. On main they freeze 1 and 6 nodes; with the fix, both freeze
the same.

The first fix (state the exception) would have kept a `Sym` session's
result dependent on whether its carrier's meter had been read before
it. That breaks the memo's one contract.

Outside a session nothing changes: the 4803 `Interval` loft table reads
the same as main within the box's noise. Inside one, the digest's wall
time is unchanged (one binary, memo on against off).

The digest's `Sym` `frozen` column did not go back to 1060 / 1177; it
went to 1914 / 2094. The memo had hidden PR 4834's change to the meter
from the digest. The re-cut log names each cause, and the measurement is
also on `work/inlane/held-value-equalities-decided-on-the-value-channel.md`.
PR 4814's "is not measured" note is corrected: with the meter computed
inside the session on the 4803 merge, the column reads 1060 / 1177.

