# rust-playground

📚 Learning and exploring the Rust programming language.


## Standalone subprojects

This repository illustrates different concepts, patterns and examples via standalone subprojects. Each subproject is
completely independent of the others and do not depend on the root project. This _standalone subproject constraint_
forces the subprojects to be complete and maximizes the reader's chances of successfully running, understanding, and
re-using the code.

The subprojects include:


### `notes/`

This is not an executable project but instead contains my notes about Rust.

See the README in [`notes/`](notes/).


### `guessing-game/`

Verbatim copy of the [*Programming a Guessing Game* chapter](https://doc.rust-lang.org/stable/book/ch02-00-guessing-game-tutorial.html).

See the README in [`guessing-game/`](guessing-game/).


### `read-file/`

A simple Rust program to read a file.  

See the README in [`read-file/`](read-file/).


### `json/`

A simple Rust program to deserialize and serialize JSON using the popular Rust JSON library called [*Serde*](https://github.com/serde-rs/json).

See the README in [`json/`](json/).


### `iterators/`

An illustration of Rust iterators.

See the README in [`iterators/`](iterators/).


## Wish List

General clean-ups, TODOs and things I wish to implement for this project:

* [ ] SKIP (I've got this covered) Create an error handling subproject
  * E.g. `Result/Ok/Err` and the `?` special thing
* [ ] Create an argument parsing subproject
* [ ] Create an async subproject
* [ ] Create a subproject that shells out to another process
