//! UI formula syntax boundary. Engine commands, control/API callers and XLSX keep English.
use gridcraft_engine::formula::{self, FormulaLocale, ParseError};
use gridcraft_engine::model::Workbook;

pub(crate) fn canonical(
    text: &str,
    locale: FormulaLocale,
    wb: &Workbook,
    sheet: usize,
    cell: gridcraft_engine::core::CellRef,
) -> Result<String, ParseError> {
    if wb.sheet(sheet).is_some_and(|sh| wb.styles.get(sh.style_id(cell)).num_fmt.as_str() == "@") {
        return Ok(text.to_string());
    }
    // Same formula detection as the engine's `input_to_cell` (no trimming: " =1" is text).
    let body = text;
    if body.starts_with('=')
        || ((body.starts_with('+') || body.starts_with('-')) && body.chars().nth(1).is_some_and(|c| c.is_alphabetic() || c == '('))
    {
        let known = |name: &str| wb.name(name, sheet).is_some() || wb.table(name).is_some();
        if locale == FormulaLocale::En {
            let mut body = body.to_string();
            let mut expr = formula::parse_any_with_known(&body, known);
            for _ in 0..8 {
                if expr.is_ok() {
                    break;
                }
                body.push(')');
                expr = formula::parse_any_with_known(&body, known);
            }
            return Ok(format!("={}", formula::print(&expr?)));
        }
        // Match the engine's existing convenience for missing closing parentheses.
        let mut body = body.to_string();
        let mut expr = formula::parse_input(&body, locale, known).or_else(|_| formula::parse_any_with_known(&body, known));
        for _ in 0..8 {
            if expr.is_ok() {
                break;
            }
            body.push(')');
            expr = formula::parse_input(&body, locale, known).or_else(|_| formula::parse_any_with_known(&body, known));
        }
        return Ok(format!("={}", formula::print(&expr?)));
    }
    if locale == FormulaLocale::En {
        return Ok(text.to_string());
    }
    // Decimal-comma number entry follows the same preference, without rewriting text or dates.
    let normalized = body.trim().replace(locale.decimal_separator(), ".");
    let number = normalized.strip_suffix('%').unwrap_or(&normalized);
    if number.parse::<f64>().is_ok_and(f64::is_finite) {
        return Ok(normalized);
    }
    Ok(text.to_string())
}

pub(crate) fn display(text: &str, locale: FormulaLocale, wb: &Workbook, sheet: usize) -> String {
    // English (and every language without a formula table) shows the stored text untouched,
    // and skips a parse per frame.
    if locale == FormulaLocale::En {
        return text.to_string();
    }
    let Some(body) = text.strip_prefix('=') else { return text.to_string() };
    formula::parse_any(body)
        .and_then(|expr| formula::print_input(&expr, locale, |name| wb.name(name, sheet).is_some() || wb.table(name).is_some()))
        .map(|body| format!("={body}"))
        .unwrap_or_else(|_| text.to_string())
}

// Descriptions/signatures for the supported Spanish and Turkish function tables; untranslated
// functions retain their English help. Parameter punctuation follows the same locale as entry.
const SPANISH_HELP: &[(&str, &str, &str)] = &[
    ("SUM", "SUMA(número1; [número2]; ...)", "Suma los números."),
    ("AVERAGE", "PROMEDIO(número1; [número2]; ...)", "Devuelve el promedio de los números."),
    ("IF", "SI(prueba_lógica; valor_si_verdadero; [valor_si_falso])", "Devuelve un valor según una condición."),
    ("COUNT", "CONTAR(valor1; [valor2]; ...)", "Cuenta las celdas que contienen números."),
    ("SUMIF", "SUMAR.SI(rango; criterio; [rango_suma])", "Suma las celdas que cumplen un criterio."),
    ("COUNTIF", "CONTAR.SI(rango; criterio)", "Cuenta las celdas que cumplen un criterio."),
    ("VLOOKUP", "BUSCARV(valor_buscado; matriz; indicador_columnas; [ordenado])", "Busca un valor en la primera columna de una tabla."),
    ("TODAY", "HOY()", "Devuelve la fecha actual."),
    ("TRUE", "VERDADERO()", "Devuelve el valor lógico VERDADERO."),
    ("FALSE", "FALSO()", "Devuelve el valor lógico FALSO."),
];

const TURKISH_HELP: &[(&str, &str, &str)] = &[
    ("SUM", "TOPLA(sayı1; [sayı2]; ...)", "Sayıları toplar."),
    ("AVERAGE", "ORTALAMA(sayı1; [sayı2]; ...)", "Sayıların aritmetik ortalamasını döndürür."),
    (
        "IF",
        "EĞER(mantıksal_sınama; [eğer_doğruysa_değer]; [eğer_yanlışsa_değer])",
        "Bir koşul doğruysa bir değer, yanlışsa başka bir değer döndürür.",
    ),
    ("IFS", "ÇOKEĞER(sınama1; değer1; ...)", "İlk doğru koşula karşılık gelen değeri döndürür."),
    ("IFERROR", "EĞERHATA(değer; eğer_hataysa_değer)", "Formül hata verdiğinde yedek değeri döndürür."),
    ("IFNA", "EĞERYOKSA(değer; eğer_yoksa_değer)", "Formül #YOK hatası verdiğinde yedek değeri döndürür."),
    ("AND", "VE(mantıksal1; [mantıksal2]; ...)", "Tüm bağımsız değişkenler doğruysa DOĞRU döndürür."),
    ("OR", "YADA(mantıksal1; [mantıksal2]; ...)", "Bağımsız değişkenlerden herhangi biri doğruysa DOĞRU döndürür."),
    ("NOT", "DEĞİL(mantıksal)", "Mantıksal değerin tersini döndürür."),
    ("COUNT", "BAĞ_DEĞ_SAY(değer1; [değer2]; ...)", "Sayı içeren hücrelerin sayısını sayar."),
    ("COUNTA", "BAĞ_DEĞ_DOLU_SAY(değer1; [değer2]; ...)", "Boş olmayan hücrelerin sayısını sayar."),
    ("COUNTBLANK", "BOŞLUK_SAY(aralık)", "Aralıktaki boş hücrelerin sayısını sayar."),
    ("SUMIF", "ETOPLA(aralık; ölçüt; [toplam_aralığı])", "Belirli bir ölçütü karşılayan hücreleri toplar."),
    ("SUMIFS", "ÇOKETOPLA(toplam_aralığı; ölçüt_aralığı1; ölçüt1; ...)", "Birden çok ölçütü karşılayan hücreleri toplar."),
    ("COUNTIF", "EĞERSAY(aralık; ölçüt)", "Belirli bir ölçütü karşılayan hücreleri sayar."),
    ("COUNTIFS", "ÇOKEĞERSAY(ölçüt_aralığı1; ölçüt1; ...)", "Birden çok ölçütü karşılayan hücreleri sayar."),
    ("AVERAGEIF", "EĞERORTALAMA(aralık; ölçüt; [ortalama_aralığı])", "Belirli bir ölçütü karşılayan hücrelerin ortalamasını alır."),
    ("AVERAGEIFS", "ÇOKEĞERORTALAMA(ortalama_aralığı; ölçüt_aralığı1; ölçüt1; ...)", "Birden çok ölçütü karşılayan hücrelerin ortalamasını alır."),
    ("MIN", "MİN(sayı1; [sayı2]; ...)", "En küçük sayıyı döndürür."),
    ("MAX", "MAK(sayı1; [sayı2]; ...)", "En büyük sayıyı döndürür."),
    ("MEDIAN", "ORTANCA(sayı1; [sayı2]; ...)", "Verilen sayıların ortancasını döndürür."),
    ("LARGE", "BÜYÜK(dizi; k)", "Veri kümesindeki k. en büyük değeri döndürür."),
    ("SMALL", "KÜÇÜK(dizi; k)", "Veri kümesindeki k. en küçük değeri döndürür."),
    (
        "VLOOKUP",
        "DÜŞEYARA(aranan_değer; tablo_dizisi; sütun_indis_sayısı; [aralık_bak])",
        "Bir tablonun ilk sütununda değer arar ve aynı satırdaki değeri döndürür.",
    ),
    (
        "HLOOKUP",
        "YATAYARA(aranan_değer; tablo_dizisi; satır_indis_sayısı; [aralık_bak])",
        "Bir tablonun üst satırında değer arar ve aynı sütundaki değeri döndürür.",
    ),
    (
        "XLOOKUP",
        "ÇAPRAZARA(aranan_değer; arama_dizisi; döndürülen_dizi; [bulunamıyorsa]; [eşleştirme_modu]; [arama_modu])",
        "Bir aralık veya dizide eşleşme arar ve karşılık gelen öğeyi döndürür.",
    ),
    ("INDEX", "İNDİS(dizi; satır_sayısı; [sütun_sayısı]; [alan_sayısı])", "Belirli bir satır ve sütun kesişimindeki değeri veya başvuruyu döndürür."),
    ("MATCH", "KAÇINCI(aranan_değer; aranan_dizi; [eşleştirme_türü])", "Bir öğenin dizideki göreli konumunu döndürür."),
    ("XMATCH", "ÇAPRAZKAÇINCI(aranan_değer; aranan_dizi; [eşleştirme_modu]; [arama_modu])", "Bir öğenin dizideki göreli konumunu döndürür."),
    ("OFFSET", "KAYDIR(başvuru; satırlar; sütunlar; [yükseklik]; [genişlik])", "Belirli bir başvurudan kaydırılmış bir başvuru döndürür."),
    ("INDIRECT", "DOLAYLI(başv_metni; [a1])", "Metin dizesiyle belirtilen başvuruyu döndürür."),
    ("CHOOSE", "ELEMAN(indis_sayısı; değer1; [değer2]; ...)", "İndis numarasına göre listeden bir değer seçer."),
    ("FILTER", "FİLTRELE(dizi; ekle; [boşsa])", "Bir veri aralığını tanımladığınız ölçütlere göre filtreler."),
    ("SORT", "SIRALA(dizi; [sıralama_indisi]; [sıralama_düzeni]; [sütuna_göre])", "Bir aralığın veya dizinin içeriğini sıralar."),
    ("UNIQUE", "BENZERSİZ(dizi; [sütuna_göre]; [yalnızca_bir_kez])", "Bir listedeki veya aralıktaki benzersiz değerlerin listesini döndürür."),
    ("SEQUENCE", "SIRALI(satırlar; [sütunlar]; [başlangıç]; [adım])", "Ardışık sayılardan oluşan bir dizi üretir."),
    ("LEFT", "SOLDAN(metin; [karakter_sayısı])", "Metnin başındaki karakterleri döndürür."),
    ("RIGHT", "SAĞDAN(metin; [karakter_sayısı])", "Metnin sonundaki karakterleri döndürür."),
    ("MID", "PARÇAAL(metin; başlangıç_sayısı; karakter_sayısı)", "Metnin ortasından belirli sayıda karakter döndürür."),
    ("LEN", "UZUNLUK(metin)", "Metindeki karakter sayısını döndürür."),
    ("FIND", "BUL(bul_metin; metin; [başlangıç_sayısı])", "Bir metnin başka bir metin içindeki konumunu bulur (büyük/küçük harf duyarlı)."),
    ("SEARCH", "MBUL(bul_metin; metin; [başlangıç_sayısı])", "Bir metnin başka bir metin içindeki konumunu bulur (büyük/küçük harf duyarsız)."),
    ("REPLACE", "DEĞİŞTİR(eski_metin; başlangıç_sayısı; karakter_sayısı; yeni_metin)", "Metnin bir bölümünü konuma göre değiştirir."),
    ("SUBSTITUTE", "YERİNEKOY(metin; eski_metin; yeni_metin; [yineleme_sayısı])", "Metin içindeki eski metni yeni metinle değiştirir."),
    ("TRIM", "KIRP(metin)", "Metindeki fazladan boşlukları kaldırır."),
    ("UPPER", "BÜYÜKHARF(metin)", "Metni büyük harfe çevirir."),
    ("LOWER", "KÜÇÜKHARF(metin)", "Metni küçük harfe çevirir."),
    ("PROPER", "YAZIM.DÜZENİ(metin)", "Metindeki her sözcüğün ilk harfini büyük harfe çevirir."),
    ("CONCATENATE", "BİRLEŞTİR(metin1; [metin2]; ...)", "Birden çok metin öğesini tek bir metinde birleştirir."),
    ("CONCAT", "ARALIKBİRLEŞTİR(metin1; [metin2]; ...)", "Birden çok aralık veya dizedeki metinleri birleştirir."),
    ("TEXTJOIN", "METİNBİRLEŞTİR(ayırıcı; boşları_yoksay; metin1; [metin2]; ...)", "Metinleri belirtilen ayırıcıyla birleştirir."),
    ("TEXT", "METNEÇEVİR(değer; biçim_metni)", "Bir sayıyı belirli bir sayı biçimiyle metne dönüştürür."),
    ("VALUE", "SAYIYAÇEVİR(metin)", "Sayı Biçimindeki bir metni sayıya dönüştürür."),
    ("ROUND", "YUVARLA(sayı; sayı_rakamlar)", "Bir sayıyı belirtilen basamak sayısına yuvarlar."),
    ("ROUNDUP", "YUKARIYUVARLA(sayı; sayı_rakamlar)", "Bir sayıyı sıfırdan uzağa doğru yukarı yuvarlar."),
    ("ROUNDDOWN", "AŞAĞIYUVARLA(sayı; sayı_rakamlar)", "Bir sayıyı sıfıra doğru aşağı yuvarlar."),
    ("INT", "TAMSAYI(sayı)", "Bir sayıyı en yakın daha küçük tamsayıya aşağı yuvarlar."),
    ("ABS", "MUTLAK(sayı)", "Bir sayının mutlak değerini döndürür."),
    ("SQRT", "KAREKÖK(sayı)", "Bir sayının pozitif karekökünü döndürür."),
    ("POWER", "KUVVET(sayı; üs)", "Bir sayının üssünü döndürür."),
    ("SUMPRODUCT", "TOPLA.ÇARPIM(dizi1; [dizi2]; ...)", "Karşılık gelen dizi bileşenlerinin çarpımlarının toplamını döndürür."),
    ("SUBTOTAL", "ALTTOPLAM(işlev_sayısı; başv1; ...)", "Bir listedeki veya veritabanındaki alt toplamı döndürür."),
    ("RAND", "S_SAYI_ÜRET()", "0 ile 1 arasında rastgele bir sayı döndürür."),
    ("RANDBETWEEN", "RASTGELEARADA(alt; üst)", "Belirttiğiniz sayılar arasında rastgele bir tamsayı döndürür."),
    ("TODAY", "BUGÜN()", "Bugünün tarihini döndürür."),
    ("NOW", "ŞİMDİ()", "Geçerli tarih ve saati döndürür."),
    ("DATE", "TARİH(yıl; ay; gün)", "Belirli bir tarihin seri numarasını döndürür."),
    ("YEAR", "YIL(seri_no)", "Bir tarihin yılını döndürür."),
    ("MONTH", "AY(seri_no)", "Bir tarihin ayını döndürür."),
    ("DAY", "GÜN(seri_no)", "Bir tarihin gününü döndürür."),
    ("EDATE", "SERİTARİH(başlangıç_tarihi; ay_sayısı)", "Başlangıç tarihinden belirtilen ay kadar önceki veya sonraki tarihi döndürür."),
    ("EOMONTH", "SERİAY(başlangıç_tarihi; ay_sayısı)", "Belirtilen ay kadar önceki veya sonraki ayın son gününü döndürür."),
    ("NETWORKDAYS", "TAMİŞGÜNÜ(başlangıç_tarihi; bitiş_tarihi; [tatiller])", "İki tarih arasındaki tam iş günü sayısını döndürür."),
    ("WORKDAY", "İŞGÜNÜ(başlangıç_tarihi; gün_sayısı; [tatiller])", "Belirtilen iş günü sayısından önceki veya sonraki tarihi döndürür."),
    ("PMT", "DEVRESEL_ÖDEME(oran; dönem_sayısı; bd; [gd]; [tür])", "Sabit ödemeler ve sabit faiz oranına dayalı kredi ödemesini hesaplar."),
    ("PV", "BD(oran; dönem_sayısı; devresel_ödeme; [gd]; [tür])", "Bir yatırımın bugünkü değerini döndürür."),
    ("FV", "GD(oran; dönem_sayısı; devresel_ödeme; [bd]; [tür])", "Bir yatırımın gelecekteki değerini döndürür."),
    ("NPV", "NBD(oran; değer1; [değer2]; ...)", "Bir yatırımın net bugünkü değerini döndürür."),
    ("IRR", "İÇ_VERİM_ORANI(değerler; [tahmin])", "Nakit akışları serisi için iç verim oranını döndürür."),
    ("ISBLANK", "EBOŞSA(değer)", "Değer boş bir hücreye başvuruyorsa DOĞRU döndürür."),
    ("ISERROR", "EHATALIYSA(değer)", "Değer herhangi bir hata değeriyse DOĞRU döndürür."),
    ("ISNUMBER", "ESAYIYSA(değer)", "Değer bir sayıysa DOĞRU döndürür."),
    ("ISTEXT", "EMETİNSE(değer)", "Değer bir metinse DOĞRU döndürür."),
    ("TRUE", "DOĞRU()", "DOĞRU mantıksal değerini döndürür."),
    ("FALSE", "YANLIŞ()", "YANLIŞ mantıksal değerini döndürür."),
];

pub(crate) fn signature(name: &str, locale: FormulaLocale) -> Option<String> {
    let canonical = locale.canonical_name(name);
    let table = match locale {
        FormulaLocale::En => None,
        FormulaLocale::Es => Some(SPANISH_HELP),
        FormulaLocale::Tr => Some(TURKISH_HELP),
    };
    if let Some(help) = table
        && let Some((_, signature, _)) = help.iter().find(|(name, ..)| *name == canonical)
    {
        return Some((*signature).into());
    }
    crate::formula_bar::function_signature(canonical).map(|s| {
        if let Some((_head, rest)) = s.split_once('(') {
            let inner = rest.strip_suffix(')').unwrap_or(rest);
            let sep = format!("{} ", locale.list_separator());
            let args: Vec<String> = inner
                .split(',')
                .map(|part| {
                    let trimmed = part.trim();
                    if locale == FormulaLocale::Tr { tr_param_token(trimmed) } else { trimmed.to_string() }
                })
                .collect();
            format!("{}({})", locale.function_name(canonical), args.join(&sep))
        } else {
            s.replace(',', &locale.list_separator().to_string())
        }
    })
}

fn tr_param_token(token: &str) -> String {
    if token.is_empty() || token == "..." {
        return token.to_string();
    }
    let opt = token.starts_with('[') && token.ends_with(']');
    let core = if opt { &token[1..token.len() - 1] } else { token };
    let split_at = core.rfind(|c: char| !c.is_ascii_digit()).map(|i| i + 1).unwrap_or(0);
    let (base, digits) =
        if split_at > 0 && split_at < core.len() && core != "a1" && core != "log10" { (&core[..split_at], &core[split_at..]) } else { (core, "") };
    let tr_base = match base {
        "number" | "num" => "sayı",
        "number_chosen" => "seçilen_sayı",
        "number_f" => "başarısızlık_sayısı",
        "number_s" => "başarı_sayısı",
        "number_sample" => "örnek_sayısı",
        "number_pop" => "anakütle_sayısı",
        "number_times" => "yineleme_sayısı",
        "num_chars" => "karakter_sayısı",
        "num_bytes" => "bayt_sayısı",
        "num_digits" | "decimals" | "places" => "basamak_sayısı",
        "value" => "değer",
        "values" => "değerler",
        "value_if_true" => "eğer_doğruysa_değer",
        "value_if_false" => "eğer_yanlışsa_değer",
        "value_if_error" => "eğer_hataysa_değer",
        "value_if_na" => "eğer_yoksa_değer",
        "text" => "metin",
        "old_text" => "eski_metin",
        "new_text" => "yeni_metin",
        "find_text" => "bul_metin",
        "within_text" => "içinde_metin",
        "date_text" => "tarih_metni",
        "time_text" => "zaman_metni",
        "format_text" | "format" => "biçim",
        "sheet_text" => "sayfa_metni",
        "type_text" => "tür_metni",
        "array" | "data_array" => "dizi",
        "array_x" => "dizi_x",
        "array_y" => "dizi_y",
        "bins_array" => "aralık_dizisi",
        "lookup_value" => "aranan_değer",
        "lookup_array" => "aranan_dizi",
        "return_array" => "döndürülen_dizi",
        "table_array" => "tablo_dizisi",
        "filter_array" | "include" => "ekle",
        "by_array" => "ölçüt_dizisi",
        "range" => "aralık",
        "sum_range" => "toplam_aralığı",
        "average_range" => "ortalama_aralığı",
        "criteria_range" => "ölçüt_aralığı",
        "max_range" => "mak_aralığı",
        "min_range" => "min_aralığı",
        "actual_range" => "gerçek_aralık",
        "expected_range" => "beklenen_aralık",
        "prob_range" => "olasılık_aralığı",
        "x_range" => "x_aralığı",
        "criteria" => "ölçüt",
        "reference" | "ref" => "başvuru",
        "logical" => "mantıksal",
        "logical_test" | "test" => "mantıksal_sınama",
        "expression" => "ifade",
        "result" => "sonuç",
        "default" | "default_or_value" => "varsayılan",
        "row_num" | "row_index_num" => "satır_indis_sayısı",
        "column_num" | "col_index_num" | "col_num" => "sütun_indis_sayısı",
        "area_num" => "alan_sayısı",
        "index_num" => "indis_sayısı",
        "rows" => "satırlar",
        "cols" | "columns" => "sütunlar",
        "height" => "yükseklik",
        "width" => "genişlik",
        "start_num" | "start" => "başlangıç",
        "step" => "adım",
        "rate" => "oran",
        "nper" => "dönem_sayısı",
        "pmt" => "devresel_ödeme",
        "pv" => "bd",
        "fv" => "gd",
        "type" => "tür",
        "guess" => "tahmin",
        "per" | "period" => "dönem",
        "start_period" => "başlangıç_dönemi",
        "end_period" => "bitiş_dönemi",
        "first_period" => "ilk_dönem",
        "cost" => "maliyet",
        "salvage" => "hurda",
        "life" => "ömür",
        "factor" => "çarpan",
        "settlement" => "teslim",
        "maturity" => "vade",
        "issue" => "ihraç",
        "first_interest" => "ilk_faiz",
        "last_interest" => "son_faiz",
        "first_coupon" => "ilk_kupon",
        "coupon" => "kupon",
        "yld" => "getiri",
        "redemption" => "itfa",
        "par" => "nominal_değer",
        "frequency" => "sıklık",
        "basis" => "temel",
        "investment" => "yatırım",
        "discount" => "iskonto",
        "pr" => "fiyat",
        "effect_rate" => "etkin_oran",
        "nominal_rate" => "nominal_oran",
        "npery" => "yıllık_dönem",
        "date" => "tarih",
        "start_date" => "başlangıç_tarihi",
        "end_date" => "bitiş_tarihi",
        "target_date" => "hedef_tarih",
        "date_purchased" => "alım_tarihi",
        "holidays" => "tatiller",
        "weekend" => "hafta_sonu",
        "year" => "yıl",
        "month" | "months" => "ay",
        "day" | "days" => "gün",
        "hour" => "saat",
        "minute" => "dakika",
        "second" => "saniye",
        "serial_number" => "seri_no",
        "return_type" => "döndür_türü",
        "return_mode" => "döndür_modu",
        "match_mode" => "eşleştirme_modu",
        "search_mode" => "arama_modu",
        "range_lookup" => "aralık_bak",
        "sort_index" => "sıralama_indisi",
        "sort_order" => "sıralama_düzeni",
        "by_col" | "scan_by_column" => "sütuna_göre",
        "exactly_once" => "yalnızca_bir_kez",
        "if_empty" => "boşsa",
        "if_not_found" => "bulunamıyorsa",
        "pad_with" => "doldur",
        "delimiter" => "ayırıcı",
        "col_delimiter" => "sütun_ayırıcı",
        "row_delimiter" => "satır_ayırıcı",
        "ignore_empty" | "ignore" => "boşları_yoksay",
        "instance_num" | "occurrence" => "yineleme_sayısı",
        "case_sensitivity" => "büyük_küçük_harf",
        "probability" | "probability_s" => "olasılık",
        "deg_freedom" => "serbestlik_derecesi",
        "tails" => "kuyruklar",
        "cumulative" => "kümülatif",
        "alpha" => "alfa",
        "beta" => "beta",
        "mean" => "ortalama",
        "standard_dev" | "sigma" => "standart_sapma",
        "significance" => "anlamlılık",
        "multiple" => "kat",
        "trials" => "denemeler",
        "sample_s" => "örnek_başarı",
        "population_s" => "anakütle_başarı",
        "known_ys" => "bilinen_y'ler",
        "known_xs" => "bilinen_x'ler",
        "new_xs" => "yeni_x'ler",
        "const" => "sabit",
        "stats" => "istatistikler",
        "database" => "veritabanı",
        "field" => "alan",
        "row_fields" => "satır_alanları",
        "col_fields" => "sütun_alanları",
        "function" => "işlev",
        "function_num" => "işlev_sayısı",
        "options" => "seçenekler",
        "inumber" => "sanal_sayı",
        "real_num" => "gerçek_sayı",
        "i_num" => "sanal_katsayı",
        "suffix" => "sonek",
        "from_unit" => "başlangıç_birimi",
        "to_unit" => "hedef_birim",
        "radix" | "base" => "taban",
        "min_length" => "en_az_uzunluk",
        "numerator" => "pay",
        "denominator" => "payda",
        "divisor" => "bölen",
        "power" => "üs",
        "angle" => "açı",
        "bottom" => "alt",
        "top" => "üst",
        "order" => "düzen",
        "quart" => "dörttebirlik",
        "size" => "boyut",
        "shift_amount" => "kaydırma_miktarı",
        "link_location" => "bağlantı_konumu",
        "friendly_name" => "görünen_ad",
        "info_type" => "bilgi_türü",
        "error_val" => "hata_değeri",
        "timeline" => "zaman_çizelgesi",
        "seasonality" => "mevsimsellik",
        "data_completion" => "veri_tamamlama",
        "aggregation" => "toplama",
        "confidence_level" => "güven_düzeyi",
        "statistic_type" => "istatistik_türü",
        "pattern" => "desen",
        "replacement" => "değiştirilen",
        "connection" => "bağlantı",
        "member_expression" => "üye_ifadesi",
        "set_expression" => "küme_ifadesi",
        "kpi_name" => "kpi_adı",
        "kpi_property" => "kpi_özelliği",
        "property" => "özellik",
        "caption" => "başlık",
        "rank" => "sıra",
        _ => base,
    };
    if opt { format!("[{tr_base}{digits}]") } else { format!("{tr_base}{digits}") }
}

pub(crate) fn description(name: &str, locale: FormulaLocale) -> Option<String> {
    let canonical = locale.canonical_name(name);
    let table = match locale {
        FormulaLocale::En => None,
        FormulaLocale::Es => Some(SPANISH_HELP),
        FormulaLocale::Tr => Some(TURKISH_HELP),
    };
    if let Some(help) = table
        && let Some((_, _, description)) = help.iter().find(|(name, ..)| *name == canonical)
    {
        return Some((*description).into());
    }
    crate::formula_bar::function_description(canonical)
        .map(|d| if locale == FormulaLocale::Tr { crate::i18n::Language::Tr.tr(&d).to_string() } else { d })
}

/// Avoid suggesting an alias that would call a workbook/table name instead of the function.
pub(crate) fn completion_name<'a>(name: &'a str, locale: FormulaLocale, wb: &Workbook, sheet: usize) -> &'a str {
    let alias = locale.function_name(name);
    if wb.name(alias, sheet).is_some() || wb.table(alias).is_some() { name } else { alias }
}

/// Engine-generated edit templates use canonical syntax. The only incomplete template is
/// the bare function opening emitted by Insert Function; typed user input never enters here.
pub(crate) fn display_template(text: &str, locale: FormulaLocale, wb: &Workbook, sheet: usize) -> String {
    if let Some(name) = text.strip_prefix('=').and_then(|s| s.strip_suffix('('))
        && !name.is_empty()
        && name.chars().all(|c| c.is_alphanumeric() || matches!(c, '_' | '.'))
    {
        return format!("={}(", completion_name(name, locale, wb, sheet));
    }
    display(text, locale, wb, sheet)
}
