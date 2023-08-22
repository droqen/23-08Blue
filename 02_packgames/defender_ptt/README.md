# Defender prototype

Building off of `23-08Blue/defenders_jamgame`, this is a click and shoot defense game, very simple.

I'm copying two modules from `slowmover_ptt/client.rs`: `camera_factory` and `screen_input`.
Minor tweak necessary to screen_input : Changed the Message being sent.
    I've been thinking about another way to handle this, i.e. could it be possible to generalize a message or component?
    The more general message or component should contain origin and dir, with a 'decoding' function made available...
    We don't currently have a way to provide decoding information on the package end but we really really should.
    Schema neeeed pure functions.

Minor tweak necessary to camera_factory : Changed the translation component's value.
    The lookat_target component is very powerful combined with translation!
    It's not perfectly flexible, but it does handle many situations with great ease.