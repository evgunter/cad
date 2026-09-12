---
id: viewer-grid-pitch-nonfinite-fallback
kind: issue
title: grid_pitch's non-finite early exit substitutes the worst value its caller could receive
status: closed
opened: 2026-09-03
closed: 2026-09-12
branch: door/grid-pitch-refusal
refs: [1643]
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

**What that pattern could NOT match**, stated because the reviewer
brief asks it of every sweep a PR body reports
(`docs/prompts/reviewer-style-lane.md` §Q1, *"when the PR body reports a
sweep, ask what its pattern could not match"*) and because the
load-bearing claim below depends on it:

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

## Re-homed to DOOR (2026-09-11, the cut in `docs/WORK-TRACKS-2026-09.md` addendum 3)

DOOR collects the rows whose fix is already written in the row — one PR
each, no design question left open. This row is here because a lane can
take it and land it without deciding anything first.

Its class at the cut was **M** — small refusal-shaped signature change,
but caller and tests ripple; unowned, fence drawn in the PR. The class
is a dispatch estimate made by reading the row against the tree on
2026-09-11, not a verdict on the finding, and a lane that finds it wrong
says so in its PR. The id, the `track:` letter where the row carries
one, and the body above are unchanged by the move.

## Refs at code-quality's sweep (2026-09-11)

`work/code-quality/` left the tracker (`docs/DOC-LEDGER.md`, sweep 11)
and its closed rows went with it. `D64` is now cited by its closing PR
1643.

## Closed (2026-09-12) — refused, and the hang claim falsified

`grid_pitch` answers `Option<f64>` and returns `None` for a
`metres_per_pixel * TARGET_PITCH_PX` that is not a positive finite
length.

**The refusal is per MARK, not per datum**, which is the shape the
first cut of this fix got wrong. `View::screen_metres_at` is the
module's one door for it: every mark is a pixel count read into world
metres through that function, and each mark asks at its OWN point —
the ruling and the pitch at the patch's centre, a plane's normal tick
and a frame's arms at the origin, an axis's ticks at each of its two
ends, a point's cross at the position. Those are different depths, so
they refuse separately. A frame the view cannot rule still says which
way it is turned; a view that scales nothing draws nothing.

Three test call sites take `.expect(…)`. Three rows in
`crates/viewer/tests/datum_draw.rs`: the refusal at the door, the
four-kind fixture, and the frame's arms surviving a refused patch.

**What those rows do NOT claim.** `a_view_with_no_finite_scale_draws_nothing`
asserts the invariant for the case its fixture reaches — a view whose
SCALE is infinite — and not for every path to a non-finite position.
There is at least one other, filed as
`inclusive-rule-range-draws-a-line-on-a-nan-count`: a datum ORIGIN at
the end of the number line, under an ordinary camera, still rules two
positions of `NaN`/`inf` per plane-like datum with every door in this
diff answering `Some`. **That door is document data, not a synthetic
view**, which is wider than any fixture here. This row closes on
"`grid_pitch` refuses, and every mark refuses with it", not on "the
module cannot emit a non-finite position".

**The row's severity claim does not hold, and this is worth keeping.**
The consequence of `f64::MIN_POSITIVE` is not a hang. The sole caller's
count is `((last - first) as usize).min(MAX_GRID_LINES)` and
`MAX_GRID_LINES` is 96 — its own doc calls itself *"a backstop for the
arithmetic going wrong at an extreme"*, and it is the backstop that
fires. Rust's float-to-int cast saturates, so an `inf` difference
becomes `usize::MAX` and clamps at 96. Executed twice: a standalone
`rustc` program over the caller's arithmetic, and the drawing itself
through `datums::draws` with the eye at `[f64::MAX; 3]`. What is
actually drawn is **390 positions — 97 lines each way plus the normal
tick — every coordinate `NaN`** (a basis vector's zero component times
an infinite offset), instantly. So: right about the smell, wrong about
the severity. A frame of `NaN` geometry handed to the renderer is still
a substituted drawing the module did not compute, and refusing is still
the right answer; but nothing was hanging, and the "~1e323 lines per
direction" arithmetic in the Finding above is unclamped and also too
large by about eighteen decades for any patch a view produces.

**The row's ownership sentence was stale.** `crates/viewer/` is
CHROME's and VIEW's ground now (`crates/viewer/tests/*` also
S-TCOST's and S-TINT's); `work.py territory --base origin/main`
reports all four. Announced in the closing PR rather than a fence
drawn fresh.

**Its class at the cut (`M`) was right.** One public signature, one
new private one, four drawing functions, three test call sites.

### The sweep, and its receipts

Swept `crates/viewer/src` for the three shapes this row's own Sweep
context named as its blind spots, plus the two it named as unmatchable
by any grep. Cited by name, because these move
(`docs/prompts/implementer-discipline.md` §7).

Patterns: `unwrap_or|unwrap_or_else|unwrap_or_default`;
`\.(max|min|clamp)\(`; `(<=|<|==|!=|>) *0\.0`;
`is_finite|is_nan|is_infinite`; `\.or_default\(|\.or_insert`;
`fn (safe|fallback|default|clamped|floored)_` (zero hits — the
named-helper hole this row predicted is empty here).

| Hit | Disposition |
| --- | --- |
| `datums.rs` `grid_pitch`'s `f64::MIN_POSITIVE` | **Fixed** — this row. |
| `datums.rs` `axis_segments`, `point_segments`, `frame_segments`' arms | **Fixed** — the same defect at three more marks, found by the reviewer 40 lines from the one this row named. Measured before the fix, same eye: axis 6 positions first `[NaN, -inf, NaN]`, point 6 first `[-inf, 0.0, 0.0]`. |
| `datums.rs` `View::metres_per_pixel_at`'s `.max(f64::MIN_POSITIVE)` | **Real hit, filed** — `work/chrome/metres-per-pixel-swallows-a-nan-depth.md`. Two arms, NaN depth and zero depth, both leaving as `f64::MIN_POSITIVE`; the fix is a design call in CHROME's house. |
| `datums.rs` `rule_patch`'s `((last - first) as usize).min(MAX_GRID_LINES)` + `for i in 0..=count` | **THIS CLASS after all, and filed**: `work/chrome/inclusive-rule-range-draws-a-line-on-a-nan-count.md`. The `.min(…)` cap IS the documented backstop and IS conservative — but the cast is not, on a difference that is not a number: `inf - inf = NaN`, `NaN as usize = 0`, and the INCLUSIVE range then rules one line at `inf`. A first disposition here cleared the whole expression as conservative; that was wrong, and the correction is the point of this line. Missed twice: once by the integer-receiver filter, once by reading the cap's argument as covering the loop below it. |
| `datums.rs` `unit`'s `else { Vec3::new(1.0, 0.0, 0.0) }` | Not this class — a substituted direction, argued at the site and proved unreachable from its one caller (`basis`: a unit normal crossed with the world axis it is least aligned with has length at least `1/√3`). A census owes the line anyway. |
| `datums.rs` `viewport_px.max(1.0)`, `datum_view`'s `height_px.max(1.0)` | Not this class — a floor on a pixel COUNT, not on a measurement of the world. |
| `gpu.rs` `index_count` and edge `vertices`' `u32::try_from(…).unwrap_or(u32::MAX)` | Same class, unreachable, **filed**: `work/chrome/gpu-index-counts-substitute-u32-max.md`. |
| `scene.rs` degenerate triangle normal `else { [0.0, 0.0, 1.0] }` | Same class, milder, **filed**: `work/chrome/degenerate-triangle-normal-is-substituted.md`. |
| `bounds.rs` `Probe::new`'s non-finite/zero seed → `1.0` | Not this class — argued at the site with the consumer named: *"a probe that refused would leave the panel with nothing to say about a field whose scale it could not guess."* |
| `sketch.rs` `arc_points`' `chord <= 0.0 \|\| radius <= 0.0` → `MAX_ARC_POINTS` | Not this class — conservative direction (maximum subdivision) and a `usize` cap; a NaN falls through to `1` via the saturating cast. |
| `sketch.rs` `tip_mark`'s `else { 0.0 }` | Not this class — argued, and zero is a refusal ("gets no marks"). |
| `sketch.rs` `arc_points`' `ratio.clamp(-1.0, 1.0).acos()` | Not this class — an `acos` domain guard. |
| `camera.rs` `fitted`, `projection_matrix`, `CameraOp::Dolly`, `sphere`, `ray`, `finite`/`op_finite`, `project` | Not this class — every one returns `Err(CameraError…)`. `camera.rs` is the fail-loud direction throughout and is the module this one should have looked like. |
| `camera.rs` `near()`'s `.max(floor)`, `clamp_pitch`, `clamp_distance` | Not this class — documented range limits on camera STATE. |
| `pickindex.rs` `segment_distance_px`, the ray-segment `t_segment`/`t_ray` `else { 0.0 }` | Not this class — the correct parameter for a degenerate segment or a parallel ray. |
| `pickindex.rs` `partial_cmp(…).unwrap_or(Ordering::Equal)` | Not this class — a total-order adapter, with the comment above asserting the NaN cannot arise. |
| `app.rs` `stack <= 0.0` → `return`, and the features-share `clamp` | Not this class — a refusal, and a share bounded into its legal range. |
| `input.rs` `world_per_px`'s `height_px <= 0.0` → `None` | Not this class — **it is the shape this fix adopts**, already in the tree. |
| `display.rs` `is_rigid`'s `!frame.is_finite()` → `false` | Not this class — a predicate refusing. |
| `scene.rs` `Delta::new` | Not this class — `Err(SceneError::InvalidDisplayTolerance)`. |
| `scene.rs` `Scene::nowhere`'s `unwrap_or_else(Aabb::poison)` | Not this class — poison, argued; this row's own census counts poison as fail-loud. |
| `props.rs`, `pane/view.rs`, `pane/create.rs`, `pane/viewport.rs`, `marks.rs`, `pickindex.rs`' `in_target`, `scene.rs`' id lookups, `session.rs`, `session/select.rs`, `tree.rs`, `frame.rs`' serial, `pane/features.rs`' indent cap, `session/delete.rs`, `theme.rs`' sRGB transfer, `input.rs`' viewport and motion guards, `widgets.rs`, `bounds.rs`' offset sign | Not this class — empty strings and slices, "nothing selected", map/counter initialisation, provenance defaults, a piecewise transfer function, and early-outs on no motion. None substitutes a measurement. |

**What this sweep could not match.** (a) The enumeration is
line-oriented, so a wrapped `.unwrap_or_else(|| {` is found by its head
only — mitigated by reading each hit's context, not by the pattern.
(b) The `.max`/`.min` list was filtered of integer-looking receivers,
which is how `grid`'s own `.min(MAX_GRID_LINES)` was missed on the
first pass; the row above is the repair, but a third such site
elsewhere in the crate would still be invisible to the pattern as
written. (c) A `match` arm `_ => <constant>` is not a shape either grep
matches. (d) `crates/viewer/tests/*` and every crate outside
`crates/viewer/src` were not swept — the fence, and this row's first
sweep already covered the tree for the `is_finite` spelling.
(e) Accurate as of `origin/main` at `67db568`.

### Filed

- `work/chrome/metres-per-pixel-swallows-a-nan-depth.md` — three arms
  now, not two: NaN depth, zero depth, and a NaN through `look_at`
  that keeps the scale legitimate while making the patch centre NaN
  (plane 6 positions / 4 non-finite, frame 18 / 4, axis 6 / 6).
- `work/chrome/inclusive-rule-range-draws-a-line-on-a-nan-count.md`
- `work/chrome/gpu-index-counts-substitute-u32-max.md`
- `work/chrome/degenerate-triangle-normal-is-substituted.md` — recast:
  the substitution IS argued at the site, with the consumer named, and
  an earlier draft of that row said otherwise. What is open is the
  missing contract in `mesh`, not the viewer's answer.

### Recorded, not scheduled

- The viewer now carries several spellings of "this view has no usable
  scale" — `camera.rs`' `Err`, `input.rs`' `world_per_px` `None`,
  `datums.rs`' `screen_metres_at`/`grid_pitch` `None`, and the floor
  and clamps this census clears as off-class. Consolidating them is a
  CHROME/VIEW question about one crate's refusal vocabulary, not a
  defect any one of them has.
- `datum_draw.rs` pins `grid_pitch(f64::MIN_POSITIVE)` as a legitimate
  reading, which it is at that door — and it is also the value the
  floor one call up substitutes. The filed row above is where that is
  answered; the pin is correct either way.
- **A plane can now lose its normal tick while still ruling.** It is
  the one place per-mark refusal costs the picture's MEANING rather
  than decoration — the tick is the drawing's only statement of which
  side is which. Argued at the site now (it was not, while the frame's
  analogue was argued at length): a tick drawn at a length the view
  did not give it does not say which way the plane faces either.
- **`half_patch_at`'s `viewport_px.max(1.0)` sits upstream of the
  refusal** and launders a NaN viewport into `1.0` before the door
  sees it, exactly as `metres_per_pixel_at`'s floor does one level
  further out. Same shape as the filed row, one argument short of
  being the same finding; noted so a lane fixing the floor knows there
  is a second one beside it.
- **`grid_pitch` keeps its own copy of the refusal** rather than going
  through `screen_metres_at`, so `grid` reaches past its own door for
  the pitch. That is deliberate — `grid_pitch` is public and asserted
  on directly, and routing it through a private helper would make the
  public claim depend on one — but it does mean the module has two
  places the condition is written.
