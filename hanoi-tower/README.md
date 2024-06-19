# Hanoi's Tower

| | |
|-|-|
| Author | Aidan Beil |
| Date | 14/2/2024 |
| Class | CS240 2963 |
| Professor | Darrell Criss |

## Organization

- [solver](hanoi-tower-solver/src/lib.rs)
- gui
    - [main](hanoi-tower-gui/src/main.rs)
    - [components](hanoi-tower-gui/src/components.rs)
    - [events](hanoi-tower-gui/src/events.rs)
    - [resources](hanoi-tower-gui/src/resources.rs)
    - [system](hanoi-tower-gui/src/systems.rs) (most of the interesting stuff)

## Usage

```
cd hanoi-tower-solver
cargo test
```

> ```
> running 3 tests
> test tests::test_hanoi_simple_iter ... ok
> test tests::test_hanoi_simple_rec ... ok
> test tests::test_hanoi_rec ... ok
> 
> test result: ok. 3 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 11.64s
> 
>    Doc-tests hanoi-tower-solver
> 
> running 0 tests
> 
> test result: ok. 0 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.00s
> ```

```
cd hanoi-tower-gui
cargo run
```

https://github.com/thefireflyer/midterm/assets/64284477/7247472c-ae22-431d-a544-1f43caeac9f9



## Approach

I knew beforehand that I wanted to separate the solver and display system, so I set up two skeleton projects to start with.

I then started with a simple 3 peg recursive version and then moved onto the iterative version. The Tower of Hanoi Wikipedia page was helpful as a reference. After I got both iterative and recursive solutions for the 3 peg case, I started working on a general case solution. Here, geeksforgeeks was somewhat helpful as it's 4 peg function helped me decide whether to spread disks out before or after creating an auxiliary stack.

All three algorithms needed a way to reference sections of a mutable array, but I wasn't able to write it in a rust safe way. Instead, I opted to remove the bottom elements of the array and store them in a separate variable. This is less efficient (O(n) versus O(1)), but I had limited time and wasn't making progress with references.

Once I got unit tests passing I started working on the gui. 3d was not a good move. For context, I've used engines like Godot and Unity before and remember feeling pretty confident in them, but I had never used Bevy before. I think some YouTube channel recommended it years ago and when I checked their examples page, everything seemed pretty straight forward. Nope. I mean to be fair, it would've been straight forward if I knew what I was doing, but rushing to learn Bevy's ECS system for the first time as well as UI, animation, event handling and camera systems was not the smoothest experience. Anyway, somehow I got it to mostly work. Bevy does actually have pretty good documentation and resources for the most part, which were incredibly helpful.

There was a weird problem I ran into where I had written all of my animation code as an actual animation (using built-in systems), but I couldn't get the player to find the disks. I honestly still don't know why, but I ended up switching to a custom frame by frame system that just uses lerps for everything.

## Review

Results wise and considering the amount of time I had to complete this, I am actually super happy with I got done. It's far from perfect, the code is a mess, nothing is optimized, there's an annoying animation velocity quirk I couldn't fix, and I'm pretty sure my general case solver isn't actually an optimal solution. However, even just getting functional UI was a relief, let alone animation and event handling.
