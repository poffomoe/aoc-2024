# 🎄 Advent of Code 2024

## Information

This repository contains my solutions to the puzzles in Advent of Code 2024.

## How to try out

```
git clone 'https://github.com/poffomoe/aoc-2024'
cd aoc-2024
mkdir input
```

Then place the input files of each day in the `input` directory with the following names: `day_1`, `day_2`, `day_3`, `..`, `day_25`.

After you've done that, you can run solutions of each day using

```
cargo run -p day_X
```

## Notes

> Day 3 was helpful. Working wih regex in Rust was something that I thought would be very simple but it was less simple than I expected. Yes regex is pretty slow and I could have coded a fast pattern matching of some sort using only std but I don't think it is worth the hassle for such a small program.

> I am like a hundred percent sure first part in Day 4 could be solved in a better way but it is what it is, that is the best solution I could come up with.