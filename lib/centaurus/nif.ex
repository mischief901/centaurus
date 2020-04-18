defmodule Centaurus.Nif do
  @moduledoc false

  use Rustler, otp_app: :centaurus, crate: :centaurus

  defp err(), do: :erlang.nif_error(:nif_not_loaded)

  def get_socket_config(), do: err()

  def get_stream_config(), do: err()
  
  def accept(_socket, _timeout), do: err()

  def connect(_socket_config, _stream_config, _address, _timeout), do: err()

  def close(_socket, _error_code, _reason), do: err()

  def close_stream(_stream, _error_code, _reason), do: err()

  def listen(_socket_config, _stream_config), do: err()

  def open_stream(_socket, _direction), do: err()

  def read(_stream, _amount, _timeout), do: err()

  def write(_stream, _data), do: err()    
end
