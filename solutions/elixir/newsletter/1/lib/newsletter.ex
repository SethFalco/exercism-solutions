defmodule Newsletter do
  @spec read_emails(String.t()) :: [String.t()]
  def read_emails(path) do
    {status, content} = File.read(path)

    case status do
      :ok -> String.trim(content) |> String.split("\n")
      _ -> []
    end
  end

  @spec open_log(String.t()) :: pid()
  def open_log(path) do
    elem(File.open(path, [:write]), 1)
  end

  @spec log_sent_email(pid(), String.t()) :: :ok
  def log_sent_email(pid, email) do
    IO.write(pid, "#{email}\n")
  end

  @spec close_log(pid()) :: :ok
  def close_log(pid) do
    File.close(pid)
  end

  @spec send_newsletter(String.t(), String.t(), any()) :: :ok
  def send_newsletter(emails_path, log_path, send_fun) do
    email_addresses = read_emails(emails_path)
    log = open_log(log_path)

    Enum.each(email_addresses, fn e ->
      if send_fun.(e) === :ok do
        log_sent_email(log, e)
      end
    end)

    close_log(log)
  end
end
