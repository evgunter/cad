---
id: heatsink-placedunion-base-union-unfinished
kind: issue
title: The heat sink's PlacedUnion migration stopped half-way on a reason #571's own adjudication retired - the base union never moved in-document
status: open
opened: 2026-08-31
github: 1344
refs: [571]
---

## From GitHub issue 1344

opened 2026-08-31, 1 comment.

`PlacedUnion` landed at #571 to retire F4's out-of-document union, and the
ratified acceptance names the heat sink by name. Half of it happened: the
fin GROUP moved into the document, the base union did not — and the reason
recorded for stopping there describes a lowering that #571's own
design-owner adjudication replaced.

## What the record says

`docs/GROUP-BOOLEAN-DESIGN.md`, acceptance:

> the heatsink's out-of-document union moves INTO the document (the F4
> note retires at its origin, both workarounds deleted per the demo
> doctrine)

and, in the ratified shape: *"The heat-sink document says
`PlacedUnion(fin, Linear{..})` (corpus `heat_sink_fins`) — F4's
out-of-document union retires wherever it is re-authored."*

## What is actually in the tree

**The corpus fixture stops before the base**, and states why —
`crates/editor-core/tests/corpus/heatsink_union.rs`, numbered finding 1:

> The heat sink's BASE is not unioned here. The kernel's `combine` door
> takes two SINGLE-SOLID operands (`JoinDesync: "operand A/B is not a
> single-solid body"`), so fusing a five-solid group into a base needs a
> kernel door that does not exist — a multi-solid boolean operand, not a
> recipe-layer question.

That reason no longer holds. `PlacedUnion` does not produce a five-solid
group. Per the #571 adjudication recorded in the design doc, the lowering
was changed *precisely so it would not*:

> This sentence originally named `graft_disjoint_all_keyed`, the
> pre-existing N-ary door; its N-SOLID output is ASM's instancing
> currency, which `setopfinish` correctly refuses as an operand […] The
> added door is the faithful elaboration of the ratified union semantics.

`crates/topo/src/instance.rs:248` on the door that replaced it:

> N placed copies of a single-solid prototype grafted onto the SAME
> target become **one solid of N shells** […] the seamed boolean path
> accepts that result as an operand, while an N-SOLID body is refused
> (`setopfinish`'s single-solid gate). A group union that wants to feed a
> later boolean must therefore produce the representation the chain it
> replaces produced, entity for entity.

And `corpus/die_tool.rs` already does it: a `PlacedUnion(ball,
Explicit(frames))` feeds straight into `Node::Boolean { op: Subtract, .. }`
in a shipped, tested corpus document.

**The demo tour still uses the pre-#571 spelling.**
`demos/tour/src/heatsink.rs` builds a `Node::Pattern`, then unions the
instances into the base in demo code, under a module-doc note that reads
as a live gap:

> F4 note, probed 2026-07-25: a Boolean recipe node cannot consume a
> Pattern node's `Instances` payload today, so the union-to-one-solid step
> lives HERE in demo code, honestly outside the document.

The literal sentence is still true — `eval::wire::body_operand` refuses
`ValuePayload::Instances` with `WrongOperand { expected: "body", found:
"instances" }`, and that is by design (Pattern's N-bodies-unfused contract
is deliberate; its `Instances` are the assembly product's currency, gathered
per-instance in `product.rs::sources_of`). But the *consequence* the note
draws — that the union must live outside the document — stopped being true
when `PlacedUnion` landed. The document can say
`Extrude(fin) → PlacedUnion(fin, Linear{count}) → Boolean(Union, base, group)`.

## What is left to establish

One thing, and it is the only genuinely open question here: `die_tool`
feeds its group in as a **cutter** whose shells are disjoint from the
target, whereas the heat sink's fins deliberately overlap the base by 1/16
(flush fin bases refuse). So the base union is a *seamed* union against a
multi-shell operand. `instance.rs` says the seamed path accepts that
representation; whether it goes through for this configuration is a run
away, not a reading away.

## Proposed

1. Re-author `demos/tour/src/heatsink.rs` onto
   `PlacedUnion` + one `Boolean(Union)`, deleting the demo-side
   `solidify` step and the F4 note, per the demo doctrine the acceptance
   cites. The fin count stays a structural param — `PlacedUnion` carries
   `SlotId::Count` for parametric rules, so `SetStructuralParam` and the
   downstream-only-recompute story are unchanged.
2. Fix or retire `heatsink_union.rs`'s finding 1, which currently tells a
   reader a door does not exist when the sibling corpus document uses it.
3. If (1) refuses at the seamed union, that refusal is the real finding
   and this issue becomes its home.

## Not proposed

Merging `Pattern` into `PlacedUnion`. They are different currencies by
ratified decision (D3's silent-dispatch-trap rule): `Pattern` yields N
solids that stay N solids and feed the assembly product; `PlacedUnion`
yields one solid of N shells and feeds a boolean. The tour uses both —
`benchlayout` wants the first, the heat sink wants the second and asks for
the first.

## Comments

**2026-08-31** — orchestrator:

On why the fins overlap the base by 1/16: it is dodging the
**undeclared-coincidence refusal**, not a tangency guard — and the tour
already runs that exact experiment next door, so the refusal is on the
record rather than a guess.

`demos/tour/src/bool_bodies.rs::table` attempts three leg unions in
"honesty order" and narrates all three live:

1. leg top face **exactly coplanar** with the tabletop's underside,
   **undeclared** (shared value `TOP_Z.0`) — refuses at the coincidence
   door, rung (b), value equality never classifies;
2. leg inset and overlapping 0.05 **into** the top — a proper transversal
   intersection, no declaration needed, works;
3. corner-straddle (the pre-M4-PR-5 workaround).

The heat sink's comment names this directly — *"the table-leg pattern,
1/16 overlap — flush fin bases would refuse"*. So the overlap is attempt
2, copied. It is a dodge, and the thing dodged is a declaration, not a
curved-contact guard: a fin sitting flush on the base is face-**coincident**
(material on opposite sides — a `Rest`), which is the twopeg mating plane's
class, not a tangency.

**The overlap is no longer the only option, and that is new.** The shipped
table does not dodge its *side* planes — it declares them
(`try_union_declared`, flush side planes glued by the declared rung, M4
PR 5). What made the same move impractical for the fins is that declaring
per-fin contacts meant one declaration set per Transform node, against
bit-identical `StableName`s with no per-instance discriminator — the very
problem `PlacedUnion` was ratified to fix. With `Instance(i)` naming,
"fin *i*'s base face against the base's top face" is a one-row selector.

So there is a third option beside "overlap by 1/16" and "leave it in demo
code":

- fins **flush** on the base, contacts **declared** through the recipe
  layer's own path (`Node::Declare` + `Boolean { declare }`), which is
  what a real extruded heat sink's geometry actually looks like — the
  1/16 embedment is a modelling fiction the part does not have.

That is strictly more interesting than reproducing the overlap in-document,
and it exercises a path that only became expressible when `PlacedUnion`
landed. It is also more likely to find something, which cuts both ways:
if the declared flush union refuses, that refusal is the finding and this
issue is its home either way.

Recommend trying flush-and-declared first, falling back to the 1/16
overlap (recorded as such) if it refuses.

## Home

S-BOOL: the one open question is whether the seamed boolean path admits a multi-shell operand for the base union — an operand gate in `crates/topo/src/boolean/*`, which is S-BOOL's territory and its charter (operand gates that refuse or mis-admit legal inputs).
