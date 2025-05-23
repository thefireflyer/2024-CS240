# Code library 

| | |
|-|-|
| Author | Aidan Beil |
| Date | 14/2/2024 |
| Class | CS240 2963 |
| Professor | Darrell Criss |

[![Rust](https://github.com/thefireflyer/cs-240-library/actions/workflows/rust.yml/badge.svg)](https://github.com/thefireflyer/cs-240-library/actions/workflows/rust.yml)

## Algorithms

| Topic                   | Algorithm            | $\Theta(f(n))$ | $\mathcal{O}(f(n))$ |
| ----------------------- | -------------------- | -------------- | --------- |
| Searching               | [Linear search]()    | | $\mathcal{O}(n)$
| Searching               | [Binary Search]()    | | $\mathcal{O}(\log n)$
| Sorting                 | [Insertion sort]()   | | $\mathcal{O}(n^2)$
| Sorting                 | [Selection sort]()   | | $\mathcal{O}(n^2)$
| Sorting                 | [Mergesort]()        | | $\mathcal{O}(n\log n)$
| Sorting                 | [Quicksort]()        | | $\mathcal{O}(n\log n)$
| Sorting                 | [Heapsort]()         | | $\mathcal{O}(n\log n)$
| Hashing                 | [Radix conversion]() | | $\mathcal{O}(n)$
| Edit distance           | [Levenshtein]()      |
| Graph search            | [BFS]()              |
| Graph search            | [DFS]()              |
| Graph search (weighted) | [Dijkstra's]()       |
| Graph search (weighted) | [A\*]()              |
| Minimum spanning graph  | [Prim's]()           |
| Black box optimization | [SGD]() |
| Black box optimization | [Simulated Annealing]() |

<!-- ## Data structures

### Sequences

|| Seek | Read | Write | Push | Pop | Insert | Delete |
|-|-|-|-|-|-|-|-|
| [Array]()| $\mathcal{O}(1)$ | $\mathcal{O}(1)$ | $\mathcal{O}(1)$ | $\mathcal{O}(1)^\ast$ | $\mathcal{O}(1)$ | $\mathcal{O}(n)^\ast$ | $\mathcal{O}(n)$
| [Singly linked list]() | $\mathcal{O}(n)$ | $\mathcal{O}(1)$ | $\mathcal{O}(1)$ | $\mathcal{O}(1)$ | $\mathcal{O}(1)$ | $\mathcal{O}(1)$ | $\mathcal{O}(1)$
| [Doubly linked list]() | $\mathcal{O}(n/2)$ | $\mathcal{O}(1)$ | $\mathcal{O}(1)$ | $\mathcal{O}(1)$ | $\mathcal{O}(1)$ | $\mathcal{O}(1)$ | $\mathcal{O}(1)$

(Seek must occur at least once, before any other operation may be performed on the element)

### Stacks

||Peek|Push|Pop
|-|-|-|-|
|[Internal array]() | $\mathcal{O}(1)$ | $\mathcal{O}(1)^\ast$ | $\mathcal{O}(1)$
|[Internal linked list]() | $\mathcal{O}(1)$ | $\mathcal{O}(1)$ | $\mathcal{O}(1)$

### Queues

|| Peek front | Peek back | Push front | Push back | Pop front | Pop back
|-|-|-|-|-|-|-|
|[Internal array]() | $\mathcal{O}(1)$ | $\mathcal{O}(1)$
|[Internal linked list]() | $\mathcal{O}(1)$ | $\mathcal{O}(1)$

### Sets

||___|___|
|-|-|-|
|[Hash set]()
|[AVL set]()

### Maps

||___|___|
|-|-|-|
|[Hash map]()
|[AVL map]()

### Heaps

||___|___|
|-|-|-|
|[Binary heap]()
|[Fibonacci heap]()

### Graphs

||___|___|
|-|-|-|
|[Adjacency list]()
|[Adjacency matrix]() -->


<!-- 
- [Graphs](src/data_structures/graphs/mod.rs)
    - [`WeightedGraph`](src/data_structures/graphs/weighted_graph.rs)
    - [Dijkstra's Algorithm](src/algorithms/graphs/dijkstras.rs) ([pseudocode](pseudocode/dijkstras.md))
    - [Prim's Algorithm](src/algorithms/graphs/prims.rs)([pseudocode](pseudocode/prims.md))
    - [`UndirectedGraph`](src/data_structures/graphs/undirected_graph.rs)
    - [`DirectedGraph`](src/data_structures/graphs/directed_graph.rs)
    - [BFS](src/algorithms/graphs/bfs.rs)
    - [DFS](src/algorithms/graphs/dfs.rs)
- [Binary Heap](src/data_structures/binary_heap.rs)
- [Hash Set](src/data_structures/sets/hashset.rs)
- Hash Tables
    - [Hashing algorithm](combined/Hasher.cs)
    - Using open addressing - [`OpenHashMap`](combined/Tables/OpenHashTable.cs)
    - Using closed addressing - [`ClosedHashMap`](combined/Tables/ClosedHashTable.cs)
    - [Unit testing](combined/Tables/TestTable.cs)
- Maps
    - [Interface](combined/Maps/IMap.cs)
    - BST Maps
        - Unbalanced - [`BST`](combined/Maps/BSTs/BST.cs)
    - [Unit testing](combined/Maps/TestMaps.cs)
- Stacks
    - [Interface](combined/Stacks/IStack.cs)
    - Using linked lists - [`LinkedStack`](combined/Stacks/LinkedStack.cs)
    - Using arrays - [`ArrayStack`](combined/Stacks/ArrayStack.cs)
    - [Unit testing](combined/Stacks/TestStacks.cs)
- Queues
    - [Interface](combined/Queues/IQueue.cs)
    - Using linked lists - [`LinkedQueue`](combined/Queues/LinkedQueue.cs)
    - Using arrays - [`ArrayQueue`](combined/Queues/ArrayQueue.cs)
    - [Unit testing](combined/Queues/TestQueues.cs)
- Linked Lists
    - Double linked list
        - [`LinkedList`](combined/LinkedList/LinkedList.cs)
        - [Unit testing](combined/LinkedList/Test.cs)
    - Single linked list
        - [`LinkedList`](src/data_structures/linked_list/single_linked_list/solution.rs)
        - [Unit testing](src/data_structures/linked_list/single_linked_list/tests.rs)
- Search algorithms
    - [Binary search](src/algorithms/search/binary_search.rs)
    - [Linear search](src/algorithms/search/linear_search.rs)
- Sorting algorithms
    - [Insertion sort](src/algorithms/sort/insertion_sort/solution.rs)
        - [Unit testing](src/algorithms/sort/insertion_sort/tests.rs)
    - [Selection sort](src/algorithms/sort/selection_sort/solution.rs)
        - [Unit testing](src/algorithms/sort/selection_sort/tests.rs)
    - [Merge sort](src/algorithms/sort/merge_sort.rs)
    - [Quick sort](src/algorithms/sort/quick_sort.rs) -->

<!-- ```
cargo test -- --format terse
```

> ```
> running 43 tests
> ...........................................
> test result: ok. 43 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 1.29s
> 
>    Doc-tests cs-240-library
> 
> running 0 tests
> 
> test result: ok. 0 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.00s
> ``` -->

<!-- --- -->
