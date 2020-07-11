{:ok, _logger} = Centaurus.Logger.start_logger()

{:ok, _runtime} = Centaurus.Core.start()

IO.inspect("Started logger and runtime.")

ExUnit.start()

