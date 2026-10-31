---
id: loft-slab-and-section-names-are-positional
kind: issue
title: A degree-1 loft's slab and section names count from the start section: key them on each section's input identity
status: open
opened: 2026-10-28
priority: P1
cost: M
---


PR 5923 names a degree-1 loft through `k ≥ 3` sections one wall per
segment per slab. The four role segments it adds hold a position counted
from the start section: `LoftSlabWall(slab, …)`, `LoftSlabSeam(slab, …)`,
`LoftSectionEdge(section, …)` and `LoftSectionVertex(section, …)`
(`crates/editor-core/src/names/role.rs`, emitted by `name_loft_topology`
in `crates/editor-core/src/names/emit_sweep.rs`). Inserting or deleting
an earlier section (`DocEdit::SetLoftSections`) therefore renames every
later slab and ring. Names README N1 says names are never positional.

Ev on [ev] PR 5913 (Decision 1a): the slab and section names must key on
each section's input identity, so an earlier insertion or deletion leaves
later names intact, and two sections drawn by the same profile stay
distinct. A positional name may stand only as a very temporary
placeholder, for no more than a couple of PRs.

**What the identity is.** A section is `LoftSection { profile: VarId,
station }` (`crates/editor-core/src/node.rs`), so its input identity is
the read it holds. The precedent is `RoleSeg::From { read: VarId, of }`,
which keys a union member on its read with no position recorded. A read
alone does not keep two sections of one profile apart: a read spelled
twice is one key there (REFERENCES DM5). The key has to be the profile's
read together with something else, such as the station's reads. Two
sections with the same profile read and the same station reads stand on
one plane, which the loft already refuses.

**Why it is parked.** Keying a name on a read is the dependency
vocabulary D10 is still building: REFS owns `names/*` and "a name scoped
by its read", with indexed-read rows still open
(`work/refs/an-indexed-read-of-rank-two-has-no-family.md`,
`work/refs/a-selects-body-takes-an-indexed-read.md`). The refactor hold
(`docs/DESIGN.md`, State of the build) keeps a new unit off that ground.
So PR 5923 keeps the positional placeholder, and N1 points here. Ev's
couple-of-PRs bound and the hold may conflict. If the hold outlasts that
bound, take the question back to Ev, or to INTENT/REFS as a unit of
theirs, rather than letting the placeholder stand.

**Done when** the four segments hold each section's identity in place of
its index; a row inserts and deletes an earlier section and finds every
later slab, ring and vertex name unchanged; a row lofts one profile at
two stations and finds their walls named apart; and N1's placeholder
sentence is gone.

## Re-homed to REFS, open (2026-10-28, TESS orchestrator)

This was parked on `d10-one-way-to-say-intent-is-unbuilt` because keying a name on a read is D10's vocabulary. That vocabulary is REFS's own charter ("a name scoped by its read"). The refactor hold binds programs building on D10's ground, not the programs building D10. So this row is REFS's, open now, P1. Ev's bound applies: the positional placeholder (PR 5923) may stand for no more than a couple of PRs.

The open part is the key for two sections of one profile, which a read alone does not separate. That design question is REFS's to settle; the station's reads are one candidate.
