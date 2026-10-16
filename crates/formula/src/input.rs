//! Localized aliases accepted at user-entry boundaries; file syntax stays English.

use crate::{Expr, ParseError, parser};

/// Formula-entry syntax selected by the interface language. Other interface languages
/// currently use the canonical English function table until a translation table is supplied.
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
pub enum FormulaLocale {
    #[default]
    En,
    Es,
    Tr,
}

const SPANISH: &[(&str, &str)] = &[
    ("SUM", "SUMA"),
    ("AVERAGE", "PROMEDIO"),
    ("IF", "SI"),
    ("COUNT", "CONTAR"),
    ("SUMIF", "SUMAR.SI"),
    ("COUNTIF", "CONTAR.SI"),
    ("VLOOKUP", "BUSCARV"),
    ("TODAY", "HOY"),
    ("TRUE", "VERDADERO"),
    ("FALSE", "FALSO"),
];

pub const TURKISH: &[(&str, &str)] = &[
    // Math & Trig
    ("SUM", "TOPLA"),
    ("SUMIF", "ETOPLA"),
    ("SUMIFS", "ÇOKETOPLA"),
    ("SUMPRODUCT", "TOPLA.ÇARPIM"),
    ("SUMSQ", "TOP.KARE"),
    ("PRODUCT", "ÇARPIM"),
    ("QUOTIENT", "BÖLÜM"),
    ("ABS", "MUTLAK"),
    ("SIGN", "İŞARET"),
    ("INT", "TAMSAYI"),
    ("TRUNC", "NSAT"),
    ("ROUND", "YUVARLA"),
    ("ROUNDUP", "YUKARIYUVARLA"),
    ("ROUNDDOWN", "AŞAĞIYUVARLA"),
    ("MROUND", "KYUVARLA"),
    ("CEILING", "TAVANAYUVARLA"),
    ("CEILING.MATH", "TAVANAYUVARLA.MATEMATİK"),
    ("FLOOR", "TABANAYUVARLA"),
    ("FLOOR.MATH", "TABANAYUVARLA.MATEMATİK"),
    ("EVEN", "ÇİFT"),
    ("ODD", "TEK"),
    ("SQRT", "KAREKÖK"),
    ("POWER", "KUVVET"),
    ("EXP", "ÜS"),
    ("PI", "Pİ"),
    ("DEGREES", "DERECE"),
    ("RADIANS", "RADYAN"),
    ("SIN", "SİN"),
    ("ASIN", "ASİN"),
    ("FACT", "FAKTÖRİYEL"),
    ("COMBIN", "KOMBİNASYON"),
    ("PERMUT", "PERMÜTASYON"),
    ("GCD", "OBEB"),
    ("LCM", "OKEK"),
    ("RAND", "S_SAYI_ÜRET"),
    ("RANDBETWEEN", "RASTGELEARADA"),
    ("RANDARRAY", "RASTGELEDİZİ"),
    ("SUBTOTAL", "ALTTOPLAM"),
    ("AGGREGATE", "TOPLAMA"),
    ("ROMAN", "ROMEN"),
    ("ARABIC", "ARAP"),
    ("BASE", "TABAN"),
    ("DECIMAL", "ONDALIK"),
    ("MMULT", "DÇARP"),
    ("MDETERM", "DDETERMİNANT"),
    ("MINVERSE", "DTERS"),
    // Statistical
    ("AVERAGE", "ORTALAMA"),
    ("AVERAGEA", "ORTALAMAA"),
    ("AVERAGEIF", "EĞERORTALAMA"),
    ("AVERAGEIFS", "ÇOKEĞERORTALAMA"),
    ("COUNT", "BAĞ_DEĞ_SAY"),
    ("COUNTA", "BAĞ_DEĞ_DOLU_SAY"),
    ("COUNTBLANK", "BOŞLUK_SAY"),
    ("COUNTIF", "EĞERSAY"),
    ("COUNTIFS", "ÇOKEĞERSAY"),
    ("MIN", "MİN"),
    ("MINA", "MİNA"),
    ("MINIFS", "ÇOKEĞERMİN"),
    ("MAX", "MAK"),
    ("MAXA", "MAKA"),
    ("MAXIFS", "ÇOKEĞERMAK"),
    ("MEDIAN", "ORTANCA"),
    ("MODE.SNGL", "ENÇOK_OLAN.TEK"),
    ("MODE.MULT", "ENÇOK_OLAN.ÇOK"),
    ("MODE", "ENÇOK_OLAN"),
    ("LARGE", "BÜYÜK"),
    ("SMALL", "KÜÇÜK"),
    ("RANK.EQ", "RANK.EŞİT"),
    ("RANK.AVG", "RANK.ORT"),
    ("PERCENTILE.INC", "YÜZDEBİRLİK.DAHİL"),
    ("PERCENTILE.EXC", "YÜZDEBİRLİK.HARİÇ"),
    ("PERCENTILE", "YÜZDEBİRLİK"),
    ("QUARTILE.INC", "DÖRTTEBİRLİK.DAHİL"),
    ("QUARTILE.EXC", "DÖRTTEBİRLİK.HARİÇ"),
    ("QUARTILE", "DÖRTTEBİRLİK"),
    ("STDEV.S", "STDSAPMA.S"),
    ("STDEV.P", "STDSAPMA.P"),
    ("STDEV", "STDSAPMA"),
    ("STDEVP", "STDSAPMAS"),
    ("CORREL", "KORELASYON"),
    ("COVARIANCE.P", "KOVARYANS.P"),
    ("COVARIANCE.S", "KOVARYANS.S"),
    ("SLOPE", "EĞİM"),
    ("INTERCEPT", "KESMENOKTASI"),
    ("FORECAST.LINEAR", "TAHMİN.DOĞRUSAL"),
    ("FORECAST", "TAHMİN"),
    ("FREQUENCY", "SIKLIK"),
    ("GEOMEAN", "GEOORT"),
    ("HARMEAN", "HARORT"),
    ("TRIMMEAN", "KIRPORTALAMA"),
    ("AVEDEV", "ORTSAPMA"),
    ("DEVSQ", "SAPKARE"),
    // Logical
    ("IF", "EĞER"),
    ("IFS", "ÇOKEĞER"),
    ("IFERROR", "EĞERHATA"),
    ("IFNA", "EĞERYOKSA"),
    ("AND", "VE"),
    ("OR", "YADA"),
    ("NOT", "DEĞİL"),
    ("XOR", "ÖZELVEYA"),
    ("TRUE", "DOĞRU"),
    ("FALSE", "YANLIŞ"),
    ("SWITCH", "İLİŞKİLENDİR"),
    // Lookup & Reference & Dynamic Arrays
    ("VLOOKUP", "DÜŞEYARA"),
    ("HLOOKUP", "YATAYARA"),
    ("XLOOKUP", "ÇAPRAZARA"),
    ("LOOKUP", "ARA"),
    ("INDEX", "İNDİS"),
    ("MATCH", "KAÇINCI"),
    ("XMATCH", "ÇAPRAZKAÇINCI"),
    ("CHOOSE", "ELEMAN"),
    ("OFFSET", "KAYDIR"),
    ("INDIRECT", "DOLAYLI"),
    ("ROW", "SATIR"),
    ("COLUMN", "SÜTUN"),
    ("ROWS", "SATIRSAY"),
    ("COLUMNS", "SÜTUNSAY"),
    ("ADDRESS", "ADRES"),
    ("AREAS", "ALANLAR"),
    ("TRANSPOSE", "DEVRİK_DÖNÜŞÜM"),
    ("FORMULATEXT", "FORMÜLMETNİ"),
    ("HYPERLINK", "KÖPRÜ"),
    ("FILTER", "FİLTRELE"),
    ("SORT", "SIRALA"),
    ("SORTBY", "SIRALAÖLÇÜT"),
    ("UNIQUE", "BENZERSİZ"),
    ("SEQUENCE", "SIRALI"),
    ("TAKE", "AL"),
    ("DROP", "BIRAK"),
    ("CHOOSECOLS", "SÜTUNSEÇ"),
    ("CHOOSEROWS", "SATIRSEÇ"),
    ("HSTACK", "YATAYYIĞ"),
    ("VSTACK", "DÜŞEYYIĞ"),
    ("TOCOL", "SÜTUNA"),
    ("TOROW", "SATIRA"),
    ("WRAPCOLS", "SÜTUNSAR"),
    ("WRAPROWS", "SATIRSAR"),
    ("EXPAND", "GENİŞLET"),
    // Text
    ("LEFT", "SOLDAN"),
    ("RIGHT", "SAĞDAN"),
    ("MID", "PARÇAAL"),
    ("LEN", "UZUNLUK"),
    ("FIND", "BUL"),
    ("SEARCH", "MBUL"),
    ("REPLACE", "DEĞİŞTİR"),
    ("SUBSTITUTE", "YERİNEKOY"),
    ("UPPER", "BÜYÜKHARF"),
    ("LOWER", "KÜÇÜKHARF"),
    ("PROPER", "YAZIM.DÜZENİ"),
    ("TRIM", "KIRP"),
    ("CLEAN", "TEMİZ"),
    ("CONCATENATE", "BİRLEŞTİR"),
    ("CONCAT", "ARALIKBİRLEŞTİR"),
    ("TEXTJOIN", "METİNBİRLEŞTİR"),
    ("TEXT", "METNEÇEVİR"),
    ("VALUE", "SAYIYAÇEVİR"),
    ("NUMBERVALUE", "SAYIDEĞERİ"),
    ("FIXED", "SAYIDÜZENLE"),
    ("DOLLAR", "LİRA"),
    ("EXACT", "ÖZDEŞ"),
    ("REPT", "YİNELE"),
    ("CHAR", "DAMGA"),
    ("CODE", "KOD"),
    ("UNICHAR", "UNİDAMGA"),
    ("TEXTBEFORE", "ÖNCEKİMETİN"),
    ("TEXTAFTER", "SONRAKİMETİN"),
    ("TEXTSPLIT", "METİNBÖL"),
    // Date & Time
    ("TODAY", "BUGÜN"),
    ("NOW", "ŞİMDİ"),
    ("DATE", "TARİH"),
    ("DATEVALUE", "TARİHSAYISI"),
    ("TIME", "ZAMAN"),
    ("TIMEVALUE", "ZAMANSAYISI"),
    ("YEAR", "YIL"),
    ("MONTH", "AY"),
    ("DAY", "GÜN"),
    ("DAYS", "GÜNSAY"),
    ("DAYS360", "GÜN360"),
    ("HOUR", "SAAT"),
    ("MINUTE", "DAKİKA"),
    ("SECOND", "SANİYE"),
    ("WEEKDAY", "HAFTANINGÜNÜ"),
    ("WEEKNUM", "HAFTASAY"),
    ("ISOWEEKNUM", "ISOHAFTASAY"),
    ("EDATE", "SERİTARİH"),
    ("EOMONTH", "SERİAY"),
    ("WORKDAY", "İŞGÜNÜ"),
    ("WORKDAY.INTL", "İŞGÜNÜ.ULUSL"),
    ("NETWORKDAYS", "TAMİŞGÜNÜ"),
    ("NETWORKDAYS.INTL", "TAMİŞGÜNÜ.ULUSL"),
    ("DATEDIF", "ETARİHLİ"),
    ("YEARFRAC", "YILORAN"),
    // Financial
    ("PMT", "DEVRESEL_ÖDEME"),
    ("IPMT", "FAİZ_ÖDEMESİ"),
    ("PPMT", "ANA_PARA_ÖDEMESİ"),
    ("PV", "BD"),
    ("FV", "GD"),
    ("NPV", "NBD"),
    ("XNPV", "ANBD"),
    ("IRR", "İÇ_VERİM_ORANI"),
    ("XIRR", "AİÇVERİMORANI"),
    ("MIRR", "D_İÇ_VERİM_ORANI"),
    ("RATE", "FAİZ_ORANI"),
    ("NPER", "TAKSİT_SAYISI"),
    ("EFFECT", "ETKİN"),
    ("NOMINAL", "NOMİNAL"),
    ("SLN", "DA"),
    ("SYD", "YAT"),
    ("DB", "AZALANBAKİYE"),
    ("DDB", "ÇİFTAZALANBAKİYE"),
    // Information & Database
    ("ISBLANK", "EBOŞSA"),
    ("ISERROR", "EHATALIYSA"),
    ("ISERR", "EHATA"),
    ("ISNA", "EYOKSA"),
    ("ISNUMBER", "ESAYIYSA"),
    ("ISTEXT", "EMETİNSE"),
    ("ISNONTEXT", "EMETİNDEĞİLSE"),
    ("ISLOGICAL", "EMANTIKSALSA"),
    ("ISREF", "EREFERANSSA"),
    ("ISFORMULA", "EFORMÜLSE"),
    ("ISEVEN", "ÇİFTMİ"),
    ("ISODD", "TEKMİ"),
    ("N", "S"),
    ("NA", "YOKSAY"),
    ("TYPE", "TÜR"),
    ("ERROR.TYPE", "HATA.TİPİ"),
    ("CELL", "HÜCRE"),
    ("SHEET", "SAYFA"),
    ("SHEETS", "SAYFALAR"),
    ("DSUM", "VSEÇTOPLA"),
    ("DAVERAGE", "VSEÇORT"),
    ("DCOUNT", "VSEÇSAY"),
    ("DCOUNTA", "VSEÇSAYDOLU"),
    ("DMIN", "VSEÇMİN"),
    ("DMAX", "VSEÇMAK"),
    ("DGET", "VSEÇAL"),
    ("DPRODUCT", "VSEÇÇARP"),
    ("DSTDEV", "VSEÇSTDSAPMA"),
    ("DSTDEVP", "VSEÇSTDSAPMAS"),
    ("DVAR", "VSEÇVAR"),
    ("DVARP", "VSEÇVARS"),
    ("INFO", "BİLGİ"),
    ("ISOMITTED", "ATLANDIYSA"),
    // Additional Financial
    ("ACCRINT", "GERÇEKFAİZ"),
    ("ACCRINTM", "VADEFAİZ"),
    ("COUPDAYBS", "KUPONGÜNBD"),
    ("COUPDAYS", "KUPONGÜN"),
    ("COUPDAYSNC", "KUPONGÜNSD"),
    ("COUPNCD", "KUPONGÜNSKT"),
    ("COUPNUM", "KUPONSAYI"),
    ("COUPPCD", "KUPONGÜNÖKT"),
    ("CUMIPMT", "TOPFAİZÖDE"),
    ("CUMPRINC", "TOPANAPARA"),
    ("DISC", "İNDİRİM"),
    ("DOLLARDE", "LİRAOND"),
    ("DOLLARFR", "LİRAKES"),
    ("DURATION", "SÜRE"),
    ("FVSCHEDULE", "GDPROGRAM"),
    ("INTRATE", "FAİZORANI"),
    ("MDURATION", "MSÜRE"),
    ("ODDFPRICE", "TEKFİYAT"),
    ("ODDFYIELD", "TEKÖDEME"),
    ("ODDLPRICE", "ÇİFTFİYAT"),
    ("ODDLYIELD", "ÇİFTÖDEME"),
    ("PDURATION", "PSÜRE"),
    ("PRICE", "FİYAT"),
    ("PRICEDISC", "İNDİRİMLİFİYAT"),
    ("PRICEMAT", "VADEFİYAT"),
    ("RECEIVED", "GETİRİ"),
    ("RRI", "GERÇEKGETİRİ"),
    ("TBILLEQ", "HTEŞDEĞER"),
    ("TBILLPRICE", "HTFİYAT"),
    ("TBILLYIELD", "HTÖDEME"),
    ("VDB", "DAB"),
    ("YIELD", "ÖDEME"),
    ("YIELDDISC", "İNDİRİMLİÖDEME"),
    ("YIELDMAT", "VADEÖDEME"),
    // Additional Math & Trig
    ("ASINH", "ASİNH"),
    ("CEILING.PRECISE", "TAVANAYUVARLA.DUYARLI"),
    ("ISO.CEILING", "ISO.TAVANAYUVARLA"),
    ("FLOOR.PRECISE", "TABANAYUVARLA.DUYARLI"),
    ("COMBINA", "KOMBİNASYONA"),
    ("CSC", "KOSEK"),
    ("CSCH", "KOSEKH"),
    ("FACTDOUBLE", "ÇİFTFAKTÖR"),
    ("MULTINOMIAL", "ÇOKTERİMLİ"),
    ("MUNIT", "BİRİMMATRİS"),
    ("SEC", "SEK"),
    ("SECH", "SEKH"),
    ("SERIESSUM", "SERİTOPLA"),
    ("SINH", "SİNH"),
    ("SQRTPI", "KAREKÖKPİ"),
    ("SUMX2MY2", "TOPX2EY2"),
    ("SUMX2PY2", "TOPX2AY2"),
    ("SUMXMY2", "TOPXEY2"),
    // Additional Statistical
    ("BETA.DIST", "BETA.DAĞ"),
    ("BETA.INV", "BETA.TERS"),
    ("BETADIST", "BETADAĞ"),
    ("BETAINV", "BETATERS"),
    ("BINOM.DIST", "BİNOM.DAĞ"),
    ("BINOM.DIST.RANGE", "BİNOM.DAĞ.ARALIK"),
    ("BINOM.INV", "BİNOM.TERS"),
    ("BINOMDIST", "BİNOMDAĞ"),
    ("CHIDIST", "KİKAREDAĞ"),
    ("CHIINV", "KİKARETERS"),
    ("CHISQ.DIST", "KİKARE.DAĞ"),
    ("CHISQ.DIST.RT", "KİKARE.DAĞ.SAĞK"),
    ("CHISQ.INV", "KİKARE.TERS"),
    ("CHISQ.INV.RT", "KİKARE.TERS.SAĞK"),
    ("CHISQ.TEST", "KİKARE.TEST"),
    ("CHITEST", "KİKARETEST"),
    ("CONFIDENCE", "GÜVENİLİRLİK"),
    ("CONFIDENCE.NORM", "GÜVENİLİRLİK.NORM"),
    ("CONFIDENCE.T", "GÜVENİLİRLİK.T"),
    ("COVAR", "KOVARYANS"),
    ("CRITBINOM", "KRİTİKBİNOM"),
    ("EXPON.DIST", "ÜSTEL.DAĞ"),
    ("EXPONDIST", "ÜSTELDAĞ"),
    ("F.DIST", "F.DAĞ"),
    ("F.DIST.RT", "F.DAĞ.SAĞK"),
    ("F.INV", "F.TERS"),
    ("F.INV.RT", "F.TERS.SAĞK"),
    ("F.TEST", "F.TEST"),
    ("FDIST", "FDAĞ"),
    ("FINV", "FTERS"),
    ("FISHERINV", "FISHERTERS"),
    ("FORECAST.ETS", "TAHMİN.ETS"),
    ("FORECAST.ETS.CONFINT", "TAHMİN.ETS.GÜVENARALIĞI"),
    ("FORECAST.ETS.SEASONALITY", "TAHMİN.ETS.MEVSİMSELLİK"),
    ("FORECAST.ETS.STAT", "TAHMİN.ETS.İSTATİSTİK"),
    ("GAMMA", "GAMA"),
    ("GAMMA.DIST", "GAMA.DAĞ"),
    ("GAMMA.INV", "GAMA.TERS"),
    ("GAMMADIST", "GAMADAĞ"),
    ("GAMMAINV", "GAMATERS"),
    ("GAMMALN", "GAMALN"),
    ("GAMMALN.PRECISE", "GAMALN.DUYARLI"),
    ("GROWTH", "BÜYÜME"),
    ("HYPGEOM.DIST", "HİPERGEOM.DAĞ"),
    ("HYPGEOMDIST", "HİPERGEOMDAĞ"),
    ("KURT", "BASIKLIK"),
    ("LINEST", "DOTAHMİN"),
    ("LOGEST", "LOGTAHMİN"),
    ("LOGINV", "LOGTERS"),
    ("LOGNORM.DIST", "LOGNORM.DAĞ"),
    ("LOGNORM.INV", "LOGNORM.TERS"),
    ("LOGNORMDIST", "LOGNORMDAĞ"),
    ("NEGBINOM.DIST", "NEGBİNOM.DAĞ"),
    ("NEGBINOMDIST", "NEGBİNOMDAĞ"),
    ("NORM.DIST", "NORM.DAĞ"),
    ("NORM.INV", "NORM.TERS"),
    ("NORM.S.DIST", "NORM.S.DAĞ"),
    ("NORM.S.INV", "NORM.S.TERS"),
    ("NORMDIST", "NORMDAĞ"),
    ("NORMINV", "NORMTERS"),
    ("NORMSDIST", "NORMSDAĞ"),
    ("NORMSINV", "NORMSTERS"),
    ("PERCENTRANK", "YÜZDERANK"),
    ("PERCENTRANK.EXC", "YÜZDERANK.HARİÇ"),
    ("PERCENTRANK.INC", "YÜZDERANK.DAHİL"),
    ("PERMUTATIONA", "PERMÜTASYONA"),
    ("POISSON.DIST", "POISSON.DAĞ"),
    ("PROB", "OLASILIK"),
    ("RSQ", "RKARE"),
    ("SKEW", "ÇARPIKLIK"),
    ("SKEW.P", "ÇARPIKLIK.P"),
    ("STANDARDIZE", "STANDARTLAŞTIRMA"),
    ("STDEVA", "STDSAPMAA"),
    ("STDEVPA", "STDSAPMAPA"),
    ("STEYX", "STHYX"),
    ("T.DIST", "T.DAĞ"),
    ("T.DIST.2T", "T.DAĞ.2K"),
    ("T.DIST.RT", "T.DAĞ.SAĞK"),
    ("T.INV", "T.TERS"),
    ("T.INV.2T", "T.TERS.2K"),
    ("T.TEST", "T.TEST"),
    ("TDIST", "TDAĞ"),
    ("TINV", "TTERS"),
    ("TREND", "EĞİLİM"),
    ("VARP", "VARS"),
    ("VARPA", "VARSA"),
    ("WEIBULL.DIST", "WEIBULL.DAĞ"),
    // Engineering
    ("BIN2DEC", "İKİLİDENONLUĞA"),
    ("BIN2HEX", "İKİLİDENONALTILIĞA"),
    ("BIN2OCT", "İKİLİDENSEKİZLİĞE"),
    ("DEC2BIN", "ONLUKTANİKİLİYE"),
    ("DEC2HEX", "ONLUKTANONALTILIĞA"),
    ("DEC2OCT", "ONLUKTANSEKİZLİĞE"),
    ("HEX2BIN", "ONALTILIKTANİKİLİYE"),
    ("HEX2DEC", "ONALTILIKTANONLUĞA"),
    ("HEX2OCT", "ONALTILIKTANSEKİZLİĞE"),
    ("OCT2BIN", "SEKİZLİKTENİKİLİYE"),
    ("OCT2DEC", "SEKİZLİKTENONLUĞA"),
    ("OCT2HEX", "SEKİZLİKTENONALTILIĞA"),
    ("BITAND", "BİTVE"),
    ("BITOR", "BİTVEYA"),
    ("BITXOR", "BİTÖZELVEYA"),
    ("BITLSHIFT", "BİTSOLAKAYDIR"),
    ("BITRSHIFT", "BİTSAĞAKAYDIR"),
    ("COMPLEX", "KARMAŞIK"),
    ("CONVERT", "ÇEVİR"),
    ("ERF", "HATAİŞLEV"),
    ("ERF.PRECISE", "HATAİŞLEV.DUYARLI"),
    ("ERFC", "TÜMHATAİŞLEV"),
    ("ERFC.PRECISE", "TÜMHATAİŞLEV.DUYARLI"),
    ("GESTEP", "BESBÜYÜK"),
    ("IMABS", "SANMUTLAK"),
    ("IMAGINARY", "SANAL"),
    ("IMARGUMENT", "SANBAĞ_DEĞİŞKEN"),
    ("IMCONJUGATE", "SANEŞLENİK"),
    ("IMCOS", "SANCOS"),
    ("IMCOSH", "SANCOSH"),
    ("IMCOT", "SANCOT"),
    ("IMCSC", "SANKOSEK"),
    ("IMCSCH", "SANKOSEKH"),
    ("IMDIV", "SANBÖL"),
    ("IMEXP", "SANÜS"),
    ("IMLN", "SANLN"),
    ("IMLOG10", "SANLOG10"),
    ("IMLOG2", "SANLOG2"),
    ("IMPOWER", "SANKUVVET"),
    ("IMPRODUCT", "SANÇARP"),
    ("IMREAL", "SANGERÇEK"),
    ("IMSEC", "SANSEK"),
    ("IMSECH", "SANSEKH"),
    ("IMSIN", "SANSİN"),
    ("IMSINH", "SANSİNH"),
    ("IMSQRT", "SANKAREKÖK"),
    ("IMSUB", "SANÇIKAR"),
    ("IMSUM", "SANTOPLA"),
    ("IMTAN", "SANTAN"),
    // Additional Text, Dynamic Array, Web & Cube
    ("ARRAYTOTEXT", "DİZİDENMETNE"),
    ("VALUETOTEXT", "DEĞERDENMETNE"),
    ("BAHTTEXT", "BAHTMETİN"),
    ("FINDB", "BULB"),
    ("SEARCHB", "MBULB"),
    ("LEFTB", "SOLDANB"),
    ("RIGHTB", "SAĞDANB"),
    ("MIDB", "PARÇAALB"),
    ("LENB", "UZUNLUKB"),
    ("REPLACEB", "DEĞİŞTİRB"),
    ("UNICODE", "UNİKOD"),
    ("REGEXEXTRACT", "DÜZİFADEÇIKAR"),
    ("REGEXREPLACE", "DÜZİFADEDEĞİŞTİR"),
    ("REGEXTEST", "DÜZİFADETEST"),
    ("BYCOL", "SÜTUNAGÖRE"),
    ("BYROW", "SATIRAGÖRE"),
    ("MAKEARRAY", "DİZİOLUŞTUR"),
    ("REDUCE", "İNDİRGE"),
    ("SCAN", "TARA"),
    ("GROUPBY", "GRUPLA"),
    ("PIVOTBY", "ÖZETLE"),
    ("PERCENTOF", "YÜZDESİ"),
    ("TRIMRANGE", "ARALIKKIRP"),
    ("ENCODEURL", "URLKODLA"),
    ("FILTERXML", "XMLFİLTRELE"),
    ("WEBSERVICE", "WEBHİZMETİ"),
    ("CUBEKPIMEMBER", "KÜPKPIÜYESİ"),
    ("CUBEMEMBER", "KÜPÜYESİ"),
    ("CUBEMEMBERPROPERTY", "KÜPÜYEÖZELLİĞİ"),
    ("CUBERANKEDMEMBER", "KÜPSIRALIÜYESİ"),
    ("CUBESET", "KÜPKÜMESİ"),
    ("CUBESETCOUNT", "KÜPKÜMESAYISI"),
    ("CUBEVALUE", "KÜPDEĞERİ"),
    // Additional statistical & financial aliases
    ("FTEST", "FTESTİ"),
    ("TTEST", "TTESTİ"),
    ("Z.TEST", "Z.TESTİ"),
    ("ZTEST", "ZTESTİ"),
    ("ISPMT", "ISPMT"),
    ("AMORDEGRC", "AMORDEGRC"),
    ("AMORLINC", "AMORLINC"),
    ("VAR", "VAR"),
    ("VARA", "VARA"),
    ("VAR.P", "VAR.P"),
    ("VAR.S", "VAR.S"),
    ("MOD", "MOD"),
    ("RANK", "RANK"),
];

const TURKISH_ALT_ALIASES: &[(&str, &str)] = &[
    ("OR", "VEYA"),
    ("XOR", "ÖZELYADA"),
    ("COUNTBLANK", "BOŞLUKSAY"),
    ("COUNT", "BAĞ.DEĞ.SAY"),
    ("COUNTA", "BAĞ.DEĞ.DOLU.SAY"),
    ("RAND", "S.SAYI.ÜRET"),
    ("RAND", "SSAYIÜRET"),
    ("TRANSPOSE", "DEVRİK.DÖNÜŞÜM"),
    ("AVERAGEIF", "ORTALAMA.EĞER"),
    ("AVERAGEIFS", "ORTALAMA.ÇOKEĞER"),
    ("ISPMT", ".ÖDEME"),
    ("VARP", "VARS"),
];

fn fold_tr_char(c: char) -> char {
    match c {
        'i' | 'İ' | 'ı' | 'I' => 'I',
        'ğ' | 'Ğ' => 'Ğ',
        'ü' | 'Ü' => 'Ü',
        'ş' | 'Ş' => 'Ş',
        'ö' | 'Ö' => 'Ö',
        'ç' | 'Ç' => 'Ç',
        _ => c.to_ascii_uppercase(),
    }
}

pub fn eq_tr_ignore_case(a: &str, b: &str) -> bool {
    a.chars().filter(|&c| c != '\u{0307}').map(fold_tr_char).eq(b.chars().filter(|&c| c != '\u{0307}').map(fold_tr_char))
}

impl FormulaLocale {
    pub fn list_separator(self) -> char {
        if matches!(self, Self::Es | Self::Tr) { ';' } else { ',' }
    }
    pub fn decimal_separator(self) -> char {
        if matches!(self, Self::Es | Self::Tr) { ',' } else { '.' }
    }
    pub fn array_column_separator(self) -> char {
        if matches!(self, Self::Es | Self::Tr) { '\\' } else { ',' }
    }
    pub fn function_name(self, canonical: &str) -> &str {
        match self {
            Self::En => canonical,
            Self::Es => SPANISH.iter().find(|(en, _)| en.eq_ignore_ascii_case(canonical)).map(|(_, es)| *es).unwrap_or(canonical),
            Self::Tr => TURKISH.iter().find(|(en, _)| en.eq_ignore_ascii_case(canonical)).map(|(_, tr)| *tr).unwrap_or(canonical),
        }
    }
    pub fn canonical_name(self, localized: &str) -> &str {
        match self {
            Self::En => localized,
            Self::Es => SPANISH.iter().find(|(_, es)| es.eq_ignore_ascii_case(localized)).map(|(en, _)| *en).unwrap_or(localized),
            Self::Tr => TURKISH
                .iter()
                .chain(TURKISH_ALT_ALIASES.iter())
                .find(|(_, tr)| eq_tr_ignore_case(tr, localized))
                .map(|(en, _)| *en)
                .unwrap_or(localized),
        }
    }
}

fn canonical_name_any(name: &str) -> &str {
    if TURKISH.iter().any(|(en, _)| en.eq_ignore_ascii_case(name)) || SPANISH.iter().any(|(en, _)| en.eq_ignore_ascii_case(name)) {
        return name;
    }
    let tr = FormulaLocale::Tr.canonical_name(name);
    if !std::ptr::eq(tr, name) {
        return tr;
    }
    FormulaLocale::Es.canonical_name(name)
}

fn has_non_en_alias(expr: &Expr) -> bool {
    let mut found = false;
    expr.walk(&mut |e| match e {
        Expr::Call(n, _) if !std::ptr::eq(canonical_name_any(n), n.as_str()) => found = true,
        Expr::Name(n) if boolean_alias(n, FormulaLocale::Tr).is_some() || boolean_alias(n, FormulaLocale::Es).is_some() => found = true,
        _ => {}
    });
    found
}

/// Parses UI input using the selected function table and separators. Existing names and
/// LET/LAMBDA bindings take precedence over aliases. File and API syntax uses [`crate::parse`].
pub fn parse_input(src: &str, locale: FormulaLocale, known_name: impl Fn(&str) -> bool) -> Result<Expr, ParseError> {
    let mut expr = parser::parse_syntax(src, locale)?;
    if locale != FormulaLocale::En {
        normalize(&mut expr, locale, &known_name, &mut Vec::new(), 0, false)?;
    }
    Ok(expr)
}

/// Parses a formula written in canonical English or any supported localized syntax
/// (Turkish/Spanish function names, `;` argument separators, `,` decimal separators),
/// returning an expression normalized to canonical English function names and booleans.
pub fn parse_any(src: &str) -> Result<Expr, ParseError> {
    parse_any_with_known(src, |_| false)
}

/// Like [`parse_any`], preserving workbook defined names and table names reported by `known_name`.
pub fn parse_any_with_known(src: &str, known_name: impl Fn(&str) -> bool) -> Result<Expr, ParseError> {
    match parser::parse(src) {
        Ok(mut expr) => {
            if has_non_en_alias(&expr)
                && src.contains(',')
                && !src.contains('.')
                && let Ok(mut tr_expr) = parser::parse_syntax(src, FormulaLocale::Tr)
            {
                normalize_any(&mut tr_expr, &known_name, &mut Vec::new(), 0)?;
                return Ok(tr_expr);
            }
            normalize_any(&mut expr, &known_name, &mut Vec::new(), 0)?;
            Ok(expr)
        }
        Err(en_err) => {
            let mut expr = parser::parse_syntax(src, FormulaLocale::Tr).map_err(|_| en_err)?;
            normalize_any(&mut expr, &known_name, &mut Vec::new(), 0)?;
            Ok(expr)
        }
    }
}

/// Prints canonical expressions for editing. Name collisions retain the canonical spelling
/// so displaying then committing a formula never changes a defined-name reference.
pub fn print_input(expr: &Expr, locale: FormulaLocale, known_name: impl Fn(&str) -> bool) -> Result<String, ParseError> {
    // Check before cloning/printing: a flat operator chain can produce a deep AST even
    // though the parser itself did not recurse deeply.
    let mut pending = vec![(expr, 0usize)];
    while let Some((node, depth)) = pending.pop() {
        if depth > parser::MAX_DEPTH {
            return Err(ParseError { msg: "formula is nested too deeply".into(), pos: 0 });
        }
        match node {
            Expr::Unary(_, child) | Expr::Paren(child) => pending.push((child, depth + 1)),
            Expr::Binary(_, left, right) => {
                pending.push((left, depth + 1));
                pending.push((right, depth + 1));
            }
            Expr::Call(_, args) => pending.extend(args.iter().map(|e| (e, depth + 1))),
            Expr::Invoke(callee, args) => {
                pending.push((callee, depth + 1));
                pending.extend(args.iter().map(|e| (e, depth + 1)));
            }
            Expr::Array(rows) => pending.extend(rows.iter().flatten().map(|e| (e, depth + 1))),
            _ => {}
        }
    }
    if locale == FormulaLocale::En {
        return Ok(crate::print(expr));
    }
    let mut expr = expr.clone();
    normalize(&mut expr, locale, &known_name, &mut Vec::new(), 0, true)?;
    let canonical = crate::print(&expr);
    let tokens = crate::lexer::tokenize(&canonical).map_err(|e| ParseError { msg: e.msg, pos: e.pos })?;
    let mut out = String::new();
    let mut array_depth = 0usize;
    for token in tokens {
        use crate::lexer::Tok;
        let text = canonical.get(token.start..token.end).unwrap_or("");
        match token.tok {
            Tok::LBrace => {
                array_depth += 1;
                out.push_str(text);
            }
            Tok::RBrace => {
                array_depth = array_depth.saturating_sub(1);
                out.push_str(text);
            }
            Tok::Number(_) => out.push_str(&text.replace('.', &locale.decimal_separator().to_string())),
            Tok::Comma => out.push(if array_depth > 0 { locale.array_column_separator() } else { locale.list_separator() }),
            _ => out.push_str(text),
        }
    }
    Ok(out)
}

pub(crate) fn boolean_alias(name: &str, locale: FormulaLocale) -> Option<bool> {
    match locale {
        FormulaLocale::En => None,
        FormulaLocale::Es => {
            if name.eq_ignore_ascii_case("VERDADERO") {
                Some(true)
            } else if name.eq_ignore_ascii_case("FALSO") {
                Some(false)
            } else {
                None
            }
        }
        FormulaLocale::Tr => {
            if eq_tr_ignore_case(name, "DOĞRU") {
                Some(true)
            } else if eq_tr_ignore_case(name, "YANLIŞ") {
                Some(false)
            } else {
                None
            }
        }
    }
}

fn normalize(
    expr: &mut Expr,
    locale: FormulaLocale,
    known_name: &impl Fn(&str) -> bool,
    bindings: &mut Vec<String>,
    depth: usize,
    display: bool,
) -> Result<(), ParseError> {
    if depth > parser::MAX_DEPTH {
        return Err(ParseError { msg: "formula is nested too deeply".into(), pos: 0 });
    }
    let bound = |name: &str, bindings: &[String]| bindings.iter().rev().any(|n| eq_tr_ignore_case(n, name)) || known_name(name);
    match expr {
        Expr::Bool(value) if display => {
            let alias = match locale {
                FormulaLocale::En => {
                    if *value {
                        "TRUE"
                    } else {
                        "FALSE"
                    }
                }
                FormulaLocale::Es => {
                    if *value {
                        "VERDADERO"
                    } else {
                        "FALSO"
                    }
                }
                FormulaLocale::Tr => {
                    if *value {
                        "DOĞRU"
                    } else {
                        "YANLIŞ"
                    }
                }
            };
            if !bound(alias, bindings) {
                *expr = Expr::Name(alias.into());
            }
        }
        Expr::Name(name) if !display && !bound(name, bindings) => {
            if let Some(value) = boolean_alias(name, locale) {
                *expr = Expr::Bool(value);
            }
        }
        Expr::Call(name, args) if name == "LET" && args.len() >= 3 && !args.len().is_multiple_of(2) => {
            let saved = bindings.len();
            if let Some((body, pairs)) = args.split_last_mut() {
                for pair in pairs.chunks_mut(2) {
                    if let [declaration, value] = pair {
                        // The new name is not in scope in its own value expression.
                        normalize(value, locale, known_name, bindings, depth + 1, display)?;
                        if let Expr::Name(name) = declaration {
                            bindings.push(name.clone());
                        }
                    }
                }
                normalize(body, locale, known_name, bindings, depth + 1, display)?;
            }
            bindings.truncate(saved);
        }
        Expr::Call(name, args) if name == "LAMBDA" => {
            let saved = bindings.len();
            if let Some((body, params)) = args.split_last_mut() {
                for param in params {
                    if let Expr::Name(name) = param {
                        bindings.push(name.clone());
                    }
                }
                normalize(body, locale, known_name, bindings, depth + 1, display)?;
            }
            bindings.truncate(saved);
        }
        Expr::Call(name, args) => {
            let alias = if display { locale.function_name(name) } else { locale.canonical_name(name) };
            if !bound(name, bindings) && (!display || !bound(alias, bindings)) {
                *name = alias.to_string();
            }
            for arg in args {
                normalize(arg, locale, known_name, bindings, depth + 1, display)?;
            }
        }
        Expr::Invoke(callee, args) => {
            normalize(callee, locale, known_name, bindings, depth + 1, display)?;
            for arg in args {
                normalize(arg, locale, known_name, bindings, depth + 1, display)?;
            }
        }
        Expr::Unary(_, child) | Expr::Paren(child) => normalize(child, locale, known_name, bindings, depth + 1, display)?,
        Expr::Binary(_, left, right) => {
            normalize(left, locale, known_name, bindings, depth + 1, display)?;
            normalize(right, locale, known_name, bindings, depth + 1, display)?;
        }
        Expr::Array(rows) if display => {
            for row in rows {
                for child in row {
                    normalize(child, locale, known_name, bindings, depth + 1, display)?;
                }
            }
        }
        // Array elements have already been parsed as literals. Sheet/structured references
        // contain user-defined labels and must not be translated.
        _ => {}
    }
    Ok(())
}

fn normalize_any(expr: &mut Expr, known_name: &impl Fn(&str) -> bool, bindings: &mut Vec<String>, depth: usize) -> Result<(), ParseError> {
    if depth > parser::MAX_DEPTH {
        return Err(ParseError { msg: "formula is nested too deeply".into(), pos: 0 });
    }
    let bound = |name: &str, bindings: &[String]| bindings.iter().rev().any(|n| eq_tr_ignore_case(n, name)) || known_name(name);
    match expr {
        Expr::Name(name) if !bound(name, bindings) => {
            if let Some(value) = boolean_alias(name, FormulaLocale::Tr).or_else(|| boolean_alias(name, FormulaLocale::Es)) {
                *expr = Expr::Bool(value);
            }
        }
        Expr::Call(name, args) if name == "LET" && args.len() >= 3 && !args.len().is_multiple_of(2) => {
            let saved = bindings.len();
            if let Some((body, pairs)) = args.split_last_mut() {
                for pair in pairs.chunks_mut(2) {
                    if let [declaration, value] = pair {
                        normalize_any(value, known_name, bindings, depth + 1)?;
                        if let Expr::Name(name) = declaration {
                            bindings.push(name.clone());
                        }
                    }
                }
                normalize_any(body, known_name, bindings, depth + 1)?;
            }
            bindings.truncate(saved);
        }
        Expr::Call(name, args) if name == "LAMBDA" => {
            let saved = bindings.len();
            if let Some((body, params)) = args.split_last_mut() {
                for param in params {
                    if let Expr::Name(name) = param {
                        bindings.push(name.clone());
                    }
                }
                normalize_any(body, known_name, bindings, depth + 1)?;
            }
            bindings.truncate(saved);
        }
        Expr::Call(name, args) => {
            if !bound(name, bindings) {
                let canonical = canonical_name_any(name);
                if !std::ptr::eq(canonical, name.as_str()) {
                    *name = canonical.to_string();
                }
            }
            for arg in args {
                normalize_any(arg, known_name, bindings, depth + 1)?;
            }
        }
        Expr::Invoke(callee, args) => {
            normalize_any(callee, known_name, bindings, depth + 1)?;
            for arg in args {
                normalize_any(arg, known_name, bindings, depth + 1)?;
            }
        }
        Expr::Unary(_, child) | Expr::Paren(child) => normalize_any(child, known_name, bindings, depth + 1)?,
        Expr::Binary(_, left, right) => {
            normalize_any(left, known_name, bindings, depth + 1)?;
            normalize_any(right, known_name, bindings, depth + 1)?;
        }
        _ => {}
    }
    Ok(())
}
