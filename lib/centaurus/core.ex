defmodule Centaurus.Core do
  @moduledoc """
  This module holds the key function for working with QUIC sockets.

  See each function for details.
  """

  alias Nif
  alias Centaurus.Types
  alias Types.Common, as: CommonType
  alias Types.Internal, as: Internal
  alias Types.QuicSocket, as: QuicSocketType
  
  @spec listen(port, address, opts) :: {:ok, listen_socket} | {:error, error}
  when port: CommonType.port_number,
    address: CommonType.ip_addr,
    opts: CommonType.socket_options,
    listen_socket: QuicSocketType.socket,
    error: CommonType.error
  def listen(port, address, opts) do
    Nif.listen_nif(port, address, opts)
  end

  @spec accept(lsocket, opts) :: {:ok, socket} | {:error, error}
  when lsocket: QuicSocketType.socket,
    opts: CommonType.socket_options,
    socket: QuicSocketType.socket,
    error: CommonType.error
  def accept(lsocket, opts) do
    Nif.accept_nif(lsocket, opts)
  end

  @spec connect(port, address, opts, timeout) :: {:ok, socket} | {:error, error}
  when port: CommonType.port_number,
    address: CommonType.ip_addr,
    opts: CommonType.socket_options,
    timeout: CommonType.timeout,
    socket: QuicSocketType.socket,
    error: CommonType.error
  def connect(port, address, opts, timeout \\ :infinity)
  
  def connect(port, address, opts, timeout) do
    Nif.connect_nif(port, address, opts, timeout)
  end

  @spec open_stream(socket, opts) :: {:ok, stream_id} | {:error, error}
  when socket: QuicSocketType.socket,
    opts: CommonType.socket_options,
    stream_id: QuicSocketType.stream,
    error: CommonType.error
  def open_stream(socket, opts) do
    Nif.open_stream(socket, opts)
  end

  @spec close_stream(stream) :: :ok
  when stream: QuicSocketType.stream
  def close_stream(stream) do
    Nif.close_stream(stream)
  end

  @spec read(stream, timeout) :: {:ok, data} | {:error, error}
  when stream: QuicSocketType.stream,
    timeout: CommonType.timeout,
    data: Internal.data,
    error: CommonType.error
  def read(stream, timeout) do
    Nif.read_nif(stream, timeout)
  end

  @spec write(stream, data) :: :ok | {:error, error}
  when stream: QuicSocketType.stream,
    data: Internal.data,
    error: CommonType.error
  def write(stream, data) do
    Nif.write_nif(stream, data)
  end
  
  @spec close(socket) :: :ok | {:error, error}
  when socket: QuicSocketType.socket,
    error: CommonType.error
  def close(socket) do
    Nif.close_nif(socket)
  end

  defmodule Nif do
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
  
end
