# INTENT stage 2, FORK-4 — re-pointing an operand (designer B)

## For Ev

**Recommendation: an operand slot is written like any other slot.** One
edit door writes every slot. An operand write is checked the way every slot
write already is: kind, liveness, acyclicity over the one dependency
relation, and DM5's distinctness. It counts as a *structural* edit. It
reports every downstream name whose resolution runs through the re-pointed
read, and refuses none of them. DM6 retires. What it got right is kept as
one sentence that D10 already says: **no edit infers a re-point.** Deleting
a node leaves its readers unresolved and never splices them onto an earlier
input. "Delete and reconnect" is a GUI composite of explicit slot writes
plus `DeleteNode`, committed as one action. The kernel never guesses which
input survives. Confidence: **likely** (high end).

Terms. An *operand slot* is a node field that reads a `Body`, `Face` or
`Edge` variable, such as a boolean's `a`/`b`, an extrude's profile or a
fillet's selection. *Re-pointing* means making a slot read a different
existing variable. A *list slot* is `Union.members` or `Loft.profiles`.
*`upstream(n)`* is the stage-2 dependency relation: the operations defining
what `n` reads, expanded through definitions.

### Premise check

1. **DM6 already permits re-pointing, but only for lists.** `SetMembers`
   (DM4) re-points list operands: it names the whole new list, infers
   nothing, refuses cycles and duplicates, and the node keeps its id. So the
   real rule today is "a `Body` slot may be re-pointed iff it sits in a
   `Vec`". Stage 2 makes a list `Vec<S>` and a field `S`, the same slot
   form. Keeping DM6 would therefore mean a union's member can be swapped
   but a boolean's `b` cannot. That special case has nothing to earn it.
   (sure)
2. **D-2's closure rule is not a reason.** It is `split`'s check that no
   recipe edge crosses the cut (`SplitError::SeveredEdge`, `refactor.rs`).
   It runs against the graph as it stands at the split door, and it caches
   nothing that a later rewire could stale. Stage 2 rewrites it to read
   `upstream`, and a slot write cannot reach across documents (VR9). DM6's
   own text never cites it; only the brief and the stage-2 spec do. (likely)
3. **The die chain argued against *inferred* splice, not explicit
   re-pointing.** DM4/DM6's walk-through costs a splice that "assumed
   intent about which input survives", on top of boolean names recording
   join depth (`FromA`/`FromB`). The flat union fixed the die. The
   no-inference half is now D10's own text ("never silently re-pointed").
   An explicit write names its new input, exactly as `SetMembers` does.
   (sure)
4. **Under D10, DM6 cannot be stated without a carve-out.** Reading is the
   only dependency, and stage 1's `SetParam` already re-points scalar
   reads. In stage 2, `upstream` expands through definitions, so these
   edits already move one operation's dependencies onto another:
   - `SetParam` on an assertion's `value` slot, made to read another
     `Measure`'s output;
   - redefining a variable that reads a measure.

   No edit vocabulary that keeps scalar slot writes also keeps "no edit
   rewires a live node's inputs". At most DM6 survives as "except
   reference-kind slots outside a list". (likely; FORK-5 decides how far
   scalars may read outputs, but the assertion case exists whatever it
   rules)
5. **D10's deletion rule needs a repair, and the re-point is that repair.**
   After stage 2, deleting a read operation is accepted, and its readers
   become `UnresolvedRead`. A scalar reader is repaired by `SetParam`. A
   name is repaired by `Rebind`. An operand reader under DM6 has no repair:
   it can only be deleted. That loses its id, strands its own readers, and
   cascades. "Unresolved, typed" would then be a dead end for exactly the
   reads that matter most. (sure)

**Provenance.** DM6 is agent-written text. It was ratified in chat on
2026-09-04 as one line of the DM1–DM6 bundle (PR 1789, merged by you). On
2026-09-16 you confirmed in chat that the splice issue owed no conversation.
The splice issue itself (`no-docedit-splices-a-deleted-node`) was filed at
your request, after cascade delete killed eighteen pips. I find no words of
yours ruling out an *explicit* re-point.

### The answers as final states

**A. Recommended: one slot door, operand writes included.**
- *Makes true:*
  - Every slot, scalar, list or single operand, is written by one door
    under one set of checks: `SlotVarKind`, live, `WouldCycle` over
    `upstream`, and DM5 over variable ids.
  - The node keeps its id; content and naming keys move as they already do
    when an operand changes.
  - The write is structural because the slot's kind is a reference kind,
    so the edit stream still separates structural edits from continuous
    ones (stage 1's split, decided by the slot kind, as `Count` is).
  - An `UnresolvedRead` gets a repair.
  - "Use that body instead", the GUI offer for references, becomes the
    same gesture as accepting a scalar offer.
- *Report, not refusal:* a re-point can change what a downstream frozen
  name denotes. The edit's `Applied.maintenance` lists each select or
  payload name whose resolution path crosses the re-pointed slot, as
  DM7 does for a removal. Two outcomes are possible:
  - **Loud.** The new operand mints different names: re-point an
    extrude's profile and its pieces' step ids change. Those names go
    `Vanished`/`NodeGone`, are reported, and are repaired by `Rebind`.
  - **Silent unless reported.** Old and new operands carry one table.
    Pass-through copies do (the die's pips are 21 transforms of one ball).
    Example: re-point chained `Boolean₃.b` from `Transform₃` to
    `Transform₄`. The rim fillet's name `FromA…(FromB(ball rim))` still
    resolves, now to pip 4's rim. That is very likely the intent, but it
    must not be invisible, hence the report.
  Refusing either outcome would forbid a legitimate edit to guard
  against a change the person just asked for.
- *Leaves possible:* graphs only an edit history could reach, for example
  a node reading something inserted after it. That is harmless:
  acyclicity is checked over `upstream`, and evaluation order is the
  schedule's, not insert order. Check that nothing assumes `order` is
  topological (see the orchestrator section).
- *Reversible:* yes. Narrowing to a rule (B) later is a refusal added at
  one door.

**B. Writable under extra rules** (same kind, no cycle, names re-checked
and refused if any would change). Same kind and no cycle are A's checks
already. "Names re-checked → refuse" blocks the repair case: after a
delete, the old names are by definition unresolved. It also blocks the
pip swap above, whose name change is the point. This is worse than A, with
nothing gained over A's report.

**C. Keep DM6.** Operands outside a list are never rewritten:
- It requires the list/field carve-out from premise 1, and the "except
  reference kinds" carve-out from premise 4.
- It leaves `UnresolvedRead` on an operand unrepairable.
- It forces delete-and-reinsert, which changes the id and strands every
  downstream select, witness, appearance and placement. That is the cost
  the splice issue named.

Its one merit is that it is the smallest stage-2 diff, and cost is not
weighed.

### Ratified text I would change

- **REFERENCES DM6:** retire it. In its place: "No edit infers a re-point:
  a delete never splices, and a read changes only by an edit naming its new
  variable."
- **DM7:** gains the re-point report alongside removal.
- **`no-docedit-splices-a-deleted-node`:** it is answered. Splice becomes a
  GUI composite, offered beside cascade delete, in which the person picks
  the survivor for each reader. Close the item, or make it a viewer unit.

**Confidence:**
- the recommendation: likely;
- premises 1, 3, 5: sure;
- premises 2, 4: likely;
- the claim that the report is computable from name segments mapped to
  slots: likely.

## For the orchestrator

- **Brief error.** The stage-2 spec (FORK-4) and this brief attribute DM6
  to "D-2's closure rule". DM6's text cites only the die and DM4. D-2 is
  `split`'s ancestor/consumer closure (`refactor.rs`, `SeveredEdge`), and
  the A12 operand-severed refusal is its twin. Neither is threatened by a
  write-time rewire. Treat the spec's attribution as unsupported unless the
  other designer finds a source.
- **For FORK-3 (selects).** If a `Face`/`Edge` variable is
  `Select { body, name }`, then the select's `body` is also a read.
  Re-pointing it should use the same door, and the spec's "Rebind becomes
  an edit of the select's name" covers only the name half. Keep the two
  forks consistent.
- **For PR B's sites.** The structural/continuous split exists today as two
  arms (`SetParam` / `SetStructuralParam`), each checked against
  `slot.is_structural()`. That is the slot kind declared twice in the edit
  stream. If operand writes join, write them through the structural arm.
  Better, consider whether the arm should be derived from the slot. This is
  off-question; I have not weighed it.
- **Unchecked: is `order` topological?** Check whether anything assumes
  `Doc::order` is a topological order, for example the load validator's
  `ForwardInput` (which the spec retires), `cascade_delete_order`, or the
  viewer tree. A re-point to a later node breaks that assumption, while
  `SetMembers` already admits it (does it? I did not check whether
  `SetMembers` refuses a member inserted after the union).
- **Unverified mechanism.** I assumed the report's "resolution path crosses
  the slot" is derivable from name segments (`FromA`→`a`, `FromB`→`b`,
  `FromMember{m}`→the member slot) plus selects whose body is downstream of
  the written node. I did not build it.
- **Unseen record.** The in-chat ratification of DM1–DM6 (2026-09-04) is
  not visible to me; PR 1789's body is the only record I read.
