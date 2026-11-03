---
id: importgate
kind: program
title: IMPORTGATE — the import door: how it reads the file at ε_in, what a foreign face becomes, what it validates, and how it refuses
status: ready
opened: 2026-10-29
group: kernel/exchange
prefix: importgate/
tag: (IMPORTGATE orchestrator)
ab_band: 16800-16899
paths: [crates/step-import/*]
keep_out: [opened 2026-10-29 by EXCH's semantic-seam split (EXCH measured 133 budget points against 30; Ev approved the division in chat 2026-10-29) - rows moved by git mv with ids and bodies unchanged - EXCH INTERP FOREIGN REIMPORT IMPORTGATE share crates/step-import by design and EXPORT shares crates/step-export, so run scripts/work.py territory on your branch and announce the seam where changes are coupled - the D10 refactor hold that covered assembly import (placements and per-instance gating and the census of imported contacts) lifted when INTENT closed on 2026-10-30 (#6103) - COINCIDE owns step-import-anchor-rows-are-recorded-and-not-linted - TRIM PCERT CHART and SSI own the certification lanes adoption promotes into, ENCL owns the reading vocabulary the door's refusals end in and parks at-rest-stored-contradictions-and-unminted-sides-panic-once-import-validates-everything on can-an-import-reach-a-state-only-a-kernel-bug-otherwise-reaches, INTERP merged in on 2026-11-02 (docs/doc-ledger/interp-leaves-the-tracker.md), FOREIGN merged in on 2026-11-02 (docs/doc-ledger/foreign-leaves-the-tracker.md) - its notes: RESTFRONT's validity-refuses-an-interior-chart-singularity lands after import-normalizes-the-rim-only-cap]
---

**The import door's contract.** Ev, 2026-10-17: "imported geometry
should never be distinguishable from native geometry — there should be
no states that are only reachable via import" (DESIGN
`[tolerance.boundary]`). This track asks when and how an import is
validated (`can-an-import-reach-a-state-only-a-kernel-bug-otherwise-reaches`,
designers first), closes the inputs that reach past the door today (a
deep file that kills the process, a negative radius, an inverted sense
the checks cannot see), and makes the door's refusals read as the
import door's: the file's ε_in, one ending per refusal, words a viewer
can show.

Charter and order: `work/importgate/plan.md`; narrative in `work/importgate/log.md`.

## Merged in 2026-11-02

Interpretation at ε_in (D7), the normal form of foreign faces and the door's validation and refusals are one question asked at one door: what an imported file becomes in the kernel, so that no state is reachable only through import. The track budget now counts only dispatchable P0, P1 and unbanded rows (`work/README.md`, Track size), so these tracks had fallen far under it; their rows moved here by `git mv` with ids and bodies unchanged.

**From INTERP** (INTERP — the file read at ε_in, the caches built at ε (D7)):

**D7's two tolerances, built.** D7 says the file's ε_in governs
interpretation and the kernel's caches are recomputed from the
description and certified at ε. Adoption today certifies the file's
parsed carrier at ε, and reads several of the file's own values
(directions, frame windows, plane origins) through windows stated in
ε_in, then certifies at ambient. This track builds the interpretation
stage (`adoption-rebuilds-caches-at-eps-from-eps-in-interpretation`, the
P1 that the rest hang from) and closes the rows that are its symptoms:
the two-dial strand, the frame-direction and cone-angle windows,
`nist_ftc_09` at 1e-12, the coincident-surface gate, and the endpoint
parameters the kernel derives rather than the file states (ellipse,
partial B-spline, the minted seam), each of which today tells a sound
file it is damaged.

**From FOREIGN** (FOREIGN — what a foreign face becomes at the import door):

**The normal form of what another system writes.** A foreign file
states faces this kernel does not hold as stated: a rim-only cap with a
chart pole inside it, a chart with a C⁰ knot inside a cell, a curved
face with rings, a conic the curve reader has no arm for. This track
decides at the import door what each becomes — re-minted and reported,
split, admitted, or refused typed — and the outer-loop inference such
faces lean on.

`import-normalizes-the-rim-only-cap` is the P0: RESTFRONT's
`validity-refuses-an-interior-chart-singularity` lands after it, or
import starts refusing files it accepts today.

### FOREIGN's handoff (2026-10-29)

The first sitting closed the P0 (`import-normalizes-the-rim-only-cap`, PR 6007: every chart singularity in an imported face is a vertex, minted from `geom::pole_form`) and took two design questions to Ev on `[ev]` PR 6014 (fork rows 215, 216). Both P1 lines are now parked on other tracks:
- **The wrap tip** (Ev: "if the complexity it adds is tolerable"): `import-mints-the-wrap-tip-at-a-chart-singularity` waits on the shared loop walk (`work/restfront/loop-readers-each-derive-a-corners-turn-with-no-shared-loop-walk.md`), CARVE's one-band revolve and RESTFRONT's tier rules. The probe of every reader is on `probe/foreign-wraptip`.
- **The C⁰-knot split** is built and held as draft PR 6013 (`foreign/c0-knot-split`), parked on QUAD's per-span quadrature row. When that lands:
  - merge main into the branch. Its copy of the QUAD row is older than main's, so take main's;
  - re-time the conic quarter torus;
  - take the PR out of draft, review it (single FULL), and merge.

  EXCH's promotion row is the other half of its cost.

What is left here is P3/P4: OCC statements that still refuse (a sphere with a vertex-loop hole, an obliquely cut cone), rings on curved faces, outer-loop inference, hyperbola/parabola, the scaffold strut (`design`). Narrative in `log.md` (FOREIGN's own log is recoverable at the SHA `docs/doc-ledger/foreign-leaves-the-tracker.md` names).
