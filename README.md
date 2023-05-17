## Create a new directory and a new Rust package for each day:

For example, to create a package for day 1, you would run:

bash
Copy code
mkdir day1
cd day1
cargo init --bin
cd ..

This creates a new directory named day1, navigates into it, creates a new binary Rust package there, and then navigates back to the root directory of the project.

## Write your code:

You would write the code for each day in the src/main.rs file of the respective package's directory.

## Run your code:

To run the code for a specific day, you would use the cargo run -p dayN command, where N is the number of the day. For example, to run the code for day 1, you would run cargo run -p day1.