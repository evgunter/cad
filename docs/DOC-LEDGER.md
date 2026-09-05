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
```

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
  was dropped. One source defect is carried as a flagged
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
`work/<program>/plan.md` / `log.md` (git rename history intact), and
the nine `SMELL-*-LOG.md` track logs are under
`work/code-quality/logs/`. `MODEL-AB-LOG.md` stays in `docs/` as the
experiment log it is. `scripts/work.py lint` refuses a plan or log
reappearing in `docs/`.

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

### Exit walks of closed programs (10 files)

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
