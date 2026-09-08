---
id: demos-tour-is-unowned-ground
kind: issue
title: demos/tour is in no program's paths, and three Track X rows wait in code-quality for ground nobody owns
status: open
opened: 2026-09-08
---


## What

`demos/tour`'s Rust scene code is in **no live program's `paths`**, and three
Track X rows are waiting in `work/code-quality/` for that ground.

Verified 2026-09-08:

- `work/code-quality/program.md` has **no `paths:` field at all** — it carries
  `blocks:` instead, because it is the waiting room where *"a structural
  finding waits until a program claims it."* Its `blocks:` includes
  `X D400-D419 S470-S489`.
- `work/ciw/program.md`'s `paths` covers `demos/*.sh` and `demos/*.py` only —
  not the Rust scenes.
- `docs/WORK-TRACKS-2026-09.md:517` gives SHELL's territory as including
  *"`demos/tour` scenes **by courtesy of** Track X"*. Courtesy, not ownership;
  and `work/shell/program.md` contains the string `demos` zero times.

**The three rows.** `grep -rl '^track: X' work/` returns five files, all in
`work/code-quality/`: `D403` (open — teapot's wall probes still run only in
the render walk), `tour-scenes-lift-componentwise-not-through-map` (open),
`D79` (parked), and two closed. So two open rows and one parked sit in a
112-file waiting room for territory no program's `paths` covers.

## Why it is filed, and by whom

Residue of a correction Ev forced on METER's exit walk (#2212). The walk had
ruled `C15` to code-quality Track X on the claim that *"`demos/` is
code-quality Track X's"*. Ev questioned it — *"huh i thought track X had
closed"* — the claim was checked and found false, and `C15` went to METER's
successor instead.

**`C15` escaped the waiting room; these three did not.** That is the part the
correction did not fix, and it is outside METER's fence, so METER reports it
rather than ruling on it. Filed here under the precedent of
`stale-track-t-citations-in-fillet-and-cert` — a finding this program holds and
routes rather than fixes, since META's `keep_out` forbids editing another
program's slate across the fence.

## What a ruling would decide

Whether `demos/tour` gets an owner at all, and if so whether the three rows
follow it. Track X is alive but thin; the alternative readings are that it
should close and its rows re-home, or that a program's `paths` should grow to
cover the scenes. Neither is this program's call.
