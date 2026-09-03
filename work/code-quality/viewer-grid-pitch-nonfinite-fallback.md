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
