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
  version "0.4.0"

  if OS.mac?
    url "https://github.com/sjparkinson/vdot/releases/download/v#{version}/vdot-v#{version}-x86_64-apple-darwin.zip"
    sha256 "45717b92af2114eeeb98ea1e56fb15f8e8a904210510f8eebfbbf2abf1a1b55b"
  elsif OS.linux?
    url "https://github.com/sjparkinson/vdot/releases/download/v#{version}/vdot-v#{version}-x86_64-unknown-linux-musl.zip"
    sha256 "f3e3eed05547e01b272682d6b0f714661bf7d484fd3c07221e1ee7e19d763a28"
  end

  def install
    bin.install "vdot"
  end

  test do
    system "#{bin}/vdot", "--help"
  end
end
