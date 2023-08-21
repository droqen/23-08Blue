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