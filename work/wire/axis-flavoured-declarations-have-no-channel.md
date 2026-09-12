---
id: axis-flavoured-declarations-have-no-channel
kind: issue
title: Axis-flavoured declarations (coaxial, structural-parallel) have no identity channel: ParamSource carries stored scalar fields only, so CoaxialEvidence and SPHSPH's option (a) cannot be served by it
status: open
opened: 2026-09-04
refs: [1593, 1604, 1372]
needs_ev: true
---


(SEAT orchestrator) Class finding from SEAT-6's dual review (PR 1593),
filed per the durable-home rule; both arms converged on it and it sits
outside SEAT's fence (the germ lanes are CURVED's now).

**The finding.** `ParamSource` (VERB-SEAT-DESIGN §3, P1) is lowered
*expression* identity for the stored scalar fields of minted
descriptions — `SurfaceField` names radii and a half-angle and, by
design, no placement datum (origins, axes, seam references are not
motion-invariant, and a token on one would have to compose through
rigid placement, the structure this channel deliberately does not
carry). VERBS-CYLSPH's coaxial cylinder×sphere arm (PR 1604,
`topo/src/boolean/join.rs::cs_pair_frame`) takes a `CoaxialEvidence`
whose comment says the parameter-identity channel is its honest
carrier; and §3 P2 says SPHSPH's structural-parallelism option "reads
the same channel at its own position". Neither is true as landed:
coaxiality and parallelism are claims about axes and centres —
placement data — so nothing `SurfaceField` can hold serves them. After
SEAT-6's seam merge `cs_pair_frame` is still called with
`CoaxialEvidence::None` in both operand orders and its sentence is
amended to say exactly this.

**What a fix needs.** A second declaration source for axis-flavoured
facts, distinct from field identity: either a placement-level
declaration (the document names two carriers' axes as one axis, the
way `BooleanDeclarations` names contacts) or a sketch/frame-level
identity that survives placement through composition (the
`SourceExpr::Placed` discipline `GeomSource` already has). Which of
those is right is a design question with a `[ev]` shape; P2's SPHSPH
sentence should be corrected when it is answered.

**Second-order note (SEAT-6's reviews, both arms).** Evidence reaches
`pair_section_frame` as one positional argument per typed position, so
the shared dispatch's signature grows once per consumer (two after the
CYLSPH merge). A per-pair evidence record resolved once at
`germ_section_frame` would grow by variant instead.

## Re-homed to WIRE (2026-09-11, the cut in `docs/WORK-TRACKS-2026-09.md` addendum 3)

WIRE is the evaluation seat's successor. `docs/DOC-LEDGER.md` sweep 9
parked three of these rows in `work/issues/` as "the successor's opening
slate" when EVAL closed on 2026-09-08, naming two more already there;
this program is that successor, and it takes the rest of the seat's
ground with them.

Its class at the cut was **H** — needs a new placement-level identity
channel; item itself calls it an `[ev]`-shaped design question. The
class is a dispatch estimate made by reading the row against the tree on
2026-09-11, not a verdict on the finding, and a lane that finds it wrong
says so in its PR. The id, the `track:` letter where the row carries
one, and the body above are unchanged by the move.

## Ev's objection, and the third option it produces (2026-09-12)

Put to Ev as a two-way fork — a placement-level declaration (a) against
a frame-level identity surviving placement (b). Ev's answer settles the
reading of `cs_pair_frame`'s sentence and then rejects the fork's
premise:

> *"that does mean never inferred, but i don't like relying on the
> numerical check to tell if it's been rotated."*

**The reading is ratified**: "NEVER inferred from a measured
axis-to-centre distance" means a declaration cannot be *obtained* by
measuring. It does not forbid verifying one, and today declarations ARE
verified — `verify_declared_contacts`
(`crates/topo/src/boolean/mod.rs:2075`) runs `verify_rest_declaration` /
`verify_tangent_declaration` at a band, where a definitely-different
carrier contradicts.

**The objection is right, and it is live rather than hypothetical.**
`Node::Declare`'s pairs name entities by `StableName` and are re-resolved
to `FaceKey`s at every evaluation, so after a rotation the declaration
re-resolves to the same (now rotated) faces and is asserted again.
Nothing about the edit invalidates it. What would catch it is the band
check — and three things are wrong with that being the only thing:

1. **It converts exact information into a tolerance question.** That one
   operand was rotated is a *structural* fact, known exactly in the
   recipe. Re-discovering it by measuring an axis-to-centre distance
   throws that away and re-derives it approximately.
2. **A decided predicate has an in-band arm.** A small rotation lands
   inside the ambiguity band and the answer is *indeterminate* — neither
   "still coaxial" nor "you broke it", on a question that has an exact
   answer.
3. It is the shape D2 exists to refuse: an extensional fallback standing
   in for an intensional fact.

### The third option: declared intent, structural invalidation

(a) and (b) are not alternatives. Each supplies exactly what the other
cannot:

- **(a) supplies the intent.** Only a declaration can say two
  independently-authored carriers are coaxial, and it is the only thing
  that can ever serve imported or hand-built geometry.
- **(b) supplies the invalidation.** Whether a rotation intervened is
  decided by comparing *placement provenance*, not geometry.

The load-bearing fact is that **coaxiality is invariant under a rigid
motion applied to BOTH carriers and destroyed by one applied to one.**
So the question "is this declaration still true?" reduces to "have these
two carriers been placed by the same chain since it was made?" — a
structural comparison, zero numerics.

### The mechanism ships today

`SourceExpr` (`crates/topo/src/source.rs:57`) is a cons-list:
`Placed { node, instance, inner }` over a `Minted { index }` base, and
its own doc states the rule this needs — *"Equal chains ⇒ equal maps
applied to equal descriptions ⇒ equal bits (D9)."* `transform_rigid`
clears `GeomSource` because it rewrites description bits, and the recipe
layer re-stamps the composed source immediately after
(`crates/topo/src/transform.rs:524-530`). So the chain is recorded and
maintained already.

Comparing the two carriers' placement *prefixes* gives all three cases:

| since the declaration | chains | verdict |
| --- | --- | --- |
| neither placed | equal (both bare) | holds |
| both placed by one node/instance | equal outer wrappers | holds — the relative pose is unchanged |
| one placed, or both by different chains | differ | **stale**, refuses structurally and can NAME the placement node that broke it |

The third row under-claims: two different chains composing to the same
relative motion would refuse though coaxiality survives. That is the
fail-loud direction and the row is re-declarable, so it is the right way
to be wrong.

### What this does NOT solve, stated plainly

- **A datum has no identity channel at all.** `DatumValue`
  (`crates/topo/src/query.rs:606`) is by its own doc *"geometry VALUES,
  not kernel entities and not recipe references"*. `GeomSource` attaches
  to surfaces, curves and points on a `Body` (`eval/wire.rs:493-511`);
  datums are document-level and have none. So a variant of (b) that
  hangs identity on the *axis datum* rather than on the carriers is
  inventing a channel, not wiring one up. The carrier-chain comparison
  above avoids that and is why it is the recommended shape.
- **Imported and hand-built geometry have no `GeomSource`**, and
  `crates/verbs/README.md` §3 P3 makes absence refuse permanently. There
  is no structural information to compare, so a declaration over
  imported carriers can only be trusted or verified numerically. **That
  is the one place the band check is the honest instrument** — not
  because measurement is good, but because there is nothing exact to use
  and the alternative is trusting an unchecked assertion.
- The re-stamp after `transform_rigid` becomes load-bearing: a placement
  that failed to re-stamp reads as a broken chain and refuses. Fail-safe
  direction, and worth a guard rather than a comment.

### What the `[ev]` PR should put to Ev

Not the original two-way fork, which this supersedes. The remaining
questions:

1. Is declared-intent-plus-structural-invalidation the shape? (The
   recommendation.)
2. What does a declaration attach to — the carrier pair, or something
   axis-shaped? The carrier pair needs no new channel; anything
   axis-shaped needs datum identity invented first.
3. For carriers with no provenance, does a declaration refuse (P3's
   precedent, consistent and strict) or fall back to band verification
   (serves imported geometry, and is the case Ev's objection does not
   reach)?

## Imported geometry: no marker, no node — and the entity ids are right there (2026-09-12)

Ev asked whether imported geometry is marked persistently as imported,
and suggested that could be an adoption step. Checked at `93280ff`:

**Correct, there is no marker — and it is stronger than that.**
`import_step` (`crates/step-import/src/lib.rs:638`) is a **free function
at the kernel seat** returning a `StepImport`. There is no
`Node::Import`; an imported body never enters a recipe. So:

- nothing stamps it. `stamp_minted` (`eval/wire.rs:472`) stamps every
  **unsourced** description during NODE evaluation, and there is no node
  — so an imported body's descriptions carry no `GeomSource` at all;
- placed instances go through `transform_rigid`, which **clears**
  `GeomSource` (`topo/src/transform.rs:524-530`) and relies on the
  recipe layer to re-stamp — and on this path there is no recipe layer
  to do it;
- so "imported" is not recorded anywhere. It is simply a `Body` whose
  descriptions have no source, indistinguishable from a hand-built one.

### Why the adoption step is the right place, and cheaper than expected

**The file's entity ids are already in hand at assembly.** `import_step`
says so in its own comment: *"a `SolidSpec`'s maps are keyed by the
file's entity ids"* — they are used to keep two copies of one component
from colliding, and then discarded. The identity channel exists in the
input and is thrown away at the door.

And a STEP entity id is **real identity, not a fabrication**: two faces
referencing one surface entity genuinely share that surface. That is the
same claim `GeomSource` makes about a recipe expression, sourced from
the file's own structure rather than invented by the reader.

**The shape already fits, including the hard case.** M8 instancing
builds N occurrences of one component as N independent bodies with fresh
topology, each placed by its own frame — so two copies must NOT compare
equal (N6: same source ⇒ bit-identical descriptions, and two differently
placed copies differ in bits). `SourceExpr` already has exactly this:

```
Placed { node, instance, inner: Minted { index } }
```

whose `instance` field is documented as *"the pattern instance index (0
for a plain Transform)"*. An imported instance is a pattern instance in
everything but name, so the existing variant serves it unchanged: same
underlying entity in `inner`, different `instance`, different token,
correct answer.

### The three obstacles, stated honestly

1. **There is no node id to put in `GeomSource { node, .. }`** — it is a
   lowered `RecipeNodeId`, and an import has none. This is the kernel
   seat having no anchors, which is `two-verb-seats-do-not-compose`'s
   whole subject. That row is **`deferred`, waiting on "a real replay
   consumer"**; an import wanting recipe-shaped identity is adjacent to
   that trigger without being identical to it, and whether it fires the
   row is Ev's call, not this one's.
2. **The adoption step must re-stamp after `transform_rigid`**, exactly
   as the recipe layer does, or the placement clears what the reader
   just attached.
3. **The scope sentence would need widening.** `topo/src/source.rs`'s
   module doc holds identity *per evaluation against the current
   document*; an imported body is not evaluated from a document, so what
   the claim means for one has to be written rather than inherited.

### What it would buy this row

Imported carriers would gain structural coaxiality on the same terms as
recipe-built ones: two cylinders whose axis placements resolve to one
STEP entity are coaxial **by the file's own structure**, decided by token
equality with no measurement. That removes the one case where this row's
recommended design had to fall back to a band check — leaving the fallback
for hand-built bodies alone, where there genuinely is no exact
information.

It is an adoption-side unit, not this row's, and it is not a
prerequisite: the design stands without it and gets better with it.

## Corrected: this is NOT a kernel-anchors problem (2026-09-12)

Ev asked whether recipe-shaped identity is actually required. **It is
not, and the earlier reading on this row was wrong.**
`crates/topo/src/source.rs`'s module doc says it outright:

> the fields here are the *lowered* pure-data forms (`u64` node ids,
> structural expression addresses). `editor-core` constructs them from
> its typed `RecipeNodeId`/`ExprPath`; **this crate only ever compares
> them for identity and flips orientation.**

Confirmed against the tree: nothing maps a `GeomSource.node` back to a
`RecipeNodeId`. The only uses of the field in `source.rs` are
carry-through in `revert` and `placed` (`:110`, `:123`) and the equality
at `:132` (`self.node == other.node && self.expr == other.expr`).

So the link this row drew to `two-verb-seats-do-not-compose` was
overstated and is withdrawn. A new minter does not need a recipe node.
It owes two things:

1. **The retirement theorem** — same `GeomSource` ⇒ bit-identical
   descriptions. A STEP entity id satisfies it: same entity ⇒ same
   parsed parameters ⇒ same bits, with placement composed in by
   `Placed` exactly as the recipe layer does.
2. **Namespace disjointness, and this one is load-bearing.**
   `RecipeNodeId(pub u64)` is a full `u64`, so there is no free high
   half to take. An import id colliding with a recipe node id would make
   two unrelated surfaces read as same-source, and the boolean's
   coincidence rung would glue them — silent wrong geometry, not a
   refusal. The partition has to be designed, not assumed, and it is
   question 4 of the `[ev]` doc.

## Asked (2026-09-12) — `docs/AXIS-DECLARATION-DESIGN.md`

`needs_ev: true`. The doc states the problem, why the original two-way
fork dissolved, the recommended shape (declared intent + structural
invalidation by placement-chain comparison), what it does not solve, and
four questions. `docs/DESIGN.md`'s companion table carries a row marked
OPEN QUESTION.

The obligation this row already recorded travels with it:
`crates/verbs/README.md` §3 P2's SPHSPH sentence is wrong as landed and
is corrected when the question is answered.
