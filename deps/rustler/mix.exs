defmodule Rustler.Mixfile do
  use Mix.Project

  def project do
    [app: :rustler,
     version: "0.21.0", # rustler_mix version
     elixir: "~> 1.6",
     build_embedded: Mix.env == :prod,
     start_permanent: Mix.env == :prod,
     name: "Rustler Mix",
     source_url: "https://github.com/rustlerium/rustler",
     homepage_url: "https://github.com/rusterlium/rustler",
     deps: deps(),
     docs: [
       extras: ["guides/Basics.md"],
       source_url_pattern: "https://github.com/rusterlium/rustler/blob/master/rustler_mix/%{path}#L%{line}"
     ],
     package: package(),
     description: description()]
  end

  def application do
    [applications: [:logger]]
  end

  defp deps do
    [
      {:toml, "~> 0.5.2"},
      {:ex_doc, "~> 0.19", only: :dev}
    ]
  end

  defp description do
    """
    Mix compiler and runtime helpers for Rustler.
    """
  end

  defp package do
    [files: ["lib", "priv", "mix.exs", "README.md"],
     maintainers: ["hansihe"],
     licenses: ["MIT", "Apache-2.0"],
     links: %{"GitHub" => "https://github.com/rusterlium/rustler"}]
  end
end
