defmodule Mktree do
  @moduledoc """
  Mktree is a high-performance library that implements Merkle Trees using Rust + Elixir. It provides efficient cryptographic hashing and validation of data, making it suitable for blockchain applications and secure data structures.
  """

  use Rustler,
    otp_app: :mktree,
    crate: "mktree_nif",
    path: "native/mktree_nif"

  @doc """
  hashing of individual leaves
  """
  def hash_leaf(_data), do: nif_error()

  @doc """
  Builds a complete Merkle tree from a list of hashed leaves.
  Returns a list of layers, where each layer is a list of hashes.
  The first layer are the initial leaves, the last is the root.

  Usage:
  ```elixir
  iex> Mktree.build_tree(["hash1", "hash2", "hash3", "hash4"])
  [["hash1", "hash2", "hash3", "hash4"], ["d8eab8000c5826fbf21e6340c96a911c7cf362c054695b73cb1a80ad0dac1cb0", ...]]
  ```
  """
  def build_tree(_leaves), do: nif_error()

  @doc """
  Computes the Merkle Root from a list of data leaves.
  First hashes the leaves and then builds the tree to get the root.
  Usage:
  ```elixir
  iex> root = Mktree.compute_root(["hash1", "hash2", "hash3", "hash4"])
  "93b46a24b0a418c5f6c31b4058dc5d0f3338a30951d3b4b5a74e9072f145c766"
  ```
  """
  def compute_root(_tree), do: nif_error()

  @doc """
  Generates a Merkle proof for a specific data leaf.
  Returns a list of tuples (sibling_hash, is_left_sibling).
  Usage:
  ```elixir
  iex> proof = Mktree.generate_proof(["hash1", "hash2", "hash3", "hash4"], "hash1")
  [{"e7bf382f6e5915b3f88619b866223ebf1d51c4c5321cccde2e9ff700a3259086", false}, ...]
  ```
  """
  def generate_proof(_leaves, _target_leaf), do: nif_error()

  @doc """
  Verifies a Merkle proof for a given leaf.
  Returns true if the proof is valid, false otherwise.
  Usage:
  ```elixir
  iex> Mktree.verify_proof("hash1", proof, root)
  true
  ```
  """
  def verify_proof(_leaf, _proof, _root), do: nif_error()

  defp nif_error, do: raise("NIF not loaded")
end
