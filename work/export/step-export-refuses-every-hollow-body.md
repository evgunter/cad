---
id: step-export-refuses-every-hollow-body
kind: issue
title: STEP export refuses every hollow body - a void shell is refused VoidShellUnsupported because the writer has no BREP_WITH_VOIDS
status: open
opened: 2026-09-28
priority: P3
cost: M
refs: [step-export-reclassifies-shell-roles-with-its-own-planar-flux]
---

Found while reviewing S-DUP's `step-export-reclassifies-shell-roles-with-its-own-planar-flux`
(PR 3393). That row's fix unblocks classification of curved
multi-shell solids, but not their export. SHELL's log (`work/shell/log.md`,
~:368) named this gap "for EXCH's board", and no row was ever filed
for it.

`crates/step-export/src/writer.rs`, `manifold_solids`, refuses any
multi-shell solid that has a shell of negative signed volume, with
`StepExportError::VoidShellUnsupported`. The crate docs
(`crates/step-export/src/lib.rs` ~:163–169) justify this: "`BREP_WITH_VOIDS`
needs a void-to-containing-shell association the kernel does not yet
record … Voided bodies wait for that designation (M5+)".
`crates/step-export/tests/export.rs`, `voided_body_refuses_typed`,
pins the refusal. So every body the shell verb builds, and every
boolean that leaves a cavity, cannot be written to STEP. Nothing on
the import side reads `BREP_WITH_VOIDS` either: `git grep -i
brep_with_voids crates/step-import` finds nothing. The bank file
`FW13_full_laptop.stp` carries one (`docs/STEP-BANK.md`).

**What the owner weighs.** Whether the kernel now records the
association the docs say is missing. A solid's shells are its outer
boundaries plus its voids, and `topo::classify_shells_of` reads the
roles. Where a solid has exactly one `Outer`, the association is the
solid itself, and the writer can emit `BREP_WITH_VOIDS(outer,
(voids…))`. That covers every shell-verb output. Several `Outer`s
under one solid are legitimate, and an island inside its own solid's
cavity is valid (`crates/topo/src/validate.rs` ~:3607, check 10's
"what check 10 does not refuse is deliberate"). Those cases need a
nesting reading to pair each void with its outer. Check 10's winding
walk already takes that reading.

**Band.** P3 by the README's "interop with other tools". Raise it if
the owner reads "the shell verb's ordinary output cannot leave the
tree" as a normal verb broken on normal geometry (P0).
