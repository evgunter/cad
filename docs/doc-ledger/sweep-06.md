# DOC-LEDGER sweep 6

Three sweeps carried this number. They are distinct sweeps of distinct
subjects, kept in one file so that a citation reading "DOC-LEDGER sweep 6"
resolves to one place.

## Sweep 6 — 2026-09-04: S-MATE leaves the tracker

S-MATE — the mate program — closed on the ratification of
`docs/S-MATE-EXIT-WALK.md` (`[ev]` PR #1528, merged by Ev 2026-09-01;
confirmed in chat 2026-09-04). Sweep 5's rule. Four files: `work/mate/`'s
`program.md`, `plan.md`, `log.md` and `MATE-EXIT.md`. Nine units, ordinals
1300–1308 in `docs/MODEL-AB-LOG.md`. Residue was re-homed before the sweep
and each moved row carries its own note.

Sweep SHA `386e170f`.

    git show 386e170f:work/mate/<FILE>
    git show 386e170f:docs/S-MATE-EXIT-WALK.md

Done-state of record: this note, the walk at the SHA above, the design at
`crates/editor-core/ASSEMBLY.md`, and the A/B rows.

## Sweep 6 — 2026-09-03: finished work leaves `docs/`; its design moves beside the code

**The rule this sweep adds.** A document written for the implementer of
finished work — an exit walk, a merged unit's spec, a design conversation
whose subject shipped — leaves `docs/`. What a later reader still needs
from a design conversation is rewritten, present tense, as a README beside
the code it governs, keeping the clause ids; `docs/DESIGN.md`'s companion
table points at those pages. An exit walk is replaced by its note here:
**this ledger is the closed program's done-state of record** (CLAUDE.md,
`work/README.md`, `memories/MEMORY.md`). **Seventy-three files.**

Sweep SHA `3ec71b16575c5887bae358331e517d2ad9348404`.

    git show 3ec71b16575c5887bae358331e517d2ad9348404:docs/<NAME>

### Exit walks of closed programs (12 files)

Every one was ratified by Ev before it went; the program's directory
had already left `work/` (sweeps 3 and 5). Closing facts, so a pointer
that lands here resolves without opening the walk:

| walk | program closed | ratified on | done-state now |
| --- | --- | --- | --- |
| `M5-EXIT-WALK.md` | 2026-08-03 | the M5 PR 14 exit sweep | this row; design at `crates/geom-brep/README.md` |
| `M6-EXIT-WALK.md` | 2026-08-08 | PR #243 | this row; carried items in `work/issues/m6-carried-items-register.md` |
| `M7-EXIT-WALK.md` | 2026-08-09 | PR #300 | this row (STEP import is live) |
| `M8-EXIT-WALK.md` | 2026-08-15 | PR #508 | this row |
| `M9-EXIT-WALK.md` | 2026-08-27 | PR #1041 | this row; the C7 join lane at `crates/topo/README.md` |
| `ASM-EXIT-WALK.md` | 2026-08-23 (v1 scope) | in-session | this row; design at `crates/editor-core/ASSEMBLY.md` |
| `GUI-EXIT-WALK.md` | 2026-08-28 | PR #1121 | this row; architecture at `crates/viewer/README.md` (GUI-5/GUI-6 banked in `docs/LONGTERM-IDEAS.md`) |
| `PCURVE-EXIT-WALK.md` | 2026-08-29 | in-chat | this row; `docs/PCURVE-UNIFY-DESIGN.md` stays (P-2 residue is S-CERT's) |
| `S-BLEND-EXIT-WALK.md` | 2026-08-31 | PR #1370 | this row; vocabulary at `crates/sweep/README.md`, enclosing tangency at `crates/profile/README.md` |
| `S-QA-EXIT-WALK.md` | 2026-08-31 | PR #1341 | this row |
| `S-MATE-EXIT-WALK.md` | 2026-09-04 | PR #1528, merged by Ev 2026-09-01; ratification confirmed in-chat 2026-09-04 | this row; design at `crates/editor-core/ASSEMBLY.md`; the S-MATE sweep in this file |
| `FILLET-EXIT-WALK.md` | 2026-09-06 | PR #1973, ratified in-chat 2026-09-06 | this row; sweep 7; vocabulary and bands at `crates/sweep/README.md` |
| `DOCM-EXIT-WALK.md` | 2026-09-14 | ratified in advance in chat 2026-09-13 ("write it as ready to merge"); merged with sweep 14 | this row; sweep 14; design at `crates/editor-core/REFERENCES.md` (DM1–DM6) and `crates/editor-core/IDENTITY.md` (DI1–DI5) |
| `FIX-EXIT-WALK.md` | 2026-09-21 | ratified in advance in chat 2026-09-21 ("please do close fix; most of those were either mis-filed or should've been done as drive-by fixes"); merged with sweep 18 | this row; sweep 18; the program claimed A/B band 1700–1799 and never drew an ordinal |
| `S-CERT-EXIT-WALK.md` | 2026-09-06 | PR #1924, merged by Ev 2026-09-06 (the merge is the ratification, per the S-MATE convention) | this row; A/B record ordinals 700–714 in `docs/MODEL-AB-LOG.md`; sweep 7 |


- `M5-EXIT-WALK.md` — M5 exit walk (PR 14) — criteria vs evidence
- `M6-EXIT-WALK.md` — M6 exit walk — criteria vs evidence
- `M7-EXIT-WALK.md` — M7 exit walk — criteria vs evidence
- `M8-EXIT-WALK.md` — M8 exit walk — criteria vs evidence
- `M9-EXIT-WALK.md` — M9 exit walk — criteria vs evidence
- `ASM-EXIT-WALK.md` — ASM exit walk — criteria vs evidence
- `GUI-EXIT-WALK.md` — GUI v1 exit walk — plan vs evidence
- `PCURVE-EXIT-WALK.md` — PCURVE — exit walk
- `S-BLEND-EXIT-WALK.md` — S-BLEND exit walk — criteria vs evidence
- `S-QA-EXIT-WALK.md` — S-QA exit walk — criteria vs evidence
- `S-MATE-EXIT-WALK.md` — S-MATE exit walk — criteria vs evidence
- `FILLET-EXIT-WALK.md` — FILLET exit walk — criteria vs evidence
- `DOCM-EXIT-WALK.md` — DOCM exit walk — criteria vs evidence
- `S-CERT-EXIT-WALK.md` — S-CERT exit walk — criteria vs evidence

### Per-unit specs, unit merged (46 files)

The standing rule (`work/README.md`: a spec is deleted at merge; the
item file, the program log entry and the `MODEL-AB-LOG.md` row are the
record), applied to every spec whose unit had merged, including the
dozen written and merged inside the week before this sweep. Kept
because their units have not merged: `BOOL-9`, `BOOL-10`, `BOOL-12`,
`MESH-12`, `PCURVE-P2`, `TCOST-K1`, `VERBS-C5ARMS` (PR-2 open),
`VERBS-CYLSPH`, and `PARAM-LINT` (a draft never dispatched).

Retired at merge after the sweep, under the same rule (each recoverable
at the parent of the commit that removed it):

- `TCOST-K1-SPEC.md` — TCOST-K1 — the patch-flux lanes' exhausted-budget cost (removed in 9029480ee; PR 1652's body, `work/tcost/TCOST-K1.md` and the ordinal-1400 row are the record)
- `FIX-ERRKINDS-SPEC.md` — FIX-ERRKINDS — one declaration for an error enum and its fieldless kind (removed in fb5825b33; the unit was **declined by Ev in-chat on 2026-09-12** rather than merged, so there is no unit PR — `work/fix/kind-mirrors-have-no-single-declaration.md`'s `## DECLINED` section carries the evidence and the reasoning, and `work/fix/a-new-kind-pair-arrives-unguarded-by-default.md` carries what survives. PR 2417 wrote the spec and is where the feasibility measurement lives.)
- `TCOST-K2-SPEC.md` — TCOST-K2 — `offset_fit::fit_offset`'s per-station seconds: the Bernstein product weight, hoisted (last on `main` at 87d33648c; PR 1697's body, `work/tcost/TCOST-K2.md` and the ordinal-1401 row are the record)
- `TCOST-K3-SPEC.md` — TCOST-K3 — the tier-3 gate's discarded certificate (last on `main` at 6381ebdd9; PR 1703's body, `work/tcost/TCOST-K3.md` and the ordinal-1402 row are the record)
- `FILLET-H4-SPEC.md` — FILLET-H4 — the material-adding closed-rim band, incl. the Phase 1 re-scope (last on `main` at fc38f753b; PR 1752's body, `work/fillet/concave-closed-rim-has-no-band.md` and the ordinal-2000 row are the record)
- `FILLET-RIM-SPEC.md` — FILLET-RIM — `topo::query::rim_of`, the exact door naming a closed rim by any one of its arcs, incl. its two fix-pass amendments (last on `main` at 40d50f272; PR 1821's body, `work/fillet/no-public-rim-arc-selector.md` and the ordinal-2001 row are the record)
- `FILLET-H5-SPEC.md` — FILLET-H5 — the plane-hosted closed rim as the annulus band with hostless crossings, incl. §Re-scope at Phase 1 and the fix-pass amendments (last on `main` at 91e6d4309; PR 1824's body, `work/fillet/repaired-pole-rim-serves-no-closed-door.md` and the ordinal-2002 row are the record)
- `FILLET-ATTR-SPEC.md` — FILLET-ATTR — `NoCornerOfPair`, every refusing crossing named nearest-anchors-first, incl. the fix-pass amendment of C1 to the channel rule (last on `main` at aa5384288; PR 1895's body, `work/fillet/fillet-refusal-describes-unbracketed-crossing.md` and the ordinal-2004 row are the record)
- `FILLET-H6-SPEC.md` — FILLET-H6 — extrude's cap-rim `Smooth` arm measured unreachable at the shipped K and made a typed refusal below the crossover; the must-carry rule homed as `geom_brep::tangent_second_order`; incl. the fix-pass amendments (last on `main` at 195460c7a; PR 1891's body, `work/fillet/extrude-cap-rim-smooth-arm-noop.md` and the ordinal-2003 row are the record)
- `FILLET-H7-SPEC.md` — FILLET-H7 — the ruled band and its transverse cut-off (`CornerConfig::TransverseCap` / `RunOutPolicy::CutOffAtTransverseCap`, ratified on PR 1819), incl. the fix-pass amendments (last on `main` at 235d05241; PR 1897's body, `work/fillet/fillet-ruled-spine-arms-no-surgery.md` and the ordinal-2005 row are the record)
- `FILLET-T-SPEC.md` — FILLET-T — Track T's `D325` + `D326`: the corner fusion's first arc a value, one `kef` door over a snapshot of the input body's faces (last on `main` at b1cc95604; PR 1943's body, `work/fillet/D325.md`, `work/fillet/D326.md` and the ordinal-2006 row are the record)
- `FILLET-SPLIT-SPEC.md` — FILLET-SPLIT — the open bands leave `surgery.rs` for `blend/open/{planar,ruled}.rs` behind the compound-bound entry re-scoped (last on `main` at 71cce611d; PR 1964's body, `work/fillet/surgery-module-holds-four-surgeries.md` and the ordinal-2007 row are the record)

- `BOOL-1-SPEC.md` — BOOL-1 — issue 1152: coplanar-split section boundaries cite non-adjacent surfaces
- `BOOL-2-SPEC.md` — BOOL-2 — issue 1011, the cone arm: point_in_solid learns ray×cone
- `BOOL-3-SPEC.md` — BOOL-3 — issue 1011, the torus arm: point_in_solid learns ray×torus
- `BOOL-8-SPEC.md` — BOOL-8 — issue 433 half (i): the line-continuation junction and `line(len)` off a directed point
- `BOOL-11-SPEC.md` — BOOL-11 — the declared point-target continuation and the structural closer
- `BOOL-13-SPEC.md` — BOOL-13 — the schema demolition: no pre-release schema version
- `CERT-1-SPEC.md` — CERT-1 — the sphere polar acceptance defects (#723 + #893)
- `CERT-2-SPEC.md` — CERT-2 — issue 762 close-out and the chart-speed guard residue
- `CERT-3-SPEC.md` — CERT-3 — issue 924: the rotation-anchor round-trip
- `LIB-G16-SPEC.md` — LIB-G16 — Node::Chamfer, the fillet's twin (recipe door for chamfer_edges)
- `LIB-G18A-SPEC.md` — LIB-G18a — the resolver and memo parameters of Python's `evaluate`
- `MATE-1-SPEC.md` — MATE-1 — issue 945: mates × patterns (the A11 member-vocabulary rider, implemented)
- `MATE-2-SPEC.md` — MATE-2 — issue 1032: declared cylindrical Rest without a planar Rest beside it
- `MATE-3-SPEC.md` — MATE-3 — issue 941 items 1–2: declared cusps (the #131 ruling's kernel half)
- `MATE-4A-SPEC.md` — MATE-4a — issue 973(a): the face rung reaches ef_bound_backed's interior arm
- `MATE-5-SPEC.md` — MATE-5 — issue 943's curved residue: the certified-ε overlap enclosure, cylinder-first
- `MATE-6-SPEC.md` — MATE-6 — issue 946: minting moves to evaluation (the Q1 ruling executed)
- `MATE-7A-SPEC.md` — MATE-7a — issue 968 items 1–2 + the π arm: the torus declared-Rest lane's first unit
- `MATE-8-SPEC.md` — MATE-8 — issue 1435: interior_witness's candidate schedule completed
- `MATE-9-SPEC.md` — MATE-9 — issue 973 part (b), stage 1: the crossing rung as the unified strength's first instance
- `MESH-1-SPEC.md` — MESH-1 — issue 1362: the walk.rs world-origin loop-area anchor
- `MESH-2-SPEC.md` — MESH-2 — issue 555: sub-floor engineered zeros refuse an ordinary annular cap
- `MESH-3-SPEC.md` — MESH-3 — issue 896: the undeclared-pole guard on walk's classification
- `MESH-4-SPEC.md` — MESH-4 — issue 881's remaining half: named ε operations
- `MESH-5-SPEC.md` — MESH-5 — issue 685: the `nu == 1` sizing-intent decision
- `MESH-6-SPEC.md` — MESH-6 — issue 897: the two uncovered S65 cases, measured
- `MESH-7-SPEC.md` — MESH-7 — issues 727 then 726: explicit iso-rectangle doors, and the SHAPE question folded onto the named predicate
- `MESH-8-SPEC.md` — MESH-8 — issue 868: the coherence-detector relocation
- `MESH-10-SPEC.md` — MESH-10 — issue 1562: the torus extent from a split seam
- `MESH-11-SPEC.md` — MESH-11 — issue 1571: the walk's arc premise, verified rather than inherited
- `PCURVE-P1B-SPEC.md` — PCURVE P-1b — the consumers, the fence, the deletions (spec)
- `QA-1-SPEC.md` — QA-1 — gates that report green without running: the #888 residue
- `QA-2-SPEC.md` — QA-2 — the matrix says what it did (#1128, #1122's visibility half, #1051 verification, #1204's minimum)
- `QA-3-SPEC.md` — QA-3 — the debt-charging class: the tools-scope k-lint path pin (#1023 + D183)
- `QA-5-SPEC.md` — QA-5 — the comparison gate that stops comparing (#1038, gate side)
- `QA-6-SPEC.md` — QA-6 — the measured-claim sweep, uncontested J-fence legs (#681, PR 1 of 2)
- `QA-6-PR2-SPEC.md` — QA-6 PR 2 — the measured-claim sweep, remaining legs (issue 681)
- `QA-7-SPEC.md` — QA-7 — CI reports test cost (#469)
- `QA-8-SPEC.md` — QA-8 — what the rustdoc gate cannot see (D180 + D301, together) and the false copies (D181, D182)
- `QA-9-SPEC.md` — QA-9 — the status line that invites wrong action (#1139)
- `SEAT-4-SPEC.md` — SEAT-4 — the Verb substrate, carried by the blend pair (unit spec)
- `SHELL-1-SPEC.md` — SHELL-1 — the `ShellNaming` birth channel: `shell`/`shell_open` return `Shelled<T>` (unit spec)
- `SHELL-2-SPEC.md` — SHELL-2 — `transform_rigid` maps an `Approx` face through the scalar's re-certification lane (unit spec)
- `SHELL-5-SPEC.md` — SHELL-5 — shell of a hollow body thickens every boundary: one thin solid per operand shell (unit spec)
- `SHELL-6-SPEC.md` — SHELL-6 — the cone nappe has one home: `face_nappe` decides once, both offset doors and the displacement read it (unit spec)
- `SHELL-7-SPEC.md` — SHELL-7 — the axial offset door takes a one-surface corner: the full-period torus shells (unit spec)
- `SHELL-8-SPEC.md` — SHELL-8 — shell is per solid, on every solid: the doors scoped, void insertion N-ary, the lift per designated solid (unit spec)
- `SHELL-9-SPEC.md` — SHELL-9 — shell runs the closing pcurve mint: the void door transfers rows, the producer re-derives them (unit spec)
- `SHELL-10-SPEC.md` — SHELL-10 — the simultaneous doors walk only their scope: the partition and the pcurve pass narrowed; the closure check could not be (unit spec)
- `TCOST-1-SPEC.md` — TCOST-1 — the per-file test gate (spec)
- `VERBS-GERMARMS-SPEC.md` — VERBS-GERMARMS — the curved pierce ring lane + the cyl×cyl germ arm (two PRs)
- `VERBS-SHELLFIX-SPEC.md` — VERBS-SHELLFIX — the two teapot-found shell defects (two PRs)
- `VERBS-SPHSPH-SPEC.md` — VERBS-SPHSPH — the sphere×sphere germ lane (ONE PR; the arms are a separate, blocked unit)
- `VERBS-TORAX-SPEC.md` — VERBS-TORAX — the offset-axial torus arm

### Design conversations, implemented, condensed into READMEs (8 files)

| deleted | decisions now at |
| --- | --- |
| `ARMS3-DESIGN.md` (A3-1…A3-3) | `crates/sweep/README.md` |
| `BLEND-VOCAB-DESIGN.md` (V1–V4) | `crates/sweep/README.md` |
| `ENCLOSING-TANGENCY-DESIGN.md` | `crates/profile/README.md`, "Enclosing tangency" |
| `CENSUS-REST-CLOSURE-DESIGN.md` | `crates/topo/README.md`, "At-rest census identity" |
| `MATE-4B-CROSSING-DESIGN.md` | `crates/topo/README.md`, "Crossing backability" |
| `GROUP-BOOLEAN-DESIGN.md` | `crates/editor-core/README.md`, "The group boolean" |
| `GQ6-RESURVEY.md` | `crates/viewer/README.md`, "Toolkit and CI posture" (the ratified toolkit row and the wasm measurement CI re-takes; the survey tables are history) |
| `Q8-SUBSTRATE-2026-08-21.md` | nothing — a dated substrate survey whose anchors were already stale, superseded by the offset decisions at `crates/geom-brep/README.md` |

- `ARMS3-DESIGN.md` — ARMS-3 — general sphere×sphere, and what a run-out at a seam vertex IS
- `BLEND-VOCAB-DESIGN.md` — How a shared blend refusal names its verb — the 917 conversation
- `ENCLOSING-TANGENCY-DESIGN.md` — The enclosing (ρ < 0) fillet tangency — ruled out; a demanding request refuses
- `CENSUS-REST-CLOSURE-DESIGN.md` — At-rest census structural identity (#943 + #591 Door-2) — design conversation
- `MATE-4B-CROSSING-DESIGN.md` — At-rest crossing backability (issue 973 part (b)) — design conversation
- `GROUP-BOOLEAN-DESIGN.md` — Group boolean in the recipe layer — D2 + F4 (ratified: A′)
- `GQ6-RESURVEY.md` — GQ6 re-survey — toolkit, viewport, picking, wasm (2026-08-16)
- `Q8-SUBSTRATE-2026-08-21.md` — Q8 offset/shell substrate survey (opus lane, 2026-08-21)

### Ratified companion design docs of closed programs, condensed into READMEs (9 files)

These were rows of `DESIGN.md`'s companion table. Each row now names
the README; the clause ids and their ratification status are
unchanged, and the READMEs state what the code does where the
conversation and the code had since diverged.

| deleted | decisions now at |
| --- | --- |
| `CURVED-DESIGN.md` (C1–C12) | `crates/geom-brep/README.md` |
| `OFFSET-DESIGN.md` (O1–O6) | `crates/geom-brep/README.md` |
| `NAMING-DESIGN.md` (N1–N7) | `crates/editor-core/src/names/README.md` |
| `SOLVER-DESIGN.md` (W1–W9) | `crates/editor-core/README.md` |
| `PROFILE-LIFT-DESIGN.md` (PP1–PP6) | `crates/editor-core/README.md` |
| `ASSEMBLY-DESIGN.md` (A1–A13, AQ1–AQ8) | `crates/editor-core/ASSEMBLY.md` |
| `CONTACT-DESIGN.md` (C1–C8) | `crates/topo/README.md` |
| `PROFILES-V2-DESIGN.md` (V1–V8) | `crates/profile/README.md` |
| `GUI-DESIGN.md` (G1–G5, GQ1–GQ7) | `crates/viewer/README.md`; its non-binding UI-ideas sketchpad and the undo-as-a-tree concept moved to `docs/LONGTERM-IDEAS.md` |

- `CURVED-DESIGN.md` — M5 curved-geometry design: SSI, pcurves, NURBS depth, fillets (pre-M5 design doc)
- `OFFSET-DESIGN.md` — Offset & shell — the Q8 design conversation
- `NAMING-DESIGN.md` — Persistent naming & selection stability (pre-M4 design doc)
- `SOLVER-DESIGN.md` — GQ1 mechanism details: witnesses, branch selection, bifurcation (pre-M4 design doc)
- `PROFILE-LIFT-DESIGN.md` — The profile-parameter lift (M10-P): guided replay at the lane scalar
- `ASSEMBLY-DESIGN.md` — Assembly Design — instances, mates, and the document seam
- `CONTACT-DESIGN.md` — Contact census & declared contact (pre-implementation design doc)
- `PROFILES-V2-DESIGN.md` — PROFILES-V2-DESIGN: profiles as programs — the representation switch
- `GUI-DESIGN.md` — GUI / Editor Architecture — Design Document

## Sweep 6 — 2026-09-04: VERBS leaves the tracker

VERBS — the modeling-verb breadth program — closed 2026-09-04 on
`docs/VERBS-EXIT-WALK.md`, ratified by Ev at PR #1793 (merged
`fd45920d5`). Sweep 5's rule. Twenty files: `work/verbs/` whole. A/B band
100–199 stays claimed in `docs/MODEL-AB-LOG.md`. Fourteen live rows were
re-homed to `work/issues/` in the dispositions commit first, each carrying
its own note.

Sweep SHA `c1e7ea19501d9e625dbd0de260d01a53a0384b42`.

    git show c1e7ea19501d9e625dbd0de260d01a53a0384b42:work/verbs/<FILE>
    git show c1e7ea19501d9e625dbd0de260d01a53a0384b42:docs/VERBS-EXIT-WALK.md

Done-state of record: this note and the walk at the SHA above — 28
delivered unit/PR rows (every implementation row of `docs/KERNEL-VERBS.md`),
two measured-and-refused boundaries, ~10 transfers, and the A/B instrument
at close.
