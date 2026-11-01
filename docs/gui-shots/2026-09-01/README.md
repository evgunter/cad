# Story-suite screenshots — 2026-09-01

Live captures of the `viewer` app on the documents the three GUI
integration "story" suites author and export through their
`PNCAD_STORY_GALLERY` door, taken with the 2026-08-27 recipe: **Xvfb
`:99`** (1280x800x24), **lavapipe software adapter**, ImageMagick
`import -window root`, `xdotool` cursor/clicks, debug profile.

- **Commit rendered: `c8ee02a91f14ec9068c0b1fd30d674e1f1a20267`** (the
  story-suites branch, pre-merge) — the first commit at which the app
  binary STARTS again: these captures are also the live verification
  of the edge-bias relocation (issue 1451's incident), including an
  edge-selection mark drawn without z-fighting.
- **Launch**: `DISPLAY=:99 target/debug/viewer <doc.pncad>` on each
  exported document; the documents themselves come from
  `PNCAD_STORY_GALLERY=<dir> cargo test -p viewer --test all story_`.

## Shots

| File | What it shows |
| --- | --- |
| `15-rook-open.png` | `story_authoring`'s chess rook open: chamfered plinth, base disc, shaft, crown block and four merlons; all 17 nodes Ok in the tree. |
| `16-rook-face-selected.png` | The shaft face clicked: face highlighted, the shaft's `Extrude` row highlighted in the tree, and the panel showing "face of feature 7", the delete affordance pricing 4 dependents, and the distance slot with its unit chooser. |
| `17-lighthouse-params.png` | `story_parametric`'s lighthouse: three coaxial drums, and the four document parameters (`base_r`, `embed`, `height`, `taper`) the whole part hangs off, listed in the panel. |
| `18-windmill-at-rest-banner.png` | `story_assembly`'s windmill: tower, hub, two sails mated CROSSED (the quarter-turned roll reference). The red banner is the at-rest census refusing the blade-overhang rests — the finding recorded on issue 943's thread, live. |

## Caveat

lavapipe again: evidence the pipeline draws correctly on the software
adapter; the real-hardware half rides issue 1097's checklist.
