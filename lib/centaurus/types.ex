defmodule Centaurus.Types do
  @moduledoc """
  A collection of types used throughout Centaurus.
  The types are split into several different modules based on expected usage.
  """

  @typedoc """
  The socket type.
  """
  @opaque socket :: reference
    
  @typedoc """
  The stream type.
  """
  @opaque stream :: reference

  @typedoc """
  The IP Address of the peer connection. Used when connecting.
  """
  @type peer_addr :: ip_addr
  
  @typedoc """
  The port of the peer connection. Used when connecting.
  """
  @type peer_port :: port_number

  @type ip_addr :: :inet.ip_addresss
  @type port_number :: :inet.port_number

  @typedoc """
  The errors that can occur either from the runtime or setting up a configuration.
  """
  @type error :: any

  @typedoc """
  Internal types
  """
  @opaque internal :: any
  
  @typedoc """
  The error codes Quic uses to close streams and sockets.

  None: Communication is complete and there was no error.
  
  """
  @type error_code :: :none | any
  
  defmodule SocketConfig do
    @moduledoc """
    The struct for Quic sockets.

    The struct has the following components:
    bind_addr: The IP Address of the local connection
    bind_port: The port of the local connection
    server_name: The server name for the certificates
    server_key: The server's private key for the certificates
    certificates: The path of where the certificates are located
    options: The connection's options (see options for details)
    """
    
    # TODO: Add certificates and server_name to enforced keys.
    @enforce_keys []
    defstruct [
      socket_pid: nil,
      bind_address: "0.0.0.0:0",
      server_name: "",
      private_key: nil,
      options: [],
      certificates: nil
    ]

    alias Centaurus.Types
    
    @type t :: %__MODULE__{
      socket_pid: pid,
      bind_address: String.t,
      server_name: String.t,
      private_key: Path.t,
      options: Types.quic_options,
      certificates: Path.t
    }

    @spec set_opts(__MODULE__.t, opts) :: {:ok, __MODULE__.t} | {:error, Types.error}
    when opts: Types.quic_options
    def set_opts(socket_config, _opts) do
      {:ok, socket_config}
    end    
  end

  defmodule StreamConfig do
    @moduledoc """
    The struct for streams.

    The struct has the following components:

    stream_id: A pid to identify the stream owner
    socket_id: Ties the stream to the Quic socket
    direction: Either Bi-directional or Uni-directional access
    options: The stream's options (see options for details)
    data: Data to read from the stream.
    """

    alias Centaurus.Types
    
    @enforce_keys []
    defstruct [
      stream_pid: nil,
      stream_type: :bi,
      options: []
    ]

    @type t :: %__MODULE__{
      stream_pid: pid,
      stream_type: :bi | :uni,
      options: Types.quic_options,
    }
    
    @spec set_opts(__MODULE__.t, opts) :: {:ok, __MODULE__.t} | {:error, Types.error}
    when opts: Types.quic_options
    def set_opts(stream_config, _opts) do
      {:ok, stream_config}
    end
  end

  defmodule Options do
    defstruct [
      timeout: nil
    ]
    
    @typedoc """
    The possible set of configuration options for Quic sockets and streams.
    """
    @type t :: %__MODULE__{
      timeout: non_neg_integer() 
    }
  end
end
