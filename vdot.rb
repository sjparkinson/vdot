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
    url "https://github.com/sjparkinson/vdot/releases/download/v0.3.4/vdot-v0.3.4-x86_64-apple-darwin.tar.gz"
    sha256 "4898d033b8d48ec0a960dd8d54f79324e26f7649d21718fb320a75e218fd94b0"
  elsif OS.linux?
    url "https://github.com/sjparkinson/vdot/releases/download/v0.3.4/vdot-v0.3.4-x86_64-unknown-linux-musl.tar.gz"
    sha256 "f4dd7845dc319038b34fcc50327628f123dfea1aee1e8d5e767836bd86559c1e"
  end

  def install
    bin.install "vdot"
  end

  test do
    system "#{bin}/vdot", "--help"
  end
end
