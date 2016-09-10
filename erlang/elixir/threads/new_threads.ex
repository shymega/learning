parent = self()

spawn_link(fn ->
  send parent, {:msg, "Hello world!"}
end)

receive do
  {:msg, contents} -> IO.puts(contents)
end
