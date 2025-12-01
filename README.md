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

[![Run Day 1](https://img.shields.io/badge/▶️_Run_Day_1-Ruby_on_Rails-CC0000?style=for-the-badge&logo=rubyonrails)](./day_one)

## 🌟 Challenge Progress

- **Goal**: Complete all 12 days using 12 different languages
- **Status**: 1/12 days complete

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
cd day_one
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

**Input**: Place your puzzle input in `lib/inputs/rotations.txt`

---

*Saving Christmas, one language at a time* 🎅✨