# Spell checker

| | |
|-|-|
| Author | Aidan Beil |
| Date | 14/2/2024 |
| Class | CS240 2963 |
| Professor | Darrell Criss |

## Organization

- [Library](spell-checker/src/lib.rs)
- CLI
    - [Main](spell-checker-cli/src/main.rs)
    - [CLI defintions](spell-checker-cli/src/cli.rs)

## Usage

```
cd spell-checker
cargo test
```

> ```
> running 2 tests
> test tests::levenshtein_distance_tests ... ok
> test tests::in_memory_tests ... ok
> 
> test result: ok. 2 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.02s
> 
>    Doc-tests spell-checker
> 
> running 0 tests
> 
> test result: ok. 0 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.00s
> ```

```
cd spell-checker-cli
cargo run
```

> ```
> Usage: spell-checker-cli [OPTIONS] <COMMAND>
> 
> Commands:
>   check          
>   suggest        
>   check-suggest  
>   add            
>   interactive    
>   help           Print this message or the help of the given subcommand(s)
> 
> Options:
>   -p, --path <PATH>  Path to dictionary
>   -h, --help         Print help (see more with '--help')
> ```

```
cd spell-checker-cli
cargo run -- -p test.dic check-suggest test cat catts flufff ptato chese tast
```

> ```
> ~~~ catts
>     cants
>     catt
>     catty
>     watts
>     bates
>     baths
>     batt
>     batty
>     c
>     ca
> 
> ~~~ flufff
>     f
>     fluff
>     fluffy
>     bluff
>     ff
>     fl
>     flu
>     sluff
>     acuff
>     bff
> 
> ~~~ ptato
>     plato
>     potato
>     prato
>     a
>     cato
>     dato
>     erato
>     jato
>     nato
>     o
> 
> ~~~ chese
>     chase
>     cheese
>     chess
>     chest
>     chose
>     e
>     these
>     achebe
>     achene
>     c
> 
> ~~~ tast
>     bast
>     cast
>     east
>     fast
>     hast
>     last
>     mast
>     oast
>     past
>     st
> ```

## Approach

Like with Hanoi's Tower, I started by creating a library component and an application component.

I started by using rust's builtin hash set since I hadn't ported my own to rust yet. Most of the library logic was pretty simple.

I wasn't familiar with Levenshtein distance, so I just went to Wikipedia and read through their documentation. I also found a site called Baeldung that had a pretty good article explaining it as well. After implementing the edit distance function, I kept getting weird looking results, so I checked Wikimedia's rust example implementation (it turned to just be correctly weird).

After getting unit tests to pass on the library, I started working through the CLI. I wanted to use Cursive, an interactive TUI framework, but they didn't have a multi-line text edit component that supported stylized text and I was already running low on time so I went with a simple Clap interface. I also didn't want to have to type in a path every time so I setup a default dictionary search algorithm for Linux machines. I use Arch and I know it works on my machine, but I haven't tested it on other distros. I also haven't had time yet to write a Windows version (I have dual boot so I could in theory test it) and I don't access to a MacOs device so I could write something but I would have no way of knowing if it works. 

## Review

I'm pretty happy with my spell checker. It's not very impressive or interesting, but it works.
