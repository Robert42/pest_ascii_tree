# pest_ascii_tree

## pest_ascii_tree

This is a small helper crate useful for quickly debugging your pest
grammar.
The rules found by parsing the file are formated into an
[`ascii_tree`].

It is useful, you you want to quickly debug your grammar without
having to write specialized code for handling the `Pairs` iterator
returned by your pest parser.

Example, for whan an output might look like.
<pre>
 expr
 ├─ expr
 │  ├─ val "u"
 │  ├─ op "+"
 │  └─ expr
 │     ├─ val "v"
 │     ├─ op "+"
 │     └─ val "w"
 ├─ op "+"
 ├─ expr
 │  ├─ val "x"
 │  ├─ op "+"
 │  └─ val "y"
 ├─ op "+"
 └─ val "z"
</pre>

Please, that the `EOI` rule is skipped.

[`ascii_tree`]: ../ascii_tree/index.html
