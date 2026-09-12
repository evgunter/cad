# Axis-flavoured declarations — a design question

**STATUS: OPEN QUESTION, not ratified.** Opened by WIRE, 2026-09-12.
The tracker row is
`work/wire/axis-flavoured-declarations-have-no-channel.md`, which
carries the provenance and the code citations; this doc states the
question and is updated in place with the answer.

## What is missing

`ParamSource` (`crates/verbs/README.md` §3, P1) is lowered *expression*
identity for the **stored scalar fields** of minted descriptions — a
radius, a half-angle. By design it carries no placement datum: a stored
scalar is motion-invariant, so `SourceExpr::Placed` exists for
`GeomSource` and deliberately not for this channel.

Two consumers need something it cannot hold:

- VERBS-CYLSPH's coaxial cylinder×sphere arm takes a `CoaxialEvidence`
  whose comment claims the parameter-identity channel is its honest
  carrier (`crates/topo/src/boolean/join.rs`, `cs_pair_frame`);
- `crates/verbs/README.md` §3 P2 says SPHSPH's structural-parallelism
  option "reads the same channel at its own position".

**Neither is true as landed.** Coaxiality and parallelism are claims
about **axes and centres** — placement data — and nothing `SurfaceField`
can hold serves them. Every in-tree caller passes
`CoaxialEvidence::None`, so the pair routes to the general rung and
poses `FrameError::NoArm`. That is honest today and it is not a design.

## Not re-litigated here

- **Coaxiality is never INFERRED from measurement.** `cs_pair_frame`'s
  own sentence, and Ev confirmed the reading on 2026-09-12: a
  declaration cannot be *obtained* by measuring an axis-to-centre
  distance at any tolerance. Whether one may be *verified* is a
  different question and is live below.
- **D2**: intensional descriptions, no extensional fallback.
- **N6**: `GeomSource` is lowered pure-data identity, compare-only,
  attached by `editor-core` (`crates/topo/src/source.rs`).

## The fork as first posed, and why it dissolved

The row posed two options: **(a)** a placement-level declaration (the
document names two carriers' axes as one axis, the way
`BooleanDeclarations` names contacts), or **(b)** a frame-level identity
that survives placement through composition.

They differ behaviourally and cover **disjoint inputs**:

| | (a) declaration | (b) provenance |
| --- | --- | --- |
| imported / hand-built geometry | **works** — the user can declare it | **cannot answer** |
| can it be wrong | yes | no, but silently *absent* |
| what invalidates it | nothing, today | a differing chain |
| persisted | yes | no — opt-in side tables |

Ev's objection (2026-09-12) is what settles it:

> *"I don't like relying on the numerical check to tell if it's been
> rotated."*

It is right, and the failure is **live rather than hypothetical**:
`Node::Declare`'s pairs name entities by `StableName` and are re-resolved
at every evaluation, so after one operand is rotated the declaration
re-resolves to the same faces and is asserted again. Nothing about the
edit invalidates it. Today only `verify_declared_contacts`
(`crates/topo/src/boolean/mod.rs:2075`) would catch it — at a band.
Three things are wrong with that being the only guard:

1. **It converts exact information into a tolerance question.** That one
   operand was rotated is a *structural* fact, known exactly in the
   recipe. Re-discovering it by measuring throws that away.
2. **A decided predicate has an in-band arm.** A small rotation lands
   inside the ambiguity band and the answer is *indeterminate* — neither
   "still coaxial" nor "you broke it", on a question with an exact
   answer.
3. It is the shape D2 exists to refuse.

## The recommended shape: declared intent, structural invalidation

(a) and (b) are not alternatives. Each supplies what the other cannot:
**(a) supplies the intent**, which nothing can infer and which is the
only thing that can serve geometry the recipe did not build; **(b)
supplies the invalidation**, decided on provenance rather than geometry.

The load-bearing fact is that **coaxiality is invariant under a rigid
motion applied to BOTH carriers and destroyed by one applied to one.**
So "is this declaration still true?" reduces to "have these two carriers
been placed by the same chain since it was made?" — a comparison of
opaque tokens, zero numerics.

### The mechanism ships today

`SourceExpr` (`crates/topo/src/source.rs:57`) is a cons-list —
`Placed { node, instance, inner }` over a `Minted { index }` base — and
its own doc states the rule this needs: *"Equal chains ⇒ equal maps
applied to equal descriptions ⇒ equal bits (D9)."* `transform_rigid`
clears `GeomSource` because it rewrites description bits, and the recipe
layer re-stamps the composed source immediately after
(`crates/topo/src/transform.rs:524-530`).

Comparing the two carriers' placement **prefixes** gives every case:

| since the declaration | chains | verdict |
| --- | --- | --- |
| neither placed | equal | holds |
| both placed by one node/instance | equal outer wrappers | holds — the relative pose is unchanged |
| one placed, or both by different chains | differ | **stale** — refuses structurally, and can NAME the placement node that broke it |

The third row **under-claims**: two different chains composing to the
same relative motion refuse though coaxiality survives. That is the
fail-loud direction and the row is re-declarable, so it is the right way
to be wrong.

## What this does not solve

- **A datum has no identity channel at all.** `DatumValue`
  (`crates/topo/src/query.rs:606`) is by its own doc *"geometry VALUES,
  not kernel entities and not recipe references"*. `GeomSource` attaches
  to surfaces, curves and points on a `Body`
  (`crates/editor-core/src/eval/wire.rs:472`); datums are
  document-level and have none. A variant that hangs identity on the
  **axis datum** is inventing a channel, not wiring one up — which is
  why the shape above compares the **carriers**.
- **Imported and hand-built bodies carry no source.** `import_step`
  (`crates/step-import/src/lib.rs:638`) is a free function at the kernel
  seat; there is no `Node::Import`, so `stamp_minted` never runs, and
  placed instances go through `transform_rigid` — which clears sources
  with no recipe layer to re-stamp. Nothing records that a body was
  imported.

  **Ev's adoption-step idea (2026-09-12) closes most of this**, and is
  cheaper than it looks: the file's entity ids are already in hand and
  discarded at the door (`import_step`: *"a `SolidSpec`'s maps are keyed
  by the file's entity ids"*). A STEP entity id is real identity — two
  faces referencing one surface entity genuinely share it — and M8
  instancing (N copies, each placed by its own frame) is a pattern in
  all but name, which `Placed`'s `instance` field already serves. It is
  an adoption-side unit, not a prerequisite: the design stands without
  it and gets better with it.

  **What it is NOT is a kernel-anchors problem.** An earlier reading
  linked this to `two-verb-seats-do-not-compose`, on the ground that
  `GeomSource { node, .. }` wants a `RecipeNodeId`. That was wrong, and
  `source.rs`'s module doc says so: the fields are *"the lowered
  pure-data forms (`u64` node ids, structural expression addresses)…
  this crate only ever compares them for identity and flips
  orientation."* Nothing maps a `GeomSource.node` back to a recipe node.
  What a new minter owes is the **retirement theorem** (same source ⇒
  bit-identical descriptions, which a STEP entity id satisfies) and
  **namespace disjointness** — and the second is load-bearing, because
  `RecipeNodeId(pub u64)` is a full `u64` with no free high half, and a
  collision would make two unrelated surfaces read as same-source and be
  glued by the boolean's coincidence rung.

## The questions

1. **Is declared-intent-plus-structural-invalidation the shape?** This
   doc recommends it. The alternative is to accept the band check as the
   only guard against a rotation, which Ev has already declined.
2. **What does a declaration attach to?** The **carrier pair** needs no
   new channel and is what the recommendation assumes. Anything
   **axis-shaped** (a datum) needs datum identity invented first, and
   would be a larger design round.
3. **For carriers with no provenance, does a declaration refuse or fall
   back to verification?** `crates/verbs/README.md` §3 P3's precedent is
   that absence refuses **permanently** — consistent and strict, and it
   locks out imported geometry entirely. The alternative is a band-gated
   verification in exactly that case and only there, on the argument
   that where there is no exact information, measurement is the honest
   instrument rather than a fallback from one. Ev's adoption step would
   shrink this question to hand-built bodies alone.
4. **If (3) admits verification anywhere, how is the lowered id space
   partitioned** between minters, given `RecipeNodeId` is a full `u64`?

## On answering

`crates/verbs/README.md` §3 P2's SPHSPH sentence is wrong as landed and
is corrected when this is answered — the row records that obligation so
it is not lost.

---

# Round 2 — Ev, 2026-09-12 (PR 2404)

> 1. sounds good
> 2. idk, what would be cleaner and more principled?
> 3. how do we know when there's no provenance?

**Q1 is RATIFIED**: declared intent, invalidated structurally by
comparing the two carriers' placement chains. No numerical check decides
whether a rotation happened. Everything below is the residue.

## Q3 first, because it answers itself and it changes Q2

**We do not know, and that is the finding.**
`Body::surface_source(k)` (`crates/topo/src/body.rs:526`) returns
`Option<&GeomSource>`, and `clear_geom_sources` (`:658`) produces
exactly the same `None`. So a bare absence conflates **four**
situations:

1. imported geometry — never stamped (there is no `Node::Import`, so
   `stamp_minted` never runs);
2. a hand-built body — never stamped;
3. a kernel-derived description — never stamped;
4. **a description `transform_rigid` cleared and the recipe layer failed
   to re-stamp** — which is a *defect*, and is indistinguishable from
   the three above.

That settles Q3's own question in favour of **refusing on absence**
(§3 P3's precedent): the one case you would be silently tolerating by
falling back to verification is the bug. A lost re-stamp would quietly
downgrade an exact structural answer to an approximate one, which is the
failure mode this whole design exists to remove. Refusing turns the same
bug into a loud, wrong-looking refusal — recoverable, and visible.

### The principled repair is to make origin POSITIVE

`None` cannot be made informative by reading it harder. The fix is to
stop inferring origin from a missing entry and **record it**: every
description carries where it came from — recipe, imported, hand-built,
kernel-derived — so absence becomes unrepresentable and the four cases
separate.

That is Ev's adoption-step idea generalised, and it upgrades it from a
convenience to the thing that makes the signal mean anything. It also
does double duty: see Q2.

## Q2 — the axis-shaped form is cleaner and more principled

Asked directly, answered directly. **Axis-shaped is the better design**,
for three reasons, the third being the one that matters:

1. **It is a function, not a relation.** N carriers on one axis is N
   facts, not N² declared pairs, and adding the N+1st does not touch the
   others.
2. **It names what is shared**, so a stale declaration refuses with
   *"these no longer share axis D"* rather than *"these two are not
   coaxial"* — the difference between a diagnosis and a symptom.
3. **It composes to the other predicates, and there is already a second
   consumer waiting.** `crates/verbs/README.md` §3 P2's SPHSPH
   structural-parallelism case needs the same kind of fact one position
   over. Against a named axis or direction, parallelism is "same
   direction" and concentricity is "same point" — one channel, several
   readings. Pairwise, each predicate needs its own evidence type and
   its own argument, which is exactly how `CoaxialEvidence` came to
   exist as a one-off that nothing could serve.

### What it costs, precisely

**Per-COMPONENT provenance, which does not exist.** `GeomSource`
identifies a whole *description*, not its axis: two different cylinders
sharing one axis come from different expressions, so their
`GeomSource`s differ and the shared axis is not derivable from them.
The granularity needed is `ParamSource`'s — per stored field — **with
placement fields admitted**, and §3 P1 excludes those deliberately and
states its reason:

> `SourceExpr::Placed` exists in the kernel only because rigid placement
> re-parameterizes a *description*, while a stored scalar field is
> motion-invariant, so no kernel op composes or interprets one and no
> second spelling of expression structure enters the kernel.

So the objection is **not** that composing through placement is
intractable — `GeomSource::Placed` is the existence proof that it is
already done, in the kernel, today. The objection is that admitting
placement fields to the parameter channel puts a *second* spelling of
expression structure beside the first. That is a real cost and it is the
thing a ruling here would be spending.

### The recommendation, and the honest catch

**Carrier-pair now, axis-shaped as what it grows into** — with one
caveat that is not free and should decide the call rather than be
discovered later:

- carrier-pair is buildable today, needs no new channel, is
  `BooleanDeclarations`' exact shape, and its invalidation reduces
  cleanly to the chain comparison Q1 just ratified;
- but a declaration is **persisted document content**. Shipping the
  pairwise vocabulary and later moving to the axis-shaped one is a
  migration of saved files, not a refactor.

So: if the axis-shaped form is wanted eventually, the cheap order is to
decide that **now** and take the pairwise form only as an explicitly
temporary shape — or to skip it. Shipping pairwise "for now" without
that decision is the expensive path.

**And the two questions share a repair.** Positive origin marking (Q3)
is the first step toward per-component provenance (Q2): both are
"record where this came from rather than inferring it from what is
missing", at two different granularities.

## Open, for round 3

- **Q2**: pairwise-now-with-migration-accepted, pairwise-as-the-answer,
  or axis-shaped as its own design round (which reopens §3 P1's scoping
  decision)?
- **Q3**: refuse on absence is recommended and argued above. Does
  positive origin marking get opened as a unit — and does it belong to
  this design or to the adoption path?
