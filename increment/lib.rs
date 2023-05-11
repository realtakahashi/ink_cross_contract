#![cfg_attr(not(feature = "std"), no_std)]

pub use self::increment::{Increment, IncrementRef};

#[ink::contract]
mod increment {

    /// Defines the storage of your contract.
    /// Add new fields to the below struct in order
    /// to add new static storage fields to your contract.
    #[ink(storage)]
    pub struct Increment {
        /// Stores a single `bool` value on the storage.
        uint_value: u128,
    }

    impl Increment {
        /// Constructor that initializes the `bool` value to the given `init_value`.
        #[ink(constructor)]
        pub fn new(init_value: u128) -> Self {
            Self { uint_value: init_value }
        }

        /// Constructor that initializes the `bool` value to `false`.
        ///
        /// Constructors can delegate to other constructors.
        #[ink(constructor)]
        pub fn default() -> Self {
            Self::new(Default::default())
        }

        /// A message that can be called on instantiated contracts.
        /// This one flips the value of the stored `bool` from `true`
        /// to `false` and vice versa.
        #[ink(message)]
        pub fn inc(&mut self, inc_value: u128) {
            self.uint_value += inc_value;
        }

        /// Simply returns the current value of our `bool`.
        #[ink(message)]
        pub fn get(&self) -> u128 {
            self.uint_value
        }
    }

    /// Unit tests in Rust are normally defined within such a `#[cfg(test)]`
    /// module and test functions are marked with a `#[test]` attribute.
    /// The below code is technically just normal Rust code.
    #[cfg(test)]
    mod tests {
        /// Imports all the definitions from the outer scope so we can use them here.
        use super::*;

        /// We test if the default constructor does its job.
        #[ink::test]
        fn default_works() {
            let increment = Increment::default();
            assert_eq!(increment.get(), false);
        }

        /// We test a simple use case of our contract.
        #[ink::test]
        fn it_works() {
            let mut increment = Increment::new(false);
            assert_eq!(increment.get(), false);
            increment.flip();
            assert_eq!(increment.get(), true);
        }
    }
}
