class Vdot < Formula
  desc "Create your .env files using Vault."
  homepage "https://github.com/sjparkinson/vdot"

  url "$STABLE_ASSET_URL"
  sha256 "$STABLE_ASSET_SHA256"

  def install
    bin.install "vdot"
  end

  test do
    system "#{bin}/vdot", "--help"
  end
end
