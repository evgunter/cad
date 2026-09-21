# docs ledger — deleted historical documents

This file is the permanent record of documents deleted from `docs/`
and — since sweep 5 — of closed programs' directories deleted from
`work/`. It replaces `docs/archive/`, whose method was to **move** dead
documents aside and index them; the method now is to **delete** them
and record the filenames here. Git is the archive — the repo is
merge-only and never rewrites history (`memories/git-workflow.md`),
so every file named below is still reachable, byte-for-byte, at the
commit named in each sweep's header.

Nothing listed here is normative, and nothing listed here was
normative when it was deleted. The living contract is `docs/DESIGN.md`
plus the companion design docs its table lists.

## Recovering a deleted file

```
git show <sweep-sha>:docs/<NAME>            # print it
git show <sweep-sha>:docs/<NAME> > /tmp/<NAME>   # restore a copy
git log --diff-filter=D -- docs/<NAME>      # find the deleting commit
git log --all --full-history -- docs/<NAME> # ...and if that is empty, this
```

**Check the last one before concluding a document is gone.** A file
this ledger does not name may have been MOVED rather than deleted (the
tracker migration moved every plan and log out of `docs/`, sweep 4), and
`--diff-filter=D` does not see a rename. `git log --follow --stat` from
either path prints the move and whether the content changed with it.
A citation to a path that no longer exists is evidence of nothing until
that has been run: the first reader to follow `docs/PERF-PLAN.md` filed
it as a document deleted without a record, and it had been `work/perf/plan.md`,
byte for byte, since the day it left.

Files listed under the `docs/archive/` group below need that prefix in
the path: `git show <sweep-sha>:docs/archive/<NAME>`. The tracker
directories sweep 5 deleted take their own prefix —
`git show <sweep-sha>:work/<program>` lists one, and
`git show <sweep-sha>:work/<program>/log.md` prints a file from it.
Every deleted path is still greppable across history with
`git log -S<string> --all`.

## The rule this sweep applied

**Deleted** — an artifact whose subject is finished and whose content
is carried somewhere that still lives:

- **Per-unit binding specs for merged units.** A spec binds an
  implementer for the length of one unit. Once the unit merges, the
  merged code is the artifact and the PR description is its
  documentation; the unit's merge record lives in its program log and
  in `docs/MODEL-AB-LOG.md`. The spec binds nothing further.
- **Plans and logs of closed milestones**, where the milestone's
  exit walk survives as the done-state of record.

**Kept** — anything a live pointer still needs:

- The **exit walks** for M5–M8. `docs/DESIGN.md` names these as each
  milestone's done-state of record. (M4's went — see the M4 note below.)
- The **live programs' plans and logs** — M9 (`M9-PLAN.md` / `M9-LOG.md`),
  LIB (`LIB-LOG.md`), ASM (`ASM-PLAN.md` / `ASM-LOG.md`) — and
  `M8-PLAN.md` / `M8-LOG.md`, which `DESIGN.md` still cites as the M8
  roadmap bullet's plan and narrative. *(Superseded by sweep 3: M8, M9
  and ASM have all closed since, and their plans and logs went with it.)*
- **Specs of units not yet merged**: `M9-3-SPEC.md` (DRAFT, mid-unit)
  and `TESS-SPLIT-SPEC.md` (spec written, dispatch queued). *(Both
  merged and were swept in sweep 3.)*
- All ratified design docs, the measurement and reference records
  (`K-REPORT.md`, `PERF-*`, `TESS-BUDGET.md`, `GUIDE.md`,
  `MODEL-AB-LOG.md`, the smell-scan logs, …).

### A note on inbound references

Some surviving documents and a number of source comments cite files
deleted here — mostly "see also" pointers in append-only logs, which
state what was true when written and are not edited in place. Those
citations are not broken: the filename plus this ledger's recovery
recipe resolves any of them. No file was deleted that a *live* pointer
depends on for its content.

---

## Sweep 1 — 2026-08-20

**Recovery SHA: `87e565b11f3d50b3761b6d6361191be872e42bde`** — the commit
immediately before the deletion, and `main`'s tip at the time, so it is
permanently reachable and needs no tag to survive. 109 files.


### Closed-milestone plans and logs (4 files)

M6 closed 2026-08-08 and M7 closed 2026-08-09; each milestone's
done-state of record is its exit walk, which is kept
(`docs/M6-EXIT-WALK.md`, `docs/M7-EXIT-WALK.md`). The plan's criteria
are quoted verbatim inside the exit walk, so the walk is self-contained.

- `M6-LOG.md` — M6 log
- `M6-PLAN.md` — M6 — main-path completions (plan)
- `M7-LOG.md` — M7 log — orchestrator record
- `M7-PLAN.md` — M7 — STEP import as adoption (plan)

### Per-unit binding specs — kernel milestones (M6–M9) (14 files)

All merged. M6/M7 units land in their milestone logs; M8-4 (#499) and
M8-F67 (#502) in `docs/M8-LOG.md`; M9-1 (#552), M9-2 (#527/#564) and
M9-D1 (#530) in `docs/M9-LOG.md`. `M9-3-SPEC.md` is NOT here — that
unit is mid-flight.

- `M6-5-SPEC.md` — M6-5 spec — edge-selection fillet vocabulary (binding)
- `M6-6-SPEC.md` — M6-6 spec — the curved sense-flip tier gate (binding)
- `M7-1-SPEC.md` — M7-1 spec — step-import skeleton + own-corpus round-trip (binding)
- `M7-2-SPEC.md` — M7-2 spec — the FreeCAD-authored foreign corpus (binding)
- `M7-3-SPEC.md` — M7-3 spec — NURBS-face import (binding)
- `M7-4-SPEC.md` — M7-4 spec — the wild corpus (binding)
- `M7-5-SPEC.md` — M7-5 — Band-seam re-mint (seamless periodic bands)
- `M7-6-SPEC.md` — M7-6 — Stage-1 NURBS surface recognition (always-promote)
- `M7-8-SPEC.md` — M7-8 — Plane × NURBS intersection certification (declare-and-check)
- `M8-4-SPEC.md` — M8-4 — the `nurbs_iso_derive` Intersection arm (boundary-iso mint)
- `M8-F67-SPEC.md` — M8-F67 — the #214 F6/F7 typed-margin fold-in
- `M9-1-SPEC.md` — M9-1 — contact vocabulary: records + declaration classes (spec)
- `M9-2-SPEC.md` — M9-2 — the A5 at-rest door (spec)
- `M9-D1-SPEC.md` — M9-D1 — the revolve pole resolution (spec)

### Per-unit binding specs — LIB (usable-as-a-library) (29 files)

All merged. Every unit has a merge entry in `docs/LIB-LOG.md` and a
row in `docs/MODEL-AB-LOG.md` carrying its date, PR, and A/B review
record. The LIBRARY-DESIGN §L5 ladder completed 2026-08-10; the units
after it are register items, all closed.

- `LIB-DOORS-SPEC.md` — LIB-DOORS spec — curated-surface gaps F1-F6 (binding)
- `LIB-G1-SPEC.md` — LIB-G1 spec — PATHS vocabulary growth, cheap set (binding)
- `LIB-G14-SPEC.md` — LIB-G14 spec — the split-naming walls, executed (binding)
- `LIB-G2-SPEC.md` — LIB-G2 spec — arc-carrier fillet modes for the PATHS algebra (binding)
- `LIB-LBRET-SPEC.md` — LIB-LBRET spec — LoopBuilder retirement (#377, ratified #386): the §2b route-3 door + rocker migration (binding)
- `LIB-ONARC-SPEC.md` — LIB-ONARC spec — the OnArc dissolution (§2c amendment 2026-08-16; binding)
- `LIB-PLACEDUNION-SPEC.md` — LIB-PLACEDUNION spec — the ratified A′ group boolean: a Pattern that fuses (binding)
- `LIB-PYBUNDLE-SPEC.md` — LIB-PYBUNDLE spec — the bindings-parity tail: G4/G6/G7/G9 + riders (binding)
- `LIB-PYG1-SPEC.md` — LIB-PYG1 spec — audit gap G1: arcs and circles in profiles from Python (binding)
- `LIB-PYG23A-SPEC.md` — LIB-PYG23A spec — audit G3 (non-xy sketch planes) + G2's loft half (binding)
- `LIB-PYG5-SPEC.md` — LIB-PYG5 spec — audit gap G5 + register R3: the detect/declare protocol from Python, and the refusal-menu wiring (binding)
- `LIB-PYPU-SPEC.md` — LIB-PYPU spec — PlacedUnion's Python/audit slice (binding)
- `LIB-PYSEL-SPEC.md` — LIB-PYSEL spec — audit gap G13: the selector surface from Python (binding)
- `LIB-RESPELL-SPEC.md` — LIB-RESPELL spec — the §2c fillet family, implemented (binding)
- `LIB-RETTAIL-SPEC.md` — LIB-RETTAIL spec — the retirement's tail: ProfileLoop demotion, bowtie re-home, shim deletion (binding)
- `LIB-RTABLE-SPEC.md` — LIB-RTABLE spec — the four-projection transition table (RESPELL-TABLE; binding)
- `LIB-SEAL-SPEC.md` — LIB-SEAL spec — ProfileLoop seals: private fields + read accessors (ruled by Ev in-chat 2026-08-16; binding)
- `LIB-SEL1-SPEC.md` — LIB-SEL1 spec — geometric selectors PR-1 (binding)
- `LIB-SWITCH-SPEC.md` — LIB-SWITCH spec — profiles-as-programs v2: the schema-v4 representation switch
- `LIB-U1-SPEC.md` — LIB-U1 spec — the façade crate + prelude (binding)
- `LIB-U10-SPEC.md` — LIB-U10 spec — docs, tutorials, corpus-as-examples (binding)
- `LIB-U2-SPEC.md` — LIB-U2 spec — PATHS algebra implementation + demo rework (binding)
- `LIB-U3-SPEC.md` — LIB-U3 spec — SectionSegments retirement (binding)
- `LIB-U4A-DOOR-SPEC.md` — LIB-U4A-DOOR spec — the chain→curve composition door in geom-curves (binding)
- `LIB-U4B-SPEC.md` — LIB-U4B spec — the frame-constructor family in geom-core (constructors only; binding)
- `LIB-U5-SPEC.md` — LIB-U5 spec — read-back/interrogation doors (binding)
- `LIB-U7-SPEC.md` — LIB-U7 spec — structural selectors + name doors (binding)
- `LIB-U8A-SPEC.md` — LIB-U8a spec — quantities, units, formatter, and the checking parser (binding)
- `LIB-U9S-SPEC.md` — LIB-U9S spec — Python bindings scaffold (binding)

### Per-unit binding specs — ASM (assemblies) (10 files)

All merged; the program's ratified v1 scope (R1+R2) is code-complete
as of ASM-R2b (#591). Merge entries are in `docs/ASM-LOG.md`.
`ASM-R2-SPEC-DRAFT.md` never became binding — it was superseded before
dispatch by the two specs that split it, ASM-R2a and ASM-R2b.

- `ASM-1-SPEC.md` — ASM-1 — document identity + content pins (binding spec)
- `ASM-2A-SPEC.md` — ASM-2A — `InstantiatePart`, single-solid parts (binding spec)
- `ASM-2B-SPEC.md` — ASM-2B — multi-solid referenced products (binding spec)
- `ASM-2K-SPEC.md` — ASM-2K — the multi-solid instancing kernel door (binding spec)
- `ASM-4-SPEC.md` — ASM-4 — split and inline (binding spec; R1's closing unit)
- `ASM-R2-SPEC-DRAFT.md` — ASM-R2 — mates, constructively (SPEC DRAFT — not yet binding)
- `ASM-R2A-SPEC.md` — ASM-R2a — the mate solve (binding spec)
- `ASM-R2B-SPEC.md` — ASM-R2b — declaration minting + the assembly at-rest gate (binding spec)
- `ASM-ROOTS-SPEC.md` — ASM-ROOTS — explicit product roots (binding spec)
- `ASM-UPD-SPEC.md` — ASM-UPD — the pin-update door (binding spec)

### Cross-program unit specs (2 files)

MESH-PROBEGATE executed as #579, and its subject no longer exists:
smell-scan S30 (#709) deleted the `mesh::probe_stats` module and the
`probe-stats` feature outright, which the spec's own header records.
TESS-SPAN merged as #594. The sibling TESS-SPLIT spec is kept — that
unit is queued, not done.

- `MESH-PROBEGATE-SPEC.md` — MESH-PROBEGATE — gate probe_stats at the module boundary (binding spec)
- `TESS-SPAN-SPEC.md` — TESS-SPAN — per-cell NURBS sizing in the shipped lane (binding spec)

### `docs/archive/` — the 2026-08-05 archive, retired in full (50 files)

The archive directory was created 2026-08-05 (method ratified by Ev:
`docs/archive/` + an index + the git tag `archive/2026-08-05`) to hold
M0–M6 historical milestone documents. Its index recorded, per file,
what superseded it: the milestone conventions ratified into
`DESIGN.md`, the K telemetry story continuing in `K-REPORT.md`, the
deferred-quadratic record in `PERF-PLAN.md`, `M5-EXIT-WALK.md` as M5's
done-state, and `M6-BOUNDARY.md`'s ruling paraphrased into `DESIGN.md`'s
M6/M7/M8 roadmap bullets. The per-unit specs it held were already
labelled "superseded by merged code + PR descriptions".

Those files are now deleted rather than parked, `INDEX.md` is replaced
by this ledger, and **`docs/archive/` no longer exists.**

- `INDEX.md` — docs/archive — index
- `M0-LOG.md` — M0 Implementation Log
- `M0-PLAN.md` — M0 Work Order — **COMPLETE (2026-07-16)**
- `M1-LOG.md` — M1 Implementation Log
- `M1-PLAN.md` — M1 Work Order — **COMPLETE (2026-07-16)**
- `M2-LOG.md` — M2 Implementation Log
- `M2-PLAN.md` — M2 Work Order — Analytic Geometry, Extrude/Revolve, Tessellation, STL
- `M3-LOG.md` — M3 Implementation Log
- `M3-PLAN.md` — M3 Work Order — Splitting, Booleans, Cross-Shell Surgery
- `M3-PR6A-SPEC.md` — M3 PR 6(a) binding spec — tier-3′ validator + touching corpus
- `M4-PLAN.md` — M4 work order: the parametric model layer
- `M4-PR1-SPEC.md` — M4 PR 1 binding spec — recipe substrate + editor-core birth
- `M4-PR2-SPEC.md` — M4 PR 2 binding spec — the evaluation service
- `M4-PR3-SPEC.md` — M4 PR 3 binding spec — naming part 1: RolePath, name tables, the CI invariant
- `M4-PR4-SPEC.md` — M4 PR 4 binding spec — Naming part 2: resolution + the diff engine
- `M4-PR5-SPEC.md` — M4 PR 5 binding spec — GeomSource + bit-identity retirement + Declare threading
- `M4-PR6-SPEC.md` — M4 PR 6 binding spec — persistence (schema v1)
- `M4-PR8-SPEC.md` — M4 PR 8 binding spec — Band 4 corpus, K-telemetry + large-K lint, exit sweep
- `M5-LOG.md` — M5 orchestrator log
- `M5-PLAN.md` — M5 work order: curved geometry (NURBS depth, SSI, fillets)
- `M5-PR1-SPEC.md` — M5 PR 1 spec (binding): interval-transcendentals adoption
- `M5-PR10-SPEC.md` — M5 PR 10 — sweeps/lofts as definitional feature nodes; schema v2 (binding spec)
- `M5-PR11-SPEC.md` — M5 PR 11 — curved tessellation + certified mass properties (binding spec)
- `M5-PR12-SPEC.md` — M5 PR 12 — fillets: the validity battery + analytic blends + the die (binding spec)
- `M5-PR13-SPEC.md` — M5 PR 13 — curved STEP subset: conics + NURBS entities (binding spec)
- `M5-PR14-SPEC.md` — M5 PR 14 — the exit sweep (binding spec)
- `M5-PR2-SPEC.md` — M5 PR 2 spec (binding): the C9 interval ring + hull-bound primitives
- `M5-PR3-SPEC.md` — M5 PR 3 spec (binding): NURBS substrate part 1
- `M5-PR4-SPEC.md` — M5 PR 4 spec (binding): NURBS substrate part 2 — projection, fitting, LSQ
- `M5-PR5-SPEC.md` — M5 PR 5 — `Ellipse` carrier + the C5 dispatch table (binding spec)
- `M5-PR6-SPEC.md` — M5 PR 6 — pcurves as per-half-edge certified caches (binding spec)
- `M5-PR7-SPEC.md` — M5 PR 7 — SSI: march-then-certify + in-op exhaustiveness (binding spec)
- `M5-PR7B-SPEC.md` — M5 PR 7b — tensor-product Bernstein composition; plane×NURBS retirement (binding spec)
- `M5-PR8-SPEC.md` — M5 PR 8 spec (binding): the BVH crate + planar boolean-sweep wiring
- `M5-PR9-SPEC.md` — M5 PR 9 — curved booleans end-to-end + the tangency regime (binding spec)
- `M5-PR9C-SPEC.md` — M5 PR 9c — the banked curved-boolean completions (binding spec, DRAFT until dispatch)
- `M5-S1-SPEC.md` — M5 S1 — the REST-contact join lane (binding spec)
- `M5-S10-SPEC.md` — M5 S10 — face orientation sense (binding spec)
- `M5-S11-SPEC.md` — M5 S11 — concave arc walls mint sense:false (binding spec)
- `M5-S13-SPEC.md` — M5 S13 — the die-pips enablers: containment-fallback re-cut + the plane×sphere germ arm (binding spec)
- `M5-S2-SPEC.md` — M5 S2 — arc-leg fillet sugar (binding spec)
- `M5-S6-SPEC.md` — M5 S6 — two-tolerance message-unification sweep (binding spec)
- `M5-S7-SPEC.md` — M5 S7 — CI/docs hygiene sweep (binding spec; Ev-directed 2026-07-30)
- `M5-S8-SPEC.md` — M5 S8 — fillet branch selection: nearest-the-authored-corner (binding spec)
- `M5-S9-SPEC.md` — M5 S9 — chord_spec arc-side repair: azimuth-window containment (binding spec)
- `M6-2-SPEC.md` — M6-2 spec — the SSI generic-T lift (binding)
- `M6-3-SPEC.md` — M6-3 spec — loft/sweep body assembly (binding)
- `M6-BOUNDARY.md` — The M5→M6 boundary: banked units + three roadmap questions (design conversation)
- `M4-LOG.md` — M4 Implementation Log
- `M4-EXIT-WALK.md` — M4 exit walk (8c) — criteria vs evidence

#### The M4 record — why it went, and what DESIGN.md lost

These two were held back from the first pass because `DESIGN.md`'s M4
bullet named them as the home of the M4 shipped-unit list and the
**F1–F8 fork-outcome record**. That record had been cut out of
`DESIGN.md` on 2026-08-05 and pasted into `M4-LOG.md`'s tail as an
appendix — a maintained, ratified record living inside an append-only
ephemeral log, which is the wrong genre for it. It had even kept
accruing M5-era updates there (the schema v2 clean break at M5 PR 10,
`Loft`/`Sweep` joining F4's vocabulary, F6's OCC blind spot re-measured
at M5 PR 13), and its F5 entry still cross-referenced "the M4 roadmap
entry above" — a pointer back into `DESIGN.md` that dangled the moment
it was relocated.

Ruled by Ev, 2026-08-20: **delete the passage rather than re-home the
record.** Two edits to `docs/DESIGN.md` accompany this sweep:

- the `### M4 fork outcomes (F1–F8)` section (10 lines) was removed. It
  held no outcomes — only a note that the record had been relocated,
  plus its own certification that *"still-live outcomes are stated
  where they bind: the dimension lattice and node vocabulary in the M4
  roadmap entry and D8, persistence schema rules in D6.3/F3's
  clean-break record, the STEP posture in D7 and the crate table."*
  That is the warrant: nothing binding was carried by the section or by
  the appendix it pointed at.
- the M4 roadmap bullet's parenthetical was trimmed to
  *(Complete 2026-07-27.)*, dropping only the sentence naming the two
  archived files. **The standing bit-identity-retirement paragraph that
  follows it is untouched** — it is marked "stated here because it
  still binds" and remains in force.

The F1–F8 outcome text itself is recoverable at the sweep SHA:
`git show 87e565b:docs/archive/M4-LOG.md` (appendix at the tail).

---

## Sweep 2 — 2026-08-20: the second smell scan is folded into the first

**`docs/SMELL-SCAN-2-2026-08.md`** — deleted, and its entire content
carried into **`docs/SMELL-SCAN-2026-08.md`**: findings S59–S116 with
their tiers, its §A as **§A2**, its §B as **§B2**, and its process
observations as **C18–C25** inside §C.

Recover the file as it stood at the merge with
`git show <this sweep's SHA>^:docs/SMELL-SCAN-2-2026-08.md`, or read it
in place — nothing was dropped, and the only edits were the ones the
merge itself required.

**Why it existed and why it does not.** It was written as a separate
file so that a scan landing mid-wave would not collide with the fix
tracks editing the first document. That collision was real. Separation
was the wrong fix for it: the two documents share **one ID space** by
design, so a reader holding an `S`-number could not tell which file to
open, and — the concrete cost — **both files stated the same wrong
number about §C**. The second scan said its process observations
continued *"at C15"*; the first scan's forward pointer said the same;
§C already ran to **C17**. Neither author could see it from inside
their own file. One register, one ID space.

**What the merge changed, exhaustively:**

- the second scan's `# Tier N` headings gained a `Second scan · ` prefix,
  so their anchors no longer collide with the first scan's;
- its `# §A` and `# §B` became `## §A2` and `## §B2`, being about that
  scan's findings rather than this document's;
- its `## C15`–`## C22` became `## C18`–`## C25`, and the sentence
  claiming §C ran to C14 was replaced with one that says what happened;
- its `## Contents` list was dropped into this document's own Contents.

Findings, verdicts (all still blank), citations and prose are otherwise
byte-for-byte as merged.


---

## Sweep 3 — 2026-08-28: the merged units' specs, and three closed programs

Sweep SHA: `4eda8abec43166ec4c027bb401a8f2cf9f3f7a9f` — every path below is recoverable at
`git show 4eda8abec43166ec4c027bb401a8f2cf9f3f7a9f:docs/<NAME>`.

Same rule as sweep 1, applied to what has closed since it ran: a
per-unit binding spec goes once its unit merges, and a closed
program's plan and log go once its exit walk is ratified as the
done-state of record.

### Per-unit specs, unit merged

- `CENSUS-G2-SPEC.md` — census gap 2 (#1080)
- `GUI-0-SPEC.md` — GUI-0, camera/viewport (#1094)
- `GUI-1-SPEC.md` — GUI-1, headless session layer (#1093)
- `GUI-2-SPEC.md` — GUI-2, click-to-select (#1106)
- `GUI-3-SPEC.md` — GUI-3, feature tree and property panel (#1101)
- `GUI-4-SPEC.md` — GUI-4, free-move, hiding, the mate tool (#1113)
- `M9-3-SPEC.md` — M9-3, the C7 join lane (#967 + #971)
- `M9-5-SPEC.md` — M9-5, the lily rebuild and the two-peg plate cell (#1037)
- `PCURVE-P1A-SPEC.md` — PCURVE P-1a (#1073)
- `TESS-SPLIT-SPEC.md` — the split-schedule unit (#951)
- `VERBS-ARMS-SPEC.md` — the ARMS cut, all three sub-units (#932, #962, #1028)
- `VERBS-CHAMFER-SPEC.md` — chamfer (#920)
- `VERBS-CYLCYL-SPEC.md` — cylinder×cylinder (#1021 + #1044)
- `VERBS-GATE-SPEC.md` — the KIND gate (#1001)
- `VERBS-LILYWELD-SPEC.md` — lily weld; PR-1 (#1109) and PR-2 (#1127,
  which closed as a MEASUREMENT — the two pins are the unit's closing record)
- `VERBS-OFFA-SPEC.md` — offset A (#994)
- `VERBS-OFFB-SPEC.md` — offset B (#1003)
- `VERBS-OFFC-SPEC.md` — offset C, `Surface::Approx` (#1012)
- `VERBS-OFFD-SPEC.md` — offset D, shell (#1043 + #1048)
- `VERBS-PIERCE-SPEC.md` — pierce (#1068)
- `VERBS-RIM-SPEC.md` — rim (#910)
- `VERBS-RING-SPEC.md` — ring (#933)
- `VERBS-TEAPOT-SPEC.md` — teapot (#1078)

### Plans and logs of closed programs

- `M8-PLAN.md`, `M8-LOG.md` — M8 closed 2026-08-15; done-state of
  record `M8-EXIT-WALK.md`. `DESIGN.md`'s M8 roadmap bullet, which
  sweep 1 named as the reason to keep them, was trimmed to its
  exit-walk pointer in this sweep.
- `M9-PLAN.md`, `M9-LOG.md` — M9 closed 2026-08-27; done-state of
  record `M9-EXIT-WALK.md`, which quotes the plan's criteria verbatim.
- `ASM-PLAN.md`, `ASM-LOG.md` — ASM closed at v1 scope 2026-08-23;
  done-state of record `ASM-EXIT-WALK.md`, which quotes the plan's
  exit shape verbatim. `DESIGN.md` and `ASSEMBLY-DESIGN.md` carried
  four live pointers at these two; all four were re-pointed at the
  exit walk in this sweep.

### Kept, and why

- `GUI-PLAN.md` / `GUI-LOG.md`, though the v1 GUI program closed
  2026-08-28. Two file-specific reasons: `GUI-EXIT-WALK.md` is the one
  walk that PARAPHRASES its plan's criteria rather than quoting them,
  so deleting the plan loses the criteria text; and `GUI-LOG.md` is
  still being appended to by post-close maintenance.
- `PCURVE-P1B-SPEC.md` (dispatched, unmerged — P-1b is next),
  `VERBS-SHELLFIX-SPEC.md` (PR-1 merged, PR-2a open as #1126, PR-2b
  unbuilt), `PARAM-LINT-SPEC.md` (DRAFT, never dispatched). All three
  still bind.
- `LIB-LOG.md`, `VERBS-PLAN.md` / `VERBS-LOG.md`, `PCURVE-PLAN.md` /
  `PCURVE-LOG.md` — live programs.
- `GENERICS-BUILD-COST.md`, `LOCAL-BUILD-PERF.md` — measurement
  records that live CI and manifest comments cite as the REASON for a
  current configuration, not as history.
- `REVIEW-STYLE-DISPATCH.md`, `GQ6-RESURVEY.md`,
  `WILD-CORPUS-LICENSES.md`, `CI-MINUTES-2026-08.md`,
  `Q8-SUBSTRATE-2026-08-21.md` — each still applied as a standing
  standard or quoted as normative from live code, CI, or a design doc.

### Inbound references

As in sweep 1, append-only logs (`VERBS-LOG.md`, `GUI-LOG.md`,
`LIB-LOG.md`, `MODEL-AB-LOG.md`, the smell-scan logs), source
comments, and completed rows in live plans still name deleted files.
Those are not broken: the filename plus the recovery recipe at the top
of this document resolves any of them. One source comment was edited
rather than left, because its tense made a live claim: `crates/mesh/
src/sizing.rs` said `TESS-SPLIT-SPEC.md` "binds its execution", present
tense, of a unit that had merged.

## Sweep 4 — 2026-09-03: the work tracker replaces the register and the survey

The tracker under `work/` (contract: `work/README.md`) is now the one
home of live work, and two documents whose content it carries are
deleted. Recover either with `git show <this sweep's SHA>^:docs/<NAME>`.

- **`SMELL-SCAN-2026-08.md`** — the structural findings register and
  its Tracks K–X schedule. Every live row is an item file
  `work/code-quality/<ROWID>.md` (109 rows, the row id kept as the file
  name), every Ev-only decision a `ruling` item there, every live
  unrowed finding an `issue` item there, and the four ordering rules,
  the partition rules, the territories table and the seams are
  `work/code-quality/plan.md`. §C's process observations are
  `work/code-quality/process-observations.md` verbatim. The census
  that reconciles all 94 finding headings against the tree is
  `work/code-quality/logs/migration-census-2026-09-03.md`; nothing
  was dropped.
  **Re-aimed at sweep 11 (2026-09-11), because that paragraph's three
  live pointers all went that day**: the rows are on the eleven programs
  of the 2026-09-11 cut and no longer in one directory
  (`docs/WORK-TRACKS-2026-09.md` addendum 3); the rules, the process
  observations and the migration census went to the archive with the
  directory and are recoverable at sweep 11's SHA. **The register's
  numbering scheme is retired, not relocated** — see sweep 11's
  amendment. One source defect is carried as a flagged
  reconstruction: partition rule 4's opening sentence was already
  missing from the document (its text began mid-sentence), and the
  plan states it as "A style review runs on every unit against …".
- **`WORK-STREAMS-2026-08.md`** — the 2026-08-29 stream cut. Every
  stream it proposed graduated to a program, and each program's
  `work/<program>/program.md` now carries the charter and territory
  the cut assigned it. Plans that cite the cut as their charter keep
  the citation; it resolves here.

### Moved, not deleted

Every `docs/<NAME>-PLAN.md` / `docs/<NAME>-LOG.md` pair is now
`work/<program>/plan.md` / `log.md` (git rename history intact). The
nine `SMELL-*-LOG.md` track logs were under `work/code-quality/logs/`
and **left the tree at sweep 11** (2026-09-11), recoverable at the SHA
that sweep names; they are closed tracks' execution records, and the one
standing rule they were cited for — *the fix mints a fresh instance of
the defect it closes* — is now a bullet in
`docs/prompts/reviewer-style-lane.md` §1, where every reviewer reads it.
`MODEL-AB-LOG.md` stays in `docs/` as the
experiment log it is. `scripts/work.py lint` refuses a plan or log
reappearing in `docs/`.

**The moves, by name**, because a class rule does not answer a
by-name lookup and the one document this ledger exists to serve is a
reader holding a stale citation. Every pair below moved in
`4916f90cfc5cd45c0092b9464fd1fed604f93140` (2026-09-03, PR #1619) or
its siblings in the same migration, each with **no content change** —
`git log --follow -- <new path>` walks straight through the rename:

| was | is now |
| --- | --- |
| `docs/PERF-PLAN.md` | `work/perf/plan.md` |
| `docs/<NAME>-PLAN.md` (every other program) | `work/<program>/plan.md` |
| `docs/<NAME>-LOG.md` (every program) | `work/<program>/log.md` |
| `docs/SMELL-{C,E,F,G,H,I,KPW,T,UV}-LOG.md` | `work/code-quality/logs/` |

`PERF-PLAN.md` is named in its own row because it is the one readers
have followed and failed to find. Measured 2026-09-11: **32 tracked
files mention it, and five cite it by the dead PATH** — the other 27
name the DOCUMENT, which still carries that name as its own title at
`work/perf/plan.md` and needs nothing done to it. Repointing the five,
and deciding whether a document keeps a name its path no longer
carries, is `work/meta/perf-plan-citations-name-a-path-that-moved.md`.
Two further mentions in THIS file (Sweep 1's archive note, DOCM-5's
per-merge record) are left as written: they record what those documents
said at the time.

## Sweep 5 — 2026-09-03: the five closed programs leave the tracker

Sweep SHA: `f955ddc75cda454a268f9214d2a753ae1a9bbd0f` — `main`'s tip
immediately before the deletion, so every path below is recoverable at
`git show f955ddc75cda454a268f9214d2a753ae1a9bbd0f:work/<program>/<FILE>`.

**A new rule, and it is broader than the earlier sweeps'.** `work/` tracks
work still to be done, not work that has been done (Ev, 2026-09-03), so a
closed program's directory leaves the tracker whole — `program.md` with
`plan.md` and `log.md`, not just the narrative pair sweeps 1 and 3 took.
This supersedes sweep 3's file-specific decision to keep the GUI plan and
log. Fifteen files, five programs, every one of them `status: closed` with
no live items:

| program | title | closed | done-state of record |
| --- | --- | --- | --- |
| `blend` | S-BLEND — fillet/chamfer completion | 2026-08-31 | `docs/S-BLEND-EXIT-WALK.md` (criteria quoted verbatim from the plan) |
| `gauth` | GAUTH — part authoring in the GUI | 2026-08-31 | Ev's in-chat ruling that no exit walk is needed — see the honesty note below |
| `gui` | GUI v1 | 2026-08-28 | `docs/GUI-EXIT-WALK.md` (paraphrases the plan's criteria — see below) |
| `pcurve` | PCURVE — edge-description unification | 2026-08-29 | `docs/PCURVE-EXIT-WALK.md` (criterion rows quoted verbatim) |
| `qa` | S-QA — gates that lie | 2026-08-31 | `docs/S-QA-EXIT-WALK.md` (criteria quoted verbatim) |

### Two honesty notes — content that now lives only in git

Sweeps 1 and 3 deleted only what a living document still carried. Two of
these five do not meet that bar, and went anyway on the rule above:

- **`gauth` had no exit walk.** Ev ruled one unnecessary (2026-08-31), and
  the closing entry of `work/gauth/log.md` was itself the program's
  done-state of record: the five merged units (#1375, #1381, #1376, #1397,
  #1407), their A/B ordinals 900–904, and the eight residue pointers. The
  residue survives on its own issues — #1374, #1379, #1384, #1385, #1387,
  #1394, #1395 and the #1386 conversation — and the ordinals survive in
  `docs/MODEL-AB-LOG.md`. The narrative does not: recover it at the sweep
  SHA.
- **`gui`'s exit walk paraphrases its plan.** `GUI-EXIT-WALK.md` checks the
  program against `GUI-PLAN.md` in the walk's own words rather than quoting
  the criteria, which is exactly why sweep 3 kept the plan. The criteria
  text is now recoverable only at the sweep SHA; the walk remains the
  done-state of record, and `docs/GUI-DESIGN.md` carries the v1 shape.

### Nothing else moved with them

- **A/B bands** (`ab_band` on each deleted `program.md`) were never the
  tracker's record: they are claimed in `docs/MODEL-AB-LOG.md`'s
  ordinal-bands section, which still names GAUTH's 900–999 and the rest.
- **`work/STATUS.md`** is regenerated by CI on main; the five closed rows
  and their empty per-program slates drop out on the next render. That is
  the board agreeing with the tracker, not a loss.
- **Residue** is unaffected. No item file anywhere in `work/` named a
  deleted program in `parent`, `blocked_on`, `rides_with` or `refs`;
  everything these programs left open had already been re-homed to a live
  program or to `work/issues/`. `scripts/work.py lint` is green.

### Inbound references

The old `docs/<NAME>-PLAN.md` / `-LOG.md` citations in CI workflows, source
comments and append-only logs were already historical after sweep 4's move;
they resolve here as before. Three *live, present-tense* claims cited the
tracker paths and were re-pointed in this sweep rather than left:

- `work/code-quality/corner-config-tag-all-concave-trihedron.md` and
  `work/issues/fillet-nonpositive-radius-false-fact-refusal.md` each argued
  their home by naming `work/blend/` as a closed program; both now name
  S-BLEND and its exit walk.
- `work/cert/unify-edge-descriptions-on-pcurves.md` pointed at
  `work/pcurve/program.md` as PCURVE's done-state of record; it now points
  at `docs/PCURVE-EXIT-WALK.md`, which is what that record actually is.

## Sweep 6 — 2026-09-04: S-MATE leaves the tracker

Sweep SHA: `386e170f` (`main`'s tip immediately before the deletion),
so every path below is recoverable at
`git show 386e170f:work/mate/<FILE>` and
`git show 386e170f:docs/S-MATE-EXIT-WALK.md`.

Sweep 5's rule. The walk rode PR #1528 as PROPOSED and Ev merged it
on 2026-09-01; the ratification it asked for was confirmed in-chat on
2026-09-04 ("if S-MATE's exit walk is merged then it means I approved
it"), and nothing had run the sweep in between. Four files, one
program:

- `work/mate/program.md`, `plan.md`, `log.md` — S-MATE's charter,
  plan and narrative: nine units (ordinals 1300–1308), two in-program
  ratifications (PRs #1440, #1469).
- `work/mate/MATE-EXIT.md` — the ratification ruling, closed by this
  sweep.

Residue was re-homed before the sweep (the log's 2026-09-03 "Seam"
entry): five issues to `work/fix/`, two to `work/docm/`, six to
`work/curved/`; the walk's handoffs ledger names every other
pointer's home. What opens with this sweep: `crates/editor-core/src/mate.rs`,
`mate/*` and `assembly.rs` pass to DOCM's territory per its program
header; `crates/topo/src/census.rs`, `boolean/rest.rs` and
`boolean/carrier_eq.rs` are unowned until a program claims them.

## Sweep 7 — 2026-09-06: S-CERT leaves the tracker

Sweep SHA: `b33ca36ac` (`main`'s tip immediately before the deletion),
so every path below is recoverable at
`git show b33ca36ac:work/cert/<FILE>` and
`git show b33ca36ac:docs/S-CERT-EXIT-WALK.md`.

Sweep 5's rule. The walk rode `[ev]` PR #1924 as PROPOSED and Ev merged
it on 2026-09-06 (`c7b0014de`), which is the ratification by the S-MATE
convention (2026-09-04). It merged without comment, so the five points
the walk left "Open with Ev" stand as walked: row 5 on the digits (146×
tighter, 1.84× above target; the dial decision is PROPS'), row 3 as the
fold live under M10-3's driver, row 10 with Track N empty and Track M
reduced to H5's schedule (two questions still open on it after #1878's
DEFER: PcurveFittedLane's representation, and whether the certified
at-rest doors become the default name), and the residue homes as
executed. Eleven files, one program:

| program | title | closed | done-state of record |
| --- | --- | --- | --- |
| `cert` | S-CERT — certified-enclosure soundness | 2026-09-06 | this entry (the walk: the plan's eleven exit-shape clauses walked verbatim against main `37eaf5b9b` — five MET, six MET-WITH-RECORDED-HONESTY, none CARRIED; the A/B record for ordinals 700–714 in `docs/MODEL-AB-LOG.md`, rows CERT1 … CERT10 and CERTM1 … CERTN3; the v6 tally candidates queued for the blinded coding with `cert/ab-state` as the source of record) |

- `work/cert/program.md`, `plan.md`, `log.md` — the charter (territory
  `geom-brep/src/props/*`, `offset_fit.rs`, `patch_bound.rs`,
  `geom-core/src/*`, `geom/src/*`, `bvh/src/*`; band 700–799), the
  ratified ground with its four rulings and the unit list, and the
  narrative: the defect cluster CERT-1 … CERT-10, then SMELL tracks M
  and N absorbed whole (CERT-M1 … M3, CERT-N1 … N3), fifteen dual
  reviews. The log's last entry ("Residue re-homed", 2026-09-05)
  predates the PROPS-charter re-points on PR #1924; the walk's
  handoffs ledger, not that entry, names the homes that were executed.
- `work/cert/CERT-M3.md`, `CERT-N3.md` — the last two units, closed
  (#1877, #1879).
- `work/cert/C24.md`, `D31.md`, `D98.md`, `D244.md` — the Track M/N rows
  and `work/cert/S235.md` — the exact conic box issue — closed on
  CERT-N3.
- `work/cert/unify-edge-descriptions-on-pcurves.md` — the ruling on
  PCURVE's edge-description question (issue #427), closed.

Residue was re-homed before the sweep, on the walk's own PR (#1924, per
Ev's "as long as all residuals are filed appropriately" and the PROPS
orchestrator's re-points from the ratified PROPS charter) — twenty-four
items, ids unchanged: fourteen to `work/props/` (`rimless-polar-cap…`,
`two-face-sphere-split…`, `props-refusal-cannot-carry…`,
`props-two-eps-vocabularies…`, `quad-face-extent…`,
`purchasable-area-tightness-valve`, `budgetexhausted…`,
`offset-fit-mignitude…`, `patch-bound-offset-fit…`, `refine-dir…`,
`quad2-rational…`, `normalize-overflow…`, `orthonormal-basis…`,
`pole-branch…`); H5 (`parent` cleared), its lane-keeping companion, the
K roster and the TESS-BUDGET finding to `work/code-quality/`;
`symbolic-tier-census` and `param-box-certification…` to `work/m10/`;
`nurbs-net-point-map-helper` to `work/fix/`; `edge-chord-len…` to
`work/bool/`; `loft-seam…` to `work/trim/`; `ssi-chart-speed…` to
`work/curved/`. Nothing to `work/issues/`. The ChartRegionLane ruling
closed (RULED defer, #1878) in `work/code-quality/`. What opens with
this sweep: nothing — the territory passed to PROPS on the same PR
(`geom-brep/src/props/*`, `offset_fit.rs`, `patch_bound.rs`,
`geom-core/src/*` and `geom/src/*` are in `work/props/program.md`'s
`paths`), and `crates/bvh/src/*` was M10's by S-CERT's own keep-out.
The A/B band 700–799 stays claimed in `docs/MODEL-AB-LOG.md`'s
ordinal-bands section as always.

### Inbound references

Append-only logs (`docs/MODEL-AB-LOG.md`, `work/props/log.md`,
`work/topo/log.md`, `work/exch/log.md`) and the dated
`docs/WORK-TRACKS-2026-09.md` keep their `work/cert/` and
`docs/S-CERT-LOG.md` citations; they resolve here as before. Live
pointers were re-pointed in this sweep rather than left: nine to the
moved items at their new paths (`crates/geom-core/src/sym.rs`,
`crates/editor-core/tests/m10_7_r1_census_probe.rs`,
`docs/ERROR-DESIGN.md`, `docs/K-REPORT.md`, `work/m10/M10-7.md` ×3,
`work/code-quality/chart-region-lane-contract.md`,
`work/fix/transform-rigid-refuses-described-nurbs.md`); the thirteen
moved items whose `## Home` section still argued `work/cert/` each gain
a `## Re-homed` section superseding it, in the form the 2026-09-04
re-home sweep used; and the CERT half of
`work/meta/stale-track-t-citations-in-fillet-and-cert.md` is discharged
by `plan.md`'s deletion.

## Sweep 8 — 2026-09-06: SEAT leaves the tracker

Sweep SHA: `326be1bf4` (`main`'s tip immediately before the deletion),
so every path below is recoverable at
`git show 326be1bf4:work/seat/<FILE>`,
`git show 326be1bf4:docs/SEAT-EXIT-WALK.md` and
`git show 326be1bf4:docs/VERB-SEAT-DESIGN.md`.

Sweep 5's rule. The walk rode `[ev]` PR #1997 as PROPOSED and Ev
ratified it in session on 2026-09-06 ("1997 looks good"), merged
`6fe98d312`; the one open text correction it carried, `[ev]` PR #1983
(VERB-SEAT-DESIGN §1 S3 names `carrier_pair_relation` and the lily's
three declarations), was ratified the same way ("1983 looks good too")
and merged before this sweep. Both merged without comment, so the three
points the walk put to Ev stand as walked: the S3 text as corrected on
#1983; SEAT-9's answer to the PR-1904 question (no over-ε geometry was
ever handed back — tier-3 validation re-certified at the run's ε — and
the residual named there, the transform lane's re-certification
against a stored tolerance, is filed for its owner); and the
compound-`Bounds` allowlist entries in `geom-core/src/real.rs` keep
their retroactive-review flag. Fifteen files, one program:

| program | title | closed | done-state of record |
| --- | --- | --- | --- |
| `seat` | SEAT — the verb-seat program | 2026-09-06 | this entry (the walk: twelve units delivered — SEAT-1 #1399, SEAT-2 #1521, SEAT-3 #1531, SEAT-4 #1547, SEAT-DV #1564, SEAT-5 #1581, SEAT-6 #1593, SEAT-7 #1910, SEAT-8 #1950, SEAT-FW #1974, SEAT-DN #1987, SEAT-9 #1995 — the plan's wave cut complete; VERB-SEAT-DESIGN walked clause by clause (§1 S1–S4, §2 V1–V4, §3 P1–P3, VS-Q1–Q6 with VS-Q4 revised at #1870, §5, §6) with every clause executed and its text now beside the code; the A/B record for ordinals 1000–1011 in `docs/MODEL-AB-LOG.md`, rows SEAT1 … SEAT9 with blocks SEAT-B1/B2/B3 concluded and their draws published — twelve FAIR pairs, five tally candidates queued for the blinded coding; the open list dispositioned below) |

- `work/seat/program.md`, `plan.md`, `log.md` — the charter (paths
  `crates/verbs/*`, `topo/src/query.rs`, `topo/src/flush.rs`,
  `editor-core/src/verbs/*`, the names seat's `geompred.rs`/`flush.rs`,
  the `seat*` suites, the design doc; band 1000–1099), the three-wave
  plan (the kernel query seat, one verb vocabulary, lowered parameter
  identity) with its side units, and the narrative from the program's
  opening through every unit's MERGED entry, the three block closes and
  the walk's opening.
- `work/seat/SEAT-6.md` … `SEAT-9.md`, `SEAT-DN.md`, `SEAT-FW.md` — the
  unit items that lived in the tracker (SEAT-1 … SEAT-5 and SEAT-DV
  closed before the tracker migration; their record is their ledger
  rows), each closed with a `## Closed` pointer.
- `work/seat/direction-normalization-two-doors-one-home.md`,
  `flush-detector-widening-to-curved-rungs.md`,
  `parameter-identity-channel-to-boolean.md`,
  `seat6-germ-end-to-end-awaits-seat7.md`,
  `shell-doors-take-tolerance-beside-tol.md` (Ev's ruling (i) on #1904
  recorded in it), `verb-seat-design-s3-names-the-planar-verifier.md`
  — closed, by the units and the two `[ev]` rulings.
- `docs/SEAT-EXIT-WALK.md` — the ratified walk, replaced by this entry.
- `docs/VERB-SEAT-DESIGN.md` — the ratified design (#1388, S3 corrected
  at #1983, VS-Q4 revised at #1870), MOVED beside the code it governs as
  `crates/verbs/README.md` in present tense with every clause id kept;
  DESIGN.md's companion table points there.

Residue was re-homed in this sweep, ids unchanged — three items:
`two-verb-seats-do-not-compose` (the #1345 items (2)/(3) the design
deferred until a replay consumer exists) to `work/issues/`;
`flush-pair-relation-has-no-caller` (S-BOOL's module) to `work/bool/`;
`two-d-director-doors-skip-the-finiteness-question` (the five-member
finiteness class, FIX's `is-finite-length-homed-in-the-query-seat` the
ruling that closes it) to `work/fix/`. Every other SEAT-originated
residue was filed where it belongs at the moment of disclosure
(`work/issues/`: `axis-flavoured-declarations-have-no-channel`,
`node-tag-space-census-blind-to-tags-outside-sentinels`,
`two-public-verb-types-verbs-and-profile`,
`dirty-pr-gets-no-actions-run`,
`approx-surface-tolerance-is-now-always-the-runs-eps`,
`offset-fit-at-tight-eps-refuses-every-curved-nurbs-chart`;
`work/lib/lib-g17-is-parked-on-a-fired-trigger`;
`work/curved/c5a2-ledger-sample-143-collides-with-seatfw`) and does not
move. What opens with this sweep: nothing — `crates/verbs` has no
program and its next change is LIB-G17's `Node::Shell`, whose enabler
(`VerbRecord::Shell`) SEAT-9 put in place. The A/B band 1000–1099
stays claimed in `docs/MODEL-AB-LOG.md`'s ordinal-bands section as
always.

### Inbound references

Append-only logs (`docs/MODEL-AB-LOG.md`, the other programs' `log.md`
files) keep their `work/seat/` and `docs/VERB-SEAT-DESIGN.md`
citations; they resolve here as before. Live pointers were re-pointed
in this sweep: DESIGN.md's companion table row and its programs
sentence; four `work/issues/` items whose `refs` named a SEAT unit id
now name the unit's PR instead. Prose citations of "VERB-SEAT-DESIGN
§n" resolve against `crates/verbs/README.md` by clause id.

## Per-merge deletion — S415's spec (2026-09-15)

Recoverable at `git show bce676c75:docs/S415-SPEC.md` (the PORT
orchestrator's commit that wrote it, which is the last revision where
it stands alone; the unit branch merged that commit and deleted the
file in its own first commit). The rule above; the unit's record is
`work/port/S415.md`, which carries the corrections to the spec's
premises, and the 2026-09-15 entry in `work/port/log.md`.

**Three of its statements are corrected by the unit** and the item
file is the statement of record for each: the printable-ASCII band is
two formats and not three (the spec has this right); `no_minted_id`
does have a kernel enum behind it (right); and the spec's claim that
`declare_err` "carries the arm's fields" where `boundary_edit_err`
does not is **wrong for that arm** — both pass `EditPayload::NONE`
with no inner variant. The spec's `|p.x| >= 2^53` threshold for the
vanishing strut offset is also wrong on the negative side and is not
a threshold on either; `strut_endpoint`'s own docs carry the checked
statement.

- `S415-SPEC.md` — S415, the three boundary residues: one scaffold rule, the Part 21 band disclosed as one rule in two crates, one hand-minted tag derived (#2633)

## Per-merge deletion — TINT-5's spec (2026-09-15)

Recoverable at `git show 8e7bcc02b:docs/TINT-5-SPEC.md` (the fix-pass
head). The second S-TINT spec written after an executed probe, and the
probe is why the unit had a shape at all: it found that the weld this
unit was to spread was `pub(crate)` inside ONE test binary, depending on
a `set_difference` that was also binary-local. A spec written without it
would have told a lane to "apply TINT-1's shape to mesh and topo",
which means copying the weld into two more crates — the defect, minted
by the fix for an instance of it.

**Three things the spec got wrong, all corrected by the lane**, which
is the point of recording it rather than a failing:

- it said editor-core had **four** `*_is_exhaustive`/`*_VARIANTS` pairs;
  there are **eight**, seven in `display_contract.rs` plus
  `m4_pr4_hit.rs`;
- it said mesh's and topo's local predicates *"check nothing of the
  kind"* about field punctuation. **Both did** — `face:`/`note:` in
  mesh, `diag:`/`what:` in topo. That inverts the risk the spec named:
  `&[]` would have been a REGRESSION at two of three sites, not merely
  a weak default. The measurement the spec demanded was still the right
  one to demand; its premise was backwards;
- it did not name the gate boundary a promotion crosses, which is what
  turned the unit's first CI run red and is the finding that
  generalizes past it.

What the spec did not reach at all, and the review did: the weld's
exhaustiveness token was `fn(&E)`, a type that cannot carry the
wildcard-free invariant nine doc comments asserted of it, and a no-op
token passed green. The fix pass closed that with `f6_variants!`.
Recorded in the PR body, in `work/tint/log.md`, and in the row's
`## Closed` section.

- `TINT-5-SPEC.md` — TINT-5, the F6 enum weld's home and three adopters
  (#2694)


## Per-merge deletion — TINT-4's spec (2026-09-15)

Recoverable at `git show 5494b9927:docs/TINT-4-SPEC.md` (the fix-pass
head). **The first S-TINT spec written after an executed probe rather
than before one**, and the probe's value was not the mechanism it
confirmed — it was discovering that two of the four rows the
orchestrator had grouped into a "roster class" were mis-classed, one of
them wrongly filed by the same seat that filed it the same morning. A
spec written from the grouping would have sent a lane to weld two rows
that wanted different mechanisms.

What the spec got right and the unit proved: `roster!`'s ident feeding
three consumers at once, the `--list` re-exec as ground truth, and the
kill-shot measurement named up front (time the self-`--list` on
`editor-core --features interval`, the largest binary in the tree) with
the lane told to stop and report if it came out badly. It came out at
6.6-7.9 ms over 1630 rows, so the design held.

What the spec did NOT anticipate, and what the unit turned out to be
about: the prose column it sanctioned. The spec said the weld holds
names and never prose and required the lane to say so — and then the
first substantive sentence written into that column was a false
citation, in the very entry the unit existed to correct. The fix pass's
class check found three more wrong or misplaced among the remaining six.
The spec's "state what it does not enforce" was carried out faithfully
and was not enough, because the column it disclosed as unchecked was
unchecked in exactly the way it said and was wrong anyway.

The spec also did not foresee the two narrowings the fix pass closed: a
`#[test]` under a nested `mod` passing a guard named
`the_header_roster_names_every_row_in_this_file`, and an assertion in
the macro's own suite that could not fail. Recorded in the PR body and
in `work/tint/r2-m10-6-header-roster-omits-the-suites-heaviest-row.md`'s
`## Closed` section.

- `TINT-4-SPEC.md` — TINT-4, a header roster welded to the rows it
  names (#2687)

## Per-merge deletion — PORT-PYOPTS's spec (2026-09-15)

Recoverable at `git show e3649a523:docs/PORT-PYOPTS-SPEC.md` (the PORT
orchestrator's commit that wrote it; the unit branch merged that commit
and carried the file to its own merge). The rule above; the unit's
record is `work/port/python-cannot-set-options-structs.md` and the two
rows it filed on LIB, plus the 2026-09-15 entry in `work/port/log.md`.

**Two of its statements are corrected by the unit**, and the item file
is the statement of record for each. The spec said the STL doors
"already expose `solid_name`/`header` as kwargs" and that "the visible
work is small" there; in fact both doors defaulted their keyword to the
EMPTY string, which is not either struct's `Default` — so a Python
caller writing no arguments got `solid ` and 80 zero bytes where a Rust
caller got the kernel's part name and producer text. And the spec said
a `NotBound` roster entry's decay was already checked; the decay half
read the stub's DECLARED NAMES, which an options keyword never is, so
an options entry would have stayed green forever. The alphabet is now
per-roster.

- `PORT-PYOPTS-SPEC.md` — PORT `python-cannot-set-options-structs`, the four options doors bound field by field with a destructure anchor per struct (#2678)

## Per-merge deletion — PORT-DOORS-1's spec (2026-09-15)

Recoverable at `git show 58e485ca4:docs/PORT-DOORS-1-SPEC.md` (the PORT
orchestrator's commit that wrote it, the last revision where it stands
alone; the unit branch merged that commit and carried the file to its
own merge). The rule above; the unit's record is
`work/port/PORT-DOORS-1.md` and the two findings it carries, plus the
2026-09-15 entries in `work/port/log.md`.

**Two of its statements are corrected by the unit**, and the item files
are the statement of record for each. The spec read the widening as a
shape change on two existing arms; it is a **vocabulary** change — the
flattened `AssemblyError::Reference` and `NoAtRestRecord` cannot hold a
mixed list and retire into one `Mint` arm, which is why the unit's class
is `H` and not the `M` the spec's own plan row estimated. And the spec
said "the two control assertions" in
`crates/editor-core/tests/msolve5_read_below_a_root.rs` move under
kind-first; **only one does** — a tie among faces is still `Ambiguous`,
so the tied-face row is untouched.

- `PORT-DOORS-1-SPEC.md` — PORT-DOORS-1, one order and one list at the assembly doors: the mint refusals widened to carry every row, kind asked before tie in `resolve_face` (#2635)

## Per-merge deletion — M10's merged-unit specs (2026-09-03)

Recoverable at `git show 08931277cf23c29d35daa41a15a4cecc6495022e:docs/<NAME>`
(the M10-4 unit head, before the state-sync commit that deleted
them). The rule above, applied at M10-4's merge to every M10 spec
whose unit had merged; each unit's record is its row in
`MODEL-AB-LOG.md` and its MERGED entry in `work/m10/log.md`.

- `M10-1-SPEC.md` — M10-1, distributions in the document (#1147)
- `M10-DI-SPEC.md` — M10-DI, the Dual contract implementation (#1154)
- `M10-P-SPEC.md` — M10-P, the profile-parameter lift (#1174)
- `M10-2-SPEC.md` — M10-2, Measure nodes and Assertions (#1213)
- `M10-3-SPEC.md` — M10-3, the E6 subdivision driver (#1231)
- `M10-4-SPEC.md` — M10-4, sensitivities and the stackup (#1627)

## Per-merge deletion — M10-10's spec (2026-09-12)

Recoverable at `git show f2efdfad2:docs/M10-10-SPEC.md` (the M10-10 unit
head, before the state-sync commit that deleted it; its acceptance
sentence "bounded by genuine flips" is corrected by the unit's own
review — the MERGED entry in `work/m10/log.md` is the statement of
record). The rule above; the unit's record is its row in
`MODEL-AB-LOG.md` and that entry.

- `M10-10-SPEC.md` — M10-10, the form-level mechanism: rule D (trig of atan, exact) with rules A/B per node made affordable, with amendment A1 (#2100)

## Per-merge deletion — M10-9's spec (2026-09-06)

Recoverable at `git show 4f8262ad9:docs/M10-9-SPEC.md` (the M10-9 unit
head, before the state-sync commit that deleted it; its §2 names
`line_span` as the dependency-widening class, which the unit's own
fix pass measured false — the MERGED entry in `work/m10/log.md` is
the correction of record). The rule above; the unit's record is its
row in `MODEL-AB-LOG.md` and that entry.

- `M10-9-SPEC.md` — M10-9, the registered-identity door: discharge by provenance (E12's reserve, taken), with amendment A1 (#2048)

## Per-merge deletion — M10-8's spec (2026-09-05)

Recoverable at `git show f5fb7fe92:docs/M10-8-SPEC.md` (the M10-8 unit
head, before the state-sync commit that deleted it). The rule above;
the unit's record is its row in `MODEL-AB-LOG.md` and its MERGED entry
in `work/m10/log.md`.

- `M10-8-SPEC.md` — M10-8, the arc family: the atom algebra measured per mechanism, the constant fold shipped alongside, rule C built and dial-off (#1828)

## Per-merge deletion — M10-7's spec (2026-09-04)

Recoverable at `git show bb3fba8bc:docs/M10-7-SPEC.md` (the M10-7 unit
head, before the state-sync commit that deleted it). The rule above;
the unit's record is its row in `MODEL-AB-LOG.md` and its MERGED entry
in `work/m10/log.md`.

- `M10-7-SPEC.md` — M10-7, parameter-aware certification: the symbolic identity tier (E12) and the extent lever (E3) (#1725)

## Per-merge deletion — M10-6's spec (2026-09-03)

Recoverable at `git show c0b38dadf3b5b7af4f6165ecd8d37ea51b39423b:docs/M10-6-SPEC.md`
(the M10-6 unit head, before the state-sync commit that deleted it).
The rule above; the unit's record is its row in `MODEL-AB-LOG.md`
and its MERGED entry in `work/m10/log.md`.

- `M10-6-SPEC.md` — M10-6, reporting, CI rows, the advisory lanes, the demo (E10/E11) (#1685)

## Per-merge deletion — M10-5's spec (2026-09-03)

Recoverable at `git show f02d2af15f04ef7d24f0b18efba1b1d19dd6af0d:docs/M10-5-SPEC.md`
(the M10-5 unit head, before the state-sync commit that deleted it).
The rule above; the unit's record is its row in `MODEL-AB-LOG.md`
and its MERGED entry in `work/m10/log.md`.

- `M10-5-SPEC.md` — M10-5, clearance and self-intersection (#1638)

## Sweep 6 — 2026-09-03: finished work leaves `docs/`; its design moves beside the code

Sweep SHA: `3ec71b16575c5887bae358331e517d2ad9348404` — `main`'s tip
immediately before the deletion, so every path below is recoverable at
`git show 3ec71b16575c5887bae358331e517d2ad9348404:docs/<NAME>`.
Seventy-three files.

**The rule this sweep adds.** A document written for the implementer
of finished work — an exit walk, a merged unit's spec, a design
conversation whose subject shipped — leaves `docs/`. What a later
reader still needs from a design conversation is rewritten, present
tense and a fraction of the length, as a README beside the code it
governs, keeping the clause ids so `CURVED-DESIGN C3` or
`ASSEMBLY-DESIGN A6` still names one decision; `docs/DESIGN.md`'s
companion table now points at those pages. An exit walk is replaced by
nothing but its row here: **this ledger is the closed program's
done-state of record** (CLAUDE.md, `work/README.md` and
`memories/MEMORY.md` say so since this sweep). Live pointers — DESIGN.md,
source comments, CI workflows, tracker items — were re-pointed at the
new pages or dropped; append-only logs keep their citations, which
resolve here as before.

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
| `S-MATE-EXIT-WALK.md` | 2026-09-04 | PR #1528, merged by Ev 2026-09-01; ratification confirmed in-chat 2026-09-04 | this row; design at `crates/editor-core/ASSEMBLY.md`; sweep 6 below |
| `FILLET-EXIT-WALK.md` | 2026-09-06 | PR #1973, ratified in-chat 2026-09-06 | this row; sweep 7 below; vocabulary and bands at `crates/sweep/README.md` |
| `DOCM-EXIT-WALK.md` | 2026-09-14 | ratified in advance in chat 2026-09-13 ("write it as ready to merge"); merged with sweep 14 | this row; sweep 14 below; design at `crates/editor-core/REFERENCES.md` (DM1–DM6) and `crates/editor-core/IDENTITY.md` (DI1–DI5) |
| `FIX-EXIT-WALK.md` | 2026-09-21 | ratified in advance in chat 2026-09-21 ("please do close fix; most of those were either mis-filed or should've been done as drive-by fixes"); merged with sweep 18 | this row; sweep 18 below; the program claimed A/B band 1700–1799 and never drew an ordinal |
| `S-CERT-EXIT-WALK.md` | 2026-09-06 | PR #1924, merged by Ev 2026-09-06 (the merge is the ratification, per the S-MATE convention) | this row; A/B record ordinals 700–714 in `docs/MODEL-AB-LOG.md`; sweep 7 above |


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

### Kept, and why

Companion docs of programs still open or with unbuilt scope stay in
`docs/`: `ERROR-DESIGN`, `DUAL-DESIGN` (M10), `LIBRARY-DESIGN`,
`RECIPE-DOORS-DESIGN` (LIB; D5 shell waits), `VERB-SEAT-DESIGN` (SEAT),
`KERNEL-VERBS`, `MIRROR-DESIGN`, `DRAFT-DESIGN` (VERBS; mirror and
draft unbuilt), `DISCIPLINES-DESIGN` (WIP), `PCURVE-UNIFY-DESIGN` (P-2
residue), `PATHS-DESIGN` and `SELECT-DESIGN` (edited by live units this
week), `MATE-7-TANGENCY-DESIGN` (ratified this week; the kissing arm
banks on it). Reference and measurement records stay as sweep 3 left
them.
## Sweep 6 — 2026-09-04: VERBS leaves the tracker

Sweep SHA: `c1e7ea19501d9e625dbd0de260d01a53a0384b42` — the closure
dispositions commit, immediately before the deletion, so every path
below is recoverable at
`git show c1e7ea19501d9e625dbd0de260d01a53a0384b42:<PATH>`.

Same rule as sweep 5: `work/` tracks work still to be done, so the
closed program's directory leaves whole. One program, twenty files,
`status: closed` with no live items:

| program | title | closed | done-state of record |
| --- | --- | --- | --- |
| `verbs` | VERBS — the modeling-verb breadth program | 2026-09-04 | `docs/VERBS-EXIT-WALK.md` (ratified by Ev at PR #1793, merged `fd45920d5`; per-register-row disposition tables, the A/B instrument state at close, and the ten-item open list resolved at this sweep) |

What the walk records: 28 delivered unit/PR rows (every implementation
row of `docs/KERNEL-VERBS.md`), 2 measured-and-refused boundaries
standing as done-states (the Steinmetz pinch family's typed refusal;
the circle-profile rim's torus half), ~10 transfers, and the A/B
program instrument at close (14 dual ordinals, 7 tally candidates
pending the blinded coding, the 5.1 era boundary, the sample-number
cascade note).

### Residue — re-homed, not lost

Before deletion, the dispositions commit closed
`coplanar-cap-pair-f7-repair-half-b` (delivered by VERBS-1031B,
PR #1671) and re-homed FOURTEEN live items to `work/issues/`:
twelve open issues (among them `verbs-1031b-assigner-checker-
divergence`, the parked `pinch-carrying-machinery-valence-4`, and
the #1076/#347 register residue) plus `VERBS-C5ARMS` (PR-2
cone×cylinder remains, spec `docs/VERBS-C5ARMS-SPEC.md`) and
`VERBS-CONE` (operand lanes, never cut) converted to issues as
successor-program seeds. Inbound refs re-pointed
(`work/seat/SEAT-6.md`, `work/issues/sphere-flux-arm-refuses-
partial-bands.md`, `work/props/m6-sense-gate-recorded-residuals.md`);
`scripts/work.py lint` green. The A/B band (100–199) stays claimed
in `docs/MODEL-AB-LOG.md`'s ordinal-bands section as always.

## Per-merge deletion — DOCM-4's spec (2026-09-04)

Recoverable at `git show c20bac059531ccfa00cded39f65ef53474f84e0d:docs/DOCM-4-SPEC.md`
(the DOCM-4 unit head, before the state-sync commit that deleted it).
The rule above; the unit's record is its row in `MODEL-AB-LOG.md`
and its MERGED entry in `work/docm/log.md`.

- `DOCM-4-SPEC.md` — DOCM-4, an evaluation carries its document's identity (#1808)

## Per-merge deletion — DOCM-3's spec (2026-09-04)

Recoverable at `git show d87d012149dfcbc917713ba8c18cbf505390040e:docs/DOCM-3-SPEC.md`
(the DOCM-3 unit head, before the state-sync commit that deleted it;
the file carries the stop-clause amendment as its last section). The
rule above; the unit's record is its row in `MODEL-AB-LOG.md` and its
MERGED entry in `work/docm/log.md`.

- `DOCM-3-SPEC.md` — DOCM-3, `Node::Union`, `DocEdit::SetMembers`, pairwise-distinct inputs (#1803)

## Per-merge deletion — DOCM-1's spec (2026-09-04)

Recoverable at `git show 17bb8fb18b994b96202d72472864140396b50199:docs/DOCM-1-SPEC.md`
(the DOCM-1 unit head, before the state-sync commit that deleted it;
the file carries the stop-clause amendment as its last section). The
rule above; the unit's record is its row in `MODEL-AB-LOG.md` and its
MERGED entry in `work/docm/log.md`.

- `DOCM-1-SPEC.md` — DOCM-1, `Datum::FaceFrame`, the sense beside the pose, the carrier-kind read (#1829)

## Per-merge deletion — DOCM-2's spec (2026-09-04)

Recoverable at `git show 286d9a08bb9f2fd8e549742e7213828005c97b89:docs/DOCM-2-SPEC.md`
(the DOCM-2 unit head, before the state-sync commit that deleted it;
the file carries the stop-clause amendment as its last section). The
rule above; the unit's record is its row in `MODEL-AB-LOG.md` and its
MERGED entry in `work/docm/log.md`.

- `DOCM-2-SPEC.md` — DOCM-2, `Node::Part`, a split's half or a pattern's instance as one body (#1860)

## Per-merge deletion — DOCM-5's spec (2026-09-04)

Recoverable at `git show 9f34220e3938076557f38722554446867a6ef3a0:docs/DOCM-5-SPEC.md`
(the DOCM-5 unit head, before the state-sync commit that deleted it;
unamended — no stop clause fired; its citation of `docs/PERF-PLAN.md`
and its fence over `product.rs` "Dual arms" are both corrected in the
unit's record). The rule above; the unit's record is its row in
`MODEL-AB-LOG.md` and its MERGED entry in `work/docm/log.md`.

- `DOCM-5-SPEC.md` — DOCM-5, the check registry's subject: one gather per landing (#1871)

## Per-merge deletion — PROPS-1's spec (2026-09-05)

Recoverable at `git show 62f81827717913c71e1dd5a213ead05e47319058:docs/PROPS-1-SPEC.md`
(the PROPS-1 unit head, before the commit that deleted it; unamended —
no stop clause fired). Two of its clauses were argued down rather than
met, and the argument is in the unit's PR: its pin (a) asks for
"narrower than the old form on every corpus row", which is false per
component once the anchor is exact, and its member-3 phrasing about the
parallel case does not survive measurement. The rule above; the unit's
record is its row in `MODEL-AB-LOG.md` and its item's `## Closed`
section.

- `PROPS-1-SPEC.md` — PROPS-1, the lost-correlation members of the linalg audit: `mirror_across_plane` and `reject_from` (#1918)
## Per-merge deletion — TOPO-D265's spec (2026-09-06)

Recoverable at `git show 7253e5efb66c479aa27c7381b878be0d1a2bd52a:docs/TOPO-D265-SPEC.md`
(the D265 unit head after the review fix pass, before the state-sync
commit that deleted it). Two of its stated facts were falsified by the
unit and are corrected in the unit's record, not here: `merge_group`
has **nine** sites that can return an `EulerOpError`, not the two the
spec named (`kev` and `kemr`) — four lookups, `loop_winding` through
`merged_outline_ring`, and four operator calls; and the spec's
expectation that the site-level argument would sweep nearly every
variant into corruption is wrong at one cell — the **(R) column is not
empty**, because `kef`'s `SameFace` is reachable on a tier-1-valid
nested group, the absorption's own ring drain having re-homed the dying
loop onto the survivor. The rule above; the unit's record is its row in
`MODEL-AB-LOG.md` and its item's `## Closed` section.

- `TOPO-D265-SPEC.md` — TOPO-D265, the merge door's arena-fault class, made true at the door (#2013)

## Per-merge deletion — TOPO-S330's spec (2026-09-05)

Recoverable at `git show 57cd299d8225afe4454bd068fa9e374439975363:docs/TOPO-S330-SPEC.md`
(the S330 unit head after the review fix pass, before the state-sync
commit that deleted it). Two of its stated facts were falsified by the
unit and are corrected in the unit's record, not here: `ops_cube` is not
a tier-3-clean planar cube (every face carries the mvfs placeholder), and
its Phase-2 shape for the poison door — a second predicate beside
`is_placeholder` — was replaced by `geom::NetState` at review. The rule
above; the unit's record is its row in `MODEL-AB-LOG.md` and its MERGED
entry in `work/topo/log.md`.

- `TOPO-S330-SPEC.md` — TOPO-S330, tier-3 check 1's described-NURBS arm (#1923)

## Per-merge deletion — MSOLVE-1's spec (2026-09-05)

Recoverable at `git show 550a9f2a9febb277ab69cf8d566850af65b51bfb:docs/MSOLVE-1-SPEC.md`
(the MSOLVE-1 unit head, before the state-sync commit that deleted it;
unamended — no stop clause fired). Three of its clauses were corrected
by measurement rather than met, and the argument is in the unit's PR:
A3's transform-of-pattern cannot reach a product (`Node::Transform`
takes one body; filed in `work/issues/`), A8's "refuses at the remap"
became a typed refusal at the split door in both directions
(`SplitError::OperandSeveredFromMate`), and the acceptance fixture the
spec's rows were first written against interpenetrated and was
re-authored as a physical seat at the fix pass. The rule above; the
unit's record is its item's `## Closed` section and its MERGED entry
in `work/msolve/log.md` (no A/B row: the program runs none).

- `MSOLVE-1-SPEC.md` — MSOLVE-1, the mate reads at its operand: the transform-aware solve (#1929)

## Per-merge deletion — PROPS lily-vec3's spec (2026-09-05)

Recoverable at
`git show 577338f4e452f3ff5839604eb4d810246671e75b:docs/PROPS-LILY-VEC3-SPEC.md`
(the unit head after the review fix pass, before the commit that
deleted it; unamended — no stop clause fired). Two of its statements
were argued rather than met, and the argument is in the unit's PR: its
census of the tuple helpers missed `Section::outline`'s 2-D algebra and
`review_probes::cross_norm`, both of which the unit converted because
the acceptance asks for ZERO tuple-algebra helpers; and its lift
spelling — "`map(S::from_f64)` at each boundary" — is right only for an
already-composed `f64` value, because `pncad::authoring::{p2, v2, p3,
v3}` are the kernel's own doors for components written at the call, so
the landed file spells the lift both ways on that line. The rule above;
the unit's record is its item's `## Closed` section (an E rider outside
the A/B experiment — no `MODEL-AB-LOG.md` row).

- `PROPS-LILY-VEC3-SPEC.md` — PROPS lily-vec3, the lily authored in `Vec3<f64>` through the kernel's own doors, lifted at the boundary (#1954)

## Per-merge deletion — PROPS ONB-measure's spec (2026-09-05)

Recoverable at `git show 8dd230c1c2964da6f5cefe27dac0836b74312bba:docs/PROPS-ONB-MEASURE-SPEC.md`
(the unit's last head; merged as PR 1939 at 48383a44c).

- `PROPS-ONB-MEASURE-SPEC.md` — PROPS ONB-measure — the four evidence-only measurements deciding the orthonormal-basis sign hull (PR 1939's body carries the tables; `work/props/interval-orthonormal-basis-sign-hull.md` is the record; no A/B row)

## Per-merge deletion — PROPS verdict-shapes' spec (2026-09-05)

Recoverable at `git show 9f52d8df89ba4d45197ee0654c2837f67ebeae21:docs/PROPS-VERDICT-SHAPES-SPEC.md`
(the unit's last head; merged as PR 1920 at 000d0100b). Moved here from
sweep 6's closed list at the 2026-09-05 sync — that list carries its own
count and recovery SHA.

- `PROPS-VERDICT-SHAPES-SPEC.md` — PROPS verdict-shapes — the two derived per-node verdict forms in one module (`resolve/vdiff.rs`), `ReplayOutcome` folded into `RunStatus` with `Absent` kept distinct, the strict-vs-population split pinned (last on `main` at 8ba880fda; PR 1920's body and `work/props/three-per-node-verdict-shapes.md` are the record; an E rider outside the A/B experiment, no row)

## Per-merge deletion — SEAT-8's spec (2026-09-05)

Recoverable at `git show 57dc0fe3a8558920c43cf433a61395d43470d337:docs/SEAT-8-SPEC.md`
(the SEAT-8 fix-pass head, before the state-sync commit that deleted
it; unamended — no stop clause fired, no ledger answer touched). Every
clause was met as written except one argued choice the spec left open:
the two-sided result landed as a per-door out-type (`SplitOut`) rather
than a record-with-body door, argued in the unit's PR as a choice, not
a force. The rule above; the unit's record is its item's `## Closed`
section (`work/seat/SEAT-8.md`) and its MERGED entry in
`work/seat/log.md`; its A/B row is MODEL-AB-LOG SEAT8.

## Per-merge deletion — PROPS span's spec (2026-09-05)

Recoverable at `git show c4cfa1c5a9349b18c67bf911f29befb610d1c9eb:docs/PROPS-SPAN-SPEC.md`
(the last commit on `main` carrying it). Two of its clauses were argued
rather than met, and the argument is in the unit's PR: it kept the three
surface `*_in_span` doors on `NurbsSurface` and, by the same shape, the
curve doors on `NurbsCurve`, on the premise that a door reading the
window's surface makes the mismatch "a type-level mismatch". Rust
lifetimes do not brand — two live references unify — so a door taking
`(structure, proof)` leaves the mismatch representable, and at the curve
half it left an index panic where the retired guard had returned poison.
Both families of doors therefore moved ONTO the windows
(`CurveWindow{2,3}`, `SurfaceWindow`), which is the only spelling in
which the pairing is unrepresentable. The rule above; the unit's record
is PR 1952's body, `work/props/span-carries-its-knot-vector.md` and the
residue item `work/props/coefficients-carry-their-knot-vector.md`.

- `PROPS-SPAN-SPEC.md` — PROPS span, `Span<'a>` carries its `KnotVector` (#1952)

## Per-merge deletion — PROPS coeffs' spec (2026-09-05)

Recoverable at `git show ea11576b4342fe00fc00639950119df5eb4e95a7:docs/PROPS-COEFFS-SPEC.md`
(the merge base the unit was cut against; the spec is on every `main`
commit from #1982's merge to the unit's). Met as written except the one
decision it left open, taken and argued at the module doc: weight
positivity stays a per-window check at the rational doors rather than
a mint-time refusal. The rule the unit lands is `crates/geom-core/README.md`
SPLINE-DESIGN S1 (coefficients against knots); the unit's record is PR
1985's body and `work/props/coefficients-carry-their-knot-vector.md`.

- `PROPS-COEFFS-SPEC.md` — PROPS coeffs, coefficients carry their knot vector (#1985)

## Per-merge deletion — PROPS vec3-doors' spec (2026-09-05)

Recoverable at
`git show 552b9cb0f35c1f1bc44d1b512df97d57f6f8a628:docs/PROPS-VEC3-DOORS-SPEC.md`
(the last commit carrying it, before the state-sync commit that deleted
it; unamended — no stop clause fired: the generic `const fn` compiled
at 1.97.0, so the fallback shape was never picked). Every clause was
met as written; the unit's PR argues three small choices rather than
deviations (one doctest reading all four constants; the 2-D twin named
in the one refusal sentence; the `skinned.rs` measurement taken as the
corpus row). The rule above; the unit's record is its item's `## Closed`
section (`work/props/vec3-point3-const-and-conversion-doors.md`, which
carries the `From` ruling) and its MERGED entry in `work/props/log.md`
(an E rider outside the A/B experiment — no `MODEL-AB-LOG.md` row).

- `PROPS-VEC3-DOORS-SPEC.md` — PROPS vec3-doors, `const fn new` on the four vector types, `Affine3::from_frame` as the one home, `SketchPlane::map`, the `Vec → Point` conversion refused at the type (#1977)
## Per-merge deletion — PROPS k-stats' spec (2026-09-05)

Recoverable at `git show 1d847bc84cfb4667eb296acf6233f2c3a88f4723:docs/PROPS-KSTATS-SPEC.md`
(the k-stats fix-pass head, before the state-sync commit that deleted
it; unamended). The ruling held — the bracket with a stack, the
returned value measured (530 call sites, 261 enclosing functions) and
declined in writing. Five clauses were argued rather than met, all in
PR 1969's body: `NodeError` carries the escalation channel beside
`NodeValue` (the named fixture fails its node, so the value cannot);
one shielding bracket on the part cache's miss path in `eval/parts.rs`
outside the named fence (an instantiate node's log is its own op's,
hit or miss, under both schedules); `Ok` nodes with escalations
bisect; the M10-6 accounting goldens and M10-7's tier-off copies re-cut
for the class the acceptance moves; and the acceptance's "did any
predicate escalate" holds for funnel predicates only — the op-minted
family, the raw `sign_within` calls and the mate solve are filed as
`work/props/escalation-channel-misses-op-minted-indeterminates.md`.
The dual's fix pass replaced the mis-nesting rule (frame ids; defined
in every profile) and the completeness claim. The rule above; the
unit's record is its item's `## Closed` section
(`work/props/k-stats-escalation-channel-and-redo.md`), PR 1969's body
and the six issues it filed.

- `PROPS-KSTATS-SPEC.md` — PROPS k-stats, the verdict log as a bracket with a stack and the escalation channel beside it (#1969)

## Per-merge deletion — PROPS rotation-floor's spec (2026-09-05)

Recoverable at
`git show 92dac21117c66ac8190eda9f80028bf1b49cce77:docs/PROPS-ROTATION-FLOOR-SPEC.md`
(the last commit carrying it, before the state-sync commit that deleted
it; unamended — no stop clause fired). A doc unit: every clause was met
as written — the ~17 % / 0 % pair re-verified at the head and unmoved,
the paragraph at `Mat3::rotation_about`, the composition rider filed at
`work/issues/mapped-curve-restrict-composes-placements-per-split.md`
(no program's `paths:` names `mapped.rs`, so `issues/` rather than a
program's slate), the item closed with the ruling. The rule above; the
unit's record is its item's `## Closed` section
(`work/props/rotation-about-diagonal-width-floor.md`) and its entry in
`work/props/log.md` (an E rider outside the A/B experiment — no
`MODEL-AB-LOG.md` row).

- `PROPS-ROTATION-FLOOR-SPEC.md` — PROPS rotation-floor, the diagonal's width floor documented at `rotation_about` rather than respelled; the composition rider re-homed (#1980)

## Per-merge deletion — SEAT-FW's spec (2026-09-05)

Recoverable at `git show 3fbfd1b9ae93931273f7855d33983a29b924e7ed:docs/SEAT-FW-SPEC.md`
(the SEAT-FW fix-pass head, before the state-sync commit that deleted
it; unamended — one Ev-gated stop fired and was honored by NOT editing
the ratified charter: `work/seat/verb-seat-design-s3-names-the-planar-
verifier.md` carries it to an `[ev]` PR). Every clause was met as
written; FW-2's scrutiny point resolved the way the spec allowed for —
the stem-glue pin stays because the refusal was never the detector's
blindness. The rule above; the unit's record is its item's `## Closed`
section (`work/seat/SEAT-FW.md`) and its MERGED entry in
`work/seat/log.md`; its A/B row is MODEL-AB-LOG SEATFW.

## Per-merge deletion — VERBS-C5ARMS's spec (2026-09-05)

Recoverable at `git show ae69dfeb2:docs/VERBS-C5ARMS-SPEC.md` (PR #1864's
merge commit, the last head carrying it). Both halves delivered: PR-1
plane×torus (#1577, VERBS) and PR-2 coaxial cone×cylinder (#1864,
CURVED — the first CURVED unit). One acceptance clause was measured a
category error rather than met: "`coned_tube`'s offset validates tier-3
with a closed-form volume pin" — the direct per-chart door is right to
refuse, and `shell(coned_tube)` succeeds flag-independently (a TORAX
row); recorded in the PR body and the A/B row (C5A2), not by amending
the spec. The rows 3/4/8 hold note points at
`docs/CURVED-SPIRIC-DESIGN.md` (ratified 2026-09-04), which supersedes
`docs/VERBS-RIMCAP-SPEC.md` §PR-2 as the klein elbow's binding text.
The unit's record is `work/curved/VERBS-C5ARMS.md`'s `## Closed` and
the MERGED entry in `work/curved/log.md`.

## Per-merge deletion — SEAT-DN's spec (2026-09-05)

Recoverable at `git show 625f3e0b91c7133c3952bcdaa97a45d52ba3d49c:docs/SEAT-DN-SPEC.md`
(the SEAT-DN fix-pass head, before the state-sync commit that deleted
it; unamended). DN-1 and DN-2 met as written; DN-3 resolved the way
the spec allowed for — measured, neither branch applied, the residue
found one level up filed and then re-scoped to its class at the fix.
The rule above; the unit's record is its item's `## Closed` section
(`work/seat/SEAT-DN.md`) and its MERGED entry in `work/seat/log.md`;
its A/B row is MODEL-AB-LOG SEATDN.

- `SEAT-DN-SPEC.md` — SEAT-DN, one direction-normalization body under two ratified funnel names; `Dir::from_unit` measured (#1987)

## Per-merge deletion — SEAT-9's spec (2026-09-05)

Recoverable at `git show ac167558386cc6acf7d5efd1b593a08bc8996ef2:docs/SEAT-9-SPEC.md`
(the SEAT-9 fix-pass head, before the state-sync commit that deleted
it; unamended). S9-1 met with one disclosed deviation (`Arity::Shell`
for "arity One", argued and accepted); S9-2 met as written after the
fix pass carried the witness into `geom-brep`'s production doors —
the frozen head had stopped one door higher and the dual weighed it.
The rule above; the unit's record is its item's `## Closed` section
(`work/seat/SEAT-9.md`) and its MERGED entry in `work/seat/log.md`;
its A/B row is MODEL-AB-LOG SEAT9.

- `SEAT-9-SPEC.md` — SEAT-9, the shell arm on `Verb` and ε travelling only as `Tol` down the offset-fit chain (#1995)

## Sweep 7 — 2026-09-08: GATES leaves the tracker

Sweep SHA: `5ce54b35bf355241de5fa5e3bb0cfeb264bf52af` — `main`'s tip immediately before the deletion,
so every path below is recoverable at
`git show 5ce54b35bf355241de5fa5e3bb0cfeb264bf52af:work/gates/<FILE>` and
`git show 5ce54b35bf355241de5fa5e3bb0cfeb264bf52af:docs/GATES-EXIT-WALK.md`.

GATES — the CI gate scripts, code-quality Track K's `scripts/gates/*`
half — opened 2026-09-06 in the tracker-wide cut and closed 2026-09-08
on Ev's ratification of `docs/GATES-EXIT-WALK.md` (PR #2185, "lgtm!").
Per the sweep-5 rule the program's directory leaves whole:
`program.md`, `plan.md`, `log.md`, and every closed item file
(26 of them: `D102`, `D103`, `D109`, `D211`, `S13`, `S49`, `anchored-exact-text-skip-has-three-homes`, `bit-identity-debug-only-gate-ends-an-item-at-a-semicolon`, `bounds-allowlist-select-cuts-at-the-first-colon`, `bounds-tripwire-blind-to-named-alias`, `clippy-panic-gate-blind-in-macros`, `debug-only-assert-euler-postcondition-is-on-no-row`, `debug-only-bit-witness-callers-are-on-no-row`, `debug-only-counters-have-no-gate`, `debug-only-helpers-outside-the-subject-list`, `debug-only-reader-cannot-place-a-statement-attribute-over-a-braced-call`, `gate-mod-path-resolved-textually`, `home-anchored-file-skip-is-unescaped`, `record-file-column-read-by-first-colon-split`, `test-module-resolution-has-three-homes`, `trait-generic-sole-bracket`, `unanchored-definition-skip`, `viewer-module-kinds-six-unreached-guards`, `whole-file-skips-are-hand-spelled-not-anchored`, `whole-file-skips-do-not-check-their-subject`, `window-view-emits-a-record-for-a-comment-only-line`). Twenty-six
rows landed over twenty-five PRs (2029–2069, 2077, 2156, 2157, 2170,
2174), each under one style review with a planted breach and a fix
pass; one `[ev]` ruling ratified (PR 2067, the gates stay greps); one
`[ev]` ruling open at the sweep (PR 2171, below). Infra-only: no A/B
rows; the band 3100–3199 was claimed for bookkeeping and stays
allocated.

| program | title | closed | done-state of record |
| --- | --- | --- | --- |
| `gates` | GATES — the CI gate scripts | 2026-09-08 | this row and the exit-walk row below; design at `scripts/gates/README.md` (the directory's page, listed in `docs/DESIGN.md`'s companion table) and in `scripts/gates/lib.sh`'s own headers |

### Residue re-homed before the deletion

Moved by `git mv` with ids kept (ownership is the directory):

| item | to |
| --- | --- |
| `D212` | `work/code-quality/` — rides `G4`, Track V's, which lives there |
| `directory-prefix-skips-have-no-subject-check` | `work/code-quality/` — open on its `crates/*/src/bin/` half, `blocked_on` `[ev]` PR 2171; the two-directory half closed with PR 2170 |

Filed on other slates during the program and already there:
`work-set-accepts-a-scalar-for-a-list-field` (META),
`doc-gate-error-sites-outside-the-gate-population` (CIW),
`bound-list-readers-have-three-homes` (code-quality). Nothing went to
`work/issues/`. Track K's `scripts/gates/*` fence returns to
code-quality with the two rows.

### The exit walk's row

| walk | program closed | ratified on | done-state now |
| --- | --- | --- | --- |
| `GATES-EXIT-WALK.md` | 2026-09-08 | PR #2185, in a PR comment | this row; the residue table above; `scripts/gates/README.md` |

## Sweep 7 — 2026-09-06: FILLET leaves the tracker

Sweep SHA: `efe21acb8f599dd146fbaadc0251dc3981ebbf9a` — `main`'s tip immediately before the deletion,
so every path below is recoverable at
`git show efe21acb8f599dd146fbaadc0251dc3981ebbf9a:work/fillet/<FILE>` and
`git show efe21acb8f599dd146fbaadc0251dc3981ebbf9a:docs/FILLET-EXIT-WALK.md`.

FILLET — the blend-completion second pass — closed 2026-09-06 on Ev's
ratification of `docs/FILLET-EXIT-WALK.md` (PR #1973, "1973 is good", in
chat). Per the sweep-5 rule the program's directory leaves whole:
`program.md`, `plan.md`, `log.md`, and every item file, every one of them
`status: closed` or `parked`-then-closed except the residue moved below.
Eight dualled units (samples #126, #131, #132, #134, #135, #136, #139,
#141; ordinals 2000–2007), three E openers under single style reviews,
five `[ev]` rulings (PRs 1733, 1734, 1735, 1736, 1916) and the H7
vocabulary ratified on PR 1819.

| program | title | closed | done-state of record |
| --- | --- | --- | --- |
| `fillet` | FILLET — blend completion, second pass | 2026-09-06 | this row and the exit-walk row below; the A/B rows in `docs/MODEL-AB-LOG.md`; design at `crates/sweep/README.md` (A3 as amended by H4/H5/H7), `crates/profile/README.md` (the `NoCornerOfPair` envelope), `crates/topo/README.md` (`rim_of`) |

### Residue re-homed before the deletion

Moved by `git mv` with ids kept (ownership is the directory):

| item | to |
| --- | --- |
| `rim-door-admits-a-double-cover` | `work/seat/` |
| `cut-off-arc-persists-as-a-corner-arc` | `work/docm/` |
| `debug-in-prose-at-blend-and-step-import` | `work/fix/` |
| `blend-size-gate-unmetered-under-epsilon`, `blend-payloads-outside-the-margin-family` | `work/props/` |
| `ambiguity-k-below-the-cap-rim-crossover`, `blend-recourses-under-describe-their-doors`, `curved-single-host-rim-refuses-at-the-half-band-gate`, `escalated-recourse-dispatch-has-no-coaxiality-arm`, `fillet-escalation-site-has-no-producer`, `hostless-rim-on-a-ringed-host-refuses`, `ladder-rim-phase-may-retire-a-new-split-key`, `overrun-attribution-picks-the-first-candidate`, `path-fillet-door-validator-tangency-disagree`, `rim-seed-finders-disagree-on-at-this-radius`, `ring-clearance-refuses-a-nested-trim-circle`, `smooth-arm-siblings-disagree-on-the-in-band-case`, `sweep-top-field-docs-make-the-spatial-claim-capend-shed`, `tangent-parallel-certifier-passes-a-transverse-arc` | `work/issues/` — the blend kernel and the profile fillet door have no live program, and S-CERT (the certifier's owner) has already left the tracker; the next blend program's opening slate |

Closed at the sweep as records, not work: `plane-hosted-rim-has-no-native-instance`
(Phase 1's census, cited by the walk). The `corner-config-tag-all-concave-trihedron`
ruling stays code-quality's (re-asked on PR 1935).

### The exit walk's row

| walk | program closed | ratified on | done-state now |
| --- | --- | --- | --- |
| `FILLET-EXIT-WALK.md` | 2026-09-06 | PR #1973, in chat | this row; the residue table above; vocabulary and bands at `crates/sweep/README.md` |
## Per-merge deletion — MSOLVE-4's spec (2026-09-06)

Recoverable at `git show e1ede48bff885c29770429897fb91876364836c6:docs/MSOLVE-4-SPEC.md`
(the MSOLVE-4 unit head, before the state-sync commit that deleted it;
unamended — no stop clause fired). One clause was withdrawn at the fix
pass rather than met: A5's "a document with no mates keys bit-for-bit
as before" — the key's format tag bumps to v6 with the mate's answer
joining the key, because the tag block's own purpose is an honest
input-set version for any future persistence, and the clause had no
consumer (the correctness arm measured it true before the bump). The
rule above; the unit's record is its item's `## Closed` section and
its MERGED entry in `work/msolve/log.md` (no A/B row: the program runs
none).

- `MSOLVE-4-SPEC.md` — MSOLVE-4, a mate's memo key carries the solve's answer (#1960)

## Per-merge deletion — DOCM-7's spec (2026-09-06)

Recoverable at `git show 1f566daa82b616655cacb91bb7c9f886109df89f:docs/DOCM-7-SPEC.md`
(the DOCM-7 unit head, before the state-sync commit that deleted it;
unamended — no stop clause fired). Its D9 sentence ("the order is the
list's, the result is not") is measured false at the B-rep and name
level by both reviews (the pair verb is asymmetric in its operands)
and across a chain of contacts by acceptance (the chained-order gap);
both are the unit's own filed issues, and DM4 carries the measured
limit. The rule above; the unit's record is its row in
`MODEL-AB-LOG.md` and its MERGED entry in `work/docm/log.md`.

- `DOCM-7-SPEC.md` — DOCM-7, `Node::Union`'s declaration channel in member space (#2028)

## Per-merge deletion — DOCM-6's spec (2026-09-06)

Recoverable at `git show aaf013910bfa7cc01b1a925c0f7e4de88eb60dab:docs/DOCM-6-SPEC.md`
(the DOCM-6 unit head, before the state-sync commit that deleted it;
§Ruling carried Ev's 2026-09-06 inner-mint-refusal ruling; no stop
clause fired). Two of its letters are corrected in the unit's record:
its fence's "not the at-rest gate's verdicts" and A5's "unchanged on
every fixture" — the `Uncertified` predicate accepting a carried
decline moves one verdict, ruled the contract's and rowed; and its
"`ASSEMBLY.md` D-1 paragraph" — the clause is A5 (D-1 is ASM-R2b's
id). The rule above; the unit's record is its row in `MODEL-AB-LOG.md`
and its MERGED entry in `work/docm/log.md`.

- `DOCM-6-SPEC.md` — DOCM-6, the instantiation seam carries mate identity and mint health (#2035)
## Per-merge deletion — MSOLVE-2's spec (2026-09-06)

Recoverable at `git show ef8926c2df3f39e5f351f4dd972931dc037e7519:docs/MSOLVE-2-SPEC.md`
(the MSOLVE-2 unit head, before the state-sync commit that deleted it;
unamended — no stop clause fired). Two clauses were corrected by
measurement rather than met, and the argument is in the unit's PR:
A2(a)'s "different inner index, same outer index" under one outer
pattern is unbuildable (a pattern takes one body), so the row holds the
outer index across two chains; and item 3 placed the `Part`-index check
in the offset, which runs only for a tree edge's first mate — the
correctness review found a declaring mate's mismatch silently green,
and the checks that need evaluation moved to the solve's own walk site
for every reference. The rule above; the unit's record is its item's
`## Closed` section and its MERGED entry in `work/msolve/log.md` (no
A/B row: the program runs none).

- `MSOLVE-2-SPEC.md` — MSOLVE-2, the member chain: nested copies through `Part`, sibling distinctness at every level (#2039)

## Per-merge deletion — MSOLVE-3's spec (2026-09-06)

Recoverable at `git show 8a7288747941ef53a8136f7fc98b89e8eb51c135:docs/MSOLVE-3-SPEC.md`
(the MSOLVE-3 unit head, before the state-sync commit that deleted it;
unamended — no stop clause fired). Written before MSOLVE-2 restructured
the vocabulary, so its "what the tree says now" names a `head_of` and a
count arm that had moved by dispatch; the unit applied its intent to
the sites that exist and says so in its PR. Two of its clauses were
corrected by the reviews rather than met: the carried refusal is an
`Arc` newtype over `NodeErrorKind` (the fault types derive equality
the kernel error type cannot), and the placer named is the refusing
node, which for a circular rule's axis is the datum. The rule above;
the unit's record is its item's `## Closed` section and its MERGED
entry in `work/msolve/log.md` (no A/B row: the program runs none).

- `MSOLVE-3-SPEC.md` — MSOLVE-3, the mate solve reports the evaluation's own refusal: `PlacerRefused`, and the placement axis decided (#2081)

## Per-merge deletion — MSOLVE-5's spec (2026-09-06)

Recoverable at `git show ae78e6e8d22005ca276bb16aac6cf49c42ae2da3:docs/MSOLVE-5-SPEC.md`
(the MSOLVE-5 unit head, before the state-sync commit that deleted it;
unamended). Its stop clause FIRED: the "operand answers, `at` a root,
product silent" bullet is reachable through a root instance's BODY
row, which `carry_names` drops. The orchestrator ruled on the PR's
draft that the operand's entry decides its kind before the root
question, so the spec's "`ReadBelowARoot` for ANY entry" narrowed to
face entries and a non-face entry answers `NotAFace { kind }`
wherever it is read; the unit's PR carries the ruling. The rule above;
the unit's record is its item's `## Closed` section and its MERGED
entry in `work/msolve/log.md` (no A/B row: the program runs none).

- `MSOLVE-5-SPEC.md` — MSOLVE-5, the at-rest gate refuses a mate read below a product root in the operand's voice (#2090)

## Per-merge deletion — TRIM-1's spec (2026-09-07)

Recoverable at `git show 416ccdfc6:docs/TRIM-1-SPEC.md` (PR #2095's
merge commit, the last head carrying it). Every clause met as written,
with the rulings section answering the lane's three questions
(rational cases (a)+(b) by bitwise weight tests; `iso_boundary_row`
unchanged; the two riders carried). One thing the spec's acceptance did
not foresee and the unit recorded: TRIM-2's opening measurement is
re-cut — on the P-2 fixture the tessellation lane stops at
`patch_bound::Degree1Crease` before any of the six filed sites. The
unit's record is `work/trim/interior-iso-curve-de-boor-extractor.md`'s
`## Closed` and the MERGED entry in `work/trim/log.md`; its A/B row is
MODEL-AB-LOG T1.

## Per-merge deletion — DOCM-8's spec (2026-09-07)

Recoverable at `git show 4b4ec3213b69cbeef7e10a312f7aa4714ad1ac09:docs/DOCM-8-SPEC.md`
(the DOCM-8 unit head, before the state-sync commit that deleted it;
AMENDED once, 2026-09-06, at its stop clause — item 1's flat mint reads
a constituent's name through its descent wrappers, A2's walker peels
them, the stop clause resolved; the amendment is commit `0cf4a650`).
Its acceptance letter that "reordering the members changes nothing
about whether a declaration resolves" is bounded in the unit's record
to faces consumed by merges (a split, a containment, a fragmented
merge stay order-shaped — filed). The rule above; the unit's record is
its row in `MODEL-AB-LOG.md` and its MERGED entry in `work/docm/log.md`.

- `DOCM-8-SPEC.md` — DOCM-8, a merged face's name is a flat constituent set; a member-space declaration resolves through the fold's merges (#2073)

## Per-merge deletion — PROPS budget-faces' spec (2026-09-08)

Recoverable at `git show 887f5e39d3b868cc748da50c8d81d02df74601dd:docs/PROPS-BUDGET-FACES-SPEC.md`
(the unit head, before the state-sync commit that deleted it). Its
face-2 instance (`d = 1e-7` on the quarter cylinder as a finite cap
stop) did not survive measurement — that instance is face 4, and the
face-2 red-first row is the bumpy patch at 1e-15 — and its face 4 is
landed as `BoundNotFinite` carrying `last_finite`, per the review. The
rule above; the unit's record is its `## Closed` and the MERGED entry
in `work/props/log.md` (an E rider: no A/B row).

## Per-merge deletion — LIB-G17's spec (2026-09-08)

Recoverable at `git show 4a093c5cb4b9bd0a901111cdd89c230e9445a075:docs/LIB-G17-SPEC.md`
(the frozen review head; the fix pass moved one decision the spec made —
the f64 witness fold now lives on the lane, `crates/editor-core/src/lane.rs`,
per the dual review's convergent finding, and the unit's record says so).
The rule above; the unit's record is its row in `MODEL-AB-LOG.md`, its
MERGED entry in `work/lib/log.md`, and `work/lib/LIB-G17.md`'s Delivered
and Fix pass sections.

- `LIB-G17-SPEC.md` — LIB-G17, `Node::Shell`, the shell recipe door (#2150)

## Sweep 9 — 2026-09-08: EVAL leaves the tracker

Sweep SHA: `9c515cb150e8f396c6f197c2354650e8e676e11d` — the ratification
commit immediately before the deletion (on the closing PR's branch,
reachable from `main` through that PR's merge commit), so every path
below is recoverable at
`git show 9c515cb150e8f396c6f197c2354650e8e676e11d:work/eval/<FILE>`,
`git show 9c515cb150e8f396c6f197c2354650e8e676e11d:docs/EVAL-EXIT-WALK.md`
and `git show 9c515cb150e8f396c6f197c2354650e8e676e11d:docs/EVAL-<N>-SPEC.md`
for the seven specs listed below.

EVAL — the evaluation seat (`crates/editor-core/src/eval/*`, the verb
seat, the names emitters, `topo::query`/`flush`), the ground SEAT held
and Track V never staffed — opened 2026-09-06 in the tracker-wide cut
and closed 2026-09-08 on Ev's ratification of `docs/EVAL-EXIT-WALK.md`
(PR #2201, "lgtm!", merged `8d34121c7`). Per the sweep-5 rule the
program's directory leaves whole: `program.md`, `plan.md`, `log.md` and
every item file, every one `status: closed` except the five re-homed
below. Eleven E units under the CIW/CHROME posture (one style review
each, a correctness arm on units 2, 6, 7, 8, 9), no A/B rows (band
3000–3099 claimed and unused), two `[ev]` rulings (PRs 2137, 2138).

| program | title | closed | done-state of record |
| --- | --- | --- | --- |
| `eval` | EVAL — the evaluation seat: wiring, the verb seat and the names emitters | 2026-09-08 | this row and the exit-walk row below; the units' PRs (2139, 2153, 2160, 2165, 2168, 2173, 2176, 2186, 2190, 2194, 2195); design at `docs/DESIGN.md` Band 1 (the content key's inputs, amended by PR 2201), `crates/profile/README.md` (V6, the validated lift), `crates/verbs/README.md` (the `Verb` convention) and the tag declarations at `eval/mod.rs`'s `mod tag` (format 7) |

What the walk records: eleven units MET (three with recorded honesty),
the two rulings answered (placers are shape-preserving over
`Instances`; a node's log is every decision made on its behalf), the
deferred row carried; ten honesty rows, among them the session
rate-limit interruption, three red pushes from skipped pre-push
checks, three spec premises the lanes refuted, and the ground
reverting to no owner at close (Ev's ratification took the walk's
recommendation).

### Residue re-homed before the deletion

Moved by `git mv` with ids kept (ownership is the directory):

| item | to |
| --- | --- |
| `D360` | `work/topo/` — a standing sweep rule over `topo`'s refusal enums |
| `map-affine-retires-into-affine3-try-map` (parked) | `work/props/` — beside the PROPS door it is parked on |
| `two-verb-seats-do-not-compose` (deferred), `frame-f64-placement-is-re-evaluated-per-profile`, `wire-expected-phrases-spell-family-words-as-literals` | `work/issues/` — the evaluation seat has no live program after EVAL; with `profile-embed-lift-has-two-homes-anchor-and-loft` and `placement-lifts-its-affine-by-hand-beside-affine3-map` already there, the successor's opening slate |

### The exit walk's row

| walk | program closed | ratified on | done-state now |
| --- | --- | --- | --- |
| `EVAL-EXIT-WALK.md` | 2026-09-08 | PR #2201, "lgtm!" | this row; the residue table above; the amended DESIGN Band 1 sentence |

### Per-unit specs, unit merged

Seven specs were not deleted at their units' merges and leave at this
sweep, recoverable at the sweep SHA; the other four left per-merge
and are ledgered here at the head that last carried each:

- `EVAL-1-SPEC.md` — EVAL-1, the affine lift's one home (#2139)
- `EVAL-2-SPEC.md` — EVAL-2, the tag vocabularies declared once (#2153)
- `EVAL-3-SPEC.md` — EVAL-3, `emit_blend` cites the kernel's arguments (#2160)
- `EVAL-4-SPEC.md` — EVAL-4, `D367`'s accept funnel (#2165)
- `EVAL-5-SPEC.md` — EVAL-5, the two `Verb` types' convention (#2168)
- `EVAL-6-SPEC.md` — EVAL-6, the placers over `Instances` (#2173; the spec rode `[ev]` PR 2137)
- `EVAL-7-SPEC.md` — EVAL-7, the node bracket (#2176; the spec rode `[ev]` PR 2138)
- `EVAL-8-SPEC.md` — EVAL-8, validate once under the pinned lift (#2186); recoverable at `git show d285b8301cc13f31ea60193f72e7de5a07a8bcf3:docs/EVAL-8-SPEC.md`
- `EVAL-9-SPEC.md` — EVAL-9, the slot nominal joins the content key, format 7 (#2190); recoverable at `git show 2011634817c3417feb72e0285ad4475856edbb24:docs/EVAL-9-SPEC.md`
- `EVAL-10-SPEC.md` — EVAL-10, one nominal environment (#2194); recoverable at `git show cf1ad66e0570ffeef1db6f1aa747341a21f77b1f:docs/EVAL-10-SPEC.md`
- `EVAL-11-SPEC.md` — EVAL-11, `node_value_kind` through the placer chain (#2195); recoverable at `git show 0e59a45d374dfa56300925e1ce8b9e08738829ca:docs/EVAL-11-SPEC.md`

## Per-merge deletion — LIB-TEAPOT's spec (2026-09-08)

Recoverable at `git show 21e6d1e284f61fd5ea303a56e41b9a19d8360364:docs/LIB-TEAPOT-SPEC.md`
(the frozen review head). Three of its sentences did not survive the
unit and the record says which: §4's SIX rim names are THREE (the lid's
annular profile takes the emitter's lamina branch) and its ONE
`Node::Fillet` is TWO (the one-request output cannot be named —
`RoleSeg::BandSlit` keyed on the source edge alone, filed); §8's third
clause is CONTRADICTED by a baseline re-cut the gate never asked for
(the file moved with no geometry moved; disclosed, the reverse re-cut
scheduled by a live probe); §9 admitted one reason for a moved render
cell and the uv sheet's `teapotlid` cells moved for another (face
order), recorded as the §9 finding it is. §5's admissible re-baseline
never fired: the axis-angle placement lands the spout at the authored
root to 0.0, measured off the placed body at the fix pass. The rule
above; the unit's record is its row in `MODEL-AB-LOG.md`, its MERGED
entry in `work/lib/log.md`, and `work/lib/LIB-TEAPOT.md`'s Delivered
section.

- `LIB-TEAPOT-SPEC.md` — LIB-TEAPOT, the tour's teapot through the document (#2206)

## Sweep 10 — 2026-09-09: METER leaves the tracker

Sweep SHA: `2723839067e80198bec2889d041e490000f02275` — the commit
immediately before the deletion (on the closing PR's branch, reachable
from `main` through that PR's merge commit; it is the state in which
METER's directory is complete and every row in it closed), so every path
below is recoverable at
`git show 2723839067e80198bec2889d041e490000f02275:work/meter/<FILE>` and
`git show 2723839067e80198bec2889d041e490000f02275:docs/METER-EXIT-WALK.md`.

METER — the budget and K instruments, code-quality Track K's `tools/*`
half (`tools/tess-lint`, `tools/tess-meter`, `tools/k-lint` and the two
documents they feed, `docs/TESS-BUDGET.md` and `docs/K-REPORT.md`, with
their committed data) — opened 2026-09-06 in the tracker-wide cut and
closed 2026-09-08 on Ev's ratification of `docs/METER-EXIT-WALK.md`
(PR #2212, ruled in three comments in that thread). Per the sweep-5 rule
the program's directory leaves whole: `program.md`, `plan.md`, `log.md`
and every item file, all seventeen `status: closed` at the sweep SHA
(`D201`, `D203`, `D206`, `D213`, `D214`,
`cert1-notes-pr-body-tracked-on-main`,
`cut-prefix-three-unpinned-spellings`,
`fold-the-two-baseline-census-files`, `k-lint-predicate-roster-unpinned`,
`k-report-baseline-fold-cert1-roster`,
`k-report-era-witnesses-have-no-guard`,
`report-header-column-phrases-unqualified`,
`sweep-deleting-work-meter-dangles-six-refs-on-instr-rows`,
`tess-budget-doc-finding-block-stale`, `tess-lint-face-ordinal-join`,
`tess-lint-twinned-csv-fixture`,
`tools-readme-is-unratified-and-owes-a-design-row`); the other twenty-two
rows the slate held were re-homed first, below. **Thirteen unit PRs**
landed, numbered 0–12 (2111, 2114, 2115, 2125, 2132, 2140, 2151, 2158,
2167, 2177, 2179, 2180, 2187) — not the twelve the plan and the log both
claimed, which is criterion 1's recorded honesty. Two `[ev]` rulings
ratified (2109, `D201`'s design fork, arm A; 2147, `tools/README.md`'s
`CC1`–`CC5`), three orchestrator state-sync PRs (2110, 2128, 2164), and
three closing PRs (2212 the walk, 2218 the discipline paragraph, 2220 the
successor). Infra-only: no A/B rows were written; the band 3200–3299 was
claimed for bookkeeping and stays allocated.

| program | title | closed | done-state of record |
| --- | --- | --- | --- |
| `meter` | METER — the budget and K instruments | 2026-09-08 | this row and the exit-walk row below; the thirteen units' PRs above; design at `tools/README.md` (`CC1`–`CC5`, the reading-boundary rule, ratified by Ev 2026-09-08 and listed in `docs/DESIGN.md`'s companion table) |

What the walk records: criterion 1 MET with recorded honesty (thirteen
units, not twelve, and `D201` built rather than deferred); criterion 2
**CARRIED** — Track K's `tools/*` half is not empty and was never close,
so the walk's central ruling opened `instr` for it; criterion 3 MET. Its
census re-derived three published figures that were wrong (the slate at
22 open and not 23, 40 files with 23 open and not 24, thirteen units and
not eleven or twelve), and §2 carries one line per unit of what it left
behind.

### Residue re-homed before the deletion

The moves happened in PR #2220, the successor PR, not in the deleting
commit; the deleting commit finds the directory already emptied of live
work.

| item | to |
| --- | --- |
| twenty rows — `C15`, `baseline-census-partition-assert-cannot-fail`, `baseline-sizing-census-pointers-stale`, `baseline-sizing-census-second-copy`, `cut-line-commit-names-no-baseline-change`, `gate-findings-name-no-columns-and-recoverable-has-two-aggregations`, `k-lint-csv-header-unpinned-against-five-producers`, `k-lint-eps-coupled-criterion-unwritten`, `k-lint-gate-described-as-diffing-the-committed-baselines`, `k-lint-last-round-is-eps-coupled-but-unrostered`, `k-lint-roster-wants-a-kernel-side-vocabulary`, `k-lint-rule-1-prose-assumes-every-in-band-site-refuses`, `report-constraint-activity-line-names-no-columns`, `tess-budget-doc-identity-column-list`, `tess-budget-doc-note-finding-rule`, `tess-lint-growth-margin-unprotected-from-ceil-quantisation`, `tess-lint-recourse-quote-half-pinned`, `tess-lint-ungated-columns-fold-silently`, `tess-lint-zero-certificate-two-meanings`, `tess-meter-sampled-retune-figure-unreproducible` | `work/instr/` — INSTR, the successor Ev ruled open on 2026-09-08 (walk §4), which takes METER's `paths` unchanged and the band 3300–3399. Nineteen are instrument rows; `C15` is the twentieth, ruled here rather than to code-quality Track X (walk §5, reversing the walk's own first ruling on Ev's challenge) |
| `reader-census-shared-disposition-survives-partial-reversion` | `work/tcost/` — S-TCOST's `paths` carries `crates/test-utils/*`, and the `Shared` ledger audit is a ruling on every row of that ledger |
| `cert1-notes-pr-body-tracked-on-main` | nowhere — **closed by disposition**: Ev ruled *"delete"* (2026-09-08), `.cert1-notes/pr-body.md` was removed in PR #2220 and the row closed citing the ruling. It asked for a disposition, not for work |

Filed on other slates during the program and already there:
`cut-regex-unanchored-admits-a-line-the-lint-refuses` and
`cut-script-header-claims-no-cross-language-gate-exists` (CIW). Nothing
went to `work/issues/`.

### The six `refs:` this sweep rewrote

Deleting the directory would have broken `scripts/work.py`'s *references
resolve* rule on six INSTR rows that cite five closed METER ids, so the
deleting commit rewrites them first, in GATES' form (`3a8dd05fe`, sweep 7):
the dying id is replaced in `refs:` by the number of the PR that closed
it — ints are PR numbers and lint does not check them — and a
`## Refs at METER's sweep (2026-09-09)` section at each citing row says
which id is now cited by which PR.

| dying id | now cited as | citing row(s) |
| --- | --- | --- |
| `D201` | 2167 | `C15` |
| `cut-prefix-three-unpinned-spellings` | 2151 | `cut-line-commit-names-no-baseline-change` |
| `k-lint-predicate-roster-unpinned` | 2115 | `k-lint-csv-header-unpinned-against-five-producers`, `k-lint-eps-coupled-criterion-unwritten` |
| `k-report-baseline-fold-cert1-roster` | 2140 | `k-lint-gate-described-as-diffing-the-committed-baselines` (already carried 2140, so the id was dropped rather than duplicated) |
| `tess-lint-twinned-csv-fixture` | 2179 | `tess-lint-recourse-quote-half-pinned` (already carried 2179, same) |

The hazard was found and filed while step 3 executed
(`sweep-deleting-work-meter-dangles-six-refs-on-instr-rows`, closed at the
sweep SHA); prose citations of `work/meter/…` and of the walk survive
across the tree unrewritten, which is what *A note on inbound references*
above is for and what GATES' sweep did with its own.

### §6's process findings — one paragraph carried, the rest ruled out

The walk's §6 wrote out four process findings the log would otherwise take
with it, and §6.4 ruled they should go into
`docs/prompts/implementer-discipline.md` with Ev seeing the diff first.
**He then ruled that only ONE paragraph goes in** (PR #2218, 2026-09-08):
*"can you keep the 'write assertions a bug could break' paragraph and
revert everything else? most of this is not relevant to most work in the
repo, which deals with normal implementation rather than these greps"* —
and, on the same PR, *"if you do what my prev comment stated then you can
consider this approved"*. The merged diff is exactly those five lines, in
§2 beside *a build is not a test*.

**So §6's other findings are deliberately not carried forward**, and this
sentence exists so a later reader does not read the omission as an
oversight: §6.1's nine assertions that cannot fail with their per-instance
record, §6.2's three fake greens with their mechanisms (the mutation
script that failed before writing, the mutation on a doc comment, and
`grep` exiting 0 on a match inside a `&&` chain), §6.3's
shape-versus-reading finding with its six sub-rules, and the two
operational lessons (one `git worktree` per concurrent lane; `main` in an
ephemeral container is not `origin/main`) survive **only** at the sweep
SHA above, in `docs/METER-EXIT-WALK.md` §6 and in `work/meter/log.md`.
That is Ev's call and it was made on the diff itself.

### The exit walk's row

| walk | program closed | ratified on | done-state now |
| --- | --- | --- | --- |
| `METER-EXIT-WALK.md` | 2026-09-08 | PR #2212, in three PR comments (*"1. open / 2. giving them to the successor sounds good / 3. huh i thought track X had closed / 4. delete / 5. i would like to see the diff / 6. this doesn't look like a question?"*, then *"the new 3, instr, and your plan all sound good!"*) | this row; the residue table above; `tools/README.md` and the three instrument crates' own headers; `work/instr/` for the twenty rows it carried forward |


## Per-merge deletion — BLEND-6's spec (2026-09-13)

Recoverable at `git show e3f8d06a4b4fda4ce3720b4f8783c218ce119890:docs/BLEND-6-SPEC.md`
(the fix-pass head; the spec's two measured errors — the dome rim's
convexity and oracle, the "nothing meters them today" premise — are
corrected in the PR body and the unit's items, not in the spec). The
unit's record is its row in `MODEL-AB-LOG.md`, its MERGED entry in
`work/blend/log.md`, and the `## Closed` sections of its two items.

- `BLEND-6-SPEC.md` — BLEND-6, the ring-clearance circle arm gets its relation (#2215)
## Per-merge deletion — BLEND-7's spec (2026-09-13)

Recoverable at `git show 3dd896cb8220f063d152101836b7cd9e1eb67e86:docs/BLEND-7-SPEC.md`
(the fix-pass head; the spec's 'concave bore twin' is convex and its
centroid sentence names the wrong corner — both corrected in the PR
body, not in the spec). The unit's record is its row in
`MODEL-AB-LOG.md`, its MERGED entry in `work/blend/log.md`, and the
`## Landed` section of its item.

- `BLEND-7-SPEC.md` — BLEND-7, a closed chain's junctions judged against the links that touch them (#2483)

## Per-merge deletion — BLEND-9's spec (2026-09-13)

Recoverable at `git show 3996ee8c1dc76b5371fa0bfb46511b68e738001f:docs/BLEND-9-SPEC.md`
(the fix-pass head; the spec's `blend8_` suite spelling and its
"restate the count as two" line were superseded — the suite is named by
subject and `folded_lever_arm`'s list is a different list — both
recorded in the PR body). The unit's record is its row in
`MODEL-AB-LOG.md`, its MERGED entry in `work/blend/log.md`, and the
`## Landed` section of its item.

- `BLEND-9-SPEC.md` — BLEND-9, the must-carry rule's in-band policy has one home (#2491)
## Per-merge deletion — BLEND-11's spec (2026-09-13)

Recoverable at `git show 7e0758c87792fd058cf150658136af5a75d68fe8:docs/BLEND-11-SPEC.md`
(the fix-pass head). Two of its sentences did not survive review: the
pick could not be spelled inside `sugar.rs` (the Bounds scope rule)
and moved to the door's `map_refusal` through
`fillet_select::nearest_candidate`; and its reason for the pick — "the
least radius reduction that would make it fit" — was falsified by both
reviewers over every grid-A entry (the deficit meters the setback, not
the radius). Both are recorded in the PR body and the unit's log entry;
the residue is `work/blend/anchor-fit-refusal-reports-a-setback-excess-not-a-radius-reduction.md`.

- `BLEND-11-SPEC.md` — BLEND-11, the overrun refusal reports the nearest fit (#2495)

## Sweep 18 — 2026-09-21: FIX leaves the tracker

Sweep SHA: `6f0e04ce1534f1c7d25e5dfbe09751b992f8492e` — the commit
immediately before the deletion (on the closing PR's branch, reachable
from `main` through that PR's merge commit; it is the state in which
FIX's directory is complete and every row in it is closed), so every
path below is recoverable at `git show 6f0e04ce1534:work/fix/<FILE>`
and `git show 6f0e04ce1534:docs/FIX-EXIT-WALK.md`.

FIX — kernel and façade doors with the fix written — opened 2026-09-03
in the tracker-wide cut of that day (`docs/WORK-TRACKS-2026-09.md`
§FIX) and closed 2026-09-21 on Ev's in-chat ruling, quoted in the walk.
**Forty-eight rows closed, forty-four carrying a PR**, none larger than
class E and none reviewed adversarially. The last wave merged five on
their own green hosted heads: #2943 (`Subject::refused` routes the
no-body refusal), #2944 (quantity's F6 row folded onto the census
weld), #2945 (the missed node id carried at ten `refactor.rs` sites),
#2946 (`FilletLegCarrier`'s scalars through `path::num`), #2948 (four
second-hop carriers name their repairs). Band **1700–1799** was claimed
at the joint opening and **never drew an ordinal** — the A/B exemption
(Ev, 2026-09-04) held for the program's life, so the band is claimed
and empty in `docs/MODEL-AB-LOG.md` as VIEW's is. The directory leaves
whole, with no row re-homed at the sweep: the slate was emptied first.

| program | title | closed | done-state of record |
| --- | --- | --- | --- |
| `fix` | FIX — kernel and façade doors with the fix written | 2026-09-21 | this entry; the walk at the sweep SHA; the code it left — `checks::Subject::refused` routing on `ProductErrorKind::means_no_body`, `refactor.rs`'s three name-shaped refusals carrying the node the rewrite stopped at, `path::num` as `crates/profile`'s one rounding grid, `quantity`'s display row on `test_utils::f6`'s census weld, and enforcement rows at `SplineError`, `KnotVectorIssue`, `MeterError`, `PatchBoundError` and `FitError` |

### What survived, and where

The program's output is the tree and three practices, not its directory.

- **The ruling that closed it, which is the durable finding.** Ev,
  2026-09-21: *"most of those were either mis-filed or should've been
  done as drive-by fixes."* Measured: **thirty-eight rows left this
  directory** across its life, re-homed to the track owning their
  ground, against forty-eight that closed from it. A one-line fix on
  another program's file is cheapest taken by whoever is next in that
  file; routing it through a separate program's slate, dispatch, lane,
  PR and seam announcement costs more than the fix. **The successor
  practice is not a successor program**: file the row on the slate of
  the program whose ground it lands on the day it is found, take it in
  passing when you are already in the file, and let `work/issues/` be
  the last resort `work/README.md` says it is.
- **The pin question**, carried as instruction 3 of every dispatch: a
  unit that changes what a refusal carries or how it renders owes an
  answer to *"does any existing pin discriminate the old behaviour from
  the new one?"*, where **no** means the missing pin is part of the
  defect. It went **five for five** in the program's last wave. The
  sharpest statement is #2946's: nothing was re-baselined, because
  nothing had ever pinned that sentence.
- **Route from the instrument, never from a fence read in prose.**
  `scripts/work.py territory` was right every time the seat ran it and
  wrong every time the seat inferred an owner instead — eight recorded
  corrections, two of which had been the stated reason a row was homed
  here at all (`topo/src/census.rs` recorded as CURVED's, actually
  REACH's; `editor-core/src/mc.rs` recorded as unowned, actually
  PROPS'). A program that spans fences by construction lives or dies on
  this.
- **A program with no ground can never rule.** The design-free sweep of
  2026-09-20 (Ev: *"kick all the design decisions back to the track they
  actually belong to"*) moved fourteen rows out of FIX and four out of
  DOOR in one sitting. FIX owns three files, so a decision about those
  was its own to take; DOOR claims none, so every decision it holds
  leaves. Both charters now say so.

## Sweep 17 — 2026-09-17: BLEND leaves the tracker

Sweep SHA: `2cfe07f2fac8baa067f5b3eff0e2f4c98fd8f8e0` — the commit
immediately before the deletion (on the closing PR's branch, reachable
from `main` through that PR's merge commit; it is the state in which
BLEND's directory is complete, `program.md` reads `status: closed`,
and every row in it is closed), so every path below is recoverable at
`git show 2cfe07f2fac8:work/blend/<FILE>`,
`git show 2cfe07f2fac8:work/blend/logs/<FILE>` and
`git show 2cfe07f2fac8:docs/BLEND-EXIT-WALK.md` (the unit specs left at
their own merges — the per-merge deletion entries above name each
one's SHA).

BLEND — the blend kernel and the profile fillet door — opened
2026-09-06 as FILLET's successor in the tracker-wide cut of that day
and closed 2026-09-17 on the walk Ev ratified on its PR (#2826,
merged `1addbf098`). **Fifteen units and one ruling**, every unit
merged on its own green hosted head: E units 1–5 under single style
reviews (#2123, #2122, #2141, #2129, #2155); unit K, the K-floor
ruling with no floor (#2149); H units 6, 7, 9, 11, 10, 8, 12, 14, 15
under nine v6 duals (#2215, #2483, #2491, #2495, #2497, #2505, #2508,
#2509, #2514) — ordinals 2900–2908 in review-dispatch order, samples
#176, #177, #181, #182, #183, #185, #184, #218, #219; zero counted
tally candidates in nine duals, three recorded and excluded (units 9,
14, 15). Unit 13 (`S90-impl`) never ran, blocked on PROPS' `H5`; it
moves to CARVE with the classification BLEND owed named as its first
step. Band 2900–2999 stays claimed and is closed at 2908. Per the
sweep-5 rule the directory leaves whole — `program.md`, `plan.md`,
`log.md`, `logs/` (the eighteen stored review briefs), and the
twenty-three closed rows — with every OPEN row re-homed first: thirty-two
by the cut of 2026-09-17 (#2810) and four on the walk's own PR (#2826).

| program | title | closed | done-state of record |
| --- | --- | --- | --- |
| `blend` | BLEND — the blend kernel and the profile fillet door | 2026-09-17 | this entry; the walk at the sweep SHA; the code it left — `geom_brep::must_carry_over_edge` as the one description rule three verbs compose, the blend surgery's `split_fragment`/`retire_fragment` with the debug postcondition over both `Retired` arenas, `battery::Junction`, the ring-clearance forms, the path fillet door asking the validator's own classifier of its stored form with its `FILLET_*` recourses routed through one map and `geom_core::MissingRecourse` as the one fall-through, `test_utils::source::predicate_census`; the A/B record at ordinals 2900–2908 and blocks BLEND-B1–B3 in `docs/MODEL-AB-LOG.md` |

### What survived, and where

The program's output is the tree, not its directory:

- **The doctrine.** One description rule (`must_carry_over_edge`)
  composed by the extrude, revolve and blend arms, each storing its own
  conventional description (units 9, 14); a recourse sentence true at
  every site its predicate fires (README A3-2), keyed through one map
  with one gap sentence and a source-side roster of the deliberately
  unrouted (units 10, 12, 15); a retirement names a source key, enforced
  by a postcondition rather than a walk (unit 8); a fresh-key or
  "unmeasured" premise is measured before it is built on (every H unit).
- **The code.** `CircleMargins` and the hostless rim's closed-form
  clearance (6); `battery::Junction` (7); `split_fragment`,
  `retire_fragment`, the surgery postcondition (8);
  `must_carry_over_edge` returning one verdict (9); the two `PathError`
  arms and the door's stored-form check (10); arm-collects, door-picks
  through `fillet_select::nearest_candidate` (11); the fillet arm on
  `PathError::Escalated` through `fillet_recourse_for`,
  `EscalationSite::Fillet` retired (12); the blend's contact edges
  through the rule with `FILLET3_CONTACT_RECOURSE` conditioned by the
  site (14); `geom_core::MissingRecourse`, `validate::SHARED_CLAUSE_ONLY`,
  `test_utils::source::predicate_census` and the roster and pairing rows
  (15); the K-floor special case removed (K).
- **The measured bounds, stated rather than overpromised.** The
  `validate.rs` near-tangency addendum stays as `profile`'s second
  predicate-keyed table with its reason at the site; the sweep-side
  unrouted list stays test-side; the annulus rim phase keeps a second
  spelling of the split provenance, measured and filed (CARVE's row);
  every spec's overturned sentence is in its per-merge ledger entry.
- **A successor program.** `work/carve/` — CARVE, what a sweep verb
  builds and how it describes it — opened by the cut of 2026-09-17 per
  `work/README.md`'s rule, holding twenty-five rows and the band
  5600–5699.

### Residue re-homed before the deletion

Thirty-two open rows moved by the cut (#2810), each by `git mv` with
its body unchanged: twenty-one to CARVE, eight to PATHS (the profile
fillet door), one each to SYM, TOPO and META — the table and the
charters are in the sweep-SHA `log.md`'s cut entry. Four more moved on
the walk's PR (#2826), each with a "Re-homed at BLEND's exit" section:
`S90-impl`, `contact-edge-arm-is-picked-from-the-carrier-kind-not-the-dihedral`,
`ruled-band-keys-a-d-hole-rim-on-the-caps-outer-cycle` and
`corner-config-recourse-and-policy-assert-a-default-for-any-tag`, all
to CARVE. The two docs rows closed in place (#2811). Nothing was left
in `work/blend/` but the program files, the stored briefs and the
closed rows.

### Deleted at this sweep

`work/blend/program.md`, `plan.md`, `log.md`, `logs/` (eighteen
review briefs), and the twenty-three closed rows:
`ambiguity-k-below-the-cap-rim-crossover`,
`assembly-recourse-omits-the-transverse-cap-open-chain`,
`blend-contact-edges-mint-the-intrinsic-description-without-the-rule`,
`blend-recourses-under-describe-their-doors`,
`closed-chain-junctions-pair-with-a-rotated-link`,
`containment-margin-backstop-unreachable-behind-the-screen`,
`curved-single-host-rim-refuses-at-the-half-band-gate`,
`escalated-convexity-sign-renders-a-flip-that-was-never-decided`,
`escalated-recourse-dispatch-has-no-coaxiality-arm`,
`escalation-recourse-dispatch-has-three-homes`,
`extrude-cap-rim-argument-and-k-star-have-five-homes`,
`fillet-escalation-site-has-no-producer`,
`hostless-rim-on-a-ringed-host-refuses`,
`kernel-verbs-cap-pair-ulp-claim-stale`,
`ladder-rim-phase-may-retire-a-new-split-key`,
`overrun-attribution-picks-the-first-candidate`,
`path-fillet-door-validator-tangency-disagree`,
`rim-seed-finders-disagree-on-at-this-radius`,
`ring-clearance-refuses-a-nested-trim-circle`,
`smooth-arm-siblings-disagree-on-the-in-band-case`,
`sweep-doc-comments-cite-tests-unenforced`,
`sweep-top-field-docs-make-the-spatial-claim-capend-shed`; and
`docs/BLEND-EXIT-WALK.md`.

## Sweep 16 — 2026-09-16: S-MESH leaves the tracker

Sweep SHA: `9f043ec2712b880f26879183b6f9dc828abe0254` — the commit
immediately before the deletion (on the closing PR's branch, reachable
from `main` through that PR's merge commit; it is the state in which
S-MESH's directory is complete, `program.md` reads `status: closed`,
and every row in it is closed), so every path below is recoverable at
`git show 9f043ec2712b:work/mesh/<FILE>`,
`git show 9f043ec2712b:docs/S-MESH-EXIT-WALK.md` and
`git show 9f043ec2712b:docs/MESH-12-SPEC.md`.

S-MESH — mesh honesty and budget — opened 2026-08-31 from the ratified
stream cut (`docs/WORK-STREAMS-2026-08.md` §S-MESH) and closed
2026-09-16 on the walk Ev ratified on its PR (#2776, "lgtm", merged
`6b1efa457`). **Eleven units**, every one merged on its own green
hosted head with a v6 dual: MESH-1 (#1389), MESH-2 (#1421), MESH-3
(#1460), MESH-5 (#1507), MESH-4 (#1517), MESH-6 (#1545), MESH-7
(#1565), MESH-8 (#1585), MESH-10 (#1595), MESH-11 (#1599), MESH-12
(#1617) — ordinals 1200–1210 in that dispatch order (MESH-5 at 1203
before MESH-4 at 1204), samples #76, #82, #88, #92, #96, #101, #106,
#110, #112, #113, #157; no tally candidate in eleven duals. MESH-9
never ran: parked on issue 950 behind its typed trigger, it moves to
TESS parked. Three rulings ratified in-program (Ev, in chat,
2026-09-01): Q1, S65 stays compiled out; Q2, option (d) — the
input-quality detectors relocate body-side; Q3, explicit doors and no
transitive floor. Per the sweep-5 rule the directory leaves whole —
`program.md`, `plan.md`, `log.md`, the MESH-12 and MESH-R rows and the
three closed issue rows — with every OPEN row re-homed first on the
walk's own PR (below).

| program | title | closed | done-state of record |
| --- | --- | --- | --- |
| `mesh` | S-MESH — mesh honesty and budget | 2026-09-16 | this entry; the walk at the sweep SHA; the code it left — `crates/mesh`'s `Eps` operations, `props::require_iso_rectangle` and `require_one_chart_branch`, `topo::coherence::examine_chart_coherence`, `props_meridian_span_winding`; the A/B record at ordinals 1200–1210 in `docs/MODEL-AB-LOG.md` |

### What survived, and where

The program's output is the tree, not its directory:

- **The doctrine.** ε as named operations on a type (MESH-4's `Eps`
  newtype — separates / coincident / dominates / pad); the never-infer
  guard shape — structural rungs only, the identified side never
  asserted from values (MESH-3); shape predicates as explicit doors
  each consumer cites, under Q3 (MESH-7, MESH-11); body-data coherence
  examined body-side and non-gating, under Q2 (MESH-8); the MESH-4
  two-build digest as the D9 instrument every later unit in both
  programs ran.
- **The code.** The walk's loop-area fold anchored at the loop's own
  bbox centre (MESH-1); the chart frame's structurally-zero far-point
  v and spade's underflow floor (MESH-2); the undeclared-pole guard in
  `walk::loop_polygon` (MESH-3); the `nu == 1` one-strip schedule,
  decided by measurement (MESH-5); the two mechanical
  `cfg(debug_assertions)` censuses for S65's uncovered cases (MESH-6);
  `props::require_iso_rectangle` and `require_one_chart_branch`
  (MESH-7, MESH-11); `topo::coherence::examine_chart_coherence`
  (MESH-8); the split-meridian lineage fold in `torus_parse`
  (MESH-10); `props_meridian_span_winding`, the typed refusal for a
  sphere meridian span past the winding bound (MESH-12).
- **The measured bounds, stated rather than overpromised.** The
  one-element grid's axes still drop the schedule (issue 1513, TESS's
  row); a rim-only sphere cap still panics at the census (issue 1615,
  un-parked at MESH-12, TESS's row); the stored spans are read raw
  past the winding bound (issue 1618, PROPS' row); the two-argument ε
  form was disclosed at MESH-4's fix pass as unadoptable without
  moving bytes.
- **A successor program.** `work/tess/` — TESS, the tessellation
  kernel — opened on the walk's PR per `work/README.md`'s rule,
  holding the fourteen mesh findings, the parked MESH-9, the seven
  Track R rows and the band 5100–5199.

### Residue re-homed before the deletion

Twenty-seven open rows moved on the walk's PR (#2776, earlier commits
than this deletion), each carrying a "Re-homed at S-MESH's exit" note,
ids unchanged, the Track R rows' `parent: MESH-R` dropped: fourteen
mesh findings (among them `rim-chords-exceed-snapped-column-count`,
MESH-9's trigger, and VIEW's rider
`degenerate-normal-rows-model-resolution-cites-a-deleted-helper`,
filed into `work/mesh/` after the walk was cut and moved beside its
parent), MESH-9 itself and the seven Track R rows S28, S236, S237,
D300, D303, D304, C23 to `work/tess/`;
`stored-spans-read-raw-past-winding-bound` (issue 1618) and the two
`props/quad.rs` rows C3 and D30 to `work/props/`;
`cert10-strict-gap-floor-gates-on-a-varying-seed` and
`sentinel-markers-with-no-reader-are-grep-only` to `work/tint/`.
MESH-R closed as dissolved. Nothing else was open.

This sweep retires, on three rows, the `refs:` entries that named rows
leaving with the directory (MESH-12, MESH-R,
`saturated-sphere-span-folds-short`,
`rim-continuation-witness-fixture-needed`,
`mesh-cert10-fold-fuzz-row-flakes-on-a-fresh-seed`):
`work/props/stored-spans-read-raw-past-winding-bound`,
`work/tess/rim-only-sphere-cap-panics-at-census` and
`work/tint/fuzz-rows-discard-trials-against-a-floor-that-counts-them`
(whose only ref it was; a sentence naming the sweep SHA replaces it).
The rows are otherwise untouched.

Filed by S-MESH's units on other programs' slates and untouched by the
sweep: `work/fix/coherence-findings-have-no-consumer` (1587, closed
since), `work/topo/graft-copies-provenance-keys-verbatim` (1597),
`work/props/two-face-sphere-split-measures-zero-volume` (1598, closed
since), `work/props/props-refusal-cannot-carry-measured-overshoot`
(1602), `work/tint/pick-face-fuzz-anti-vacuity-guard-trips-at-effort-1`
(the GUI-1 seeded fuzz guard that reddened MESH-12's landing, proven
not the PR's; filed on DOCM, re-homed by DOCM's sweep).

What opens with this sweep: `crates/mesh/*` and
`crates/topo/src/coherence.rs` are TESS's outright (its `program.md`
`paths` names them); the keep-out prose in FIX, INSTR, PIPE, PRED,
PROPS, TOPO and TRIM's `program.md`s that still names S-MESH as the
owner of that ground now means TESS and is each program's to re-word;
the tess-budget re-baseline stays PROPS' under the keep-out it
inherited from S-MESH's.

### The docs that moved with the program

| doc | from | to |
| --- | --- | --- |
| `S-MESH-EXIT-WALK.md` | `docs/` | deleted with this sweep; recoverable at the sweep SHA |
| `MESH-12-SPEC.md` | `docs/` | deleted with this sweep (the one binding spec still in `docs/` — its unit merged 2026-09-08 and the spec outlived the merge; the earlier ten left `docs/` under the standing per-unit rule, listed above); recoverable at the sweep SHA |

## Sweep 15 — 2026-09-16: S-BOOL leaves the tracker

Sweep SHA: `32082e8a24fd826d75fc2529bc2a0468b6430f01` — the commit
immediately before the deletion (on the closing PR's branch, reachable
from `main` through that PR's merge commit; it is the state in which
S-BOOL's directory is complete, `program.md` reads `status: closed`,
and every row in it is closed), so every path below is recoverable at
`git show 32082e8a24fd:work/bool/<FILE>`,
`git show 32082e8a24fd:docs/S-BOOL-EXIT-WALK.md` and
`git show 32082e8a24fd:docs/BOOL-<n>-SPEC.md`.

S-BOOL — boolean reach and containment — opened 2026-08-31 from the
ratified stream cut (`docs/WORK-STREAMS-2026-08.md` §S-BOOL) and closed
2026-09-16 on the walk Ev ratified on its PR (#2775, "lgtm!", merged
`3e4e0c8a3`). **Thirteen units**, every one merged on its own green
hosted head with a v6 dual: BOOL-1 (#1378), BOOL-2 (#1425), BOOL-3
(#1464), BOOL-8 (#1508), BOOL-11 (#1520), BOOL-13 (#1553), BOOL-12
(#1573), BOOL-9 (#2134), BOOL-10 (#2135), BOOL-5 (#2748), BOOL-6
(#2752), BOOL-7 (#2755), BOOL-4 (#2767) — ordinals 1100–1112, samples
#75, #84, #91, #93, #100, #104, #156, #164, #212, #214, #215, #216,
#217; window tally BOOL-9 +1 fable, BOOL-10 +1 opus, BOOL-7 +1 fable.
Per the sweep-5 rule the directory leaves whole — `program.md`,
`plan.md`, `log.md`, seven unit rows and the closed issue rows — with
every OPEN row re-homed first on the walk's own PR (below).

| program | title | closed | done-state of record |
| --- | --- | --- | --- |
| `bool` | S-BOOL — boolean reach and containment | 2026-09-16 | this entry; the walk at the sweep SHA; the design at `docs/PATHS-DESIGN.md` §3/§4 (the Q1 chain's re-wordings, ratified in-chat 2026-09-01 and 2026-09-13), `crates/editor-core/ASSEMBLY.md` (interference decided by the material test) and `crates/topo/src/census.rs`'s arm-2 doc; the A/B record at ordinals 1100–1112 in `docs/MODEL-AB-LOG.md` |

### What survived, and where

The program's output is the tree, not its directory:

- **The doctrine.** The Q1 ruling chain in `docs/PATHS-DESIGN.md`
  §3/§4 — the straight continuation and the declared point-target
  continuation as structural joints, the declared arrival at the seam,
  the vertex table as a cache with authoring through the lattice only,
  `arc_continue` retired — under the sixth-round ruling (every
  zero-turn joint is a declared tangent joint; the lattice never asks
  whether carriers are the same). The declared-split arc form was
  built, reviewed and then declined on its cost; it is history at
  `f79fa7081`, not main.
- **The code.** `point_in_solid`'s cone and torus arms with the grazing
  posture escalating (BOOL-2/3); the coplanar-split citations restated
  (BOOL-1); the schema demolition (BOOL-13); the rim-free spherical
  wedge's props arm with `props_wedge_azimuth` and `props_band_opposite`
  (BOOL-5); the per-slab stacking fold in `loft.rs` (BOOL-6); the
  vdiff shadow-exec rung, per partner at the boolean's operand with its
  two halves recorded as limits (BOOL-7); the census's material
  containment test over a per-solid point-in-solid entry, with
  `InstanceInterference` as the decided refusal (BOOL-4).
- **The measured bounds, stated rather than overpromised.** Curls past
  a full turn build self-overlapping bodies with every tier silent
  (BLEND's row); the shadow-exec rung recovers the pruned-pair half of
  a `SideOf` vanish and neither the collapse half nor the OrderAlong
  half (WIRE's rows); the material test admits vertex-on-face and
  edge-in-face touches only as locally one-sided rests and blocks the
  other touch kinds, and the box gate still clears a partial overlap
  whose boundaries meet only in touches (CURVED's rows).
- **A successor program.** `work/paths/` — PATHS, the profile lattice —
  opened on the walk's PR per `work/README.md`'s rule, holding the
  eight lattice rows and the band 5000–5099.

### Residue re-homed before the deletion

Fifty-eight open rows moved on the walk's PR (#2775, an earlier commit
than this deletion), each carrying a "Re-homed at S-BOOL's exit"
note, ids unchanged, the Track Q rows' `parent: BOOL-Q` dropped:
twenty-three boolean, containment, join and declaration rows and six
Track Q rows (D280, D284, D95, G9, S173, S234) to `work/curved/` (its
charter inherits S-BOOL's ceded ground at this exit); eight lattice
rows to `work/paths/`; four to `work/topo/` (the provenance graft, the
two `topo::split` rows, the deferred void-birth marking); six to
`work/blend/` (the profile fillet door's three, the two loft findings,
the subdivided-side lowering — `crates/sweep/src/loft.rs` added to
BLEND's `paths`); one each to `work/guard/` (the raw-door gate),
`work/lib/` (the Python refusal-predicate pins) and `work/wire/` (the
collapse-half limit); D46, D57, D281 to `work/pred/`; D287, D66 to
`work/tint/`; H11 to `work/props/`; the two heat-sink demo rows to
`work/issues/` (demos/tour is in no program's `paths`). BOOL-Q closed
as dissolved; BOOL-4 and issue 750 closed at BOOL-4's merge. The
descendant-cycle repro patch travelled with the CURVED row that cites
it. Nothing else was open.

Filed by S-BOOL's units on other programs' slates and untouched by the
sweep: `work/props/certificate-types-have-public-fields-and-are-forgeable`,
`work/props/sphere-wedge-arm-does-not-fold-split-meridians-by-lineage`,
`work/props/props-curved-carries-two-readings-of-d9-unreachable-vs-poison`,
`work/props/sphere-flux-arm-refuses-partial-bands` (re-scoped),
`work/blend/skin-coincident-section-check-is-an-unbanded-f64-compare`
(re-scoped), `work/wire/order-along-qualifier-records-no-partner-so-its-pruned-pair-vanish-cannot-be-recovered`,
`work/lib/north-star-audit-verb-list-names-arc-continue`,
`work/issues/klein-scene-should-adopt-the-one-body-loop-sweep`,
`work/docm/pick-face-fuzz-anti-vacuity-guard-trips-at-effort-1` (since
re-homed by DOCM's sweep), `work/curved/partial-overlap-with-touch-only-boundaries-clears-at-the-census-gate`,
`work/curved/touch-kinds-without-a-local-side-analysis-block-the-material-test`,
`work/topo/an-inside-out-part-passes-tier-3-because-only-the-body-total-volume-is-pinned`.

What opens with this sweep: `crates/topo/src/boolean/*` and
`splitting/*` are CURVED's outright (the fence both `program.md`s
carried dissolves; CURVED's own keep-out prose still names it and is
CURVED's to re-word); `crates/editor-core/src/resolve/vdiff.rs` is
EDIT's (`resolve/` is EDIT's territory); `crates/profile/*` is PATHS';
`crates/sweep/src/loft.rs` is BLEND's.

### The docs that moved with the program

| doc | from | to |
| --- | --- | --- |
| `S-BOOL-EXIT-WALK.md` | `docs/` | deleted with this sweep; recoverable at the sweep SHA |
| `BOOL-4-SPEC.md`, `BOOL-5-SPEC.md`, `BOOL-6-SPEC.md`, `BOOL-7-SPEC.md`, `BOOL-9-SPEC.md`, `BOOL-10-SPEC.md`, `BOOL-12-SPEC.md` | `docs/` | deleted with this sweep (the seven binding specs whose units merged; the earlier six left `docs/` at their merges); recoverable at the sweep SHA |

## Sweep 14 — 2026-09-14: DOCM leaves the tracker

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
deletion entries above name each unit head).

## Sweep 13 — 2026-09-13: M10 leaves the tracker

Sweep SHA: `bfafe4ad10286e1624f23a1100b5cb9c2fe1d7d6` — the commit
immediately before the deletion (on the closing PR's branch, reachable
from `main` through that PR's merge commit; it is the state in which
M10's directory is complete, `program.md` reads `status: closed`, and
every row in it is closed), so every path below is recoverable at
`git show bfafe4ad10286e1624f23a1100b5cb9c2fe1d7d6:work/m10/<FILE>` and
`git show bfafe4ad10286e1624f23a1100b5cb9c2fe1d7d6:docs/M10-EXIT-WALK.md`.

M10 — the error-propagation MVP — opened 2026-08-29 and closed
2026-09-13 on Ev's ratification of `docs/M10-EXIT-WALK.md` (PR #1700,
in chat: approve the walk and do the exit sweep; the orchestrator that
re-cut the walk had stopped and a successor session carried both).
**Thirteen units**, every one merged on its own green hosted head:
M10-D (#1146), M10-DI (#1154), M10-1 (#1147), M10-P (#1174), M10-2
(#1213), M10-3 (#1231), M10-4 (#1627), M10-5 (#1638), M10-6 (#1685),
M10-7 (#1725), M10-8 (#1828), M10-9 (#2048), M10-10 (#2100). Per the
sweep-5 rule the directory leaves whole — `program.md`, `plan.md`,
`log.md`, the seven unit rows still in it (`M10-4` … `M10-10`) and
twelve closed issue rows — with the twenty-two OPEN rows re-homed
first, below.

| program | title | closed | done-state of record |
| --- | --- | --- | --- |
| `m10` | M10 — the error-propagation MVP | 2026-09-13 | this entry; the walk at the sweep SHA; the design at `docs/ERROR-DESIGN.md` (E1–E12, ratified) and `docs/DUAL-DESIGN.md` (DL1–DL6); the A/B record at ordinals 500–511 in `docs/MODEL-AB-LOG.md` |

### What survived, and where

The program's output is the tree, not its directory:

- **The design.** `docs/ERROR-DESIGN.md` E1–E12 and its E3 amendments
  (E12 ratified #1712), and `docs/DUAL-DESIGN.md` DL1–DL6 (#1146,
  which closed the D1 hedge: a dual is tangent transport and never
  certifies). Both are PROPS' files from this sweep.
- **The code.** Distributions, the `Measure` sink and report-only
  `Assertion`s, the E6 subdivision driver, the `Dual` sensitivities
  and the certified-worst-case stackup, the E7 clearance
  trichotomy, the E10/E11 reporting rows, and the E12 symbolic
  identity tier (`geom_core::sym`) with its registered-identity door.
- **The gates.** Three E10 CI rows, live and read by STEP conclusion
  and by LOG: assertion gating over every registered document, the
  goldened ε-keyed accounting, and the driver K population (rule 1 —
  an in-band indeterminate — measured 0 at every ε row, not
  demotable by any caller). The K addenda are in `docs/K-REPORT.md`,
  INSTR's file.
- **The demo.** `demos/tour/src/tolerance.rs`, two stops through
  public doors, with its own test inside `ci.yml`'s tour step
  asserting the numbers the captions print.
- **A successor program.** `work/sym/` — SYM, the E12 symbolic
  identity tier — opened in this sweep on Ev's call, holding fourteen
  of the twenty-two rows and the band 4700–4799.

### The numbers the walk carried, because nothing else now does

Criterion 11 — Ev's condition for closing the program, "a macroscopic
box certifies: the two-hole plate's real study returns certified
leaves bounded by genuine flips, not by ε". At the LEAF: the plate's
real study (±0.05 mm on the spacing, σ = 0.01 mm on the radii) driven
whole at 1,024 leaves certifies **431** leaves and refuses 593 on
budget — **89.07 %** of the mass, a certified worst-case hull
`[0.419, 0.845]` mm against the 0.5 mm floor — and every refused leaf,
refined to any depth by both reviewers, is bounded by the document's
own `assert_bound`: the real flip, which enters the box at 0.625 of
the study. Identical to the digit at three ε rows. At the CEILING: the
widest box that certifies WHOLE is **0.263** of the study (0.237 at
ε = 1e-6), bounded by dependency widening of the assertion's own
affine margin, which subdivision resolves — the class now at
`work/sym/real-margin-dependency-widening`.

**Verified at the sweep by execution**, not read off the walk: the
tour's own gating row (`the_two_stops_say_what_their_captions_say`,
the row that puts the cell inside `ci.yml`) ran green on the sweep
head at the default ε — `Receipt { certified: 193, refused: 319,
splits: 511 }`, holds 0.8337 of the tolerance mass, violated 0.0002,
unresolved 0.1661, in 320.6 s — which is criterion 8's and honesty
row 18's arithmetic to the digit.

### Residue re-homed before the deletion

Twenty-two open rows. The moves are earlier commits of the closing PR,
so the deleting commit finds the directory holding only closed work.
Every id is unchanged and every row carries a note in its own body
saying why it landed where it did. **Ev's call at the sweep** (in
chat, 2026-09-13): "the 14 to a successor program, the others to
either another successor program or a preexisting program" — the
fourteen rows standing on one territory being what `work/README.md`
calls a successor's opening slate rather than residue.

| item | to | why |
| --- | --- | --- |
| `real-margin-dependency-widening` | `work/sym/` | the numeric channel's half of E12's division of labour — what stands between the plate's 0.263 ceiling and the flip at 0.625 |
| `plate-ceiling-is-now-the-scaffold-pushforward` | `work/sym/` | one predicate bounds all five measured documents; the fix half is a PCURVE/D3 question and `geom-brep/src/certify.rs` is in no program's paths — TRIM is PCURVE's successor |
| `rule-d-reaches-the-unit-bulge-only` | `work/sym/` | rule D is the tier's, and this is the next ceiling class after M10-10 |
| `interval-self-dot-straddles-before-rule-a` | `work/sym/` | rule A's reach; the FIX is `powi(2)` in PROPS' `linalg/vec.rs` and is announced there — PROPS' linalg lane may take it |
| `param-box-certification-of-implicit-quantities` | `work/sym/` | the tier's frontier: an iterated quantity has no expression in the parameters. Came to M10 from S-CERT; follows the tier |
| `declared-tangency-needs-the-registered-identity-door` | `work/sym/` | the door's live consumer; open rather than parked, and it waits on BLEND's fillet row |
| `the-span-identity-is-not-a-theorem-of-the-floats` | `work/sym/` | the limit of what a registration means; `register_equal` is in PROPS' `real.rs`, reached by announced seam |
| `the-witness-slack-is-eps-independent` | `work/sym/` | the same door and the same seam: `WITNESS_REL` does not move with the run's ε |
| `sym-registration-flattens-two-axes` | `work/sym/` | the door's shape and its cost, on the record |
| `symbolic-tier-costs-95-percent-of-the-m10-3-drive` | `work/sym/` | the work it asks for is a profile INSIDE the normal form. **The measurement stays S-TCOST's** and is named as such in the row |
| `derived-frame-placement-freezes-on-the-symbolic-lane` | `work/sym/` | DOCM-1's review found it; every freeze is a budget refusal in `sym::form_in`, so the row follows the mechanism and names DOCM |
| `symbolic-tier-census` | `work/sym/` | the tier's own 107-row reference, which `sym.rs`'s module docs cite |
| `sym-rs-is-one-file-with-a-347-line-header` | `work/sym/` | this program's file and nothing else's |
| `registered-is-spelled-five-times-and-pinned-once` | `work/sym/` | four of the five spellings are the tier's; the fifth is INSTR's k-lint column and is named |
| `certified-hull-padding-is-the-leaf-width-not-the-lane` | `work/props/` | the hull is `stackup.rs`'s, PROPS' already; the tier is the cause and not the site |
| `coincidence-zone-priced-budget-at-the-floor` | `work/props/` | a `RefusalReason` arm in `drive::classify_replay`; already `refs` PROPS' `k-stats-escalation-channel-and-redo` |
| `min-clearance-refusal-stringly-twin` | `work/props/` | the twin is in `measure.rs`; the layering question it waits on is the 1055 valve's seam |
| `mc-lanes-draws-are-not-reproducible-from-outside-the-crate` | `work/props/` | `mc.rs` is the advisory half of the analysis lane; the `pncad` re-export half is LIB's and is named |
| `fillet-tangency-is-not-the-constructors-node` | `work/blend/` | the profile fillet door: `build_seg` re-derives the centre the joint classifier needs. BLEND's title names that door and it inherited FILLET's residue |
| `revolve-carriers-state-only-the-rim` | `work/blend/` | `sweep/src/revolve/*` is BLEND's paths and what is owed is a constructor change |
| `symbolic-tier-and-clearance-engine` | `work/shell/` | `min_separation` is concrete at `Interval` in SHELL's `clearance.rs`; SHELL-3 is the same question from the other end |
| `pncad-py-eval-err-variants-outside-the-tag-inventory` | `work/census/` | CENSUS's class exactly — a vocabulary spelled by hand and the census that cannot see one spelling; `pncad-py` is LIB's and is announced |

**The territory moved with the rows.** PROPS takes the analysis lane
its `keep_out` had named since 2026-09-06 — `analysis.rs`,
`distribution.rs`, `drive.rs`, `measure.rs`, `mc.rs`, the `m10*` and
`e4_dual*` suites, `ERROR-DESIGN.md` and `DUAL-DESIGN.md`. SYM takes
`geom-core/src/sym.rs`, `sym/*` and `geom-core/tests/m10_*`, which is
a double claim inside PROPS' `geom-core/src/*` glob, written on BOTH
sides in the opening commit as `work/README.md` requires.
`crates/bvh/src/*`, which PROPS' clause had parked on M10, is in no
program's paths now and PROPS' `keep_out` says so.

### The inbound pointers this sweep rewrote

Every live `work/m10/…` citation in the tree, in three classes:

- **a row this sweep re-homed** — repointed at its new directory, the
  id unchanged: `geom-core/src/sym.rs`, `real.rs`, `k_stats.rs` and
  `Cargo.toml`, the `sweep` carriers, `editor-core`'s clearance and
  measure and its m10 suites, `demos/tour/src/tolerance.rs`,
  `docs/ERROR-DESIGN.md`, `docs/K-REPORT.md`,
  `scripts/gates/register-equal-allowlist.sh`,
  `crates/pncad-py/tests/test_binding_census.py`, and eight tracker
  rows;
- **a row re-homed at an EARLIER sweep**, so the citation was already
  stale: `signed-penetration-depth` (CURVED),
  `clearance-window-tightening-needs-chart-boundary` (TRIM),
  `contribution-bounds-via-dual-interval` (PROPS). Two of those are
  the pointers `docs/TRIM-3-SPEC.md` item 8 asks TRIM-3 to correct as
  a rider; **that rider is now a no-op** and the spec was left alone
  rather than edited under a dispatched lane;
- **a row closed WITH the program**, or M10's own plan or log — named
  as M10's closed row with this ledger entry as the way back. Eleven
  sites in `crates/` say so now (`first-refusal-at-twice-the-ceiling-
  is-an-order-artefact` ×5, `plate-rim-residual-needs-the-wide-
  coefficient-ring` ×3, `plate-ceiling-is-now-the-arc-span-identity`,
  `rule-d-multiple-reader-wraps-on-a-huge-dyadic-coefficient` and
  `M10-8`), and eleven more across seven tracker rows.

One citation was FALSE rather than stale and was fixed:
`work/props/three-per-node-verdict-shapes` said `drive.rs` is edited
by an announced seam because it is M10 territory. It is PROPS' own
from this sweep, so the seam is discharged.

**Eight header `refs:` broke on the unit rows** and `lint` caught
every one — this is the case CITE's sweep did not have. `M10-4`,
`M10-5`, `M10-7`, `M10-8` and `M10-10` were named by eight surviving
rows across BOOL, PROPS, SHELL and SYM, and each id was replaced with
that unit's **PR number** (1627, 1638, 1725, 1828, 2100), which is an
int the tracker does not resolve and which is where the unit's
documentation actually lives. Nothing else in those headers moved.

**Left as history, deliberately**: the provenance lines in programs'
logs ("`X` from `work/m10/`"), `docs/MODEL-AB-LOG.md`'s unit rows, and
the nine per-merge deletion entries above that cite
`work/m10/log.md` or `work/m10/M10-*.md` as a unit's statement of
record. Each states what was true when it was written, and every one
of those paths is recoverable at this entry's sweep SHA.

### Honesty notes

- **The walk was ratified by a session that did not write it.** Ev's
  word was to approve #1700 and sweep; this session read the walk,
  verified the one claim it could verify in reasonable time by
  execution (the tour's gating row, above), and did not re-run the
  1,024-leaf study or the hosted K rows. Criterion 11's 431/89.07 %
  and the 0.263 ceiling are the walk's numbers, carried forward here
  on the evidence rows it cites, not re-measured at the sweep.
- **Fourteen rows against eight is not a tidy split, and the fourteen
  are not finished work.** They are what four units on the symbolic
  tier left standing: three documents that do not certify their
  studies at any affordable dial, a door with two unbuilt registrants,
  a tier at 95 % of the interval drive that nobody has profiled
  inside, and one file of 3,898 lines. A program opened on that slate
  starts in debt, which is the honest shape of it.
- **Two rows SYM's door needs are not SYM's.** The fillet's declared
  tangency and the revolve carriers' span identity are constructor
  changes in `crates/profile` and `crates/sweep`, so they went to
  BLEND; SYM holds the consumer of the first and cannot finish it
  alone. The pairing is stated in both directions (`work/sym/log.md`,
  `work/blend/log.md`) rather than left for whoever picks one up.
- **`work/m10/plan.md` is not quoted anywhere.** The walk quoted its
  criterion rows verbatim and the walk is recoverable at the sweep
  SHA; the plan itself, with its substrate inventory and its lane
  order, is recoverable there too and is nowhere else.
- **The A/B band 500–599 stays claimed and closed**, on the VIEW and
  S-TCOST precedent: thirteen dual reviews at ordinals 500–511 (M10-D
  was a design pass), samples #39, #40, #43, #49, #50, #114, #115,
  #118, #124, #145, #150 and #173, with three symmetric tally pairs.
  Nothing renumbers.
## Per-merge deletion — BLEND-10's spec (2026-09-13)

Recoverable at `git show 99a9d12448dcc47bbe1f029c0ec084b58716a167:docs/BLEND-10-SPEC.md`
(the fix-pass head). Three of its sentences did not survive
measurement: its hypothesis named reconstruction error where the loss
is the stored chord-plus-bulge form flattening to a line below
`θ* = √(8ε/r)` (and a second, first-order-in-radius loss the reviewers
found); its recourse said "a smaller radius", which makes the sagitta
worse; and its "before the arc is emitted" cannot run, since the
outgoing joint's second segment does not exist at emission. All three
are recorded in the PR body and the unit's log entry.

- `BLEND-10-SPEC.md` — BLEND-10, the path fillet door never mints a joint the validator refuses (#2497)

## Per-merge deletion — BLEND-12's spec (2026-09-13)

Recoverable at `git show 594f484787444628d3b932f6933b1686e2fca9e1:docs/BLEND-12-SPEC.md`
(the fix-pass head). Its Phase 1 table did not survive the dual: two of
the five gates the unit called pre-empted are driveable in band through
the public door at every ε row (`fillet_corner_turn`, whose margin is
levered by the leg's extent; `fillet_offset_lever`, whose threshold
grows with the corner's squared scale), so six of nine are driven and
the two exact-order gates are pre-empted only at scalar `f64`. The
retirement it asked for stands: `EscalationSite::Fillet` had no producer
and is gone with its arm. Recorded in the PR body and the unit's log
entry.

- `BLEND-12-SPEC.md` — BLEND-12, the fillet door's in-band verdicts render their own sentence (#2508)
## Per-merge deletion — BLEND-8's spec (2026-09-13)

Recoverable at `git show 968e8210415b03c0f75229119dfc6a73dfcd8c97:docs/BLEND-8-SPEC.md`
(the fix-pass head). Its premise did not survive Phase 1: the item
called the ladder phase's fresh-key retirement "unmeasured", and the
spec asked for it to be measured, but it was LIVE on 14 shipped rows at
the merge base (a revolve-minted cap seam running pole → rim) — the
unit closed a defect the tree already exhibited. Two more sentences
fell to measurement: its door (b) (a profile authored in reverse) is
unreachable because `profile` canonicalizes traversal, and its
document-layer row cannot exist — the witness body is not authorable
through the recipe layer and `emit_blend`'s retired-set guard cannot
observe the defect (filed for EVAL). All three are recorded in the PR
body and the unit's log entry.

- `BLEND-8-SPEC.md` — BLEND-8, the ladder rim phase never retires a fresh split key (#2505)

## Per-merge deletion — BLEND-14's spec (2026-09-13)

Recoverable at `git show 253183c0dbc63bb9d4a1dd98f6732d317254da04:docs/BLEND-14-SPEC.md`
(the fix-pass head). Its claim that the rule is reachable on the
near-osculating family did not survive Phase 1 — the clearance screen
refuses that family before the rule can read it, and the rule is
reached in band through the public door only at radii of a few `K·ε`
or on a slim wedge whose corner arcs' extent is the folded arm (the
lever its out-of-scope clause asked to be named, which the dual named);
its `UnderDetermined` arm, described as a lane-refused pair's, is
reached instead by an admitted pair whose jet is under-determined.
Recorded in the PR body and the unit's log entry.

- `BLEND-14-SPEC.md` — BLEND-14, the blend's contact edges carry the tangency rule (#2509)

## Per-merge deletion — BLEND-15's spec (2026-09-17)

Recoverable at `git show 575963daab1c85bfce1ee7e097cf368d2ae73e1a:docs/BLEND-15-SPEC.md`
(the fix-pass head). Three of its sentences did not survive: the gap
sentence is a `Display` newtype in `geom-core`, not the constant it
asked for (the sentence interpolates the name mid-way); the roster it
specified as a per-crate census became one reader homed in
`test_utils::source` after both reviewers defeated the per-crate copies
by mutation; and its "one name, two sentences" list is measured within
each crate, not between them (three names in `profile` carry a
validator sentence and a door sentence, on purpose). Recorded in the PR
body and the unit's log entry.

- `BLEND-15-SPEC.md` — BLEND-15, an escalation's recourse is routed by one rule with one fall-through (#2514)

## Sweep 12 — 2026-09-12: CITE leaves the tracker

Sweep SHA: `116d96c01d4a03084d4701d7d58fb3b5dcf1703b` — the commit
immediately before the deletion (on the closing PR's branch, reachable
from `main` through that PR's merge commit; it is the state in which
CITE's directory is complete, `program.md` reads `status: closed`, and
every row in it is closed), so every path below is recoverable at
`git show 116d96c01d4a03084d4701d7d58fb3b5dcf1703b:work/cite/<FILE>` and
`git show 116d96c01d4a03084d4701d7d58fb3b5dcf1703b:docs/CITE-EXIT-WALK.md`.

CITE — citations, numbering and the paperwork a lane runs on — opened
2026-09-11 in the tracker cut of that day
(`docs/WORK-TRACKS-2026-09.md` addendum 3) and closed 2026-09-12 on Ev's
ratification of `docs/CITE-EXIT-WALK.md` (PR #2405). It claimed **no
paths**, by charter: its repair ground was `work/<program>/*.md`, which
is one-file-one-item ground. Per the sweep-5 rule the directory leaves
whole — `program.md`, `plan.md`, `log.md` and eight item files, all
`status: closed` at the sweep SHA (`C-namespace`, `S176`,
`build-slot-banner-leaks-the-holders-command-line`,
`code-quality-item-quotes-a-viewer-doc-string-that-was-rewritten`,
`d107-release-profile-job-lives-in-nightly`,
`doc-line-citations-rot-silently`,
`loud-skip-marker-row-cites-a-lib-paragraph-that-was-reversed`,
`tracker-file-line-citations-measured`); the other four rows the slate
held were re-homed first, below. **Two PRs** merged, #2397 (the
convention, the repairs and four rulings) and #2405 (the walk).
Infra-and-prose: **no A/B rows were written and no ordinal was spent**;
the band 4000–4099 stays claimed in `docs/MODEL-AB-LOG.md`.

| program | title | closed | done-state of record |
| --- | --- | --- | --- |
| `cite` | CITE — citations, numbering and the paperwork a lane runs on | 2026-09-12 | this row and the exit-walk row below; the two PRs above; the standing rule at `docs/prompts/implementer-discipline.md` §7 |

### What survived, and where

The program's output is deliberately not in its directory:

- **The rule.** `docs/prompts/implementer-discipline.md` §7 — *"Cite by
  name; line numbers rot. A number may ride along beside the name and is
  allowed to go stale; a bare `file.rs:NNN` is not a citation."* Landed
  2026-09-12 on Ev's wording, cut down by him twice from a three-
  paragraph draft. `docs/prompts/*` is META's territory; landed on Ev's
  word rather than taken.
- **Three repaired rows**, each on its owning program's slate with a
  marked section recording what the repair put in question and leaving
  the ruling to the owner:
  `work/door/viewer-pathverb-all-hand-written-seventeen.md`,
  `work/topo/D107.md`,
  `work/tint/loud-skip-marker-is-a-hand-kept-idiom.md`.
- **One leak closed.** `local-scripts/with-build-slot.sh` no longer
  records the caller's command line in the holder file, so the banner a
  waiting caller prints cannot carry another lane's test filter or
  scratch path between blinded review lanes.

### The measurement, because nothing else now carries it

`tracker-file-line-citations-measured` is deleted with the directory and
its figures are load-bearing for §7, so they are restated here. Taken
2026-09-09 at `0762714fd` over every `<path>:<line>` in every
`status: open` row of `work/**/*.md`, bare basenames resolved through
`git ls-files`:

| | count |
| --- | --- |
| citations, in 317 open rows across 22 programs | **1,508** |
| already anchored (a backticked identifier or quoted phrase within one line) | **1,446 (96%)** |
| unanchored | 62 (4%) |
| file missing or line past EOF | 31 (2.1%) |
| basename ambiguous, not checkable that way | 204 |

Per program the anchored figure runs 90–100%. **That 96% is why §7 is a
ratification of existing practice rather than a proposal**, and why the
walk declines both a sweep and a gate: a line-range check sees only the
2.1%, while VIEW's three hand-sweeps found roughly three quarters of the
citations they touched pointing at the wrong *subject*, nearly all of it
inside the column such a check passes.

### Residue re-homed before the deletion

The moves happened in #2397 and #2405, not in the deleting commit; the
deleting commit finds the directory already emptied of live work.

| item | to |
| --- | --- |
| `S351` | `work/trim/` — the placement rule it watches is in `crates/geom-brep/src/nurbs_iso.rs`, TRIM's `paths`. Checked at the move and **not fired**: both pointers resolve and both cite `nurbs_iso`'s module docs by name |
| `d321-row-number-reissued` | `work/meta/` — overtaken on both halves by sweep 11; the one surviving thing, a `work.py` check that an id is never reissued, is META's |
| `lane-scratchpad-is-shared-between-worktrees` | `work/meta/` — `deferred` by Ev (the rule is not project-specific); with the `memories/` half ruled out, `docs/prompts/*` is the only document left in play and it is META's |
| `no-local-script-builds-all-four-cargo-workspaces` | `work/ciw/` — the fix is in `local-scripts/*`, CIW's `paths`; CITE never started it |

### The four inbound pointers this sweep rewrote

All four named `work/cite/` paths and none was a header `refs:`, so no
row's references broke. They were repointed at what survives, in the
commit at the sweep SHA:

- `work/door/viewer-pathverb-…` and `work/topo/D107.md` cited
  `work/cite/S176.md` as the argument for cite-by-name → now cite §7,
  which states the rule;
- `work/topo/D107.md` cited `work/cite/d107-release-profile-job-lives-in-nightly`
  for the two line numbers that had drifted → now names it as a record
  recoverable through this ledger;
- sweep 11's two entries above cited `work/cite/plan.md` for `d321`'s
  disposition → now name `work/meta/d321-row-number-reissued.md`.

### Honesty notes

- **Four of the twelve rows were re-homed unstarted, not finished.** A
  program that opens twelve and works eight did not finish twelve; the
  four moved because they were somebody else's.
- **`S176`'s `Verdict:` was blank for three weeks and CITE filled it**,
  on the measurement rather than on a ruling from Ev. The row says so in
  its own words, and the row is now recoverable only at the sweep SHA.
- **The walk tables the plan's slate rather than quoting it.** The
  plan's table has a `where the work lands` column that is about
  dispatch and is not a criterion. The plan is recoverable at the sweep
  SHA.
- **The routing list the plan promised was never produced**, and the
  walk argues it should not have been: Ev authorised repair-in-place at
  the first ask, and the row that motivated the fence
  (`loud-skip-marker-…`) exists precisely because §6 reported the same
  rot twice and filed nothing both times. The distinction CITE leaves
  behind is that **repairing what a row POINTS AT is not the same act as
  ruling on what it CLAIMS**, and only the second needs the owner.

## Sweep 11 — 2026-09-11: code-quality leaves the tracker

Sweep SHA: `8851abb6daff4822f5a55c98e940c4c061223953` — the commit immediately before the deletion (the
`main` tip this PR branched from; it is the state in which
`work/code-quality/` is complete and every row in it closed, the 110
live ones having left the same day in the cut below), so every path here
is recoverable at
`git show 8851abb6daff4822f5a55c98e940c4c061223953:work/code-quality/<FILE>`
and `git show 8851abb6daff4822f5a55c98e940c4c061223953:work/issues/<FILE>`.

**code-quality** — *"where a structural finding waits until a program
claims it"* — opened 2026-08-18 as the tracker home of the 2026-08
structural-findings register (`docs/SMELL-SCAN-2026-08.md`, sweep 4) and
its Tracks K–X schedule, and closed 2026-09-11. It is the first program
to close **empty by design rather than by finishing its board**: it was
a holding ground, its charter said a row leaves the moment a program
claims it, and on 2026-09-11 all 110 of its remaining live rows and
`work/issues/`'s were claimed at once by eleven programs opened for them
(`docs/WORK-TRACKS-2026-09.md` addendum 3, PR #2370). What was left the
next day was 32 closed rows, two rule documents, a log, and ten closed
tracks' execution records.

**No exit walk was written, on Ev's direction (in-chat, 2026-09-11:
*"can you delete all the closed items in issues, and the code-quality dir
entirely"*).** The contract's exception — a program closes on a ratified
`docs/<NAME>-EXIT-WALK.md` *or* on Ev's ruling that it needs none — is
what this sweep runs on, and it is recorded here because the absence
would otherwise read as an omission. The three criteria a walk would have
tested are answered by the cut instead: its board is empty (criterion 1),
its successors exist and are named below (criterion 2), and its rules
survive relocation (criterion 3, the section that follows).

### What survived, and where

Two documents were **first moved to `docs/` and then, on Ev's
correction the same day, deleted with everything else** — see the
amendment at the end of this entry, which is the disposition of record.
What survives of them is named there: one sentence in the reviewer
brief, and the rules each of the eleven programs actually uses, inlined
into that program's own `plan.md`.

### What was deleted

- **`work/code-quality/` whole** (46 tracked files at the sweep SHA, less
  the two moved above): `program.md`, `log.md`, the **32 closed rows**
  (`C13`, `C14`, `D106`, `D202`, `D204`, `D205`, `D207`, `D208`, `D209`,
  `D224`, `D288`, `D289`, `D320`, `D321`, `D323`, `D324`, `D402`, `D64`,
  `D68`, `S22-row-1`, `S26`, `S290`, `chart-region-lane-contract`,
  `corner-config-tag-all-concave-trihedron`,
  `demo-tour-dead-constant-breaks-compile`,
  `demo-typed-refusal-exit-convention`,
  `directory-prefix-skips-have-no-subject-check`,
  `flat-pack-gap-rationale-invented-mechanism`,
  `probe-cutaway-comment-claims-shipped-box`, `scaled-square`,
  `smell-scan-2026-08-findings-register`,
  `tour-suite-never-runs-wall-probes`), and **`logs/`** — the ten closed
  tracks' execution records (`SMELL-C`, `SMELL-E`, `SMELL-F`, `SMELL-G`,
  `SMELL-H`, `SMELL-I`, `SMELL-KPW`, `SMELL-T`, `SMELL-UV` and
  `migration-census-2026-09-03.md`), about 11,000 lines.
- **The seven closed items in `work/issues/`**
  (`actions-budget-denies-job-starts`,
  `bounds-census-roster-lists-anchor-span-twice-with-two-dispositions`,
  `fillet-specs-require-a-narrowing-ci-config`,
  `freecad-lane-reports-no-drift-on-a-cell-whose-geometry-changed`,
  `m10-5-e2e-channel-slider-reds-at-eps-1e-6`,
  `render-lanes-checkout-merge-ref-vanishes`,
  `reviewer-pair-rebuilds-two-trees-two-rules`), on the same direction
  and the same rule: `work/` tracks work still to be done.
  `work/issues/README.md` stays and the directory keeps its purpose.

| program | title | closed | done-state of record |
| --- | --- | --- | --- |
| `code-quality` | Code quality — where a structural finding waits until a program claims it | 2026-09-11 | this row and the amendment below; `docs/WORK-TRACKS-2026-09.md` addendum 3 for where its 110 live rows went; the merged PRs of its closed tracks, named in the logs recoverable at the SHA above |

### The eighteen `refs:` this sweep rewrote

Deleting the closed rows would have broken `scripts/work.py`'s
*references resolve* rule on sixteen live rows across ten programs. They
were rewritten first, in GATES' and METER's form (sweeps 7 and 10): the
dying id is replaced by the number of the PR that closed it — ints are PR
numbers and lint does not check them — and a
`## Refs at code-quality's sweep (2026-09-11)` section on each citing row
says what changed and why.

| dying id | now cited as | citing row(s) |
| --- | --- | --- |
| `D64` | 1643 | `work/comb/L4.md`, `work/door/viewer-grid-pitch-nonfinite-fallback.md` |
| `D205` | 1642 | `work/tint/D386.md`, `work/topo/D261.md` |
| `D204` | 1642 | `work/instr/k-lint-csv-header-unpinned-against-five-producers.md` |
| `D320`, `D321` | 1782 (one entry, not two — both closed in it) | `work/wire/profile-has-no-scalar-lift-door.md`; `D320` alone in `work/scalar/sweep-test-rebuilds-validated-net-for-v-reversal.md` |
| `D323`, `D324` | 1783 (one entry, same reason) | `work/comb/L5.md` |
| `S26` | 1366 | `work/props/purchasable-area-tightness-valve.md` |
| `C13` | `epsilon-has-no-type-of-its-own` — the live row whose §Closed IS the ruling that closed it | `work/scalar/D283.md` |

**Five references were dropped rather than re-aimed**, and the reason is
the one the METER sweep did not meet: the row closed with **no PR to be
cited by**. Three closed on a ruling with no implementation (`C13` and
`C14`, whose rulings are recorded in the two live `work/exch/` rows that
cited them — so those rows were pointing at their own record;
`fillet-specs-require-a-narrowing-ci-config`, closed BY
`work/ciw/delete-config-trailer.md`, which cited it). Two recorded no
closing PR at all (`D68`, cited by `work/guard/D212.md` and
`work/guard/G4.md`; `D289`, cited by
`work/tint/decoration-seam-header-names-no-pin-for-enclose.md` — the 1533
in `D289`'s own `refs:` was CERT-M1, where it was *filed*, not where it
landed). Each of those five citing rows carries the dropped id, its
title, and the SHA it is recoverable at, in its
`## Refs at code-quality's sweep` section; nothing was silently removed.

`work.py lint` before the sweep: 0 problems, 23 warnings. After: 0
problems, 22 warnings — the one that went was the duplicate `github: 1607`
claim, whose second claimant was a closed `work/issues/` row.

### A note on inbound references, again

**102 files cite `work/code-quality/…` in prose, 217 times**, and they
survive unrewritten, which is what *A note on inbound references* above is
for and what GATES' and METER's sweeps did with their own. Two classes
were fixed, because both are live contract text rather than a citation:

- **The eleven programs of the 2026-09-11 cut** had their `plan.md`
  charters re-aimed off `work/code-quality/plan.md`; see the amendment
  below for where each rule they cited now lives.
- **`work/README.md`** (META's file, edited here by announced seam
  because this sweep is what makes it false, in the same commit): the
  clause saying `work/code-quality/` is where a finding waits for a
  claim, the `process-observations.md` row of the layout, the
  "code-quality only" gloss on `track:` and on `blocks:`. **A finding
  now goes straight onto the slate of the program whose ground it lands
  on**, and `work/issues/` is the last resort it always was.
- `docs/DESIGN.md`'s two citations of `work/code-quality/S14.md` and
  `S65.md` were already stale from the cut and now name `work/pipe/` and
  `work/pred/`.

One consequence is named rather than left to be found: **`d321-row-number-reissued`
(CITE's) and `S176` (CITE's) both ask for edits inside `SMELL-T-LOG.md`,
`SMELL-KPW-LOG.md` and `SMELL-G-LOG.md`**, which this sweep archived.
`S176`'s live half is its convention and is unaffected; `d321` is
overtaken on both halves, which the amendment below explains. CITE's
plan recorded it and left the tree at sweep 12; the row itself is
`work/meta/d321-row-number-reissued.md`.


### Amendment (2026-09-11, same day): the two relocated documents were deleted too

This sweep first moved `plan.md` and `process-observations.md` into
`docs/` on the argument that `plan.md` was cited 60 times and was
therefore load-bearing. **Ev rejected that** (in-chat: *"where is it
cited? we don't want to mint any new rows because we're using the in
repo issue tracker now, not the one big doc that descends from"*), and
the count did not survive being read:

- **48 of the 60 are one of three provenance sentences** repeated
  verbatim in `## Claimed by` sections — 17 rows in `work/tint/` and 14
  in `work/topo/` — and each of those sentences says, in its own second
  clause, that the content is **restated in the claiming program's own
  plan**. They are history, not lookups.
- 9 were this ledger, 3 `work/README.md`, 1 `docs/WORK-TRACKS-2026-09.md`.
- **Ten were live**, all of them in the eleven new programs' `plan.md`
  charters, and every one cited a single self-contained rule.

**The block ledger was the other half of the argument and it is simply
retired.** Ids in this tracker come from an item's name; the per-track
`D<N>`/`S<N>` blocks were the numbering of the 2026-08 register that the
tracker replaced, and keeping a document alive so that a future row
could be minted from a block would have preserved the scheme this
project stopped using. **No new row is minted from a block.** The rows
that carry such an id keep it — ids are stable for life — and nothing
allocates another.

**So both documents are deleted**, recoverable at this entry's sweep
SHA, and what was live in them went to where it is used:

| what | where it went |
| --- | --- |
| *The fix mints a fresh instance of the defect it closes; naming the trap in your own PR body does not prevent it; only a reader who did not write the fix has ever caught it* | **`docs/prompts/reviewer-style-lane.md` §1**, as a bullet in the stance — the standing brief every reviewer reads, which is what the rule is for. It was the only sentence in either document with no surviving home |
| The ordering, partition, seam and fence rules the eleven charters cited (ten citations) | **inlined into the citing `plan.md`**, one to three sentences each, so each program states the rule it runs on instead of pointing at a document about a program that no longer exists |
| *Ask what a reported sweep's pattern could not match* (`C15`) | nowhere — it was **already** `docs/prompts/reviewer-style-lane.md` §Q1. `work/door/viewer-grid-pitch-nonfinite-fallback.md` now cites the brief instead of `§C` |
| The `C2`/`H17` and `C21` labels on `work/comb/S37.md` and `work/door/run-on-whitespace-in-message-literals.md` | **folded into those rows** as the populations and dispositions they stood for; the labels themselves were a process-observation number and a track row id colliding, which is `C-namespace`'s point |
| The W `test-utils` ceiling seam, and P's three sub-lanes | nowhere — already restated, the first in all 17 `work/tint/` rows that cite it and the second in `work/topo/plan.md`, both by the same sentence that cited the plan |
| `C1`–`C27` otherwise | the archive. Of 27 process observations, **three were cited by a live row** and all three are handled above; the rest are a closed program's retrospective and are recoverable at the sweep SHA |

One observation is worth naming here rather than leaving at a SHA,
because it records a decision rather than a finding: **`C21` carries
Ev's ruling of 2026-08-20** that the *skip-reading-as-a-pass* class
stays un-rolled-up — a class rule was drafted around giving skips
*floors*, which concedes the skip, and the prior question is whether the
test should be skipping at all. A future scan re-opens that question
rather than re-proposing the floors.

`d321-row-number-reissued` (CITE's) is **overtaken on both halves** by
this amendment: its retired-id rule has no number ledger to live in now
that the blocks are retired, and its citation disambiguation was inside
two `SMELL-*-LOG.md` files this sweep archived. The row itself says so
and names the one thing still worth doing — a `work.py` check that an id
is never reissued — as META's; it was re-homed to
`work/meta/d321-row-number-reissued.md` when CITE closed at sweep 12.

## Per-merge deletion — DOCM-9's spec (2026-09-14)

Recoverable at `git show ec7f7770b569fb3c3aa3c87e086f66170166712c:docs/DOCM-9-SPEC.md`
(the DOCM-9 unit head, before the state-sync commit that deleted it;
unamended — no stop clause fired). Two of its letters are corrected in
the unit's record: item 3's "the first uncertified leaf outward" is
read as the first leaf the driver decided definitely otherwise (a
certified leaf and a flip-crossing leaf cannot be neighbours; the
literal reading is dead code), and A1's `NewFailure` row is unmet
because the driver prices a failing leaf as budget rather than naming
it (PROPS's row). The rule above; the unit's record is its row in
`MODEL-AB-LOG.md` and its MERGED entry in `work/docm/log.md`.

- `DOCM-9-SPEC.md` — DOCM-9, the certified locally-valid range query (#2534)

## Per-merge deletion — SYM-2's spec (2026-09-14)

Recoverable at `git show b73b289a2c01677b2a328899fc1ce8336e023778:docs/SYM-2-SPEC.md`
(the fix-pass head). One of its three moves did not land: the header's
"What remains outside the PLAIN form" paragraph was named as `form.rs`'s
whole, and half of it binds the rules (`algebra`, `trig`, `signed`,
`SymRules`), so that half went back to `sym.rs` in the fix pass. The
reviewer's proposed fourth move (the 1,045-line test module out) was
attempted and reverted: `scripts/gates/register-equal-allowlist.sh`
exempts `sym.rs` whole and hides the tier's thirteen test calls of the
door, filed on `guard`'s slate. Recorded in the PR body and the unit's
log entry.

- `SYM-2-SPEC.md` — SYM-2, the tier's file split: the coefficient tower and the polynomial out of `sym.rs`, the header distributed with them (#2532)


## Per-merge deletion — SYM-1's spec (2026-09-14)

Recoverable at `git show 9153d39be:docs/SYM-1-SPEC.md` (the landing
head). Two of its sentences did not survive the measurement: the
feature it named `sym-profile` is `sym-profile-testing` (the
`test-features-dev-only.sh` gate recognises only that suffix), and its
premise that the tier "skips where the numeric channel already
answers" was false in every profile this workspace builds — the
`Decide` impl's `debug_assert!` runs the discharge on every definite
margin, which the review found by execution and the unit's instrument
now tells apart from the decision path's walks. Recorded in the PR body
and the unit's log entry.

- `SYM-1-SPEC.md` — SYM-1, the profile inside the normal form (#2530)


## Per-merge deletion — SYM-3's spec (2026-09-14)

Recoverable at `git show da1f21e50:docs/SYM-3-SPEC.md` (the fix-pass
head). Its second ask's premise did not survive the render: the boss's
residue at `bulge = 2` is not the item's `atan|b|`-against-`atan b`
guess (`abs(2)` folds) but a coefficient-ring freeze first and an
`abs` over a non-constant argument second — and opening either
partially LOSES door discharges, which the spec's four-cause menu did
not foresee; that finding is filed as
`coefficient-ring-width-is-not-monotone-in-reach`. Recorded in the PR
body and the unit's log entry.

- `SYM-3-SPEC.md` — SYM-3, what stands at a bulge that is not 1 (#2558)

## Per-merge deletion — CURVED-MERGEDOOR's spec (2026-09-14)

Recoverable at `git show 0e1d06b47:docs/CURVED-MERGEDOOR-SPEC.md` (PR
#2105's merge commit, the last head carrying it). Every clause was met
as ruled: the spec's STOP 2 fired (scenes A/B reach the REST-zip's chord
on a cylinder wall once the door records) and the orchestrator re-scoped
the rows to the measured frontier (comment 5568446264); the rulings
§Rulings (public-door posture; `faces` carried) and the re-scope ruling
(no record for a pair with zero live faces; dedup at the door) are in
the unit's record — `work/curved/cylindrical-rest-pair-hits-planar-merge.md`'s
`## Closed` and the MERGED entry in `work/curved/log.md`; its A/B row is
MODEL-AB-LOG MDOOR. Shape (2) is banked on S-BOOL's
`cosurface-disjoint-curved-walls-refuse` with its consumer measured.


## Per-merge deletion — SYM-4's spec (2026-09-14)

Recoverable at `git show 972d802ff:docs/SYM-4-SPEC.md` (the fix-pass
head). Two of its sentences did not survive the measurement: its
"rendered-form digest row" became a per-walk, per-origin ledger of
form digests (a superset — `DecisionShape` carries no node id and
renders only blocked residuals), and its inline monomial and cached
degree were measured and NOT taken (under one percent each, and a new
shipped dependency for the first). Its "before 57 %" storage share was
SYM-1's reading; the unit's own before read 53.5 %. Recorded in the PR
body and the unit's log entry.

- `SYM-4-SPEC.md` — SYM-4, the cost of a form (#2565)

## Per-merge deletion — SYM-6's spec (2026-09-14)

Recoverable at `git show 8547c73e9:docs/SYM-6-SPEC.md` (the fix-pass
head). Written conditional on `[ev]` #2552 and amended twice the same
day: A1 (Ev picked D1 = (1); Phase 3 held), A2 (Ev took the refined
D2 — the refusal arm split by witness KIND; Phase 3 in scope). Two of
its sentences did not survive: "the four implementations" (five —
`Dual` forwards too) and a re-read of `m10_9_witness_limits_interval`
(a file that never existed; the citation is fixed in `real.rs`). The
picked route's own text on the item said the slack becomes
`max(tol.eps(), WITNESS_REL · scale)`; what shipped and what the spec
ratifies is `tol.eps() · max(|a|, |b|, 1)`, and the difference is
recorded on the item. Recorded in the PR body and the unit's log entry.

- `SYM-6-SPEC.md` — SYM-6, the door's witness moves with the run's ε
  (#2604)

## Per-merge deletion — SYM-5's spec (2026-09-14)

Recoverable at `git show 68d31a750:docs/SYM-5-SPEC.md` (the fix-pass
head). Its fixture was refuted by PR-1's review and replaced by
Amendment A1 (the tilted derived frame); its Phase 2 named three
mechanisms and the measurement took (b) — the quotient's common factor
at every early-walk node, dial `common_factor`, not the `unit_vector`
atom of (a) — with the (a)/(c) reading recorded as unmeasured; its
"bounds stated like `EARLY_STEPS`" became a structural bound (monotone
in terms and degree, NOT in coefficient width — a row pins the loss);
its "shipped on only if affordable on the five measured documents" was
not met on the pad and the bracket on the header's own instrument and
the dial ships ON anyway by the orchestrator's call, disclosed as a
deviation with the numbers; its acceptance width is the twin's minus
5e-2 (PROPS' clause-1 defect, pinned by name). Recorded in the PR
bodies (#2568, #2589) and the unit's two log entries.

- `SYM-5-SPEC.md` — SYM-5, a stored unit vector does not double the
  degree (#2568 PR-1, #2589 PR-2)

## Per-merge deletion — SYM-7's spec (2026-09-15)

Recoverable at `git show e9f75d5b5:docs/SYM-7-SPEC.md` (the fix-pass
head). Its sentences that did not survive: the memo keyed on the
budget alone (keyed on `(budget, rules)` — `combine`'s constant fold
reads two dials); the "opaque-sequence pin" as the premise's ALARM
(no drive mints an opaque, so the row measures nothing on the
measured documents and says so; the premise governs hits, not
soundness — the memo's soundness is that an id is a content hash of
syntax); the "per-leaf accounting goldens' `frozen` column" (none
existed; no golden moved); Phase 3's gate (measured 7.4 % table work
against a ≥ 10 % wall gate, not taken). Recorded in the PR body and
the unit's log entry.

- `SYM-7-SPEC.md` — SYM-7, the plain form outlives the leaf (#2609)

## Per-merge deletion — SYM-8's spec (2026-09-21)

Recoverable at `git show 7edff5e97:docs/SYM-8-SPEC.md` (the fix-pass
head). Its sentences that did not survive: Phase 1.3's stop clause
("if a split moves DOWN or a ceiling falls under the fold ... the
rule's predicate is narrowed until it does not") — literally tripped
on R2's rounded pad (`symbolic_zero` 858 → 854, `registered`
104 → 128, `numeric` 991 → 971; no decision lost, no ceiling moved),
and RULED a ratified spec deviation by the SYM orchestrator on
2026-09-21 with `symbolic_zero` pinned on all five pad documents
(`work/decide/SYM-8.md`; Ev may overrule); the predicate as first
written over `trig::manifestly_nonneg` (the shipped predicate is
manifest POSITIVITY, `manifest::positive`, strict at the signed-zero
edge, and `manifestly_nonneg` is retired); the denominator side
condition argued from a retired sentence (it rides `quotient`'s
four-source argument; rule F mints no new denominator); "M10-8's tier
exactly" for `shipped_without_the_door()` (already false before rule
F — the constructor is a door differential and keeps rule F;
`a0_alone()` is M10-8's tier). Recorded in the PR body and the unit's
log entry.

- `SYM-8-SPEC.md` — SYM-8, the manifest sign (#2616)

## Per-merge deletion — CURVED-TORUS's spec (2026-09-15)

Recoverable at `git show 4617fcc5b:docs/CURVED-TORUS-SPEC.md` (PR
#2535's merge commit, the last head carrying it). Both PRs delivered:
PR-1 (#1907, the boundary-tight torus operand box, A/B row TBOX) and
PR-2 (#2535, the circle-residual torus arm, A/B row TARM). The spec's
§Rulings and its Amendments of 2026-09-14 (the monotonicity theorem
`margin_K ≥ margin_1 − f2·(Δθ/K)²/8` replacing the refuted sentence;
the charge table at the arc-scoped `f2`; the full-carrier `f2`
corrected to the code's 8.36e3) are the unit's record together with
the two closed items `work/curved/torus-operand-boxes-span-whole-ring.md`
and `work/curved/circle-residual-harmonics-needs-torus-arm.md` and the
MERGED entries in `work/curved/log.md`. Still open from the spec's
residue: `torus-operand-gate-admission` (the lily's remaining pin's
retirement) and `the-chord-dip-charge-has-two-homes` (S-BOOL's half).


## Per-merge deletion — D290's spec (2026-09-15)

Recoverable at `git show f91aea516:docs/D290-SPEC.md` (the fix-pass
head). Its sentences that did not survive: the interior formula as the
unit-domain `lo + (hi − lo)·k` (landed as the general `[a, b] → [lo, hi]`
form, bit-identical on a unit source); "add a `KnotVectorIssue` variant
if none says it" for the domain refusal (landed as a `SplineError` arm —
a request-shaped refusal, not a vector defect — so `KnotVectorIssue`
kept `Eq`); the surface-level door left to the implementer's call (not
landed: no `src` consumer). Recorded in the PR body and the unit's
`## Closed` section.

- `D290-SPEC.md` — D290, the knot rescale is a `KnotVector` door (#2461)

## Per-merge deletion — TRIM-3's spec (2026-09-15)

Recoverable at `git show f1e984812:docs/TRIM-3-SPEC.md` (PR #2554's
merge commit, the last head carrying it). Both PRs delivered: PR-1
(#1911, the `topo` chart-boundary description and outside test, A/B row
T3A) and PR-2 (#2554, the clearance seam in `editor-core/clearance.rs`,
A/B row T3B). The spec's §Rulings and its Amendments (the metred hull;
the cylinder root rule reading the hull verbatim — now the named
`cut_root`) are the unit's record together with the closed item
`work/trim/clearance-window-tightening-needs-chart-boundary.md`, the
two MERGED entries in `work/trim/log.md` (PR-1 2026-09-07, PR-2
2026-09-15), and the six residues the unit
left on the program (`clearance-window-cone-sphere-torus`,
`exact-region-cells-for-lower-bound-only`,
`min-separation-tightening-crosses-the-drive`,
`revolved-bands-reach-no-clearance-row`, `three-tables-of-the-chart-arms`,
`a-refused-chart-boundary-has-no-reachable-window`). The seam into
SHELL/M10's file was announced on #1911 (comment 5568210053) and
merged on a week's silence, recorded in the TRIM log.

## Per-merge deletion — S393's spec (2026-09-15)

Recoverable at `git show 2f786af43:docs/S393-SPEC.md` (the fix-pass
head). Its sentences that did not survive: "the two hand copies" (five
were folded: the sweep suite's, the tour's two sweep cells, its loft
cell and its S-duct, and the cert5 fixture); "eight call sites" (three
plus the definition); the tour pin on the sweep cells (sited on the
loft cell as invariants instead — the cells' placements are three inline
lines); the `0.95` pin in `geom-core` (sited in the sweep suite,
PROPS' file being outside the fence); the fence itself (the k-lint gate
fired and the runbook's re-cut reached INSTR's baseline and its census
pins). Recorded in the PR body and the unit's `## Closed` section.

- `S393-SPEC.md` — S393, the path sweep's start frame has a door (#2466)

## Per-merge deletion — CENSUS-INERT-DENY's spec (2026-09-15)

Recoverable at `git show 6cf25b356:docs/CENSUS-INERT-DENY-SPEC.md` (PR
#2634's last head before the merge). CENSUS's first unit. Its sentences
that did not survive, all three refuted by the lane re-taking the
measurements the spec itself flagged as a hypothesis: "13 prose sites"
(14 in `.rs`, and 16 counting the two in markdown the spec's `.rs`-only
count could not see); "the fourteenth non-attribute is `doc.rs`'s
multi-line `#[serde(bound(…))]`" (no `deny_unknown_fields` appears in a
`serde(bound)` — that site is an ordinary attribute above an interleaved
comment); and the prose placing `Qualifier` and `RoleSeg` on the inert
side of the named-field rule (`Qualifier::OrderAlong { rank, of }` is a
struct variant and `RoleSeg` has fourteen — both govern, and the spec's
own total of 22 inert sites was unaffected, the error being in its
prose rather than its measurement). Also not landed as written: the
spec's allowance for keeping an inert attribute with a one-sentence
"habit-guard" reason (zero keeps taken — a kept site needs an exemption
in the instrument, and an exemption list is the hand-written list this
program exists to remove), and its "one style review, no correctness
lane" leaving the guard's blind-spot list unchecked (the style lane
refuted the list's exclusivity claim with four executed counterexamples
and the fix pass found a fifth).

The three premise corrections are recorded on the item file, which
survives this deletion, and the unit's record is
`work/census/log.md`'s CENSUS-INERT-DENY entry plus the PR body.
Residue, both filed rather than left in prose:
`work/census/census-sees-an-inert-attribute-but-not-a-missing-one.md`
(the census is one-directional) and
`work/msolve/mate-primitive-accepts-a-stray-field-the-module-docs-say-refuses.md`
(its one confirmed instance).

- `CENSUS-INERT-DENY-SPEC.md` — CENSUS-INERT-DENY, an attribute with nothing to deny (#2634)
## Per-merge deletion — TINT-1's spec (2026-09-15)

Recoverable at `git show d61ee4256:docs/TINT-1-SPEC.md` (the state-sync
head). Its sentences that did not survive, and the reason matters more
than usual because the spec was **wrong** rather than merely superseded:
*"Use the compiler"*, *"a variant added tomorrow makes this file fail to
COMPILE"*, and the instruction to derive the ban list from an exhaustive
`match` from a value **to its variant identifier**. rustc checks a
match's PATTERNS and never its strings, so arms returning hand-typed
identifiers left a RENAME silent — the style review demonstrated it on
the first implementation, which was green while banning a dead
identifier and leaving the live one unbanned. What landed reads the
identifier off each value's own derived `Debug`
(`test_utils::f6::variant_identifier`) and keeps the match as a bare
exhaustiveness token that forces the author to open the file and
nothing more. Also not landed: the spec's guess that `SelectRefusal::Band`
lacked a case (only its ban entry was missing), and its framing of the
`fields` roster as derivable (tried, false-positives on a door's own
prose prefix). Recorded in the PR body and the unit's `## Closed`
section.

- `TINT-1-SPEC.md` — TINT-1, the `assert_f6` ban lists stop being
  hand-written mirrors (#2648)

## Per-merge deletion — VREV's spec (2026-09-15)

Recoverable at `git show 4122ddacd:docs/VREV-SPEC.md` (the fix-pass
head). Its sentences that did not survive: the mirror test as the
rounded compare `fl(k_i + k_{m−i}) == fl(lo + hi)` (landed as an exact
2Sum compare: the rounded form admits a half-ulp asymmetry that
ties-to-even); "the same point set … bit for bit where `lo + hi − v` is
exact" (a magnitude-relative bound on a dense grid: the mirrored basis
values are not bit-identical at some knot values); the error as a
`KnotAlgebraError` variant (landed beside the door as
`KnotMirrorError`); `reversed_v` native with `reversed_u` derived (the
module's conjugation direction is the other way, and the fix pass
conformed to it); "keep the `.expect` text" on `set_face_surface` (its
`Result` cannot check the claim; the message now says what it can).
Recorded in the PR body and the unit's `## Closed` section.

- `VREV-SPEC.md` — VREV, a v-reversal door on `NurbsSurface`, exact or refused (#2627)

## Per-merge deletion — UNITVEC's spec (2026-09-15)

Recoverable at `git show a0281d87e:docs/UNITVEC-SPEC.md` (the fix-pass
head). Its sentences that did not survive: the `sin_cos` mint
("`UnitVec3::from_angle`-style mints … as the crate needs") — no
customer in the tree, `path_start_frame`'s in-plane axes are cross
products, so it lands with its first customer; "`Vec3::orthonormal_basis`
takes the witness (its precondition sentence goes away)" — its two
production callers (`newell.rs`, `recognize.rs`) normalize with no
length decision, which the same paragraph forbids adding, so the bare
door stays with its precondition pointing at the witness door
`UnitVec3::orthonormal_basis` and both sites are filed; "hand it on" for
`mirror_across_plane` (it mints and reads the witness straight back —
the Householder entries are components); the fence's `viewer/src/sketch.rs`
(the take landed in `viewer/src/datums.rs`, and WIRE's `wire.rs` took
the door, not just imports). Recorded in the PR body and the unit's
`## Closed` section.

- `UNITVEC-SPEC.md` — UNITVEC, the unit-vector witness moves to geom-core, minted by the decided ladder (#2646)

## Per-merge deletion — CENSUS-DEBUG's spec (2026-09-15)

Recoverable at `git show 67e56e2cf:docs/CENSUS-DEBUG-SPEC.md` (PR
#2655's last head before the merge). CENSUS's second unit. Its sentences
that did not survive:

- **the criterion excluding `topo/src/props.rs`'s `SignCertificate`** —
  the spec ruled it out because it uses `write!` rather than
  `debug_struct(…).finish()`, which is a claim about the TERMINATOR.
  The class is the tie to the declaration, and by that question it is
  squarely in: it renders in braced struct shape and reads `self.runs`
  by name. The instruction not to fix it was followed; the criterion is
  corrected on
  `work/census/hand-listed-partialeq-siblings-outside-the-census-debug-fence.md`,
  which names the spec as the source of the wrong criterion.
- **"the per-field question is compiler-known, so a census must not
  re-ask it"** — true of the twelve impls Half A touched, false of the
  population the resulting census then declares clean. Four body shapes
  answered green, one of them (`self.0.name`, a newtype reading a NAMED
  field of its inner type) de-listed at the site as something the
  classifier answers. Three are now closed in the classifier and the
  fourth is on the blind-spot list.
- **Half B's population** — stated as the item's six sites / seven
  impls; `knots.rs`'s `impl PartialEq for Span<'_>`, sitting directly
  under a `Debug` the hit list does carry, makes it seven and eight.

Also not landed as written: the spec's hit list of four new in-class
`Debug` impls was correct, but its reading of the item's own decay
understated it — the item's enumeration rule gave 14 impls on
2026-09-15 against the eight rows it recorded on 2026-09-06.

The premise corrections are recorded on the item file, which survives
this deletion, and the unit's record is `work/census/log.md`'s
CENSUS-DEBUG entry plus the PR body. Residue, all filed:
`work/census/hand-listed-partialeq-siblings-outside-the-census-debug-fence.md`
(six out-of-fence walks, two of them invisible to the census),
`work/shell/geometrywitness-eq-ignores-the-two-chart-axes-its-uv-fields-are-stated-in.md`,
`work/topo/censussubject-eq-answers-false-for-a-new-variant-against-itself.md`
and `work/mesh/memo-dumps-hide-the-closed-bit-the-counters-depend-on.md`.

- `CENSUS-DEBUG-SPEC.md` — CENSUS-DEBUG, a hand-listed `Debug` under a completeness claim (#2655)

## Per-merge deletion — SENSE-DOORS's spec (2026-09-15)

Recoverable at `git show 68f1d4729:docs/SENSE-DOORS-SPEC.md` (the
fix-pass head). Its sentences that did not survive: "a conditional
negation is exact in IEEE, so … bit for bit" — bit-identical at `f64` on
every non-NaN, non-zero value, but the old `κ·(±1)` product carried a
NaN's sign bit, a signed zero and, at `Interval`, an ulp of padding at
tiny magnitudes that the negation does not (head strictly tighter; no
verdict or margin moves; a signed-zero row pins it); "the missing-face
arm refuses through the same vocabulary" (it refuses, but with its own
words now — `face_of` says "declared face key does not resolve" and keeps
"lost its surface" for a stale surface key); "check whether anything
relied on the zero arm" — nothing could: `linear_rim_side` refuses
`Zero` itself, and that is now the one home of the claim, with the
sphere's and cylinder's sites reading identically and unguarded; the
fence (the fix pass reached `props/mod.rs`, `boolean/reduce.rs`, the
`mesh` and `sweep` suites and `crates/topo/README.md`, each said in the
PR body). Recorded in the PR body and the unit's `## Closed` section.

- `SENSE-DOORS-SPEC.md` — SENSE-DOORS, the five bare-T sense-sign doors take the bit (#2649)

## Per-merge deletion — CENSUS-TAG-REACH's spec (2026-09-15)

Recoverable at `git show 246ae018c:docs/CENSUS-TAG-REACH-SPEC.md` (PR
#2660's last head before the merge). CENSUS's third unit. Its sentences
that did not survive:

- **the criterion** — the spec said these words reach Python as
  `.variant`. They reach it as **`reason`**: `eval_err` writes that
  field and `pncad.pyi` declares `EvaluationError.reason`. A sweep on
  `variant` finds none of the sites and reports the class clean. The
  second criterion error in three specs, and the more dangerous kind,
  because a wrong criterion silently re-scopes the whole unit.
- **the fence** — "an `eval_err` call site" missed `node_failed` and
  `poisoned` at direct `typed_err` raises, the same door's vocabulary
  ten lines away. Ten sites, not seven.
- **the disposition** — the spec ruled that each word become a
  `pub const` in `tags.rs`, and required the lane to test by execution
  what would stop the next word. **Against the spec's own disposition,
  nothing red**: a fresh site minting a word that had never existed
  passed 85 Rust and 832 Python tests. The word became a TYPE, and then
  — after the style review found the wall guarded `eval_err` rather than
  the door — a type carried by `ErrorClass::Evaluation` itself.

The premise corrections are on the item file, which survives this
deletion, and the unit's record is `work/census/log.md`'s
CENSUS-TAG-REACH entry plus the PR body. Residue, all filed on
`work/census/`: `py-discriminant-getters-under-src-py-are-outside-every-inventory`,
`py-reason-and-variant-literals-outside-any-enum`,
`four-censuses-of-python-visible-vocabulary-in-one-crate`,
`pncad-py-tests-rs-is-six-thousand-lines-and-carries-two-unremeasured-floors`,
`datum-kind-vocabulary-is-hand-spelled-and-uncensused` and
`evaluationerror-stub-lists-five-reasons-and-the-door-raises-six`.

- `CENSUS-TAG-REACH-SPEC.md` — CENSUS-TAG-REACH, the refusal word rides the class (#2660)

## Per-merge deletion — RATE-PAIR's spec (2026-09-15)

Recoverable at `git show 9530c9070:docs/RATE-PAIR-SPEC.md` (the fix-pass
head). Its sentences that did not survive: "`Margin::levered` with an
angular ARM (m per radian …) is NOT a rate per parameter unit and
stays" — false in the tree for the plane and spline kinds, whose
u-channel arm is `chart_stretch_sup`'s own `SupSpeed`; the carve-out was
redrawn for a LEVER that is not the curve's or chart's own parameter
rate, `azimuth_arm` became `chart_u_arm` returning `ChartArm { Angular,
Rate(SupSpeed) }`, `certify.rs` check 2's circle/ellipse arms went
through `metered` with the `InfSpeed` `param_rate` already mints, and
the remaining angular arms are filed as the class's next member;
"`chart_stretch_sup` … (`SupSpeed` pairs)" — its Cone arm could not
honour the tag (`|S_u| = v·sin α`), so the door refuses
`NoChartSup::ConeAzimuthGrowsWithV` and `chart_stretch_sup_v` mints the
cone's exact `v` arm; "the two conversions" on `InfSpeed` — `to_param`
had no caller and the wrong safe side, deleted; "a digest row committed
before the change" — the committed narration hash was not reproducible
from its recipe, replaced by a zero-parameter recipe re-taken at
`origin/main` and at head; "the `Display`/`Debug` of every receipt …
keeps its text" — `PatchRegularity`'s derived `Debug` prints the tag
(nothing observable reads it; disclosed). Recorded in the PR body and
the unit's `## Closed` section.

- `RATE-PAIR-SPEC.md` — RATE-PAIR, SupSpeed and InfSpeed beside Margin; metered takes the inf; a sup door (#2657)

## Per-merge deletion — CENSUS-PY-GETTERS's spec (2026-09-15)

Recoverable at `git show 712d1a071:docs/CENSUS-PY-GETTERS-SPEC.md` (PR
#2663's last head before the merge). CENSUS's fourth unit, first of the
`pncad-py` block. Its sentences that did not survive:

- **the count, presented as verified** — the spec said six Python-visible
  maps and 23 words, and said it had **independently re-measured** them.
  The row's pattern and that check were both lowercase-anchored scans for
  `=> "word"`, so both missed `py/value.rs`'s `dimension_name`, whose
  four words are CAPITALISED and reach Python as `Measurement.dimension`.
  Nine minting, seven Python-visible, 27 words. **A verification shaped
  like the finding it checks is not a verification**, and that is the
  lesson rather than the arithmetic.
- **the siting argument** — `errors.rs` claimed to hold "the one
  Python-visible alphabet that is not lower snake case", with a
  counter-example 370 lines below it in the same file
  (`ErrorClass::class_name`'s 35 exception-class names). The siting stands
  on the reader's lower-snake constraint instead.
- **a stated blind spot** — the unit reported that a rename probe cannot
  see a map moved wholesale out of `tags.rs`, and built an argument on it.
  The inventory guard has an explicit GONE branch; one probe dissolves
  the claim. The entry was reasoned rather than executed.

Not a spec sentence but recorded here because it was the ORCHESTRATOR's
error: the adjudication of the style review directed a row to be filed
for the `#[pyclass]` enum vocabulary as an uncensused population. It is
not — `tests/test_stubs.py` holds all 114 member names against the stub
name-for-name in both directions, proven by renaming `ArcSweep::Ccw`.
The fix pass refused the filing with the probe output. Filing it would
have been this program's fifth overclaiming row.

The corrections are on the item file, which survives this deletion, and
the unit's record is `work/census/log.md`'s CENSUS-PY-GETTERS entry plus
the PR body. Residue filed on `work/census/`:
`sixty-one-tag-words-are-minted-by-two-or-more-maps-and-seven-are-read`
and `errors-rs-holds-four-python-visible-maps-and-nothing-enumerates-that-file`.

- `CENSUS-PY-GETTERS-SPEC.md` — CENSUS-PY-GETTERS, seven discriminant maps move to the inventory (#2663)
## Per-merge deletion — TINT-2's spec (2026-09-15)

Recoverable at `git show befcfe4ae:docs/TINT-2-SPEC.md` (the fix-pass
head). **Its central recommendation was impossible**, which is why this
entry is longer than the deletion warrants: the spec offered a fork and
leaned to option (A), *"make a stand-down a fact the suite can floor —
`vacuity::Exposure` already has that shape in the same module"*. The
lane refused it and the review verified the refusal behaviourally: under
the pinned `cargo-nextest 0.9.140` a two-row probe reports pids 12156
and 12157 with a `static AtomicUsize` reading 0 in both, so **nextest is
process-per-test** and nothing a `stood_down` call records can be read by
any other row. Option (A) had no mechanism inside the unit's fence. What
landed is (B) on both halves plus `test_utils::loud_skip_marker!` as one
home — which the spec did not propose, because it also asserted, wrongly,
that the unit should weigh only tallying against stripping.

Other sentences that did not survive: the spec's claim that the marker
population's hand-kept enumeration lived only in the `println!` bodies
(two marker NAMES were enumerations too, and the name is the half that
reaches the PASS list); and its framing of `crates/viewer/src/lib.rs` as
simply out of fence (true, but it is also the tree's worked example of
the fix shape, which the spec did not say). What DID survive and was the
spec's real contribution: the working-half/broken-half framing that made
the two rows one unit, and the requirement that the unit state what its
guard does not enforce — which it did, at its sites and in its PR.

- `TINT-2-SPEC.md` — TINT-2, a stand-down that nobody can hear (#2656)

## Per-merge deletion — CENSUS-PY-RAISE-LITERALS's spec (2026-09-15)

Recoverable at `git show d1d3c1b3c:docs/CENSUS-PY-RAISE-LITERALS-SPEC.md`
(PR #2682's last head before the merge). CENSUS's fifth unit. Its
sentences that did not survive:

- **the second-mint claim** — the spec repeated the item's statement that
  `unclassified`, `wireframe` and `not_utf8` each duplicate a word their
  own door's inventoried map mints. **Only `unclassified` does**; the
  other two return nothing from `tags.rs` and are new words on an
  attribute an inventoried map otherwise fills. The orchestrator had
  verified `unclassified` and carried the generalisation over the other
  two — **verified the example, asserted the class**, which is the same
  shape as CENSUS-PY-GETTERS' lowercase-anchored check by a different
  mechanism.
- **the const-in-a-match-arm proposal**, inherited from the item — a
  `pub const` that `select_refusal_tag`'s wildcard and the `flush.rs`
  site both read. True of a top-level const, **false of a match ARM
  reading one**: the tag reader's `ArmShape` admits a literal, a nested
  match, a block, `None`, `Some(..)` and a delegation, and nothing else.
  A one-arm map was taken instead, because a delegation is a shape the
  reader already admits.

The spec's framing that survived and was the unit's spine: **a `pub
const` pins a word's text and does not close its class**, so each shape
is asked whether its door's attribute can carry a type — proven by
CENSUS-TAG-REACH and re-driven here.

The corrections are on the item file, which survives this deletion, and
the unit's record is `work/census/log.md`'s CENSUS-PY-RAISE-LITERALS
entry plus the PR body. Residue filed on `work/census/`:
`prose-counts-of-a-populations-size-in-pncad-py-doc-comments`,
`both-unclassified-crossings-are-unreachable-and-so-is-the-repair-on-one`
and `dimension-error-op-carries-twelve-words-minted-at-call-sites`; and
on `work/lib/`, `validationerror-stub-declares-one-of-the-class-two-shapes`.

- `CENSUS-PY-RAISE-LITERALS-SPEC.md` — CENSUS-PY-RAISE-LITERALS, nine raise-site words get homes (#2682)

## Per-merge deletion — EXHAUST-LANE's spec (2026-09-15)

Recoverable at `git show 44a44b8de:docs/EXHAUST-LANE-SPEC.md` (the
fix-pass head). Its sentences that did not survive: "the one writer
(`sweep`) builds the receipt from the lane it was handed" — `sweep`
now counts into a private tally and the two accounting doors attach
the lane (a seeding path hands back no receipt to state a lane on);
"`seed_chart_plane` … stays where it is" — it took the accounting
door's shape (`speed`, metres floor, one crossing) so the chart lane
crosses the same way twice; "metres by one private function" — the
method is public on `ExhaustLane` (`meters`, `speed`) and the refusal
carries a named payload (`ExhaustivenessRefusal`) so its readings
return `f64`, not `Option`; the sibling-division sentence's "mint the
`SupSpeed` where the closure reads the box" — the box's sup got one
home (`Box3::speed_sup`, an `f64`: minting a tag there would pick the
collapse policy the two chart-rate sites deliberately disagree on,
which is the filed TRIM row). Recorded in the PR body and the unit's
`## Closed` section.

- `EXHAUST-LANE-SPEC.md` — EXHAUST-LANE, the exhaustiveness receipt carries its lane (#2667)

## Per-merge deletion — PROPS mignitude-floor's spec (2026-09-15)

Recoverable at `git show f050e2d6f:docs/PROPS-MIGNITUDE-FLOOR-SPEC.md`
(the unit head, before the state-sync commit that deleted it). Its
acceptance held as written — `e_lo` within a factor of two of `|d|` at
the micron row's sup cell (0.56 of it), every certificate tighter or
unchanged, the fit's bits unmoved — with two sentences the measurement
did not support. "Every certificate tightens or holds" is true of the
CELL bound and not of the DOOR bound, which is not monotone in it: one
of seventy requests rose by 1.8 per cent through the marking schedule,
which the spec called a MAJOR finding to stop and report, and which the
unit reported, filed and did not fix (both review lanes judged filing
right — a marking rule that changes every request's schedule is a unit,
not a rider). And the spec's "read whether meter 1 has a witness to
read together" resolves to: it does not need one, because
`offset_meters::cell_normal` already joins its componentwise assembly
with two together-readings and takes the largest. The rule above; the
unit's record is its `## Closed` section, its row in `MODEL-AB-LOG.md`
(ordinal 2404, sample #208) and the MERGED entry in `work/props/log.md`.

- `PROPS-MIGNITUDE-FLOOR-SPEC.md` — PROPS mignitude-floor, the floor on the residual's norm read through the sign witness instead of componentwise (#2469)

## Per-merge deletion — FRAME-WITNESS's spec (2026-09-15)

Recoverable at `git show 65dd211e3:docs/FRAME-WITNESS-SPEC.md` (the
fix-pass head). Its sentences that did not survive: "`from_aim` … the
`point_at`/`path_start_frame` recipe" as a public mint — the dual's
bilateral MAJOR: it took perpendicularity on trust, so it is
`pub(in crate::linalg)` and the public aim door is
`from_aim_and_reference`, which makes the perpendicular; "`w = u × v`
… the rounded cross product" — for the aim mints `w` is the aim
verbatim, `v = aim × u`, and the type's doc says `w` is the third
witness the mint produced; "`gram_schmidt(origin, u_raw, v_raw,
site_u, site_v, band)`" — one `site`, the refusal an `OrthoFrameError
{ axis, error }` naming the ROLE (`OrthoAxis::U` kept, `V` yields)
shared by all four mints; "`Node.tube` … mint the frame … through
`from_aim`" — the one home is `from_axis_and_reference(origin,
axis_raw, reference, site, band)` and the five copies of that ladder
(Python tube doors, `tube_args`, the tour) became one call each;
"WIRE's private `AxisFrame` becomes `OrthoFrame`" — read and folded
through `frame_axes` (no separate struct remains); "`SketchPlane` …
hand the frame through" for the stored placement — it holds any
`Affine3`, said honestly, with the witness dying at the read boundary
(the filed BOOL row). Recorded in the PR body and the unit's
`## Closed` section.

- `FRAME-WITNESS-SPEC.md` — FRAME-WITNESS, the frame witness in geom-core, minted by the decided ladders; `from_frame` and the tube door take it (#2675)

## Per-merge deletion — SENSE-FOLD's spec (2026-09-15)

Recoverable at `git show 80ab05d69:docs/SENSE-FOLD-SPEC.md` (the
fix-pass head). Its sentences that did not survive: "every consumer
spells `radius * side` as a conditional negation of the radius" — the
negation has one home, `sided(side, x)` in `blend/battery.rs` beside
`Convexity` (whose `signed` is that call), and both `arms.rs` siblings
fold onto it; "consider `Option<OutwardNormal<T>>`" for `outward_of` —
the helper is deleted and its three callers read the keyed door;
"keeps a raw-text row tree-wide" — the guard
`no_source_file_folds_the_bit_by_hand` walks `crates/*/src` in the
`code_only` view and pins the three D6-sanctioned scalar negations
(`dihedral.rs` `kappa_rel`, `walk.rs` `area`, `shell.rs` `thickness`)
rather than zero, with the vector class at zero and its blind spots
(renamed bit, `copysign`, a `±1` local, `match`, tests/demos/tools/
benches) stated in-file; "a row on one concave and one convex blend
where the ball side differs, bits pinned" — the fold's proof is the
stated differential (`a+b` vs `a-(-b)` identical; `s-rim` vs `-(rim-s)`
identical except a signed zero at `s == rim` no consumer reads) plus
the suites and the tour digest, the R2 fillet bit dumps having been
dropped as assertion-free; the curved reading, unnamed by the spec,
got a home of its own (`geom_brep::implicit_outward_normal`, the topo
alias gone). Recorded in the PR body and the unit's `## Closed`
section.

- `SENSE-FOLD-SPEC.md` — SENSE-FOLD, the hand multiplies of a normal by `sense_sign` fold onto `OutwardNormal`, and `Face::sense_sign` retires (#2668)

## Per-merge deletion — CENSUS-ERRORS-ARRIVAL's spec (2026-09-15)

Recoverable at `git show fa49e26b0:docs/CENSUS-ERRORS-ARRIVAL-SPEC.md`
(PR #2691's last head before the merge). CENSUS's sixth unit. Its
sentences that did not survive:

- **the five-map framing.** The spec said the unit was worth running
  "rather than for the five maps, none of which is unguarded today",
  and the row's table listed five. The file's literal-minting
  population is **ten items and 53 literals**, three of the extras
  Python-visible. The orchestrator had verified each of the five maps'
  pins and asserted the shape of the population from them — standing
  finding 10, *verified the example, asserted the class*, applied to
  the orchestrator's own framing by the lane. The count moved twice
  inside the unit: the lane first reported nine and 52, and the tenth
  (`is_bare_camel_token`, one char literal) appeared only when the
  char-literal hole the style review found was closed.
- **the three dispositions, presented as the field.** The spec named a
  looser reader over the file's ITEMS, a population reduction, and a
  rule that Python-visible words come from `tags.rs` only — and
  deliberately took none. **The lane took a fourth**, keyed on the
  file's LITERALS, and the spec's own argument against the first
  bullet ("it must walk `impl` bodies … itself a hand-maintained thing
  needing a guard") turned out to describe the shape that was taken
  rather than to count against it: it does walk `impl` bodies, it is
  hand-maintained, and it has its own guard and four refusals.

The spec's framing that survived, and that this unit is the evidence
for: **not pre-deciding the disposition.** Three of the four specs
before it pre-decided one and two of those were overturned by a probe
the lane ran; this one declined, and the disposition the lane found is
one no bullet on that page named.

What did NOT survive contact with the style review, recorded because
the PR body first asserted it: the reader's central claim that *"there
is no form a word can arrive in that the reader was not taught,
because there is no form"*, and that a wrong attribution is always
loud. **Both were executed as false** — a char literal was read and
dropped along with the item spelling nothing else, and an attribute
literal was charged to the rostered item above it, where a deletion in
the same item cancelled it. Both are closed, not narrowed. That is
this program's **fourth consecutive** short exclusivity list.

The corrections are on the item file, which survives this deletion,
and the unit's record is `work/census/log.md`'s CENSUS-ERRORS-ARRIVAL
entry plus the PR body. Residue filed on `work/census/`:
`payload-attribute-names-are-spelled-twice-and-held-equal-by-nothing`,
`dimension-mismatch-sentence-is-spelled-in-two-crates-and-held-equal-by-nothing`,
`the-field-brace-fingerprint-is-spelled-at-eight-sites-in-six-crates`
and `the-errors-arrival-blind-spot-list-claimed-exclusivity-and-was-short`.

- `CENSUS-ERRORS-ARRIVAL-SPEC.md` — CENSUS-ERRORS-ARRIVAL, an arrival alarm over `errors.rs` keyed on its literals (#2691)

## Per-merge deletion — TINT-3's spec (2026-09-15)

Recoverable at `git show da1b20f85:docs/TINT-3-SPEC.md` (the fix-pass
head). **The first S-TINT spec whose mechanism survived contact**, and
the reason is the section the two before it lacked: it named the
measurement that would kill the design (does `include_str!` inside an
exported macro resolve at the invoking file or the defining one) and
required the lane to take it before writing the fix. It came out the
spec's way; had it not, all fifteen rows would have checked
`test-utils`' own tree while reporting on fifteen crates, silently.

What did not survive: the spec's count. It said **fourteen** and the
tree held **fifteen** — `crates/test-utils/tests/all.rs` landed between
the re-derivation and the fix, which is also the row's own `test-utils`
rider resolving itself by growing a copy. The spec also did not
anticipate the coupling that turned out to be the unit's real subject:
`crates/test-utils/tests/reader_census.rs` was detecting each aggregating
`all.rs` by a margin of exactly one `.rs"` literal that
`include_str!("all.rs")` supplied, so the collapse took fourteen of
fifteen aggregators to zero margin and forced two detectors to move. No
spec could have named that; the lane measured it, the review adjudicated
it a correction rather than a silencing, and the fix pass closed the
residue it left. Recorded in the PR body and the unit's `## Closed`
section.

- `TINT-3-SPEC.md` — TINT-3, fourteen byte-identical aggregation guards
  (#2680)

## Per-merge deletion — CENSUS-ARRIVAL-RESIDUE's spec (2026-09-16)

Recoverable at `git show 1634ae3f9:docs/CENSUS-ARRIVAL-RESIDUE-SPEC.md`
(PR #2704's last head before the merge). CENSUS's seventh unit — the
four residues the sixth shipped disclosed and unscheduled. Its
sentences that did not survive:

- **acceptance 5's "7795 lines".** `main` was **7805**. 7795 was true
  at `fa49e26b0`, the commit that wrote it, and the merge `17a68a0fa`
  is 7805 because another lane added an `ortho_frame_error_tag` row to
  `TAG_INVENTORY` **on the other parent**. So the size row's own
  mitigation — *state the command that re-derives the count and the SHA
  it was taken at* — **does not survive a concurrent change to the same
  file**, and the stale number reached main, the row's title, the slate
  and this spec. That is a mechanism for standing finding 12 that
  nothing had recorded, and it is written into the size row.
- **"What I verified" item 4's aim.** The spec pointed at
  `starts_an_item` as where form-keying could return. Wrong: it
  survived nineteen hand-built cases, admitting no type position and
  rejecting no stable-Rust item form. **The form-keying was one reader
  over**, in `declaration_heads`, whose `declaration_name` split a LINE
  and required the keyword to be its first token — so
  `read_minting_items("impl Subject { pub const ALL… }")` answered `{}`,
  silently. Residue 1 had promoted that reader from the attribution
  rule to the POPULATION key, which turned a mis-charged literal into a
  missing row. The conclusion the spec drew was right and the location
  it named was not.

The spec's framing that survived: **not pre-deciding residue 1, and
asking for a measurement instead.** The measurement found that the
disclosure it was testing described two hypotheticals the file holds no
instance of, while a live case — `EvalReason::ATTRIBUTES` — sat
unrostered; that is what moved the population to the file's
declarations.

The corrections are on the item file, which survives this deletion, and
the unit's record is `work/census/log.md`'s CENSUS-ARRIVAL-RESIDUE
entry plus the PR body. Residue filed on `work/census/`:
`validation-error-reason-is-raised-and-the-stub-declares-only-door`,
`one-stub-convention-has-two-readers-in-two-languages` and
`the-mint-reader-hosts-three-lexer-operations-of-its-own`; on other
programs' slates, `work/tint/item-body-takes-a-const-generic-brace-for-an-item-body`
and `work/ciw/doc-gate-cannot-see-a-broken-link-inside-a-cfg-test-module`;
and on `work/meta/`,
`a-plan-whose-table-orders-the-work-has-no-check-that-it-lists-the-work`.

- `CENSUS-ARRIVAL-RESIDUE-SPEC.md` — CENSUS-ARRIVAL-RESIDUE, the arrival alarm's four disclosed residues, three repaired and one measured (#2704)

## Per-merge deletion — CENSUS-HAND-LISTED-SIBLINGS's spec (2026-09-16)

Recoverable at `git show 0687260bc:docs/CENSUS-HAND-LISTED-SIBLINGS-SPEC.md`
(PR #2712's last head before the merge). CENSUS's eighth unit — the six
hand-listed `PartialEq`/`Debug` walks CENSUS-DEBUG checked and filed
rather than swept. Its sentences that did not survive:

- **"four fences".** The territory section's heading undercounted its
  own body: five programs over five paths, the fifth being
  `crates/test-utils/*` (TCOST's and TINT's), which the body named and
  the heading did not.
- **the citation `hand_written_impl_census.rs:774`** for
  `every_known_hand_listed_impl_is_still_found`. `:774` is inside the
  assert message; the `fn` was at `:747` when the spec was written.
  The spec's own fact 5 told the lane to re-derive it and the spec did
  not — and the row it binds closes with the sentence *"line citations
  rot on every edit above them"*.
- **the framing of `SignCertificate` as having no correspondence to
  restore.** The spec said the render "makes `finish()`'s completeness
  claim by hand while having no tie to the declaration to restore", and
  that half is true of `SignCertificate`'s own five fields. It is false
  one type out: the render prints `VolumeEnclosure`'s complete
  three-field roster under their own names, so there was a field list
  to tie and the unit's first pass did not tie it. A style review added
  a fourth field to that type and `topo` compiled clean.

The spec's framing that survived: **not choosing `SignCertificate`'s
disposition and asking for a measurement instead.** The measurement —
zero of four rendered things are fields, zero of five fields are
rendered — is what decided the braces, and it is right; what it did not
reach is the paragraph above.

The corrections are on the item file, which survives this deletion, and
the unit's record is `work/census/log.md`'s CENSUS-HAND-LISTED-SIBLINGS
entry plus the PR body. Residue filed on `work/census/`:
`componentwise-equality-of-the-linear-types-is-hand-listed`; and on
`work/tint/`, `the-per-impl-sight-anchor-is-a-suppression-list-that-shrinks`
and `census-answers-no-field-read-for-a-walk-that-reads-a-field`.

- `CENSUS-HAND-LISTED-SIBLINGS-SPEC.md` — CENSUS-HAND-LISTED-SIBLINGS, six hand-listed walks get a tie to their declaration (#2712)

## Per-merge deletion — EDIT-PICK's spec (2026-09-16)

Recoverable at `git show dde873c04:docs/EDIT-PICK-SPEC.md` (PR
#2721's fix-pass head, carrying the spec's own amendment section).
EDIT's first kernel unit under the v6 dual. The sentence that did not
survive, and it was the spec's central one: **"the box-entry guard
removes only answers already proven wrong."** True in exact arithmetic,
false in `f64` — the guard compared a rounded `t` against a bound
widened for the box's rounding and not the test's, so on any
axis-planar triangle (whose box is degenerate along one axis and whose
entry parameter therefore IS the hit's parameter everywhere) genuine
well-conditioned hits fell below it by ULPs and were refused. Both
reviewers found it independently, one on a corpus document where the
service then answered the wrong face. The spec's framing that survived:
the derived determinant bound as the mechanism for the noise class, and
the demand that every "measured over every landing" claim be a row. What
the spec did not foresee: the certified determinant alone leaves the
ring case wrong (a certified-but-small determinant with exact zero
barycentrics and a cancelled quotient for `t`), which the fix pass
closed by taking `t` from the hit point's projection — the amendment
section records it, and the residual it leaves is
`work/edit/pick-accepts-uncertified-barycentrics-on-a-certified-determinant`.

## Per-merge deletion — PROPS band-doors' spec (2026-09-16)

Recoverable at `git show 96618224e:docs/PROPS-BAND-DOORS-SPEC.md` (the merge
of main into the unit head, before the state-sync commit that deleted
it). Two of its sentences did not survive contact. It ruled a named
constructor for the six sites it counted; one of those six takes the
ambiguity constant as a deliberate parameter, so the ruling was
withdrawn for it on the lane's evidence and the review lane's sharper
second reason. And it asked for the rows to rest "on the validator's
invariants" without saying where, which the lane first read as a local
restatement of a private validator — a premise that rots silently — and
the review turned into rows through the real `Tolerance::init`, one
process each. The rule above; the unit's record is the two items'
`## Closed` sections and the MERGED entry in `work/props/log.md`. An E
rider: single style review, no A/B row.

- `PROPS-BAND-DOORS-SPEC.md` — PROPS band-doors, the error documentation made true and the constructor five suites hand-rolled (#2729)

## Per-merge deletion — PROPS affine-try-map's spec (2026-09-16)

Recoverable at `git show c6a8bc33d:docs/PROPS-AFFINE-TRY-MAP-SPEC.md` (the
merge of main into the unit head, before the state-sync commit that
deleted it). It held as written, including the seam it named: the
unmerged unit that rewrites `orthonormal_basis` in the same file merges
textually, executed rather than assumed. Its one open call — how `map`
is spelled once `try_map` exists — was left to the unit and the unit
argued it in the PR. What the spec did not anticipate is where the
evidence would come from: it asked that a transposed column be shown
red, and the mutation that proves it also caught a row of the unit's
own that could not fail for the reason it was named after. The rule
above; the unit's record is the two items' `## Closed` sections and the
MERGED entry in `work/props/log.md`. An E rider: single style review,
no A/B row.

- `PROPS-AFFINE-TRY-MAP-SPEC.md` — PROPS affine-try-map, the kernel owns the fallible per-coordinate walk too (#2743)

## Per-merge deletion — EDIT-RADIUS's spec (2026-09-20)

Recoverable at `git show 370e560e7:docs/EDIT-RADIUS-SPEC.md` (PR
#2892's frozen review head; the spec was not amended on the branch —
its two corrected premises are argued in the PR body and recorded in
the unit row's `## Built`). EDIT's second kernel unit of block
EDIT-B2, ruled by the EDIT orchestrator on the row's own analysis and
DM8: the replay record gains a per-radius emission record
(`ReplayStructure.radii`) and DM8's map reads it in place of "one
radius, one segment", the key's feed widening with the attach so
"attached ⊆ keyed" holds by construction. Its central premises held —
the record's shape and home, the emission at the moment the bulge is
set, the map through the one checked permutation, the feed's
inclusion, DM8's sentence re-worded not re-decided — and two fell
before the build, each corrected by the implementer against the
tree: premise 2's "the current step is the emitter" names, for a
fused verb's carrier arc, an `At`/`Toward` holding no radius (every
role addresses the fused verb's own step); and the three-radii row's
"two `Sweep` specs" is unrepresentable (`arc_fillet_arc(Sweep, r,
Radius)` is the chain). None fell to the dual, which was
APPROVE-WITH-FIXES on both arms with no MAJOR; its findings and the
fix pass that built their union are the unit row's `## Built` and
its `### Fix pass`. The rule above; the unit's record is its row's
`## Closed`, its row in `MODEL-AB-LOG.md` (ordinals 4806/4807, sample
#223; block EDIT-B2 slot 1 concluded) and the MERGED entry in
`work/edit/log.md`.

## Per-merge deletion — EDIT-DECL's spec (2026-09-19)

Recoverable at `git show df47b36ab:docs/EDIT-DECL-SPEC.md` (PR
#2809's fix-pass head, carrying the `## Amended at the fix pass`
section). EDIT's first kernel unit of block EDIT-B2, built on Ev's
ruling from `[ev]` PR #2795 (sited declarations: a `Declare`'s pairs
name entities at their members, one pass, the site is the side, DM6
untouched). Its central premises held — the sited payload, the side
from the site, the union's routing by site with the member-space
rewrite ahead of the shared look-through, the doors following the
payload — and three fell before the build, each corrected by the
implementer against the tree: `persist/pairs.rs` is the
appearance-store codec and not the declare one; `pncad::select`'s
declare doors are re-exports of `editor-core`'s; `declare_node` holds
no consumer context, so the flush finding carries its sites rather
than the door siting names. One fell to the dual: premise 4's "fold-
minted rows are not declaration subjects, by type" left a union's
refusal against a `Merged` row with no site to carry, and the build
degraded it to an emission bug — both blinded reviewers found it with
red probes on the real door, and the fix pass ruled the refusal's
shape (sited at a constituent for a merged row; typed
`UndeclarableContact` for a row no member stands for), the decision
unchanged and Ev told on the next `[ev]` PR. Premise 7's "refuses
typed … has no case" was withdrawn with it. The rule above; the
unit's record is its row's `## Closed`, its row in `MODEL-AB-LOG.md`
(ordinals 4804/4805, sample #216; block EDIT-B2 slot 0 concluded) and
the MERGED entry in `work/edit/log.md`.

## Per-merge deletion — EDIT-PICK3's spec (2026-09-17)

Recoverable at `git show af694692:docs/EDIT-PICK3-SPEC.md` (PR
#2786's fix-pass head, carrying the `## Amended at the fix pass`
section). EDIT's third kernel unit on the pick door, built on Ev's
ruling from `[ev]` PR #2764 (the `t` interval, the order, the
tie-break, the clamp, the box as early-out only). Its central premises
held — the enclosure was verified exactly by both reviewers, the
closed-∧-INFORM measurement reproduced to the number — and three of its
sentences fell, each measured before it was built on or by the dual:
the headline ring fixture could not turn green (the noise-floor
candidate's certified interval precedes the aimed vertex, so the
tie-break never runs; the class was pinned on `tube_arc`), the ruling's
pairwise order has 3-cycles and was built as a rule over the set of
candidates no other precedes, and premise 5's early-out inequality was
unsound for the width tie-break the same ruling added — both blinded
reviewers found it with red probes on the real door, and the fix pass
derived the margin (no chosen factor) so `Pruned == Every` is a
theorem, the decision unchanged and Ev told on the next `[ev]` PR.
Premise 4's "nearest point" was the wrong word for a retraction. The
rule above; the unit's record is its row's `## Closed`, the ruling
row's `## Amended` and `## Closed`, its row in `MODEL-AB-LOG.md`
(ordinals 4802/4803, sample #215; block EDIT-B1 concluded) and the
MERGED entry in `work/edit/log.md`.

## Per-merge deletion — EDIT-PICK2's spec (2026-09-16)

Recoverable at `git show 121608392:docs/EDIT-PICK2-SPEC.md` (PR
#2746's fix-pass head, carrying both amendment sections). EDIT's second
kernel unit on the pick door, and the second whose central premise
fell: the spec ruled the conjunction of the item's second and third
shapes (MEET ∧ INFORM) and rejected the third alone; the implementer
measured, before building on it, that MEET admits an out-of-range
barycentric, the hit point then leaves the closed triangle, and the
early-out's premise breaks — order independence traded, the one thing
the spec said would not be. The orchestrator re-ruled to the closed
comparison ∧ INFORM, which the amendment records with the three-rule
table that forced it. The spec's example ray was also mis-stated
(informative, not uninformative, under the unit's own bound) and its
residue-row figure was wrong by five orders (2.7e-11 for 7.19e-16),
found by both reviewers. What survived: the demand that the bound be
derived and never tuned, and that every corpus claim be a row — both
of which are what caught the spec. The residue is one ruling row,
`what-t-the-pick-door-answers-and-with-what-width`, which the
orchestrator has now been wrong about twice and so puts to Ev.

## Per-merge deletion — PROPS sphere-pole-side's spec (2026-09-16)

Recoverable at `git show 1b7cfb766:docs/PROPS-SPHERE-POLE-SIDE-SPEC.md` (the
merge of main into the unit head, before the state-sync commit that
deleted it), including its 2026-09-14 amendment. Its construction held
and its amendment did its job — the new predicate's recorded verdicts
are a face fact under re-anchoring, which the amendment demanded in
advance precisely because two neighbouring predicates are not. Three
of its sentences did not survive. It asserted that `du_of_rims` already
sums a full rim to `τ`, which is where the unit's own MAJOR came from,
so the spec is the first author of that defect. It said the
material-sign gate should take the predicate, which the unit showed it
cannot and the dual showed it can in a weaker sense-free form. And it
named the die's pips as the rim-only shape to pin, which they are not —
a pip ball is revolved and carries a seam meridian. The rule above; the
unit's record is its three items' `## Closed` sections, the measurement
left on the fourth, its row in `MODEL-AB-LOG.md` (ordinal 2406, sample
#214) and the MERGED entry in `work/props/log.md`.

- `PROPS-SPHERE-POLE-SIDE-SPEC.md` — PROPS sphere-pole-side, a rim's traversal names the side its face's interior lies on (#2741)

- `EXCH-H1-SPEC.md` — EXCH-H1, degree-1 line promotion and the
  seam-class Line limb (#1798). Deleted at merge per the spec
  lifecycle; recoverable at `6ebcef1fd`. The unit's record is
  `work/exch/EXCH-H1.md`, the parent issue's `## Closed`, its row in
  `MODEL-AB-LOG.md` (ordinal 2100) and the landed entry in
  `work/exch/log.md`. The §Re-scope ruling (the rung lives in
  `run_iso_checks`' seam class, not `nurbs_iso_derive`) travels in
  the log's adjudication entry.

## Per-merge deletion — MSOLVE-6's spec (2026-09-19)

Recoverable at `git show 71e92e224b64ce2a20117136282950cd10ca0728:docs/MSOLVE-6-SPEC.md`
(the MSOLVE-6 unit head, before the state-sync commit that deleted it),
including its "Amendment — the edit door" section, ruled by Ev on
`[ev]` PR 2118. Its construction held: the lever, the reach trait and
the edit door landed as written, with three deviations argued on the
PR (`split`/`inline` take a resolver rather than a reach, because the
part being minted is what no store holds yet; the edit refuses only
where the prior solve reached NO verdict, and keeps the cluster's
frame where it DECIDED there is no pose; `MateReach` is keyed by the
part, not the instance). The orchestrator ruled against one of its
sentences: the spec asked `lever_arm`'s doc to "keep the story" of
the retired floor, and the implementer discipline's rule against
retired-code archaeology in docs wins — the doc is two sentences.
The rule above; the unit's record is its item's `## Closed` section
and its MERGED entry in `work/msolve/log.md` (no A/B row: the program
runs none).

- `MSOLVE-6-SPEC.md` — MSOLVE-6, the mate's lever is the mated parts' own extent (#2116)

## Per-merge deletion — MSOLVE-7's spec (2026-09-19)

Recoverable at `git show dcf5e149d5f7ef97154473f499011b54f27e03c3:docs/MSOLVE-7-SPEC.md`
(the MSOLVE-7 unit head, before the state-sync commit that deleted
it; unamended). Its construction held; three of its sentences did
not survive measurement. It named `fold_pair` as `derived_offset`'s
caller, which is `pair_left_factor` (the lane took the better
letter). It said a delete's dependents cascade, which the door
measures as a `DeleteWouldDangle` refusal — the condition is
unreachable either way, as the spec said. And its §2 seated only the
dangling-input shape at the transform, while a transform over a
DATUM as the axis is reachable through `apply` and was seated at the
pattern on one road and the transform on the other; the fix pass
seated it at the transform through the one classifier, which is the
spec's thesis carried one shape further. The rule above; the unit's
record is its item's `## Closed` section and its MERGED entry in
`work/msolve/log.md` (no A/B row: the program runs none).

- `MSOLVE-7-SPEC.md` — MSOLVE-7, the member walk's residue: one environment, one seat, one account, one attribute (#2885)

## Per-merge deletion - PROPS escalation-channel's spec (2026-09-20)

Recoverable at `git show 3502371ec:docs/PROPS-ESCALATION-CHANNEL-SPEC.md`
(PR #2928's merge commit, the last head carrying it). A/B row
`escalation`, ordinal 2407, sample #225. The spec's rulings are the
unit's record together with the two closed items
(`work/props/escalation-channel-misses-op-minted-indeterminates.md`,
`work/props/indeterminate-error-arms-sweep.md`), the MERGED entry in
`work/props/log.md`, and the five residues it left:
`work/props/should-classify-replays-error-enum-arms-be-deleted.md`,
`work/props/the-gating-corpus-reaches-no-collapsed-arm-gate.md`,
`work/props/nurbs-span-meter-cannot-tell-a-reversed-domain-from-a-collapsed-one.md`,
`work/curved/topo-mints-indeterminates-outside-the-funnel.md` and
`work/msolve/mate-lane-escalations-reach-no-nodes-log.md`.

**Two spec notes at deletion, both cases of the spec asserting a premise
instead of requiring a decide** - the same fault the sphere-pole-side
spec shipped a MAJOR through, recorded here so the pattern is visible
across units rather than once per unit:

- The spec's **Posture** asserted "Recorded-verdict populations WILL
  move". They did not. The unit measured zero movement and made the zero
  its receipt, which was the right answer; had it obeyed the spec
  instead of measuring, it would have re-baselined goldens that nothing
  had moved.
- The spec inherited its item's **"roughly forty error variants across
  five crates"** as fact. The true population is 88 across nine crates,
  the item's stated recipe yields 83, and the written-down 82 came from
  neither - a third ad-hoc script whose enclosing-enum regex silently
  dropped `PropsError::Escalated`. A measurement whose stated recipe
  does not reproduce it is not a measurement; the unit said so in the
  item and re-derived all three figures.

A third note, on the spec's **order** (the channel first, the sweep
second, the sweep scoped by what the channel then carries): the order
held and was worth ruling, but the sweep it scoped inverted the item's
premise rather than executing it, retiring nothing. Both blinded
reviewers independently hunted the counterexample - a variant whose
every construction and consumer sits inside an open bracket - and both
failed to find one, which is stronger evidence for the 0-retired
conclusion than the argument that predicted it.

## Per-merge deletion — MSOLVE-8's spec (2026-09-20)

Recoverable at `git show 399d411c87031e18c2e9135c44fd02c4c66c7b76:docs/MSOLVE-8-SPEC.md`
(the MSOLVE-8 unit head, before the state-sync commit that deleted
it; unamended). Its construction held; two of its sentences were
overruled by measurement and one of its clauses was tested. Its §3
offered a re-mint road "where geom-core admits no witnessed
transport" and fenced geom-core out; the lane took that road for the
frame read too, and the reviews measured its cost (the aim decided
three times per mate under one funnel name, a copied refusal
projection), so the orchestrator widened the fence by one door —
geom-core's `point_at_frame`, the witness `point_at` already built.
Its §3 also let a rotated direction be "re-minted under the run's
band", which on the planar-pair line was a SECOND decision two ulps
from `parallel`'s and moved ten door-built documents from UNDER to
Indeterminate at the boundary (the correctness arm's MAJOR); the fix
is one decision, `parallel` minting the witness it decides on. And
its stop clause named the condition the lane met (`OrthoFrame`
reproducing `point_at` bit for bit) and was argued past rather than
honoured — recorded in the log as a process lapse; the ruling settled
it. The rule above; the unit's record is its item's `## Closed`
section and its MERGED entry in `work/msolve/log.md` (no A/B row: the
program runs none).

- `MSOLVE-8-SPEC.md` — MSOLVE-8, a levered clash names its arm; the coset's directions carry the witness; `MateFault` names its consumers (#2896)

## Per-merge deletion — MSOLVE-10's spec (2026-09-20)

Recoverable at `git show 392fc0ad95e6a7ec044ca9a18bb99f125d3c56cd:docs/MSOLVE-10-SPEC.md`
(the MSOLVE-10 unit head, before the state-sync commit that deleted
it; unamended). Its construction held: `admit_mate` is the per-mate
prefix of the solve and the insert door asks it. Five of its
sentences were overruled by the reviews. Its §2 and §5 said a mate
the table refuses "never enters the document"; the load door's
snapshot walk asks only the non-finite predicate, and the ruling
kept it so — the doors decide edits and the solve decides states, a
state a mate comes to hold after insert is the solve's, and a `save`
that refused states the doors produced would be a trap — so every
such sentence reads "refused at the insert door" and one row pins
the snapshot road. Its stop clause ("changing what the solve decides
for any mate") did not name the pair the fold never reads (two
members over one instance), whose datum the solve never decides and
the door now refuses; ruled the door's, on the datum alone, and
pinned. Its §4 had the viewer refuse the table's static gaps in the
table's words, which the lane read as a second match over the pair;
the fence widened by one `pub fn table_gap` in `mate.rs` so the
table has one home. Its §1 wanted `fold_pair` to call `admit_mate`
literally; the honest shape is shared doors (`check_references`,
`admit_class`, `mate_coset`) in the solve's own order, and the
restatement the first shape left (the solve's first loop) was
removed. Its §3's replay sentence was right and the lane's first
comments on it were not (`Maintain::Never` refuses at the
maintenance; `Recorded` re-applies); the two replay rules are stated
once at `Maintain::reach`. The unit's record is its item's `## Closed`
section and its MERGED entry in `work/msolve/log.md` (no A/B row: the
program runs none).

- `MSOLVE-10-SPEC.md` — MSOLVE-10, a mate the coset table refuses on its own is refused at the edit door (#2913)

## Per-merge deletion — TRIM-2's spec (2026-09-20)

Recoverable at `git show d0e577121:docs/TRIM-2-SPEC.md` (PR #2863's
merge commit, the last head carrying it). Both PRs delivered: PR-1 (#2564, the trimmed-region
quadrature, A/B row T2Q) and PR-2 (#2863, the tessellation arms, A/B
row T2T). The spec's §8 Rulings (the rectangle certificate as the
all-iso fast path; the Newton–Cotes window fenced; the `Fitted` mesh
arms stay; the item corrected by PR-1; PR-2's seam TESS's) and its
Amendments (§4's PROPS gate with the week's-silence fallback, closed on
Ev's ruling that PROPS was paused) are the unit's record together with
the closed item `work/trim/general-pcurve-face-props-and-tess-refuse.md`,
the two MERGED entries in `work/trim/log.md`, and the residues left on
the program (`curved-trim-e2e-fixture-waits-for-a-producer`,
`trimmed-quadrature-composite-rounds`,
`chord-count-arithmetic-is-plain-f64-across-every-speed-arm`).
Spec note at deletion: §3's survey sentence that `nurbs_tighten`
"skips `Harmonic`" was wrong (the harmonic arm answers a UV speed
bound); PR-2's spiric-adjacent refusal at that site was the right
disposition and the spiric spec's PR-1b said so.

## Per-merge deletion — CURVE3-JET's spec (2026-09-21)

Recoverable at `git show 1960e94732:docs/CURVE3-JET-SPEC.md` (the
fix-pass head). Its sentences that did not survive: "`param_near`'s
`Circle` arm … NOT a site" — folded in the fix pass (R1 showed it was
the one remaining same-receiver pair in production and that the door
removes exactly the frame duplication the item priced), so the class is
closed at fourteen sites and fifteen pairs; "`Circle` → ONE
`azimuth::frame` and both its fields" — the point half goes through a
private `circle_point(center, &frame, radius)` that `circle_at` also
calls, so `eval` and `ders1` share one expression rather than two
spellings; "the `ders` walk with `ders1_in_span` … and two hulls" —
the located-span walk is written once (`located_walk(t, door, hull)`)
and `eval`, `deriv`, `ders1`, `ders` are each one call of it, bit
identity pinned by both `span_bit_identity` digests unchanged; "a
one-ulp mutant … also reds the `topo` tier-3 rows or the tour" — it
does not, structurally: a NURBS-carried `Curve3` cannot reach the
enum-door sites through any user program today (the boolean refuses
NURBS input carriers before tier 3), which both arms reproduced end to
end, and the closing row is a curved-boolean row on CURVED's ground;
"the `Tangent` arm's interior branch" fold — the arm is decided once,
each `match` arm taking its point through its own door. Recorded in
the PR body and the unit's `## Closed` section.

- `CURVE3-JET-SPEC.md` — CURVE3-JET, the whole-curve order-1 jet door `ders1`; the eval/deriv pairs fold onto it (#2708)
## Per-merge deletion — RING-1's spec (2026-09-21)

Recoverable at `git show cdf3767ed7:docs/RING-1-SPEC.md` (the fix-pass
head). Its sentences that did not survive: "the ten `cfg(test)` sites
in `geom-core` stay" — there are nine, all under `#[cfg(test)]`, and
none became unconditional by construction; "the 47 cfg sites in the 23
other `src` files" — 48, re-derived after the merge of `origin/main`
(plus four in `demos/tour` and 126 test files carrying the crate-level
gate), RING-3's starting count; "the feature gates the kernel's
instantiation" — over-claims: what it gates is the lane-trait impls
above `geom-core` and the interval test files, and both reviews showed
end to end that the lane traits, not the feature, are what keeps a
default-build caller out of the kernel doors at `Interval`
(`chart_region_overlap::<Interval>` — `Decide + CertifiedBounds` —
instantiates and runs in a default build; `validate_geometric::
<Interval>` refuses on `PropsQuadLane`); "the same crate is already an
unconditional dev-dependency" — the duplicate `[dev-dependencies]`
entry is removed, the normal dependency being the same edge; "if
`test-features-dev-only.sh` asserts that `interval-transcendentals` is
dev-only or optional" — it does not (its subject is `test-support` /
`*-testing` features on non-dev edges), nothing in it changed;
"`crates/geom-core/README.md`'s scalar list, if it names the gate" — it
does not. Recorded in the PR body and the unit's `## Closed` section.

- `RING-1-SPEC.md` — RING-1, `geom_core::interval` compiles unconditionally; the feature gates only the instantiation (#2971)
