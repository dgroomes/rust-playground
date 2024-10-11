# guessing_game

Verbatim copy of the [*Programming a Guessing Game* chapter](https://doc.rust-lang.org/stable/book/ch02-00-guessing-game-tutorial.html).


## Instructions

Follow these instructions to build and run the guessing game.

1. Build and run:
   * ```shell
     cargo run
     ```
   * It will prompt you to enter a number. You should be able to whittle down the range of possible numbers until you
     guess the winner.
   * Altogether, it should look something like this:
   * ```shell
     $ cargo run
       ... omitted ...
     Guess the number!
     Please input your guess.
     3
     You guessed: 3
     Too small!
     Please input your guess.
     11
     You guessed: 11
     Too big!
     Please input your guess.
     9
     You guessed: 9
     You win! 
     ```
