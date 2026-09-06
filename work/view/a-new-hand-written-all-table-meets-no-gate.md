---
id: a-new-hand-written-all-table-meets-no-gate
kind: issue
title: nothing mechanical stops a new hand-written const ALL table appearing in the viewer
status: open
opened: 2026-09-06
---


Filed by the `const-all` unit as the disclosed non-take its own PR
owed a schedule for (PR 2046, finding S13). §Q6: a disclosed non-take
owes a named unit, and a README paragraph is not one.

## What is and is not held

The nine converted vocabularies are held by the compiler: their `ALL`
is projected from the enum's declaration by
`crates/viewer/src/vocab.rs`'s `vocabulary!`, so a variant cannot reach
the enum without reaching the list. **Nothing holds the next one.** An
author who writes

    impl NewChoice {
        pub const ALL: [Self; 3] = [Self::A, Self::B, Self::C];
    }

meets no compile error, no clippy lint and no gate — only
`crates/viewer/README.md`'s **Closed vocabularies are declared once**,
if they read it. That is the standard this crate rejected when it built
`scripts/gates/viewer-module-kinds.sh`, whose own header
(`scripts/gates/viewer-module-kinds.sh:10-18`) exists because a rule
sold as "mechanically checkable" spent its first life with nothing
reading one.

## The gate, and why it was not written in 2046

The scan is cheap and the reviewer of 2046 described it: **hit on any
`const ALL: [Self; N] = [ … ]` (and the un-named shape: any `const`
array literal of two or more `Type::Variant` entries) under
`crates/viewer/src`, allowlist the three kinds the README section
ratifies** — a struct-constant registry (`Theme::ALL`), a deliberately
partial list, a mirror of an enum declared in another crate.

The reason it is an item rather than a hunk of that PR is siting, not
size. A gate must fire on its own inputs (Ev, 2026-08-20, on S61), and
this one's inputs are `crates/viewer/src` plus the README section that
holds its allowlist — so it needs a `ci.yml` step, a
`scripts/gates/gate-roster.sh` registration, a line in
`local-scripts/ci-local.sh`, and a `scripts/check-ci-mirror-parity.py`
decision about whether it is TIER-blind (its allowlist lives in a
README, so a docs-only change can falsify it — which is exactly the
argument that put `viewer-module-kinds.sh` in the `mirror` job). None
of those is a question about `const ALL`, and answering them inside a
PR that already converted nine enums is how a review loses the thread.

## Hazards for whoever takes it

- `scripts/gates/lib.sh:113-141` forbids a trailing `|| true` on a
  scanning pipeline: it swallows grep's exit 2 (a real error) along
  with exit 1 (no match). This program has already re-minted that
  defect once, at #1953.
- The allowlist must be READ from the README section, not restated in
  the script — `viewer-module-kinds.sh`'s own contract, and the reason
  its rosters have not gone stale.
- `crates/viewer/tests/` holds four hand-written complete variant lists
  already (`viewer-suites-hold-hand-written-complete-variant-lists`),
  so whether the scan covers the suites is a decision, not an
  oversight. They are a different shape — inline arrays in rows, not
  `const ALL` tables — and a scan tuned for one will not see the other.
