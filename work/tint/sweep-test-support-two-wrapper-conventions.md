---
id: sweep-test-support-two-wrapper-conventions
kind: issue
title: sweep::test_support carries two conventions for the scalar and its stated reason is false at three of five sites
status: open
opened: 2026-09-15
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
