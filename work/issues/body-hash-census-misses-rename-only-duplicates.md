---
id: body-hash-census-misses-rename-only-duplicates
kind: issue
title: the body-hash duplicate census misses rename-only twins
status: open
opened: 2026-09-03
---


TCOST-8's second sweep pattern hashed every `fn` block's body TEXT
(whitespace-stripped, lines joined) and grouped by hash, which is what
found the same job under different names (`unit_cylinder`, `zcyl`) that
a name census cannot see. **A textual hash still misses a duplicate
whose only difference is a parameter or local name**, and TCOST-8's PR
body states that blind spot but does not close it.

Confirmed instances in `crates/geom-brep/tests/` (line numbers are
post-TCOST-8):

- **`width`, three spellings of one body.** `arc_eval_anchor.rs:38`
  (`x.hi() - x.lo()`) and `review_arceval_r1_probes.rs` (same) hash
  together; `revolved_point_anchor.rs:37` (`e.hi() - e.lo()`) does not,
  and is the same function under a renamed parameter. TCOST-8 kept the
  first pair apart on a stated reviewer-pair reason and never saw the
  third.
- **The max-of-three-widths variant.** `cert3r1_e2e.rs:69` writes it as
  a closure `|e: Interval| e.hi() - e.lo()` folded with `.max`;
  `r2_cert3_e2e.rs:67` writes the same fold inline over `p.x`/`p.y`/
  `p.z`; `arc_eval_anchor.rs`'s `point_width` is the two-coordinate
  form. Three shapes of "the widest coordinate of an enclosure", none of
  which hash together, and none of which any census in TCOST-7 or
  TCOST-8 reported.

**The obligation this leaves.** The sweep is a class claim, so the
pattern's blind spot is a claim too: *no rename-only duplicate exists*
is unverified across `crates/geom-brep/tests/` and equally across the
crates TCOST-B1 and TCOST-B2 converted to a shared tree, since they were
swept the same way. Closing it wants a normalized hash — alpha-rename
locals and parameters to positional placeholders before hashing, or
compare token streams with identifiers elided — re-run over every crate
that has a `tests/shared/`, with the hit list published the way TCOST-8
published its own.

Not fixed here on purpose: rewriting the census is a tool change whose
output is a new hit list, and a hit list wants a unit that can act on it.
