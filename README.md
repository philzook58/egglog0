# egglog

Using the [egg](https://egraphs-good.github.io/) library with a file format and semantics similar to datalog.

Explanatory blog posts: 
- <https://www.philipzucker.com/egglog-checkpoint/>
- <https://www.philipzucker.com/egglog2-monic/>
## Try It Online!!!

<http://www.philipzucker.com/egglog/>

## Building

To run on a file locally:
`cargo run --release tests/examples.pl`

To build the wasm library:
`wasm-pack build --target web`

Note: I started modifying egg a bit. I exposed the Subst datatypes field.