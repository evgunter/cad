---
id: a-union-member-is-keyed-by-its-read
kind: issue
title: A union keys a member's names by the operation it reads, so two members read out of one operation (a split's two halves) cannot be named apart
status: open
opened: 2026-10-09
priority: P0
cost: M
refs: [part-split-half-retires, operands-are-reads, a-name-is-scoped-by-the-variable-holding-its-body]
---

**The finding.** DM4 keyed a union member's names by the operation it
reads (`FromMember { member: RecipeNodeId, of }`), while unit B
(`operands-are-reads`, PR 4342) made DM5 distinctness over the variables
read. Two members read out of one operation, `Union[split.above,
split.below]`, carried one key, so the union refused them typed
(`NodeErrorKind::MembersShareAnOperation`, `eval/wire.rs`, `wire_union`),
and the way to rejoin two halves was the pair `Boolean`, keyed by seat.
The union key is one of four places where an operation stands in for a
read: the union's name key, `SitedRef.at`, `coincide::construction`'s
entry (`operation_of` lowers each read to its node, so the walk cannot
tell a split's halves apart either) and the evaluator's `Results`.

**What is decided (FORK-DM4).** REFERENCES DM3, DM4 and DM5, NAMES N1 and
N2, and D10 Repetition state it:

- **The key is the read.** An operation that carries an entity in from an
  input names it `From { read: VarId, of }`, keyed by the variable the slot
  holds. It replaces `FromA`, `FromB`, `FromMember` and `FromTarget`.
- **Three nodes.** `Union { members }` and `Intersect { members }`, two
  node kinds, each over two or more reads, sharing DM4's one-pass pairwise
  judgement, the fold in list order and `SetMembers`; a glue keeps the
  earlier listed member's description. `Subtract { from, tool }` is the one
  pair; several tools are `subtract(a, union[tools])`. `Boolean { op, a, b }`
  and `BooleanOp` leave the document.
- **A name is scoped by the variable that holds its body** (N1, the
  scope). Built by the second unit.
- **One `Bodies` argument.** Union and intersect take one argument of kind
  `Bodies`: a read of a family (`Union(xs)`) or the member reads spelled at
  the slot (`Union([a, b, c])`): a `Bodies` defined by index (a family) or
  by enumeration (independent reads, each named by its own read), which
  every `Bodies` reader takes. Naming an enumeration as a definition
  several readers share is not built here (no immediate need; nothing
  forbids it, and it is a small later change);
  a mix is ill-typed (`SlotVarKind`). A union or intersect of one body
  builds that body (no boolean runs; names are `From { read, of }` as in
  any union), and of none builds the typed empty body, so a family whose
  `N` is 1 or 0 builds instead of refusing.
- **A family member.** `xs[i]` is a read of the family `xs` at one `Count`
  expression per index, not a variable; it keys by `xs`, the index said
  once in `Member`; `Union(xs)` and `Union([xs[0], …])` name alike.
- **A repeated read keeps both and glues** (DM5). `[A, A]` glues every cell
  to its twin structurally; `A ∪ A` and `A ∩ A` are `A`, `A − A` the typed
  empty body; a glued row is `Merged` of one name, which is that name (N3).
  DM5's door check and `DuplicateInput` retire.

**Unit 1 (this item): the read key and the three nodes.**

- `RoleSeg`: `From { read: VarId, of }` replaces `FromA`, `FromB`,
  `FromMember`, `FromTarget`; the rewriters, `lift`, the words and N6's
  walk read one segment kind.
- `Node`: `Union`, `Intersect { members, declare }`, `Subtract { from,
  tool, declare }`; `Boolean` and `BooleanOp` retire. `wire_union` takes
  the verb (union or intersect), its duplicate-operation loop and
  `MembersShareAnOperation` go with its class, Python tag and pins
  (`intent_s2_b_reads::dm5_is_over_the_variables_read` now builds, volume
  1.0; `a_pair_declared_across_one_splits_halves_is_sided_by_table`
  renamed and rewritten over `Union`); `wire_boolean` becomes subtract
  only. An empty intersect fold step is the typed empty result; for a
  union it stays a kernel bug (`UNION_STEP_EMPTY`).
- `emit_union::{member_view, member_name, keyed}` and `name_union`'s
  `member_of` key by the read and restrict a member's view to its port's
  rows; `emit_topo`'s boolean arm names subtract's seats by `From`, a seam
  vertex citing the least input edge in name order (N2).
- `SitedRef.at` becomes a `VarId` (until stage 4 F retires declarations);
  `member_site` / `site_operand` lose the "sided by table" inference.
- The argument: `Union`/`Intersect` hold one `Bodies` operand, a family
  read or a spelled list of reads (the shape a placement's shape list
  takes); a mix refuses `SlotVarKind` at the insert, `SetMembers` and load
  doors; the floor of two goes (`EditError::TooFewMembers` retires): a
  union or intersect of one body builds that body with no boolean run and
  `From { read, of }` names, and of none builds the typed empty body.
- The indexed read: a slot reading one member holds the family's
  `VarId` and one `Count` variable per index (VR4), carried through
  `Operand`, lowering, persistence, `upstream` and the keys; DM3's member
  definition goes.
- DM5: the door check, `EditError::DuplicateInput` and the validator's
  arm retire. Pinned row: a member against itself (`Union([A, A])`, and
  `Intersect`, `Subtract { from: A, tool: A }`) reaches the coincident-shell
  lane with every cell structural, builds `A` (resp. the typed empty
  body), and names each row as `A`'s; a cell left unglued surfaces as the
  emitter's `DuplicateName` refusal, never a silent table. A declared pair
  (until stage 4 F) sited at a read spelled twice sites both members,
  under DM4's same-member-pair rule.
- `coincide::construction` and `NamedCell::Entity` take the read, not the
  operation (needed as soon as two halves build).
- Persist: one load-time migration. `Boolean { Union, a, b }` →
  `Union [a, b]`, `Intersect` likewise, `Subtract` to its seats; a chain
  stays a chain (flattening is the author's edit, DM6). Names map
  `FromA`/`FromB`/`FromMember`/`FromTarget` → `From` by one total map.
  Stored documents: `plate_param.pncad`, `die_composed_tour.pncad`,
  `pre_b_families.json`.
- Python: `Node.boolean` and `BooleanOp` retire; `Node.union(members)`
  stays; `Node.intersect(members)` and `Node.subtract` arrive (two seats;
  `from` is a Python keyword, so the stub names that parameter);
  the stub and `ty` follow.
- Viewer: the two-pick Boolean tool becomes union and intersect with N
  picks and subtract with two.
- What moves: every name digest over a document holding a union or a
  boolean (the corpus digests, `lib_g16`, `perf2`, the `m4_pr3` die digests,
  `m4_pr4`'s `DIAGNOSIS_DIGEST`, `name_words_corpus`'s `SAID_DIGEST`,
  `asm2b`, `seat4`); the variant feeds the node preimage, so node ids
  re-mint. Geometry does not move. No bits-keeping special case (a bare
  node id when the operation has one output): two spellings of one key.
- Review check: a three-member intersect with two coincident faces, since
  the kernel's intersect lane under a fold is not traced.

**Unit 2:** `a-name-is-scoped-by-the-variable-holding-its-body`, per-variable
tables, `Results` keyed by read and the split's roles dropping the half.
