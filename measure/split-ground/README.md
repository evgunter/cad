# The split-ground measurement (`embedwalk/split-ground-instrument`, not for merge)

`origin/main` at `c85571202` plus `crates/topo/src/split_ground.rs`: every
split the injectivity walk (`topo::clearance::embedded`) makes, recorded
with its reason, its parent, its depth and the seconds spent classifying
the pair; and the at-rest gate's ask (`embed/gate`'s
`AtRestBody::embedded`, each solid against itself at depth 40 and 65,536
cell pairs) ported as a log-and-pass hook in `AtRestBody::validate`. No
verdict changes; with `CAD_SPLIT_GROUND` unset nothing is asked or
recorded.

Reasons: a touching pair's departure exit (`unpresented:` with each
face's presentation state, `Local@<line>` naming the `Unpresented::Local`
site in `crates/topo/src/injective.rs` on this branch; `both-presented:`
with the last undecided margin's class; `stuck-both`, `walled`); a
distance pair's separation class (`lo-at-or-below-zero`, `lo-in-band`,
`wholly-in-band`) and whether its faces share a cell. Site codes:
`13945`/`23945` the plane corner's convexity failing decided-reflex /
undecided; `5EEVV` `present_plane`'s catch-all, a cell holding `EE` edges
and `VV` vertices.

```sh
S=~/.local/share/cad-work/<lane>-scratch   # log goes to $S/log
S=$S measure/split-ground/run.sh topo mesh sweep editor-core
python3 measure/split-ground/sg-summ.py $S/log     # summary.txt
python3 measure/split-ground/sg-tables.py $S/log   # tables.txt
```

`log.tgz` is the run the row cites (`tar xzf log.tgz` gives `log/`).
