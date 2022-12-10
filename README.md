![advent-of-code-2022](./advent-of-code-2022-hero.png)

My attempts at the [Advent of Code 2022](https://adventofcode.com/2022) challenges in [Rust](https://www.rust-lang.org).

This is primarily a learning experience, and the code may not be following best practices.

## Solutions

| Day | Name                      | Code                           | Input Data                         | Time †     | GitHub Action Output |
|-----|---------------------------|--------------------------------|------------------------------------|------------|--------|
| 1   | Calorie Counting          | [src/day_01.rs](src/day_01.rs) | [src/day_01.data](src/day_01.data) | `0m0.002s` | [![Day-01](https://github.com/leifgehrmann/advent-of-code-2022/actions/workflows/Day-01.yml/badge.svg?branch=main)](https://github.com/leifgehrmann/advent-of-code-2022/actions/workflows/Day-01.yml?query=branch%3Amain)
| 2   | Rock Paper Scissors       | [src/day_02.rs](src/day_02.rs) | [src/day_02.data](src/day_02.data) | `0m0.002s` | [![Day-02](https://github.com/leifgehrmann/advent-of-code-2022/actions/workflows/Day-02.yml/badge.svg?branch=main)](https://github.com/leifgehrmann/advent-of-code-2022/actions/workflows/Day-02.yml?query=branch%3Amain)
| 3   | Rucksack Reorganization   | [src/day_03.rs](src/day_03.rs) | [src/day_03.data](src/day_03.data) | `0m0.002s` | [![Day-03](https://github.com/leifgehrmann/advent-of-code-2022/actions/workflows/Day-03.yml/badge.svg?branch=main)](https://github.com/leifgehrmann/advent-of-code-2022/actions/workflows/Day-03.yml?query=branch%3Amain)
| 4   | Camp Cleanup              | [src/day_04.rs](src/day_04.rs) | [src/day_04.data](src/day_04.data) | `0m0.001s` | [![Day-04](https://github.com/leifgehrmann/advent-of-code-2022/actions/workflows/Day-04.yml/badge.svg?branch=main)](https://github.com/leifgehrmann/advent-of-code-2022/actions/workflows/Day-04.yml?query=branch%3Amain)
| 5   | Supply Stacks             | [src/day_05.rs](src/day_05.rs) | [src/day_05.data](src/day_05.data) | `0m0.002s` | [![Day-05](https://github.com/leifgehrmann/advent-of-code-2022/actions/workflows/Day-05.yml/badge.svg?branch=main)](https://github.com/leifgehrmann/advent-of-code-2022/actions/workflows/Day-05.yml?query=branch%3Amain)
| 6   | Tuning Trouble            | [src/day_06.rs](src/day_06.rs) | [src/day_06.data](src/day_06.data) | `0m0.002s` | [![Day-06](https://github.com/leifgehrmann/advent-of-code-2022/actions/workflows/Day-06.yml/badge.svg?branch=main)](https://github.com/leifgehrmann/advent-of-code-2022/actions/workflows/Day-06.yml?query=branch%3Amain)
| 7   | No Space Left On Device   |  |  |  | 
| 8   | Treetop Tree House        | [src/day_08.rs](src/day_08.rs) | [src/day_08.data](src/day_08.data) | `0m0.002s` | [![Day-08](https://github.com/leifgehrmann/advent-of-code-2022/actions/workflows/Day-08.yml/badge.svg?branch=main)](https://github.com/leifgehrmann/advent-of-code-2022/actions/workflows/Day-08.yml?query=branch%3Amain)
| 9   | Rope Bridge               |  |  |  | 
| 10  | Cathode-Ray Tube          | [src/day_10.rs](src/day_10.rs) | [src/day_10.data](src/day_10.data) |         | [![Day-10](https://github.com/leifgehrmann/advent-of-code-2022/actions/workflows/Day-10.yml/badge.svg?branch=main)](https://github.com/leifgehrmann/advent-of-code-2022/actions/workflows/Day-10.yml?query=branch%3Amain)

† _The measured execution time in GitHub Actions_

## How to run

All puzzles are compiled into a single executable, and individual puzzles are selected using the first argument, which is the day number (01-25).

For example, to build and execute the puzzle for Day 2, run:

```
cargo run -- 02
```

To build optimized artifacts, run:

```
cargo build –-release --verbose
./target/release/aoc 02
```

## Attribution

* Hero background image by [Anna Peipina](https://unsplash.com/photos/hLx3QC71kzk) on [Unsplash](https://unsplash.com/).
