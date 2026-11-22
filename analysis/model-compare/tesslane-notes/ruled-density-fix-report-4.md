(4c implementer) Runs on 7a6f5bfe6e, `mesh` whole, slow set included (nextest default profile):
- ε 1e-6: 539/539 passed.
- ε 1e-12: 539/539 passed.
- Default ε: 3293/3294 for mesh+geom-brep, as sent; the one red is geom-brep's fuzz row.
Every run you listed is now done. Three things wait on you:
(1) the curved-neighbour STOP;
(2) the one-line doc fix to `Walked::SharedSidesCut` ("not decided … farther than");
(3) whether to build the undecided-as-greater row.
I'm idle until you rule, and nothing has been pushed past 7a6f5bfe6e.
