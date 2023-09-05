# 23-08Blue
temp repo for boat game. until august's blue moon.

# Organization
This repo is a collection of both related and unrelated ~~embers~~ packages. What follows is a small tour of the most relevant ones roughly organized in reverse chronological order (at least, at the header level); if you have further questions, contact droqen.

# 03_packthis
Packages created since the 'this' package change was made.

### Usable/re-usable packages
- `ease` -- a lot of components for smooth movement duplicated on server and client without data usage
- `easymover` -- just create a mover and tell it where to go to from where it is now! -- see `usage.rs` -- depends on `ease`
- `instant_camera` -- convenience package that spawns a clientside camera at 5,5,5 looking at 0,0,0 (the default Ambient camera)
- `clicks_auto` -- when the player uses the mouse or its buttons, creates simpler / more accessible clientside objects. must be passed a `camera`

### TODO Sept 5
Let's just wrap this up...
- [x] implement blockable (done! arghhh, the raycast distance thing: https://github.com/AmbientRun/Ambient/issues/797)
- [X] display player score
- [X] implement pickups
    - [X] spawn pickups
    - [X] change player score on pickup
- [X] change serverside player 'level' component according to player score thresholds
    - [ ] client zoom by player level
    - [ ] player movement speed by player level
    - [ ] sprite according to player level

# 01_messembers
Experimenting with using messages for triggering ember functionality, rather than having embers start on their own.
I regard this as a very successful experiment.
`ambient run 01_messembers/demo`

### desert_minigame (1%)
-> The idea with this one was using something like Fredrik & Tobias' click-to-move jam game, but with a sort of abstract "desert sandstorm" environment to navigate. I haven't done much work on it yet, just the very beginnings of a sandstorm particle effect.
### earth_minigame (1%)
-> I wanted to add some camera controls to the "click and shoot" defenders_jamgame, but I haven't done much work on it yet.
### uphill_minigame (50%)
-> You can push the rock up the hill, though it becomes mostly impossible with one person as it gets steep enough.
Too much code to be the frontpage example.

# defenders_jamgame
A quick demo put together on call with Mithun and Magda for the frontpage game idea jam.
`ambient run defenders_jamgame`

# demo_cutefruit
An entry point project which has as its dependencies `a_nice_overhead_camera`, `ww_gen`, `buoy`, and `skatemouse`.
`ambient run demo_cutefruit`

# BoatGameBlue
A collection of ~~embers~~ packages meant to become The Boat Game, abandoned and harvested for parts
