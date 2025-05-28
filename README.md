# 🌳 Mktree: Merkle Tree in Rust for Elixir
Mktree is a high-performance library that implements **Merkle Trees** using **Rust + Elixir**. It provides efficient cryptographic hashing and validation of data, making it suitable for blockchain applications and secure data structures.

## ⚡ Features
✅ **SHA256 hashing of individual leaves** (`hash_leaf/1`)  
✅ **Building a complete Merkle tree** (`build_tree/1`)  
✅ **Computing the Merkle Root** (`compute_root/1`)  
✅ **Generating an inclusion proof** (`generate_proof/2`)  
✅ **Verifying inclusion proofs** (`verify_proof/3`)  

## 🚀 Installation
The package can be installed by adding mktree to your list of dependencies in mix.exs:

```elixir
def deps do
  [
    {:mktree, "~> 0.1.0"}
  ]
end
```

```bash
# Install dependencies
mix deps.get

# Compile the Rust NIF
cargo build

# Compile Elixir module
mix compile
```

## Usage Example
```elixir
# Define transaction data
leaves = ["tx1", "tx2", "tx3", "tx4"]

# Build Merkle tree
tree = Mktree.build_tree(leaves)

# Compute Merkle Root
root = Mktree.compute_root(leaves)
IO.puts("Merkle Root: #{root}")

# Generate an inclusion proof for "tx3"
proof = Mktree.generate_proof(leaves, "tx3")
IO.inspect(proof, label: "Proof generated for tx3")

# Verify proof
is_valid = Mktree.verify_proof("tx3", proof, root)
IO.puts("Proof validity for tx3: #{is_valid}")
```

## 🧑‍💻 Contributing
Contributions are welcome! Feel free to open issues or submit pull requests.

## ⚖️ License
This project is licensed under MIT License.
