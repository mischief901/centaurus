defmodule Centaurus.Types do
  @moduledoc """
  A collection of types used throughout Centaurus.
  The types are split into several different modules based on expected usage.
  """
  
  defmodule QuicSocket do
    @moduledoc """
    The struct for Quic sockets.

    The struct has the following components:
    socket: A unique reference to identify the Quic socket
    ip_addr: The IP Address of the connection
    port: The port of the connection
    server_name: The server name for the certificates
    certificates: The path of where the certificates are located
    options: The connection's options (see options for details)
    """

    alias Types.Common

    defmodule QuicStream do
      @moduledoc """
      The struct for streams.

      The struct has the following components:
      stream_id: A unique reference to identify the stream
      socket_id: Ties the stream to the Quic socket
      direction: Either Bi-directional or Uni-directional access
      options: The stream's options (see options for details)
      data: Data to read from the stream.
      """

      @enforce_keys [:stream_id, :direction, :owner]
      defstruct [
        stream_id: nil,
        socket_id: nil,
        direction: :bi,
        data: "",
        options: [],
      ]

      @type t :: %__MODULE__{
        stream_id: stream_id,
        socket_id: Centaurus.Types.QuicSocket.socket,
        direction: :bi | :uni,
        data: String.t,
        options: Centaurus.Types.Common.socket_options,
      }

      @typedoc """
      The stream_id is a reference to a specific stream for the Rust component
      to identify. It does not have any useful application on the Elixir side.
      """
      @opaque stream_id :: reference
      
    end

    # TODO: Add certificates and server_name to enforced keys.
    @enforce_keys []
    defstruct [
      socket: nil,
      ip_addr: nil,
      port: nil,
      server_name: "",
      options: [],
      certificates: nil
    ]

    @type t :: %__MODULE__{
      socket: socket,
      ip_addr: Common.ip_addr,
      port: Common.port,
      server_name: String.t,
      options: Common.socket_options,
      certificates: Path.t
    }
    
    @typedoc """
    The socket type is not the actual socket. It is a reference to the 
    Rust thread. The reference is also maintained in the stream struct.
    """
    @opaque socket :: reference
    
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
    
    @typedoc """
    The error codes Quic uses to close streams and sockets.

    None: Communication is complete and there was no error.
    
    """
    @type error_code :: :none | any
    
  end

  defmodule Internal do
    
    @typedoc """
    Internal types
    """
    @opaque internal :: any
  end
  
end
