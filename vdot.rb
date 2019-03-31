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

  if OS.mac?
    url "https://github.com/sjparkinson/vdot/releases/download/v0.3.7/vdot-v0.3.7-x86_64-apple-darwin.tar.gz"
    sha256 "d27cbe0401311ef9c8f722e1cd9a6cc5638f0bc16890e388e13a70dcfec1b2de"
  elsif OS.linux?
    url "https://github.com/sjparkinson/vdot/releases/download/v0.3.7/vdot-v0.3.7-x86_64-unknown-linux-musl.tar.gz"
    sha256 "13d1301fbf544784c198a0b9f6e43c942383664857b030ff1810785448916f10"
  end

  def install
    bin.install "vdot"
  end

  test do
    system "#{bin}/vdot", "--help"
  end
end
