use gridcraft_formula::{FormulaLocale, TURKISH, eq_tr_ignore_case, parse, parse_input, print, print_input};

fn input(src: &str) -> String {
    print(&parse_input(src, FormulaLocale::Tr, |_| false).unwrap())
}

#[test]
fn turkish_table_entries_are_unique_and_round_trip() {
    for (i, (en, tr)) in TURKISH.iter().enumerate() {
        assert!(!en.is_empty() && !tr.is_empty());
        for (other_en, other_tr) in TURKISH.iter().take(i) {
            assert!(!en.eq_ignore_ascii_case(other_en), "duplicate English function in TURKISH: {en}");
            assert!(!eq_tr_ignore_case(tr, other_tr), "duplicate Turkish function in TURKISH: {tr} vs {other_tr}");
        }
        assert_eq!(FormulaLocale::Tr.function_name(en), *tr);
        assert_eq!(FormulaLocale::Tr.canonical_name(tr), *en);
        assert_eq!(FormulaLocale::Tr.canonical_name(&tr.to_lowercase()), *en, "lowercase failed for {tr}");
    }
}

#[test]
fn turkish_aliases_and_argument_separators() {
    for (src, expected) in [
        ("=topla(A1:A10)", "SUM(A1:A10)"),
        ("ORTALAMA(A1:A10)", "AVERAGE(A1:A10)"),
        ("EĞER(A1>0;\"Evet\";\"Hayır\")", "IF(A1>0,\"Evet\",\"Hayır\")"),
        ("=eğer(A1>0;1;0)", "IF(A1>0,1,0)"),
        ("BAĞ_DEĞ_SAY(A1:A10)", "COUNT(A1:A10)"),
        ("BAĞ_DEĞ_DOLU_SAY(A1:A10)", "COUNTA(A1:A10)"),
        ("ETOPLA(A1:A10;\">0\")", "SUMIF(A1:A10,\">0\")"),
        ("EĞERSAY(A1:A10;\">0\")", "COUNTIF(A1:A10,\">0\")"),
        ("DÜŞEYARA(A2;D:E;2;YANLIŞ)", "VLOOKUP(A2,D:E,2,FALSE)"),
        ("=düşeyara(A2;D:E;2;yanlış)", "VLOOKUP(A2,D:E,2,FALSE)"),
        ("ÇAPRAZARA(A2;B2:B10;C2:C10;\"Yok\")", "XLOOKUP(A2,B2:B10,C2:C10,\"Yok\")"),
        ("İNDİS(A1:C10;KAÇINCI(E1;A1:A10;0);2)", "INDEX(A1:C10,MATCH(E1,A1:A10,0),2)"),
        ("=indis(A1:C10;kaçıncı(E1;A1:A10;0);2)", "INDEX(A1:C10,MATCH(E1,A1:A10,0),2)"),
        ("BUGÜN()", "TODAY()"),
        ("ŞİMDİ()", "NOW()"),
        ("=doğru", "TRUE"),
        ("=yanlış", "FALSE"),
        ("DOĞRU()", "TRUE()"),
        ("YANLIŞ()", "FALSE()"),
        ("TOPLA(1;SUM(2;3);ORTALAMA(4;6))", "SUM(1,SUM(2,3),AVERAGE(4,6))"),
        ("EĞER(DOĞRU;;)", "IF(TRUE,,)"),
        ("LAMBDA(x;x+1)(2)", "LAMBDA(x,x+1)(2)"),
    ] {
        assert_eq!(input(src), expected, "{src}");
    }
}

#[test]
fn turkish_input_preserves_reference_and_array_syntax() {
    for (src, expected) in [
        ("TOPLA({1\\2;3\\4};5)", "SUM({1,2;3,4},5)"),
        ("{DOĞRU\\YANLIŞ;TRUE\\FALSE}", "{TRUE,FALSE;TRUE,FALSE}"),
        ("SUM((A1:A3;C1:C3))", "SUM((A1:A3,C1:C3))"),
        ("\"TOPLA;YANLIŞ\"&\"a\"\"b\"", "\"TOPLA;YANLIŞ\"&\"a\"\"b\""),
        ("'TOPLA;YANLIŞ'!A1+TOPLA!B2+TOPLA!YANLIŞ", "'TOPLA;YANLIŞ'!A1+TOPLA!B2+TOPLA!YANLIŞ"),
        ("TOPLA[EĞERSAY]+YANLIŞ[EĞER]+BAĞ_DEĞ_SAY", "TOPLA[EĞERSAY]+YANLIŞ[EĞER]+BAĞ_DEĞ_SAY"),
    ] {
        assert_eq!(input(src), expected, "{src}");
    }
    for bad in [r"TOPLA({1;2\3})", "{TOPLA(A1)}", "{YANLIŞ;}"] {
        assert!(parse_input(bad, FormulaLocale::Tr, |_| false).is_err(), "{bad}");
    }
}

#[test]
fn turkish_locale_controls_decimal_list_array_separators_and_display() {
    let tr = FormulaLocale::Tr;
    let en = FormulaLocale::En;
    let expr = parse_input("EĞER(DOĞRU;TOPLA(1,5;2,25);0)", tr, |_| false).unwrap();
    assert_eq!(print(&expr), "IF(TRUE,SUM(1.5,2.25),0)");
    assert_eq!(print_input(&expr, tr, |_| false).unwrap(), "EĞER(DOĞRU;TOPLA(1,5;2,25);0)");
    assert_eq!(print_input(&expr, en, |_| false).unwrap(), print(&expr));
    assert!(parse_input("TOPLA(1.5;2)", tr, |_| false).is_err());
    let expr = parse("SUM({1.5,2.25;3,4})+SUM((A1:A3,C1:C3))").unwrap();
    let shown = print_input(&expr, tr, |_| false).unwrap();
    assert_eq!(print(&parse_input(&shown, tr, |_| false).unwrap()), print(&expr));
    let expr = parse("LET(TOPLA,LAMBDA(x,x+1),SUM(TOPLA(2),FALSE))").unwrap();
    let shown = print_input(&expr, tr, |n| n == "YANLIŞ").unwrap();
    assert_eq!(shown, "LET(TOPLA;LAMBDA(x;x+1);SUM(TOPLA(2);FALSE))");
    assert_eq!(print(&parse_input(&shown, tr, |n| n == "YANLIŞ").unwrap()), print(&expr));
}
