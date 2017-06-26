defmodule KV.Bucket do
  @doc """
  Starts a new bucket.
  """

  def start_link() do
    Agent.start_link(fn -> %{} end)
  end

  def get(b, k) do
    Agent.get(b, &Map.get(&1, k))
  end

  def put(b, k, v) do
    Agent.update(b, &Map.put(&1, k, v))
      assert KV.Bucket.get(bucket, "milk") == 3
end
end
