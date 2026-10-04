//! JIS かな配列のキー割り当て。
//!
//! かな入力では `ToUnicode` の結果ではなくスキャンコード（物理キー位置）を使う。
//! JIS 配列では `¥` キーと `ろ` キーがどちらも `\` を返すため、文字からは
//! 「ー」と「ろ」を区別できない。Shift+0 のように文字を返さないキーもある。

const EXTENDED_KEY_BIT: isize = 1 << 24;

/// かな入力時に 1 回のキー押下で入力する文字。
///
/// 句読点や括弧は ASCII 記号で返す。ローマ字入力と同じ経路を通り、
/// 句読点・記号スタイルと全角半角設定がそのまま適用される。
pub(crate) fn jis_kana_for_key(lparam: isize, is_shift_pressed: bool) -> Option<char> {
    if lparam & EXTENDED_KEY_BIT != 0 {
        // テンキーの `/` などはスキャンコードが文字キーと重なる。
        return None;
    }

    let scan_code = ((lparam >> 16) & 0xFF) as u8;
    if is_shift_pressed {
        if let Some(ch) = shifted_jis_kana(scan_code) {
            return Some(ch);
        }
    }

    jis_kana(scan_code)
}

fn jis_kana(scan_code: u8) -> Option<char> {
    let ch = match scan_code {
        0x02 => 'ぬ',
        0x03 => 'ふ',
        0x04 => 'あ',
        0x05 => 'う',
        0x06 => 'え',
        0x07 => 'お',
        0x08 => 'や',
        0x09 => 'ゆ',
        0x0A => 'よ',
        0x0B => 'わ',
        0x0C => 'ほ',
        0x0D => 'へ',
        0x7D => 'ー', // ¥

        0x10 => 'た',
        0x11 => 'て',
        0x12 => 'い',
        0x13 => 'す',
        0x14 => 'か',
        0x15 => 'ん',
        0x16 => 'な',
        0x17 => 'に',
        0x18 => 'ら',
        0x19 => 'せ',
        0x1A => '゛', // @
        0x1B => '゜', // [

        0x1E => 'ち',
        0x1F => 'と',
        0x20 => 'し',
        0x21 => 'は',
        0x22 => 'き',
        0x23 => 'く',
        0x24 => 'ま',
        0x25 => 'の',
        0x26 => 'り',
        0x27 => 'れ', // ;
        0x28 => 'け', // :
        0x2B => 'む', // ]

        0x2C => 'つ',
        0x2D => 'さ',
        0x2E => 'そ',
        0x2F => 'ひ',
        0x30 => 'こ',
        0x31 => 'み',
        0x32 => 'も',
        0x33 => 'ね', // ,
        0x34 => 'る', // .
        0x35 => 'め', // /
        0x73 => 'ろ', // \ (ろ)
        _ => return None,
    };
    Some(ch)
}

fn shifted_jis_kana(scan_code: u8) -> Option<char> {
    let ch = match scan_code {
        0x04 => 'ぁ',
        0x05 => 'ぅ',
        0x06 => 'ぇ',
        0x07 => 'ぉ',
        0x08 => 'ゃ',
        0x09 => 'ゅ',
        0x0A => 'ょ',
        0x0B => 'を',
        0x12 => 'ぃ',
        0x2C => 'っ',
        0x1B => '[', // 「
        0x2B => ']', // 」
        0x33 => ',', // 、
        0x34 => '.', // 。
        0x35 => '/', // ・
        _ => return None,
    };
    Some(ch)
}

#[cfg(test)]
mod tests {
    use super::jis_kana_for_key;

    fn lparam(scan_code: isize) -> isize {
        1 | (scan_code << 16)
    }

    fn kana(scan_code: isize, shift: bool) -> Option<char> {
        jis_kana_for_key(lparam(scan_code), shift)
    }

    #[test]
    fn maps_unshifted_rows() {
        let number_row: String = (0x02..=0x0D).filter_map(|code| kana(code, false)).collect();
        assert_eq!(number_row, "ぬふあうえおやゆよわほへ");

        let top_row: String = (0x10..=0x1B).filter_map(|code| kana(code, false)).collect();
        assert_eq!(top_row, "たていすかんなにらせ゛゜");

        let home_row: String = (0x1E..=0x28).filter_map(|code| kana(code, false)).collect();
        assert_eq!(home_row, "ちとしはきくまのりれけ");

        let bottom_row: String = (0x2C..=0x35).filter_map(|code| kana(code, false)).collect();
        assert_eq!(bottom_row, "つさそひこみもねるめ");
    }

    #[test]
    fn distinguishes_yen_and_ro_keys() {
        assert_eq!(kana(0x7D, false), Some('ー'));
        assert_eq!(kana(0x73, false), Some('ろ'));
        assert_eq!(kana(0x7D, true), Some('ー'));
        assert_eq!(kana(0x73, true), Some('ろ'));
        assert_eq!(kana(0x2B, false), Some('む'));
    }

    #[test]
    fn maps_shifted_small_kana_and_wo() {
        let shifted: String = [0x04, 0x05, 0x06, 0x07, 0x08, 0x09, 0x0A, 0x0B, 0x12, 0x2C]
            .into_iter()
            .filter_map(|code| kana(code, true))
            .collect();
        assert_eq!(shifted, "ぁぅぇぉゃゅょをぃっ");
    }

    #[test]
    fn shifted_punctuation_uses_ascii_symbols() {
        assert_eq!(kana(0x33, true), Some(','));
        assert_eq!(kana(0x34, true), Some('.'));
        assert_eq!(kana(0x35, true), Some('/'));
        assert_eq!(kana(0x1B, true), Some('['));
        assert_eq!(kana(0x2B, true), Some(']'));
    }

    #[test]
    fn shift_keeps_kana_without_shifted_variant() {
        assert_eq!(kana(0x14, true), Some('か'));
        assert_eq!(kana(0x02, true), Some('ぬ'));
        assert_eq!(kana(0x1A, true), Some('゛'));
    }

    #[test]
    fn ignores_extended_and_non_character_keys() {
        // テンキー `/` は E0 35。
        assert_eq!(jis_kana_for_key(lparam(0x35) | (1 << 24), false), None);
        // Space, Enter, 半角/全角
        assert_eq!(kana(0x39, false), None);
        assert_eq!(kana(0x1C, false), None);
        assert_eq!(kana(0x29, false), None);
    }
}
