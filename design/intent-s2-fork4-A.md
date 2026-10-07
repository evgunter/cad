# FORK-4 — re-pointing an operand
## For Ev

**Recommendation (sure).** After stage 2 an operand slot is written by the
same door as every other slot: *set this slot to read that variable*,
stated in full, nothing inferred. The write passes the checks the insert
door and `SetMembers` already make of a node's reads — kind, liveness,
acyclicity, DM5 distinctness — and **reports**, never refuses, the
downstream names it strands, as DM7 has a delete do. No edit infers a
rewiring: there is no kernel splice that picks a survivor. DM6 is rewritten
from "no edit rewires a live node's inputs" to "no edit *infers* one; a
read changes only by an edit naming the new variable in full".

**Premise check.** The fork asks whether operands "join the edit
vocabulary". Three things make it not a question:

- *D10 already requires the edit.* D10 (Ev-ratified, PR 3990) says
  "deleting a variable leaves its readers unresolved, typed, never
  **silently** re-pointed", and stage 2 retires `DeleteWouldDangle`. So a
  node with a stranded operand read is a sanctioned document state, and it
  needs an in-place repair as a stranded name has `Rebind`. Delete-and-
  reinsert is not one: ids are never reused, so every selection, witness,
  appearance and placement keyed by the reader's id orphans. D10 forbids
  the *inference*, not the edit.
- *Ev's own words* (transcript behind PR 3990): "no more dag edges from
  node to node. the role played by edges now is replaced by sharing
  variables". A slot that reads a variable is re-pointable by construction;
  a read-only slot kind is a special case that must earn its place.
- *Stage 1 and stage 2 already do it.* `SetParam` with a lone `Var` leaf
  re-points a scalar slot (the GUI's offer of an existing variable is that
  gesture), and the stage 2 spec makes `SetMembers` "a slot write on a
  list slot". What DM6 would then say is "list and scalar slots are
  writable, single operand slots are not" — a refusal protecting nothing
  that `SetMembers`' own checks do not already cover.

**Do DM6's reasons survive D10?** No.

- *The die chain* argued against splice **inference** over a pairwise
  chain, whose names record join depth. DM4's flat `Union` cured the chain;
  an explicit re-point infers nothing, and its naming cost (below) is
  reported where it happens. DM6 is Ev's (in chat, 2026-09-04, "possibly
  never") but predates D10, the later ruling, which chose "silently".
- *The closure rule* (a split's cut is closed under the dependency relation
  in both directions, `refactor::split`) reads the relation **as it stands
  at the cut**; nothing in it depends on edges being immutable (likely).
  What DM6 protected, unstated, is "insertion order is topological by
  construction" (`persist/check.rs`, `ForwardInput`). That invariant is
  already false: `SetMembers` accepts a member inserted after the union,
  and the saved document then refuses `ForwardInput` (reproduced on main;
  filed `set-members-admits-a-forward-member-the-save-validator-refuses`).

**The design, as a final state.**

1. **One door.** The slot write: the node has the slot; the formula lowers
   to a read of the slot's kind (`SlotVarKind` otherwise); the read is live.
   For an operand slot, the rewritten node then passes `check_node_inputs`
   (DM5) and acyclicity over reads — the same functions `SetMembers` calls
   today, not mirrors. A list operand is the same door with a list; that is
   what `SetMembers` becomes. Every slot kind has exactly this door; the
   record is structural whenever the dependency relation moved.
2. **Nothing inferred.** No "primary input", no survivor rule. The chrome's
   delete may *offer* a repair — re-point the N stranded readers to the
   deleted node's own operand, the user choosing where the node had two
   (`Boolean`, `Split`, `Revolve`; `Loft` has none) — as the stage-1 offer
   of an existing variable is an offer. Policy lives in the offer, never in
   the door.
3. **Strands are reported, not refused** (DM7's rule: a name is not an
   edge). After a re-point, every name downstream whose derivation nodes are
   no longer upstream of the site reading it — the question
   `DeclaredNameNotUpstream` and `resolve::upstream_nodes` already ask —
   rides `Applied.maintenance` as a strand; the N5 ladder diagnoses it at
   evaluation and `Rebind` repairs it. The chrome shows the count before
   commit, as cascade delete shows its dependent count.
4. **"Silently blends the wrong edge" is unrepresentable** (likely). An N1
   name spells its minting node and resolves only where its derivation path
   exists: `Cap@E` is in a `Transform(E)`'s table (the same cap, moved —
   correct) and is not in `Boolean(E, X)`'s, whose row is `FromA(Cap@E)`, so
   it fails typed. Re-pointing never makes a frozen name denote a different
   entity.
5. **One dependency relation.** `Doc::order` is presentation order; the
   spec's `Doc::upstream` answers every "before" question: the load door
   refuses a cycle over reads (the state no door produces) and drops the
   positional `ForwardInput`; `cascade_delete_order` closes over consumers
   instead of one forward pass; `DeclaredNameNotUpstream` asks the
   relation. Owed today regardless, by the filed defect.
6. **Keys and ids.** Upstream identity enters a content key by content, so
   a re-point re-evaluates the readers and nothing else; node ids are
   minted at insert and never move; naming keys move as under `SetMembers`.
   Split and inline are unchanged. A mate side (after F) re-pointed is the
   side re-authored, through the mate admission check `SetParam` already
   runs for a mate datum.

**Worked example.** The die: deleting a pip stays `SetMembers` without it
plus `DeleteNode`. A *pairwise* chain `B7 = B6 − pip7` under two fillets:
`DeleteNode(B7)` is accepted, `B8.a` is reported stranded, the chrome
offers "read `B6` instead", and on confirm the rim fillet's `FromA`-depth
names for pips 8–21 are reported stranded and repaired through N5's
offers. The cost DM4 measured is the same; it is now the user's loud
choice rather than an impossibility, and the flat `Union` remains the way
not to pay it.

**Ratified text to change.** REFERENCES DM6 (as above) and §0's "cycle-
checked at the edit door (`InsertNode`)" → at every door that writes a
read; `SetMembers`' doc "the one edit that changes a live node's inputs";
`SetProgram`'s doc "the plane cannot be carried, which is the whole of
DM6's claim" (still true of `SetProgram`; the plane is the operand door's);
ASSEMBLY A12's "a stranded operand is re-authored" (retires with PR F);
`no-docedit-splices-a-deleted-node` closes, answered as an offer over the
slot door, not a splice.

**Alternatives.**

- *A — keep DM6: single operand slots read-only after insert.* A stranded
  read has no repair but delete-and-reinsert; list and scalar slots
  writable, single slots not; a new refusal guarding nothing. Reversing the
  recommendation later is that one refusal, so A is where B can retreat.
- *C — the door plus a kernel splice* (a delete that re-points readers to
  a survivor). Rejected: inference in the door, "primary input" is policy,
  and it is exactly the silent re-pointing D10 names.
- *D — the door, refusing a re-point that strands a name.* Rejected: DM7
  chose report over refusal because a name is not an edge, and whether a
  name resolves is partly evaluation's question.

**Flag for FORK-3.** After E a fillet's `target` and its selections' body
state one fact twice, so re-pointing `target` alone is a nonsense edit; the
clean form is a fillet that reads edges, its body their common body.

**Confidence.** Recommendation: sure. Names unrepresentability (4): likely.
Closure-rule independence: likely. The `SetMembers` defect: sure
(reproduced).

## For the orchestrator

- The brief's "D-2's closure rule" is not cited by DM6's text, and I found
  no dependency of `refactor::split` on edge immutability; if a specific
  hazard was meant, it is not in `refactor.rs`. The positional-order
  invariant is the real one, and it is already broken.
- Defect filed on this branch:
  `work/recipe/set-members-admits-a-forward-member-the-save-validator-refuses.md`,
  reproduced with a temporary test (deleted, not committed).
- `SetParam` / `SetStructuralParam` restate `SlotId::is_structural`; a third
  `SetOperand` arm would restate it again. I lean to one arm deriving
  `structural` from the slot, but did not re-litigate stage 1's pair.
- Spec §3 says B keeps DM6 until ruled. B should route the list write
  through the general operand path with single slots refused, so this
  ruling lands as a refusal removal.
- Not checked: consumers of `EditRecord.structural` under a re-point; the
  viewer tree's row order when a reader precedes its input. I did not
  confirm which stage-1 PRs have merged; nothing here depends on it. Main
  moved during the lane (`f5014512` → `ac2c5db2`); nothing read changed.
