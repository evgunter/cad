# Sweep 14 — 2026-09-14: DOCM leaves the tracker

Sweep SHA: `1cb0e8000dc74adb3fa7c8bd5a8e51b9fcd6bebe` — the commit
immediately before the deletion (on the closing PR's branch, reachable
from `main` through that PR's merge commit; it is the state in which
DOCM's directory is complete, `program.md` reads `status: closed`, and
every row in it is closed), so every path below is recoverable at
`git show 1cb0e8000dc74adb3fa7c8bd5a8e51b9fcd6bebe:work/docm/<FILE>` and
`git show 1cb0e8000dc74adb3fa7c8bd5a8e51b9fcd6bebe:docs/DOCM-EXIT-WALK.md`.

DOCM — the document model — opened 2026-09-03 and closed 2026-09-14
on the walk Ev ratified in advance (in chat, 2026-09-13: "write it as
ready to merge"), merged with this sweep. **Nine units**, every one
merged on its own green hosted head with a v6 dual: DOCM-4 (#1808),
DOCM-3 (#1803), DOCM-1 (#1829), DOCM-2 (#1860), DOCM-5 (#1871),
DOCM-7 (#2028), DOCM-6 (#2035), DOCM-8 (#2073), DOCM-9 (#2534) —
ordinals 1800–1808, samples #126–#130, #148, #149, #154, #187 — and
three mechanical E units without a review lane (#1839, #1840, #1851).
Per the sweep-5 rule the directory leaves whole — `program.md`,
`plan.md`, `log.md`, the nine unit rows and the fourteen closed issue
rows — with the thirty-two OPEN rows re-homed first, below, and one
closed at the sweep.

| program | title | closed | done-state of record |
| --- | --- | --- | --- |
| `docm` | DOCM — the document model | 2026-09-14 | this entry; the walk at the sweep SHA; the design at `crates/editor-core/REFERENCES.md` (DM1–DM6) and `crates/editor-core/IDENTITY.md` (DI1–DI5), both ratified in-chat 2026-09-04 and amended by Ev's rulings of 2026-09-06; the A/B record at ordinals 1800–1808 in `docs/MODEL-AB-LOG.md` |

### What survived, and where

The program's output is the tree, not its directory:

- **The design.** `crates/editor-core/REFERENCES.md` — what a recipe
  reference may be (DM1 the derived frame, DM2 the carrier-kind read,
  DM3 the projection node, DM4 the n-ary union with its member-space
  declaration channel and flat merged name, DM5 pairwise-distinct
  inputs, DM6 splice not added) — and `crates/editor-core/IDENTITY.md`
  — a held value names the world it came from (DI1–DI5). Both were
  `docs/DOCM-*-DESIGN.md` until this sweep; present tense, clause ids
  kept, `docs/DESIGN.md`'s companion table repointed. The three
  identity clauses whose builds are the viewer's (DI1, DI2's re-mount,
  DI5) say so and are open on VIEW's and CHROME's slates.
- **The code.** `Datum::FaceFrame` and the carrier-kind read;
  `Node::Part`; the n-ary `Node::Union` with `DocEdit::SetMembers`,
  its declaration channel in member space, the flat `Merged` mint and
  the look-through through merges; `Evaluation` carrying its
  document's identity; the check registry's `Subject` and one gather
  per landing; `PartValue` carrying mate identity and mint health
  across the instantiation seam with `CarriedMintRefusal` at the
  outermost gate; `range.rs`, the certified locally-valid range query.
- **The measured bounds, stated rather than overpromised.** A
  member-space declaration resolves through merges only (a face
  consumed by a split, by containment or by a fragmented merge stays
  order-shaped — WIRE's row); the certified range query certifies
  nothing on the repo's corpus at affordable budgets and cannot yet
  name a failure boundary (PROPS's rows); an inner document's mint
  refusal refuses the outer gate with no advisory channel.
- **A successor program.** `work/edit/` — EDIT, the document model's
  residue — opened in this sweep per `work/README.md`'s rule (a dozen
  items on one territory are a successor's opening slate), holding
  twelve of the thirty-two rows on DOCM's own ground and the band
  4800–4899.

### Residue re-homed before the deletion

Thirty-two open rows moved (an earlier commit of the closing PR, so the
deleting commit finds the directory holding only closed work) and one
closed at the sweep with the reason in its file
(`docm1-face-frame-owes-a-reader-census-ledger-line` — the ledger line
exists on main). Every id is unchanged and every moved row carries a
"Re-homed (2026-09-13)" note saying why it landed where it did.

| item | to | why |
| --- | --- | --- |
| `C6` | `work/edit/` | the `ProgramStep`/`WireStep`/`SegTag` mirrors on `program.rs` — DOCM's inherited Track V row |
| `D366` | `work/edit/` | `NodeErrorKind`'s projection; LIB answers the Python side |
| `authored-step-to-canonical-segment-map-has-no-home` | `work/edit/` | its DOCM half; BOOL's half by announced seam |
| `blend-selection-canonical-check-load-only` | `work/edit/` | one predicate at two doors (`edit.rs`, `persist/*`) |
| `debug-in-prose-residue-after-finding-sink` | `work/edit/` | `PersistError`/`EditError`/`NamingError` prose debt — the inherited Track V class |
| `deletenode-strands-a-declare-payload-name` | `work/edit/` | a payload reference as a weaker edge — a design conversation on the edit vocabulary |
| `doc-param-unit-edit-has-no-door` | `work/edit/` | the `DocEdit` set's missing door |
| `no-docedit-splices-a-deleted-node` | `work/edit/` | DM6 says no rewire; whether a splice is a new persisted edit is EDIT's `[ev]` conversation |
| `pair-doors-outside-the-three-do-not-check-document-identity` | `work/edit/` | DI3's residue at the resolver's doors |
| `pick-grazing-ray-answer-depends-on-candidate-order` | `work/edit/` | `resolve/pick.rs` |
| `recorded-program-arguments-carry-no-notation` | `work/edit/` | the persisted program's bare arguments |
| `replay-and-load-keep-the-document-without-its-maintenance` | `work/edit/` | `Doc::replay` and the load door |
| `blend-slit-name-collides-when-two-rims-share-a-meridian` | `work/wire/` | the names emitter (`names/emit*.rs` is WIRE's) |
| `cut-off-arc-persists-as-a-corner-arc` | `work/wire/` | the names emitter |
| `member-space-look-through-stops-at-splits-containment-and-fragmented-merges` | `work/wire/` | `eval/wire.rs`'s union routing — DM4's measured bound |
| `nobodyroots-classification-has-two-homes` | `work/wire/` | `product.rs` |
| `product-refuses-naming-when-one-instance-is-placed-under-two-roots` | `work/wire/` | `product.rs` |
| `the-pair-verbs-declared-merge-is-asymmetric-in-its-operands` | `work/wire/` | the pair verb's emitter |
| `two-emitter-refusals-a-legal-declared-union-reaches` | `work/wire/` | `names/emit_topo.rs`, `names/emit.rs` |
| `area-overlap-contact-admitted-but-unmerged-refuses-at-the-next-step` | `work/bool/` | the rest door and the F7 gate (`topo/src/boolean/*`) |
| `join-desync-on-the-star-fixture` | `work/bool/` | the boolean's join |
| `rows-do-not-cross-a-boolean-remap` | `work/bool/` | the boolean's key remap (unreachable today) |
| `two-parts-of-one-body-at-one-boolean-refuse-as-ray-exhausted` | `work/bool/` | containment |
| `mate-clocking-has-no-gui-path` | `work/msolve/` | mate authoring (`mate/*`); the viewer half rides as CHROME's announced seam |
| `node-placer-field-docs-say-body-where-instances-are-accepted` | `work/door/` | the fix is written; one PR on `node.rs` |
| `part-fault-partproduct-degrades-the-product-refusal` | `work/door/` | the fix is written; one PR on `eval/parts.rs` |
| `vectorslot-slots-has-no-reader` | `work/door/` | the fix is written; one PR on `eval/slots.rs` |
| `the-third-datum-axis-phrase-lives-in-mate-member` | `work/door/` | the fix is written; one PR on `mate/member.rs` |
| `a-document-vocabulary-declared-outside-the-macro-is-uncensused` | `work/census/` | one vocabulary outside the census macro |
| `load-path-stringifies-structured-refusals` | `work/port/` | the bindings' never-strings contract at a crate boundary (LIB may take it) |
| `pick-face-fuzz-anti-vacuity-guard-trips-at-effort-1` | `work/tint/` | a probabilistic test guard |
| `random-integer-rays-search-trips-at-eps-1e-6-on-one-run` | `work/tint/` | a once-flaky test guard |

Filed by DOCM's last two units on other programs' slates and untouched
by the sweep: `work/chrome/certify-affordance-on-the-bounds-panel`,
`work/lib/certified-range-has-no-python-door`,
`work/lib/document-layer-export-guard-counts-cfg-gated-names`,
`work/props/parametric-polygon-loop-certifies-nothing` (and the
evidence added to `coincidence-zone-priced-budget-at-the-floor`),
`work/chrome/a-declared-union-has-no-one-pass-authoring-path`,
`work/fix/prose-gate-has-no-mechanical-guard`'s fourth instance.

### The docs that moved with the program

| doc | from | to |
| --- | --- | --- |
| `DOCM-REFERENCES-DESIGN.md` | `docs/` | `crates/editor-core/REFERENCES.md` (present tense, DM1–DM6 kept) |
| `DOCM-IDENTITY-DESIGN.md` | `docs/` | `crates/editor-core/IDENTITY.md` (present tense, DI1–DI5 kept) |
| `DOCM-EXIT-WALK.md` | `docs/` | deleted with this sweep; recoverable at the sweep SHA |

The nine unit specs left `docs/` at their merges (the per-merge
deletion notes in `docs/doc-ledger/` name each unit head).
