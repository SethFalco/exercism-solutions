defmodule FileSniffer do
  @spec type_from_extension(String.t()) :: String.t() | nil
  def type_from_extension(extension) do
    case extension do
      "exe" -> "application/octet-stream"
      "bmp" -> "image/bmp"
      "png" -> "image/png"
      "jpg" -> "image/jpg"
      "gif" -> "image/gif"
      _ -> nil
    end
  end

  @spec type_from_binary(binary()) :: String.t() | nil
  def type_from_binary(file_binary) do
    case file_binary do
      <<0x7F, 0x45, 0x4C, 0x46, _::binary>> -> "application/octet-stream"
      <<0x42, 0x4D, _::binary>> -> "image/bmp"
      <<0x89, 0x50, 0x4E, 0x47, 0x0D, 0x0A, 0x1A, 0x0A, _::binary>> -> "image/png"
      <<0xFF, 0xD8, 0xFF, _::binary>> -> "image/jpg"
      <<0x47, 0x49, 0x46, _::binary>> -> "image/gif"
      _ -> nil
    end
  end

  @spec verify(binary(), String.t()) :: {:ok | :error, String.t()}
  def verify(file_binary, extension) do
    mime = type_from_extension(extension)

    if mime !== nil and type_from_binary(file_binary) === mime do
      {:ok, mime}
    else
      {:error, "Warning, file format and file extension do not match."}
    end
  end
end
