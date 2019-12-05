
# Advent of Code 2019 #

Two years later, I join again. These are my solutions for the Advent of Code
2019 coding challenges. This year I decided to go with Rust since I have been
nagged on and off by friends to finally get into it and I felt now might be a
good time to delve a little deeper than on a surface level!

## Toolchain ##

Since I hadn't done anything with Rust and I didn't really know of any toolchain
or setup to use, I went with a fresh rustup installation. I'm adding my steps
here as I don't often use Rust so it's easier to document my workflow this way.

    $ rustup toolchain install stable

In this case we're on the stable 2018 release.

From there, initialized a new cargo package for the entire repository as I want
to reuse shared libraries instead of recompiling for each day of the challenge
and having additional overhead on storage.

    $ cargo new adventofcode19

## Setup ##

With the cargo package configured, I opted for a friend's approach from a
previous year by specifying files to build instead of housing everything under a
single main. With the understanding that single days normally share their
inputs, days are split into their separate directories and have A and B
variations. The input data files were normally named `input.txt`.

Each program reads the first argument as the input file location.

To build a specific day or variation, simply run the corresponding command.

    $ cargo run --bin 1a ./res/1/input.txt

Alternatively you can build all by omitting the specific day.

    $ cargo build

## License ##

There's no license. This is just for fun and learning so take from it what you
want as you please. Keep in mind this was done as a learning exercise for Rust.
In that sense the solutions may and quite probably are not optimal. Either way
and once again, Merry Christmas!
