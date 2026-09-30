---
id: dead-work-citations-from-shipped-code-and-docs
kind: issue
title: Six shipped files cite work/ items at paths that no longer resolve
status: open
opened: 2026-09-13
priority: P4
cost: E
---


## Finding

Shipped source and design pages cite `work/` items by path. A program is
closed by deleting its tracker directory (`work/README.md`; the walk
stays recoverable at the SHA `docs/DOC-LEDGER.md` names), so every such
citation becomes a dead path the day its owning program closes — and
nothing in the tree checks them. A reader who follows one finds nothing
and cannot tell "this reading was superseded" from "this reading is
still live, somewhere".

Found while sweeping for stale citations in BLEND-12's fix pass (R2
NOTE-3). Five instances were found then, each a path that does not
resolve at this SHA; a sixth was added on 2026-09-15 and is in its own
section below, with a correction to how the fifth row reads.

| citing file | cited path | still there? |
| --- | --- | --- |
| `crates/profile/README.md` (the fillet section's "what stays") | `work/issues/nocornersidecandidate-has-no-producer.md` | no |
| `crates/editor-core/src/clearance.rs` (the `Interval` basis note) | `work/issues/interval-orthonormal-basis-sign-hull.md` | no |
| `crates/pncad-py/src/prose_census.rs` (the step-import row's reason) | `work/issues/debug-in-prose-at-blend-and-step-import.md` | no |
| `crates/topo/src/offset_nappe.rs` (module docs) | `work/issues/cone-nappe-is-decided-in-five-places.md` | no |
| `crates/viewer/src/props.rs` (module docs) | `work/issues/doc-param-unit-edit-has-no-door.md` | no |

The `profile` one is repaired in PR 2508 (BLEND-12), which is where the
class was found — that file was in the unit's fence. The other four are
listed here rather than fixed, because each belongs to a different
crate's owner and none of them is a BLEND path.

**`work/<program>/…` citations are the same class with a shorter fuse**:
they die the moment that program closes rather than when an issue is
resolved, and the tree holds several (`work/curved/…` and `work/fix/…`
citations in `clearance.rs` and `prose_census.rs` resolve today and will
not once those programs close).

## What would actually fix it

Two shapes, and the choice is the point of this item:

1. **A gate.** A row that walks `crates/**/*.rs` and `crates/*/README.md`
   for `work/…\.md` citations and asserts each resolves. Cheap, and it
   inverts the failure: a program that closes discovers, on its own PR,
   which shipped prose was leaning on it. The cost is that closing a
   program then obliges editing other crates' files, which is exactly
   the coupling the tracker's directory-is-the-claim rule avoids.
2. **A convention.** Shipped prose cites the READING, not the path — it
   states the finding in a sentence and, where provenance matters, names
   the SHA rather than a file that moves. `docs/DOC-LEDGER.md` is
   already the done-state of record for exactly this reason.

(2) is the smaller change and the one the ledger's existence argues for;
(1) is the only one that catches the instances already in the tree. They
compose: adopt (2), and gate (1) against new citations only.

No obvious single owner — the five instances span four crates — so this
sits in `work/issues/` per `work/README.md`'s last-resort rule.

## A sixth instance, and a correction to the fifth (2026-09-15, CHROME's `chrome/citation-repoint`)

**Sixth instance.** `crates/viewer/src/pane/properties.rs` — the
comment on the parameter row's unit LABEL, explaining why there is no
picker beside it — cites
`work/issues/doc-param-unit-edit-has-no-door.md`. Same dead path as the
`props.rs` row in the table above, same cited item, a different file.
The two were presumably split apart by
`viewer-session-god-module-split` (#1830) with the citation copied
along.

**Correction to the fifth row.** The table reads *"still there? no"*
for `work/issues/doc-param-unit-edit-has-no-door.md`, which is true of
the PATH and misleading about the item: it was **claimed by EDIT**, not
resolved and not deleted. It is open at
`work/edit/doc-param-unit-edit-has-no-door.md`. That makes these two
instances a different sub-case from the other four, and a cheaper one:
the other four cite items that died with their programs and need shape
(2)'s rewrite-the-reading treatment, while these two need a one-word
path edit — or nothing, if shape (2) is adopted and the sentence is
rewritten to state the reading anyway.

It also sharpens the argument for a gate: this class has a **second**
fuse nobody has named, the `work/README.md` rule that *claiming an
issue MOVES the file*. A citation can rot without any program closing
and without the finding going anywhere, just because its owner was
identified.

Both tracker-side citations of the same item — in
`work/chrome/parameter-row-field-has-no-text-door.md` and
`work/chrome/add-parameter-form-authors-canonical-only.md` — were
re-pointed to `work/edit/…` by that pass. The two source files are
outside its fence and are recorded here instead.

## The `work/docm/…` family, now dead (2026-09-16, EDIT's `edit/error-prose`)

The second fuse this row names — a program closing — has fired.
DOCM closed on 2026-09-13 (`docs/DOC-LEDGER.md`, sweep 14) and its
directory was deleted, so every shipped `work/docm/…` citation is now a
path that does not resolve. `rg -n 'work/docm/' crates/` at this SHA:

| citing file | cited item |
| --- | --- |
| `crates/pncad-py/src/prose_census.rs` (two roster reasons) | `debug-in-prose-residue-after-finding-sink` |
| `crates/viewer/tests/index_memo.rs` | `pick-grazing-ray-answer-depends-on-candidate-order` |
| `crates/editor-core/tests/docm7_union_declare.rs` | `the-pair-verbs-declared-merge-is-asymmetric-in-its-operands` |
| `crates/editor-core/tests/wire_operand_door.rs` | `the-third-datum-axis-phrase-lives-in-mate-member` |
| `crates/editor-core/ASSEMBLY.md` | `pair-doors-outside-the-three-do-not-check-document-identity` |
| `crates/editor-core/src/program.rs` | `a-document-vocabulary-declared-outside-the-macro-is-uncensused` |
| `crates/editor-core/src/eval/mod.rs` | `pair-doors-outside-the-three-do-not-check-document-identity` |
| `crates/editor-core/src/eval/wire.rs` | `member-space-look-through-stops-at-splits-containment-and-fragmented-merges` |
| `crates/editor-core/src/names/role.rs` | `the-pair-verbs-declared-merge-is-asymmetric-in-its-operands` |

Ten citations in nine files. Every one is the CHEAP sub-case the
2026-09-15 correction identified: the items were claimed, not resolved,
and all seven distinct items live today under `work/edit/`,
`work/wire/`, `work/door/` or `work/census/`. Nothing was deleted; only
the directory moved.

A repair is still not mechanical on the program name. DOCM's rows went
to four different successors — `the-third-datum-axis-phrase-lives-in-mate-member`
is DOOR's, not EDIT's or WIRE's — so each citation has to be resolved
against the tracker rather than rewritten by pattern.

The two in `prose_census.rs` are repaired in `edit/error-prose`, which
was editing that file anyway. The other eight span three programs'
fences and are left here.

This is a sharper argument for the gate (shape 1) than the row had
before: ten dead citations appeared at one commit, on files the closing
program could not edit, and nothing in CI said so. The gate needed to
catch them is the weak one — assert that a cited path resolves — because
all nine paths are simply absent.

### Repaired on EDIT's own paths (2026-09-16, `edit/pair-apply-names`)

Four of the ten, each a path edit to the item's live directory, done
in the PR that was editing three of the four files anyway:

| citing file | cited item | now |
| --- | --- | --- |
| `crates/editor-core/src/node.rs` | `member-space-look-through-stops-at-splits-containment-and-fragmented-merges` | `work/wire/` |
| `crates/editor-core/src/names/role.rs` | `the-pair-verbs-declared-merge-is-asymmetric-in-its-operands` | `work/wire/` |
| `crates/editor-core/src/program.rs` | `a-document-vocabulary-declared-outside-the-macro-is-uncensused` | `work/census/` |
| `crates/editor-core/src/eval/mod.rs` | `pair-doors-outside-the-three-do-not-check-document-identity` | citation removed; the field points at `ASSEMBLY.md`'s A2a, which is the one place the list of pairing doors is written |

`crates/editor-core/ASSEMBLY.md` is repaired the other way, and it is
this row's shape (2): A2a now names the tracker rows by **id**, with no
directory, so the claim that moves a row cannot rot the design page.

**A tenth citation the 2026-09-16 census missed**: `node.rs` is not in
the table above it. `rg 'work/docm/' crates/` finds it, so the miss was
in the transcription rather than the search — one more reason the gate
(shape 1) is the only version of this that stays true.

What is left after this pass, and why it is left: `eval/wire.rs` is
WIRE's, `index_memo.rs` is the viewer's, and `docm7_union_declare.rs`
and `wire_operand_door.rs` are the test programs' — four citations in
four files, none of them on EDIT's paths.

## The `work/meter/…` family, and the first sweep of `crates/` outside `docm` (2026-09-16, INSTR unit 0)

**A third family has fired.** METER closed on 2026-09-08 and its
directory left the tracker at sweep 10, so `work/meter/…` joins
`work/docm/…` above. `grep -rn "work/meter/" tools/` finds **thirteen
live citations across five files** — six in `tess-lint`, six in
`k-lint`, one in `tess-meter`, counting `tools/README.md` — with the
file-by-file breakdown, the non-uniform repair and the scheduling on
`work/instr/tools-doc-prose-cites-thirteen-dead-work-meter-paths`,
which is INSTR's ground and where that half is owned. Two points from
it belong to this row rather than to that one:

1. **The gate as this row proposes it would have caught none of the
   thirteen.** Shape (1) is scoped to `crates/**/*.rs` and
   `crates/*/README.md`. Every one of the thirteen is under `tools/`,
   and `tools` is `exclude`d from the workspace `Cargo.toml`, so a
   gate written as a workspace test does not see them even if the glob
   is widened. `scripts/doc-gate.sh --print-roots` derives the real
   root list.
2. **One of the thirteen sits inside an assertion string**
   (`tools/k-lint/tests/predicate_roster.rs`), so a failing test hands
   a dead path to the reader at the moment they are least able to
   check it. That is an argument for shape (2) independent of the
   gate: prose that states the reading cannot do this.

**And the wider `crates/` sweep this row implies had never been run.**
`grep -rno "work/[a-z0-9-]\+/" crates/*/src crates/*/tests`, with each
prefix tested against the live tree, finds — beyond the `work/docm/`
family already listed — **five citations in four files, in three
further dead families**:

| citing file | cited path | where the row is now |
| --- | --- | --- |
| `crates/profile/src/path.rs` | `work/seat/two-d-director-doors-skip-the-finiteness-question` | `work/fix/` |
| `crates/topo/src/boolean/rest.rs` | `work/seat/flush-pair-relation-has-no-caller.md` | `work/bool/` |
| `crates/topo/src/validate.rs` | `work/verbs/verbs-1031b-assigner-checker-divergence.md` | `work/curved/` |
| `crates/sweep/src/blend/mod.rs` (two citations) | `work/code-quality/corner-config-tag-all-concave-trihedron.md` | nowhere |

Four of the five are the CHEAP sub-case the 2026-09-15 correction
named — claimed, not resolved, and each landing in a DIFFERENT
successor, so again not rewritable by pattern. The fifth
(`corner-config-tag-all-concave-trihedron`, cited twice from one file)
resolves nowhere and needs the reading stated or a SHA named.

**One near-miss, recorded so a later sweeper does not re-file it.**
`crates/editor-core/src/mate/member.rs` names
`work/seat/direction-normalization-two-doors-one-home` but cites it AS
`docs/DOC-LEDGER.md`'s entry for it, not as a live path. That is shape
(2) already done correctly, and it is the only instance in the tree of
the convention this row recommends — worth reading before writing the
convention down.

**What this sweep could not match**: a citation naming a row by id
with no `work/<program>/` prefix; a paraphrase; a prefix for a program
that has not closed yet (unlit fuse, and the tree holds many); and
anything outside `crates/*/src`, `crates/*/tests` and `tools/` — in
particular `crates/*/README.md` and `crates/*/ASSEMBLY.md`, which the
`work/docm/` table above reached but this pass did not.

### Four repaired at DOOR's exit sweep (2026-09-21, sweep 19)

DOOR left the tracker on 2026-09-21 and its sweep repaired every
`work/`-path citation in shipped code that its directory was about to
break, rather than leaving them as this row's tenth through thirteenth.
Three were **already dead before the sweep**, and that is the finding
worth adding: they point at `work/door/all-census-idiom-forces-the-visit-not-the-update`,
which moved to `work/census/` in the **design-free sweep of
2026-09-20** — so a re-home breaks these citations exactly as a
closure does, five days earlier and with no ledger entry to resolve
them by. **This row's fuse metaphor is too narrow**: it is not only
that a citation dies the day its program closes, it is that a citation
dies the day its row MOVES, and rows move far more often than programs
close.

| citing file | cited path | now |
| --- | --- | --- |
| `crates/topo/src/boolean/mod.rs` | `work/door/all-census-idiom-forces-the-visit-not-the-update` | `work/census/` |
| `crates/topo/src/query.rs` | `work/door/all-census-idiom-forces-the-visit-not-the-update` | `work/census/` |
| `crates/editor-core/tests/m4_pr1_dims.rs` | `work/door/all-census-idiom-forces-the-visit-not-the-update` | `work/census/` |
| `crates/editor-core/tests/wire_operand_door.rs` | `work/docm/the-third-datum-axis-phrase-lives-in-mate-member.md` | the row closed (#2984); the citation now states what the code does and names `docs/DOC-LEDGER.md` sweep 19 |

The fourth is shape (2) done as the near-miss above recommends: the row
it cited had already been dead at `work/docm/` since sweep 14, was
closed by DOOR's #2984, and would have been dead again at `work/door/`
within the week — so pointing it at a third directory would have been
the third wrong answer. It names the ledger instead, which is the only
form that does not rot.

**The gate this strengthens is still the weak one** (shape 1): assert
that a cited `work/` path resolves. All four paths above were simply
absent, and CI said nothing across two sweeps and a re-home.
