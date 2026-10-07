# FORK-6 — split and named variables (designer A)

## For Ev

**Recommendation (sure):** split **moves** a named variable whose
readers all go into the part: the part declares it under its name, and
the remainder no longer holds it. A variable that something staying
behind still reads keeps refusing `UncutVarReference`, as it does
today for named and anonymous variables alike. The promise is
**`inline(split(d))` is `d` up to minted ids** (node, step and
variable ids) and A10's one regrouping. That is A4's acceptance with
"node ids" widened to every minted id. `split(inline(h))` is not
promised.

**Premise correction (sure).** The brief's option (a) says a named
variable read on both sides "stays in both as two ids". Today split
does not do that: it refuses *any* variable read on both sides,
named or anonymous (`UncutVarReference`, D-2's "no silent sharing",
older than PR C). `place_mate_frame_offset` already tests the named
case. So (a) and (b) are the same answer. The only open case is a
named variable that **only the cut** reads.

**What the issue is evidence of (sure).** Split copied such a
variable: one in the part, and a stale twin left in the remainder.
Inline's merge-by-equal-value was what quietly folded the twin back
in. The two halves were a matched pair, and the ruling removed one
half. The twin is a defect in its own right, round trip aside. After a
split the remainder shows a named `slide` that drives nothing. Editing
it changes nothing, though it reads as if it set the part's slide. One
intent ends up in two variables that nothing keeps in step. The fix
is to stop making the twin, not to fold it back later.

### The rule

Call a variable **cut-only** when every reader is cut. A reader is a
node slot, or the definition of a variable that stays in the
remainder; definitions are followed through. A **shared** variable has
a reader on each side. An **unread** named variable has no reader at
all.

| Variable | Split does | Inline does |
|---|---|---|
| anonymous, cut-only | carries it (today) | carries it as a new id (today) |
| named, cut-only | **moves it**: declared in the part under its name, removed from the remainder | carries it as a new id under its name; refuses `VarNameConflict` if the host holds the name |
| shared, either kind | refuses `UncutVarReference` (today) | n/a |
| unread named | stays in the remainder (VR7) | n/a: inline carries the part's whole table |

A reader may be a variable as well as a node. Take a remainder that
holds an unread named `k = w + 1` while the cut alone reads `w`. Then
`w` is shared and split refuses, naming `k`. Today this case passes,
because the remainder keeps `w`. If the move counted only node readers,
`k` would be left unresolved.

**Worked example.** `d` holds `slide = 2 mm`, read only by the cut
seat's offset. `split` gives a part holding `slide` and a remainder
without it. `inline` carries `slide` into the remainder under a new
id; the name is free, so it lands as `slide`. The result is `d` with
`slide` under a new id. Now say the person declared another `slide`
in the remainder between the two steps. Inline refuses
`VarNameConflict`, which is the ruling working as meant, not a broken
round trip.

### Sub-question: is the removal a move or a deletion? A move (sure)

VR7 keeps an unread named variable so that detaching its last reader
inside one document does not lose a name the person chose. D10's
deletion rule is about readers left unresolved. Neither applies. The
variable, its name and its definition carry on in the part. Every
reader it had is there and reads it. The remainder keeps no reader
that could be stranded. This is how a cut node and its label already
behave: removed from the remainder, never called a deletion, because
they carry on in the part. "The kernel never drops or mints a name"
(VR2) holds: the name exists once before and once after. No new
lifecycle rule is needed. In the edit list, the removal is
`DeleteVar` on a variable with no readers, and it strands nothing.
A4's *Split* paragraph should gain one sentence saying so, in the
words of the table above.

### Shared variables: refuse, or cross as a caller-supplied argument?

**Refuse, for now (likely).** Crossing as an argument would mean the
part declares a free variable, the instance binds it to the
remainder's variable, and inline substitutes the binding. That is
the general form, and the round trip would survive it. But nothing
today can express it: `InstantiatePart` holds `doc_ref`, `interface`,
`gauge` and `offset`, and no binding. The closest thing is the offset.
Its rigid steps are slots, so a host variable can already drive *where*
the part sits. That is why `UncutVarReference` offers a promote as the
recourse for a shared offset variable. Making a part's value depend on
its caller is a feature of its own: a parametric part, or a
configuration. It needs its own decisions about what the content pin
covers, defaults, how the analysis lanes cross the seam, and what a
`DocRef` names. Split should not invent that on the side. If you want
parametric parts, that is a separate design row. If it lands, split
gains this third arm and the inverse property above still holds.

### The other answers, as final states

- **(c) leave split alone.** The stale twin above stays in the
  remainder, `inline(split(d))` refuses whenever the cut reads a
  named variable, and A4's ratified acceptance no longer holds. The
  person must rename or delete a variable they never asked to have.
  Reject (sure).
- **Inline merges a same-name variable that nothing in the host
  reads.** This brings merging back by another door, and VR1/VR2 rule
  it out: identity is decided by "unread", not by the person. It also
  leaves the stale twin in place between the two steps. Reject (sure).

### Ratified text touched

- ASSEMBLY A4, *Acceptance*: "up to node ids" becomes "up to minted
  ids". This is a re-wording that VR1 forces (variables now have
  minted ids), not a second decision (likely). I could not trace this
  sentence to Ev's own words: `git log -S` lands on a regenerated
  STATUS commit.
- ASSEMBLY A4, *Split*: add the move sentence. It states a new
  behaviour, so it waits on your sign-off with this fork.
- `refactor.rs`'s comment "the remainder keeps its table either way:
  a named variable nothing reads is legal document state" was written
  by an agent in a fix pass (`0eeb19d3a`), not by you. It goes.

**Reversibility:** easy; one place in split, inline unchanged.

## For the orchestrator

- **Brief error:** split already refuses every variable read on both
  sides, named too (`refactor.rs`, "Variables: read by cut nodes";
  also on `1dbd9fe4b`, before PR C). The second half of
  `the_offset_and_its_parameter_cross_split_and_inline` tests it.
- **Implementation notes for the lane.**
  1. Today `cut_refs`/`kept_refs` count node readers only
     (`node_var_reads`). The move needs kept readers to include the
     definitions of remainder variables that are not moving. That
     means a fixed point: a cut-read variable moves iff no kept node
     reads it and no non-moving variable's definition reads it. Unread
     named variables are non-moving.
     `UncutVarReference.kept_node` then becomes a reader (node or
     variable). That is a new refusal on a case accepted today. Give it
     a row.
  2. The remainder gains a `DeleteVar` per moved variable, after
     the cut nodes go; assert `Maintenance` reports no strand.
  3. `same_up_to_ids` must map variable ids, not compare them.
  4. Add rows: (i) a named cut-only variable is absent from the
     remainder and present in the part; (ii) the `k = w + 1` refusal;
     (iii) a named variable declared in the remainder between split and
     inline refuses `VarNameConflict`; (iv) an unread named variable
     stays in the remainder.
- Not checked: the Python façade's `split` and the viewer's variable
  table after a split; look in the same PR. Possible idea row, not
  filed: parametric part instances (the general form of the shared
  case), only if Ev wants it. The three failing rows should pass
  under the move unchanged, bar (3).
