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
  version "0.4.14"

  if OS.mac?
    url "https://github.com/sjparkinson/vdot/releases/download/v#{version}/vdot-v#{version}-x86_64-apple-darwin.zip"
    sha256 "583680519d577d446ddfdb0ed6081a462682bde3b61aa18ea2cb1b323a88af41"
  elsif OS.linux?
    url "https://github.com/sjparkinson/vdot/releases/download/v#{version}/vdot-v#{version}-x86_64-unknown-linux-musl.zip"
    sha256 "bd4eb4bd223368cda5a9f0ce245c5dc186076f0484fa1cb63e91e70e6fa8bae6"
  end

  def install
    bin.install "vdot"
  end

  test do
    system "#{bin}/vdot", "--help"
  end
end
