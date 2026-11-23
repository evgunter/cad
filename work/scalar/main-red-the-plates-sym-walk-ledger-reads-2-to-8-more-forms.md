---
id: main-red-the-plates-sym-walk-ledger-reads-2-to-8-more-forms
kind: issue
title: main-red: the plate's Sym walk ledger reads 2–8 more forms than pinned
status: closed
closed: 2026-11-23
opened: 2026-11-21
priority: P2
cost: E
---


Found by the ONEHOME operands lane (PR 7206), measured on `origin/main`
at `b5c3fb4b0`.

`crates/editor-core/tests/m10_sym_profile_interval.rs::the_forms_the_plates_walks_build_are_pinned`
is in the slow set (`.config/nextest.toml`). On main it fails at
`m10_sym_profile_interval.rs:622` ("plate at eps 1e-9: the walks built
different forms"). The ledger, measured against pinned:

| line | measured | pinned |
|---|---|---|
| Plain/Decision | calls 979 forms 39705 frozen 54 digest `3c078f726bf785600e18f9290d698538` | calls 979 forms 39703 frozen 54 digest `7efda4b66066a94e596eb7fa2f430a65` |
| Plain/Assertion | calls 1047 forms 14594 frozen 0 digest `149d505c7ea827531e5b1e17f3c39c3c` | calls 1047 forms 14592 frozen 0 digest `e6c6e5c3cd56ef78f91f2c7626f73670` |
| Early/Decision | calls 308 forms 28232 frozen 0 digest `10ed8def872f6c15f2ccd18ea920e1f7` | the same |
| Early/Assertion | calls 1047 forms 15164 frozen 0 digest `9318ef481605c6889e98b345a2fb7404` | calls 1047 forms 15162 frozen 0 digest `c7fd707ed0a699ee0a5d3f1d55c1fa69` |
| Door/Decision | calls 550 forms 15364 frozen 0 digest `866798cff9ca3bb865f034d6987d4e55` | calls 550 forms 15356 frozen 0 digest `032cdcf5efa6396ea7a39bf7b7a8f1ab` |

Every call count and frozen count matches the pin; four lines build 2
or 8 more forms. The counts are `SymCounts { symbolic: 979,
checks_expanded: 1047, numeric: 1047, frozen: 54, … }`, with no
registration refused or contradicted and no theorem disputed. PR 7206's
branch reads the identical ledger, so the move is main's.

Predecessor: `work/scalar/main-red-the-plates-sym-walk-ledger-reads-12-more-forms.md`
(closed, PR 6663), the same row's earlier drift with other numbers.
Not yet bisected.

## Bisected and re-pinned (GREEN U3, 2026-11-22)

(GREEN, branch `green/editor-1e-12`) The same move as `work/green/main-red-the-plates-sym-forms-ledger-moved-at-1e-12-since-the-ellipse-speed-magnitudes.md`: one pin serves every ε, and the five lines here are the ones that row quotes at 1e-12. It bisects to PR 7183 (`germ/ellipse-speed-magnitudes`, `6d58cebd`), and the 14 forms are the `Abs` and `Max` the PR's conic speed readers build over the plate's symbolic radii. The evidence is on the GREEN row; GREEN's PR re-pins `PLATE_LEDGER` at main's reading, which greens this row at the default ε too.

The same PR found one move inside this row's numbers that is not PR 7183's: `Plain/Decision`'s digest, which PR 7193 (`sect/certificate-by-support`) moved by building the same forms in another order (the GREEN row has the probe). (GREEN, PR 7389's fix pass, 2026-11-22)

## Closed (CIW, 2026-11-23)

The ledger golden is gone. PR 7501 (`ciw/census-digest-goldens`) replaces it with the plate's growth guard, the slab's two-replay determinism and freeze-nothing rows, and the dial differential (no early-walk dial moves a plain line). A form count moving by 2–8 is not a decision moving, so no defect is suspected.
