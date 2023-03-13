# Summury of "ink_cross_contract"
- This dApp is testing for cross contract call.

# Structure
```
 flipper_1    <------->    flipper_2
```

# What problem solve?
- If you implement it blindly, flipper_1 will have a reference to flipper_2, and flipper_2 will have a reference to flipper_1.
- In the rust world it's called a circular reference and it's a compilation error.
- This program solves this.