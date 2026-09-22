# Sweep 3 — 2026-08-28: the merged units' specs, and three closed programs

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
in `docs/DOC-LEDGER.md` resolves any of them. One source comment was edited
rather than left, because its tense made a live claim: `crates/mesh/
src/sizing.rs` said `TESS-SPLIT-SPEC.md` "binds its execution", present
tense, of a unit that had merged.
