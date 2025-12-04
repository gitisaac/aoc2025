# 🎄 Advent of Code 2025 🎄

My solutions for [Advent of Code 2025](https://adventofcode.com/2025) implemented in Rust.

## Project Structure

```
aoc2025/
├── Cargo.toml              # Workspace configuration with common dependencies
├── input/                  # All input files
│   ├── day01.txt          # Actual puzzle input
│   ├── day01_sample.txt   # Sample input from problem description
│   └── ...
├── src/
│   ├── bin/               # Individual day solutions
│   │   ├── day01.rs
│   │   ├── day02.rs
│   │   └── ...
│   ├── lib.rs            # Shared utilities and macros
│   └── utils/            # Helper modules
│       ├── mod.rs
│       ├── grid.rs       # 2D grid utilities, Point, Direction
│       ├── math.rs       # Mathematical functions (GCD, LCM, primes, etc.)
│       └── parsing.rs    # Input parsing utilities
└── README.md
```

## Usage

### Running Solutions

```bash
# Run a specific day
cargo run --bin day01

# Run with release optimizations
cargo run --release --bin day01
```

### Adding a New Day

1. Copy the template:
   ```bash
   cp src/bin/day01.rs src/bin/day{XX}.rs
   ```

2. Add input files:
   ```bash
   touch input/day{XX}.txt input/day{XX}_sample.txt
   ```

3. Add the binary to `Cargo.toml`:
   ```toml
   [[bin]]
   name = "day{XX}"
   path = "src/bin/day{XX}.rs"
   ```

4. Implement your solution in the new file!

### Utilities Available

- **Input handling**: `read_input()`, `read_sample()`, `parse_lines()`
- **Grid utilities**: `Point`, `Direction`, `Grid<T>` with neighbor functions
- **Math functions**: `gcd()`, `lcm()`, `mod_pow()`, `is_prime()`, etc.
- **Parsing helpers**: `extract_integers()`, `parse_coord_pair()`, etc.
- **Timing**: `time_it!` macro for benchmarking
- **Pretty output**: Colored terminal output with the `aoc_main!` macro

## Dependencies

- `regex` - Pattern matching
- `itertools` - Iterator utilities
- `nom` - Parser combinators
- `rayon` - Parallel processing
- `anyhow` - Error handling
- `colored` - Terminal colors
- `clap` - Command-line parsing

## Tips

- Use `cargo run --bin day01` to run individual days
- Sample inputs are tested first if available
- The `aoc_main!` macro provides nice formatting and timing
- Shared utilities help avoid code duplication
- All common AoC patterns are pre-implemented in utils modules

Happy coding! 🦀✨