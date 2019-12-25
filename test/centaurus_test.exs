defmodule CentaurusTest do
  use ExUnit.Case
  doctest Centaurus

  test "greets the world" do
    assert Centaurus.hello() == :world
  end
end
