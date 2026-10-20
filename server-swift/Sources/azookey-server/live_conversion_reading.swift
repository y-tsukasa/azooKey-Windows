// ライブ変換で、変換せずに読みのまま見せるかどうかの規則。
// azooKey macOS に合わせ、短い入力と削除の直後は読み（ひらがな）を先頭の候補にする。
// 候補を消すわけではないので、スペースを押せば次の候補（漢字）へ進める。

let defaultLiveConversionMinLength = 2
let liveConversionMinLengthRange = 1...10

func normalizedLiveConversionMinLength(_ value: Int?) -> Int {
    guard let value else {
        return defaultLiveConversionMinLength
    }
    return min(max(value, liveConversionMinLengthRange.lowerBound), liveConversionMinLengthRange.upperBound)
}

func liveConversionShowsReading(hiraganaCount: Int, minLength: Int, afterDeletion: Bool) -> Bool {
    hiraganaCount > 0 && (afterDeletion || hiraganaCount < minLength)
}

/// 読みと同じ表記の候補を先頭へ移す。見つからなければ並びを変えない。
func prioritizeReadingCandidate<Element>(
    _ elements: [Element],
    reading: String,
    text: (Element) -> String
) -> [Element] {
    guard let index = elements.firstIndex(where: { text($0) == reading }), index > 0 else {
        return elements
    }
    var reordered = elements
    reordered.insert(reordered.remove(at: index), at: 0)
    return reordered
}
