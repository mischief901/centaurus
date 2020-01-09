defmodule Centaurus.Core do
  @moduledoc """
  This module holds the key function for working with QUIC sockets.

  See each function for details.
  """

  alias Centaurus.Types
  import Types.Common
  import Types.QuicSocket
  import Types.Internal
  
  @spec listen(port :: port_number,
    address :: ip_addr,
    opts :: socket_options) :: {:ok, listen_socket} | {:error, error}
  def listen(port, address, opts) do
    
  end

  @spec accept(lsocket :: listen_socket,
    opts :: socket_options) :: {:ok, socket} | {:error, error}
  def accept(lsocket, opts) do

  end

  @spec connect(port :: port_number,
    address :: ip_addr,
    opts :: socket_options,
    timeout :: timeout) :: {:ok, socket} | {:error, error}
  def connect(port, address, opts, timeout \\ :infinity)
  
  def connect(port, address, opts, timeout) do

  end

  @spec open_stream(socket :: socket,
    opts :: socket_options) :: {:ok, socket} | {:error, error}
  def open_stream(socket, opts) do

  end

  @spec close(socket :: socket) :: :ok | {:error, error}
  def close(socket) do

  end
  
end
