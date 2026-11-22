---
id: main-red-the-gather-records-one-sample-tier-3-prime-does-not
kind: issue
title: main red (probe roster): assemble_one_local_battery's gather records 2409 samples and tier 3′ 2408 since PR 6380, so scripts/k_probe_sweep.sh stops at its first plain suite
status: closed
opened: 2026-11-09
priority: P2
cost: E
closed: 2026-11-16
pr: 7017
branch: onehome/gather-sample
---


Filed by `onecopy/main-reds-k-and-m10`, which needs the K probe sweep to run to its end.

## What

`crates/editor-core/tests/assemble_one_local_battery.rs`,
`the_gather_pays_tier_3_prime_once_and_the_at_rest_check_nothing`
(`--features probe`, the first suite `scripts/k_probe_sweep.sh` runs in its
plain half), fails at ε 1e-9 with `the gather recorded 2409 samples and
tier 3′ 2408: the gather must pay tier 3′ once`. The row asserts that the
product's gather records exactly the samples `topo::validate_pseudomanifold`
records on the same body; one sample is now recorded by the gather and not
by tier 3′.

## Where it began

Bisected first-parent over main from `27bcf1be41` (green): merge
`b2cc80453b`, PR 6380 (`onehome/run-owns-rounding`, "the run owns f64
rounding — admission rule on (ε, K), floor charges out, session box at the
finished-body gate, bare-rounding gate"). Its first parent `5f2af4e111`
passes. The PR's body does not mention the row; it runs only under
`--features probe`, which no PR gate runs.

Unverified reading: the session box read at the finished-body gate is a
decision the gather takes and tier 3′ does not. If so, either the gather's
census is no longer tier 3′'s alone and the row's claim moves, or the box
read belongs inside tier 3′.

Reproduce: `CAD_TOLERANCE_EPS=1e-9 cargo test -p editor-core --features
probe --test all -- assemble_one_local_battery::`.

## Closed (2026-11-16): the box read belongs inside tier 3′

The extra sample is `session_box_reach`, read once and Positive. A trace of the
two multisets, removed after, left it alone on the gather's side.

Tier 3 asks the session box first (`structural_via`, `validate.rs:6580`), after
`validate_closed` and before its local battery. Tier 3′'s own path
(`pseudomanifold_certificate_via`) said "with empty records 3′ is tier 3
exactly" but ran `validate_closed` and the battery without the gate. So a body
past the box refused at tier 3 and passed tier 3′'s battery. The gather
finishes its aggregate through tier 3, box included, then runs the census over
the kept verdict, which is where the one sample came from.

Tier 3′'s path now asks the box where tier 3 does. With that:

- `assemble_one_local_battery` passes: the gather's samples are a standalone
  tier-3′ pass's, sample for sample;
- the new row `session_box::tests::tier_3_prime_refuses_a_body_past_the_session_box_as_tier_3_does`
  pins that tier 3′ refuses the brick 2 km out as tier 3 does, and turns red
  without the gate.
