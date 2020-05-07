# This is a Homebrew formula for vdot.
#
# You can install vdot with `brew` by running:
#
# ```
# brew tap sjparkinson/vdot https://github.com/sjparkinson/vdot
# brew install vdot
# ```
#
# See https://docs.brew.sh/Taps for more information.

class Vdot < Formula
  desc "Create your .env files and start processes using Vault"
  homepage "https://github.com/sjparkinson/vdot"
  version "0.4.7"

  if OS.mac?
    url "https://github.com/sjparkinson/vdot/releases/download/v#{version}/vdot-v#{version}-x86_64-apple-darwin.zip"
    sha256 "30d433d16a7a5a095f23a0d8d5b851449e00339b1594088b26192a1d44a4e92b"
  elsif OS.linux?
    url "https://github.com/sjparkinson/vdot/releases/download/v#{version}/vdot-v#{version}-x86_64-unknown-linux-musl.zip"
    sha256 "e831e17605c3679fc5d3ebe80d87e7de04ef2c97c0f9bfe7c555a73f3febbed9"
  end

  def install
    bin.install "vdot"
  end

  test do
    system "#{bin}/vdot", "--help"
  end
end
