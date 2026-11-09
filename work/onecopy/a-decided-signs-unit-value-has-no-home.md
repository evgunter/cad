---
id: a-decided-signs-unit-value-has-no-home
kind: issue
title: A ±1 read off a decided Sign is spelled six times, with −1 three ways, and has no NonzeroSign → T home
status: open
opened: 2026-11-09
---


Found by the FULL review of PR 6615 (`onecopy/unit-sign`, head 55adff6b). That PR gave the HULL reading its home, `Real::unit_sign` (`one().copysign(x)`). The DECIDED twin is still spelled by hand: a ±1 picked off a `Sign` that has already been decided, whose Zero arm refuses.

**Sites** (at 55adff6b):
- `geom-brep/src/props/curved.rs:759`: `t_sign`, a private helper.
- `curved.rs:3005`: the same file spells it again in a closure instead of calling `t_sign`.
- `pcurve_cache.rs:11265`
- `pcurve_cache.rs:12017`
- `locus.rs:459`
- `certify.rs:3567`: `NonzeroSign`, `from_f64(±1.0)`.

−1 is spelled three ways: `-T::one()`, `T::zero() - T::one()` and `from_f64(-1.0)`. Each Zero arm differs (a different error each time), so the home wants a `NonzeroSign`, i.e. a decided Sign with no Zero, mapped to `T`. Each caller then keeps its own refusal at the point where it decides.

**Not this class:** PR 6615 already records that the four `select_le_zero(±1, ∓1)` picks decide a tie, which is a different question.

**Also:** `pcurve_cache.rs:11246` binds a local closure NAMED `unit_sign` that decides (levered, Err on undecided). It has the opposite contract to `Real::unit_sign`. Rename it when the home lands.

**The sweep should also cover:**
- `match … { Positive => one, Negative => -one }`, which 6615's second pass did not;
- the three spellings of −1 above.
