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
  """
  def build_tree(_leaves), do: nif_error()

  @doc """
  Computes the Merkle Root from a list of data leaves.
  First hashes the leaves and then builds the tree to get the root.
  """
  def compute_root(_tree), do: nif_error()

  @doc """
  Verifies a Merkle proof for a given leaf.
  Returns true if the proof is valid, false otherwise.
  """
  def verify_proof(_leaf, _proof, _root), do: nif_error()

  @doc """
  Generates a Merkle proof for a specific data leaf.
  Returns a list of tuples (sibling_hash, is_left_sibling).
  """
  def generate_proof(_leaves, _target_leaf), do: nif_error()

  defp nif_error, do: raise("NIF not loaded")
end
