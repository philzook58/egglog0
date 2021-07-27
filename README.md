# egglog

Using the egg library with a file format and semantics similar to datalog.

Explanatory blog post: 

## Try It Online!!!

<http://www.philipzucker.com/egglog/>

## Building

To run on a file locally:
`cargo run --release tests/examples.pl`

To build the wasm library:
`wasm-pack build --target web`

Note: I started modifying egg a bit. I exposed the Subst datatypes field.