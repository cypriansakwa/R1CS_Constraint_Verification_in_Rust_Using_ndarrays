# R1CS Constraint Verification in Rust Using ndarrays

This Rust program verifies a simple Rank-1 Constraint System (R1CS) equation using the `ndarray` crate.

## Overview

The program checks whether the following equation holds using matrix-vector multiplication:

41 × 103 = 4223

The witness vector and constraint matrices are represented using ndarrays, and the verification is performed by ensuring the computed left-hand side (LHS) matches the right-hand side (RHS).

## Prerequisites

Ensure you have Rust installed. If not, install it via rustup: https://rustup.rs/

## Installation

Clone the repository and navigate into the project directory:

git clone https://github.com/cypriansakwa/R1CS_Constraint_Verification_in_Rust_Using_ndarrays.git
cd R1CS_Constraint_Verification_in_Rust_Using_ndarrays

Then, add dependencies and build the project:
```csharp
[dependencies]
ndarray = "0.16.1"
```
cargo add ndarray cargo build

## Running the Program

Execute the program with:
```csharp
cargo run
```
If the constraint holds, the output will be:
R1CS constraint verified successfully!
