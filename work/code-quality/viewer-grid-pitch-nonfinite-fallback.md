---
id: viewer-grid-pitch-nonfinite-fallback
kind: issue
title: grid_pitch's non-finite early exit substitutes the worst value its caller could receive
status: open
opened: 2026-09-03
refs: [D64]
---


## Was

Filed by the `D64`(d) lane, out of the early-exit fallback sweep that
bullet ordered. It is the sweep's ONE hit whose substituted value is a
plausible-looking reading rather than a refusal.

## Finding

`crates/viewer/src/datums.rs`, `grid_pitch`:

```rust
let wanted = metres_per_pixel * TARGET_PITCH_PX;
if !wanted.is_finite() || wanted <= 0.0 {
    return f64::MIN_POSITIVE;
}
```

Every other line of this function is argued at the site — the ratio
comparison, the logarithmic reading, the ladder. This early exit is
not, and it is the only branch that returns a number the rest of the
function did not compute.

**The substituted value is the worst one for the consumer.** The sole
caller takes the drawn patch's half-extent and turns it into index
bounds on multiples of the pitch, `ceil`/`floor` outward. At a pitch of
`f64::MIN_POSITIVE` those bounds are ~1e323 lines per direction: not a
degraded grid, a hang. A refusal — `Option`, or the caller drawing no
grid for a view whose scale is not a number — is what the rest of the
module's discipline implies. The function's own doc says it is "public
because it is the module's one arithmetic claim worth asserting on its
own"; this branch is outside that claim, asserted by nothing
(`crates/viewer/tests/datum_draw.rs` exercises the ladder, not the
guard), and reachable exactly when a view's scale has already gone
wrong.

**No track owns this.** `crates/viewer/` appears in no row of the
territories table in `work/code-quality/plan.md` and in no line of
Track J's retired ground — it is unowned in the sense the `geom-brep`
seam gives the phrase, so this is filed as a finding rather than
routed, and whoever takes it draws the fence in the same PR.

## Sweep context

The sweep this came out of covered every `*.rs` in the tree for the
shape `if <expr>.is_finite()/.is_nan()/.is_infinite() { return|continue|break … }`
— 162 hits over 75 files. The population's overwhelming disposition is
**not this class**: `return Err` (67), `return None`/`Ok(None)` (~35),
a poison value (`RingInterval::poison`, `Self::nai`, `Aabb::poison`,
`f64::NAN`) (~14), a conservative bound (`f64::INFINITY`,
`return true` on a "possibly intersects") (~10), and `continue` inside
a fuzz filter (18). Every one of those is the fail-loud direction and
most carry an argument at the site. This is the residue.

**What that pattern could NOT match**, stated because §C's `C15` asks
it of every sweep and because the load-bearing claim below depends on
it:

- **Multi-line conditions.** The matcher is line-oriented, so a guard
  whose `||` chain wraps across a newline (`if !a.is_finite()` on one
  line, `|| b <= 0.0 {` on the next) is invisible to it.
- **Non-finiteness guards spelled without the predicate.** `if x <= 0.0
  { return DEFAULT }` and `if d == 0.0 { return DEFAULT }` are the same
  class and carry no `is_finite`/`is_nan`/`is_infinite` to match on.
- **Fallbacks that contain no `if` at all** — `unwrap_or`,
  `unwrap_or_else`, `unwrap_or_default`, `.max(…)`, `.min(…)`,
  `clamp(…)`. This is the largest hole and the most idiomatic spelling
  of the defect: `grid_pitch`'s own hit would have been a `.max()` in a
  slightly different hand.

**The second sweep, shaped to that blind spot, over Track K's fence**
(`tools/*/src`, `scripts/gates/`): `unwrap_or|.max(|.min(|clamp(`, plus
`is_finite|is_nan|is_infinite|<= 0.0|< 0.0|== 0.0`, plus shell
`${VAR:-default}`. **Inside the fence: zero of this class.** The hit
list, one line per hit:

| Hit | Disposition |
| --- | --- |
| `k-lint/src/lib.rs` `.min(BASELINE_FLOOR_MARGIN)` | Not this class — the rule-2 cap, argued in the module docs and pinned by `threshold_provenance.rs`'s `capped(PROXIMITY_FACTOR)` row. |
| `tess-meter/src/lib.rs` `divisions`' `.max(1.0)` | Not this class — argued in the function's own doc, and the two assertions above it refuse a non-reading loudly rather than substituting one. |
| `tess-meter/src/lib.rs` `patch_cells`' `if du <= 0.0 \|\| dv <= 0.0 { continue }` | Not this class — a geometric exclusion (cell outside the trim box), argued inline; it substitutes nothing. |
| `tess-meter/src/lib.rs` `.min`/`.max` in the cell-box intersection and `best_split_cells` | Not this class — interval arithmetic, not a fallback. |
| `tess-lint/src/lib.rs` `strip_suffix("-dirty").unwrap_or(commit)` | Not this class — string parsing, no measurement substituted. |
| `tess-lint/src/main.rs` ×4 `unwrap_or_else(…bail…)` | Not this class — these exit; they are the fail-loud direction. |
| `k-lint`/`tess-lint` `Admissible` predicates | Not this class — the refusal machinery itself. |
| `scripts/gates/lib.sh` `${TMPDIR:-/tmp}`, `${GITHUB_ACTIONS:-}`, `${BASHPID:-$$}`; `probe-suite-census.sh` `${have:-0}`, `${other:-0}` | Not this class — environment defaults and counter initialisation, not substituted readings. (`probe-suite-census.sh` is outside the fence in any case.) |

**And this sweep's own blind spot:** it is still line-oriented, so a
wrapped `.unwrap_or_else(|| {` whose substituted value is on a later
line is matched only by its head; and neither sweep can see a fallback
routed through a named helper (`fn safe_pitch(…) -> f64`), which has
no syntactic tell at the call site at all.
