---
id: one-way-to-say-dependency-and-intent
kind: ruling
title: One way to say dependency and intent: variables, spaces, placements, constructions and assertions
status: closed
opened: 2026-10-03
priority: P0
pr: 3990
closed: 2026-10-03
---


## The question

A document today says "this depends on that" and "these are meant to
coincide" in many unrelated ways, and none of them tells the next
feature how it should say the same thing. The user-facing cost is
declarations that restate what the numbers already say. What is the
one model of dependency, placement and intent the kernel should have,
and what does each current mechanism become under it?

## The evidence

- **Five kinds of dependency.** Consuming inputs (`Node::inputs`,
  which also decide product roots, A10); stored reading edges
  (`Gauge.parent`, `InstantiatePart.gauge`); recomputed reading edges
  (mate heads, A12); name references deliberately not edges (declared
  pairs, `payload_names`); name references that are edges
  (`Measure` refs, `crates/editor-core/src/node.rs`).
- **Six ways to place something.** Absolute-coordinate `Datum`s,
  `Transform`, `Pattern`/`PlacedUnion` frames, gauge plus offset,
  mates (placing or only declaring by where they fall in a spanning
  tree, A11 (4), invisible to the user), the viewer's free-move probe.
- **Eight ways to state intent.** Shared surface key or `GeomSource`;
  declared pairs on `Boolean`/`Union` (`Rest`, `Tangent`,
  continuation, seam); a mate's class; PATHS tangent joints; axis
  declarations (`docs/AXIS-DECLARATION-DESIGN.md`, ratified, unbuilt);
  declared-distinct parameters (`docs/PARAM-LINT-SPEC.md`, draft);
  report-only `Assertion` nodes (E10).
- **The ceremony.** One two-peg mate takes 29 declarations
  (`demos/tour/src/twopeg.rs`); a through-hole as deep as its plate
  must declare its caps flush (`demos/tour/src/plate.rs`); the guide
  teaches interpenetrating parts by 4 mm to avoid flush geometry
  (`docs/GUIDE.md`, the bracket).
- **Inconsistencies.** `ParamSource` lowers two separately typed equal
  literals to equal tokens, so equal values ARE read as intent there
  (`crates/topo/src/param_source.rs`, `field_source_evidence`),
  against the banked "coincidence is structural or declared, never
  inferred from values". Document parameters are f64 only (no derived
  parameters). Mate alignments hold raw f64, not `Expr`.
- **What already exists to build on.** The symbolic tier
  (`crates/geom-core/src/sym.rs`, E12) decides a margin whose
  polynomial normal form over the parameter symbols is zero as a
  theorem for every parameter value — but it treats a literal as a
  constant. A9 already defines relative freedom as component structure;
  A11 (2) already has groups that live in their own space; the viewer's
  free-move probe is already display-only. DISCIPLINES DS2/DS3 already
  separate identification-grade disciplines (the built solid depends on
  the verdict) from classification-grade ones (only acceptance does).

## Ev's direction (in chat, 2026-10-03)

These were agreed in conversation and are inputs, not options:

1. **Shape and location are different questions.** Nothing consumes
   anything; a part has no location of its own. A *placement* is a
   bundle of relations that together pin one copy of a part relative
   to others; two placements of a part are two copies; a relation
   added to a pinned placement is an overconstraint. A boolean
   requires its operands to be already related; it carries no
   placing constraints of its own.
2. **No raw numbers.** A slot holds a variable whose type suits the
   slot: continuous slots take variables, discrete ones (a sign, a
   branch, an enum) take discrete variables, and a slot with one
   sensible value takes none. Parameters can be defined from other
   parameters. The GUI makes inline variable creation easy and offers
   an existing variable when a typed value matches it.
3. **Nodes are operations on variables.** A node takes variables and
   defines new ones (possibly many), and variables are typed beyond
   scalars (frames, faces, bodies), so references to geometry stay.
4. **No absolute coordinates.** Spaces are what is related to what. A
   world node — undeletable, related to like a part — exists only to
   give export its coordinates; the kernel's internal computing frame
   per space is chosen near the geometry and depends on nothing about
   the world node.
5. **Tangent by construction.** Coaxiality by sharing one axis object;
   tangency by sketch constructions that read another surface's trace
   in the sketch plane (which also expresses tangency at a single
   point).
6. **Checked (in)equalities replace declared contacts** where a thing
   cannot be made true by construction; they are not called mates
   ("mate" stays the placement relation).
7. **Contact and tangency complaints become lints where the answer is
   already known.** A lint finding is quieted by an assertion on the
   same measure at the same site whose bound the observation meets; a
   bound that crosses zero does not quiet a contact finding.
   An in-band coincidence that is not already decided stays a
   refusal in a boolean.

## What is asked of the designers

The final state: what a document is (its node, variable and relation
vocabulary), what "structural" means, what each mechanism listed under
the evidence becomes or whether it disappears, what the user sees, and
which ratified clauses change. Name every remaining fork with a
recommendation. Where the direction above is wrong against the tree,
say so.

## Ev's words

Ev's messages of the 2026-10-03 conversation, verbatim:
`git show 5f7a1c71e3:docs/ev-transcripts/2026-10-03-one-way-to-say-dependency-and-intent.md`.
Part 2 (decision 2: rung (2) first, (3) as a later rung):
`git show 3d70e5de72:docs/ev-transcripts/2026-10-03-one-way-to-say-dependency-and-intent-part-2.md`.
Part 3 (non-structural Zero coincidences glue and are linted):
`git show fae23dbc71:docs/ev-transcripts/2026-10-03-one-way-to-say-dependency-and-intent-part-3.md`.
Part 4 (the `unproven-coincidence` lint):
`git show c4158a079c:docs/ev-transcripts/2026-10-03-one-way-to-say-dependency-and-intent-part-4.md`.
Part 5 (the principle states only the margin; the lint checks structure):
`git show da589659c0:docs/ev-transcripts/2026-10-03-one-way-to-say-dependency-and-intent-part-5.md`.
Part 6 (the consistency pass and the sign-off):
`git show 9046cfe90d:docs/ev-transcripts/2026-10-03-one-way-to-say-dependency-and-intent-part-6.md`.
