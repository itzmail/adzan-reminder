use std::collections::HashMap;

use crate::helpers::serde_helpers::string_or_null;
use serde::{Deserialize, Serialize};

/// Entity untuk satu kota/kabupaten
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Kota {
    #[serde(deserialize_with = "string_or_null")]
    pub id: String,

    #[serde(deserialize_with = "string_or_null")]
    pub lokasi: String,
}

/// Response list kota
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct DaftarKotaResponse {
    pub status: bool,
    pub data: Vec<Kota>,
}

/// Koordinat lokasi (optional, kalau nanti mau pakai)
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Koordinat {
    pub lat: f64,
    pub lon: f64,
    pub lintang: String,
    pub bujur: String,
}

/// Jadwal sholat satu hari
#[derive(Serialize, Deserialize, Debug, Clone, Default)]
pub struct JadwalSholat {
    pub tanggal: String,
    pub imsak: String,
    pub subuh: String,
    pub terbit: String,
    pub dhuha: String,
    pub dzuhur: String,
    pub ashar: String,
    pub maghrib: String,
    pub isya: String,
}

/// Data utama jadwal
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct JadwalData {
    pub id: String,
    pub kabko: String,
    pub prov: String,
    pub jadwal: HashMap<String, JadwalSholat>,
}

/// Response jadwal harian
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct JadwalResponse {
    pub status: bool,
    pub message: String,
    pub data: JadwalData,
}

#[cfg(test)]
mod tests {
    use super::*;
    use serde_json;

    #[test]
    fn test_deserialize_jadwal_response() {
        let json_response = r#"
        {
          "status": true,
          "message": "success",
          "data": {
            "id": "eda80a3d5b344bc40f3bc04f65b7a357",
            "kabko": "KOTA KEDIRI",
            "prov": "JAWA TIMUR",
            "jadwal": {
              "2025-12-30": {
                "tanggal": "Selasa, 30/12/2025",
                "imsak": "03:44",
                "subuh": "03:54",
                "terbit": "05:14",
                "dhuha": "05:43",
                "dzuhur": "11:38",
                "ashar": "15:04",
                "maghrib": "17:55",
                "isya": "19:11"
              }
            }
          }
        }
        "#;

        let result: Result<JadwalResponse, _> = serde_json::from_str(json_response);

        match result {
            Ok(response) => {
                assert_eq!(response.status, true);
                assert_eq!(response.message, "success");
                assert_eq!(response.data.id, "eda80a3d5b344bc40f3bc04f65b7a357");
                assert_eq!(response.data.kabko, "KOTA KEDIRI");
                assert_eq!(response.data.prov, "JAWA TIMUR");

                // Test jadwal untuk tanggal 2025-12-30
                let jadwal_hari = response.data.jadwal.get("2025-12-30").unwrap();
                assert_eq!(jadwal_hari.tanggal, "Selasa, 30/12/2025");
                assert_eq!(jadwal_hari.imsak, "03:44");
                assert_eq!(jadwal_hari.subuh, "03:54");
                assert_eq!(jadwal_hari.terbit, "05:14");
                assert_eq!(jadwal_hari.dhuha, "05:43");
                assert_eq!(jadwal_hari.dzuhur, "11:38");
                assert_eq!(jadwal_hari.ashar, "15:04");
                assert_eq!(jadwal_hari.maghrib, "17:55");
                assert_eq!(jadwal_hari.isya, "19:11");

                println!("✅ DTO berhasil deserialize response JSON dengan benar!");
            }
            Err(e) => {
                panic!("❌ Gagal deserialize JSON: {}", e);
            }
        }
    }
}
