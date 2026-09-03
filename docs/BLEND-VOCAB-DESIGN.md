# How a shared blend refusal names its verb — the 917 conversation

**Status: RATIFIED (Ev, 👍 on the PR-1279 impact discussion,
2026-08-30), with the three open choices settled as recommended:
"edge blend" as the neutral noun, `Filleted`/`Chamfered` collapse
to one blend-named result type with both names kept as aliases,
and the V3 rename ships with the unit.** V1/V2 are the substance,
V3 the mechanical half, V4 a constraint restated from the issue.
Executed by unit BLEND-6, sequenced after BLEND-5. Mechanics
measured; the impact/reversibility record (no persisted footprint
anywhere in this doc; switching costs V2 ≈ V4 < V1 < V3) is on the
PR thread.

## Settled ground (landed, cited)

- The chamfer refuses through `FilletError` BY DESIGN (VERBS-CHAMFER;
  the near-parallel-enum failure class). The type is right; several
  of its words are not: every shared arm's `Display` opens
  `"fillet: "` / `"fillet assembly: "` / `"fillet chain: "`
  (`sweep/fillet/mod.rs:700-…`), so a `chamfer_edges` caller reads
  the wrong verb over the right fact.
- `BlendKind { Fillet, Chamfer }` exists in `sweep::fillet` with an
  honest `Display` (`mod.rs:106-123`), documenting exactly which
  predicates are ball facts vs shared facts.
- **LIB-G16 (merged #1224) already answered this question one layer
  up**: the recipe layer refuses `NodeErrorKind::Blend { verb:
  BlendKind, error }` — one wrapper carrying the verb OUTSIDE the
  shared error, four duplicate variants avoided, fillet
  messages/tags byte-identical, chamfer discriminated. That shape is
  ratified precedent (RECIPE-DOORS D2), and it is the shape V1
  copies down.
- Out of scope, already fenced elsewhere: the `fillet3_*` predicate
  names (K-corpus roster carriers — the issue rules them right as
  they stand); the persisted `RoleSeg`/`RimSupport` vocabulary
  (BLEND-5, issue 961); `OpGroup::Fillet`'s name (G16's `// #917`
  marker sits there and BLEND-6 discharges it).

## V1 — the verb crosses as ONE wrapper at the kernel doors

`fillet_edges` and `chamfer_edges` return a refusal that carries
`BlendKind` once, at the door — the kernel-direct twin of G16's
`Blend { verb, error }` — rather than a field on twenty variants
(plumbing) or a per-verb enum (the class the reuse exists to avoid).
The recipe layer then reads the verb the kernel already attached
instead of re-deriving it; the unit measures that no path renders a
double prefix.

## V2 — inner prose goes verb-neutral; the wrapper supplies the verb

The `Display` literals drop their hard-coded verb (the wrapper
prefixes `"fillet: "` or `"chamfer: "`); arms that are genuinely
ball-only keep ball language (they are unreachable from the chamfer
battery, and the unit states which those are, measured). Every
SHARED recourse constant is re-measured against BOTH verbs under the
issue-1278 rule — a recourse is a claim about a second request, so
its pin follows it per verb; `FILLET3_ASSEMBLY_RECOURSE`'s
fillet-only door list gets conditioned or split rather than spoken
to a chamfer caller.

## V3 — the mechanical rename rides the same unit, last

`FilletError` → the blend-named refusal type, `FilletRequest` /
`Filleted` / `FilletNaming` / `blend_surgery`'s module path
likewise (~255 references, mechanical, compiler-enumerated), as the
unit's closing commits after the semantic half is green — so the
rename can never be mistaken for the fix. Exceptions stay as fenced
above; identifiers naming genuinely ball-specific machinery
(`corner_ball`, spine/torus language) keep their names because they
name facts, not the shared surface.

## V4 — no parallel enum, restated as a constraint

The issue's own closing condition: this must not be closed by
minting `ChamferError`. V1/V2 satisfy 917's substance ("how a
shared refusal names the verb that raised it") with one
discrimination point per layer.

## The settled choices (ratified with the doc)

1. **The neutral noun is "edge blend"** where prose must speak
   generically — the module's own doc vocabulary.
2. **`Filleted`/`Chamfered` collapse to one blend-named result
   type**, with both names kept as aliases for call-site
   readability.
3. **V3's path rename ships with the unit** (`sweep::fillet` → a
   blend-named module): one churn, one review.

**Sequencing:** BLEND-6 runs after BLEND-5 (issue 961) per the
plan — same files, single-owner order — and discharges G16's
`// #917` marker at `OpGroup::Fillet`.
