# FORK-2 — how the explicit product list is kept

## For Ev — round 4

**Recommendation (likely): no per-edit clause.**
- The insert, delete and re-point doors apply the default rule, and
  `SetProduct` is the one override. This is the other report's round-3
  design, which I now share whole.

**The correction is right.** I checked `DocEdit::InsertNode` in `edit.rs`
and `persist::Loaded`.
- The log stores edits as authored, and replay lowers them again through
  the doors. The record of what an edit did is handed back to the caller,
  not stored.
- So my round-2 "each door records the lowered change" would have been a
  new stored mechanism for the product alone.
- The product default is one more lowering at the door, in the place
  formula lowering already holds: `Formula` in the log, `Expr` in the
  snapshot.
- My replay argument therefore lapses. The hazard it named is shared by
  every lowering, and the pre-release policy (Band 4) already accepts it.

**What the clause would still earn: not a `DocEdit` field.**
- What is left is that a deviation would be one edit instead of two, and
  the log would read more clearly.
- A façade keyword, `product="append"` or `"leave"`, lowered to insert plus
  `SetProduct` in one history action, gives the person and the script the
  same one gesture.
- The one difference: grouping is the viewer's own state, so after a reopen,
  undo separates the two edits. The state in between is a valid document,
  so nothing breaks.
- A field on every body-defining insert and delete, for a property that few
  of them use, is a second spelling of a list change. It does not earn its
  place.

**Ratification text: accepted, with three sentences added** so the clause
is complete on its own:
- "A body-defining operation is one with a `Body` output; an operation
  defining no body, among them a mate, measure, assertion, select or
  gauge, is never listed and moves nothing."
- "Split and inline state their own list edits: the part's product is the
  cut's listed bodies in list order, the instance takes the first one's
  place, and inline splices the part's product back there. A cut holding
  no listed body refuses."
- "An empty product is a valid document; only the gather refuses,
  `EmptyProduct`."

**Is anything left for Ev?** No fork remains. Ev rules only on the
ratification text, because it replaces A10.

---

## For Ev — round 3

**Recommendation (likely): the two reports now converge. Nothing is left
for Ev to rule between them; Ev rules the whole design.** It replaces
ratified A10 text, so it needs Ev's sign-off.

1. A body-defining insert or delete, and re-pointing a body read, carries a
   product clause: `follow` (the default, and what a bare edit means),
   `leave` or `append`.
2. The door lowers the clause into a concrete list change and records that
   change: `Append`, `ReplaceAt { at, remove }` or `Leave`. Replay
   re-validates it like any list edit.
3. `follow` is A10's tip transfer, restricted to a *body-defining operation*
   (one with a `Body` output) reading a `Body` variable.
   - A body that reads no listed body appends.
   - One that reads listed bodies takes the earliest one's place, and the
     others leave the list.
   - Deleting a listed body puts its orphaned operands in its place.
   - Nothing else moves the list.
4. A delete always removes the deleted body's own entry. Under `leave` it
   reports the removal (`Maintenance::Unlisted`), so no dead entry ever
   exists.
5. Two edits state their own list changes:
   - split: the part's product is the cut's listed bodies, and the instance
     takes the first one's place;
   - inline: the part's product is spliced back into the instance's place.
6. Never listed and never moving the list: Promote, mates, measures,
   assertions and selects.
7. `SetProduct` states the whole list. An empty product is a valid
   document; only the gather refuses.

**What dropping the clause would buy, now that both forms record the
lowered change: almost nothing.**
- What remains is one authored field on three edits and in the Python
  constructor's signature. That field is invisible to anyone who takes the
  default, which every caller does unless they mean otherwise.
- The cost is real. Keeping a cut's tool becomes insert-then-`SetProduct`.
  The log then records a `ReplaceAt` that drops the tool and a `SetProduct`
  that puts it back, and an undo that splits the two lands between them.
- My round-2 move to the bare door rule answered A's round-1 objection that
  a clause "earns nothing". Once the change is recorded either way, that
  objection no longer holds.

**Does my round-1 argument stand? Yes (likely).**
- Of the two concerns, the replay concern is now met by recording the
  change, under either form.
- The intent concern still stands. With a clause, a deviation is one edit
  that says what was meant; without it, a deviation is a correction of
  something the door just did.
- I return to the clause, which is where A now stands.

**Is any remaining difference worth Ev? No.**
- The delete-under-`leave` disagreement is gone: I conceded in round 2 that
  the entry is removed and reported.
- I accept A's wording ("a body-defining operation reading a `Body`
  variable"). The two wordings agree on every node today.
- One small difference remains: a split whose cut holds no listed body.
  - I would refuse it at the split door.
  - A's design would let it mint a part whose instance refuses
    `EmptyProduct` at every evaluation.
  - This is an implementation-level refusal, for the PR C implementer to
    settle. It is not a design fork.

**Confidence.**
- The rule: sure.
- The clause, lowered and recorded: likely.
- Nothing left to fork between the reports: sure.

---

## For Ev — round 2

**Recommendation (likely), revised.**
- The kernel's insert, delete and re-point doors apply the follow rule as
  their default effect, as the other report proposes. Nothing re-derives the
  list, and `SetProduct` is the explicit override. I drop the per-edit
  clause.
- I add one thing: **each door records the list change it made as data.**
  The edit stored in the log carries the resulting list change (or the door
  appends a `SetProduct` to the log). Replay then reads the change and never
  re-runs the rule.
- Both reports already agree on the rule: A10's "the tip replaces its
  operands", restricted to operations that define a body.

**The other report's strongest point, conceded.** A clause on every insert
and delete earns nothing that `SetProduct` does not already say.
- Every deviation is one more edit, and the viewer already groups an insert
  plus a `SetProduct` into one user action.
- One mechanism beats two. A person keeping a tool beside a cut sends cut
  plus `SetProduct`, and undoes it as one action.

**Where the other report is wrong.** Its reversibility claim is that "the
default is sugar over `SetProduct`, so dropping it changes no stored data".
That holds only if the door's effect is stored.
- A save is a snapshot plus the edit log since that snapshot, and load
  replays that log through `apply` (`persist/mod.rs`, Format).
- With the rule applied only at the door, a stored `Insert` means "insert,
  and change the list by whatever the rule says at load time". Revising or
  dropping the rule therefore changes the product of every saved document
  whose log holds a body insert after its snapshot.
- Recording the door's change makes the claim true. The rule then binds
  only new edits.

**Does my replay argument hold? Yes, but it is narrow (sure).**
- It covers only the log after the snapshot; the snapshot's product is
  stored outright.
- Most changes to the rule would arrive with a format change, and an older
  file refuses with the regenerate recourse anyway.
- So the argument is about getting the record right, not a weighty user
  hazard. It costs one recorded list change per body edit. That price is
  what makes "reversible, changes no stored data" true rather than nearly
  true.

**Delete, conceded.** Deleting a listed operation removes its entry, and its
unlisted orphans take its place.
- No edit can leave a dead entry in the list, so my round-1
  unresolved-entry state is unrepresentable.
- That is better than refusing it at the gather. D10's "never silently
  re-pointed" is about readers; it does not apply here, because the entry
  *was* the deleted thing and the edit that removed it says so.

**Also agreed.**
- The other report's re-pointing rule (`SetMembers`, or an operand write if
  FORK-4 admits one): the rule applies to the reads it adds and drops, at
  the operation's own entry.
- An empty product is a valid document; only the gather refuses
  `EmptyProduct`.

**One residual difference: split of a cut with no listed body.** I would
refuse it at the split door rather than mint a part whose instance refuses
`EmptyProduct` at every evaluation. A refactor that yields a broken
instance should fail where it is asked. Confidence: likely.

**Convergent final state.**
1. The follow rule runs at the insert, delete and re-point doors.
2. Each door records the list change it made.
3. `SetProduct` is the override.
4. Every stored entry is live and a body.
5. Split and inline state their own list changes.
6. Promote, mates, measures and assertions never touch the list.

## For Ev — round 1

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

**Other clause values:** `leave` (no list change); `append` (keep the
operands and add the result: D10's "operand stays first-class", e.g. a cut's
tool); an explicit position or replacement set. `SetProduct { bodies }`
remains the whole-list edit.

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
   - Reversible toward the recommendation only by adding the clause. Second.
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

**Confidence.** Where the rule lives: likely. The rule is A10's restricted
to body reads: sure. Delete-`leave` leaves an unresolved entry rather than
dropping it (D10's reader rule): likely. An empty-product cut refuses at
split: likely.

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
- The spec drops Promote's slot insert (`edit.rs:6030`); this agrees.
- **Tests.** Test 6 is compatible: the migration writes today's root bodies
  as the list. Mates leave the list, so root-count rows shift by the mates.
