defmodule Mktree.MixProject do
  use Mix.Project

  @version "0.1.1"

  def project do
    [
      app: :mktree,
      version: @version,
      elixir: "~> 1.14",
      description: "A merkle tree implementation in Elixir with Rustler",
      package: package(),
      start_permanent: Mix.env() == :prod,
      deps: deps()
    ]
  end

  def application do
    [
      extra_applications: [:logger]
    ]
  end

  defp deps do
    [
      {:rustler, "~> 0.34.0"},
      {:ex_doc, ">= 0.0.0", only: :dev, runtime: false}
    ]
  end

  defp package do
    [
      name: :mktree,
      files: ["lib", "native", "mix.exs", "README*", "LICENSE*", ".formatter.exs"],
      maintainers: ["Carlos Suarez"],
      licenses: ["MIT"],
      links: %{"GitHub" => "https://github.com/casz92/mktree"}
    ]
  end
end
