# FORK-2 — how the explicit product list is kept (designer A)

## For Ev

**Recommendation (likely): keep the product explicit, and move it by one local rule at the edits that make or unmake a *body-defining read*, plus `SetProduct` outright. Nothing ever recomputes the list from the graph.**

Terms. A *body-defining operation* is a node with a `Body` output: extrude, revolve, boolean, union, fillet, chamfer, shell, transform, pattern, split, projection (`Part`), instance. A Measure, Assertion, Mate, Select, Gauge, Datum or Profile defines no body. A *body-defining read* is such an operation reading a `Body` variable. "Listed" means in `Doc::product`.

The rule, stated at four doors and nowhere else:

1. **Insert** of a body-defining operation: its body enters where the first listed body it reads sits, and every listed body it reads leaves; reading no listed body, it appends. Any other insert moves nothing.
2. **Delete** of a listed operation: its entry leaves, and at that position enter, in document order, the live bodies it read that are not listed and that no other body-defining operation reads. Deleting an unlisted node moves nothing.
3. **Re-pointing a read** (`SetMembers` today; an operand `SetParam` if FORK-4 admits one) applies 1 to the reads it makes and 2 to the reads it unmakes, at the operation's own entry; an unlisted operation's re-pointing moves nothing.
4. **`SetProduct`** states the list outright: live, duplicate-free, `Body` only (the spec's `ProductFault`). An empty product is a valid document; only the gather refuses, `EmptyProduct`.

Split, inline and Promote compose these and add no rule: the part's product is the cut's listed bodies in list order and the instance takes the first one's place; inline splices the part's product at the instance's entry; Promote moves nothing, because a gauge is not a body. No mate, gauge, measure or assertion is ever in the list, so the guide's "a mate is a product root, and it is still a surprise" paragraph goes.

**What this makes true.**

- By default the product is the *frontier of body construction*: the bodies no body-defining operation has built on. That is A10's rule with non-body readers struck out, applied at the edit rather than re-derived afterwards, so every viewer flow and the guide's assembly walk read exactly as today for body operations, and the tour's root counts become product lists with the same members.
- A Measure, Assertion or Mate reading a body never strips it: the cut plate is its own product, and the bench stand's product is its three instances.
- A stated deviation survives unrelated edits: a probe boolean left out by `SetProduct`, an operand kept beside its fillet, a narration body unlisted. Inserting a measure afterwards, or filleting another body, touches none of it. This is what an explicit list buys over a derived one, and it is why rule 2 restores only what no body-defining operation still reads: after fillet F over A and boolean G over A and C, the list is [F, G]; deleting G gives [F, C], the state before G, and A stays unlisted because F still builds on it.
- The list is never recomputed wholesale, so the document's product changes only at the edit whose subject is listed or reads a listed body. A script sees `doc.product` move exactly when it inserted or deleted a body operation or said so.

**What it leaves possible that may look wrong.** [A, F(A)] is representable: one material twice. It is a legitimate stated product (a before/after document), and the one case where it corrupts the gather, two entries carrying one body's names verbatim through transforms or projections, is already refused by `PlacedUnderTwoRoots`; a fillet mints its own names, so [A, F(A)] gathers. The viewer draws the product, so after a projection the split's other half leaves the picture, as today. That is the product's truth; seeing superseded operands is display state, which is how Ev put it on 2026-10-03 (operands stay viewable as placement displays, coloured apart from "stuff that is actually in the main space"). `a-projected-split-is-unreachable-from-the-viewport` is the display half and stays open.

**Reversible.** The default is sugar over `SetProduct`: dropping it (option B) or adding an insert-time intent (option D) changes no stored data.

**Premise check.** The fork is stated right, with one correction: "no rule falls out of the structure" is true of the graph after the edit and not of the edit itself. An insert knows what it reads at the moment it is applied, and that is enough to state a default without ever asking the graph who consumes whom. The second thing the fork folds together is "what is drawn" and "what is produced"; Ev's remark above separates them, and the design keeps them separate: product is main-space material, the rest is display.

**Ratified text.** D10's "the product is an explicit list of `Body` variables" stands. It was the assistant's proposal in the 2026-10-03 conversation, approved in passing (Ev's own words were "no more nodes that consume other nodes"), so it is weaker evidence than a sentence Ev wrote, but nothing here argues against it. What changes: A10's maintenance clause and its "the root set is exactly the sink set" sentence (the PR C text above replaces them), A12's "an ordinary non-body root", A4's acceptance wording, and the guide passages the spec lists. Ev's 2026-09-09 ruling on narration bodies (`[ev]` PR 2231, option D: no declared root set) rested on the sink rule and is overtaken by D10; the PR should say so.

**Alternatives weighed.**

- **B. Explicit only, no maintenance.** Every insert leaves the list alone; a script's first extrude yields an empty product, and every viewer flow sends `SetProduct`. Purest, and hostile to both a person and a script: the first thing anyone does is draw a body. A contains B, since the default is sugar over the same door. Lean against.
- **C. Derived: the product is the `Body` outputs no body-defining operation reads, with only the order explicit.** No maintenance, nothing stale, and it fixes the cut plate and the mates too. It cannot say "produce A beside F(A)" or "leave this probe out", so a narration body is back to a second document, and it reverses the ratified sentence. The strongest competitor if deviations are not wanted; D10's thesis is that intent is said, so lean against.
- **D. An intent on the insert** (`produce: Supersede | Aside`). One edit and one undo for a probe body. It is a special case `SetProduct` already covers, and the viewer's several-edit door batches insert-plus-`SetProduct` as one action, so it earns nothing today. Addable later without moving data.
- **A10 over reads unchanged** (every reader supersedes). Dead: it is the cut-plate bug generalised, which the spec already says.

**Confidence.** Recommendation: likely. "The default reproduces today's experience for every body operation": sure, it is A10's rule restricted to body-defining readers at edit time. "Verbatim-chain double gathering is already refused": likely. "Ev's placement-display remark supports product ≠ drawn": likely; it is one sentence, read in context.

## For the orchestrator

- **Provenance.** The checkout is shallow, so `git log -S` on A10's phrases hits the graft (2026-09-20). The ASM-ROOTS spec is ledgered and deleted; I could not read it. Ev's verbatim words behind D10 are at commit `5f7a1c71e3` (`docs/ev-transcripts/2026-10-03-...`), fetched through the GitHub API since the commit is outside the clone. "Explicit list" is there only as the assistant's proposal ("nodes disappearing into a fancy operation on these variables" got "3. cool").
- **Spec alignment.** PR C's text matches this report except that `on_set_members`' wholesale recompute becomes rule 3's local splice, and the spec should say outright that an empty product is a valid document (today coverage makes every non-empty document have a root). Promote's slot insert goes, as the spec says.
- **FORK-1 interaction.** Whether a pattern's `Instances` is one listable `Body` variable decides if Duplicate's two-projection hack can go; display state keys hidden instances through `instances_by_root`, which must walk reads-ancestry after B.
- **Viewer and guide rows.** Each combine op's row states the product afterwards; under the default they read as today. `PROJECTION_HIDES_THE_REST` stays true.
- **Not checked.** The part-2 transcript (`3d70e5de72`) is about coincidence only. No defect filed off the question.
