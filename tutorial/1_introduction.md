# 1. Introduction

With this tutorial we will take you through the process of easily building your own blockchain using [Substrate](https://github.com/paritytech/substrate), an open source Rust Blockchain Development Kit by Parity. More specificaly, the goal is to create a simple domain name runtime module for the next generation of web.

A runtime is the block execution logic of the blockchain and consists of different runtime modules. A module typically consists of storage items, functions, and events to enable a certain set of features. The following are the modules that ship with the Substrate Runtime Module Library (SRML).

* [Assets](https://crates.parity.io/srml_assets/index.html) 
* [Aura](https://crates.parity.io/srml_aura/index.html)  
* [Balances](https://crates.parity.io/srml_balances/index.html) 
* [Consensus](https://crates.parity.io/srml_consensus/index.html) 
* [Contract](https://crates.parity.io/srml_contract/index.html) 
* [Council](https://crates.parity.io/srml_council/index.html) 
* [Democracy]() 
* [Finality Tracker](https://crates.parity.io/srml_democracy/index.html) 
* [Grandpa](https://crates.parity.io/srml_grandpa/index.html) 
* [Indices](https://crates.parity.io/srml_indices/index.html) 
* [Session](https://crates.parity.io/srml_session/index.html) 
* [Staking](https://crates.parity.io/srml_staking/index.html) 
* [Sudo](https://crates.parity.io/srml_sudo/index.html) 
* [Timestamp](https://crates.parity.io/srml_timestamp/index.html) 
* [Treasury](https://crates.parity.io/srml_treasury/index.html) 

Don’t worry if you have never used Rust before. If you have some programming experience, you will be able to follow the tutorial. Rust is a strongly typed programming language, which you’ll love to hate and will help you to build a reliable and efficient software. If you want to get a better understanding of Rust, feel free to take a look at the [official website](https://www.rust-lang.org/). This source also provides great Rust tutorials and books. 

In case you want to skip the tutorial or simply check certain things, you can find the finalized runtime module in the node folder of this repository.

**-> [Next: 2. Setup](./2_setup.md)**
