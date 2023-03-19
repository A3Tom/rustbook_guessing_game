# Guessing Game
_Part 1 of 3 of the [Rust Programming Book](https://doc.rust-lang.org/book/ch02-00-guessing-game-tutorial.html)_

Aight so apart from being an absolute banger of a novel, this book also has 3 programming follow along sections.
This is the first of three, a cli guessing game


## How to run

Hit up 
`cargo run` 
And off ye go ! 

`glhf`

## Hings I took away from this jaunt

### Converting input values to other type

```rust
let mut value_i_want = String::new();

let value_i_want: u32 = input.trim()
    .parse()
    .expect("Haul, geez a number!");
```

That's not a typo - you can literally set a variable like this