---
id: rule-ds-closed-form-cache-is-keyed-without-the-rules
kind: issue
title: Rule D's closed-form cache (Session::trig_closed) is keyed by the argument's digest alone, so a walk under other rules is handed a closed form built under the first walk's
status: open
opened: 2026-10-06
priority: P3
cost: M
refs: [DECIDE-10]
---


Found by DECIDE-10's FULL review (NOTE N1), and filed by DECIDE-10's
fix pass. The cache pre-dates DECIDE-10.

`trig::closed_forms` (`crates/geom-core/src/sym/trig.rs`) memoizes
`(cos kψ, sin kψ)` per session in `Session::trig_closed`. The key is
the argument form's digest alone. What it builds depends on the rules
in force:
- `sqrt_atom` runs rule E's normalisation under `common_factor`;
- it mints through `root::mint`, which takes rule G's dials and, under
  `signed_root`, a certified sign read whose gate `Closed::gated`
  carries.

Every walk in a session shares the cache. So a walk under different
rules is handed the closed form the first walk to reach that argument
built:
- a retry attempt (`SymRetry::kept_atom` masks rule G);
- the decision path's walk with the reads shut
  (`SymRules::without_value_reads`).

With rule C on, a gated closed form can reach the shut walk. That walk
counts an ungated zero only, so this can cost it a theorem but never
mislabel one, and `without_value_reads`' doc says so.

The review's rows (`decide/10-review`,
`sym_root_rows::decide10_review::review_rule_c_on_and_the_trig_cache`)
read the same label under `shipped`, `shipped + C`, `shipped + C,
reads shut`, `all` and `all, reads shut`. On those shapes the leak
moves no label.

**Why not keyed by the rules here.** Keying the cache by the rules'
relevant bits changes what a RETRY attempt builds. The kept-atom
ladder's recoveries are pinned (`sym_9_retry_interval`), so that is not
verdict-neutral without a measurement. A fix keys by the bits
`build_closed_forms` reads (`common_factor`, rule G's four dials,
`signed_root`) and the ring bound, and re-takes `sym_9`'s rows.
