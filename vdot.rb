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
  version "0.4.8"

  if OS.mac?
    url "https://github.com/sjparkinson/vdot/releases/download/v#{version}/vdot-v#{version}-x86_64-apple-darwin.zip"
    sha256 "f48b45e547a258c0e055c3f0ba943f096ef3bbfb3effa57571770965b53a7a5f"
  elsif OS.linux?
    url "https://github.com/sjparkinson/vdot/releases/download/v#{version}/vdot-v#{version}-x86_64-unknown-linux-musl.zip"
    sha256 "2f52faaf82bd705eed6928972d05d27c1b75a11f6aaa580748b41ed25707270c"
  end

  def install
    bin.install "vdot"
  end

  test do
    system "#{bin}/vdot", "--help"
  end
end
