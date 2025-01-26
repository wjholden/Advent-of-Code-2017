# 2017

I discovered Advent of Code in 2017 while in the Philippines with my family.
I didn't finish AoC 2017 back then, so now (7 years later) I'm giving it another attempt (using Rust).

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
18. `# `
19. `  `
20. `# `
21. `  `
22. `# `
23. `  `
24. `  `
25. `# `

# Lessons Learned

* *Specializing* is the opposite of *generalizing*.
* I really like these puzzles like days 10 and 14 that build on each other.
I abstracted my Knot Hash code into a small library with its own tests.
* You can do "table-driven testing" in Rust, but I'm not sure that I love it.