defmodule Basic do
  def main() do
    {:ok, bytes} = File.read("images/large.jpg")
    {:ok, mirage} = Mirage.from_bytes(bytes)
    {:ok, bytes, mirage} = Mirage.resize(mirage.resource, 600, 400)

    IO.puts(~s[New Height Is: #{mirage.height}])
    IO.puts(~s[New Width Is: #{mirage.width}])

    File.write("images/small.jpg", bytes)
  end
end
