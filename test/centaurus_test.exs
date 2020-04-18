defmodule CentaurusTest do
  use ExUnit.Case
  doctest Centaurus

  alias Centaurus.Core
  alias Centaurus.Nif
  alias Centaurus.Types
  alias Types.SocketConfig
  alias Types.StreamConfig
  
  setup do
    socket_config = %SocketConfig {
      bind_address: "127.0.0.1:9001",
      server_name: "centaurus_test",
      private_key: Path.join(:code.priv_dir(:centaurus), "server_key.pem"),
      certificates: Path.join(:code.priv_dir(:centaurus), "certificates")
    }
    stream_config_uni = %StreamConfig {
      direction: :uni
    }
    stream_config_bi = %StreamConfig {
      direction: :bi
    }
    {:ok, socket_config: socket_config, stream_config_uni: stream_config_uni, stream_config_bi: stream_config_bi}
  end

  test "Getting SocketConfig", context do
    socket_config = Nif.get_socket_config()
    :io.format("~p~n", [socket_config])
    :io.format("~p~n", [context[:socket_config]])
    assert(true)
  end


  test "Getting StreamConfig", context do
    stream_config = Nif.get_stream_config()
    :io.format("~p~n", [stream_config])
    :io.format("~p~n", [context[:stream_config_uni]])
    assert(true)
  end
  
  test "Listen", context do
    {:ok, _socket} = Core.listen(context[:socket_config], context[:stream_config_uni])
    assert(true)
  end

  test "Connect", context do
    {:ok, _socket} = Core.connect(context[:socket_config], context[:stream_config_uni], 0, {0,0,0,0}, [], 0)
    assert(true)
  end
end
