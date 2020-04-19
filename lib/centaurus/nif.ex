defmodule Centaurus.Nif do
  @moduledoc false

  use Rustler, otp_app: :centaurus, crate: :centaurus

  defp err(), do: :erlang.nif_error(:nif_not_loaded)
  
  def accept(_socket, _timeout), do: err()

  def connect(_socket_config, _stream_config, _address, _timeout), do: err()

  def close(_socket, _error_code, _reason), do: err()

  def close_stream(_stream, _error_code, _reason), do: err()

  def listen(_socket_config, _stream_config), do: err()

  def open_stream(_socket, _direction), do: err()

  def read(_stream, _amount, _timeout), do: err()

  def write(_stream, _data), do: err()    

  # The rest are for testing the translation between Elixir and Rust.
  def encode_socket_config(_socket), do: err()
  def encode_stream_config(_stream), do: err()
  def decode_socket_config(), do: err()
  def decode_stream_config(), do: err()
  def encode_socket(_socket), do: err()
  def encode_stream(_stream), do: err()
  def decode_socket(), do: err()
  def decode_stream(), do: err()
  def encode_application_error(_error), do: err()
  def decode_application_error(), do: err()
  def encode_certificates(_certs), do: err()
  def decode_certificates(), do: err()
  def encode_private_key(_key), do: err()
  def decode_private_key(), do: err()
  def encode_socket_addr(_addr), do: err()
  def decode_socket_addr(), do: err()
  def encode_stream_type(_stream), do: err()
  def decode_stream_type(), do: err()
  def encode_socket_type(_socket), do: err()
  def decode_socket_type(), do: err()
  def encode_quic_opts(_opts), do: err()
  def decode_quic_opts(), do: err()
  def encode_quic_socket(_socket), do: err()
  def encode_quic_stream(_stream), do: err()  

end
