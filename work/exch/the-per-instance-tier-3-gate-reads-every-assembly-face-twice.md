---
id: the-per-instance-tier-3-gate-reads-every-assembly-face-twice
kind: issue
title: an assembly import reads every face twice: the per-instance tier-3 gate, then the aggregate tier-3' gate
status: open
opened: 2026-09-24
---


Found and executed by ATREST-3's reviewers (PR #3191); filed by that
unit's fix pass.

**The measurement.** A two-instance assembly of certifying solids
records 8 `props_quad*` K-funnel verdicts through `import_step`
against 4 for one `topo::mass_properties` of the imported body. Each
placed instance is gated on its own first (`crates/step-import/src/lib.rs`,
`gate` = `topo::validate_geometric`, inside the instance loop, skipped
only when the file holds one instance), and then the aggregate gate
(`gate3` = `topo::validate_pseudomanifold_certificate`, continued with
`SignCertificate::measure`) walks every face again. `gate3`'s doc now
says so; before ATREST-3's fix pass it claimed "one read of each face".

**The question this opens — an `[ev]` question for EXCH, not a fix.**
The per-instance gate's stated reason is `docs/DESIGN.md` import step 4:
*"whole-body sums (the +V flux) would let an inside-out solid cancel
against a right-side-out neighbour, so each solid is gated on its own
body before aggregation"*. Since ATREST-1, check 7's subject is the
SOLID (`topo::validate` `check7_subjects`), so the aggregate gate
already decides every solid's sign on that solid's own faces and the
cancellation cannot happen there. Whether the per-instance gate still
buys anything the aggregate one does not (a per-instance subject for
the error's `solid` id, a verdict before the graft) is EXCH's to
argue; retiring it would change DESIGN step 4's ratified text, so it
goes to Ev as an `[ev]` PR rather than landing as a cleanup.
