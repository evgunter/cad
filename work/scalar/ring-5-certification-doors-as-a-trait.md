---
id: ring-5-certification-doors-as-a-trait
kind: unit
title: RING-5: the certification doors become a sealed extension trait on Interval, imported by name, with a gate on its importers
status: open
opened: 2026-09-24
branch: scalar/ring-5
needs_ev: true
---


## What

Ev's follow-up to RING-3 (in chat, 2026-09-24, option 1 of two): the
certification doors RING-3 put on `Interval` as inherent methods move
into a sealed extension trait in `geom_core::interval::certification`,
imported by path, not re-exported at the root; `poison` → `refused`;
`from_bounds`, `repr_bits`, `is_certified`, `from_certified` stay
inherent. A new gate allowlists the importers and forbids `Real`, globs,
`super::*`, the evaluation hull and `.is_poison(` in their production
code; the two mixed files separate by pure moves (`quad_lane` to its own
file, `probe_tube_chart`'s certification tail into `ssi/enclose.rs`);
the census re-keys on the trait and cross-reads the gate. C9 re-worded
(Ev's text — the PR is `[ev]`). Absorbs
`certification-value-hygiene-has-no-gate` items 1, 2, 3, 6, 7 and the doc
half of 5. Spec: `docs/RING-5-SPEC.md` (deleted at merge). Survey:
`/home/user/scalar-briefs/survey-ring5.md`. Dispatched after LANE-4P
(#3165) lands (both edit `topo/src/props.rs` and `bounds-allowlist.sh`).

**Review tier: DUAL** (two Opus reviewers under
`docs/DUAL-REVIEW-PROTOCOL.md`): it changes a ratified clause, installs a
standing gate whose blind spots are the question, and its failure mode —
a site that quietly resolves to `Real` — is silent by nature.
