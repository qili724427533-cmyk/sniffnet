#![allow(clippy::match_same_arms)]

use crate::translations::types::language::Language;

pub fn filter_traffic_translation(language: Language) -> String {
    match language {
        Language::EN => "Filter traffic",
        Language::CS => "Filtr provozu",
        Language::IT => "Filtra il traffico",
        Language::DE => "Datenverkehr filtern",
        Language::ZH => "筛选流量",
        Language::ZH_TW => "篩選流量",
        Language::TR => "Trafiği filtrele",
        Language::JA => "トラフィックをフィルタリング",
        Language::ES => "Filtrar tráfico",
        Language::RO => "Filtrează traficul",
        Language::ID => "Filter lalulintas data",
        Language::FR => "Filtrer le traffic",
        Language::UK => "Фільтр трафіку",
        Language::SV => "Filtrera trafik",
        Language::EL => "Φιλτράρισμα ροής",
        _ => "Filter traffic",
    }
    .to_string()
}

// the source from which Sniffnet reads network traffic (i.e., a capture file or a network adapter)
pub fn traffic_source_translation(language: Language) -> &'static str {
    match language {
        Language::EN => "Traffic source",
        Language::CS => "Zdroj provozu",
        Language::IT => "Fonte del traffico",
        Language::DE => "Datenquelle",
        Language::ZH => "流量来源",
        Language::ZH_TW => "流量來源",
        Language::TR => "Trafik kaynağı",
        Language::JA => "トラフィック元",
        Language::ES => "Fuente de tráfico",
        Language::RO => "Sursa traficului",
        Language::ID => "Asal lalulintas data",
        Language::FR => "Source du traffic",
        Language::UK => "Джерело трафіку",
        Language::SV => "Trafikkälla",
        Language::EL => "Πηγή ροής",
        _ => "Traffic source",
    }
}

pub fn remote_notifications_translation(language: Language) -> &'static str {
    match language {
        Language::EN => "Remote notifications",
        Language::IT => "Notifiche remote",
        Language::DE => "Remote-Benachrichtigungen",
        Language::ZH => "远程通知",
        Language::ZH_TW => "遠端通知",
        Language::TR => "Uzak bildirimler",
        Language::JA => "リモート通知",
        Language::ES => "Notificaciones remotas",
        Language::RO => "Notificări la distanță",
        Language::ID => "Pemberitahuan jarak jauh",
        Language::FR => "Notifications distantes",
        Language::UK => "Віддалені сповіщення",
        Language::SV => "Fjärrnotiser",
        Language::EL => "Απομακρυσμένες ειδοποιήσεις",
        _ => "Remote notifications",
    }
}

pub fn ip_blacklist_translation(language: Language) -> &'static str {
    match language {
        Language::EN => "IP blacklist",
        Language::IT => "Blacklist IP",
        Language::DE => "IP-Blacklist",
        Language::ZH => "IP 黑名单",
        Language::ZH_TW => "IP 黑名單",
        Language::TR => "IP kara listesi",
        Language::JA => "IP ブラックリスト",
        Language::ES => "Blacklist de IPs",
        Language::RO => "Blacklist IP-uri",
        Language::ID => "Daftar hitam IP",
        Language::FR => "Blacklist d'IP",
        Language::UK => "Чорний список IP-адрес",
        Language::SV => "IP-svartlista",
        Language::EL => "Λίστα μπλοκαρισμένων διευθύνσεων",
        _ => "IP blacklist",
    }
}

pub fn ip_blacklist_not_selected_translation(language: Language) -> &'static str {
    match language {
        Language::EN => "No blacklist file selected.",
        Language::CS => "Není vybrán žádný soubor blacklistu.",
        Language::DE => "Keine Blacklist-Datei ausgewählt.",
        Language::EL => "Δεν έχει επιλεγεί αρχείο λίστας αποκλεισμού.",
        Language::ES => "No se ha seleccionado ningún archivo de blacklist.",
        Language::FI => "Blacklist-tiedostoa ei ole valittu.",
        Language::FR => "Aucun fichier de blacklist sélectionné.",
        Language::ID => "Belum ada berkas daftar hitam yang dipilih.",
        Language::IT => "Nessun file blacklist selezionato.",
        Language::JA => "ブラックリストファイルが選択されていません。",
        Language::KO => "선택된 블랙리스트 파일이 없습니다.",
        Language::NL => "Geen blacklistbestand geselecteerd.",
        Language::PL => "Nie wybrano pliku blacklisty.",
        Language::PT => "Nenhum arquivo de blacklist selecionado.",
        Language::RO => "Nu a fost selectat niciun fișier blacklist.",
        Language::RU => "Файл черного списка не выбран.",
        Language::SV => "Ingen svartlistfil har valts.",
        Language::TR => "Kara liste dosyası seçilmedi.",
        Language::UK => "Файл чорного списку не вибрано.",
        Language::UZ => "Qora ro'yxat fayli tanlanmagan.",
        Language::VI => "Chưa chọn tệp danh sách đen.",
        Language::ZH => "未选择黑名单文件。",
        Language::ZH_TW => "未選擇黑名單檔案。",
    }
}

pub fn ip_blacklist_loading_translation(language: Language) -> &'static str {
    match language {
        Language::EN => "Loading blacklist...",
        Language::CS => "Načítá se blacklist...",
        Language::DE => "Blacklist wird geladen...",
        Language::EL => "Φόρτωση λίστας αποκλεισμού...",
        Language::ES => "Cargando blacklist...",
        Language::FI => "Ladataan blacklistia...",
        Language::FR => "Chargement de la blacklist...",
        Language::ID => "Memuat daftar hitam...",
        Language::IT => "Caricamento blacklist...",
        Language::JA => "ブラックリストを読み込み中...",
        Language::KO => "블랙리스트를 불러오는 중...",
        Language::NL => "Blacklist laden...",
        Language::PL => "Wczytywanie blacklisty...",
        Language::PT => "Carregando blacklist...",
        Language::RO => "Se încarcă blacklistul...",
        Language::RU => "Загрузка черного списка...",
        Language::SV => "Läser in svartlistan...",
        Language::TR => "Kara liste yükleniyor...",
        Language::UK => "Завантаження чорного списку...",
        Language::UZ => "Qora ro'yxat yuklanmoqda...",
        Language::VI => "Đang tải danh sách đen...",
        Language::ZH => "正在加载黑名单...",
        Language::ZH_TW => "正在載入黑名單...",
    }
}

pub fn ip_blacklist_file_read_error_translation(language: Language) -> &'static str {
    match language {
        Language::EN => "Could not read blacklist file.",
        Language::CS => "Soubor blacklistu nelze přečíst.",
        Language::DE => "Blacklist-Datei konnte nicht gelesen werden.",
        Language::EL => "Δεν ήταν δυνατή η ανάγνωση του αρχείου λίστας αποκλεισμού.",
        Language::ES => "No se pudo leer el archivo de blacklist.",
        Language::FI => "Blacklist-tiedostoa ei voitu lukea.",
        Language::FR => "Impossible de lire le fichier de blacklist.",
        Language::ID => "Berkas daftar hitam tidak dapat dibaca.",
        Language::IT => "Impossibile leggere il file blacklist.",
        Language::JA => "ブラックリストファイルを読み取れませんでした。",
        Language::KO => "블랙리스트 파일을 읽을 수 없습니다.",
        Language::NL => "Kan het blacklistbestand niet lezen.",
        Language::PL => "Nie można odczytać pliku blacklisty.",
        Language::PT => "Não foi possível ler o arquivo de blacklist.",
        Language::RO => "Nu s-a putut citi fișierul blacklist.",
        Language::RU => "Не удалось прочитать файл черного списка.",
        Language::SV => "Kunde inte läsa svartlistfilen.",
        Language::TR => "Kara liste dosyası okunamadı.",
        Language::UK => "Не вдалося прочитати файл чорного списку.",
        Language::UZ => "Qora ro'yxat faylini o'qib bo'lmadi.",
        Language::VI => "Không thể đọc tệp danh sách đen.",
        Language::ZH => "无法读取黑名单文件。",
        Language::ZH_TW => "無法讀取黑名單檔案。",
    }
}

pub fn ip_blacklist_no_valid_entries_translation(language: Language) -> &'static str {
    match language {
        Language::EN => "No valid IP addresses or CIDR ranges found.",
        Language::CS => "Nebyly nalezeny žádné platné IP adresy ani rozsahy CIDR.",
        Language::DE => "Keine gültigen IP-Adressen oder CIDR-Bereiche gefunden.",
        Language::EL => "Δεν βρέθηκαν έγκυρες διευθύνσεις IP ή εύρη CIDR.",
        Language::ES => "No se encontraron direcciones IP ni rangos CIDR válidos.",
        Language::FI => "Kelvollisia IP-osoitteita tai CIDR-alueita ei löytynyt.",
        Language::FR => "Aucune adresse IP ni plage CIDR valide trouvée.",
        Language::ID => "Tidak ada alamat IP atau rentang CIDR yang valid ditemukan.",
        Language::IT => "Nessun indirizzo IP o intervallo CIDR valido trovato.",
        Language::JA => "有効な IP アドレスまたは CIDR 範囲が見つかりませんでした。",
        Language::KO => "유효한 IP 주소 또는 CIDR 범위를 찾을 수 없습니다.",
        Language::NL => "Geen geldige IP-adressen of CIDR-bereiken gevonden.",
        Language::PL => "Nie znaleziono prawidłowych adresów IP ani zakresów CIDR.",
        Language::PT => "Nenhum endereço IP ou intervalo CIDR válido encontrado.",
        Language::RO => "Nu au fost găsite adrese IP sau intervale CIDR valide.",
        Language::RU => "Действительные IP-адреса или диапазоны CIDR не найдены.",
        Language::SV => "Inga giltiga IP-adresser eller CIDR-intervall hittades.",
        Language::TR => "Geçerli IP adresi veya CIDR aralığı bulunamadı.",
        Language::UK => "Дійсні IP-адреси або діапазони CIDR не знайдено.",
        Language::UZ => "Yaroqli IP manzillar yoki CIDR oraliqlari topilmadi.",
        Language::VI => "Không tìm thấy địa chỉ IP hoặc dải CIDR hợp lệ.",
        Language::ZH => "未找到有效的 IP 地址或 CIDR 范围。",
        Language::ZH_TW => "找不到有效的 IP 位址或 CIDR 範圍。",
    }
}

pub fn ip_blacklist_loaded_translation(
    language: Language,
    ip_count: usize,
    network_count: usize,
) -> String {
    match language {
        Language::EN => {
            format!("Loaded {ip_count} IP addresses and {network_count} CIDR ranges.")
        }
        Language::CS => {
            format!("Načteno {ip_count} IP adres a {network_count} rozsahů CIDR.")
        }
        Language::DE => {
            format!("{ip_count} IP-Adressen und {network_count} CIDR-Bereiche geladen.")
        }
        Language::EL => {
            format!("Φορτώθηκαν {ip_count} διευθύνσεις IP και {network_count} εύρη CIDR.")
        }
        Language::ES => {
            format!("Cargadas {ip_count} direcciones IP y {network_count} rangos CIDR.")
        }
        Language::FI => {
            format!("Ladattu {ip_count} IP-osoitetta ja {network_count} CIDR-aluetta.")
        }
        Language::FR => {
            format!("{ip_count} adresses IP et {network_count} plages CIDR chargées.")
        }
        Language::ID => {
            format!("Dimuat {ip_count} alamat IP dan {network_count} rentang CIDR.")
        }
        Language::IT => {
            format!("Caricati {ip_count} indirizzi IP e {network_count} intervalli CIDR.")
        }
        Language::JA => {
            format!(
                "{ip_count} 件の IP アドレスと {network_count} 件の CIDR 範囲を読み込みました。"
            )
        }
        Language::KO => {
            format!("IP 주소 {ip_count}개와 CIDR 범위 {network_count}개를 불러왔습니다.")
        }
        Language::NL => {
            format!("{ip_count} IP-adressen en {network_count} CIDR-bereiken geladen.")
        }
        Language::PL => {
            format!("Wczytano adresów IP: {ip_count}, zakresów CIDR: {network_count}.")
        }
        Language::PT => {
            format!("Carregados {ip_count} endereços IP e {network_count} intervalos CIDR.")
        }
        Language::RO => {
            format!("Încărcate {ip_count} adrese IP și {network_count} intervale CIDR.")
        }
        Language::RU => {
            format!("Загружено IP-адресов: {ip_count}, диапазонов CIDR: {network_count}.")
        }
        Language::SV => {
            format!("{ip_count} IP-adresser och {network_count} CIDR-intervall har lästs in.")
        }
        Language::TR => {
            format!("{ip_count} IP adresi ve {network_count} CIDR aralığı yüklendi.")
        }
        Language::UK => {
            format!("Завантажено IP-адрес: {ip_count}, діапазонів CIDR: {network_count}.")
        }
        Language::UZ => {
            format!("{ip_count} ta IP manzil va {network_count} ta CIDR oralig'i yuklandi.")
        }
        Language::VI => {
            format!("Đã tải {ip_count} địa chỉ IP và {network_count} dải CIDR.")
        }
        Language::ZH => {
            format!("已加载 {ip_count} 个 IP 地址和 {network_count} 个 CIDR 范围。")
        }
        Language::ZH_TW => {
            format!("已載入 {ip_count} 個 IP 位址和 {network_count} 個 CIDR 範圍。")
        }
    }
}

pub fn blacklisted_transmitted_translation(language: Language) -> &'static str {
    match language {
        Language::EN => "New data exchanged from a blacklisted IP",
        Language::IT => "Nuovi dati scambiati da un IP in blacklist",
        Language::DE => "Neue Daten von einer IP auf der Blacklist ausgetauscht",
        Language::ZH => "与黑名单 IP 交换的新数据",
        Language::ZH_TW => "與黑名單 IP 交換的新資料",
        Language::TR => "Kara listedeki bir IP ile veri alışverişi yapıldı",
        Language::JA => "ブラックリストに登録されたIPから新しいデータが交換されました",
        Language::ES => "Nuevos datos intercambiados con una IP en la blacklist",
        Language::RO => "Noi date schimbate de la un IP în blacklist",
        Language::ID => "Data baru didapat dari daftar hitam IP",
        Language::FR => "Nouvelles données échangées depuis une IP blacklistée",
        Language::UK => "Отримано нові дані з IP-адреси з чорного списку",
        Language::SV => "Ny data utbytt från en svartlistad IP-adress",
        Language::EL => "Νέα δεδομένα ανταλλαχθηκαν από μια μπλοκαρισμένη διεύθυνση",
        _ => "New data exchanged from a blacklisted IP",
    }
}

pub fn only_show_blacklisted_translation(language: Language) -> &'static str {
    match language {
        Language::EN => "Only show blacklisted",
        Language::IT => "Mostra solo in blacklist",
        Language::DE => "Zeige nur auf der Blacklist Stehende",
        Language::ZH => "仅显示黑名单",
        Language::ZH_TW => "僅顯示黑名單",
        Language::TR => "Sadece kara listedekileri göster",
        Language::JA => "ブラックリストのみ表示",
        Language::ES => "Mostrar solo blacklist",
        Language::RO => "Afișează doar blacklist",
        Language::ID => "Hanya tampilkan daftar hitam",
        Language::FR => "Montrer seulement les blacklistées",
        Language::UK => "Показувати лише заблоковані",
        Language::SV => "Visa endast svartlistade",
        Language::EL => "Εμφάνιση μόνο μπλοκαρισμένων διευθύνσεων",
        _ => "Only show blacklisted",
    }
}

pub fn program_translation(language: Language) -> &'static str {
    match language {
        Language::EN | Language::TR | Language::RO | Language::ID | Language::SV => "Program",
        Language::IT => "Programma",
        Language::DE => "Programm",
        Language::ZH => "程序",
        Language::ZH_TW => "程式",
        Language::JA => "プログラム",
        Language::ES => "Programa",
        Language::FR => "Programme",
        Language::UK => "Програма",
        Language::EL => "Πρόγραμμα",
        _ => "Program",
    }
}

pub fn no_favorites_saved_translation(language: Language) -> &'static str {
    match language {
        Language::EN => "No favorites saved yet",
        Language::IT => "Nessun preferito salvato",
        Language::DE => "Noch keine Favoriten gespeichert",
        Language::ZH => "尚未保存任何收藏",
        Language::ZH_TW => "尚未儲存任何我的最愛",
        Language::TR => "Henüz kaydedilmiş favori yok",
        Language::JA => "お気に入りはまだ保存されていません",
        Language::ES => "Aún no hay favoritos guardados",
        Language::RO => "Niciun favorit salvat încă",
        Language::ID => "Belum ada favorit yang tersimpan",
        Language::FR => "Aucun favoris enregistrés pour le moment",
        Language::UK => "Ще немає збережених улюблених",
        Language::SV => "Inga favoriter sparade ännu",
        Language::EL => "Δεν έχει αποθηκευτεί κανένα αγαπημένο στοιχείο ακόμη",
        _ => "No favorites saved yet",
    }
}
