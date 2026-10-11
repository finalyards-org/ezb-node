# Dev notes

## Two or one crate?

Reasons for keep them together:

- If we use same features (which we currently do), keeping in the same crate reduces duplication.


Reasons for keeping them (= `config` with and without `feature = "gen"`) apart:

- They are functionally, and by their runtime environment, totally different

Both approaches will work.

