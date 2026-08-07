---
name: library-program-state
description: LIBRARY-DESIGN ratified 2026-08-06 (#229); dispatch notes for U1/U2 that aren't in the doc
metadata:
  type: project
---

The usable-as-a-library program is RATIFIED as docs/LIBRARY-DESIGN.md
(#229, 2026-08-06; #228 was the companion doc de-rot). Live status
belongs to the doc itself — this memory carries only the
dispatch-context a fresh session would otherwise rebuild:

- **Natural first dispatches**: U1 (façade crate — zero collision
  with anything live, S-shaped) and U2's substrate (PATHS impl,
  v2-fronted per LQ4). Units may run parallel with M7 where
  footprints are independent (LQ5 ruling) — no need to wait for the
  M7 exit walk. Standard process applies: A/B difficulty logged
  BEFORE the draw, blinded adversarial review, PR held.
- **U2 substrate should measure first**: LoopBuilder/profile
  validation doors (crates/profile/src/sugar.rs is the raw layer the
  lowering verifies against), and the editor-core ProfileDesc
  opacity seam (profile_desc.rs) that the v2 switch replaces. The
  #104 thread + PATHS-DESIGN §5 carry the elaboration contract.
- **Evidence trail**: the demo-corpus pain table (file:line) is
  LIBRARY-DESIGN §L2; the verified editor-core shipped-vs-absent
  inventory is GUI-DESIGN's freshness note. Both were verified
  against code 2026-08-06 — re-verify only if those files moved.
- **Acceptance signal to remember at U2/U3 merge time**: the
  triplicated corpora (tour scene / step-export fixture /
  editor-core doc per body) should collapse to one authored source;
  still-triplicated afterward = smell, act on it.
- **LQ residue owners**: LQ3 → U4's measured spec; LQ7 tail (wheel
  cadence, schema/package version coupling) → implementation time.
  Pre-release: NO backcompat machinery anywhere; version reset at
  release (Evan's LQ7 ruling — applies beyond this program).
