class Adzan < Formula
  desc "Adzan reminder CLI with notification and adzan sound"
  homepage "https://github.com/itzmail/adzan-reminder"
  version "0.1.2"

  if Hardware::CPU.intel?
    url "https://github.com/itzmail/adzan-reminder/releases/download/v0.1.2/adzan-macos-x86_64"
    sha256 "b98abf29e7cbe64598a74d5fe2842d22202d630e31a46c4974903e5b86f148b0"
  else
    url "https://github.com/itzmail/adzan-reminder/releases/download/v0.1.2/adzan-macos-arm64"
    sha256 "0a5bbde81f282669431129959164263ce56239b89908b786a78b9fceb12b4919"
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
