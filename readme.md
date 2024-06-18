Brainstorming time.
I think the way to handle the terminal output for showing the correct letters and correct places is the term crate, for sure.
In order to do that properly, I think I need a function that can mask certain letters of the output, if that makes sense?
The comparison happens outside the function, I think I just need to feed the guess, a mask, and a colour.
The mask is probably an array of 0's and 1's (maybe even just a byte/word) that tells us whether we colour that character or not.

Initial thought was to call the same function twice, but that means printing the guess to the terminal twice in different colours.
Maybe instead of a colour, we feed it two masks? That way we can output a single response with the correct feedback.
While not letting perfect be the enemy of the good, I want to try and avoid simply writing if/else catches for each letter.
If that's what I have to do to start off, however, then that's what I have to do.