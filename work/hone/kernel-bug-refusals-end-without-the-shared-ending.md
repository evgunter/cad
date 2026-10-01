---
id: kernel-bug-refusals-end-without-the-shared-ending
kind: issue
title: topo: the split's kernel-bug refusals end in a '(kernel bug)' tag, not the shared kernel-defect ending
status: open
opened: 2026-09-28
cost: E
priority: P4
---


(ENCL implementer, from the sweep of
`work/encl/kernel-defect-endings-and-repair-labels-have-no-shared-home.md`.)

## What

The kernel-defect ending has one home now:
`geom_core::KERNEL_DEFECT_ENDING` ("There is no way through: this is
a kernel defect; report it"), `KERNEL_OR_FILE_DEFECT_ENDING` for a
body that may have been read from a file, and the
`geom_core::kernel_defect_ending!` macros for a `&'static str` site.
The split's kernel-bug refusals end instead in a "(kernel bug)" tag,
which carries neither the marker `test_utils::refusal::recourse_markers`
counts nor the report:

- `topo::splitting::SplitReduceError::ConsecutiveOnSectors`
  (`crates/topo/src/splitting/mod.rs:416`).
- `topo::splitting::SplitFinishError::TornComponent`
  (`crates/topo/src/splitting/finish.rs:205`)
  (`crates/topo/src/splitting/finish.rs`).
- `topo::chord_join::SplitJoinError::UnpairedLooseEnds`,
  `SectionLoopMixed`, `CutInvariant` (`crates/topo/src/chord_join.rs:427`,
  `:431`, `:435`;
  shared with TANG by territory).

All are rows in `editor-core/tests/refusal_concision_chains.rs`
(`KERNEL_KEYED` admits their keys), and all render zero markers.

## Repair shape

Replace each tag with ". {KERNEL_DEFECT_ENDING}" (or the file
variant where the body may have been read).

## More instances (TOPO, from `euler-op-corruption-refusals-end-in-a-tag`)

That unit gave `EulerOpError`'s corruption arms the shared ending and
swept `crates/topo/src` for the rest of the shape: a string literal
holding "(malformed", "malformed body", "(kernel bug)", "kernel bug",
"a corrupt body)", "repair the reference", "report it", "report this"
or "kernel defect", then every `Display` arm of a variant named
`Corrupt*`, `Torn*`, `Dangling*`, `Stale*` or `*Broken`, which finds
the ones that end in nothing. Outside its variant set, beside the
three above:

- `topo::RevertError`'s link arms end in " (malformed body)"
  (`crates/topo/src/revert.rs`, the `fmt` that writes the tag after its
  `RevertLink` match).
- `topo::pcurves::PcurveMintError::Corrupt` ends in a repair no public
  door offers ("read the structural validators' report and repair the
  reference it names", `crates/topo/src/pcurves.rs`); its sibling
  `SiteRowRefusal::Corrupt` ends in `KERNEL_DEFECT_ENDING` now.
- `topo::shell`'s "stopped resolving mid-construction (kernel bug)"
  (`crates/topo/src/shell.rs`, the `Display` arm that writes it) and
  `replace_face_offset`'s "referential coherence broke mid-plan (kernel
  bug)" (`crates/topo/src/replace_face.rs`, `Corrupt`).
- `topo::splitting::SplitFinishError::Corrupt` ("the finish traversal
  failed (corrupt body)") and `SplitReduceError::CorruptOperand`, which
  ends in nothing (`crates/topo/src/splitting/finish.rs`,
  `splitting/mod.rs`).
- `topo::chart_region`'s `Corrupt` hand-writes its own report sentence
  ("Rebuild the body through the Euler operators, and report this").
- BOOL's ground, recorded only: `BooleanError::CorruptOperand` and
  `TornComponent` ("(kernel bug or corrupt …)", "(kernel bug)" on the
  seam zip), `boolean::solid_contain`'s `CorruptFace` and
  `boolean::contain`'s `Corrupt` (which tells the caller to "repair the
  body's topology"), all in `crates/topo/src/boolean/`.

**Which ending.** The Euler-operator unit found that no file reaches a
tier-1-torn body: kernel crates take no serde, the document layer
persists no `Body`, and STEP import assembles through the public doors,
which preserve tier 1. So a refusal of tier-1 corruption is a kernel
defect and ends in `KERNEL_DEFECT_ENDING`, not the file variant.
`topo::props`' `Corrupt` and `transform`'s `Corrupt` and
`NullScaffold` (`crates/topo/src/props.rs`, `transform.rs`) report
structural corruption in `KERNEL_OR_FILE_DEFECT_ENDING` today, and
should be checked against that finding. A tier-3 (geometric) refusal at
rest, such as `validate`'s `DEFECT` ending, is a different question,
because a file's geometry does reach the body.

**Missed by that sweep, and re-swept after main moved** (PR 3621's
review, C7; the same pattern set, plus `*Invariant` and `*Desync` in
the name pass, on the merge of main at `79555f031e`):

- `topo::chord_join::SplitJoinError::Corrupt` ends in a "(corrupt
  body)" tag ("the join's traversal failed at {entity}",
  `crates/topo/src/chord_join.rs`), and its sibling `SectionInvariant`
  ("curved-section invariant at face …") ends in nothing.
- `topo::boolean::voids`' `VoidInsertError::Corrupt { what: "graft
  refused outside its own error surface (kernel bug)" }`
  (`crates/topo/src/boolean/voids.rs`, the arm that maps a graft
  refusal), rendered as the bare `what`.
- `BooleanError::ClassificationInvariant` ("classification invariant
  violated: {what}") ends in nothing (`crates/topo/src/boolean/mod.rs`).
  The "(kernel bug or corrupt …)" tag the BOOL bullet above names is
  `BooleanError::JoinDesync`'s and the seam zip's "(kernel bug)" is
  `ZipCorrespondence`'s; `CorruptOperand` ends in "(a broken body)".
- `topo::props::quad_lane`'s point lookup reports a corrupt body as
  `PropsError::QuadratureUnsupported` ("corrupt body reaching the
  quadrature lane (a key did not resolve)",
  `crates/topo/src/props/quad_lane.rs`), whose render tells the user to
  state the face inside the certified inventory: a false recourse, not
  only a missing ending.
- `topo::splitting::containment`'s `CorruptLoop` ("loop … is not
  walkable") ends in nothing.
- Main's `BooleanError::VolumeCorrupt` puts
  `KERNEL_OR_FILE_DEFECT_ENDING` on an operand's props corruption (and
  `KERNEL_DEFECT_ENDING` on the result's), and `census`'s
  `Undecided::CorruptInstance` ends in the file variant too. Both are
  the same re-check as the `props`/`transform` note above: whether a
  file reaches the structure they read.

The pattern cannot see a corruption refusal whose variant has an
ordinary name and whose text names no defect (the `quad_lane` arm was
found by its `what` literal, not its variant), nor a `what` payload
built outside a string literal.
