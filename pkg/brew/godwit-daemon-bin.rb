class GodwitDaemonBin < Formula
  version '0.1.8'
  desc "A daemon runner for GodWit."
  homepage "https://github.com/Passeriform/GodWit-Daemon"

  if OS.mac?
      url "https://github.com/Passeriform/GodWit-Daemon/releases/download/#{version}/GodWit-Daemon-#{version}-x86_64-apple-darwin.tar.gz"
      sha256 "974351ca7d00083ba2fad52e2f2539c8ff114119c139420f592507962ab43b75"
  elsif OS.linux?
      url "https://github.com/Passeriform/GodWit-Daemon/releases/download/#{version}/GodWit-Daemon-#{version}-x86_64-unknown-linux.tar.gz"
      sha256 "c6bba6d643b1a1f18994683e26d4d2b998b41a7a7360e63cb8ec9db8ffbf793c"
  end

  conflicts_with "godwit-daemon"

  def install
    bin.install "godwit-daemon"
    man1.install "docs/godwit-daemon.1"

    bash_completion.install "completions/godwit-daemon.bash"
    zsh_completion.install "completions/godwit-daemon.zsh"
  end
end
