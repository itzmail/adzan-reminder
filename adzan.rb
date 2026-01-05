class Adzan < Formula
  desc "Adzan reminder CLI with notification and adzan sound"
  homepage "https://github.com/itzmail/adzan-reminder"
  url "https://github.com/itzmail/adzan-reminder/releases/download/v0.1.0/adzan-macos"
  sha256 "7b17e14801a3c4b23b19cdcdb054a7026b4427c9760016ea8f560b05bb1680b1"
  version "0.1.0"

  resource "adzan_sound" do
    url "https://raw.githubusercontent.com/itzmail/adzan-reminder/main/assets/suara_bedug.mp3"
    sha256 "ad06ceed5937b0e83a66dd2b8b33e139e787cf4fa4647921dac067a754623e6c"
  end

  def install
    bin.install "adzan-macos" => "adzan"

    # Buat folder assets
    (bin/"../assets").mkpath

    # Install suara adzan dari resource (handle nama file yang berubah)
    resource("adzan_sound").stage do
      mp3_file = Dir["*"].find { |f| f.end_with?(".mp3") }
      if mp3_file
        (bin/"../assets").install mp3_file => "suara_bedug.mp3"
      else
        raise "File MP3 tidak ditemukan setelah download"
      end
    end
  end

  test do
    system "#{bin}/adzan", "--help"
  end
end
