# Ev's words, verbatim — the 2026-10-03 conversation behind ruling `one-way-to-say-dependency-and-intent`

Backup reference only: this file is deleted in the commit after the one
that adds it, and the ruling and `[ev]` PR #3990 point at this commit.
Every message below is Ev's, verbatim, in order. Between them the
assistant proposed and answered; its side is summarized in brackets.

---

**Ev, 1:**

> the way this kernel has pieces depend on one another and on the parameters, and the ways that this kernel lets the user express intent via either structural sharing or explicit declarations, has gotten out of hand. there are many ways to say these things, with no real unifying picture or clear sense of how the next piece that needs to add something like this should do it. and ultimately this cashes out in a ui that is confusing for the user where the constraints feel overbearing ("why do i have to explicitly declare something i already said with numbers?"), rather than a natural system that teaches the user to write things parametrically so that they don't run afoul of the checks at all in normal usage.
>
> i have some inchoate ideas for ways that things could improve; can you weigh in on/push back on them?
>
> * no more nodes that consume other nodes. a boolean takes in two parts that belong to *different spaces*, and places them in the main space. (the user should still be able to view multiple spaces from one document using the "relatively unconstrained" placement display, so they can view and edit something that will go into a boolean next to the existing part, but the placement display can still be used even after the boolean is created, because it doesn't actually reflect the part existing in the same space. in the gui, color is used to distinguish placement displays vs stuff that is actually in the main space
> * no more raw numbers. everything is a variable. (the gui makes it easy to make new variables inline while writing something, and displays the value of the variables conveniently)
> * no more dag edges from node to node. the role played by edges now is replaced by sharing variables
> * no absolute coordinates (this is an especially questionable one). different spaces are just implicit in whether parts are constrained relative to each other. there is no canonical main space. parts don't sit at (0,0,0) in their own space; they just don't have a location (or orientation). adding a mate, e.g., supplies a constraint, and has no position it needs to efface. the location and orientation within the viewer is always set by the placement display info and never anything "real"
> * all the wacky declared tangencies are replaced by good ways to define parts to make them tangent by construction. e.g. if you want to put a cone on a sphere you should be able to say, like, "a cone {with a base with this radius and pointing in this direction, with a base on this plane, with height h and pointing in this direction} x {on this sphere tangent at the base, on this coaxial torus tangent at the base (inner branch), on this coaxial torus tangent at the base (outer branch)}". more concretely this probably looks like setting a slope in the sketch plane based on a derivative of a shape as it appears sliced by the sketch plane, which is far more general as well.
> * if there is a case where you can't make something true by construction (inevitable at least before a solver exists), you declare it as an (in)equality that is checked. these will later feed the solver, and they supersede all declared contacts
> * complaining about undeclared contacts and tangency are highly recommended but optional lints rather than fully entangled with the kernel. there may be a plugin system for lints if necessary, but it'd be simpler for them to just all be checked at rest (perhaps with some kind of caching). the recourse for such lints (at least those which can't be fixed by construction, which we could tell by whether it'd make them overconstrained) is a declared inequality or equation, like for a contact that side A \subset side B (shhh it's an inequality for the partial order on sets don't worry about my bad terminology), or for an overlap that A \cap B has positive volume. (this idea is quite vague so far about which kinds of such declarations should suffice to make the lints quiet, so that would need to be considerably fleshed out.) (i already talked a bit with an agent a while ago about making these into lints, and so there may be some relevant doc floating around, but it might be too outdated to be helpful anyway.)
> * also very curious if you have your own ideas after digging into the architecture! (also, if the architecture is unclear, feel very welcome to do drive-by editing of READMEs not just for rot but also for clarity and concision. feel free to use subagents for this)

[Assistant: proposed "structural = identically zero in the parameters" via the symbolic tier (E12), which needs "no raw numbers" because the tier treats literals as constants; agreed with no consuming but pushed back on "operands always in different spaces"; agreed with no raw numbers with a carve-out for exact structural constants; pushed back on "no edges" (topological references are not scalars; sharing a scalar leaks how A was built) and proposed typed variables with nodes as let-bindings; agreed with no absolute coordinates with a deterministic internal gauge per space; agreed with tangency by construction, noting slice tangency implies surface tangency only when the generator preserves the surface's structure; agreed with checked (in)equalities, noting a declared equality in a boolean also licenses identification, and proposed keeping `require` and `constrain` distinct; split lints by DS2 (at rest: lint; in booleans: refusal for in-band non-identities); proposed that a lint finding is quieted by an assertion on the same measure at the same site whose bound the observation meets; noted mates are placing or declaring by spanning-tree position.]

**Ev, 2:**

> 1. ah, i think i said "operands always in different spaces" but what i meant is "the operands are in a different space *than their output*", motivated by Transform more than Boolean, which has the question of "what happens to the original part?" far more clearly; the answer this would give is "you can't Transform an already placed part, you just determine its placement by relation to other parts, and transforming it after that would be a contradiction anyway; but even when you determine a placement you are still free to determine another placement in order to place two copies of the part, because the placement isn't a hole on the part to be filled, it's a description of how two inherently-unplaced parts relate to each other". that second part is a more uncertain, because i think it would also be reasonable to require an explicit copy in order to place the same part twice, and this might be less confusing, but i think i lean towards the original of like "what the part is shaped like and where it is are completely different questions". "either they are already related, or the boolean carries the constraints that relate them" - we should pick only one of these, and per your argument it should probably be "require that they're already related". "per object" and "per space" might be kind of the same thing, so makes sense.
> 2. i think i agree with the sentiment, but not the literal phrasing; there are cases where a rational factor or rational multiple of pi happens to be the value taken from a continuous space of possible values. i might say that the variable's type should suit its slot; probably in some of your cases there is only one form that makes sense, so no variable, but there might be some where a discrete-valued variable like a sign makes sense. your two suggestions are good, and i'm a bit shocked that parameters are f64 only now instead of also having Expr be usable in a similar way
> 3. that is a great point. i like the idea of nodes disappearing into "a fancy operation on these variables" (one that may produce many more variables, not just one)
> 4. this makes sense! since it does matter for step export, it should arguably be something the user can edit otherwise as well, but i'm much happier to have just one number rather than a profusion of them. we could even make it so that the world frame is a special undeletable node that looks like a part in the sense that other things can relate to it kind of like it's a part, but it actually just sets the coordinates
> 5. yep, this is kind of what i was gesturing at with "it's more general"; you can use it to define something that's tangent in only one place. i think coaxiality by using the same axis object covers the normal cases
> 6. i'm not quite sure what you mean by the equation also being an instruction in the boolean case (at least if we go with "the boolean takes two parts that are already placed relative to each other"). for require vs constrain, i think the case you say can't happen for a fully determined geometry, as would be necessary before the solver exists anyway?
> 7. this makes sense--yes, i was referring to the cases where right now we already know the answer but require the user to also say it, which in-band never is
>
>
> i think your lint suggestion makes sense. my main concern is it being easier to write vacuous constraints than good ones, but if they have to turn on zero then i think this won't be the case
>
> i'm surprised that mates work that way today. i like your idea; we shouldn't call assertions "mates", though; i think it is probably best to reserve that term for the placement case which, iirc, *does* have a solver today.
>
> thanks so much, this is really shaping up!

[Assistant: placement as a bundle of relations pinning one copy (two placements = two copies; a relation on a pinned copy is an overconstraint); variable types suit the slot; world node read only at export, internal computing frame per space independent of it; dropped the "instruction" claim except for imported geometry; require vs constrain matters only for the future solver's DOF counting; vacuity prevented by requiring the quieting assertion to be an equality at zero; proposed a design doc, two-designer weighing, an `[ev]` PR, and a README pass.]

**Ev, 3:**

> 1. sounds good!
> 2. also sounds good!
> 3. cool
> 4. interesting point on internal coordinates. it certainly makes sense to use local coordinates when available. i guess i would've expected there to still be some freedom in the choice of those coordinates. but it is fine to have none of those choices depend on the world node
> 5. cool
> 6. cool; tbh i think it would be kind of slick for the solver's DOF-counting layer to be clever when the "overconstraint" is actually consistent, but i won't fight over it if it'd be difficult to build or a bad idea for some other reason.
>
>
> re vacuity, my "turn on zero" phrasing was vague, but what i meant was "maybe we don't even allow inequalities to quiet it if they cross zero", which is a somewhat stronger condition than your -inf case
>
> that plan sounds good!

[Assistant: filed the ruling, dispatched the designer pair, explained the internal frame must be a deterministic rule, sketched "redundant but consistent" as a layer-2 downgrade, adopted "a bound that straddles zero quiets nothing", asked about a separate README branch.]

**Ev, 4:**

> can your pr, when you make it, post a note in all `active` orchestrator's logs telling them that there's a big refactor underway and to hold off on starting any units which meaningfully use this stuff (and if that blocks their entire program, to switch its state to "blocked" and stop)
>
> for being clever, we should either have assertions and constraints be the same thing, or not do any particular cleverness with the solver. having constraints that fall back to being assertions seems worse than either
>
> feel free to use whatever branches you want

**Ev, 5:**

> re the orchestrators note, it should specifically say that they CAN finish units they've already started even if they collide

**Ev, 6:**

> heh, very much a nit but everything in this should be p0, since the problem is actively getting worse as more things are built haphazardly, and the work blocks ~everything else until complete

**Ev, 7:**

> oh by the way, could you include this conversation or at least my parts of it verbatim in some way that designers trying to figure out whether i specifically endorsed some principle will find it? pr comment, git commit text, file that is deleted but is referenced by commit, whatever as long as it's findable (and doesn't live as a file forever, since your design doc updates do that and this would just be backup reference)
