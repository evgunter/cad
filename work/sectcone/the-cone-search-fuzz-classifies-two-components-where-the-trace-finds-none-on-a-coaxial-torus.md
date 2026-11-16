---
id: the-cone-search-fuzz-classifies-two-components-where-the-trace-finds-none-on-a-coaxial-torus
kind: issue
title: The cone search's fuzz classifies two components where the traced section is empty, on a coaxial torus (seed 0x748fbae93496066c)
status: open
opened: 2026-11-15
---


Found by BOXFINISH's curved-straddle unit (PR 6979) on a whole-suite
run of `topo`: the fuzzer drew a fresh seed and went red. The unit's
diff is the ray's edge arms (`boolean/solid_contain`), which the section
certificate does not call; the seed replays red on that branch, which
has `origin/main` merged, and was not replayed on a clean main.

**The row.** `crates/topo/src/boolean/section_cert/cone_search.rs`,
`the_cone_arms_agree_with_the_traced_section` (shape 1, a
counterexample search).

**Reproduce.**
`CAD_FUZZ_SEED=0x748fbae93496066c CAD_FUZZ_EFFORT=1 cargo nextest run -p topo --lib -E 'test(the_cone_arms_agree_with_the_traced_section)'`
at ε 1e-9.

**The payload** (one mismatch):

```
coaxial torus: cone apex (123.22201582090372, 137.11760287715344, -103.37349740713759)
axis (-0.27202665779511415, 0.7305438856977157, 0.6263442571928819) α 0.7323530582816763;
coaxial torus z -163.1480262576019 R 126.19347057819449 r 15.255253255175266:
traced [], classified [Component { unbounded: false, essential_f: true, essential_g: true,
witness: None, reaches: None, pad: 0.0 } ×2]
```

The trace finds no section and the classification states two bounded
components essential on both surfaces. Which of the two is wrong is not
measured here: with the torus's centre 163 below the apex along the
axis at α ≈ 0.73, the cone's radius there is about 146, against a tube
from 111 to 141, so the nappe may pass outside the tube (the trace's
reading) and the classified pair would be the other nappe's or none.

**What would close it.** Decide which reading is right at this pose,
fix the other, and pin the pose as a deterministic row beside the fix.
