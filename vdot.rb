class Vdot < Formula
  desc "Create your .env files and start processes using Vault"
  homepage "https://github.com/sjparkinson/vdot"

  url "https://github.com/sjparkinson/vdot/releases/download/v0.3.4/vdot-v0.3.4-x86_64-apple-darwin.tar.gz"
  sha256 "4898d033b8d48ec0a960dd8d54f79324e26f7649d21718fb320a75e218fd94b0"

  def install
    bin.install "vdot"
  end

  test do
    system "#{bin}/vdot", "--help"
  end
end
