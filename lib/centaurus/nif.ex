defmodule Centaurus.Nif do
  @moduledoc false

  use Rustler, otp_app: :centaurus, crate: :centaurus

  defp err(), do: :erlang.nif_error(:nif_not_loaded)

  # Core functions:
  def start(), do: err()
  def accept(_socket, _timeout), do: err()
  def connect(_socket_config, _stream_config, _address, _timeout), do: err()
  def close(_socket, _error_code, _reason), do: err()
  def close_stream(_stream, _error_code), do: err()
  def listen(_socket_config, _stream_config), do: err()
  def open_stream(_socket, _direction), do: err()
  def read(_stream, _amount, _timeout), do: err()
  def write(_stream, _data), do: err()

  # Logger section:
  def logger(), do: err()

  # Testing section:
  def create_cert_and_key(_directory, _server_name), do: err()
  def test_socket_config(_socket), do: err()
  def test_stream_config(_stream), do: err()
  def test_socket(_socket), do: err()
  def test_stream(_stream), do: err()
  def get_socket(), do: err()
  def get_stream(), do: err()
  def test_application_error(_error), do: err()
  def test_certificates(_certs), do: err()
  def test_private_key(_key), do: err()
  def test_socket_addr(_addr), do: err()
  def test_stream_type(_stream), do: err()
  def test_socket_type(_socket), do: err()
  def test_quic_opts(_opts), do: err()
  def test_quic_socket(_socket), do: err()
  def test_quic_stream(_stream), do: err()  
  def test_pid(_pid), do: err()
end
