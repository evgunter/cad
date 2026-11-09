---
id: census-roster-shape-rules-miss-a-type-alias-a-string-row-and-scope-a-file-wide-exemption
kind: issue
title: The census rosters' shape rules miss a Band type alias and a String row, and the census's elsewhere exemption is file-wide and fires on a struct field
status: closed
opened: 2026-11-09
closed: 2026-11-09
pr: 6640
---


Found by the delta review of PR 6627 (`onecopy/census-rosters`, 4c9538aa). All NOTE-grade; that PR merged with them open.

1. **A `Band` type alias.** `names_band` (least-band-on-a-door.sh, BAND_TAKERS) matches the identifier `Band` only. `pub type Bnd = Band; … band: Bnd` stays silent, and neither header names the blind spot.
2. **A row taken as `String`.** The census's `row_like` does not match it.
3. **The "elsewhere" exemption is file-wide and broader than its header says.** A `use super::*` child calling `choose("n", m, "ROW")` in one fn is silenced (zero sites; reproduced) by any of these, anywhere in the same file:
   - `let choose = 3;` in another fn;
   - a struct field `choose: bool`. The `after.starts_with(':')` test fires on fields, and a field cannot rebind a bare call;
   - an unrelated `use x::choose`.

   Scope the exemption to the binding's own block, and do not count fields. The header calls this blind spot "binds or imports that name elsewhere", which does not cover the field case.
4. **Style.** The gate's Band-naming rule and the census's `row_like` rule are two lexical shape rules for one home. k_stats' header now states the contract both rely on.

Nothing in the real tree hits these today: the census still reads the same 27 sites.

**The FIX pass of PR 6640** also reads a closure by its own `|…|` head where an expression starts, scoped to its body; a method binds nothing; a text array or slice is predicate names (so `decide_across` is no row-taker). The class behind the three passes is filed as `the-census-lexical-scope-reader-keeps-minting-blind-spots`.
