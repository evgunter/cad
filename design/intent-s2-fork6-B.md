# FORK-6 — split moves a variable with its readers; inline never merges

## For Ev

**Recommendation.** Split *moves* every variable the cut reads: one whose
every reader is cut leaves the remainder (a recorded `DeleteVar`) and is
declared in the part, named or anonymous alike; one read on both sides keeps
today's refusal, `UncutVarReference`. Inline carries every part variable
into the host under a new id and refuses `VarNameConflict` on a name the
host holds. The promise is one-directional: `inline(split(d))` is `d` up to
node and variable ids for every cut split admits. This is the brief's (a);
its "both sides" half is already true. Confidence: sure on move-over-copy,
likely on the closure rule and on refuse-over-argument below.

**Premise corrections.**

1. Split already refuses a *named* variable read on both sides.
   `UncutVarReference` filters nothing by name (the `cut_refs`/`kept_refs`
   pass in `split`), and
   `the_offset_and_its_parameter_cross_split_and_inline` asserts it for the
   named `slide`. PR C extended the refusal to anonymous variables; it did
   not create it. So (a) and (b) are one option, and the only open case is
   a named variable read by the cut alone.
2. Anonymous variables already move. Deleting the cut nodes detaches them,
   VR7's lifecycle removes them from the remainder at every edit door
   (`edit::door`), and the carry declares them in the part. Only named
   variables are *copied*. Today's split and today's inline were two halves
   of one value-identity mechanism: split duplicated the name expecting
   inline to re-identify the copy by equal value. The ruling removed the
   second half; this fork is the first.

**Why a move, not a copy.** VR2 makes a name text held beside the variable;
it does not change what the variable is, so it cannot change which document
the variable goes with at the seam. A copy left in the remainder is unread,
carries a name the host did not ask to keep, and is "one variable become
two" (D-2's no silent sharing), the very thing the inline ruling forbids
reading back. Nothing can read the remainder's copy that could not read the
part's, since a part reads nothing of its host, so the copy serves no one.

**The sub-question: move, or a deletion with its own rule?** A move. VR7
keeps an unread named variable because an ordinary edit that detaches its
readers has not asked about the declaration. Split is the one edit whose
meaning is "this goes to another document", and a variable goes where its
readers go, as a node does: `DeleteNode` plus `InsertNode` is a node's move,
`DeleteVar` plus `DeclareVar` is a variable's, both recorded in the edit
lists split returns. D10's deletion rule is met: no reader is left in the
remainder, and every part-side reader is re-pointed by the recorded declare,
nothing silently. No new rule, no `Maintenance` (nothing read it when it
went).

**The rule, as a closure.** A cut node's side is cut, a kept node's kept. A
variable's side is the union of its readers' sides, a reader being a node or
a definition (the lifecycle door already counts a definition as a reader:
`unread_anonymous_vars`). All cut: moves. Cut and kept: refuses
`UncutVarReference`. All kept, or no readers and no reads: stays. An unread
named *defined* variable follows its reads: all moving, it moves; mixed, it
refuses, since its definition reads across the seam. Worked example: `w` is
named and read by a cut extrude; `w2 = 2*w` is named and read by nothing.
`w` moves and `w2` moves with it. Left behind, `w2` would read a deleted id
(legal under VR7, but stranded by a refactoring the person asked to be
lossless) and no inline could ever restore it; moved, the round trip is
exact.

**Both sides: an argument, or a refusal?** A refusal, today, and the refusal
is where an argument mechanism would plug in later. A part reads nothing
from its caller: `InstantiatePart` holds a document reference, an interface
record, a gauge and an offset, and no bindings. `SetOffset` and `SetProgram`
"arguments" are the host's own slots with VR6 fresh tables; stage 2's ports
are outputs an operation defines, not inputs a document takes. Making a
both-sides variable "something the part reads from its caller" means parts
become functions with per-instance bindings; VR9 settles "a variable
belongs to its document", and the stage-1 item deferred per-instance
arguments. That is its own fork, not one split should decide by
implication. D-2 already refuses a node read across the seam rather than
inventing a port for it; a variable read across the seam is the same
severed read. Should part arguments come, this same site turns the refusal
into "cross as an argument bound to the host's `w`", and inline inverts it
by substitution; nothing decided here resists that.

**The inverse promise.** `inline(split(d, cut))` is `d` up to node and
variable ids for every cut split admits; the round-trip comparator already
matches named variables by name. `split(inline(h, i), image)` is `h` up to
ids *plus* any unread named variable the part held: inline carries it into
the host (it never drops a name), and split has no reader to send it back
with, so it stays the host's. That is the one asymmetry. It follows from
VR7 and the never-drop rule, and it is right: the person sees the variable
and can delete or rename it, where a kernel that chose a side for it
silently could not be undone.

**Consequences.** The three failing rows pass with no test change. The
remainder's edit list gains the `DeleteVar`s. `UncutVarReference`'s text and
promote recourse stand. `VarNameConflict` keeps its name; its recourse
becomes "rename either variable", since today's "define this document's as
the referenced document's" names the merge the ruling retired. Reversible:
a copy can be reintroduced by not recording the deletes, and nothing
downstream stores which happened.

**Rejected.** (c), leave split alone: keeps the forward half of the
value-identity inference and makes every round trip a manual delete. Inline
leaving a conflicting carried variable unnamed: drops a name (VR2). Split
refusing a named variable the cut alone reads: makes the name decide a
lifecycle, against VR2, and treats named and anonymous differently at the
one seam where they should not differ.

**Ratified text.** A4's acceptance (`crates/editor-core/ASSEMBLY.md`,
Ev-ratified 2026-10-03): "Inline-of-split returns the document split was
given, up to node ids and that one regrouping." The move keeps it; (c)
would retire it for every document whose part reads a named variable. One
re-wording lands with the change, not as a decision: "up to node and
variable ids". The inline ruling already forces it, since a carried named
variable now returns under a new id, as anonymous ones have since PR C, and
nothing reads a variable's id but its own document (VR1). VARIABLES-DESIGN
is agent-written and was not put before Ev
(`variables-are-identities-with-labels`: it "did not go to Ev"); it gains
one line at VR9: "Split moves a variable with its readers; inline carries
every variable of the part under a new id." The refactor module doc states
the closure.

## For the orchestrator

- The brief's option (b), "split refuses a named variable read on both
  sides", is the present code: `UncutVarReference` has no name filter, and
  the pmfo row's second half asserts it on the named `slide`. The lane may
  have read the literals spec's "now for anonymous variables as well" as
  "only anonymous".
- Implementation sketch: after `split`'s reverse-order `DeleteNode`s,
  record `DeleteVar` per moved named variable in reverse definition order
  (readers before what they read). Compute sides over node reads plus
  `definition_readers`, not `node_var_reads` alone, so an unread definition
  following its reads is covered. The mixed-definition refusal needs a
  reader that is a variable: `UncutVarReference` names a `SpokenNode` on
  each side, so either widen `kept_node` to a node-or-variable spelling or
  add a sibling arm. Change `VarNameConflict`'s recourse text.
- Not checked: whether `DeleteVar` on a variable another definition reads
  is refused or merely structural. The door doc says readers are left
  unresolved; the reverse-definition-order deletion avoids the question
  either way.
- Off-question, not run: PR C's load row "an anonymous variable read only
  by an unread definition refuses at load" against `unread_anonymous_vars`
  counting a definition as a holder looks like two notions of "read" for
  the anonymous lifecycle, door versus load. Worth filing if it holds.
- The stage-2 spec has split's closure read `Doc::upstream`. The closure
  above is that relation extended to free variables; stage 2 should fold
  both into one pass rather than keep a node closure and a variable
  closure side by side.
