# Running the viewer

```sh
cargo run -p viewer --features app -- [document.pncad]
```

The optional path is opened at startup through the same typed `Open`
operation the dialog feeds — and it is the only way to open a document
on a system where no file-chooser backend exists (below).

## Mouse bindings

| Gesture | Action |
| --- | --- |
| LEFT click | select (face pick; also feeds the mate tool's picks) |
| MIDDLE drag | orbit (shift+middle: pan) |
| RIGHT drag | pan |
| scroll | zoom |

Left-drag in the viewport is inert **by design** — primary is reserved
for selection. Moving an instance is not a viewport drag either: the
free-move probe is the typed x/y/z fields in the instance section of
the Properties panel (display-only, mm).

## File dialogs need a system backend

Open… / Save As… go through `rfd`, whose Linux backends are the
XDG desktop portal (`xdg-desktop-portal` plus a working frontend for
your desktop) or, failing that, a `zenity` binary. With neither
present, the dialog **returns nothing** — `rfd` reports a missing
backend and a user cancel identically — so the app can only say "no
file chosen" on the status line rather than which of the two happened.
If Open…/Save As… only ever produce that message, install/start a
portal or install `zenity`, or pass the document path on the command
line as above.

First-light findings behind this note (window sizing, silent dialogs):
issue [#1097](https://github.com/evgunter/cad/issues/1097).
