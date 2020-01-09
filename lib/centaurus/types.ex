defmodule Centaurus.Types do
  @moduledoc """
  A collection of types used throughout Centaurus.
  The types are split into several different modules based on expected usage.
  """
  
  defmodule QuicSocket do
    @moduledoc """
    The types corresponding to sockets.
    """
    
    @typedoc """
    The socket type is a sum datatype for the nif, port, and unix socket types.
    """
    @type socket :: nif_socket | port_socket | unix_socket

    @typedoc """
    The nif_socket type corresponds to the nif connection.
    """
    @opaque nif_socket :: reference

    @typedoc """
    The port_socket type corresponds to Erlang's port type.
    """
    @opaque port_socket :: port

    @typedoc """
    The unix_socket type corresponds to Erlang's socket type.
    """
    @opaque unix_socket :: :socket.socket
    
    @typedoc """
    The stream_id type is a monotonically increasing positive integer starting 
    at 0 and incremented by every open stream. Stream 0 is the main stream.
    """
    @opaque stream_id :: non_neg_integer

    @typedoc """
    A unique representation to identify each quic connection and stream id.
    """
    @opaque quic_stream :: {socket, stream_id}

    @typedoc """
    A representation of the quic listening socket.
    """
    @opaque listen_socket :: socket
  end

  defmodule Common do
    @moduledoc """
    Common types used throughout network communications.
    e.g. :inet.ip_address or :inet.port_number or timeout
    """

    @type ip_addr :: :inet.ip_address
    @type port_number :: :inet.port_number
    @type timeout :: :infinity | non_neg_integer
    @type socket_options :: list(any)
    @type error :: any
  end

  defmodule Internal do
    @moduledoc """
    These are internal types for different functions.
    """
    
    @typedoc """
    Internal types
    """
    @opaque internal :: any
    
  end
  
end
