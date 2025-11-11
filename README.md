# 2017

I discovered Advent of Code in 2017 while in the Philippines with my family.
I didn't finish AoC 2017 back then, so now ~~(7 years later)~~ (8 years later) I'm giving it another attempt (using Rust).

# Daily Themes and Stars

The symbol `#` indicates that I did this problem in 2017.

1. `##`
2. `##`
3. `##`
4. `##`
5. `##`
6. `##`
7. `##`
8. `##`
9. `**` Stateful parsers
10. `**` Hashing
11. `#*` Hexagonal grids
12. `##`
13. `**` Sentinel values (`0 != None`), refactoring gone wrong, eliminate candidate solutions as early as possible
14. `**` More hashing, software engineering, Hamming weight (aka `popcount`), searches (I used BFS)
15. `##`
16. `**` Periods, shortcuts, circular arrays
17. `**` More periods, fast insertions, don't compute something you don't care about
18. `#*` Assembly language, channels, concurrency, reading comprehension (small details in spec, unexpected possibilities in input)
19. `**` Complex numbers, pathfinding
20. `#*` Vectors, particle simulations, order-of-operations
21. `**` Matrices, copying
22. `#*` Vectors, finite-state machines
23. `**` Assembly language, reverse engineering, off-by-one errors
24. `**` Graph paths
25. `#*`

# Lessons Learned

* *Specializing* is the opposite of *generalizing*.
* I really like these puzzles like days 10 and 14 that build on each other.
I abstracted my Knot Hash code into a small library with its own tests.
* You can do "table-driven testing" in Rust, but I'm not sure that I love it.
* Values passed to `unwrap_or` are eagerly evaluated. This means that if there
is a side-effect from whatever you have in `unwrap_or`, you will have that
effect.
* Rust's strong static typing really got in the way in day 21 with Nalgebra.
I had to copy a lot of data that I think should have been OK to reference.

# Resources
* https://corrode.dev/blog/defensive-programming/

# Misc

Download puzzle inputs:

```
(1..25) | % {
    $o = "day{0:d2}.txt" -f $_;
    $u = "https://adventofcode.com/2017/day/$($_)/input";
    curl $u --cookie "session=[your-session-key]" -o $o
}
```
