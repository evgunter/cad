# CENSUS — one vocabulary, spelled by hand in several places (plan)

**STATUS: OPEN (2026-09-11).** Opened in the tracker cut of 2026-09-11
(`docs/WORK-TRACKS-2026-09.md` addendum 3).

Branch prefix: **`census/`**. Away-channel tag `(CENSUS orchestrator)`.
A/B ordinal band **CENSUS = 3900–3999**.

## Charter

The repository's answer to "is this list complete?" is an instrument: a
census test or a gate that enumerates the real population and compares.
Where one exists it works. Where one does not, the list is hand-written
and drifts, and where one DOES exist but scans the wrong population, the
drift is invisible *and* certified — which is worse.

Every row here is one of those three shapes. They are one program
because the fix has one shape too: **find the one place the population
is declared, make every other site read it, and leave an instrument that
fails when a spelling is added.** A lane that has done one of these rows
can do the next.

The trap this program must not spring is the standing one in its purest
form — **the fix mints a fresh instance of the defect it closes**, and
naming that in your own PR body does not prevent it: a census added by
hand is a new hand-written list. An
instrument that enumerates by reading source text is a reader, and the
reader needs a guard of its own — which is the lesson GUARD's
`gate-roster-and-probe-census-have-no-reader-guards` is paying for on
the other side of the fence.

## Territory — none, and why

This program claims **no paths**. Its subject crosses every crate by
construction: the class is "a vocabulary with several spellings", and
the spellings are in `editor-core`, `viewer`, `topo`, `profile`,
`geom-core`, `geom-brep`, `sweep` and `pncad-py` at once. Each row draws
its fence in the PR that lands it and announces it to the owners; the
`keep_out` in `program.md` names the seven that are already known.

## The slate

| item | class | what it is | where the work lands |
| --- | --- | --- | --- |
| `inert-deny-unknown-fields-on-unit-enums` **(closed — CENSUS-INERT-DENY, #2634)** | **M** | Workspace sweep of 73 sites, but each decided mechanically by unit-vs-struct | `crates/editor-core/src/names/role.rs` + ~17 `editor-core/src` files, `crates/viewer/src/prefs.rs`, editor-core wire tests |
| `hand-listed-debug-censuses-in-geom-core-geom-and-topo` **(closed — CENSUS-DEBUG, #2655)** | **M** | Pattern already proven by #2093, but 9 impls in 3 crates and the PartialEq half needs a scope call | `crates/geom-core/src/spline/knots.rs`, `crates/geom-core/src/spline/hull.rs`, `crates/geom/src/curves/nurbs.rs` (incl. `nurbs_curve!`), `crates/geom/src/surfaces/nurbs.rs` — `Debug` impls and the sibling `PartialEq` impls |
| `S113` | **M** | Multi-file; member (d) needs a real invariance re-derivation of the retry ladder. | `crates/geom-core/src/ring_interval.rs`, `crates/topo/src/chart_region.rs`, `crates/topo/src/splitting/containment.rs`, `demos/README.md` (+ S64/S67/S74/S89/S98 members) |
| `S133` | **M** | profile half discharged; remaining sweep+disposition rides staffed lanes, scope needs judgement. | `crates/topo/src/chord_join.rs`, `crates/profile/src/path/{path.rs,family.rs,program.rs}` |
| `S57` | **M** | Five known sites, but widening a crate-scoped guard to a concept needs a new instrument. | `crates/editor-core/src/names/emit_topo.rs`, `crates/sweep/src/blend/{build.rs,battery.rs}`, `crates/topo/src/face_normal.rs`, the anti-re-fork guard in `scripts/gates/*`; unswept `crates/mesh/src/walk.rs`, `crates/step-export/src/` |
| `pncad-py-eval-err-variants-outside-the-tag-inventory` **(closed — CENSUS-TAG-REACH, #2660)** | **E** | Arrived from M10 (2026-09-13). Ten sites in `py/value.rs` mint the evaluation door's refusal words as string literals the inventory cannot lex; the disposition between the row's two closes was the orchestrator's, and the third one it took — give the door a typed reason — is what makes the class closable. Landed 2026-09-15; E held, because the design call was made in the spec. | `crates/pncad-py/src/errors.rs` (`EvalReason`), `crates/pncad-py/src/tags.rs` (`eval_reason_tag`), `crates/pncad-py/src/tests.rs` (`TAG_INVENTORY`, `TAG_CONSTS` and the reader's own guard), `crates/pncad-py/src/py/value.rs` |
| `a-document-vocabulary-declared-outside-the-macro-is-uncensused` | **M** | Arrived from DOCM (2026-09-13). The three live instances are dispositioned by hand at the site; what is open is the general case, and every door to it is a walk over source TEXT — the exact instrument PR 2501 removed as unsound. | `crates/editor-core/src/program.rs` (the `document_vocabulary!` invocation and its `DOCUMENT_VOCABULARIES` doc) |
| `census-sees-an-inert-attribute-but-not-a-missing-one` | **H** | Arrived from `inert-deny-unknown-fields-on-unit-enums` (2026-09-15); class estimated by that lane, order not yet placed. The census sees the inert attribute and is blind to the missing one. The walk is the easy half; the verdict key is a design call (which `Deserialize` types OWE the attribute) and the one confirmed instance changes what a document accepts. | `crates/test-utils/tests/deny_unknown_fields_census.rs`, `crates/editor-core/src/persist/mod.rs`, and the msolve instance `crates/editor-core/src/mate.rs` |
| `hand-listed-partialeq-siblings-outside-the-census-debug-fence` **(closed — CENSUS-HAND-LISTED-SIBLINGS, #2712)** | **M** | Arrived from `hand-listed-debug-censuses-…` (2026-09-15) with its class estimated by that lane; order not yet placed. Six hand-listed `PartialEq`/`Debug` walks the CENSUS-DEBUG unit checked and filed rather than swept. All six repaired with the added-field case executed at each; the four `KNOWN_HAND_LISTED` entries deleted in the same diff, leaving one. `SignCertificate`'s braced struct shape was measured (zero of four rendered names are fields, zero of five fields are rendered) and the shape was what went. Two of the row's own claims were wrong and are corrected on it: `loop_bit_eq` is held by E0027 and always was, and `SketchPlane` declares one field, not two. It also records that `expr.rs` and `topo/src/props.rs` are claimed by no open program, which is why the row is on this slate at all. | `crates/editor-core/src/{expr.rs,mate/coset.rs,program.rs,names/role.rs}`, `crates/topo/src/props.rs`, `crates/profile/src/lib.rs`; `crates/editor-core/src/clearance.rs` is shell's under its own row |
| `componentwise-equality-of-the-linear-types-is-hand-listed` | **M** | From CENSUS-HAND-LISTED-SIBLINGS (2026-09-16). `a.x == b.x && a.y == b.y && a.z == b.z` at eleven sites in seven crates, none tied to a declaration; executed — a fourth component on `Vec3` compiles `editor-core` clean. Three kernel copies are the row (two of them one shared pair) and six assertion sites ride it; the disposition is whether the linear types should carry the door, not a sweep of eleven. | `crates/editor-core/src/{mate/coset.rs,clearance.rs}`, `crates/mesh/src/planar.rs`, `crates/step-import/src/assemble.rs`, three test suites and `demos/tour/src/skinned.rs` |
| `py-discriminant-getters-under-src-py-are-outside-every-inventory` **(closed — CENSUS-PY-GETTERS, #2663)** | **M** | From CENSUS-TAG-REACH (2026-09-15). 30 `-> &'static str` functions under `src/py/`, 6 minting 23 Python-visible words no inventory reads — and **7 of the 23 have a second spelling in `tags.rs`**, held equal by nothing. Two are named `*_tag` and live outside that file. | `crates/pncad-py/src/py/{mate.rs,assembly.rs,refactor.rs,mesh.rs,doc.rs}`, `crates/pncad-py/src/tags.rs` |
| `py-reason-and-variant-literals-outside-any-enum` **(closed — CENSUS-PY-RAISE-LITERALS, #2682)** | **M** | From CENSUS-TAG-REACH. Nine words minted at raise sites in four files: six in doors with no enum, three on an attribute an inventoried map otherwise fills — and only ONE of those three is a second spelling of a word its own map mints (re-measured 2026-09-15; the row carries the correction). Carries a separate `AttributeError` defect at `flush.rs`. | `crates/pncad-py/src/py/{flush.rs,value.rs,doc.rs,mesh.rs}` hold the nine; `crates/pncad-py/src/py/select.rs` is the `AttributeError` half; `crates/pncad-py/src/{errors.rs,tags.rs}` |
| `tag-vocabularies-restated-in-py-doc-comments` | **M** | From CENSUS-PY-GETTERS (2026-09-15); **filed by that unit and absent from this slate until 2026-09-15** — see the log. Eight getters under `src/py/` restate in prose a vocabulary their own map holds, in **three shapes** the row refuses to flatten: four a bare roster, three mixing a map's words with neighbouring attribute names, one (`ClassAdmission::why`) using the words to say which arm carries which sentence, where deleting them loses a statement the map does not make. **A disposition has to check `pncad.pyi` first**: a pyo3 doc comment is the property's `__doc__`, so where the stub names the attribute and not its words, deleting the roster deletes it from `help()` — and the stub is not uniform. | `crates/pncad-py/src/py/{assembly.rs,mate.rs,checks.rs}`, `crates/pncad-py/pncad.pyi` |
| `ring-contact-and-census-contact-share-two-words-by-prose-alone` | **E** | From CENSUS-PY-GETTERS (2026-09-15); **filed by that unit and absent from this slate until 2026-09-15** — see the log. Three maps share contact words held equal by a doc sentence nothing executes: rename one map's `vertex_on_edge` and `TAG_INVENTORY` reds on that map alone while the sibling keeps the old spelling, its doc still claiming they agree. The fix is three lines beside two that already exist. Its tail is a kernel question, not a binding one — whether `edge_along_edge` and `edge_edge_overlap` are one concept under two names. | `crates/pncad-py/src/tags.rs`, `crates/pncad-py/src/tests.rs` |
| `datum-kind-vocabulary-is-hand-spelled-and-uncensused` | **E** | From CENSUS-DEBUG's sibling sweep. `Datum.kind`'s five words are a `&'static str` struct field on a `#[pyclass]` — neither a literal beside a key nor a getter, so every sweep so far was blind to the shape; three have no Rust pin at all. | `crates/pncad-py/src/py/value.rs` |
| `evaluationerror-stub-lists-five-reasons-and-the-door-raises-six` | **E** | From CENSUS-TAG-REACH. `pncad.pyi` hand-lists a vocabulary that now has a machine census (`EvalReason`); the stub says five and the door raises six. | `crates/pncad-py/pncad.pyi`, `crates/pncad-py/src/errors.rs` |
| `pncad-py-tests-rs-is-six-thousand-lines-and-carries-two-unremeasured-floors` | **M** | From CENSUS-TAG-REACH. **9162 lines** (`git show <sha>:crates/pncad-py/src/tests.rs | wc -l` at `c9c007ef6`; 6317 when filed, 6798 at the merge base, 7805 on `main` at `370bd6f41`) holding the taxonomy pin, three censuses, **four** source recognisers and their guards — three over Rust, one over `pncad.pyi`, the literal-attribution one 658 lines by itself; two magic floors (`>= 60` functions, `>= 500` literals) that nothing re-measures. CENSUS-ERRORS-ARRIVAL grew it 997 lines and CENSUS-ARRIVAL-RESIDUE a further 1357 (1544 added, 187 removed), and the row carries both sides of the split argument now — and records that its first re-measurement was born stale, that the correction then went stale at its own MERGE, and that the "two recognisers" beside the line count was itself a count the row never re-measured. The floors are the row; the split is the file owner's call. | `crates/pncad-py/src/tests.rs` |
| `four-censuses-of-python-visible-vocabulary-in-one-crate` | **H** | From CENSUS-TAG-REACH. `TAG_INVENTORY`, `NODE_KIND_ROSTER`, `surface_census` and `prose_census` ask one question — can a Python caller reach every member of this vocabulary — over four populations with four devices; two more devices in `tests/` bring it to six. Whether six should be fewer is the design call. CENSUS-ERRORS-ARRIVAL's `ERRORS_MINTING_ITEMS` is **not** a member and the row says why — it asks whether an ITEM arrived, not whether a word is reachable — but it is a design input, being the one question no compile-time device can answer. | `crates/pncad-py/src/{tests.rs,surface_census.rs,prose_census.rs,node_kind.rs}`, `crates/pncad-py/tests/{test_binding_census.py,test_stubs.py}` |
| `sixty-one-tag-words-are-minted-by-two-or-more-maps-and-seven-are-read` | **M** | From CENSUS-PY-GETTERS (2026-09-15). `tags.rs` mints 61 words that two or more maps speak — `band` in sixteen, `escalated` in ten. Seven were read and dispositioned; **54 are covered by a scoping rule nobody read them against**. The instrument that holds the population to a roster landed with the unit; the judgement on the remainder did not. | `crates/pncad-py/src/tags.rs`, `crates/pncad-py/src/tests.rs` |
| `errors-rs-holds-four-python-visible-maps-and-nothing-enumerates-that-file` **(closed — CENSUS-ERRORS-ARRIVAL, #2691)** | **M** (was E) | From CENSUS-PY-GETTERS. Every map has a pin; the defect is that **nothing enumerates the file**, so a fifth arrived with no "NEW tag function" sentence to catch it. Landed 2026-09-15 as an arrival alarm keyed on the file's LITERALS rather than on any item form — neither of the two shapes the row named. **E was wrong**: the row's own proposal inherits the form-keying that made every earlier instrument here go blind, so the disposition had to be found rather than read off, and an instrument plus its own guard had to be built. | `crates/pncad-py/src/errors.rs`, `crates/pncad-py/src/tests.rs` |
| `payload-attribute-names-are-spelled-twice-and-held-equal-by-nothing` | **M** | From CENSUS-ERRORS-ARRIVAL (2026-09-15); class estimated by that lane, order not yet placed. 67 `("word", field.is_some())` rows across four payload modules no instrument reads, naming 64 Python-visible attribute words; **31 of them are spelled a second time at a raise site under `src/py/`** and nothing holds the two equal. No live mismatch — the defect is that nothing would catch one. | `crates/pncad-py/src/{edit_payload.rs,check_payload.rs,mate_payload.rs,pick_payload.rs}`, `crates/pncad-py/src/py/`, `crates/pncad-py/src/tests.rs` |
| `dimension-mismatch-sentence-is-spelled-in-two-crates-and-held-equal-by-nothing` | **E** | From CENSUS-ERRORS-ARRIVAL (2026-09-15); class estimated by that lane's fix pass, order not yet placed. `"cannot apply \`{}\` to {} and {}"` is rendered word for word by two `Display` impls in two crates, held equal by nothing — **while the doc two lines above the first argues the two types are deliberately unrelated**. The prose asserts distinctness and the string asserts identity; which reading is right is the row. | `crates/pncad-py/src/errors.rs`, `crates/editor-core/src/expr.rs` |
| `the-field-brace-fingerprint-is-spelled-at-eight-sites-in-six-crates` | **M** | From CENSUS-ERRORS-ARRIVAL (2026-09-15); class estimated by that lane's fix pass, order not yet placed. `reads_as_prose` states the Display-vs-Debug rule with two fingerprints; the `" { "` half is re-spelled at **seven executable sites in five crates** outside it (thirteen raw hits, the rest prose) and **none of the seven carries the other half**, so each is a silently weaker test than the rule it quotes. `prose_census.rs`'s needle set is where the sweep's own blind spot is. | `crates/{topo,viewer,sweep,editor-core,pncad-py}/…` — seven checks; `crates/pncad-py/src/prose_census.rs` |
| `the-errors-arrival-blind-spot-list-claimed-exclusivity-and-was-short` | **E** | From CENSUS-ERRORS-ARRIVAL (2026-09-15); class estimated by that lane's fix pass, order not yet placed. The **fourth consecutive** short exclusivity list (standing finding 2), wrong in two executed places. Both closed; what the row carries is the four-item residue, each executed — an item spelling no literal, a within-item word swap, an `impl` at indentation losing its qualifier, and a `held_by` column nothing re-derives. | `crates/pncad-py/src/tests.rs`, `crates/pncad-py/src/errors.rs` |
| `validation-error-reason-is-raised-and-the-stub-declares-only-door` | **E** | From CENSUS-ARRIVAL-RESIDUE (2026-09-15); class estimated by that lane. `ValidationError` is raised with `reason` on its measurement refusal and with `door` on its four validator refusals, and `pncad.pyi` declares `door`, `failure_count` and `findings` unconditionally and `reason` not at all — so the stub is short by one attribute AND promises three on raises that carry one. Adding the line is trivial; deciding whether four unconditional declarations should be `Optional` is the row. Held in the meantime by `the_discriminant_attribute_names_are_declared_in_the_stub`, whose gap column reds if the stub gains the declaration. | `crates/pncad-py/pncad.pyi`, `crates/pncad-py/src/errors.rs`, `crates/pncad-py/src/tests.rs` |
| `one-stub-convention-has-two-readers-in-two-languages` | **E** | From CENSUS-ARRIVAL-RESIDUE's style review (2026-09-15). `pncad.pyi`'s `Final`-vs-bare convention has a Python reader (`tests/test_stubs.py`, `ast`) and a Rust reader (`src/tests.rs`, line prefixes and triple-quote parity), held equal by nothing — and **they had already drifted**: a bare `Final` read as class-level in one and instance-level in the other. The drift is closed and tested; the pair that produced it is not. The unit weighed moving the check to Python and kept it in Rust with the reason at the site, so the row is the pair, not the placement. | `crates/pncad-py/src/tests.rs`, `crates/pncad-py/tests/test_stubs.py`, `crates/pncad-py/pncad.pyi` |
| `the-mint-reader-hosts-three-lexer-operations-of-its-own` | **E** | From CENSUS-ARRIVAL-RESIDUE (2026-09-15), disclosing what it took rather than leaving it in a PR body — the unit's spec said to file a wanted widening rather than take it. `balanced_open` (the inverse of `test_utils::source::balanced_end`), `item_start` and `strip_modifier` live inside the census that consumes them, which is the siting unit 6's close-out moved four operations OUT of. `item_start` is the sharper half: the sibling census answers the same question with `boundary_before` alone, which admits an `impl` in type position. | `crates/pncad-py/src/tests.rs`, `crates/test-utils/src/source.rs` (TCOST's and TINT's — a move needs their assent) |
| `prose-counts-of-a-populations-size-in-pncad-py-doc-comments` | **M** | From CENSUS-PY-RAISE-LITERALS (2026-09-15). A doc comment stating how many arms, words or maps a population has, with nothing re-deriving it. 195 raw hits, seven repaired in the unit, ten named unverified — and one (`step_import_error_tag`'s "twenty-two arms") was **never right**, `git log -S` putting the sentence at a commit where the map already had 23. | `crates/pncad-py/src/**` |
| `both-unclassified-crossings-are-unreachable-and-so-is-the-repair-on-one` | **E** | From CENSUS-PY-RAISE-LITERALS. `SelectRefusal`'s eight arms are all matched above the forced wildcard and `ContactClass` has exactly two, both matched — so `unclassified` is the only inventoried word nothing can make the binding emit, **and the `AttributeError` repair sits on that same dead path**, so it cannot go red either. | `crates/pncad-py/src/py/{flush.rs,select.rs}`, `crates/pncad-py/src/tags.rs` |
| `dimension-error-op-carries-twelve-words-minted-at-call-sites` | **M** | From CENSUS-PY-RAISE-LITERALS. Twelve `DimensionError.op` words minted at call sites of two `&'static str` parameters; six asserted, six not. `MeasureUnavailableAt.door` takes one from a kernel struct-field literal in `editor-core`. | `crates/pncad-py/src/py/{quantity.rs,analysis.rs,doc.rs,measure.rs}` |
| `prose-census-cannot-see-a-bypassed-prose-renderer` | **H** | Instrument rework plus triage of 453 unmeasured sites; verdict key is a design choice | `crates/pncad-py/src/prose_census.rs` (`census()` scan set, `declaration_verdict`), plus sites it reds: `crates/viewer/src/session/refuse.rs`, `crates/editor-core/src/edit.rs`, `crates/pncad-py/src/py/`, `crates/test-utils/` |
| `the-prose-word-for-a-kind-has-four-spellings-and-only-display-is-censused` | **H** | Two decisions owed, owner undecided, 23+ sites over seven crates and three programs. | `crates/pncad-py/src/prose_census.rs`, `crates/editor-core/src/{expr.rs,node.rs,mate.rs,edit.rs}`, `crates/viewer/src/{session/refuse.rs,tools.rs,sketch.rs,pane/properties.rs}`, 23 `label()`/`name()` sites across `geom-brep`, `sweep`, `topo`, `profile`, `geom-core` |

## Order

`inert-deny-unknown-fields-on-unit-enums` first: 73 sites decided
mechanically by unit-vs-struct, no design call, and it establishes the
lane's habit of leaving the instrument behind.
`hand-listed-debug-censuses-…` next — the pattern is already proven by
PR 2093, so the row is a repetition with a scope call on the `PartialEq`
half.

`pncad-py-eval-err-variants-outside-the-tag-inventory` third: it is the
charter stated in one file and the smallest row on the slate, and it
rides the habit the first two build. Its one call — widen the reader to
lex the literal-variant `eval_err` sites, or rule a call-site literal
deliberately out of scope and pin the one uncovered word — is decided
in the spec rather than handed to the lane, because it is a question
about the gate's REACH and answering it is the unit.

`a-document-vocabulary-declared-outside-the-macro-is-uncensused` fourth,
and it is the row where this program's standing trap is sharpest: its
three named doors are all source-text walks, and PR 2501 deleted one
such walk after measuring it report agreement over a set missing exactly
the variant it existed to catch. A lane taking this owes the reader's
own guard before the reader, or the reasoned decision to ship the third
door (accept and say so at the site) — which is what ships today and may
well be the right answer.

**The `pncad-py` block runs next, as a block**, and that is a departure
from the class axis worth stating. Five rows — `py-discriminant-getters-…`
(M), `py-reason-and-variant-literals-…` (M), `datum-kind-…` (E),
`evaluationerror-stub-…` (E) and `pncad-py-tests-rs-…` (M) — are one
crate, one class, and all five were found by CENSUS-TAG-REACH's own
sweeps. **Their fix shape is now proven rather than hypothetical**: the
word rides a closed type, the type rides the class where a class
selects the door, and `tags.rs` holds the map the inventory reads. A
lane that has read that unit can do all five; a lane arriving cold in a
month re-derives the argument from scratch, and two of the rows record
facts with a shelf life (which words have a second spelling, which
doors have no enum).

**The four rows CENSUS-ARRIVAL-RESIDUE filed, and the two that are not
CENSUS's.** `validation-error-reason-…` goes first of them: it is the
only row on this slate naming a **live, user-visible** defect rather
than a missing instrument — `ValidationError` is raised with `reason`
and `pncad.pyi` does not declare it — and it is held in the meantime by
a suppression that reds when the stub gains the line, so closing it is
loud. `one-stub-convention-…` follows it, being the same file and the
pair that produced that gap. `the-mint-reader-hosts-three-lexer-operations-…`
wants TCOST's and TINT's assent to move anything, so its readiness is
not this program's to decide alone.

**Two of that unit's findings went to other programs' slates and are
NOT CENSUS's to schedule**:
`work/tint/item-body-takes-a-const-generic-brace-for-an-item-body.md`
and
`work/ciw/doc-gate-cannot-see-a-broken-link-inside-a-cfg-test-module.md`.
Both are recorded here only so a later reader does not go looking for
them on this slate.

`tag-vocabularies-restated-in-py-doc-comments` and
`ring-contact-and-census-contact-share-two-words-by-prose-alone` join
the block as well, and they are placed here two units late: both were
filed by CENSUS-PY-GETTERS and **neither reached this slate until
2026-09-15**, so two units' worth of ordering was reasoned over a set
that did not contain them. The ring-contact row is the cheaper and
goes first of the two — three lines beside two that exist, with a
kernel question as its tail. The prose-restatement row wants the stub
checked before any of its eight sites is touched, which is a reading
task the row has already scoped and nobody has done.

The three rows CENSUS-PY-RAISE-LITERALS filed join the block too, and
one of them is ordered by a fact with a shelf life:
`prose-counts-of-a-populations-size-…` holds **ten named unverified
counts**, several of them over kernel enums, which means they go stale
on someone else's change rather than on this program's — a row that
decays while nobody touches it.

`sixty-one-tag-words-…` and `errors-rs-holds-four-…` joined the block
and ran with it — both are `tags.rs`/`errors.rs` and both were opened by
the unit that landed there, so the same warm context applied.
`errors-rs-holds-four-…` **is closed** (CENSUS-ERRORS-ARRIVAL, #2691).
The 61-word row carries the sharper obligation of the two and is still
open: its instrument ships, and what is missing is the READING of 54
pairs, which is exactly the kind of debt that stops looking urgent once
the instrument is green.

**The four rows CENSUS-ERRORS-ARRIVAL filed, ordered.** They do not run
as a block — they came out of one unit but they answer to four
different owners.

`the-errors-arrival-blind-spot-list-…` goes **first of the four and
soon**, and its argument is warmth rather than class. It is the residue
of the instrument that just landed, entirely inside the two files that
unit touched, and each of its four items is already EXECUTED — an item
spelling no literal, a within-item word swap, a nested `impl` losing its
qualifier, and a `held_by` column nothing re-derives. Almost all of its
value is that those probes exist and are described; a lane arriving cold
re-runs four experiments to get back to where the row starts.

`payload-attribute-names-…` joins the `pncad-py` block. It is the same
shape one file over — a word spelled twice and held equal by nothing —
and it is the literal "everywhere else" the arrival alarm names as out
of scope. Its 31 second spellings are a fact with a shelf life: each is
a raise site that can move.

`the-field-brace-fingerprint-…` is routing, and decays the way
`hand-listed-partialeq-siblings-…` does: its seven executable sites are
in five crates that are other programs', so owners move as programs
close. It carries a tie the others do not — **its own sweep's blind spot
is `prose_census.rs`'s needle set**, which is the instrument
`prose-census-cannot-see-a-bypassed-prose-renderer` exists to fix. If
that row lands first this one should be re-swept before it is specced;
if this one runs first, it owes the re-sweep to itself.

`dimension-mismatch-sentence-…` is **E by size and not by difficulty**,
and goes last of the four. Two `Display` impls in two crates render one
sentence word for word while the doc two lines above the first argues
the types are deliberately unrelated — so the row is a question about
which of the two readings is true, not an edit, and the answer belongs
to whoever owns `crates/editor-core/src/expr.rs`. This plan already
records that `expr.rs` is claimed by NO open program, which is the same
routing hazard the `PartialEq` row carries.

`four-censuses-of-python-visible-vocabulary-in-one-crate` does **not**
ride with them and stays with the H rows: it asks whether four
instruments should be fewer, which is a design call that wants the five
above landed first — three of them change what the instruments see.

`hand-listed-partialeq-siblings-outside-the-census-debug-fence`
**closed 2026-09-16** (#2712). The argument for running it early held:
its routing had not yet decayed, and all six owners were still the ones
the row recorded. It filed one row on this slate —
`componentwise-equality-of-the-linear-types-is-hand-listed`, the same
class one level down, whose disposition is a design question about
whether the linear types should carry the door rather than a sweep —
and two on TINT's, which are **not CENSUS's to schedule**:
`the-per-impl-sight-anchor-is-a-suppression-list-that-shrinks` and
`census-answers-no-field-read-for-a-walk-that-reads-a-field`. The
first of those is the sharpest thing the unit produced and is worth a
sentence here because it changes what this program can rely on: the
arrival census has **no** per-impl blindness detection and, on the
measurement, never had more than one impl's worth.

Its original placement argument, which held:
`hand-listed-partialeq-siblings-outside-the-census-debug-fence` fifth,
and **early for its class rather than late**, which is a deliberate
departure from the order's axis. Most of its six sites are other
programs' — `expr.rs` and `role.rs` are EDIT's, `coset.rs` MSOLVE's,
`profile/src/lib.rs` S-BOOL's — so the bulk of the row is routing, and
routing decays: the row already records that `expr.rs` and
`topo/src/props.rs` are claimed by NO open program, which is a fact with
a shelf life, and every program that closes between now and then moves
an owner. The two sites the census cannot see (`SketchPlane` behind a
delegation, `NameRef` through a tuple index) are the harder half and
are what keeps it **M** rather than E.

Then `S113` and `S133`, which are prose and duplication counts that
several staffed lanes already ride.

`census-sees-an-inert-attribute-but-not-a-missing-one` is **H and goes
with the H rows, but couples to none of them** — the prose-census pair
below has an order between its two members, and this row has no such
tie. Placed here because the class axis is what this order runs on, and
it is genuinely H: the walk is the easy half, and the verdict key is a
design call about which `Deserialize` types OWE the attribute, which is
a reachability question about the type graph that no text walk answers.

A later orchestrator may reasonably pull it forward, and the argument
for doing so is recorded rather than taken: it is the direct complement
of `inert-deny-unknown-fields-on-unit-enums`, its instrument is built
and warm, and the longer it waits the more the one-directional census
reads as the finished answer. What argues against is that its design
call wants more of the tree swept first. Neither dominates today.

The two prose-census rows go last and go together:
`prose-census-cannot-see-a-bypassed-prose-renderer` fixes the
instrument's scan set, and
`the-prose-word-for-a-kind-has-four-spellings-…` is the population that
instrument would then see. Landing the second first means censusing four
spellings by hand into a census that cannot see one of them.

`S57` is the seam row: its call sites are landed here and its
anti-re-fork guard is `scripts/gates/*`, so the guard half is **filed on
GUARD**, never landed from here.

## Review posture

**No A/B protocol** (Ev, 2026-09-15). The band stays claimed for
bookkeeping, as `docs/MODEL-AB-LOG.md` already records for this
program; nothing draws an ordinal from it.

**One style review per unit.** A full correctness review is reserved
for the hardest units — the **H** rows on the slate and nothing else.

Both postures above were settled by Ev in chat on 2026-09-15, which is
why `git log -S` finds nothing older than this plan for either: the
attribution is the record, and the tree carries no earlier one to
check it against.

The scan-set rule this section used to state mechanically — a
correctness arm on every unit that changes what an instrument SCANS —
is **carried by each unit's dispatch brief, written by the
orchestrator** (Ev, 2026-09-15). It is **not** a standing clause in
`docs/prompts/reviewer-style-lane.md`, and **it is not going to be**
(Ev, 2026-09-15, asked directly): *"this doesn't go in
reviewer-style-lane because most implementation work does not refer to
such instruments."* That is a signal-to-noise ruling and it settles the
question — a clause firing on every unit everywhere would tax every
lane in the repo for a case the majority never meet, and a rule that is
skimmed is worse than one that is written per-unit by someone who has
read the diff.

So the obligation reaches a reviewer only because the orchestrator
writes it into that unit's brief, and a unit dispatched without it is
dispatched without the rule. **The trigger is therefore stated here
rather than left to per-unit judgement**: a unit carries the obligation
if it lands, changes, removes or relies on a census, gate, inventory or
source-text reader — **including one the unit itself creates**, which
is the case the first four units all fell under and the one easiest to
miss, because the instrument does not exist yet when the spec is
written.

The hazard it is drawn against is real and is not a function of how
hard the unit was: a census that stops seeing a population fails
silently, reports agreement over the set it can still read, and no test
goes red. So the brief for any unit touching a census, gate, inventory
or reader **names silent omission as a thing to hunt for by name**:
what does the instrument no longer read after this diff, and what would
it report if the population it exists to watch went missing entirely?
That question is cheap for a style lane to carry and expensive to
discover later, which is the whole argument for putting it in the brief
rather than buying a second lane for it.

Units that get that added obligation today:
`inert-deny-unknown-fields-on-unit-enums` (only if it leaves an
instrument behind), `hand-listed-debug-censuses-…`,
`pncad-py-eval-err-variants-outside-the-tag-inventory`,
`a-document-vocabulary-declared-outside-the-macro-is-uncensused`, and
every **H** row — which gets the full review as well.

## How the class column is read

`E` / `M` / `H` is a **dispatch estimate**, made on 2026-09-11 by reading
each row against the tree, and it is the axis this program's order runs
on. It is not a verdict on the finding and it is not in any header: no
field carries it, `work.py` does not parse it, and this table is the only
place it lives. A lane that finds the estimate wrong says so in its PR
and this table is corrected in the same PR.

- **E** — the fix is written in the row or obvious from it: one or a few
  files, no design question, no ruling, small diff.
- **M** — multi-file, or a small design call (where a shared home lives,
  what a door looks like), or a census or instrument to build first.
- **H** — cross-cutting, numeric or algorithmic, gated on a ruling, or
  spanning several programs' territory.

The cut that opened this program is `docs/WORK-TRACKS-2026-09.md`
addendum 3; it is a survey, and this plan supersedes it as the charter.
