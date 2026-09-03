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
