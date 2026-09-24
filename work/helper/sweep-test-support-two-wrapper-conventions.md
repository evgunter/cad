---
id: sweep-test-support-two-wrapper-conventions
kind: issue
title: sweep::test_support carries two conventions for the scalar and its stated reason is false at three of five sites
status: open
opened: 2026-09-15
priority: P4
cost: E
---

## Finding

- **Where**: `crates/sweep/src/test_support.rs` — `revolved_about_y`,
  `waisted`, `ball_poled_z`, `bowl`, `hemisphere_on_flat_base`, each an
  `f64` wrapper one line over an `_at<T>` twin
- **Importance**: low
- **Confidence**: sure — the five signatures were read, not grepped
- **Raised by**: the reviewer of `S52`'s PR #2639, 2026-09-15

`S52` made the extrusion family (`extruded`, `prism_on`, `prism`,
`prism_at`, `brick`, `block`, `cube`) generic **in place**, with no `f64`
wrapper, and argued in its PR that the module's existing wrapper pairs
exist because those fixtures' scalar is not inferable from their
arguments. **That argument is true at two of the five and false at
three.**

| pair | scalar inferable from arguments? | so the wrapper is |
| --- | --- | --- |
| `waisted(tol)` / `waisted_at<T>(tol)` | no | necessary |
| `bowl(tol)` / `bowl_at<T>(tol)` | no | necessary |
| `revolved_about_y(verts, rev, tol)` | **yes** (`Vec<ProfileVertex<T>>`) | avoidable |
| `ball_poled_z(r, c, tol)` | **yes** (`r: T`, `c: Vec3<T>`) | avoidable |
| `hemisphere_on_flat_base(r, tol)` | **yes** (`r: T`) | avoidable |

So the module carries **two conventions for one question** and neither
is stated at the other's site. The three avoidable ones are exactly the
`cube(l: f64)`-over-`cube_at<T>` shape `SUITE`'s `X4` names, standing in
the module that closes it.

The fix is mechanical — delete the three wrappers, let inference do the
work at the call sites — but it is a separate unit because it touches
every caller of three fixtures and none of it is `S52`'s remainder. What
it needs first is the **rule**, written once in the module header: a
fixture whose scalar an argument pins is generic in place; one whose
scalar only the return type pins keeps an `f64` name so its callers do
not turbofish. Two of these pairs are then the rule and three are the
defect, and a reader can tell which is which.

## `_at` now means two different things in this module (2026-09-15, SUITE/S392)

Added by the reviewer of `S392`'s PR #2650, on the tree that PR leaves.

The rule this row asks for has to settle a second question at the same
time, because the suffix the five pairs above use for **the scalar** is
now also in use for **the placement**:

| door | `_at` means |
| --- | --- |
| `waisted_at<T>`, `bowl_at<T>`, `ball_poled_z_at<T>`, `hemisphere_on_flat_base_at<T>`, `revolved_about_y_at<T>` | at scalar `T` |
| `prism_at(verts, z0, h, tol)` | at station `z0` |
| `loft_prism_at(zs, tol)` | at placements `zs` |
| `rim_arcs_at`, `one_edge_rim_at`, `arcs_at` | at a rim radius and height |

Both senses predate `S392` — `prism_at` and the three `*_at` query
helpers are the placement sense, the five wrapper pairs are the scalar
sense — so this is an accumulation the module already had, not one that
unit introduced; `loft_prism_at` joins the larger of the two groups.
The reviewer's judgement, recorded because it is the part a later lane
would otherwise re-litigate: `loft_prism_at` is a **placement** door and
not a scalar wrapper (a generic loft family is impossible —
`Section = Vec<ProfileLoop<f64>>`), so it is not a sixth member of the
class above.

The consequence for the fix: **deleting the three avoidable scalar
wrappers does not leave `_at` unambiguous**, because the placement
sense stays and is the majority. So the header rule this row wants
should say what `_at` means (the placement, which is the reading the
word carries in English) and what the surviving scalar pairs are called
instead — or accept both senses explicitly and say how a reader tells
them apart from the signature. Either is fine; leaving it unsaid is
what put a reader in front of eleven doors with one suffix and two
meanings.
