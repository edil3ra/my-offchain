# Run 
cargo run -- transaction.csv

# Run integration test
cargo run test

# Assumption
It looks like tx is an autoincrement so I made the assumption that it was ordered.

# Concurrency
This code does not deal with concurency, if I had to used concurrency I will used tokio.
The bottleneck would be IO in this case as we will either read file form disk or from network.
It looks like a map reduce problem to solve it with concurrency.

# I did not add any benchmark, nor test it with huge data

# tests
Most of my test are dealing only with one client.
It should work exactly the same for multiple clients as order of rows does not matter.

