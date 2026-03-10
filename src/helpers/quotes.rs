use rand::seq::SliceRandom;
use rand::Rng;
use serde::Deserialize;

#[derive(Deserialize, Debug)]
pub struct MessageBank {
    pub hadist_prioritas: Vec<Hadist>,
    pub quotes_sindiran: Vec<Quote>,
}

#[derive(Deserialize, Debug)]
pub struct Hadist {
    pub perawi: String,
    pub teks_indo: String,
    pub konteks: String,
}

#[derive(Deserialize, Debug)]
pub struct Quote {
    pub level: String,
    pub content: String,
}

const MESSAGE_JSON: &str = include_str!("../../assets/message.json");

pub fn get_random_message() -> String {
    let bank: MessageBank = match serde_json::from_str(MESSAGE_JSON) {
        Ok(b) => b,
        Err(e) => {
            eprintln!("Gagal membaca message.json: {}", e);
            return Default::default();
        }
    };

    let mut rng = rand::thread_rng();

    // Pilih secara acak antara hadist (0) atau quote (1)
    let is_hadist: bool = rng.gen_bool(0.5);

    if is_hadist && !bank.hadist_prioritas.is_empty() {
        if let Some(hadist) = bank.hadist_prioritas.choose(&mut rng) {
            return format!("\"{}\"\n\n— {}", hadist.teks_indo, hadist.perawi);
        }
    } else if !is_hadist && !bank.quotes_sindiran.is_empty() {
        if let Some(quote) = bank.quotes_sindiran.choose(&mut rng) {
            return format!("\"{}\"", quote.content);
        }
    }

    // Fallback jika json kosong
    "Waktunya sholat! Tinggalkan urusan dunia sejenak.".to_string()
}
