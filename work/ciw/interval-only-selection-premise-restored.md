---
id: interval-only-selection-premise-restored
kind: issue
title: The interval-only selection's original premise holds again; hosted keeps the whole suite
status: open
opened: 2026-09-04
parent: reinstate-full-configuration-runs
refs: [1796]
---

## The finding

`scripts/interval-only-selection.py` computes the tests the `interval`
feature ADDS — the interval-gated difference — so a run that has already
executed the default lane need not re-execute the ~93% both lanes share.
Measured on run 31665082966: of 2995 tests the interval lane executed, 214
are `#[cfg(feature = "interval")]`-gated and the other 2781 cost 4406.7 cpu-s
re-running code the default legs had already run in the same run.

Hosted CI **reverted** to the whole suite on 2026-08-22, for one reason
written at `test-interval`'s header in `.github/workflows/ci.yml`: a sampled
run drew ONE lane, so on an interval draw the default legs did not exist and
the subtracted 93% would have been gated by nothing.

**`reinstate-full-configuration-runs` removes that reason** (2026-09-04). Both
lanes run on every code-tier run again, so the overlap is once more pure
re-execution and the selection's original premise holds.

## Why the unit did not act on it

Restoring the subtraction REDUCES what a hosted run executes. The unit it
would ride was authorised to make a run gate MORE, not less, and a cost lever
pointed the other way needs its own argument — on a runner whose minutes are
free, against a saving of ~234 s of execution per eps row per lane, and with
the eps matrix multiplying whichever way it goes.

## What acting on it would need

* A measurement on the 4-vCPU runner rather than the 2-vCPU figures above:
  what the subtraction saves now, per eps leg, against the whole-suite leg's
  measured 56 s median (window 2026-09-04T04:00Z–07:52Z, n=66).
* The soundness premise is already gated and unaffected —
  `scripts/check-interval-cfg-additive.py` runs in `discipline` on every
  code-tier run.
* `scripts/interval-only-selection.py` is in **CIW's** `paths:`, but the
  hosted `test-interval` shape is a CI build knob and the boundary with
  S-TCOST's cost levers should be settled before anyone edits it.

## The state to preserve either way

`local-scripts/ci-local.sh` still uses the selection and is the script's only
caller, declared in `check-ci-mirror-parity.py`'s `MIRROR_EXEMPT` with that
reason. Nothing about the local half changes here.
