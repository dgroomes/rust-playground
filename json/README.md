# json

A simple Rust program to deserialize and serialize JSON using the popular Rust JSON library called [*Serde*](https://github.com/serde-rs/json).


## Instructions

Build and run the example program:
```shell
cargo run
```

Altogether, it will look like the following.

```text
$ cargo run
  ... omitted ...
Parsing a JSON document: { "_id" : "01001", "city" : "AGAWAM", "loc" : [ -72.622739, 42.070206 ], "pop" : 15338, "state" : "MA" }
Deserialized to a struct: city=AGAWAM, state=MA, pop=15338
Serialized to JSON again (but pretty printed and without all of the original fields):
{
  "city": "AGAWAM",
  "state": "MA",
  "pop": 15338
}
```


## Reference

* [*Serde*](https://github.com/serde-rs/json)
    * <https://github.com/serde-rs/json>
    * I believe this is the de-facto JSON serialization/deserialization library used in the Rust community.
