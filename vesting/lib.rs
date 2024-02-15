#![cfg_attr(not(feature = "std"), no_std, no_main)]

#[ink::contract]
mod vesting_contract {

    #[ink(storage)]
    pub struct VestingContract {
        releasable_balance: Balance,
        released_balance: Balance,
        duration_time: Timestamp,
        start_time: Timestamp,
        beneficiary: AccountId,
        owner: AccountId,
    }

    /// Error for when the beneficiary is a zero address.
    /// & Error for when the releasable balance is zero.
    #[derive(Debug, PartialEq, Eq, scale::Encode, scale::Decode)]
    #[cfg_attr(feature = "std", derive(scale_info::TypeInfo))]
    pub enum Error {
        InvalidBeneficiary,
        ZeroReleasableBalance,
    }

    /// To emit events when a release is made.
    #[ink(event)]
    pub struct Released {
        value: Balance,
        to: AccountId,
    }

    /// ## This is to set the following during contract deployment:
    /// - beneficiary: the account that will receive the tokens
    /// - duration_time: duration of the vesting period,
    ///   please note that this is in seconds
    /// - start_time: the time (as Unix time) at which point
    ///   vesting starts
    /// - owner: the account that can release the tokens
    /// - releasable_balance: the initial amount of tokens vested
    /// - released_balance: the initial amount of tokens released
    ///
    /// # Note:
    /// The beneficiary cannot be the zero address.
    ///
    impl VestingContract {
        #[ink(constructor, payable)]
        pub fn new(
            beneficiary: AccountId,
            duration_time_in_sec: Timestamp,
        ) -> Result<Self, Error> {
            if beneficiary == AccountId::from([0x0; 32]) {
                return Err(Error::InvalidBeneficiary);
            }
            
            // This is multiplied by 1000 to conform to the
            // Timestamp fomat in ink.
            let duration_time = duration_time_in_sec * 1000;

            let start_time = Self::env().block_timestamp();
            let owner = Self::env().caller();
            let releasable_balance = 0;
            let released_balance = 0;

            Ok(Self {
                duration_time,
                start_time,
                beneficiary,
                owner,
                releasable_balance,
                released_balance,
            })
        }

        /// This returns current block timestamp.
        pub fn time_now(&self) -> Timestamp {
            self.env().block_timestamp()
        }

        /// This returns this contract balance.
        #[ink(message)]
        pub fn this_contract_balance(&self) -> Balance {
            self.env().balance()
        }

        /// This returns the beneficiary wallet addr.
        #[ink(message)]
        pub fn beneficiary(&self) -> AccountId {
            self.beneficiary
        }

        /// This returns the time at which point
        /// vesting starts.
        #[ink(message)]
        pub fn start_time(&self) -> Timestamp {
            self.start_time
        }

        /// This returns the duration of the vesting
        /// period, in seconds.
        #[ink(message)]
        pub fn duration_time(&self) -> Timestamp {
            self.duration_time
        }

        /// This returns the time at which point
        /// vesting ends.
        #[ink(message)]
        pub fn end_time(&self) -> Timestamp {
            self.start_time() + self.duration_time()
        }

        /// This returns the amount of time remaining
        /// until the end of the vesting period.
        #[ink(message)]
        pub fn time_remaining(&self) -> Timestamp {
            if self.time_now() < self.end_time() {
                return self.end_time() - self.time_now();
            } else {
                return 0;
            }
        }

        /// This returns the amount of native token that
        /// has already vested.
        #[ink(message)]
        pub fn released_balance(&self) -> Balance {
            self.released_balance
        }

        /// This returns the amount of native token that
        /// is currently available for release.
        #[ink(message)]
        pub fn releasable_balance(&self) -> Balance {
            return self.vested_amount() as Balance - self.released_balance();
        }

        /// This calculates the amount that has already vested
        /// but hasn't been released from the contract yet.
        #[ink(message)]
        pub fn vested_amount(&self) -> Balance {
            return self.vesting_schedule(self.this_contract_balance(), self.time_now());
        }

        /// This sends the releasable balance to the beneficiary.
        /// wallet address; no matter who triggers the release.
        #[ink(message)]
        pub fn release(&mut self) -> Result<(), Error> {
            let releasable = self.releasable_balance();
            if releasable == 0 {
                return Err(Error::ZeroReleasableBalance);
            }

            self.released_balance += releasable;
            self.env()
                .transfer(self.beneficiary, releasable)
                .expect("Transfer failed during release");

            self.env().emit_event(Released {
                value: releasable,
                to: self.beneficiary,
            });

            Ok(())
        }

        /// This calculates the amount of tokens that have vested up
        /// to the given current_time.
        ///
        /// The vesting schedule is linear, meaning tokens are
        /// released evenly over the vesting duration.
        ///
        /// # Parameters:
        /// - total_allocation: The total number of tokens
        ///   allocated for vesting.
        /// - current_time: The current timestamp for which
        ///   we want to check the vested amount.
        ///
        /// # Returns:
        /// - `0` if the current_time is before the vesting start time.
        /// - total_allocation if the current_time is after the vesting
        ///   end time or at least equal to it.
        /// - A prorated amount based on how much time has passed since
        ///   the start of the vesting period if the `current_time` is
        ///   during the vesting period.
        ///
        /// # Example:
        /// If the vesting duration is 200 seconds and 100 seconds have
        /// passed since the start time, then 50% of the total_allocation
        /// would have vested.
        ///
        pub fn vesting_schedule(
            &self,
            total_allocation: Balance,
            current_time: Timestamp,
        ) -> Balance {
            if current_time < self.start_time() {
                return 0;
            } else if current_time >= self.end_time() {
                return total_allocation;
            } else {
                return (total_allocation
                    * (current_time - self.start_time()) as Balance)
                    / self.duration_time() as Balance;
            }
        }
    }

    #[cfg(test)]
    mod tests {
        use super::*;

        /// Checking that the default constructor does its job.
        #[ink::test]
        fn new_creates_contract_with_correct_values() {
            let contract = VestingContract::new(AccountId::from([0x01; 32]), 200).unwrap();

            assert_eq!(contract.beneficiary(), AccountId::from([0x01; 32]));
            assert_eq!(contract.duration_time(), 200 * 1000);
            assert_eq!(contract.released_balance(), 0);
            assert_eq!(contract.releasable_balance(), 0);
        }

        /// There should be some time remaining before the vesting period ends.
        #[ink::test]
        fn time_remaining_works() {
            let contract = VestingContract::new(AccountId::from([0x01; 32]), 200).unwrap();
            assert!(contract.time_remaining() > 0);
        }

        /// # Checking that tokens cannot be released before
        /// the vesting period:
        ///     - Trying to release tokens before the vesting period
        ///       has ended, it will return an error.
        ///     - The released_balance should remain 0 since no tokens
        ///       were released.
        #[ink::test]
        fn release_before_vesting_period_fails() {
            let mut contract = VestingContract::new(AccountId::from([0x01; 32]), 200).unwrap();

            assert_eq!(contract.release(), Err(Error::ZeroReleasableBalance));
            assert_eq!(contract.released_balance(), 0);
        }

        /// # Checking if tokens can be released after the vesting period:
        ///     - Setting the duration_time to 0 to simulate the end of
        ///       the vesting period.
        ///     - And then simulate a deposit into the contract.
        ///     - After releasing, the released_balance should match the
        ///       amount we simulated as a deposit.
        #[ink::test]
        fn release_after_vesting_period_works() {
            let mut contract = VestingContract::new(AccountId::from([0x01; 32]), 0).unwrap();
            contract.releasable_balance += 1000000;

            assert_eq!(contract.release(), Ok(()));
            assert_eq!(contract.released_balance(), 1000000);
        }

        /// # Checking the vesting_schedule function for a specific behavior:
        ///     - Given a total allocation and a current time halfway through
        ///       the vesting period, the vested amount should be half of
        ///       the total allocation.
        #[ink::test]
        fn vesting_schedule_works() {
            let contract = VestingContract::new(AccountId::from([0x01; 32]), 200).unwrap();

            assert_eq!(
                contract.vesting_schedule(1000, contract.start_time() + 100 * 1000),
                500
            );
        }
    }
}
