defmodule CentaurusTest do
  use ExUnit.Case
  doctest Centaurus

  alias Centaurus.Core
  alias Centaurus.Types
  alias Types.SocketConfig
  alias Types.StreamConfig
  alias Types.Options
  
  setup do
    socket_config = %SocketConfig {
      socket_pid: self(),
      bind_address: "127.0.0.1:9001",
      server_name: "centaurus_test",
      private_key: Path.join(:code.priv_dir(:centaurus), "server_key.pem"),
      certificates: Path.join(:code.priv_dir(:centaurus), "certificates"),
      options: %Options{}
    }
    stream_config_uni = %StreamConfig {
      stream_pid: self(),
      stream_type: :uni,
      options: %Options{}
    }
    stream_config_bi = %StreamConfig {
      stream_pid: self(),
      stream_type: :bi,
      options: %Options{}
    }
    {:ok, socket_config: socket_config, stream_config_uni: stream_config_uni, stream_config_bi: stream_config_bi}
  end

  test "Listen", context do
    {:ok, _socket} = Core.listen(context[:socket_config], context[:stream_config_uni])
    assert(true)
  end

  test "Connect", context do
    {:ok, _socket} = Core.connect(context[:socket_config], context[:stream_config_uni], 0, {0,0,0,0}, %Options{}, 0)
    assert(true)
  end
end
