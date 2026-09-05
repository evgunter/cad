---
id: exception-arms-untested-while-the-list-is-empty
kind: issue
title: viewer-module-kinds' four exception self-test arms borrow a live entry, so they stop running when the list empties
status: open
opened: 2026-09-05
---


Disclosed by the unit that emptied the list
(`pick-and-parts-name-the-session-driver`, Ev's hoist ruling on #1883).

## What happens

`scripts/gates/viewer-module-kinds.sh`'s self-test has five arms for
the `VOCAB_EXCEPTIONS` machinery. Four of them read
`VOCAB_EXCEPTIONS[0]` — a LIVE entry — and plant their defect into
whichever file that entry names: a sixth site appended, a site
stripped, a different forbidden name added, the header's claim
removed. With the list empty they have no subject, so they are guarded
off and the machinery they cover is unexercised.

The fifth arm needs no entry (a module writing ITSELF a permission)
and still runs, and every driver-name arm is now stronger than it was:
each is a vocabulary naming a driver with no exemption in force at all,
which is the state the gate exists for.

## Why it is a real gap and not bookkeeping

The exemption machinery is still in the gate, documented as available
for the next seam that needs one. A future lane adding an entry would
be relying on code that no self-test has run since this PR — and the
arms are the ones that make the exemption SITE-granular, which is the
whole property `work/code-quality/D103.md` is weighing.

## What resolving it looks like

The self-test should plant its own exempted fixture rather than borrow
whatever the tree is currently wrong about: a module written into the
clean fixture that names a driver once, with the header claim, plus an
entry for it. That needs `VOCAB_EXCEPTIONS` to be overridable for the
self-test's child runs (`gate_selftest_case` re-executes `"$0" --root
$tmp`, so an environment variable the array falls back to is the small
version), and the four planters to aim at the fixture instead of
`[0]`.

Roughly thirty lines in a file whose subject this PR was not — the
unit was a hoist, and rewriting a self-test's fixture strategy inside
it would have been the widening the same PR declined twice. It is
cheap and self-contained for whoever takes it.

## One thing already fixed, not deferred

Emptying the list also exposed a latent crash: with no hits at all the
union pipeline's `grep -v` matched nothing, exited 1, and under
`set -euo pipefail` killed the gate with **exit 1 and no diagnosis** —
a gate that could not pass a clean tree, indistinguishable on CI from a
real finding. It survived because the clean fixture plants the exempted
files, so every run the gate had ever seen had at least one hit. That
line now ends in `|| true` with the reason written beside it.
