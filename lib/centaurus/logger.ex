defmodule Centaurus.Log do
  @moduledoc """
  The Logger module provides a way to configure and run a backend logger for the Rust code.
  A separate logger is necessary due to the fairly small buffer used to pass messages between
  the Rust NIF and the Beam. Logs are collected and saved in "priv/log" with a timestamp. The
  max file size is 10 MB.
  """

  @doc """
  Starts the logger.
  """
  @spec start_logger() :: {:ok, pid}
  def start_logger() do
    Task.start_link(Centaurus.Nif, :logger, [])
  end
end
