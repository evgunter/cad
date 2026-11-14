---
id: the-composed-dies-step-bytes-moved-at-6482-and-are-pinned-unjudged
kind: issue
title: The composed die's STEP bytes moved at PR 6482 and nobody has judged it: composed_die.step is pinned at main's bytes
status: open
opened: 2026-11-14
priority: P2
cost: E
---


Filed by GREEN's `green/goldens` lane, from nightly 43371638371 (main `e36675c1`). The second half of `work/green/main-red-the-filleted-die-moved-in-seat4-and-the-step-export-fixture.md`, which GREEN closed by pinning.

**What moved.** `crates/step-export/tests/export.rs` `committed_fixtures_are_byte_golden` stops at the first fixture that drifts, so `filleted_die.step` hid this one: regenerating every fixture on main drifts `composed_die.step` too.

**The merge.** PR 6482 (`onecopy/angle-reads`, merge `a6f0b5e99c`, 2026-11-04), bisected first-parent by `onecopy/main-reds-triage` on 2026-11-09 from `f1ec7c3f93`, the last first-parent merge that wrote the fixture: it regenerates byte-equal there and drifts from `a6f0b5e99c` on. PR 6482's description mentions no STEP fixture. Its `m10_p_fence` note records 16 enclosure bounds on `die_composed` moving by at most `7.2e-16`, which may be the same move reaching the written bytes.

**What GREEN did.** `composed_die.step` is regenerated at main's bytes with `cargo run -p step-export --example export_fixtures -- crates/step-export/tests/fixtures`, so the row is green and reds on the next move. The bytes that moved are in the PR that filed this row.

**Owed.** Say whether the composed die's new bytes are only that rounding. If they are, nothing more is owed and this row closes with that sentence; if something else moved, judge it.

Reproduce: `cargo run -p step-export --example export_fixtures -- /tmp/out` at `a6f0b5e99c^1` and at `a6f0b5e99c`, and diff `composed_die.step`.
