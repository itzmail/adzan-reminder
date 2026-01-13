class Adzan < Formula
  desc "Adzan reminder CLI with notification and adzan sound"
  homepage "https://github.com/itzmail/adzan-reminder"
  version "0.1.5"

  if Hardware::CPU.intel?
    url "https://github.com/itzmail/adzan-reminder/releases/download/v0.1.5/adzan-macos-x86_64"
    sha256 "383a64debd8763cfb5773ebc080c5562db8b9f0931d38f582c4e0e0f055d08df"
  else
    url "https://github.com/itzmail/adzan-reminder/releases/download/v0.1.5/adzan-macos-arm64"
    sha256 "89ae5ef8b78522a5e6898a1ecb04c1b15a30f06f47a5821eeccd2535befa4d93"
  end

  resource "adzan_sound" do
    url "https://raw.githubusercontent.com/itzmail/adzan-reminder/main/assets/suara_bedug.mp3"
    sha256 "ad06ceed5937b0e83a66dd2b8b33e139e787cf4fa4647921dac067a754623e6c"  # hash MP3-mu
  end

  def install
    bin.install (Hardware::CPU.intel? ? "adzan-macos-x86_64" : "adzan-macos-arm64") => "adzan"

    # Buat folder assets
    (bin/"../assets").mkpath

    # Install suara adzan dari resource
    resource("adzan_sound").stage do
      # Ambil file pertama (GitHub raw kadang kasih nama aneh)
      downloaded_file = Dir["*"].first
      if downloaded_file
        (bin/"../assets").install downloaded_file => "suara_bedug.mp3"
      else
        raise "File suara adzan tidak ditemukan setelah download"
      end
    end
  end

  test do
    system "#{bin}/adzan", "--help"
  end
end
