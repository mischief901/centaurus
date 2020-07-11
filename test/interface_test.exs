defmodule InterfaceTest do
  use ExUnit.Case, async: true
  
  alias Centaurus.Types
  alias Centaurus.Nif
  alias Types.SocketConfig
  alias Types.StreamConfig
  alias Types.Options
  
  setup do
    context = [
      socket_config: %SocketConfig{
        socket_pid: self(),
        bind_address: "127.0.0.1:0",
        server_name: "localhost",
        options: %Options{},
        private_key: "/",
        certificates: "/"
      },
      stream_config: %StreamConfig{
        stream_pid: self(),
        stream_type: :bi,
        options: %Options{},
      },
    ]
    {:ok, context}
  end

  test "SocketConfig", context do
    assert({:ok, context[:socket_config]} == Nif.test_socket_config(context[:socket_config]))
  end

  test "StreamConfig", context do
    assert({:ok, context[:stream_config]} == Nif.test_stream_config(context[:stream_config]))
  end

  test "Pid roundtrip" do
    pid = self()
    pid_socket = Nif.test_quic_socket(pid)
    pid_stream = Nif.test_quic_stream(pid)
    assert({:ok, pid} == pid_socket)
    assert({:ok, pid} == pid_stream)
  end

  test "Socket" do
    {:ok, socket} = Nif.get_socket
    assert({:ok, socket} == Nif.test_socket(socket))
  end
  
  test "Stream" do
    {:ok, stream} = Nif.get_stream
    assert({:ok, stream} == Nif.test_stream(stream))
  end

  test "Application Error" do
    assert({:ok, 0} == Nif.test_application_error(0))
  end

  test "Certificates" do
    certs = "/home/centaurus/test"
    assert({:ok, certs} == Nif.test_certificates(certs))
  end

  test "Private Key" do
    key = "/home/centaurus/test"
    assert({:ok, key} == Nif.test_private_key(key))
  end

  test "Socket Addr" do
    addr = "127.0.0.1:0"
    assert({:ok, addr} == Nif.test_socket_addr(addr))
  end

  test "Socket Type" do
    assert({:ok, :server} == Nif.test_socket_type(:server))
    assert({:ok, :client} == Nif.test_socket_type(:client))
  end
  
  test "Stream Type" do
    assert({:ok, :bi} == Nif.test_stream_type(:bi))
    assert({:ok, :uni} == Nif.test_stream_type(:uni))
  end

  test "Quic Opts" do
    assert({:ok, %Options{}} == Nif.test_quic_opts(%Options{}))
    options = %Options{ timeout: 1000 }
    assert({:ok, options} == Nif.test_quic_opts(options))
  end
end
