# AUTH-9 — a union that declares its contact

**Row**: `work/author/addboolean-doc-names-a-vocabulary-that-does-not-exist`
(P0, D). Read it in full, including the 2026-09-21 section "What the gap
COSTS, measured". Also read EDIT's closed
`work/edit/a-declared-union-has-no-one-pass-authoring-path`, whose
closure section is Ev's ruling in code (PR #2809).

**Branch** `author/declared-union`. **Never merge; I merge.**

## Why this unit is dispatchable now

The plan parked this row: it "waits on EDIT's `DocEdit` vocabulary". EDIT
closed that dependency on 2026-09-19 (#2809, ruled by Ev):

> a declaration is authored in one pass: the five-edit workaround is
> gone.

The row's literal claim is still true: no `DocEdit` attaches a
declaration to a live node after the fact. But nothing has to. A
declared union is authored in one pass: insert a `Node::Declare { pairs:
[((SitedRef, SitedRef), ContactClass)] }`, then insert the `Boolean` that
names it. The viewer's `SessionOp::AddBoolean` still authors `declare:
None`, and its doc still says a declaration "is added afterwards through
the vocabulary that owns it". No such vocabulary exists, and none is
needed any more.

## What a person gets

AUTH-1 measured the blocked gesture. Draw a boss on a picked face and
union it with the block. The union refuses
`UndeclaredContact { finding: FlushFinding { pair: (block's Cap(End),
boss's Cap(Start)), class: Rest, … } }`, because a boss drawn on a face
frame is flush **by construction**. That is the most ordinary way to add
material to a part, and today it's a dead end.

After this unit, the refusal offers to declare the contact. Accepting
it commits the `Declare` and the union as **one action, one undo**
(`commit_run`, the AUTH-3/AUTH-4 shape).

## Check these first, because I'm not sure of them

- **How the finding reaches the viewer.** AUTH-1 saw `UndeclaredContact
  { finding: FlushFinding { pair, class, … } }`. #2809's closure says
  "the FLUSH FINDING carries its sites". I couldn't find `pub struct
  FlushFinding` by grep, and `assembly.rs` has a *different*
  `ValidationError::UndeclaredContact { contact: CensusContact, witness }`.
  Find the one the union door returns, through `pncad`, and say which
  it is. **If the refusal doesn't carry enough to author the `Declare`
  honestly (both sites and the class), say so and stop.** Don't
  reconstruct sites from elsewhere.
- **Several contacts.** Does one refusal report every undeclared contact,
  or only the first? If only the first, a union with two flush faces
  needs declare, retry, declare. Decide how the gesture handles that and
  say why.
- **Does the class come from the finding, or does the author choose it?**
  `Node::Declare`'s doc says a class-less pair is unrepresentable,
  because defaulting "would let a `Tangent` intent be verified against
  the conformal table". The finding's class is what the kernel detected.
  My reading is that the author confirms it rather than picks it, but
  check that against CONTACT-DESIGN and say.

## Design calls: decide and say

1. **Where the offer lives.** On the refusal sentence, as a "declare
   this contact" affordance next to the words (the tool stays open), or
   as a declare option on the union tool upfront. The refusal-first
   shape matches the kernel's detect→declare protocol.
2. **What the author sees before accepting.** The contact is between two
   named faces, and they have to know which ones. Use the chrome's one
   spelling of a node and face (`tree::node_number`, `BlendTarget`'s
   `Display`); don't mint a new one. `face-pick-cannot-name-which-face`
   is an open AUTHOR row, so if a face can't be named honestly, say what
   the offer shows instead.
3. **The `AddBoolean` doc**, which the row is about. Make it true.

## Traps

- Eight of eight AUTHOR units have minted a fresh duplication while
  closing one. The usual culprits are a copied kernel sentence or a
  second spelling of a number or name. Check your own diff.
- A composition no test holds is this program's recurring defect.
  Drive the real tool panel (`crate::pane::headless`) through
  refusal → offer → accept → one undo.
- **Before merging, I run editor-core's `node_standing` census myself.**
  A viewer-only PR doesn't build it (2026-09-28 latency cut), and it
  counts viewer `Evaluation` reads. If you read results, run it locally:
  `cargo test -p editor-core --test all node_standing::`.

## Scope, verification, deliverable

In: `session/op.rs`, `session.rs`, `combine.rs`, `pane/create.rs`, plus
the tool/seat files the union tool uses. Out: `crates/editor-core`. The
kernel door exists; if it's missing something, file it on EDIT.

- Rows that go red: the flush boss-on-face union refuses; accepting the
  offer lands one `Declare` and one union as one undo, and the union
  evaluates; declining leaves the document unchanged; a non-flush union
  still lands with no declaration. Name the mutations, restore them from
  a byte copy, and **touch** afterwards.
- Local: fmt; clippy (both feature sets, `-D warnings`); `doc-gate.sh`;
  every `scripts/gates/*.sh`; `--lib` and `--test all` as separate runs;
  `work.py lint`; `work.py territory` output in the PR.
- `CARGO_TARGET_DIR=/root/auth-9-target` on every invocation, including
  excluded roots. Scratch in `/root/auth-9-scratch/`. Wrap `cargo` in
  `local-scripts/with-build-slot.sh -- <cmd>`.
- CI: confirm the head SHA, read `gate ok`, and say what the change
  filter selected.

PR titled `AUTH-9: a union that declares its contact`. Report to me;
don't merge.
