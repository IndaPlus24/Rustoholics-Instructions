# DD1337 Week 2
Author: Viola Söderlund   
Modified by: Isak Larsson
## Getting started

### Install Rust

1) Install [Rustup](https://rustup.rs/).
2) _If you're running Windows_, install [Visual Studio C++ Build Tools](https://visualstudio.microsoft.com/visual-cpp-build-tools/).
3)  _If you're developing in Visual Studio Code_, install the *superior* [Rust extension](https://marketplace.visualstudio.com/items?itemName=rust-lang.rust-analyzer).

### Prepare for your assigment

Your first assignment is turned in by uploading it to a repository named `<KTH_ID>-<task name>` (ex. `bwidman-rustIntro`) under the `IndaPlus23` organisation. Be careful to get the spelling right.

The grade to an assigment is left in the form of an issue with "Pass", "Komplettering", or "Fail" in the title. In case of "Komplettering", read the instructions on what to adjust down in the issue description. Leave a comment on the issue upon reupload of the assignment. "Pass" and "Fail" are self explanatory. 

1) Create a repository named `<KTH_ID>-<task name>`. (ex. `bwidman-rustIntro`)
2) Clone your newly created repository.
`git clone git@github.com:IndaPlus23/<KTH_ID>-<task name>.git`
3) Create one Rust crate (term for application or library) per subassignment. 

#### How to create a Rust application (_binary_) crate

1) Navigate in your terminal or command prompt to `<KTH_ID>-<task name>`.
2) Initialise your Rust crate.
`cargo init <crate_name>`
3) Navigate into your newly created root directory.
4) Build and run your application.
`cargo run`

Your clean build should look like:
```
appropriate_name
|- src
  |- main.rs
|- target
|- Cargo.lock
|- Cargo.toml
```

Write your source code in `src`, where the `main` function is located in `src/main.rs`. To make it easier, begin by copying the contents of `./kattis_template/src/main.rs` into your `main.rs` file.

## Assignment

### Kattis problems

This week, you're going to learn the basics of Rust by solving easier [Kattis](https://kth.kattis.com) problems. For each problem, create one Rust crate in your repo. Include a screenshot of your Kattis submission to prove solution. See `./minimal_scalar_product` for a Kattis solution example.

Solve at least two of the following problems:
- [Summera tal](https://kth.kattis.com/courses/DD2016/plusplus23/assignments/nx4ezb/problems/kth.javap.sumsort)
- [Avstånd till kanten](https://kth.kattis.com/courses/DD2016/plusplus23/assignments/nx4ezb/problems/kth.javap.kant)
- [Cyber-Clara och anmälningslistorna](https://kth.kattis.com/courses/DD2016/plusplus23/assignments/nx4ezb/problems/kth.grupdat.anmalningslistorna)

_(optional fun)_:
- [Game Rank](https://open.kattis.com/problems/gamerank)
- [Quantum](https://open.kattis.com/problems/quantum)

_(optional challenge)_:
- _Cyber-Clara och anmälningslistorna_ is a special problem. The Rust [statistics board](https://kth.kattis.com/problems/kth.grupdat.anmalningslistorna/statistics) is littered with +- and ++-students. **Take them down!**
- A example solution to the Kattis problem [Minimal Scalar Product](https://open.kattis.com/problems/minimumscalar) can be found in `./minimal_scalar_product`. This solution runs at 0.06s. See the [statistics](https://open.kattis.com/problems/minimumscalar/statistics) for the Rust language. As you can see, it's possible to solve this problem in much less time. Write your own solution, which may be based on the example solution, and which runs quicker than 0.06s.

### Questions

Be prepared to answer the following questions during the next övning.

#### Method chaining

Observe the following code:

```rust
let input = io::stdin();

let mut lines = input
    .lock()
    .lines()
    .map(|_line| _line.ok().unwrap().to_string());

// for every line, assuming input strings with the characters '0' and '1' seperated by whitelines

let binary_line = lines
    .next().unwrap()
    .split(" ")
    .map(|_title| {
        _title
            .chars()
            .map(|_character| {
                match _character {
                    '0' => false,
                    _ => true
                }
            })
            .collect::<Vec<bool>>()
    })
    .collect::<Vec<Vec<bool>>>();
```

Know the answer of the following question:
- What is the value of `binary_line`?

#### Iterables

Observe the following code:

```rust
use std::collections::{ HashMap, HashSet };

/*...*/

let limit: usize = 10;

let mut index_store: HashMap<usize, usize> = HashMap::with_capacity(limit + 1);
let mut value_store: Vec<HashSet<usize>> = Vec::with_capacity(limit + 1);
        
for value in 1..(limit + 1) {
    index_store.insert(value, value - 1);

    let mut sequence: HashSet<usize> = HashSet::new();
    sequence.insert(value);

    value_store.push(sequence);
}
```

Know the answer of the following questions:
- What is the value of `index_store`?
- What is the value of `value_store`?
