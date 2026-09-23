---
id: product-gate-says-verbatim-then-states-the-difference
kind: issue
title: the per-part at-rest gate's trigger is one decided policy with a home in neither of the two crates that implement it (product.rs called its copy verbatim)
status: open
pr: 2499
opened: 2026-09-12
priority: P1
cost: D
---



(WIRE orchestrator) Found by the full review of PR 2442 as a prose
residue in a file the PR edits (S11), sharpened here. Pre-existing and
out of that PR's fence.

`crates/editor-core/src/product.rs:664-668`:

> Pass 2: the per-source gate, asked only when the product holds more
> than one solid (**the import loop's rule, verbatim**). The count is
> over SOLIDS, not sources: one source may itself carry several (an
> instantiated sub-assembly), and it is the product's solid count the
> rule speaks about.

The two sentences disagree. The import loop's condition is
`model.instances.len() > 1` (`crates/step-import/src/lib.rs:700`) — an
**instance** count, over the instances of one solid spec. The product's
is `total_solids > 1`, summed over every source's `solids().count()`
(`product.rs:669-673`). They are not the same predicate and they do not
count the same things; the very next sentence is the explanation of how
they differ, written as if it were an elaboration of "verbatim".

Both rules may well be right where they sit — the import loop's
per-solid subject is the placed copy of one spec, the product's is the
aggregate — and that is the point: the word "verbatim" claims a shared
authority that neither site can check and that the comment itself
immediately contradicts. A reader who trusts it and changes one rule
will not know the other did not move.

What is owed is small and is a choice between two shapes: either the
two predicates really are one rule about "the subject a gate is asked
about" and should have one home that both cite, or they are two rules
and this comment should say which one it is and what the import loop's
does differently. Nothing gates the word either way, which is the
general hazard — a self-declared copy is a claim no test can read.

The same file carries two more statements of the condition
(`product.rs:42`, `:155`), both accurate about solids; they are the
rule's restatements, not the false part.

## Read against the code (2026-09-12, PR carrying this row)

**The two really are one rule with two spellings, and that makes this a
duplication row rather than a prose one.**

The import loop's `model.instances.len() > 1` counts instances, but an
instance is one solid there and **the kernel refuses it if it is not**:
`build_one_solid` assembles one `SolidSpec` into a body of its own,
`transform_rigid` maps it without changing its solid count, and the
loop grafts it with `topo::graft_disjoint` — the SINGLE-solid door,
which opens with `if src.solids().count() != 1` and returns
`BooleanError::JoinDesync { what: "graft source is not a well-formed
single-solid body" }`. So an instance contributing zero or several
solids is a typed refusal of the import, not a silent miscount: the
equivalence is enforced at runtime, not merely true by construction.
And the graft appends one solid per call, so the shipped body's solid
count IS `instances.len()` — the property
`the_assembly_record_retains_the_occurrence_structure` and
`the_assembly_record_covers_a_file_that_places_nothing` pin in
`crates/step-import/tests/freecad.rs`, the second by asserting
`record.len() == body.solids().count()` directly.

The import loop's predicate is therefore **"the shipped body holds more
than one solid"** — the same predicate `product.rs` spells
`total_solids > 1`.

So the comment's second sentence is not "the explanation of how they
differ", as this row read it. It is the transcription hazard AT THIS
SITE: here a source may carry several solids, so counting sources would
answer a different question. Both sentences are true.

What was actually wrong was one word. "Verbatim" claims word-for-word
identity of an expression that is deliberately spelled differently at
each site, and it is the self-declared-copy vocabulary
`reviewer-style-lane.md` Q1 greps for — a claim no test can read. The
gate no longer makes it; it points at the module doc, which is where
this file states the shared shape once.

## What remains open

The policy itself — **the per-part at-rest gate is asked only when the
aggregate holds more than one solid, because with one the per-part and
aggregate subjects are the same body** — has no home. It is stated at
**seven** sites across three crates, each pointing at another by prose:

| site | what it says |
| --- | --- |
| `step-import`'s materialization loop | the rule, as `instances.len() > 1`, with the identity-not-exemption reason |
| `step-import`'s shared-gate comment | the two subjects |
| `product.rs`'s module doc | "the same F8/D7 shape as the import loop" |
| `product.rs`'s `ProductError::SolidInvalid` doc | the trigger again, for the refusal |
| `product.rs`'s pass-2 gate | the rule, as `total_solids > 1` |
| `topo::graft_disjoint_all`'s doc | "the step-import loop's per-solid-then-aggregate shape" |
| `step-import`'s `vertex_rest_contact` doc | **the consumer case** — see below |

**The seventh is a different kind, and it is the sharpest evidence the
policy needs a home.** `vertex_rest_contact` does not restate the rule,
it RELIES on it: *"the per-solid gate above sees only the pre-graft
copies and only when more than one instance ships"* is its stated
premise for refusing an unresolvable vertex with
`StepImportError::VertexWithoutPoint` rather than passing over it. A
consumer citing an unhomed rule by prose is worse than another
restatement of it — a restatement that rots is wrong documentation,
while a premise that rots is a refusal decided on a condition that no
longer holds, and nothing at either end would say so.

**An eighth statement has already drifted, and it is off the slate.**
`work/perf/assemble-aggregate-census-is-quadratic-in-solids.md` stated
the trigger as *"runs only when a SOURCE has more than one solid"* —
the wrong subject; it is the product's count, summed over sources. That
row is closed, so nobody will re-derive it, but it is the prediction
this row makes happening in the wild: a restatement of an unhomed rule,
made in good faith by a lane that had to read the rule off a comment,
wrong within two days. Corrected in place by this PR (a factual
citation fix in a closed record, not a re-opening), and cited as a
line-numbered pair the sweep below could not have matched anyway.

**What the sweep could not match.** The census above is `grep`-shaped:
the phrases "more than one solid", "multi-solid", "instances.len() > 1"
and "one instance" across `crates/`, then a read of every hit. It finds
restatements that use the rule's own vocabulary. It cannot find a site
that states the trigger in different words (PERF's "a source has more
than one solid" was found by a reviewer's read, not by that pattern),
and it does not cross `work/`, `docs/` or the test suites at all — so
the seven sites are a floor on the count, not the count.

**What would settle it**: the F8/D7 policy stated once where both
callers can cite it — `topo`'s at-rest door is the only place both
depend on — with the two call sites and the consumer citing that
instead of each other. That is a cross-crate design move on a ratified
decision, not a comment edit, which is why this row stays open after
the word came out.

## What this PR's comment edit did and did not buy

`product.rs`'s gate now says "(this module's F8/D7 shape)", which
points at the module doc — and **the module doc says "the same F8/D7
shape as the import loop"**. That is a prose pointer to a prose
pointer. It is a real improvement in blast radius: a rename in
`step-import` no longer rots the gate comment, and the file states the
cross-crate claim in one place instead of two. But the cross-crate
prose pointer is MOVED, not removed, and this row's finding is
unaffected by it.

## An instance of the class, found inside the fix that trims it

The chain above originally cited a test called
`the_assembly_record_indexes_the_shipped_solids`. **That test has never
existed** — not in the tree, and `git log -S` over all history finds
the name was never written. It was taken on faith from a comment in
`step-import`'s materialization loop and propagated verbatim into this
row and into the PR body before review caught it. The property IS
pinned, by the two `freecad.rs` rows now cited, so the chain's
conclusion survives; the citation did not.

Recorded rather than quietly corrected, because of where it happened:
a hand-written, unchecked, already-rotted citation, written by the same
PR that trims `placement.rs`'s prose about hand-written unchecked
citations. It is a live instance of META's
`doc-citations-no-gate-checks-rot-silently` arm B, and it cost a
reviewer the read that caught it.


## Recharacterized 2026-09-13 (PR 2499) — a duplication, not a prose slip; the row STAYS OPEN

The unit was told to check whether the two sentences were one rule with
two spellings **before** calling it a prose fix, because `verbatim` at a
copy site is the self-declared-duplication tell. They are one rule.

`model.instances.len() > 1` counts instances, and an instance is exactly
one solid — `build_one_solid` makes one body per `SolidSpec`,
`transform_rigid` does not change a solid count, and `graft_disjoint`
**hard-refuses** `src.solids().count() != 1` with `JoinDesync`. That last
link is stronger than the unit first stated it: the equivalence is
**enforced at runtime**, not true by construction, so an instance
contributing zero or several solids is a typed refusal of the import
rather than a silent miscount.

So both sites say "the aggregate holds more than one solid", and the
word `verbatim` was the only thing wrong — it claims word-for-word
identity of an expression deliberately spelled differently at each site.

**What the row now carries, and why it stays open: the policy has no
home.** It is stated at **seven** sites across three crates, each citing
another by prose. The seventh is a different kind and the review found
it: `vertex_rest_contact`'s doc does not restate the policy, it
**relies** on it as the premise for refusing rather than passing over an
unresolvable vertex. A restatement that rots is wrong documentation; a
**premise** that rots is a refusal decided on a condition that no longer
holds, and nothing at either end would say so.

An eighth statement had already drifted within two days of the row being
written — a PERF row stating the trigger with the wrong subject — which
is the row's own prediction happening in the wild. The sweep is
grep-shaped over the rule's vocabulary and **seven is a floor**: it
cannot find a site phrased differently, and does not cross `work/`,
`docs/` or the suites.
