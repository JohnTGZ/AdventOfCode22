# AdventOfCode22
Taking this christmas to learn Rust :D
Some of the solutions might be rather lengthy and contrived in order to force myself to use some Rust concept that I learned along the way.

# Common library Bugs
1. For the aoc_common FileContents::build, account for newline-terminated input.

# Comments on each day

## Day 1
Not challenging, but this being my first puzzle to solve in Rust, it was kind of painful to wrestle with the borrow checker, hope this improves over time. One thing I learned  is to use `String` in a struct rather than `&str`, as the struct needs to own the `String` variable, rather than referring to a borrowed variable from somewhere else in the memory (that might have expired).

## Day 2
I used a `hashmap` to save the a custom data structure that contains the corresponding win/loss of each shape and their scores. It is then a simple matter of iterating through the instructions and retrieving the hashmap entries.

## Day 3
Trivial to solve with a hashmap to store the count of each character and a `hashset` to determine which characters have been encountered before. I wonder if there is a better way though...

## Day 4
A matter of parsing the input correctly and then using range checks to determine if the 2 number ranges are partially or fully overlapping.

## Day 5
I attempted to use regular expression to parse the starting crate configuration (and failed). I tried to abstract away the logic of moving the crates by using a `Shipyard`, `Instructions` and `Moves` struct. I'm storing the stack of crates in a `vector of vectors` and I'm sure there is a more efficient way of organizing it as the memory locations between vectors in a 2D vector are not contiguous. For example, by using a 1D continuous vector and an array to keep track of the index where each stack starts and ends. 
## Day 6
Day 6 is rather simple. Just use a `hashset` to check for duplicate characters within a packet.

## Day 7
Day 7 was much more challenging. Im pretty sure there is a better way to solve the puzzle without having to construct an actual file directory system you could crawl through. Nevertheless, I did it to practice using `smart pointers` in Rust :)

## Day 8 
Having some experience with robotics costmap helps with thinking about how to represent and store the forest height map. It was a good challenge :) Just had to make sure I read the rules properly about which trees could or could not be seen.

## Day 9 [C++]
Solved both parts in C++ to practice it (I'm getting rather `rusty` :) ). I didn't look too carefully at the way the tail is supposed to follow the head, and came up with overly-complex ways to simulate the motion, when it is actually a pretty simple motion derived from `comparing if the head and tails are on similar rows/columns`.

## Day 10
Very fun problem to solve. I used a `loop` in Rust for the first time, which will iterate through the file input line by line until there is no input left. How do I then simulate the cycles where the instructions are still executing? I simply have a conditional check that skips reading input until the instruciton is complete (And add the value to the X register). 

## Day 11
- I made use of `enums` to present each operation for the monkey (`+`, `-`, `/`, `*`)
- A monkey struct stores all of the necessary data such as the `items VecDeque`, `Operation enum`, and which monkey to throw to.
- Used match guards :) Pretty neat feature to match more generic patterns (such as if a superstring contains a substring)
- We can't borrow more than one mutable references from from a vector (even if it's from 2 different indexes as the Rust compiler tries to guarantee it's safety), we could `split_at_mut()` but I opt to clone instead an individual Monkey `struct` and then reassign it back later.
- The second part 
    - Might only need to care about the last few digits of the number (up to the hundreds)?
    - would probably require me to conver the number to strings and multiply them?
    - https://doc.rust-lang.org/std/primitive.u32.html
# TODO 
- Add uncrustify as a linting tool

