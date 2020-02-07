defmodule Centaurus.Core do
  @moduledoc """
  This module holds the key function for working with QUIC sockets.

  See each function for details.
  """

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
    
  end

  @spec accept(lsocket, opts) :: {:ok, socket} | {:error, error}
  when lsocket: QuicSocketType.socket,
    opts: CommonType.socket_options,
    socket: QuicSocketType.socket,
    error: CommonType.error
  def accept(lsocket, opts) do

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

  end

  @spec open_stream(socket, opts) :: {:ok, stream_id} | {:error, error}
  when socket: QuicSocketType.socket,
    opts: CommonType.socket_options,
    stream_id: QuicSocketType.stream,
    error: CommonType.error
  def open_stream(socket, opts) do

  end

  @spec close_stream(stream) :: :ok
  when stream: QuicSocketType.stream
  def close_stream(stream) do

  end

  @spec read(stream, timeout) :: {:ok, data} | {:error, error}
  when stream: QuicSocketType.stream,
    timeout: CommonType.timeout,
    data: Internal.data,
    error: CommonType.error
  def read(stream, timeout) do

  end

  @spec write(stream, data) :: :ok | {:error, error}
  when stream: QuicSocketType.stream,
    data: Internal.data,
    error: CommonType.error
  def write(stream, data) do

  end
  
  @spec close(socket) :: :ok | {:error, error}
  when socket: QuicSocketType.socket,
    error: CommonType.error
  def close(socket) do

  end
  
end
