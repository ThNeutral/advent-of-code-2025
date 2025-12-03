defmodule Task1 do
  def main(_args \\ []) do
    lines = read_to_lines("./input.txt")
    rotations = pre_process_lines(lines)
    answer = count_answer(rotations)
    IO.puts answer
  end

  defp read_to_lines(path) do
    File.read!(path) |> String.split("\n") |> Enum.map(&String.trim/1)
  end

  defp pre_process_lines(lines) do
    Enum.map(lines, &extract_sign/1)
  end

  defp count_answer(rotations) do
    {_, answer} = Enum.reduce(rotations, {50, 0}, fn next, {rotation, answer} ->
      next_rotation = rotation + next

      passes = count_passes(rotation, next_rotation)

      {rem(next_rotation, 100), answer + passes}
    end)

    answer
  end

  defp count_passes(previous_rotation, next_rotation) do
    passes =
      if div(previous_rotation, 100) != div(next_rotation, 100) do
        prev_rotation_cycles = div(abs(previous_rotation), 100)
        next_rotation_cycles = div(abs(next_rotation), 100)

        abs(prev_rotation_cycles - next_rotation_cycles)
      else
        0
      end

    if previous_rotation != 0 and sign(previous_rotation) != sign(next_rotation) do
      passes + 1
    else
      passes
    end
  end

  defp extract_sign(<<"L" <> step_size_string>>) do
    -parse_integer!(step_size_string)
  end

  defp extract_sign(<<"R" <> step_size_string>>) do
    parse_integer!(step_size_string)
  end

  defp parse_integer!(string) do
    case Integer.parse(string) do
      {int, ""} ->
        int
      _ ->
        raise ArgumentError, message: ~c"Not an int: #{string}"
    end
  end

  defp sign(int) do
    cond do
      int < 0 -> -1
      int > 0 -> 1
      true -> 0
    end
  end
end
