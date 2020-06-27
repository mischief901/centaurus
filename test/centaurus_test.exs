defmodule CentaurusTest do
  use ExUnit.Case
  doctest Centaurus

  alias Centaurus.Core
  alias Centaurus.Types
  alias Types.SocketConfig
  alias Types.StreamConfig
  alias Types.Options
  
  setup_all do
    cert_dir = :code.priv_dir(:centaurus) |> to_string
    Centaurus.Nif.create_cert_and_key(cert_dir, "centaurus_test")
    
    socket_config_der_server = %SocketConfig {
      socket_pid: self(),
      bind_address: "127.0.0.1:9001",
      server_name: "centaurus_test",
      private_key: Path.join(:code.priv_dir(:centaurus), "key.der"),
      certificates: Path.join(:code.priv_dir(:centaurus), "cert.der"),
      options: %Options{}
    }
    socket_config_pem_server = %SocketConfig {
      socket_pid: self(),
      bind_address: "127.0.0.1:9002",
      server_name: "centaurus_test",
      private_key: Path.join(:code.priv_dir(:centaurus), "key.pem"),
      certificates: Path.join(:code.priv_dir(:centaurus), "cert.pem"),
      options: %Options{}
    }
    socket_config_der_client = %SocketConfig {
      socket_pid: self(),
      bind_address: "127.0.0.1:9003",
      server_name: "centaurus_test",
      private_key: Path.join(:code.priv_dir(:centaurus), "key.der"),
      certificates: Path.join(:code.priv_dir(:centaurus), "cert.der"),
      options: %Options{}
    }
    socket_config_pem_client = %SocketConfig {
      socket_pid: self(),
      bind_address: "127.0.0.1:9004",
      server_name: "centaurus_test",
      private_key: Path.join(:code.priv_dir(:centaurus), "key.pem"),
      certificates: Path.join(:code.priv_dir(:centaurus), "cert.pem"),
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

    {:ok, runtime} = Centaurus.Core.start()
    
    {:ok,
     socket_config_server: %{der: socket_config_der_server,
                             pem: socket_config_pem_server},
     socket_config_client: %{der: socket_config_der_client,
                             pem: socket_config_pem_client},
     stream_config_uni: stream_config_uni,
     stream_config_bi: stream_config_bi,
     runtime: runtime
    }
  end

  test "Listen Der", context do
    socket_config = %SocketConfig{context[:socket_config_server][:der] | socket_pid: self()}
    stream_config = %StreamConfig{context[:stream_config_uni] | stream_pid: self()}
    {:ok, socket} = Core.listen(socket_config, stream_config)
    {:ok, _socket} = Core.accept(socket, 10_000)
    assert(true)
  end

  test "Listen Pem", context do
    socket_config = %SocketConfig{context[:socket_config_server][:pem] | socket_pid: self()}
    stream_config = %StreamConfig{context[:stream_config_uni] | stream_pid: self()}
    {:ok, socket} = Core.listen(socket_config, stream_config)
    {:ok, _socket} = Core.accept(socket, 10_000)
    assert(true)
  end

  test "Connect Der", context do
    socket_config = %SocketConfig{context[:socket_config_client][:der] | socket_pid: self()}
    stream_config = %StreamConfig{context[:stream_config_uni] | stream_pid: self()}
    {:ok, _socket} = Core.connect(socket_config, stream_config, 9001, {127,0,0,1}, %Options{}, 10_000)
    assert(true)
  end

  test "Connect Pem", context do
    socket_config = %SocketConfig{context[:socket_config_client][:pem] | socket_pid: self()}
    stream_config = %StreamConfig{context[:stream_config_uni] | stream_pid: self()}
    {:error, _error} = Core.connect(socket_config, stream_config, 0, {0,0,0,0}, %Options{}, 0)
    assert(true)
  end
end
