---
id: LIB-MECH1
kind: unit
title: the mechanical residue bundle — the python-feature clippy row and five banked repairs
status: review
opened: 2026-09-03
pr: 1696
branch: claude/lib-mechanical-clippy-ci-tadd42
refs: [the-python-feature-half-of-pncad-py-is-linted-by-no-ci-row, pncad-py-python-feature-clippy-lane-is-red, census-points-at-a-deleted-lib-log, stub-check-never-descends-class-attributes, python-refusal-tag-values-pinned-nowhere, pin-mismatch-recourse-emitted-twice, subject-body-drops-the-declared-contacts]
---

Mechanical under the 08-29 ruling (no A/B row, no block slot; brief-as-spec,
the LIB-CUR4 precedent). The B-family slate closed the census's charters and
left a banked-findings pile behind it; this unit takes the members of that
pile whose done-state is MACHINE-CHECKED rather than judgment-checked, in one
PR, and leaves the rest.

## The selection rule, and what it excluded

Taken: a finding whose fix is fully specified by its own issue file, lands
inside `crates/pncad*` or the CI halves, and ends in a guard, a pin or a
green lane rather than in a design call.

Not taken, with the reason each stays open: a fix that needs a ratified
decision or Ev's sign-off (`facade-guards-defer-to-rustdoc-json`,
`step-writer-hardcodes-user-header-fields`, `bench-corpus-staleness-hole`,
`stl-header-refuses-plausible-names`,
`facade-polygon-door-demoted-without-replacement`,
`save-a-copy-duplicate-id-bricks-store`, `epsilon-has-no-type-of-its-own`,
`memo-admission-and-resolver-state`); a semantics call that changes an
observable raise
(`the-quantity-boundary-compares-and-hashes-as-if-poison-and-signed-zero-cannot-arrive`);
a curation judgement that belongs to
the queue `DanglingRef` joined, not to a mechanical lane
(`escalation-payload-is-uncarried-under-thirteen-refusals`,
`loop-key-is-uncurated-and-invisible-to-payload-scans`,
`next-payload-rung-under-the-cur3-cur4-carriages`,
`mesh-pick-error-is-unmatchable-under-node-pick-error`,
`resolution-failure-arms-are-unmatchable-under-resolution`); a convention
change that is only worth making for every op at once
(`lib-per-arm-error-tags`); prose authored into kernel crates
(`tier-3-prime-findings-render-through-debug`,
`mate-contradiction-names-one-mate-twice`, and finding 2 of
`pin-mismatch-recourse-emitted-twice`); and work that is a unit apiece rather
than a bundle member (`chamfer-has-no-recipe-layer-door`,
`pncad-py-seven-doors-lack-field-projection`,
`load-path-stringifies-structured-refusals`,
`pncad-py-doc-has-no-node-kind-read-door`).

## Delivered

**The python-feature clippy row** (both duplicate issues). `cargo clippy
-p pncad-py --features python --all-targets -- -D warnings` now runs in
`ci.yml`'s `python-suite` job and `nightly.yml`'s ungated re-take, with
the local mirror beside the other clippy rows. Sited there rather than in
`clippy` because those jobs already install an interpreter, already cache
the `python` feature graph, and are off the Rust critical path: no job the
merge gate waits on gains any compile time. Measured 37.0s cold, 0.17s
warm. `Datum::axes` takes a reasoned `#[allow(clippy::type_complexity)]`
— the written tuple IS the `#[pyo3(get)]` projection and `pncad.pyi`
states it literally, so an alias would name the pair on one side of the
boundary only. What the siting gives up, stated at the step: the row
inherits the seed key, so a lint reaching `src/py/` from below the seeds
waits for the nightly.

**The stub check descends one level.** `test_stubs.py` compared
top-level names only, so a forgotten `.pyi` enum member changed no
top-level name and left the suite green. It now walks every top-level
stub class's attributes both directions, with the `Final`-versus-bare
annotation convention named and self-enforcing, and a fail-loud roster of
the statement kinds the walk understands. It found real drift on its
first run: `Datum.in_plane` is a live `#[pyo3(get)]` the stub never
declared.

**The census stops pointing at a deleted file.** Its three
`docs/LIB-LOG.md` pointers re-aimed at `work/lib/log.md`'s "LIB residual
register"; the lineage sentence keeps the old spelling as history. A
tree-wide sweep found no other live pointer at it.

**The tag VALUES are pinned, the whole table.** 37 tag functions, 354
literal occurrences, of which 189 distinct words appeared in no test at
all before this. `TAG_INVENTORY` is re-derived from `src/tags.rs` at test
time on the no-interpreter row; a renamed, added or deleted value, a new
or deleted tag function, or a moved delegation reds by name. It reads through
`test_utils::source`, the shared lexer, rather than one of its own: the
first draft hand-rolled a reader, `reader_census` caught it as the tree
is built to, and the conversion is the adoption that crate's own docs ask
of `pncad-py` by name. The reader fails loud on anything it cannot follow
rather than enumerating less. What it proves is a rename; a MIS-mapping
still belongs to the construction pins, which cover 18 of 37 functions by
sample, and the test says so.

**One recourse paragraph per refusal.** `impl PartResolver for Workspace`
re-appended `PIN_MISMATCH_RECOURSE` to a message whose `Display` already
ended on it, so every pin-mismatch that reached an evaluation carried the
paragraph twice. The arm is gone and both armed pins flipped with it —
and the demo's gap note went with them, since after the fix it would
print a false statement to a reader.

**`subject_body` keeps its contacts.** It called `product::sources_of`
and dropped the records one line later, so `pncad.subject_body` answered
with a plain body and a subject carrying declarations reported its own
certified seam as undeclared under the tier-3′ gate. The door hands on
the pair `sources_of` already builds. The regression pin's red was seen:
16 undeclared-contact findings before, clean after, on a body whose
value-door read passed throughout.

## The review round

A style pass and a correctness pass ran over the finished branch. The
correctness pass found no behaviour bug — it attacked the widened
`subject_body` and the deleted `PinMismatch` arm by execution and both
held — and between them they found four claims THIS BUNDLE had written
that were false: the demo's roster still advertising a gap it had just
closed, "three armed pins" that were two, a coupling called unguarded
that `crates/viewer/tests/instance_authoring.rs` guards, and a
dimensionless-direction convention claimed over a field whose first half
is a position in metres. All four are repaired here. Two holes were also
closed: `ty` could not see the one property this bundle added to the
stub, and the class walk's reach was pinned by name but not by count.

## Banked

Three findings carried out rather than swept, each with its own file:
`work/lib/select-refusal-predicate-names-are-unpinned.md` (four of five
reachable `SelectRefusal.predicate` names pinned nowhere, and neither
carrying arm is constructible from `pncad-py`) and
`work/lib/two-refusals-carry-no-recourse-sentence.md` (finding 2 of the
pin-mismatch issue — authoring recourse prose into a kernel crate, which
is not mechanical), and
`work/lib/datum-in-plane-reads-back-a-length-pair-bare.md` (the write
door takes `tuple[Length, Length]` where this read door answers bare
floats; changing a published Python type is not mechanical either).
