---
id: expect-one-solid-on-solids-next-has-twenty-homes
kind: issue
title: expect("one solid") on solids().next() has twenty homes and no door
status: open
opened: 2026-09-22
priority: P3
cost: D
refs: [the-face-to-solid-walk-is-spelled-per-test-file, 2977]
---


`body.solids().next().expect("one solid")` — "the only solid of this
body" spelled as a partial read of an iterator — has **twenty** homes
across `crates/topo` and `crates/sweep`, fourteen of them added by PR
#2977 while it was making check 7's subject the solid. There is no door
for the idiom, unlike the restricted face read next to it
(`Body::faces_of_solid`).

Nothing is broken today: every hit sits beside an assertion that pins
the solid count, so a body that grew a second solid reds on the count
rather than silently answering about the first solid in arena order.
The hazard is that the two are separable — several of these bodies come
from split or boolean products, exactly the family that can be
multi-solid, and a future edit that drops or weakens the neighbouring
count assertion leaves a read that quietly names an arbitrary solid.

**The hit list** (`grep -rn 'expect("one solid")' --include=*.rs crates/`,
line numbers as of this row and allowed to rot):

  - `crates/sweep/tests/m5_pr9c_sphere_doors.rs:240`
  - `crates/sweep/tests/m5_s12_curved_ops.rs:242`
  - `crates/sweep/tests/revert_periodic_wrap.rs:167`
  - `crates/sweep/tests/revert_plane_charts.rs:133`
  - `crates/sweep/tests/review_m3_pr1_sweep.rs:216`
  - `crates/sweep/tests/review_m3_pr1_sweep.rs:279`
  - `crates/sweep/tests/review_s12_adv.rs:291`
  - `crates/sweep/tests/shell5_r1_probes.rs:78`
  - `crates/sweep/tests/shell5_r2_probes.rs:197`
  - `crates/sweep/tests/shell8_multi_solid.rs:334`
  - `crates/sweep/tests/shell9_probe.rs:102`
  - `crates/sweep/tests/shell9_r1_probes.rs:426`
  - `crates/sweep/tests/verbs_shell.rs:416`
  - `crates/sweep/tests/verbs_shell.rs:706`
  - `crates/sweep/tests/verbs_shell.rs:775`
  - `crates/topo/src/tier3_tests.rs:800`
  - `crates/topo/tests/geometric_cube.rs:424`
  - `crates/topo/tests/geometric_cube.rs:447`
  - `crates/topo/tests/m3_pr1_surgery.rs:182`
  - `crates/topo/tests/review_m3_pr1.rs:625`

**The blind spot of that pattern**, checked: the same read under other
messages and other recovery. `grep -rn 'solids()\.next()'` over
`crates/` finds **56** sites in all — the other 36 spell `.unwrap()`,
`.unwrap().0`, `.is_none()` or a message of their own
(`crates/topo/tests/void_door.rs` alone has eleven, and
`crates/topo/src/instance.rs`, `movefac.rs`, `shell10_r2_probes.rs`,
`offset_together.rs`, `crates/sweep/src/blend/surgery.rs` and
`crates/editor-core/src/product.rs` are the non-test homes). A door
would want all 56, not the 20 that share a string.

**The work**: decide whether `Body` should answer "the one solid of
this body" — a `Result`/`Option` door that refuses a body holding none
or several, the way `faces_of_solid` refuses a key that does not
resolve — and if so, sweep the 56 onto it. Filed rather than fixed in
#2977 because twenty-plus call-site rewrites in a unit about check 7's
subject would bury the change that unit is for.

## Re-homed to HELPER, 2026-09-22 (ATREST orchestrator)

Filed on `work/atrest/` by ATREST-1's fix pass, which found it while
making check 7's subject the solid. Moved here by `git mv` with the id
and body unchanged: the class is HELPER's — *one test helper, oracle
or walk with several private homes* — and ATREST's charter is what the
at-rest validator DECIDES, not how a suite spells its topology
helpers.

**Not a duplicate of `the-face-to-solid-walk-is-spelled-per-test-file`,
and both should be read together.** That row is the `face → Face::shell
→ Shell::solid` walk, which now has a door (`Body::solid_of_face`).
This one is `body.solids().next().expect("one solid")` — solid
ENUMERATION under an assumption, not a walk, and no door exists for it
because the thing missing is not a function but a checked premise.

**Why it is more than tidiness, and the part HELPER should not file
under duplication alone.** `.next()` cannot check "one solid". Several
of the twenty bodies come from split and boolean products — exactly
the family that can be multi-solid — and ATREST-1 has just made
multi-solid bodies matter more at rest: tier 3's check 7 now has one
verdict per solid, so a test that silently reads the FIRST solid of a
body it assumed was single now asserts about a subject the validator
distinguishes. Nothing is broken today; the vector-length assertions
in those rows catch it. The point is that the idiom cannot, and it has
twenty homes.

Cross-filed reference, not a claim on HELPER's ordering.
