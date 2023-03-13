#![cfg_attr(not(feature = "std"), no_std)]

#[ink::contract]
mod flipper_2 {
    use flipper_interface::flipper_interface::FlipperInterfaceRef;

    /// Defines the storage of your contract.
    /// Add new fields to the below struct in order
    /// to add new static storage fields to your contract.
    #[ink(storage)]
    pub struct Flipper2 {
        /// Stores a single `bool` value on the storage.
        value: bool
    }

    impl Flipper2 {
        /// Constructor that initializes the `bool` value to the given `init_value`.
        #[ink(constructor)]
        pub fn new(init_value: bool) -> Self {
            Self { 
                value: init_value,
            }
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
        pub fn flip(&mut self) {
            self.value = !self.value;
        }

        /// Simply returns the current value of our `bool`.
        #[ink(message)]
        pub fn get(&self) -> bool {
            self.value
        }

        #[ink(message)]
        pub fn other_contract_flip(&mut self, flipper:AccountId) {
            let mut interface: FlipperInterfaceRef = ink::env::call::FromAccountId::from_account_id(flipper);
            interface.flip();
        }

        #[ink(message)]
        pub fn other_contract_get(&self, flipper:AccountId) -> bool {
            let interface: FlipperInterfaceRef = ink::env::call::FromAccountId::from_account_id(flipper);
            interface.get()
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
            let flipper_2 = Flipper2::default();
            assert_eq!(flipper_2.get(), false);
        }

        /// We test a simple use case of our contract.
        #[ink::test]
        fn it_works() {
            let mut flipper_2 = Flipper2::new(false);
            assert_eq!(flipper_2.get(), false);
            flipper_2.flip();
            assert_eq!(flipper_2.get(), true);
        }
    }
}
