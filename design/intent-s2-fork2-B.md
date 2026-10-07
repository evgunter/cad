# FORK-2 — how the explicit product list is kept (designer B)

## For Ev

**Recommendation (likely).** The product list changes only when an edit says
so. The kernel holds no rule that guesses it. An edit that makes or deletes a
body carries a **product clause**. Its default spelling is `follow`, and the
edit door lowers `follow` into the exact list change, which is what the
document records. This mirrors VR6, where a `Formula` is authored and an
`Expr` is stored: `follow` is authored, and the concrete change is stored.
The viewer and Python both get today's behaviour without asking for it. The
list, once written, is never re-derived from the graph.

**Terms.**
- *Listed*: in `Doc::product`.
- *Body read*: an operation reading a `Body` variable directly in one of its
  operand slots. Reading a `Face` or `Edge` selection of a body does not
  count, and neither does a measure, mate or assert, because none of those
  defines a `Body`.
- *Orphan*: a live, unlisted body that no live body-defining operation reads.

**The `follow` rule, lowered at the door:**
- **Inserting a body that reads no listed body** (a box, an instance, a body
  made from unlisted tools): its `Body` outputs are appended, in port order.
- **Inserting a body that reads listed bodies** (a boolean over two listed
  bodies, a fillet, a transform, a pattern): its outputs take the place of
  the earliest listed operand, and the other listed operands leave the list.
  The tip replaces its operands, as today.
- **Inserting anything that defines no `Body`** (a measure, mate, assert,
  select, gauge or variable): the list is untouched. This is what fixes the
  cut plate: a measured part stays listed. A mated instance stays listed
  too, and mates are never listed.
- **Deleting a listed body**: its orphaned operands, in document order, take
  its place.
- **Deleting an unlisted node**: the list is untouched.

**Other clause values.**
- `leave`: no list change.
- `append`: keep the operands and add the result. This is D10's "operand
  stays first-class", for example keeping a cut's tool.
- An explicit position or replacement set.
- `SetProduct { bodies }` remains the whole-list edit.

A raw delete with `leave` follows D10's deletion rule. The product entry is a
reader, so it stays in the list unresolved and typed, and the gather refuses
it by name, with the recourse "remove it from the product or list another
body". It is never silently dropped or re-pointed.

**Split, inline and Promote.**
- Split and inline are refactors that must preserve the product, so their
  list edits are forced and stated in the edit lists they return. The part's
  product is the cut's listed bodies in list order. The instance's output
  takes the first one's place in the remainder's list. Inline splices the
  part's product back into that place. A cut that holds no listed body
  refuses, because the part would have an empty product.
- Promote mints a gauge, which is not a body, so the list is untouched.

**What people and scripts see.**
- Python's `Doc.product` and the viewer's product badge read the stored list
  and nothing else.
- In the viewer, a combine operation has a "keep operands" toggle that sends
  `append`, and the product panel can reorder or unlist bodies through
  `SetProduct`.
- A script never has to state the product, but can.
- The guide's assembly walk changes in two places: mates leave the list, and
  `set_roots` stops refusing "uncovered", because coverage retires.

**Premise check (sure).** The fork is not "which rule"; any sane rule is
A10's tip transfer, restricted to body reads. It is *where the rule lives*.
- Under A10 the rule was the bookkeeping of an invariant: the list was the
  sink set, so the door had to maintain it.
- D10 removes the invariant. A rule left at the door would now *infer
  intent*: "you read a listed body, so you meant to replace it."
- The project already settled where an intent guess lives. VR2 says the
  kernel mints no name and the client proposes one. D10 says typing a value
  *offers* a variable; it does not bind one. The product follows the same
  pattern: a guess may be proposed by a default, but what is stored is what
  was stated.

**Ratified text to change.**
- A10's maintenance sentence and its two invariants are replaced by the
  above. D10's closing paragraph already retires A10's sink rule.
- A4's acceptance clause, "the cut's roots come together where the first of
  them was", keeps its meaning and is restated over the product.
- A12's "a mate is an ordinary non-body root" goes.

**Alternatives, as final states.**
1. **Door rule (the kernel follows silently).** Each insert or delete
   applies the same rule as an implicit side effect, and `SetProduct`
   overrides it.
   - Same everyday behaviour, one fewer concept, scripts unchanged.
   - But each edit changes a second datum it never mentions. Keeping an
     operand takes two edits that fight: the insert drops it, then a
     `SetProduct` puts it back.
   - The saved edit log replays through whatever the rule is at load time,
     so revising the rule changes the product of every document replayed
     from its history.
   - Reversible toward my recommendation only by adding the clause anyway.
     I rank it second.
2. **Purely manual (no default at all).** The person or the script states
   every product change.
   - Honest, but every viewer creation flow gains a step, and so does every
     script.
   - Forgetting to list a new body shows up only as "empty product" at
     evaluation.
   - Bad experience for no semantic gain over a stated default.
3. **Derived, with no stored list** (the product is the bodies no
   body-defining operation reads).
   - It contradicts D10's ratified "explicit list".
   - It cannot express "keep the tool", which D10's operand rule exists to
     allow.
   - Rejected.

**Reversibility (likely).** The recommendation records concrete list changes,
so the `follow` rule can be revised later without moving any saved document.
Moving to the door rule would be a one-line change of where `follow` is
applied. The reverse move would need every edit to grow a clause.

**Worked example.** `a = box` → `[a]`; `b = cyl` → `[a, b]`;
`c = cut(a, b)` → `[c]`; `m = measure(c)`, `assert(m)` → `[c]` (today `c`
stops being a sink); `f = fillet(c, append)` → `[c, f]`; `SetProduct [f]` →
`[f]`; delete `f` (follow) → `[c]`, since `c` is an orphan again; delete `c`
(leave) → `[c†]`, `†` unresolved, and the gather refuses naming `c`.

**Confidence.**
- Where the rule lives: likely.
- That the rule is A10's restricted to body reads: sure.
- That delete-`leave` leaves an unresolved entry rather than dropping it:
  likely. This follows D10's reader rule.
- That an empty-product cut refuses at split: likely.

## For the orchestrator

- **Provenance.** D10's ratifying PR is #3990 (`[ev]`, closed). I could not
  confirm, in Ev's own words, the phrase "the product is an explicit list"
  from the shallow clone's `git log -S`: every hit is a graft or merge. I
  took it as Ev-ratified.
- **Mechanics left to the implementer.**
  - Whether the lowered clause is recorded inside `InsertNode`/`DeleteNode`
    or as a following `SetProduct` in the same history entry. The viewer
    already groups the edits of one action (`crates/viewer/src/history.rs`).
    The saved log flattens edits, so either form replays exactly.
  - Python could take the clause as `DocEdit.insert(node, product="follow")`,
    with `follow` as the default.
- **Not covered by the fork list: `SetMembers` / slot re-pointing.** It
  touches FORK-4. Under `follow` I would apply the same rule to the reads it
  adds and drops: listed added reads leave, and orphaned dropped reads
  append. Until FORK-4 rules, `leave` is defensible.
- **Viewer draws only the product.** `display.rs` (`instances_by_root`)
  keys the drawn scene by the list. Unlisted bodies, including a kept-out
  tool, are invisible, as today. Whether "drawn" should widen beyond the
  product is a separate viewer question; I did not design it.
- **The spec's sites.** The spec says Promote's slot insert (`edit.rs:6030`)
  goes. Under this answer Promote needs no product code at all, which agrees.
- **Test 6 (migration preserves the product).** It is compatible: the
  migration writes today's root bodies as the list.
- **Tests.** Mates leave the list; the root-count rows shift by the mate count.
