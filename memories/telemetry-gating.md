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

Retrofitting is the expensive direction — the falsifier's own gating is
still open (issue #558) because it collides with an existing feature
name and moves a ratified test out of the default row.

**Enforced**: the `discipline` job's "no ambient environment in the
kernel" grep (ci.yml + ci-local.sh), allowlisting only
`CAD_TOLERANCE_EPS` and the fuzz dials.
