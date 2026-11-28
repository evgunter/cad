---
id: censusread
kind: program
title: CENSUSREAD — the census's own reads at rest: the confirm passes and screens, the edge-in-face and curve-touch lanes, the self-touch links, and the material sides a record cites
status: ready
opened: 2026-11-25
group: kernel/boolean/contact
prefix: censusread/
tag: (CENSUSREAD orchestrator)
paths: [crates/topo/src/census.rs, crates/topo/src/census/*, crates/topo/src/link.rs, crates/topo/src/link/*, crates/topo/src/boolean/contact_verify.rs]
keep_out: [opened 2026-11-25 when CONTACTHOLD closed (docs/doc-ledger/contacthold-leaves-the-tracker.md) - its census and link rows moved here by git mv with ids unchanged, ATREST owns what a contact record is and the stage 4 I and stage 5 C closers on the same census.rs - this track owns how the census reads the cells a record or a sweep hands it, HELDRECORDS owns which records a finished body holds and carries - a row here that turns out to need a record a producer does not mint belongs there, census.rs and census/* are shared with ATREST and HELDRECORDS - run scripts/work.py territory on your branch and announce the seam in the PR where the changes are coupled]
---

**How the census reads at rest.** The census (`crates/topo/src/census.rs`,
`census/*`), the self-touch links (`link.rs`, `link/*`) and the contact
verify (`boolean/contact_verify.rs`) read the cells a record or a sweep
hands them and answer a verdict. This track holds the places those
reads are wrong, too wide or silent:

- **confirm passes and screens** that escalate a residual before the
  region would decide, or swallow a lane's side verdict;
- **the edge-in-face and curve-touch lanes** that do not yet exist: a
  line or conic edge lying in a curved face, an edge–face record off a
  line and a plane;
- **the self-touch links**: lens candidates, sheets on a tangent plane,
  lunes, cause arms no row reaches;
- **material sides**: edge–edge, point and coincident-curved-edge
  records whose confirms read no side.

Opened 2026-11-25 from CONTACTHOLD's census rows
(`docs/doc-ledger/contacthold-leaves-the-tracker.md`). No P0 or P1 is on
the slate. Slate and order: `work/censusread/plan.md`; narrative in
`work/censusread/log.md`.
