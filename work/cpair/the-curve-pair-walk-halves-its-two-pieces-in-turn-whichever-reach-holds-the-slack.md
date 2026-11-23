---
id: the-curve-pair-walk-halves-its-two-pieces-in-turn-whichever-reach-holds-the-slack
kind: issue
title: The curve-pair walk halves its two pieces in turn, whichever piece's reach keeps them from parting
status: open
opened: 2026-11-22
---


Found by BOXFINISH's sweep for the shape its split unit fixed in `editor_core::drive` (`work/boxfinish/a-drive-splits-a-box-along-a-variable-its-undecided-node-does-not-read.md`): a refinement step chosen without asking what the undecided thing depends on. **Read, not measured**: the choice rule below is quoted from the code; that splits are wasted is an argument from what the test reads, and no cell count was taken. (BOXFINISH split-reads lane)

- **`curve_pair::walk`'s `meet`** (`Piece::halves`): the shallower piece is halved first, ties to `a`,
  unless one has stopped. `pieces_apart` reads both reaches; where one span is much longer or faster
  than the other the walk still alternates.
