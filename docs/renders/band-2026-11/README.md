# Band renders — 2026-11

Before-and-after pictures of the mixed-corner step 1, the knife-edge
fillet and the rounded-outline fillet, drawn for Ev on `[ev]` PR 6507.
Each body is authored through `pncad` in `scenes.rs`, tessellated by the
kernel at a chordal δ of 1e-3 and written as binary STL; `draw.py`
rasterizes the STL and draws nothing the kernel did not produce. These
are one-off pictures from one box, not a render lane:
`check_render_provenance.py` does not read this directory.

| File | Kernel | Size |
| --- | --- | --- |
| `lbar-inside.png`, `boss-vertical.png`, `flush-pair.png`, `inset-rim.png` | `band/mixed-vertex-gate` at `1c99535d22`, with `main` at `efef8f397d` merged in locally | 0.2 |
| `valley.png`, `lune-cusp.png` | `main` at `efef8f397d` | 0.3 |
| `rounded-plate.png`, `pocket-floor.png` | `band/closed-mixed-chain` at `4b913f5c9a` | 0.05, 0.14 |
| `bracket-fillet-iso.png` (the bracket filleted all round, two iso views) | `band/bracket-all-three` at `81ea1d1f28` | 0.05 |
| `bracket-fillet-corners.png` (its boss's base near `y = 0.5`: a flush and an inset mixed corner) | `band/bracket-all-three` at `81ea1d1f28` | 0.05 |
| `bracket-chamfer-iso.png` (the bore-less bracket chamfered all round, two iso views) | `band/bracket-all-three` at `81ea1d1f28` | 0.05 |
| `bracket-chamfer-corners.png` (its boss's base near `y = 0.5`) | `band/bracket-all-three` at `81ea1d1f28` | 0.05 |
| `odd-plus-one-flush.png`, `odd-plus-one-inset.png` (the right-angle flush and inset corners, the odd edge with one pair edge chamfered, before and after) | `band/bracket-odd-plus-one`, PR 7038's fix pass | 0.1 |
| `odd-plus-one-flush-120.png` (the flush corner at a 120° plan angle) | `band/bracket-odd-plus-one`, PR 7038's fix pass | 0.1 |
| `odd-plus-one-bracket.png` (the bore-less bracket, the odd edge with one other at all four mixed corners: whole, both corners near `y = 0.5`, the inset one near `y = 3.5`) | `band/bracket-odd-plus-one`, PR 7038's fix pass | 0.05 |

What did not build is in the PR that added this page.

## Making them again

```sh
# per kernel checkout and group (mixed, knife or chain):
CARGO_TARGET_DIR=<dir outside the tree> docs/renders/band-2026-11/make-stl.sh <checkout> <group> <size> <stl-dir>
# then, with a sizes.txt of `<scene> <size>` lines in <stl-dir>:
demos/.venv/bin/python -I docs/renders/band-2026-11/draw.py <stl-dir> <png-dir>
```

The bracket pictures come from triangle soups instead, not STL:
`bracket_soups.rs` tessellates the two bodies at δ = 2e-3 into
`bracket-fillet.soup` and `bracket-chamfer.soup` (committed beside it),
and `demos/.venv/bin/python -I docs/renders/band-2026-11/draw_soup.py
docs/renders/band-2026-11` draws them, each face coloured by its
surface (plane grey, cylinder blue, torus orange, sphere green). The
fix pass on PR 6873 changed no geometry of either body.

The odd-plus-one pictures come from soups too: `odd_plus_one_soups.rs`
reads `band_mixed_corner_odd_plus_one`'s own fixtures, so it runs as a
module of `crates/sweep/tests/all.rs` (its header says how), dumping
each scene's `-before.soup` and `-after.soup` here, and
`demos/.venv/bin/python -I docs/renders/band-2026-11/draw_soup.py
docs/renders/band-2026-11 odd-plus-one` draws each pair side by side.

`make-stl.sh` builds `scenes.rs` in a throwaway crate outside the repo,
so this directory adds no cargo root.
