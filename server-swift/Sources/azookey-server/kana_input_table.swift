import KanaKanjiConverterModule

// JIS かな入力では、クライアントが物理キーをかな 1 文字へ変換してから送る。
// サーバー側の表は直前のかなと濁点・半濁点の合成だけを受け持ち、
// 表にない文字はそのまま変換対象へ入る。
let kanaInputTableName = "azookey-windows-kana-jis"

private let dakutenPairs: [(base: Character, voiced: Character)] = [
    ("う", "ゔ"),
    ("か", "が"), ("き", "ぎ"), ("く", "ぐ"), ("け", "げ"), ("こ", "ご"),
    ("さ", "ざ"), ("し", "じ"), ("す", "ず"), ("せ", "ぜ"), ("そ", "ぞ"),
    ("た", "だ"), ("ち", "ぢ"), ("つ", "づ"), ("て", "で"), ("と", "ど"),
    ("は", "ば"), ("ひ", "び"), ("ふ", "ぶ"), ("へ", "べ"), ("ほ", "ぼ"),
    ("ゝ", "ゞ"),
]

private let handakutenPairs: [(base: Character, semiVoiced: Character)] = [
    ("は", "ぱ"), ("ひ", "ぴ"), ("ふ", "ぷ"), ("へ", "ぺ"), ("ほ", "ぽ"),
]

func kanaInputTableEntries() -> [(key: String, value: String)] {
    dakutenPairs.map { (key: "\($0.base)゛", value: String($0.voiced)) }
        + handakutenPairs.map { (key: "\($0.base)゜", value: String($0.semiVoiced)) }
}

func makeKanaInputTable() -> InputTable {
    var mapping: [[InputTable.KeyElement]: [InputTable.ValueElement]] = [:]
    for entry in kanaInputTableEntries() {
        mapping[entry.key.map { .piece(.character($0)) }] = entry.value.map { .character($0) }
    }
    return InputTable(baseMapping: mapping)
}
