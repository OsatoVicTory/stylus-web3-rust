// Conditionally compile the program without a main function, unless "export-abi" feature is enabled.
#![cfg_attr(not(feature = "export-abi"), no_main)]

// Set up a global memory allocator using MiniAlloc for efficient memory management in the smart contract.
// commented
// #[global_allocator]
// static ALLOC: mini_alloc::MiniAlloc = mini_alloc::MiniAlloc::INIT;

// Import the alloc crate to enable heap allocations in a no-std environment.
extern crate alloc;


// Import necessary types and functions from the Stylus SDK and Alloy Primitives crates.
// These include U256 for large integers, Address for user addresses, and various
// storage types for managing data on the blockchain.
#[allow(unused_imports)]
use stylus_sdk::{alloy_primitives::U256, prelude::*};
use alloy_primitives::{Address, Uint};
// use stylus_sdk::{block, console};
use stylus_sdk::storage::{StorageString, StorageVec};



// Define the storage structure for the Blog smart contract using the sol_storage! macro.
// This structure contains mappings to store information such as the number of posts,
// post content, user token balances, referrals, and more.

sol_storage! {
    #[entrypoint]
    pub struct Blog {
        mapping(address => uint256) points; // Track the number of points per user
        mapping(address => StorageVec<StorageString>) tasks; // Stores tasks completed by user
        mapping(address => StorageVec<StorageString>) activities; // Stores tasks completed by user
    }
}

// Declare that `Blog` is a contract with the following external methods.
#[public]
impl Blog {

    // Implement the Blog smart contract.
    // This function allows users to purchase tokens by adding the specified amount to their balance.

    pub fn complete_task(&mut self, user_address: Address, amount: Uint<256, 4>, task_id: String) {
        let mut points_accessor = self.points.setter(user_address);
        let current_points = points_accessor.get();
        points_accessor.set(current_points + amount);

        let mut tasks_accessor = self.tasks.setter(user_address);
        let mut new_task_slot = tasks_accessor.grow();
        new_task_slot.set_str(&task_id);
    }

    pub fn transfer_token(&mut self, sender_address: Address, receiver_address: Address, amount: Uint<256, 4>, sender_activity: String, receiver_activity: String) {
        let mut sender_accessor = self.points.setter(sender_address);
        let sender_points = sender_accessor.get();
        sender_accessor.set(sender_points - amount);

        let mut sender_access = self.activities.setter(sender_address);
        let mut sender_slot = sender_access.grow();
        sender_slot.set_str(&sender_activity);

        let mut receiver_accessor = self.points.setter(receiver_address);
        let receiver_points = receiver_accessor.get();
        receiver_accessor.set(receiver_points + amount);

        let mut receiver_access = self.activities.setter(receiver_address);
        let mut receiver_slot = receiver_access.grow();
        receiver_slot.set_str(&receiver_activity);
    }

    pub fn get_points(&self, user_address: Address) -> Uint<256, 4> {
        return self.points.get(user_address);
    }

    pub fn get_tasks(&self, user_address: Address) -> Vec<String> {
        let tasks_accessor = self.tasks.get(user_address);
        let mut tasks = Vec::new();
        for i in 0..tasks_accessor.len() {
            if let Some(tasks_guard) = tasks_accessor.get(i) {
                tasks.push(tasks_guard.get_string());
            }
        }
        tasks
    }

    pub fn get_activities(&self, user_address: Address) -> Vec<String> {
        let activities_accessor = self.activities.get(user_address);
        let mut activities = Vec::new();
        for i in 0..activities_accessor.len() {
            if let Some(activities_guard) = activities_accessor.get(i) {
                activities.push(activities_guard.get_string());
            }
        }
        activities
    }
}