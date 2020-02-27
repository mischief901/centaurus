defmodule Centaurus do
  @moduledoc """
  Start and load nifs for either port or nif based communications.
  """

  @doc """
  Loads the shared library. If the path for the shared library is not supplied,
  the library is found at the location of :code.priv_dir.
  This function crashes when the shared library is not found.
  Valid options:
  - :nif
  - {:nif, path/to/lib.so}
  - :port
  - {:port, path/to/port}
  """
  @spec load(opt) :: :ok
  when opt: type | {type, String.t},
    type: :nif | :port
  def load :nif do
    priv_path = :code.priv_dir(__MODULE__)
    nif_path = Path.wildcard("native/*_nif")
    path = Path.join(priv_path, nif_path) |> Path.expand
    load {:nif, path}
  end

  def load :port do
    priv_path = :code.priv_dir(__MODULE__)
    port_path = Path.wildcard("native/*_port")
    path = Path.join(priv_path, port_path) |> Path.expand
    load {:port, path}
  end

  def load {:nif, path} do
    :ok = :erlang.load_nif(path)
    :ok
  end

  def load {:port, path} do
    true = File.exists?(path)
    :ok
  end  
end
