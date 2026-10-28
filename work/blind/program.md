---
id: blind
kind: program
title: BLIND — the kernel-discipline gates' matchers, and the spellings they miss
status: ready
opened: 2026-10-26
group: infra/ci
prefix: blind/
tag: (BLIND orchestrator)
ab_band: 15600-15699
paths: [scripts/gates/bounds-allowlist.sh, scripts/gates/interval-square-allowlist.sh, scripts/gates/bit-identity-consumer.sh, scripts/gates/reporting-margin-door.sh, scripts/gates/register-equal-allowlist.sh, scripts/gates/no-ambient-env.sh, crates/topo/tests/r1_lane1_bracket_read_census.rs]
keep_out: [opened 2026-10-26 by GUARD's split - the rows moved by git mv with ids and bodies unchanged and GUARD stays open on scripts/gates/* whole with the gate harness, so every gate script here is shared ground with GUARD - a change to lib.sh's shared reader or selftest harness is GUARD's and is announced there rather than made here, crates/geom-core/tests/bounds_census.rs and crates/test-utils/src/source.rs are S-TINT's and S-TCOST's ground - S210's census half lands there by announced seam, the certified doors the bounds gate would have to see (crates/geom-brep/src/props/*, crates/profile/src, crates/mesh/src, crates/bvh/src) belong to PROPS and others - S210's and S41's reach into them is a row filed on the owner and never a diff from here, crates/profile/src/path/arc_fillet.rs and the ~49 profile sites G4 collapses are edited by announced seam as G4's own body says, .github/workflows/* is CIW's, run scripts/work.py territory on your branch and announce the seam in the PR where the changes are coupled]
---

The kernel's discipline gates — `bounds-allowlist.sh`,
`interval-square-allowlist.sh`, `bit-identity-consumer.sh`,
`reporting-margin-door.sh`, `register-equal-allowlist.sh`,
`no-ambient-env.sh` and the bracket-read census — each police one rule
from `docs/DESIGN.md` or a ratified README clause, and every row here is
a spelling of the policed thing that the matcher misses (or, twice, an
innocent spelling it refuses). Cut out of GUARD on 2026-10-26 when GUARD
measured 43 points against 30. Charter and order: `work/blind/plan.md`;
narrative in `work/blind/log.md`.
