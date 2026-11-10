---
id: a-question-mark-inside-a-rung-is-registered-nowhere
kind: issue
title: A ? on a band reading inside a climb's rung closure is registered by no gate
status: open
opened: 2026-11-10
priority: P3
cost: M
---


Moved from `question-mark-ladders-the-gate-registered` when it closed (QML T6, `onecopy/refusal-standing`), from PR 6675's review.

A port moves a loop's reads inside one rung of a `climb`. A `?` that stays inside that rung's reader is registered nowhere afterwards: the register reads `?`s in a search loop, and the rung's reader is a closure the gate does not see as one. This is a second hiding place for the T1 review's F3 class (a sequential `?` outside any loop).

**Today's in-rung `?`s are premises.** Each is needed by the reads after it, so the first refusal is the rung's own refusal, not a kept reading another rung answers past:
- `boolean::sectors::crescent`'s pair: `parallel_same(..)?`, then `bool_sector_crescent_across`, then `coincident_run(..)?`, each read only where the one before it held;
- `ring_path::path_parity`'s pair: `misses_in_plane`, `segment_misses` and `spline_crossings`, each read on what the pair's previous reads placed;
- `shell::walls_cross`: no `?` inside the rung; its pieces are built before the climb from topology only.

**What would not be a premise:** a later conjunct inside a rung that is independent of the earlier ones, so it could answer for the rung on its own. That is a search inside the rung, and it is the F3 shape again.

**A cheap gate shape, proposed and not built:** inside the closure passed to `climb`/`climb_evidence`/`climb_agreeing`, count `?` on a band reading (the register's own matcher, run on the closure's body) and register each closure's count with a disposition (`PREMISE`: each read needs the one before; `SEARCH`: port it). The closure is bounded by the call's argument list, so the existing statement view finds it, and the register file is `escalation-question-marks-callee.counts`'s format with a `rung:` prefix.

Since then QML T5 put `held_cut`'s three reads inside its rung, each a premise of the next (`nested`, then `on_arc`, then `tied_held`).

## Done when

The `rung:` register exists and reds an unregistered in-rung `?`, with each of today's sites dispositioned `PREMISE` or `SEARCH`, or a reviewer rules the shape not worth a gate and this row says why.
