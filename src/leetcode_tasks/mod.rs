pub mod descr;
pub mod subm_send;
pub mod subm_show;
pub mod task_actions;
pub mod task_build;
pub mod taskfulldata;
pub mod test_send;

use serde::{Deserialize, Serialize};

#[allow(unused)]
#[derive(Deserialize, Debug)]
pub(crate) struct Rate {
    pub likes: u32,
    pub dislikes: u32,
}

#[allow(unused)]
#[derive(Deserialize, Debug)]
pub(crate) struct Descryption {
    pub name: String,
    pub content: String,
}

#[allow(unused)]
pub enum Category {
    AllTopics,
    Algorithms,
    DataBase,
    JavaScript,
    Shell,
    Concurrency,
}

#[allow(unused)]
pub enum Difficulty {
    Easy,
    Medium,
    Hard,
}

#[allow(unused)]
pub enum Status {
    Todo,
    Solved,
    Attempted,
}

#[derive(Serialize, Debug, Deserialize)]
pub enum Tags {
    Array,
    String,
    HashTable,
    Math,
    DynamicProgramming,
    Sorting,
    Greedy,
    DepthFirstSearch,
    Database,
    BinarySearch,
    BreadthFirstSearch,
    Tree,
    Matrix,
    TwoPointers,
    BinaryTree,
    BitManipulation,
    HeapPriorityQueue,
    Stack,
    Graph,
    PrefixSum,
    Simulation,
    Design,
    Counting,
    Backtracking,
    SlidingWindow,
    UnionFind,
    LinkedList,
    OrderedSet,
    MonotonicStack,
    Enumeration,
    Recursion,
    Trie,
    DivideAndConquer,
    Bitmask,
    BinarySearchTree,
    NumberTheory,
    Queue,
    SegmentTree,
    Memoization,
    Geometry,
    TopologicalSort,
    BinaryIndexedTree,
    HashFunction,
    GameTheory,
    ShortestPath,
    Combinatorics,
    DataStream,
    Interactive,
    StringMatching,
    RollingHash,
    Brainteaser,
    Randomized,
    MonotonicQueue,
    MergeSort,
    Iterator,
    Concurrency,
    DoublyLinkedList,
    ProbabilityStatistics,
    Quickselect,
    BucketSort,
    SuffixArray,
    MinimumSpanningTree,
    CountingSort,
    Shell,
    LineSweep,
    ReservoirSampling,
    EulerianCircuit,
    RadixSort,
    StronglyConnectedComponent,
    RejectionSampling,
    BiconnectedComponent,
}
