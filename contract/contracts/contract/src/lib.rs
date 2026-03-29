#![no_std]
use soroban_sdk::{contract, contractimpl, Env, Address};

#[contract]
pub struct Remit;

#[contractimpl]
impl Remit {
   pub fn send(env: Env, from: Address, to: Address, amount: i128) {
       env.storage().instance().set(&(from, to), &amount);
   }
}
