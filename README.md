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

[![Run Day 1](https://img.shields.io/badge/▶️_Run_Day_1-Ruby_on_Rails-CC0000?style=for-the-badge&logo=rubyonrails)](./day_01)

[![Run Day 2](https://img.shields.io/badge/▶️_Run_Day_2-Java-5382A1?style=for-the-badge\&logo=openjdk)](./day_02)

[![Run Day 3](https://img.shields.io/badge/▶️_Run_Day_3-Go-00ADD8?style=for-the-badge&logo=go)](./day_03)

## 🌟 Challenge Progress

- **Goal**: Complete all 12 days using 12 different languages
- **Status**: 3/12 days complete

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
# In terminal
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
# In terminal
cd day_02
javac GiftShop.java
```

**Run**:

```bash
# In terminal
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
# In terminal
cd day_03
```

**Run**:
```bash
go run main.go
```

**Input**: Place your puzzle input in `day_03/joltages.txt`

---

*Saving Christmas, one language at a time* 🎅✨