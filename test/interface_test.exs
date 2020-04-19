defmodule InterfaceTest do
  use ExUnit.Case
  
  alias Centaurus.Types
  alias Centaurus.Nif
  alias Types.SocketConfig
  alias Types.StreamConfig
  
  setup do
    context = [
      socket_config: %SocketConfig{
        socket_pid: nil,
        bind_address: "127.0.0.1:0",
        server_name: "localhost",
        options: [],
        private_key: "/",
        certificates: "/"
      },
      stream_config: %StreamConfig{
        stream_pid: nil,
        stream_type: :bi,
        options: [],
      },
    ]
    {:ok, context}
  end

  test "SocketConfig", context do
    socket_config = Nif.decode_socket_config()
    assert(socket_config == {:ok, context[:socket_config]})
    assert({:ok, {}} == Nif.encode_socket_config(context[:socket_config]))
  end


  test "StreamConfig", context do
    stream_config = Nif.decode_stream_config()
    assert(stream_config == {:ok, context[:stream_config]})
    assert({:ok, {}} == Nif.encode_stream_config(context[:stream_config]))
  end

#  test "Socket", context do
#    
#  end

#  test "Stream", context do
    
#  end
    
end
