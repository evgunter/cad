---
id: main-red-the-plates-sym-walk-ledger-reads-2-to-8-more-forms
kind: issue
title: main-red: the plate's Sym walk ledger reads 2–8 more forms than pinned
status: open
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
