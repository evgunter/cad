---
id: inert-allow-attributes-on-test-modules-are-house-style-and-half-are-unneeded
kind: issue
title: 124 of 257 cfg(test) modules carry allow(unwrap_used, expect_used, panic) and nobody knows how many are load-bearing
status: open
opened: 2026-09-11
refs: [2375]
---


## Finding

From the full review of WIRE's PR 2375 (S5, confidence `sure`), raised
as a **class** rather than the instance it started as.

The instance: PR 2375's new `mod tests` in
`crates/editor-core/src/placement.rs:293-294` carries
`#[allow(clippy::unwrap_used, clippy::expect_used, clippy::panic)]` and
the module contains **none of the three** — clippy with `-D warnings` is
clean with the attribute deleted. That one is being removed in the PR's
fix pass.

The class: **124 of 257 `#[cfg(test)]` modules under `crates/*/src`
carry the same attribute**, and nothing distinguishes the load-bearing
ones from the copied ones. An inert blanket `allow` is a lint the tree
has opted out of at a site that never needed it, which is exactly the
state where a *real* `unwrap` later slips in unremarked.

## Why it is in `work/issues/` and not on a program's slate

It spans every crate's `src/`, so no program's `paths` contains it and
no program's charter is obviously about it. `work/README.md` reserves
this directory for *"a finding whose owner is undecided or disputed"*,
which this is: GUARD is the gates program but its `paths` are
`scripts/gates/*` only, and a lint-hygiene sweep over 124 modules is not
a gate.

Whoever claims it MOVES the file (`git mv` plus the header edit), per
the same contract.

## What a taker owes

A mechanical check rather than a reading: for each of the 124, delete
the attribute and see whether clippy reddens. The ones that redden keep
it; the ones that do not lose it. That is the whole unit, and its value
is that the remaining attributes then MEAN something.

**What the count could not see**, stated because the sweep is otherwise
a claim: it counted `#[cfg(test)]` modules under `crates/*/src` only —
not `crates/*/tests/` (S-TINT's ground), not the workspace-excluded
roots (`benches`, `demos/tour`, `demos/wild`, `tools/*`,
`interval-transcendentals`), and not attributes written at other
granularities (on a function, or at the top of a test file).
