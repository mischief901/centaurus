defmodule Centaurus.MixProject do
  use Mix.Project

  def project do
    [
      app: :centaurus,
      version: "0.1.0",
      elixir: "~> 1.10",
      compilers: [:rustler] ++ Mix.compilers(),
      rustler_crates: [
        centaurus: [
          mode: :debug
        ]
      ],
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
      {:rustler, "~> 0.22.0-rc.0"}
    ]
  end
end
