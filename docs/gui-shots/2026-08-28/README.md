# GUI-4 assembly-interaction screenshots — 2026-08-28

Live captures of the `viewer` app on an ASSEMBLY document, taken with
the 2026-08-27 recipe unchanged: **Xvfb `:99`** (1280x800x24),
**lavapipe/llvmpipe software adapter** (not real hardware — #1097's
checklist stands), ImageMagick `import -window root`, `xdotool`
cursors, debug profile.

- **Commit rendered: `5cfa7355a30bd805cee2f5803d9fa5ff13b92058`**
  (the GUI-4 branch, pre-merge).
- **Launch**: `DISPLAY=:99 target/debug/viewer <assembly.pncad>` —
  the CLI document argument this unit added (the same typed `Open`
  the dialog feeds; no desktop portal or `zenity` exists headless, so
  the dialog cannot run here).
- **Document**: the GUI-4 suites' gallery-shaped workspace (two part
  documents beside the assembly that pins them; three instances, one
  solved mate), i.e. the exit-walk fixture as saved by the walk row.

## Shots

| File | What it shows |
| --- | --- |
| `10-assembly-open.png` | The assembly OPEN and resolved: three `InstantiatePart` rows plus the `Mate` in the tree (each instance with its `shown` checkbox), and the viewport drawing the free post beside the mated pair — the post seated under the shelf by the SOLVED placement. |
| `11-instance-selected.png` | The unconstrained instance selected: Properties shows the per-instance section — `shown in viewport`, and the free-move probe's mm drags. |
| `12-free-move-probe.png` | After dragging the probe's x by 60 mm: the instance is drawn DISPLACED and in the **violet probe tint** — the G3 visual-distinctness treatment, live — while the mated pair stays grey. The value reads `60.0` in the panel and lives nowhere but display state. |
| `13-shelf-hidden.png` | The shelf's `shown` box unchecked: the shelf leaves the picture (revealing the mated post behind it), while its tree row stays. Hide is display state; the document is untouched. |
| `14-mate-tool-pick.png` | The mate tool active with its first pick held (`pick a: face of instance 2`, the face highlighted): the class choice offered through the kernel's own admission table (`Rest` selected, `admission: Mints`), the primitive radios, the sense toggle, and Commit/Cancel. Below, instance 2's own section shows the typed mate-constrained refusal sentence in place of the free-move drags. |

## Caveat

lavapipe again: evidence the pipeline (including the new per-corner
probe-flag attribute and its tint) draws correctly on the software
adapter; the real-hardware half rides #1097's checklist.
