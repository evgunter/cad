---
id: the-pair-verbs-declared-merge-is-asymmetric-in-its-operands
kind: ruling
title: A declared merge is asymmetric in the pair verb's operands: which member's rims fragment follows the A/B assignment
status: open
opened: 2026-09-06
refs: [2028, does-n3-retire-loudly-generalise-to-the-folds-other-compositions]
priority: P0
cost: D
---


## What

Found by both DOCM-7 reviewers, independently and by execution (R1's
`r1_c6_the_reorder_asymmetry_is_the_pair_verbs`, R2's
`r2_the_pair_boolean_is_asymmetric_in_its_operands` — the second on a
bare `Node::Boolean` with no union in the picture at all).

When a coincident face pair is DECLARED, the pair verb's emitter is not
symmetric in its two operands:

- the merged face keeps operand A's carrier — the same plane, with a
  different `origin`;
- the rim edges the merge splits are operand A's, so the
  `Fragment(OrderAlong)` rows sit on A's rims and not on B's.

Swapping the two operands therefore changes the result's NAMES and its
descriptions, while the volume, the face/edge/vertex counts and the set
of `Merged` rows are identical.

Under `Node::Union` the fold is left-associative and the routing takes
no A/B freedom (the accumulation is always operand A, the joining member
always operand B), so reordering the member list is what moves the
asymmetry: `a_declared_pair_routes_by_member_id_and_survives_a_reorder`
(`crates/editor-core/tests/docm7_union_declare.rs`) now ASSERTS it —
fragments on `a` under `[a, far, b]`, on `b` under `[b, a, far]`.

## Where it contradicts what is written

`crates/editor-core/src/names/role.rs`, `RoleSeg::FromMember`'s doc said
"a member's names are a function of the member's identity alone —
neither its position nor how many members precede it". That is true of
the WRAPPER and false of the table: which rows exist is the pair verb's
answer, and it depends on the operand seat. DOCM-7 rewrote the paragraph
to say exactly that and to point here.

Two other sites carry the unqualified sentence and are NOT this unit's
to edit:

- `docs/DOCM-7-SPEC.md:107` (D9) — the spec is deleted at merge, so it
  goes with it.
- `docs/DOCM-REFERENCES-DESIGN.md`, DM4's bullet — the ratified design.
  Its sentence is narrower than the one in `role.rs` ("the order is the
  list's, and the list is data"), so it is not falsified outright; what
  it does not say is that the ORDER shows in the names.

## The question

Whether the pair verb's merge should be symmetric — pick the surviving
carrier and the fragmented rims by a canonical rule over the two names
rather than by operand seat, the way `collapse` already canonicalizes a
`Seam`'s two sides. That would make a union's names order-free, which is
what DM4's spirit asks for, and would move `Fragment` rows in every
existing declared-merge document (goldens, corpus).

Or a ruling that a boolean's names are the operands' and the order is
the author's, with DM4 saying so.

## Where it stands

Open, on DOCM's slate, for Ev — it is the pair verb's behaviour, older
than DOCM-7, surfaced by it. Nothing blocks DOCM-7: the asymmetry is
asserted, and `role.rs` states the measured truth.

## Re-homed (2026-09-13)

Moved from `work/docm/` to `work/wire/` at DOCM's exit sweep (`docs/DOC-LEDGER.md`,
sweep 14): the file it names is WIRE's (`names/emit*.rs`, `eval/wire.rs`, `product.rs` are in WIRE's paths). Id, body and header are unchanged; the directory is the
claim (`work/README.md`). Any `## Home` section above is superseded by
this line and is kept as the record of why the file was where it was.

(At DOCM's exit sweep, `refs` names the PRs `DOCM-7` stood for: `DOCM-7` = #2028 — the unit rows left the tracker with `work/docm/`; `docs/DOC-LEDGER.md` sweep 14.)

## Read against the tree (2026-09-15) — live, re-kinded `ruling`, and one citation has rotted

Read by the WIRE orchestrator before dispatch. The asymmetry is still
asserted:
`crates/editor-core/tests/docm7_union_declare.rs`,
`a_declared_pair_routes_by_member_id_and_survives_a_reorder`. `role.rs`
still carries the corrected `FromMember` paragraph.

**Re-kinded from `issue` to `ruling`.** The row's own `Where it stands`
says *"for Ev"*, and the tree agrees it cannot be a lane's call: the
symmetric answer moves `Fragment` rows in every existing declared-merge
document — goldens and corpus — which is a ratified-behaviour change, not
a refactor. Filed as an issue it read as dispatchable work on the board.

**Citation rot, and it sharpens the row rather than weakening it.** The
row sends a reader to `docs/DOCM-REFERENCES-DESIGN.md`, DM4's bullet.
**That file no longer exists**: DOCM's exit replaced it with
`crates/editor-core/REFERENCES.md` (`docs/DOC-LEDGER.md`, present tense,
DM1–DM6 kept). DM4's sentence is there — *"the order is the list's, and
the list is data"* — and `docs/DESIGN.md`'s companion table lists that
page as **Ratified**. So the sentence this row asks about now sits in a
ratified design page, which under CLAUDE.md is Ev's to change: the rot
moved the citation from a deleted spec into the exact document class that
makes this a ruling.

The other site the row excused, `docs/DOCM-7-SPEC.md:107`, is gone with
the spec as predicted.

## NOT settled by the 2026-09-15 ruling — and this row is now the only ruling on WIRE's slate

`does-n3-retire-loudly-generalise-to-the-folds-other-compositions` was
ruled by Ev on PR 2677: a composition that breaks *one name denotes one
entity* **refuses**, and **offers** where a **unique best** offer exists.
That answered the other four compositions this program had open. **It
does not reach this one, and the reason is worth stating so nobody
applies it here by analogy.**

The rule governs a **reference whose entity went away** — its protected
value, in Ev's words, is that N3's rejected alternative was *"not even
refusing, just silently taking the merged descendant"*, i.e. **never
silently re-point a name**.

Nothing here is re-pointed. No name vanishes, nothing resolves to the
wrong entity, and no refusal is owed: two member orders produce **two
valid documents** whose names differ, because the merged face keeps
operand A's carrier and the `Fragment(OrderAlong)` rows sit on A's rims.
The volume, the face/edge/vertex counts and the set of `Merged` rows are
identical either way. There is no reference to refuse and nothing to
offer, so the rule has no purchase.

**The question this row asks is therefore still open and still Ev's**:
should the pair verb's merge be **symmetric** — the surviving carrier and
the fragmented rims picked by a canonical rule over the two names, the
way `collapse` already canonicalizes a `Seam`'s two sides — or is a
boolean's names the operands' and the order the author's, with DM4 saying
so?

What it costs is unchanged and is why it is not a lane's call: the
symmetric answer **moves `Fragment` rows in every existing
declared-merge document**, goldens and corpus. And DM4's sentence now
lives in `crates/editor-core/REFERENCES.md`, which `docs/DESIGN.md`'s
companion table lists as **Ratified** — so the answer amends ratified
text either way, by changing what it decides or by adding what it does
not currently say (that the ORDER shows in the names).
