---
name: Telemetry gating rule
description: Instrumentation is feature-gated at its module boundary from the first commit and armed by an explicit call, never by the environment
type: convention
---

# Telemetry: gate it from the first commit

**Rule.** Any probe, meter, or falsifier that records from inside the
kernel is:

1. **feature-gated at its MODULE boundary** — a live half and an inert
   half with identical signatures, so `armed()` folds to `const false`
   and the call sites in the hot lane carry no `#[cfg]` (worked example:
   `mesh::budget`);
2. **armed by an explicit call**, never by an environment variable;
3. given its own CI row, because the default rows then exercise the
   inert half.

**Why, concretely.** `mesh::probe_stats` was added ungated and grew an
ambient `NURBS_PROBE` arm. It reached shipped builds: same release
binary, same arguments, 7.9 s → 19.8 s on the demo tour, and the
sampling block's `assert!` meant an environment variable converted
`tessellate`'s typed-error contract into a panic. Nothing had asked for
it; it just never had a gate to stop it.

Retrofitting is the expensive direction, and #558 is the receipt:
gating `probe_stats` after the fact cost a feature-naming ruling and a
CI-lane change, for a module that would have cost neither on day one.
It is done — `mesh`'s `probe-stats` feature, named for its module the
way `budget` is, deliberately NOT folded into `budget` (that feature
has a release consumer, `scripts/tess_budget_sweep.sh`, and sharing
would have put the falsifier's `assert!` back into a release
artifact). Two consequences worth carrying forward:

* **The name collision is real and was not resolved by renaming.**
  `mesh` already had a `probe` feature meaning the K-telemetry
  recording scalar. Renaming THAT would have churned six manifests,
  `demos/tour`, `scripts/k_probe_sweep.sh` and
  `local-scripts/bt-add-probe-feature.py` to relabel the feature that
  was there first, so the new one took the module's own name and both
  manifest entries carry a NAME CAUTION paragraph instead.
* **Rule 3 has teeth only if the new row is UNCONDITIONAL.** Gating
  moved `z1_per_triangle_certificate_falsification` out of the default
  `cargo test -p mesh` row, and M8-5's MIN-1 had adopted self-arming
  *specifically* so the hosted gate ran it unconditionally. The
  replacement is ci.yml's "mesh certificate falsifier (feature =
  probe-stats)" row in the `k-lint` job, which runs whenever anything
  builds. When you gate telemetry, budget for that row.

**Enforced**: the `discipline` job's "no ambient environment in the
kernel" grep (ci.yml + ci-local.sh), allowlisting only
`CAD_TOLERANCE_EPS` and the fuzz dials — that grep bans rule 2's
failure mode. Rule 1 is enforced by the inert half itself: both gated
modules carry a `const _: () = assert!(!armed());` in their disarmed
side, so a stub that stops folding fails the BUILD.
