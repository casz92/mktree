defmodule Mktree do
  @moduledoc "Módulo para cálculos de Merkle Root utilizando un NIF en Rust."

  use Rustler,
  otp_app: :mktree,
  crate: "mktree_nif",
  path: "native/mktree_nif"

  def hash_leaf(_data), do: nif_error()
  def build_tree(_leaves), do: nif_error()
  def compute_root(_tree), do: nif_error()
  def verify_proof(_leaf, _proof, _root), do: nif_error()
  def generate_proof(_leaves, _target_leaf), do: nif_error()

  defp nif_error, do: raise "NIF no cargado"
end
