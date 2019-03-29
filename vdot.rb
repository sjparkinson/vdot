# This is a formula for enabling vdot to be installed with Homebrew.
#
# You can install vdot on macOS with `brew` by running:
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
    url "https://github.com/sjparkinson/vdot/releases/download/v0.3.6/vdot-v0.3.6-x86_64-apple-darwin.tar.gz"
    sha256 "361b5f0595735454724577d3c4553c697dd7fed4d7e0573fd387273b1ae2f587"
  elsif OS.linux?
    url "https://github.com/sjparkinson/vdot/releases/download/v0.3.6/vdot-v0.3.6-x86_64-unknown-linux-musl.tar.gz"
    sha256 "6ac26e727928efe3ae33b556d83e3939faf7d5a590e25a3bf47ea5ec86b3ece8"
  end

  def install
    bin.install "vdot"
  end

  test do
    system "#{bin}/vdot", "--help"
  end
end
