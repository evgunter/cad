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

What did not build is in the PR that added this page.

## Making them again

```sh
# per kernel checkout and group (mixed, knife or chain):
CARGO_TARGET_DIR=<dir outside the tree> docs/renders/band-2026-11/make-stl.sh <checkout> <group> <size> <stl-dir>
# then, with a sizes.txt of `<scene> <size>` lines in <stl-dir>:
demos/.venv/bin/python -I docs/renders/band-2026-11/draw.py <stl-dir> <png-dir>
```

`make-stl.sh` builds `scenes.rs` in a throwaway crate outside the repo,
so this directory adds no cargo root.
