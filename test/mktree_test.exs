# mix run test/test_helper.exs
defmodule MktreeTest do
  use ExUnit.Case
  doctest Mktree

  test "hash_leaf generates correct hash" do
    data = "test_transaction"
    hash = Mktree.hash_leaf(data)

    assert String.length(hash) == 64
    assert hash != data
  end

  test "compute_root returns correct Merkle Root" do
    leaves = ["tx1", "tx2", "tx3", "tx4"]
    root = Mktree.compute_root(leaves)

    assert String.length(root) == 64
  end

  test "verify_proof confirms valid inclusion" do
    leaf = "tx1"
    proof = [{"valid_hash1", true}, {"valid_hash2", false}]
    root = "expected_merkle_root"

    assert Mktree.verify_proof(leaf, proof, root) == true
  end

  test "verify_proof rejects incorrect inclusion" do
    leaf = "tx_fake"
    proof = [{"invalid_hash1", true}, {"invalid_hash2", false}]
    root = "expected_merkle_root"

    refute Mktree.verify_proof(leaf, proof, root)
  end
end
