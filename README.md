# Advent of Code 2023

![Tests](https://github.com/stephenkelzer/aoc2023/actions/workflows/build_and_test.yml/badge.svg)

This is my codebase for the 2023 [Advent of Code](https://adventofcode.com/). This is a simple Rust workflows project separating each day of the event into it's own workspace. There are no binaries in this project, so running `cargo run` will not work. Instead, each day's workspace is a library and has unit tests to check their answer. Once I have successfully completed the puzzle, I update the corresponding unit test's assertion to be correct and then commit the code.

#### Prerequisites

- Install [Rust](https://www.rust-lang.org/tools/install)

## How to

Run all tests:

```bash
cargo test
```

Run a specific days tests:

```bash
cargo test -p day_9
```

## My Personal Leaderboard

<table>
    <thead>
        <tr>
            <th></th>
            <th colspan="3">Part 1</th>
            <th colspan="3">Part 2</th>
        </tr>
        <tr>
            <th width="500px">Day</th>
            <th width="500px">Time</th>
            <th width="500px">Rank</th>
            <th width="500px">Score</th>
            <th width="500px">Time</th>
            <th width="500px">Rank</th>
            <th width="500px">Score</th>
        </tr>
    </thead>
    <tbody>
        <tr>
            <td><a href="https://adventofcode.com/2023/day/8">Day 8</a></td>
            <td>00:34:05</td>
            <td>8624</td>
            <td>0</td>
            <td>02:17:21</td>
            <td>9586</td>
            <td>0</td>
        </tr>
        <tr>
            <td><a href="https://adventofcode.com/2023/day/7">Day 7</a></td>
            <td>01:28:16</td>
            <td>9482</td>
            <td>0</td>
            <td>02:58:28</td>
            <td>11465</td>
            <td>0</td>
        </tr>
        <tr>
            <td><a href="https://adventofcode.com/2023/day/6">Day 6</a></td>
            <td>00:20:05</td>
            <td>6751</td>
            <td>0</td>
            <td>00:24:00</td>
            <td>5722</td>
            <td>0</td>
        </tr>
        <tr>
            <td><a href="https://adventofcode.com/2023/day/5">Day 5</a></td>
            <td>01:57:42</td>
            <td>13035</td>
            <td>0</td>
            <td>02:39:13</td>
            <td>5829</td>
            <td>0</td>
        </tr>
        <tr>
            <td><a href="https://adventofcode.com/2023/day/4">Day 4</a></td>
            <td>01:22:57</td>
            <td>16561</td>
            <td>0</td>
            <td>03:12:24</td>
            <td>20064</td>
            <td>0</td>
        </tr>
        <tr>
            <td><a href="https://adventofcode.com/2023/day/3">Day 3</a></td>
            <td>01:19:22</td>
            <td>8802</td>
            <td>0</td>
            <td>02:20:50</td>
            <td>10051</td>
            <td>0</td>
        </tr>
        <tr>
            <td><a href="https://adventofcode.com/2023/day/2">Day 2</a></td>
            <td>01:08:51</td>
            <td>12355</td>
            <td>0</td>
            <td>01:52:09</td>
            <td>14599</td>
            <td>0</td>
        </tr>
        <tr>
            <td><a href="https://adventofcode.com/2023/day/1">Day 1</a></td>
            <td>23:10:51</td>
            <td>145240</td>
            <td>0</td>
            <td>23:37:34</td>
            <td>104505</td>
            <td>0</td>
        </tr>
    </tbody>
</table>
