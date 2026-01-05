class Adzan < Formula
  desc "Adzan reminder CLI with notification and adzan sound"
  homepage "https://github.com/itzmail/adzan-reminder"
  url "https://github.com/itzmail/adzan-reminder/releases/download/v0.1.0/adzan-macos"
  sha256 "7b17e14801a3c4b23b19cdcdb054a7026b4427c9760016ea8f560b05bb1680b1"
  version "0.1.0"

  def install
    bin.install "adzan-macos" => "adzan"

    # Copy assets (suara adzan)
    (bin/"../assets").mkpath
    (bin/"../assets").install "assets/adzan_bedug.mp3"
  end

  test do
    system "#{bin}/adzan", "--help"
  end
end
