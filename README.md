# Advent of Code 2023

![Tests](https://github.com/stephenkelzer/aoc2023/actions/workflows/run_tests.yml/badge.svg)

This is my codebase for the 2023 [Advent of Code](https://adventofcode.com/). This is a simple Rust workflows project separating each day of the event into it's own workspace. There are no binaries in this project, so running `cargo run` will not work. Instead, each day's workspace is a library and has unit tests to check their answer. Once I have successfully completed the puzzle, I update the corresponding unit test's assertion to be correct and then commit the code.

#### Prerequisites

- Install [Rust](https://www.rust-lang.org/tools/install)

## How to

Run all tests:

```bash
cargo test --release
```

Run a specific days tests:

```bash
cargo test -p day_09 --release
```

## My Personal Leaderboard

<table>
    <thead>
        <tr>
            <th></th>
            <th></th>
            <th colspan="3">Part 1</th>
            <th colspan="3">Part 2</th>
        </tr>
        <tr>
            <th width="500px">Day</th>
            <th width="500px">Code</th>
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
            <td><a href="https://adventofcode.com/2023/day/15">Day_15</a></td>
            <td><a href="./days/day_15/src/lib.rs">view</a></td>
            <td>00:15:40</td>
            <td>5000</td>
            <td>0</td>
            <td>01:52:14</td>
            <td>7355</td>
            <td>0</td>
        </tr>
        <tr>
            <td><a href="https://adventofcode.com/2023/day/14">Day_14</a></td>
            <td><a href="./days/day_14/src/lib.rs">view</a></td>
            <td>01:30:58</td>
            <td>8128</td>
            <td>0</td>
            <td>02:42:46</td>
            <td>6002</td>
            <td>0</td>
        </tr>
        <tr>
            <td><a href="https://adventofcode.com/2023/day/13">Day_13</a></td>
            <td><a href="./days/day_13/src/lib.rs">view</a></td>
            <td>01:38:32</td>
            <td>6395</td>
            <td>0</td>
            <td>01:56:36</td>
            <td>5023</td>
            <td>0</td>
        </tr>
        <tr>
            <td><a href="https://adventofcode.com/2023/day/12">Day_12</a></td>
            <td><a href="./days/day_12/src/lib.rs">view</a></td>
            <td>01:05:03</td>
            <td>4498</td>
            <td>0</td>
            <td>01:46:56</td>
            <td>2015</td>
            <td>0</td>
        </tr>
        <tr>
            <td><a href="https://adventofcode.com/2023/day/11">Day_11</a></td>
            <td><a href="./days/day_11/src/lib.rs">view</a></td>
            <td>02:06:07</td>
            <td>10388</td>
            <td>0</td>
            <td>02:25:26</td>
            <td>9626</td>
            <td>0</td>
        </tr>
        <tr>
            <td><a href="https://adventofcode.com/2023/day/10">Day_10</a></td>
            <td><a href="./days/day_10/src/lib.rs">view</a></td>
            <td>01:48:55</td>
            <td>7254</td>
            <td>0</td>
            <td>09:28:17</td>
            <td>12430</td>
            <td>0</td>
        </tr>
        <tr>
            <td><a href="https://adventofcode.com/2023/day/9">Day_09</a></td>
            <td><a href="./days/day_09/src/lib.rs">view</a></td>
            <td>00:31:09</td>
            <td>5988</td>
            <td>0</td>
            <td>01:08:10</td>
            <td>8545</td>
            <td>0</td>
        </tr>
        <tr>
            <td><a href="https://adventofcode.com/2023/day/8">Day_08</a></td>
            <td><a href="./days/day_08/src/lib.rs">view</a></td>
            <td>00:34:05</td>
            <td>8624</td>
            <td>0</td>
            <td>02:17:21</td>
            <td>9586</td>
            <td>0</td>
        </tr>
        <tr>
            <td><a href="https://adventofcode.com/2023/day/7">Day_07</a></td>
            <td><a href="./days/day_07/src/lib.rs">view</a></td>
            <td>01:28:16</td>
            <td>9482</td>
            <td>0</td>
            <td>02:58:28</td>
            <td>11465</td>
            <td>0</td>
        </tr>
        <tr>
            <td><a href="https://adventofcode.com/2023/day/6">Day_06</a></td>
            <td><a href="./days/day_06/src/lib.rs">view</a></td>
            <td>00:20:05</td>
            <td>6751</td>
            <td>0</td>
            <td>00:24:00</td>
            <td>5722</td>
            <td>0</td>
        </tr>
        <tr>
            <td><a href="https://adventofcode.com/2023/day/5">Day_05</a></td>
            <td><a href="./days/day_05/src/lib.rs">view</a></td>
            <td>01:57:42</td>
            <td>13035</td>
            <td>0</td>
            <td>02:39:13</td>
            <td>5829</td>
            <td>0</td>
        </tr>
        <tr>
            <td><a href="https://adventofcode.com/2023/day/4">Day_04</a></td>
            <td><a href="./days/day_04/src/lib.rs">view</a></td>
            <td>01:22:57</td>
            <td>16561</td>
            <td>0</td>
            <td>03:12:24</td>
            <td>20064</td>
            <td>0</td>
        </tr>
        <tr>
            <td><a href="https://adventofcode.com/2023/day/3">Day_03</a></td>
            <td><a href="./days/day_03/src/lib.rs">view</a></td>
            <td>01:19:22</td>
            <td>8802</td>
            <td>0</td>
            <td>02:20:50</td>
            <td>10051</td>
            <td>0</td>
        </tr>
        <tr>
            <td><a href="https://adventofcode.com/2023/day/2">Day_02</a></td>
            <td><a href="./days/day_02/src/lib.rs">view</a></td>
            <td>01:08:51</td>
            <td>12355</td>
            <td>0</td>
            <td>01:52:09</td>
            <td>14599</td>
            <td>0</td>
        </tr>
        <tr>
            <td><a href="https://adventofcode.com/2023/day/1">Day_01</a></td>
            <td><a href="./days/day_01/src/lib.rs">view</a></td>
            <td>23:10:51</td>
            <td>145240</td>
            <td>0</td>
            <td>23:37:34</td>
            <td>104505</td>
            <td>0</td>
        </tr>
    </tbody>
</table>
