# EDIT-DECL — a declaration is sited at the members (spec)

**Unit:** `a-declared-union-has-no-one-pass-authoring-path` (kernel
unit, v6 dual, block EDIT-B2 slot 0). **Ruled by Ev** on `[ev]` PR
#2795 (2026-09-17): the row's `## RULED` section and the amended DM4
clause in `crates/editor-core/REFERENCES.md` are the ruling; this file
restates them as premises. Deleted at merge and recorded in
`docs/DOC-LEDGER.md`, as the EDIT-PICK specs were.

Branch `edit/sited-declarations`. Read first: the row and its
`## RULED`; DM4, DM6 and DM7 in `crates/editor-core/REFERENCES.md`
(present tense — DM4 already describes the sited shape, the tree does
not yet build it); `crates/editor-core/src/node.rs` (`Node::Declare`,
`Node::Boolean`/`Node::Union`'s `declare` field docs, `SitedRef`,
`payload_names`, `payload_read_sites`, `rebind_payload_names`,
`declare_rest`); `crates/editor-core/src/eval/wire.rs` (`wire_boolean`,
`wire_union`, `declared_pairs`, `DeclSite`, `declared_bucket`,
`decl_site`, `route_declarations`, `look_through_merges`,
`declare_landing`, `resolve_declarations`); `eval/mod.rs`'s four
`Declare*` arms and `UnionDeclareStep`; `edit.rs`'s
`DeclareNamesMissingNode` / `ReadSiteMissingNode` door;
`persist/pairs.rs` and `persist/kernel_wire.rs::contact_class` (the
persisted pair); the suites `docm7_union_declare.rs`,
`docm8_flat_merged.rs`, `dm7_delete_strands.rs`, `rv_dm7_probes.rs`,
`m5_s1_rest_declare.rs`; the corpus documents that declare
(`tests/corpus/{kiss_carry,slots,part_select}.rs`, `fixture/pr4.rs`);
`crates/pncad/src/select.rs` (the flush → `Declare` door) and
`crates/pncad-py/src/py/doc.rs`'s `boolean`/`union`/`declare`
constructors; `docs/prompts/implementer-discipline.md`.

## The ruling, as premises

1. **A declared pair names two SITED entities.** `Node::Declare`'s
   payload becomes `Vec<((SitedRef, SitedRef), ContactClass)>`:
   `SitedRef { at, name }` with `at` the node the entity is read at
   and `name` the entity's `StableName` in `at`'s own table. For a
   pair boolean `at` is one of its operands; for a union `at` is a
   member. A declaration therefore names only what exists BEFORE the
   consumer, and the fixture's five-edit path (a first union, the
   `Declare` in its space, a second union, a `Rebind` loop, a delete)
   retires: `Declare` then consumer, two edits, one pass. `SitedRef`
   is the mate head's shape already; reuse it, do not mint a twin.
2. **The site is the side.** `wire_boolean` resolves each name in the
   ONE table its site names (`at == a` → `a_table`, `at == b` →
   `b_table`); a site that is neither operand refuses typed
   (`NodeErrorKind::DeclareSiteNotAnOperand { at }` or the spelling
   the crate's convention gives — one new arm, `tags.rs`'d). The
   side-picking in `declare_landing` goes with `DeclareBothOperands`,
   which retires: two placements of one prototype are declarable
   through the PAIR boolean now (`the_pair_boolean_cannot_declare_between_two_placements_of_one_prototype`
   inverts into "…can, sited"). Kind-before-multiplicity, the tie's
   place in the ladder and `DeclareUnsupportedPair` (with its
   `cross_operand`, now read off the two sites) are unchanged.
3. **The union derives the step from the two sites.** `route_declarations`
   reads `DeclSite::Member(i)` from `at`'s position in `members`
   instead of from a `FromMember` segment; a site not in the list
   refuses through the N5 ladder as a vanished name does
   (`SetMembers` leaves `declare` as it was — DM4's sentence). Each
   bucket's pair is rewritten into the node's member space at the door
   (`member_view`'s one segment: `FromMember { member: at, of: name }`)
   so `look_through_merges` and the shared resolver run exactly as
   they do today: a member face the fold has merged away resolves to
   the flat `Merged` row that holds it, in every member order
   (DOCM-8's rows keep their meaning; re-baseline their fixtures'
   authoring, not their claims). What the union PUBLISHES is unchanged
   — `FromMember { member, of }` is what it mints; the sited pair is
   what it consumes.
4. **Fold-minted rows are not declaration subjects.** `DeclSite::
   Accumulated` and `Unkeyed` — a `Seam`/`Merged`/`Fragment`/`OutputBody`
   row of the union's own space paired with a member — were reachable
   only through the five-edit path this ruling retires; under a sited
   payload the union's own space has no site that is live before it,
   so the class is unrepresentable BY TYPE rather than refused at
   evaluation. Delete the two variants, `declared_bucket`'s
   member-versus-fold asymmetry and `latest_member`; `UnionDeclareStep`
   keeps only the arm a sited pair can still reach (two sites whose
   later member is member 0 has no step — say whether that is even
   possible once the bucket is `max(i, j) − 1`, and delete the arm if
   not). The DOCM-7 rows that pinned the fold-row cases
   (`an_accumulation_entity_paired_with_an_earlier_member_refuses`,
   `the_unions_own_body_row_is_refused_as_unroutable_not_as_vanished`,
   `a_fold_row_routed_before_the_step_that_mints_it_has_no_step`,
   `two_fold_rows_are_carried_at_the_first_step_that_has_both`) retire
   WITH the class, each replaced by the row that says the shape cannot
   be written (a `compile_fail` doctest or a serde-refusal row on the
   persisted form, whichever the case admits) — never silently. A
   merged face stays reachable through premise 3's look-through, which
   is the case a user has.
5. **The doors follow the payload.** `payload_names` yields each pair's
   two `name`s (the insert door's live-node check, DM7's strand report
   and `Rebind` keep working on them); `payload_read_sites` yields each
   pair's two `at`s (the insert door's `ReadSiteMissingNode` check, as
   for a mate; a deleted site is N5's dangling case refused at the
   next evaluation, never at the delete — DM7's sentence). The load
   door asks nothing new: a `Declare` can exist unconsumed, so a site
   that is not the consumer's operand is the evaluation's refusal
   (premise 2), and `Node::bad_declare_input` is unchanged.
6. **The persisted form moves, and the corpus with it.** The pair codec
   (`persist/pairs.rs`, `kernel_wire::contact_class::pairs`) persists
   sited pairs; the format is not versioned in this tree and no
   migration is written — every declaring document in the tree is
   re-authored (the five corpus/fixture documents, all pair booleans,
   where `at` is the operand each name already resolves in; the DOCM-7
   and DOCM-8 suites' unions). Say in the PR body which persisted files
   changed and that none outside the tree exist to migrate. The
   `deny_unknown_fields` census and `tags.rs` move mechanically.
7. **The façades follow mechanically** (LIB's, by announcement).
   `pncad::select::declare_node`/`declare_all` mint sited pairs from
   INSPECTED findings: a finding's names are the consumer's published
   rows, so the door inverts the member view (`FromMember { member, of
   }` → `SitedRef { at: member, name: of }`) or, for a pair boolean,
   sites each name at the operand it landed in; a finding whose names
   cannot be sited refuses typed through `DeclareError`. The Python
   `declare`/`declare_all` sugar and `Node.declare(findings)` follow;
   `Node.boolean`/`Node.union` are unchanged. `demos/tour` declares
   nothing today (`diefillet.rs` passes `declare: None`) — check with
   `cargo check --manifest-path demos/tour/Cargo.toml` anyway.
8. **DM6 is untouched.** No edit rewires a live node; no `SetDeclare`.
   The `declare` edge is authored at `InsertNode` of the consumer, as
   today.

## Rows

Red first, then the door:

- **One pass.** A union of two flush placements of one prototype,
  authored as `Declare` (sited at `m1`, `m2`) then `Union`, in two
  edits, evaluates to the fused volume — the row
  `a_union_of_two_flush_placements_of_one_prototype_fuses_when_declared`
  re-authored, its `declared_union` helper losing the second union and
  the rebind loop. Watch it fail to COMPILE on the old payload first;
  that is the red.
- **The pair boolean declares between two placements of one prototype**
  (premise 2), inverting the DOCM-7 row that said it cannot.
- **The site is the side**: a pair sited `(a, a)` is operand A's
  carried contact; `(a, b)` is a cross-operand face pair; a site that
  is neither operand refuses with the new arm; a union pair whose site
  left the member list refuses through the ladder.
- **Routing by site survives a reorder** (`a_declared_pair_routes_by_member_id_and_survives_a_reorder`,
  re-authored); a same-member pair is a carried record at its step; a
  merged-away member face resolves through the look-through in every
  member order (DOCM-8's `docm8_flat_merged` rows, re-authored).
- **The doors**: insert refuses a `Declare` whose name's node or whose
  site does not exist (`DeclareNamesMissingNode`, `ReadSiteMissingNode`);
  DM7 reports a strand per name and none per site
  (`deleting_a_declared_union_names_every_pair_of_its_declare`,
  `every_payload_kind_that_carries_a_name_reports_its_strand`
  re-baselined); `Rebind` rewrites a pair's name and leaves its site.
- **Persistence**: a declared union round-trips (snapshot and edit
  log) and replays bit-identically (`a_declared_union_replays_bit_identically`,
  `m5_s1_rest_declare`'s round trip); `a_declared_unions_document_loads_but_does_not_replay_in_order`
  — say what it pinned and whether the class survives one-pass
  authoring (it should not; retire it with the sentence).
- **The unrepresentable class** (premise 4), pinned as "cannot be
  written".
- **The façade**: `declare_all` on a union's undeclared-contact
  findings yields sited pairs that make the union evaluate; the Python
  north-star/guide tests that declare are re-run.

## Mutants (each named with the rows it reds)

Resolve a name in both tables regardless of its site (the site-is-the-
side rows red); route a union pair by the name's node instead of its
site (the reorder row reds); drop the member-space rewrite before the
resolver (the look-through rows red); `payload_read_sites` without the
sites (the insert-door row reds); `payload_names` yielding the sites
too (DM7's strand count reds).

## Territory

`crates/editor-core/src/{node.rs, edit.rs, eval/wire.rs, eval/mod.rs,
persist/*}` (EDIT); `crates/editor-core/tests/*` (TCOST/TINT, the
suites named above); `crates/editor-core/REFERENCES.md` only if a
sentence the build moved needs re-wording (DM4 is already the ruled
text — do not re-litigate it); `crates/pncad/src/select.rs`,
`crates/pncad-py/src/{py/doc.rs, tags.rs, tests.rs}`, the Python tests
(LIB's, mechanical, announced here); `demos/tour` if it compiles
against a moved type. List every path `python3 scripts/work.py
territory` reports for your diff.

## Verification

Hosted CI green on the pushed head (read step conclusions). Locally:
`cargo test -p editor-core --test all` filtered to the suites above,
`cargo check -p pncad-py --features python`, `cargo test -p pncad-py
--lib`, the python suite through the wheel if the box builds it, the
`deny_unknown_fields` census, `scripts/doc-gate.sh --skip-viewer-toolkit`
for the moved rustdoc. The PR body carries: every deviation from these
premises stated and argued or scheduled; the list of persisted files
re-authored; the retired rows and what replaced each; the mutant
table; the sweep for `Node::Declare { pairs }` readers and what it
could not match; the CI run id.

## Amended at the fix pass (2026-09-17)

The unit's v6 dual review converged on one MAJOR, and the orchestrator
ruled it. Two premises move; everything else above stands as written.

**Premise 4 keeps its decision and gains the refusal shape it was
missing.** Fold-minted rows are still not declaration subjects — the
class is unrepresentable by type, and `Node::Declare`'s
`compile_fail` + running-twin pair is where that is now pinned. What
the premise did not say is what a union's REFUSAL does when the
contact a user actually has is against such a row, and the answer was:
`union_refusal` pushed every finding through `sited_member`, which
answered `None` for a `Merged`/`Seam`/`Fragment`/`OutputBody`-tailed
row, and the whole refusal degraded to `NamingError::Emission` — the
crate blaming itself for the user's document. Both reviewers built it
independently. The shape is:

- **A `Merged` row's contact is refused `UndeclaredContact` with that
  side sited at a CONSTITUENT.** The merged row's flat constituent set
  (N3) holds member faces; the refusal takes the one whose member comes
  first in the union's member order (D9 — deterministic), sites the
  finding there with that member's own face name, and carries the whole
  set beside it (`NodeErrorKind::UndeclaredContact`'s `merged` field,
  rendered in the refusal's prose). Any constituent declares the same
  contact, because a declaration resolves to the merged row through
  `look_through_merges`, so the pick is immaterial and the caller can
  declare the finding verbatim.
- **A row no member's entity stands for** — a fragment, the union's own
  body — is refused `NodeErrorKind::UndeclarableContact { row, diag }`,
  which says the contact is with a row the fold minted and that a sited
  declaration cannot name it. `tags.rs`'d as `undeclarable_contact`.
- **`sited_member` is TOTAL in the type**: it answers
  `Member` / `Merged` / `FoldMinted` instead of `Option`, so the caller
  must handle the fold's own rows rather than collapsing them into an
  invariant break. `union_refusal`'s doc and `refusal_menu`'s "never
  masked" sentence are true again.

**Premise 7's "no case" is withdrawn.** It said the detect→declare
protocol was total by construction and that a finding whose names
cannot be sited has no case. It has one: the fold-minted row above,
which the typed arm answers for. What remains true is the part the PR
argued — the SITE travels with the finding, so no downstream door has
to recover it.

**What the ruling could not be built as stated.** The typed arm was to
get one row per fold-row kind, "`Seam` at least". A contact refusal
resolves a FACE key pair through the operand tables
(`wire.rs`'s `face_name`), and `RoleSeg::Seam` mints edges (face ×
face) and vertices (edge × face) but never a face, while
`RoleSeg::OutputBody` names the body. So the only fold-row kind a
contact refusal can name is `Fragment`, which
`docm8_flat_merged::a_contact_against_a_fold_minted_fragment_is_undeclarable`
pins; the row's doc states the reach and why the other two are out of
it.
