---
id: pcurve-fit-domain-refusal-moves-the-m10-sym-walk-ledger
kind: issue
title: PR 3612 moved the m10 symbolic-walk ledger's Decision forms, so main's m10_sym_profile_interval row is red at the default eps
status: open
opened: 2026-10-01
priority: P3
cost: E
refs: [3612]
---


Filed by the REACH orchestrator after a bisection (2026-10-01).

## What

`crates/editor-core/tests/m10_sym_profile_interval.rs`
`the_forms_the_walks_build_are_pinned_per_eps_row` is red on main at
the default ε. It reddens every PR whose gate runs editor-core's
default suite: PR 3615 merged over it, with the red annotated.

**Bisected** (default ε only; the 1e-6 and 1e-12 rows were not run):
- green at `4e50090523`, the parent of PR 3612's merge;
- red at `fefdc50aba`, PR 3612's merge (`pcert/pcurve-fit-domain-refusal`);
- red on `origin/main` `61bf888dfb`, with the identical text.

The failure, at the test's ledger assertion:

    slab at eps 1e-9: the walks built different forms
      actual: Plain/Decision calls 980 forms 9426 frozen 0 digest d6944216b808695de65a32c884bbff84
      pinned: Plain/Decision calls 980 forms 9686 frozen 0 digest 68a31dec794118be1e5494c295c01a77

**What moved:** only the Decision walks. The Assertion and Report lines
are byte-identical, and so are the call counts and largest form sizes.
- slab Plain/Decision: 9686 → 9426 forms;
- plate Plain/Decision: 15046 → 14609;
- plate Early/Decision: 7995 → 7741;
- plate Door/Decision: 11864 → 11548.

The walks make the same calls but build fewer forms, with a different
digest chain.

## What is owed

Decide whether the new walk is right. PR 3612 changed
`geom-brep/src/certify.rs`, `edge_nurbs.rs`, `fitted_lane.rs` and
`pcurve.rs`. If it is right, re-baseline the ledger and say in the PR
what moved and why (implementer-discipline §3). Otherwise fix the
kernel. The bisection logs are at
`~/.local/share/cad-work/reach-bisect-m10/` on the REACH box.
