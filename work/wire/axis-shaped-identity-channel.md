---
id: axis-shaped-identity-channel
kind: issue
title: Build the ratified axis-shaped identity channel: a per-component, placement-composing source for axis-flavoured facts
status: parked
opened: 2026-09-12
refs: [2404, 1593, 1604]
blocked_on: [geom-source-absence-conflates-four-origins]
---


## What

**Ratified design, unbuilt.** `docs/AXIS-DECLARATION-DESIGN.md`
(Ev, 2026-09-12, PR 2404). This row is the channel it ratifies.

A declaration names an **axis**; coaxiality between two carriers is
derived from it, and whether it still holds is decided by comparing
placement provenance — token equality, zero numerics. Absence of
provenance **refuses** (`crates/verbs/README.md` §3 P3's precedent).

## What is actually new

Not a reopening of §3 P1. P1's exclusion is scoped to **motion-invariant**
fields and an axis is not one, so this sits on `GeomSource`'s side of the
line P1 draws — where `SourceExpr::Placed` composition already exists in
the kernel. Read `docs/AXIS-DECLARATION-DESIGN.md`'s Round 3 before
re-deriving that; an earlier reading had it the other way and was wrong.

What IS new is **granularity**. `GeomSource` identifies a whole
description (a surface key); an axis is a *component* of one. Two
different cylinders sharing an axis come from different expressions, so
their `GeomSource`s differ and the shared axis is not derivable from
them. A per-component source that composes through placement is the
work.

## The staleness rule, ratified

| since the declaration | chains | verdict |
| --- | --- | --- |
| neither placed | equal | holds |
| both placed by one node/instance | equal outer wrappers | holds — relative pose unchanged |
| one placed, or both by different chains | differ | **stale** — refuses structurally, naming the placement node that broke it |

Row three under-claims (two chains composing to the same relative motion
refuse though coaxiality survives). That is the fail-loud direction and
the row is re-declarable; it was ratified knowing this.

## Territory, and why this is not one lane's diff

- `crates/verbs/` is **WIRE's** — the vocabulary and the README clause.
- `crates/topo/src/source.rs` is **TOPO's** — `GeomSource`/`SourceExpr`
  live there and a finer granularity changes them.
- `crates/topo/src/boolean/join.rs` is **S-BOOL's/CURVED's** —
  `cs_pair_frame` is where a declaration is consumed, and its own
  sentence already says so and stays accurate.
- `crates/editor-core/` — the attach side, and the document-level
  declaration node.

So this is a sequence with announced seams, not a unit. Cutting it is
the first task; do not dispatch it as one lane.

## Read first (and see the cut below, which supersedes this heading)

`work/topo/geom-source-absence-conflates-four-origins.md` decides what
"no provenance" means, and this channel's refusal arm depends on that
answer being implementable. It is not a CORRECTNESS blocker — the
refusal is right either way — but a channel built before it will refuse
on four situations it cannot tell apart, one of which is a bug in the
re-stamp. When this said "blocked on nothing" the row had not been cut;
cutting it showed the block is one of ORDER and OWNERSHIP, and the row
is now parked on that row as its named trigger.

## The cut (WIRE orchestrator, 2026-09-12)

Cutting this was the row's own first task. Here is the sequence, its
owners, and what each step owes. **The headline is that WIRE cannot
start it**: steps 1–3 are TOPO's and EXCH's ground, and WIRE's own steps
are downstream of all three. So this row is `parked` on step 1 rather
than open, and the cut is written here so the sequence survives WIRE's
closure — a ratified design whose first mover is another program is
exactly the thing that gets lost when the program that ratified it
exits.

### The spine: this channel is `ParamSource`'s shape at `GeomSource`'s granularity

`crates/verbs/README.md` §3 already has the three-part shape a provenance
channel takes in this codebase, and its own file table (`README.md:35-37`)
says which crate owns each part:

| part | `ParamSource`'s answer | the axis channel's |
| --- | --- | --- |
| **P1** the lowered token | `topo/src/param_source.rs` — opaque, side tables | a per-component source beside `GeomSource` in `topo/src/source.rs`, composing through placement |
| **P2** attach / propagate / consume | `editor-core` attaches; `topo` propagates; `join.rs` consumes | same three seats, `cs_pair_frame` as the consumer |
| **P3** absence refuses | `editor-core/tests/seat6_param_source.rs` | ratified as ruling 3 |

Read against that table the sequence falls out, and so does the reason
the order is what it is: **P1 before P2 before P3, and the document-level
declaration last of all.**

### Step 1 — positive origin marking (TOPO)

`work/topo/geom-source-absence-conflates-four-origins.md`.

`None` covers imported, hand-built, kernel-derived and a failed
re-stamp. The ratified design says absence refuses, which is correct
**and** means the channel's refusal arm cannot say which of four
situations it is refusing, one of them a bug. The ratified doc names
this as "the first step toward per-component provenance": both are
*record where this came from rather than inferring it from what is
missing*, at two granularities.

Not a correctness blocker — the refusal is right either way — but a
channel built before it refuses four things with one sentence.

### Step 2 — the import adoption (EXCH)

`work/exch/step-import-discards-the-entity-ids-that-are-its-identity-channel.md`.

Fills one of step 1's origins with real content, from ids `import_step`
already holds and discards at the door. In principle parallel to step 1;
in practice after it, because step 1 decides the vocabulary the mark is
written in. Ev's idea, 2026-09-12.

### Step 3 — the per-component source (TOPO) — **the channel itself**

The P1 equivalent, and the only genuinely new work the ratification
created. `GeomSource` identifies a whole description; an axis is a
*component* of one, so two cylinders sharing an axis have different
`GeomSource`s and the shared axis is not derivable from them.

Round 3's correction is load-bearing here and must not be re-derived:
this sits on `GeomSource`'s side of §3 P1's line, **not** `ParamSource`'s,
because P1's exclusion is scoped to *motion-invariant* fields and an axis
is not one. `SourceExpr::Placed` composition already exists in the
kernel; this extends its discipline at a finer granularity. **P1 stands
untouched and nothing here reopens it.** An earlier reading had this
backwards and was wrong.

### Step 4 — attach, propagate, consume (editor-core + S-BOOL/CURVED)

The P2 equivalent. `editor-core` attaches on the recipe side;
`topo` carries and composes; `crates/topo/src/boolean/join.rs`'s
`cs_pair_frame` is where a declaration is consumed, and its own sentence
already says so and stays accurate — verified at ratification, nothing
to change there.

### Step 5 — the declaration node and the vocabulary clause (DOCM + WIRE)

The document-level node that names an axis, and `crates/verbs/README.md`
§3's fourth clause group describing the channel the way P1–P3 describe
theirs. **Last, and deliberately so**: a declaration is persisted
document content, so shipping a shape and changing it later is a
migration of saved files rather than a refactor. The axis-shaped form is
ratified, so the shape is decided — but the node must not land before
the channel that validates it, or the document stores rows nothing can
answer.

WIRE's parts are the `crates/verbs/` clause and the vocabulary; the node
is DOCM's.

### Step 6 — absence refuses, with rows (WIRE)

The P3 equivalent, on `seat6_param_source.rs`'s model. Ratified as
ruling 3; the test is what makes it true rather than intended.

### What this row is parked on

**Step 1 landing**, as the named trigger. Not because steps 4–6 are
un-designable before then, but because a channel whose refusal cannot
distinguish a bug from three legitimate states is a channel built to be
rebuilt — and step 1 is the cheaper half of the same repair.

### The under-claim, re-stated so a taker does not "fix" it

The staleness table's row three refuses when two different chains
compose to the same relative motion, though coaxiality survives. **That
is the fail-loud direction, it was ratified knowing it, and the row is
re-declarable.** A taker who makes it exact by comparing composed
motions numerically has replaced a token comparison with a measurement
and broken ruling 1.
