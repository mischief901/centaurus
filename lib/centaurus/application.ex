defmodule Centaurus.Application do
  @moduledoc """
  The application side of Centaurus.
  """

  use Application

  
  def start(_type, _args) do
    children = []
    Supervisor.start_link(children, strategy: :one_for_one)
  end

  
  def stop(_state) do
    :ok
  end
  
end
