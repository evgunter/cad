---
id: member-space-look-through-stops-at-splits-containment-and-fragmented-merges
kind: issue
title: A member-space declaration resolves through merges only: a face consumed by a split, by containment, or inside a fragmented merged row is still order-shaped
status: open
opened: 2026-09-07
refs: [2073, does-n3-retire-loudly-generalise-to-the-folds-other-compositions]
priority: P0
cost: H
---

## What

DOCM-8 built the look-through for MERGES: a member-space name that is
no row at its step is rewritten to the accumulation's flat `Merged`
row that lists it (`crates/editor-core/src/eval/wire.rs`,
`look_through_merges`). Three other ways the fold consumes a member
face leave a declaration naming it order-shaped, measured by both
DOCM-8 reviews on head `6d433b6f`:

- **Split by a later member.** R1's fixture (`a` and `c` flush along
  x; `s` sits on `a`'s top cap, its footprint strictly inside it, so
  folding `s` in fragments the cap): of the six orders, `[a, c, s]`
  and `[c, a, s]` fuse; `[a, s, c]` and `[s, a, c]` refuse
  `DeclareResolve { Vanished { a.cap(End) } }` — the cap is neither a
  row nor in any merged row's set; `[c, s, a]` and `[s, c, a]` refuse
  the pre-existing `Emission("seam vertex parentage underdetermined
  from incident edges")` (`two-emitter-refusals-a-legal-declared-union-reaches`).
  The base refused six of six, so the look-through is a strict
  improvement; pinned as a measurement by
  `docm8_flat_merged::a_member_face_split_by_a_later_member_is_still_order_shaped`.
  R1's three-neighbour star (`c` between `w` and `e` along x, `t`
  stacked on its y-walls) is the same class: 8 of 24 orders `Vanished`
  on `c`'s end cap, 10 the seam-vertex `Emission`.
- **Consumed by containment.** R2's `r2_p7`: `a`'s x = 1 wall inside
  `big`, declared against `far`'s wall. Two orders `Vanished`, four
  `ContactContradicted` (the wall is a row at the step the pair is fed
  to and the kernel then contradicts the carrier claim).
- **A merged row later fragmented.** A member face inside
  `[Merged(set), Fragment(q)]` is in no searched set: that row is a
  fragment, not a merge, and `look_through_merges` reads bare
  `[Merged(set)]` rows only. Unreachable today: the emitter refuses
  the shape first (`crates/editor-core/src/names/emit.rs`,
  `unique_shared_edge` — R2 NOTE-4, `r2_p4`).

## Where it contradicts what is written

Nothing now. The three prose sites (`wire.rs` `route_declarations`,
`node.rs` `Node::Union`, `names/role.rs` `RoleSeg::FromMember`) and
N3 state the bound — merges only — and name this file.

## What a look-through into fragmented rows would need

A membership test cannot answer it: a fragment row's set names the
faces the MERGE listed, and which fragment a member face's material
ended in is a geometric question (the fragments' discriminators are
`SideOf`/`OrderAlong` verdicts against the cutting partners, N2), so
the rewrite would have to choose among the fragments by re-measuring
the member face against the partners — the re-measurement DOCM-8's
rule forbids at the routing step. The split case is the same
question one step earlier (which fragment of `a.cap` does the pair
mean?), and containment has no row to rewrite to at all. Any of the
three is a design ruling on what a member-space declaration means for
a face that is no longer one face.


## Re-homed (2026-09-13)

Moved from `work/docm/` to `work/wire/` at DOCM's exit sweep (`docs/DOC-LEDGER.md`,
sweep 14): the file it names is WIRE's (`names/emit*.rs`, `eval/wire.rs`, `product.rs` are in WIRE's paths). Id, body and header are unchanged; the directory is the
claim (`work/README.md`). Any `## Home` section above is superseded by
this line and is kept as the record of why the file was where it was.

(At DOCM's exit sweep, `refs` names the PRs `DOCM-8` stood for: `DOCM-8` = #2073 — the unit rows left the tracker with `work/docm/`; `docs/DOC-LEDGER.md` sweep 14.)

## Read against the tree (2026-09-15) — verbatim still true, and re-kinded `ruling`

Read by the WIRE orchestrator before dispatch. `look_through_merges`
(`crates/editor-core/src/eval/wire.rs`) is unchanged in the respect this
row is about: its search is

```rust
[RoleSeg::Merged(set)] if names::merged::covers(set, name) => Some(row)
```

over the accumulation table — **bare `Merged` rows only**, with an
explicit guard above it rejecting anything whose path is not exactly
`[FromMember { .. }]`. A `[Merged(set), Fragment(q)]` row matches
neither, exactly as the row says. The two-matches arm refuses
`MEMBER_FACE_IN_TWO_MERGES`, which is a fourth shape the row does not
mention and which a taker should fold into the same question.

**Re-kinded from `issue` to `ruling`.** The row already contains the
argument for it and states the conclusion in its own last sentence:
*"Any of the three is a design ruling on what a member-space declaration
means for a face that is no longer one face."* It also proves no lane
can close it — a membership test cannot answer which fragment a member
face's material ended in, and the geometric re-measurement that could is
the one DM4's routing step forbids. A row a lane cannot close is not a
unit, and leaving it `kind: issue` on the slate made it look dispatchable
to anyone reading the board.

**Not blocked on it:** the emitter's misclassification of these shapes as
`Emission` (a kernel bug by definition) is separable and takeable now —
see `two-emitter-refusals-a-legal-declared-union-reaches`. That unit runs
first regardless of how this ruling lands, and the fragmented-merge case
is unreachable until it does.

## RULED (Ev, PR 2677, 2026-09-15) — re-kinded `ruling` → `issue`

The framing question this row was parked behind is answered:
`does-n3-retire-loudly-generalise-to-the-folds-other-compositions`.

**The rule:** a composition that breaks *one name denotes one entity*
**refuses**; it **offers** a replacement where a **unique best** offer
exists, and refuses with no offer where one does not. Ev's context is
the half that decides this row: N3's rejected alternative was *"not even
refusing, just silently taking the merged descendant"* — so the value
protected is **never silently re-point**, and the absence of a computable
offer is not a reason to fall back to silence.

**So all three of this row's cases resolve the same way: refuse, with no
offer.**

- **Split** — which fragment the declaration meant is geometric, and
  DM4's routing step forbids re-measuring there, so no *unique best*
  offer exists. (`ResolutionFailure::offers` is a `Vec<StableName>`
  documented *"Empty when nothing structural offers itself"*, so this
  needs no new mechanism.)
- **Containment** — no replacement exists at all.
- **Fragmented merged row** — the split's reasoning. Still unreachable
  until `two-emitter-refusals-a-legal-declared-union-reaches` lands.

**What this row is now**: make all three refuse, typed, with the refusal
saying which composition consumed the entity — not `Vanished` where the
author cannot tell a split from a containment. It is no longer a
decision; it is the application of one.

**The residue, stated so a taker does not re-derive it.** N3's own offer
is plural (*"the merged name vanishes with its constituents offered"*),
so a reading exists under which a split offers its fragment SET. The
ruling row takes the narrower reading and says why: N3's plural case is
an exact decomposition of what the merged name covered, whereas a
split's fragments are candidates for what the reference meant, and
offering candidates is one step from the silent pick the rule exists to
prevent. Disagree in a PR, not in silence.

Two things this row already establishes and a taker should not
re-measure: the `ContactContradicted` arm in four of the containment
orders (the wall IS a row at the step the pair is fed to), and that the
base refused six of six before the merge look-through, so that
look-through is a strict improvement and is not what is being undone.
