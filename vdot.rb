# typed: false
# frozen_string_literal: true

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
  url "https://github.com/sjparkinson/vdot/archive/refs/tags/v0.4.10.tar.gz"
  sha256 "358581663a301699193c92afca4c16a828879cac09fa42a5a582a19271ab9206"

  head "https://github.com/sjparkinson/vdot.git", branch: "main"

  livecheck do
    url :stable
    strategy :github_latest
  end

  depends_on "rust" => :build

  def install
    system "cargo", "install", *std_cargo_args
  end

  test do
    system "#{bin}/vdot", "--help"
  end
end
