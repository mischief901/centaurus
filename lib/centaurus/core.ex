defmodule Centaurus.Core do
  @moduledoc """
  This module holds the key function for working with QUIC sockets.

  See each function for details.
  """

  alias Centaurus.Nif
  alias Centaurus.Types
  alias Types.SocketConfig
  alias Types.StreamConfig
  alias Types.Options

  @doc """
  Starts the underlying runtime.
  """
  @spec start() :: {:ok, pid}
  def start() do
    Task.start_link(Centaurus.Nif, :start, [])
  end
  
  @doc """
  Creates a socket configuration from the supplied options.
  """
  @spec socket_config(Options.t) :: {:ok, SocketConfig.t} | {:error, Types.error}
  def socket_config(quic_options) do
    %SocketConfig{}
    |> SocketConfig.set_opts(quic_options)
  end

  @doc """
  Creates a stream configuration from the supplied options.
  """
  @spec stream_config(Options.t) :: {:ok, StreamConfig.t} | {:error, Types.error}
  def stream_config(quic_options) do
    %StreamConfig{}
    |> StreamConfig.set_opts(quic_options)
  end
  
  @doc """
  Opens a Quic socket to listen for incoming connections.

  If port == 0, an available port number is assigned.
  """
  @spec listen(socket_config, stream_config) :: {:ok, Types.socket} | {:error, error}
  when socket_config: SocketConfig.t,
    stream_config: StreamConfig.t,
    error: Types.error
  def listen(socket_config, stream_config)
  def listen(socket_config, stream_config) do
    Nif.listen(socket_config, stream_config)
  end

  @doc """
  Accepts a single incoming connection and returns a QuicSocket for the new connection.
  """
  @spec accept(Types.socket, timeout) :: {:ok, Types.socket} | {:error, error}
  when timeout: timeout,
    error: Types.error
  def accept(socket, timeout \\ :infinity)
  def accept(socket, timeout) do
    Nif.accept(socket, timeout)
  end

  @doc """
  Opens a connection to the specified server. Returns a QuicSocket on success.
  """
  @spec connect(socket_config, stream_config, port, address, opts, timeout) :: {:ok, Types.socket} | {:error, error}
  when port: Types.port_number,
    address: Types.ip_addr,
    opts: Types.socket_options,
    timeout: timeout,
    socket_config: SocketConfig.t,
    stream_config: StreamConfig.t,
    error: Types.error
  def connect(socket_config, stream_config, port, address, opts, timeout \\ :infinity)
  def connect(socket_config, stream_config, port, address, _opts, timeout) do
    address = :inet.ntoa(address) |> to_string
    port = to_string(port)
    Nif.connect(socket_config, stream_config, address <> ":" <> port, timeout)
  end

  @doc """
  Opens a stream on the connection. Returns a QuicStream.
  Direction can either be :uni for unidirectional streams (write only) or
  :bi for bidirectional streams (read and write).
  """
  @spec open_stream(Types.socket, direction) :: {:ok, Types.stream} | {:error, error}
  when direction: :bi | :uni,
    error: Types.error
  def open_stream(socket, direction) do
    Nif.open_stream(socket, direction)
  end

  @doc """
  Closes the stream with the given error code (Default of none).
  Error codes are ignored for unidirectional streams. (Not Applicable)

  Valid error codes are:
  none - No error, communication is complete.
  """
  @spec close_stream(Types.stream, Types.error_code) :: :ok
  def close_stream(stream, error_code \\ :none)
  def close_stream(stream, error_code) do
    Nif.close_stream(stream, error_code)
  end

  @doc """
  Reads any available data from the stream.
  Timeout defaults to infinity.
  """
  @spec read(Types.stream, amount, timeout) :: {:ok, data} | {:error, error}
  when amount: non_neg_integer(),
    timeout: timeout,
    data: String.t,
    error: Types.error
  def read(stream, amount, timeout \\ :infinity)
  def read(stream, amount, timeout) do
    Nif.read(stream, amount, timeout)
  end

  @doc """
  Writes the data to the stream.
  """
  @spec write(Types.stream, data) :: :ok | {:error, error}
  when data: String.t,
    error: Types.error
  def write(stream, data) do
    Nif.write(stream, data)
  end

  @doc """
  Closes the socket with the given error code (Default of none).

  Valid error codes are:
  none - No error, communication is complete.
  """
  @spec close(Types.socket, error_code, reason) :: :ok
  when error_code: Types.error_code,
    reason: String.t
  def close(socket, error_code \\ :none, reason \\ "")
  def close(socket, error_code, reason) do
    Nif.close(socket, error_code, reason)
  end
end
