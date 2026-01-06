class Adzan < Formula
  desc "Adzan reminder CLI with notification and adzan sound"
  homepage "https://github.com/itzmail/adzan-reminder"
  version "0.1.1"

  if Hardware::CPU.intel?
    url "https://github.com/itzmail/adzan-reminder/releases/download/v0.1.2/adzan-macos-x86_64"
    sha256 "b98abf29e7cbe64598a74d5fe2842d22202d630e31a46c4974903e5b86f148b0"
  else
    url "https://github.com/itzmail/adzan-reminder/releases/download/v0.1.2/adzan-macos-arm64"
    sha256 "0a5bbde81f282669431129959164263ce56239b89908b786a78b9fceb12b4919"
  end

  def install
    bin.install (Hardware::CPU.intel? ? "adzan-macos-x86_64" : "adzan-macos-arm64") => "adzan"

    # Copy assets (suara adzan)
    (bin/"../assets").mkpath
    (bin/"../assets").install "assets/suara_bedug.mp3"
  end

  test do
    system "#{bin}/adzan", "--help"
  end
end
