defmodule Centaurus.Core do
  @moduledoc """
  This module holds the key function for working with QUIC sockets.

  See each function for details.
  """

  alias Nif
  alias Centaurus.Types
  alias Types.Common, as: CommonType
  alias Types.Internal, as: Internal
  alias Types.QuicSocket
  alias QuicSocket.QuicStream

  @doc """
  Opens a Quic socket to listen for incoming connections.

  If port == 0, an available port number is assigned.
  """
  @spec listen(port, opts) :: {:ok, quic_socket} | {:error, error}
  when port: CommonType.port_number,
    opts: CommonType.socket_options,
    quic_socket: QuicSocket.t,
    error: CommonType.error
  def listen(port, opts \\ [])
  def listen(port, opts) do
    %QuicSocket{
      port: port,
      options: opts
    }
    |> Nif.listen_nif
  end

  @doc """
  Accepts a single incoming connection and returns a QuicSocket for the new connection.
  """
  @spec accept(quic_socket, timeout) :: {:ok, socket} | {:error, error}
  when quic_socket: QuicSocket.t,
    timeout: CommonType.timeout,
    socket: QuicSocket.t,
    error: CommonType.error
  def accept(quic_socket, timeout \\ :infinity)
  def accept(quic_socket, timeout) do
    Nif.accept_nif(quic_socket, timeout)
  end

  @doc """
  Opens a connection to the specified server. Returns a QuicSocket on success.
  """
  @spec connect(port, address, opts, timeout) :: {:ok, quic_socket} | {:error, error}
  when port: CommonType.port_number,
    address: CommonType.ip_addr,
    opts: CommonType.socket_options,
    timeout: CommonType.timeout,
    quic_socket: QuicSocket.t,
    error: CommonType.error
  def connect(port, address, opts, timeout \\ :infinity)
  def connect(port, address, opts, timeout) do
    %QuicSocket{
      port: port,
      ip_addr: address,
      options: opts
    }
    |> Nif.connect_nif(timeout)
  end

  @doc """
  Opens a stream on the connection. Returns a QuicStream.
  Direction can either be :uni for unidirectional streams (write only) or
  :bi for bidirectional streams (read and write).
  """
  @spec open_stream(quic_socket, direction) :: {:ok, stream} | {:error, error}
  when quic_socket: QuicSocket.t,
    direction: :bi | :uni,
    error: CommonType.error,
    stream: QuicStream.t
  def open_stream(quic_socket) do
    Nif.open_stream(quic_socket)
  end

  @doc """
  Closes the stream with the given error code (Default of none).
  Error codes are ignored for unidirectional streams. (Not Applicable)

  Valid error codes are:
  none - No error, communication is complete.
  """
  @spec close_stream(stream, error_code) :: :ok
  when stream: QuicStream.t,
    error_code: CommonType.error_code
  def close_stream(stream, error_code \\ :none)
  def close_stream(stream, error_code) do
    Nif.close_stream(stream, error_code)
  end

  @doc """
  Reads any available data from the stream.
  Timeout defaults to infinity.
  """
  @spec read(stream, timeout) :: {:ok, data} | {:error, error}
  when stream: QuicStream.t,
    timeout: CommonType.timeout,
    data: Internal.data,
    error: CommonType.error
  def read(stream, timeout \\ :infinity)
  def read(stream, timeout) do
    Nif.read_nif(stream, timeout)
  end

  @doc """
  Writes the data to the stream.
  """
  @spec write(stream, data) :: :ok | {:error, error}
  when stream: QuicStream.t
    data: Internal.data,
    error: CommonType.error
  def write(stream, data) do
    Nif.write_nif(stream, data)
  end

  @doc """
  Closes the socket with the given error code (Default of none).

  Valid error codes are:
  none - No error, communication is complete.
  """
  @spec close(socket, error_code) :: :ok
  when socket: QuicSocket.t,
    error_code: CommonType.error_code
  def close(socket, error_code \\ :none)
  def close(socket, error_code) do
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
