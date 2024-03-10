# mb2-embassy-display-scan
Bart Massey 2024

This crate demos display scanout using Embassy on the
MicroBit v2.

The scanout happens at increasing frame rate, starting at 1
fps. See the source for details. As scanout gets faster, the
illusion of a solidly-lit display increases.

Move your head or jiggle the board as the frame rate gets
higher to see some effects.

Note that a "real" scanout should happen five times faster,
as a whole row can be illuminated at once. On the other
hand, a "real" scanout will allow multiple perceived
brightness levels: with 10 brightness levels, the scanout
will take 10 times as long.
