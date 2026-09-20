---
id: indeterminate-error-arms-sweep
kind: issue
title: The ~40 Indeterminate-carrying error variants the escalation channel makes unnecessary to match on: a deletion sweep
status: review
pr: 2928
branch: props/escalation-channel
opened: 2026-09-05
refs: [k-stats-escalation-channel-and-redo, escalation-channel-misses-op-minted-indeterminates, should-classify-replays-error-enum-arms-be-deleted, topo-mints-indeterminates-outside-the-funnel, 1969]
---

## What

`grep -rn 'source: Indeterminate\|cause: Indeterminate\|(Indeterminate)' crates/*/src`
finds roughly 40 error-enum variants across `sweep`, `topo`, `profile`,
`geom-brep` and `editor-core` that carry a funnel escalation out of an
op (`ExtrudeError::ExtrusionEscalated`, `LoftError::StackingEscalated`,
`RevolveError::AxisEscalated`, `BlendError::Escalated`,
`ChartRegionError::Escalated`, `PlaneEqError::Escalated`,
`StructureRefusalKind::Indeterminate`, `NodeErrorKind::Escalated`, …).
PR #1969's escalation channel (`NodeValue::escalations`,
`NodeError::escalations`) makes MATCHING on them unnecessary for the
question "did a predicate escalate, and on what margin": the
subdivision driver reads the log first (`crates/editor-core/src/drive.rs`,
`classify_replay`).

## What this item is

The deletion sweep the k-stats spec explicitly named as a different
unit: which of those variants still carry information a consumer needs
(the op's own context around the escalation — Display text, the
recourse sentence, the site) and which are pure wrappers a consumer
could read off the log instead. Not the two arms in `classify_replay`:
those stay load-bearing until
`escalation-channel-misses-op-minted-indeterminates` lands (the log
does not carry op-minted `Indeterminate`s or the mate solve's).

## Acceptance

A hit list of the variants with a disposition each (kept: names why;
deleted: its consumer now reads the log), a sweep pattern with its
blind spot stated, and no behaviour change in what a consumer can
learn.


## Measured (PR 2928): the population is 88, and 0 are retired

### The recipe, the number it gives, and the number first written down

Re-derived after review found the two did not agree. Three figures, and
the gap between them is the finding:

- **The item's own recipe** —
  `grep -rn 'source: Indeterminate\|cause: Indeterminate\|(Indeterminate)' crates/*/src`
  — returns 79 lines, **62** of which are variant declarations, of which
  38 match the narrow line shapes this file first counted as "roughly
  forty".
- **The widened recipe**, `(source|cause|diag): (geom_core::)?Indeterminate`
  plus the bare tuple form, returns **83** declaration lines.
- **The true population, from an enum-body parser** (brace-matched,
  `#[cfg(test)]` bodies excluded, every payload shape): **88**.

An earlier revision of this file wrote **82** for the widened recipe. It
came from neither grep nor parser but from a third ad-hoc walk-back
script, which silently dropped `PropsError::Escalated`
(`geom-brep/src/props/mod.rs`) when its enclosing-enum regex mis-read
the file. **A measurement whose stated recipe does not reproduce it is
not a measurement**, so: the recipe gives 83, the population is 88, and
82 was wrong.

### The five outside even the widened recipe — every declared blind spot, realized

| variant | payload | why the recipe misses it |
|---|---|---|
| `FoldStop::Indeterminate` | `(Box<Indeterminate>)` | boxed tuple |
| `MateFault::Indeterminate` | `{ mate, diag: Box<Indeterminate> }` | boxed field |
| `FrameError::Degenerate` | `{ input, indeterminate: Option<Indeterminate> }` | a fourth field name, inside an `Option` |
| `BooleanError::ContactContradicted` | `{ declaration, margin: Indeterminate, steer }` | a fourth field name |
| `ValidationError::ContactContradicted` | `{ declaration, witness, margin, steer }` | a fourth field name |

The blind spots this file declared — "an `Indeterminate` nested inside
another type (a `Box`, a nested refusal struct, an `Option`), and a
variant whose payload field has some fourth name" — are all three
REALIZED, not hypothetical. **Two of them are family 3's own carriers**
(`FoldStop` and `MateFault`), i.e. the sweep's pattern was blind to
exactly the family the sibling item is about.

Their disposition is the same as the rest — kept — and two of them are
the clearest cases in the whole population:
`FrameError::Degenerate`'s `Option` IS the distinction (`None` = a
definite zero, `Some` = in-band), which the log cannot express at all,
because for a definite zero the funnel never escalated.

### Still not matched, after the widening

An `Indeterminate` reached through a type alias; one carried inside a
nested refusal struct that is itself the variant's payload; a variant in
a file the walk does not read (`include!`, `#[path]`). And the whole
class of escalations that are not carried by an enum variant at all —
see the shape sweep below.

### No dead variant

Every one of the 88 is constructed; the smallest count of qualified
`Enum::Variant` references outside a declaration is 2
(`SplitReduceError::CrossingEscalated`).

### Why none is a pure wrapper — and the counterexample search that failed

**The escalation log is a per-bracket SIDE channel**, not a second copy
of the error. `Bracket::open` has four shipped call sites
(`editor-core/src/eval/mod.rs` per node, `eval/parts.rs`'s shield, two
in `topo/src/props.rs`), plus `k_stats::detached` in
`editor-core/src/names/discriminate.rs`, `mesh/src/tessellate.rs` **and
`topo::props`' own detached face map** — the last was missing from this
item's first enumeration. `record_escalation` drops what it is given
when no frame is open. So for `step-export`, `step-import`, `stl`,
`verbs`, `pncad`, `pncad-py`, `viewer`, `demos/tour`, `demos/wild` and
every library consumer, the log is empty and the enum is the only
channel carrying the escalation.

**That argument predicts a counterexample shape**: a variant every one
of whose constructions AND consumptions sits inside an open bracket
would be readable off the log instead. Both arms of PR 2928's dual went
looking for one, independently, and **neither found one**. That is the
strongest evidence this conclusion has, and it is worth more than the
argument that predicted it.

**A correction against this item's own tidiness**: `record_escalation`
is the one place the channel is MINTED, not the one place it is
written — `k_stats::splice` also extends it, with a detached run's
escalations. One mint, two writers. The first revision of this item said
"one write", which is literally false.

Secondary reasons, measured: **40 of 83 carry a SITE the log has no
field for**; **every arm composes the op's own sentence and recourse in
`Display`** (no bare `write!(f, "{diag}")` anywhere; gated by
`every_props_error_arm_names_a_recourse`,
`every_chart_region_arm_names_a_recourse` and the `profile`/`sweep`
`recourse_roster` suites); and a group state a DEFINITE fact carrying
the classifier's diagnostic as evidence.

### Reason (c) is hollow for five variants, and saying so is the point

The "carries the classifier's diagnostic as evidence" reason is not
sound everywhere it was applied. In
`crates/topo/src/boolean/contact_verify.rs`, **five** shipped sites
build `ContactRefusal::Contradicted` with a `MarginDiag::Invalid`
payload after a DEFINITE `Sign::Positive` — so the "diagnostic" those
variants carry is FABRICATED: an `Invalid` nobody classified, attached
to a margin that was measured. For those five the reason states nothing.
What actually keeps the variant is the `declaration` and the `steer`
beside the diagnostic, which the log has no field for either — reason
(a), not (c).

The mis-typing itself is `work/curved/topo-mints-indeterminates-outside-the-funnel.md`.

### The shape sweep, which the spelling sweep missed

Sweeping for the SHAPE (a definite sign, then a hand-built escalation)
rather than the spelling finds a second family the variant census cannot
see, because it is about how the payload is BUILT and not about what
carries it: `topo` holds four separate local
`fn invalid(band, predicate)` helpers, minting after a definite sign at
eight shipped sites across `census.rs`, `boolean/contain.rs`,
`boolean/sectors.rs`, `splitting/rules.rs` and `sector_shape.rs`, plus
seven struct-literal mints in `boolean/contact_verify.rs`. Filed on the
two programs whose ground they land on. **This is what the PR's
"closed structurally" claim did not hold against, and the claim has been
narrowed to what shipped: made impossible at the eight `geom-brep`
sites, guarded by a spelling census over that one crate.**

### `classify_replay`'s two arms stay; the question is now open, not answered

The ruling was "not until the channel carries the op-minted family". It
does now. They stay — but the first revision of this section gave a
REASON that is false on both legs, and it is corrected rather than
quietly dropped: it said two shipped paths escalate with no bracket
open, naming the mate solve and a rayon map. `work/perf/rayon-maps-...`
says in bold that the verdict channel is already safe at those sites
(`eval_node` brackets inside the mapped closure; what is lost is the
`probe` sink and `sym::report`), and a mate escalation arrives as
`NodeErrorKind::Mate`, which is not one of the two arms. Neither
supports the claim.

What is true: every mint of either arm's kind is in `eval::wire`, inside
`eval_node`'s bracket, so no live path reaches them — and that is a
search, not a licence to delete, because read (2) speaks for the log's
FIRST escalation where an arm speaks for the one the error carried.
`work/props/should-classify-replays-error-enum-arms-be-deleted.md`
holds the question with both arms' searches recorded.

### What this item was right about, restated

What PR 1969's channel bought is that `classify_replay` does not have to
MATCH on those variants to ask "did a predicate escalate, and on what
margin". That is a fact about one consumer, and it is worth what it
cost. It is not a fact about the variants: they are the ops' own typed
refusals, and this measurement finds no consumer that could read one off
the log instead.
