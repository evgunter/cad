---
id: py-target-vocabulary-cannot-spell-the-declared-tangent-arrival
kind: issue
title: PyTarget and Tgt carry Point and Start only, so the Python authoring surface cannot express StartArriving, and nothing says that is deliberate
status: open
opened: 2026-09-12
---



(WIRE orchestrator) Reported by D364's lane (PR 2445) from its
`Target::Start` sweep, outside that unit's fence. Verified against the
tree before filing.

`profile::Target` has three forms — `Point`, `Start` and
`StartArriving`, the last ruled by Ev in-chat 2026-09-02 as the seam's
DECLARED tangent joint ("the seam is the one junction whose arriving
leg is the later-authored one, so the declaration that elsewhere rides
the departing leg rides the target here", `crates/profile/src/path/program.rs:116-135`).

The Python binding carries two of the three, twice:

- `PyTarget` (`crates/pncad-py/src/py/path.rs:225-228`) — `Point`,
  `Start(StartToken)`. Its doc says *"A leg's target: an authored
  absolute point, or `Start`"*, which is an accurate description of
  what it holds and a stale one of the vocabulary it extracts.
- `Tgt` (`:255-258`) — `Point`, `Start`, the arc modes' endpoint.
- `Tgt::of` (`:261-267`) matches `PyTarget` exhaustively, so **nothing
  forces either list**: it compiles, and will keep compiling, however
  the kernel vocabulary grows.

`StartArriving` appears nowhere under `crates/pncad-py/`
(`grep -rn StartArriving crates/pncad-py/` — no hits), so a Python
author cannot write the declared tangent arrival at all. The kernel
CHECKS the arriving direction against `Start`'s own and refuses a seam
that contradicts it, so the Python surface is not merely less
convenient here — it cannot express one of the two things the ruling
made distinct.

## What is owed, and the part that is a question

Carrying the form through is mechanical (a `PyTarget` variant, a `Tgt`
variant, `Tgt::of`'s arm, the `.pyi`). What is not mechanical is
whether the omission is deliberate: it may be that the Python authoring
surface deliberately offers the undeclared close only, and the seam
declaration is a typed-surface affair. Nothing at either site says so,
which is the filing's real subject — **an absence with no statement is
indistinguishable from an oversight**, and this one has survived a
ruling.

Whichever way it goes, the structural half stands on its own: two
hand-written spellings of a kernel vocabulary with no anchor. D364
(PR 2445) gives `profile::Target` a `TargetKind` with an `ALL`
projected from the variant declaration; once that is on main these two
lists can be anchored on it the way `editor-core`'s census now is, and
a form the kernel gains stops the binding compiling instead of quietly
not being spellable.
