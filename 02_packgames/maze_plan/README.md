# Maze game
Building off of Fredrik+Tobias' very simple "click to move crab"...
Spawn a simple maze (Complex separate file)
add movement animation (Clientside, should be easy)
add collision (Serverside, should be easy)

## Its parts
on server -
    spawn player object on query.
    just add components to the player object.
on client -
    spawn a 'model' for each player.
    animate model's position according to components.

## Two ways to do input

(way 1)
client - calculate raycast position for camera, click position with math
server - receive click position

(way 2)
client - calculate raycast position for camera
server - receive raycast position, do raycast, convert to click position. do something with click position

i prefer way 1, as it is always better to do as much as possible on the client - add'ly it uses math instead of a physics raycast which is just better

## How to do player movement animation?

I need to do a test prototype, let's call it slowmover_ptt
[working on this now]

* I spent an hour or so on this. It looks and feels good, let's move on soon.

* I spent 10-20 minutes cleaning up the project a little, and spawning a cube map

## Raycast movement - can it possibly feel good, or will it feel awkward and weird...? Let's find out.

i already have a hook for this in the server.
i'm now spawning a collidery cube map... let's start raycasting

* It only took about 15 minutes to get this working! It's not the best, the feedback is a bit weird. But hey, it *does* work.

See `mazemovement.gif`: This is movement working! There's no feedback on a bad move.