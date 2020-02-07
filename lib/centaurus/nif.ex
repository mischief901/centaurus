defmodule Centaurus.Nif do
  @moduledoc false

  @nif_error :erlang.nif_error(:nif_not_loaded)
  
  def accept_nif(_lsocket, _opts), do: @nif_error

  def connect_nif(_ip, _port, _timeout, _opts), do: @nif_error

  def close_nif(_socket), do: @nif_error

  def close_stream_nif(_stream), do: @nif_error

  def listen_nif(_port, _opts), do: @nif_error

  def open_stream_nif(_socket, _opts), do: @nif_error

  def read_nif(_stream, _timeout), do: @nif_error

  def write_nif(_stream, _data), do: @nif_error
  
end
