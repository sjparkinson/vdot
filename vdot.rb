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
  version "0.4.6"

  if OS.mac?
    url "https://github.com/sjparkinson/vdot/releases/download/v#{version}/vdot-v#{version}-x86_64-apple-darwin.zip"
    sha256 " d2e8386f038b8518da6b1df584580d6483c9e83ab736b7dd86c8d0a9b161cc8c"
  elsif OS.linux?
    url "https://github.com/sjparkinson/vdot/releases/download/v#{version}/vdot-v#{version}-x86_64-unknown-linux-musl.zip"
    sha256 " 99ab6aebb3563f1660d6f6ca260b4b0c99dd303048e073b21ae904d13e634fc2"
  end

  def install
    bin.install "vdot"
  end

  test do
    system "#{bin}/vdot", "--help"
  end
end
