defmodule KV.BucketTest do
  use ExUnit.Case, async: true

  setup do
    {:ok, b} = KV.Bucket.start_link()
    
    {:ok, b: b}
  end
  
  test "stores values by key", %{b: b} do
    assert KV.Bucket.get(b, "milk") == nil

    KV.Bucket.put(b, "milk", 3)
    assert KV.Bucket.get(b, "milk") == 3
  end
end

