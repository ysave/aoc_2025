# Advent of Code 2025 🎄
```
    *    .    *    .  *   .  ❄  .   *   .   *    .  *   .   *
  .   *   .  *   .    *  .   *   .  *   .   *  .   *   .  *  
 *  .   *   .  *   .   *   .  *   .   *  .   *   .  *   .   *
       🌟        .    *    .    *    .    *    .        ✨
  .      *   .    *    .    *    .    *    .    *   .      .
    *         .        *         .        *         .    *
           ❄               *           ❄               *
  .    *      .    *    .    *    .    *    .    *      .    

                A D V E N T   O F   C O D E
                        2 0 2 5

       *      /\      *      /\      *      /\      *
    .   *    /  \   .   *   /  \   .   *   /  \   .   *
   *   .    /____\   *  .  /____\   *  .  /____\   *  .
  .   *       ||    .   *    ||    .   *    ||    .   *
 ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
  ❄  ~  ❄  ~  ❄  ~  ❄  ~  ❄  ~  ❄  ~  ❄  ~  ❄  ~  ❄  ~  ❄
 ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
```

A polyglot journey through Advent of Code 2025 - solving each day's puzzle in a different programming language!

[![Day 1](https://img.shields.io/badge/Day_1-Ruby_on_Rails-CC0000?style=for-the-badge&logo=rubyonrails&logoColor=white)](./day_01)

[![Day 2](https://img.shields.io/badge/Day_2-Java-5382A1?style=for-the-badge&logo=openjdk&logoColor=white)](./day_02)

[![Day 3](https://img.shields.io/badge/Day_3-Go-00ADD8?style=for-the-badge&logo=go&logoColor=white)](./day_03)

[![Day 4](https://img.shields.io/badge/Day_4-Rust-000000?style=for-the-badge&logo=rust&logoColor=white)](./day_04)

[![Day 5](https://img.shields.io/badge/Day_5-Python-3670A0?style=for-the-badge&logo=python&logoColor=white)](./day_04)


## 🌟 Challenge Progress

- **Goal**: Complete all 12 days using 12 different languages
- **Status**: 5/12 days complete

---

## Day 1: Secret Entrance 🔐
**Language**: Ruby on Rails  
**Stars**: ⭐⭐

#### Problem Summary
Navigate a circular safe dial (0-99) following rotation instructions to find secret passwords:
- **Part 1**: Count how many times the dial lands exactly on 0 after completing each rotation
- **Part 2**: Count every time the dial passes through 0, including during rotations

### 🚀 Quick Start
**Prerequisites**: Ruby 3.x, Rails 7.x

**Manual Setup**:
```bash
# In Terminal
cd day_01
bundle install
rails server
rails console
```

**Run**:
```ruby
# In Rails console
RotationChecker.part_one  # => 1064
RotationChecker.part_two  # => 6122
```

**Input**: Place your puzzle input in `day_01/lib/inputs/rotations.txt`

---

## Day 2: Gift Shop 🎁

**Language**: Java
**Stars**: ⭐⭐

You arrive in the bustling North Pole gift shop, only to discover that a playful Elf has filled the product database with invalid IDs. Your mission: identify all invalid IDs in each range.

#### Problem Summary

- **Part 1**: An ID is invalid if it is made of **a sequence of digits repeated exactly twice**.
Examples:
  - `55` → `5` + `5`
  - `6464` → `64` + `64`
  - `123123` → `123` + `123`
- **Part 2**: An ID is invalid if it is made of **any sequence of digits repeated at least twice**.
Examples:
  - `12341234` (1234 ×2)
  - `1212121212` (12 ×5)
  - `1111111` (1 ×7)

### 🚀 Quick Start
**Prerequisites**: Java 17+ (or any modern JDK)

**Manual Setup**:

```bash
# In Terminal
cd day_02
javac GiftShop.java
```

**Run**:

```bash
# In Terminal
java GiftShop
```

**Input**: Place your puzzle input in `day_02/inputs.txt`

---

## Day 3: Lobby 🔋

**Language**: Go
**Stars**: ⭐⭐

You descend into the vast lobby only to find all elevators offline due to an electrical surge. The escalator to the printing department also needs power. Your task: find the optimal battery configurations to generate maximum joltage.

#### Problem Summary

The batteries are arranged in banks (one per line), each battery labeled with a joltage rating from 1-9.

- **Part 1**: Turn on exactly **2 batteries** per bank to maximize joltage. The joltage produced equals the 2-digit number formed by those batteries.
  - Example: `987654321111111` → turn on `9` and `8` → `98` jolts
  - Example: `811111111111119` → turn on `8` and `9` → `89` jolts

- **Part 2**: Turn on exactly **12 batteries** per bank to maximize joltage. The joltage produced equals the 12-digit number formed by those batteries.
  - Example: `987654321111111` → `987654321111` jolts (skip three `1`s)

### 🚀 Quick Start
**Prerequisites**: Go 1.21+

**Manual Setup**:
```bash
# In Terminal
cd day_03
```

**Run**:
```bash
# In Terminal
go run main.go
```

**Input**: Place your puzzle input in `day_03/joltages.txt`

---

## Day 4: Printing Department 📄

**Language**: Rust
**Stars**: ⭐⭐

You arrive at the printing department where large rolls of paper are everywhere. To help the Elves break through a wall to reach the cafeteria, you need to optimize forklift operations by identifying which paper rolls can be accessed.

#### Problem Summary

Paper rolls (`@`) are arranged on a grid. A forklift can only access a roll if it has fewer than 4 neighboring rolls (checking all 8 adjacent positions).

- **Part 1**: Count how many rolls can be accessed by a forklift in the initial configuration.
  - Example: In a 10x10 grid, 13 rolls might be accessible

- **Part 2**: Count the total number of rolls that can be removed through iterative removal.
  - When a roll is removed, other rolls may become accessible
  - Continue removing accessible rolls until no more can be accessed
  - Example: Starting with 13 accessible rolls, you might remove 43 total through multiple passes

### 🚀 Quick Start
**Prerequisites**: Rust 1.70+ (with Cargo)

**Manual Setup**:
```bash
# In Terminal
cd day_04
```

**Run**:
```bash
# In Terminal
cargo run
# or compile and run
rustc main.rs && ./main
```

**Input**: Place your puzzle input in `day_04/rolls.txt`

---


## Day 5: Cafeteria 🍽️

**Language**: Python
**Stars**: ⭐⭐

You break through to the cafeteria, only to find the kitchen in chaos. The new inventory management system has made it impossible to determine which ingredients are fresh and which are spoiled.

#### Problem Summary

The database contains fresh ingredient ID ranges and available ingredient IDs:

- **Part 1**: Count how many available ingredient IDs fall within the fresh ranges.
  - Example: Given ranges `3-5`, `10-14`, `16-20`, `12-18` and IDs `1`, `5`, `8`, `11`, `17`, `32`
  - Fresh IDs: `5`, `11`, `17` → **3 fresh ingredients**

- **Part 2**: Count the total number of ingredient IDs covered by all fresh ranges (ignoring the available IDs list).
  - The ranges can overlap and should be merged
  - Example: Same ranges cover `3`, `4`, `5`, `10-20` → **14 total fresh IDs**

### 🚀 Quick Start
**Prerequisites**: Python 3.x

**Manual Setup**:
```bash
# In Terminal
cd day_05
```

**Run**:
```bash
# In Terminal
python main.py
```

**Input**: Place your puzzle input in `day_05/ingredients.txt`

---

*Saving Christmas, one language at a time* 🎅✨