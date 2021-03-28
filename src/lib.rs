use std::fmt::Display;
extern crate unicode_normalization;
use unicode_normalization::char::compose;
use unicode_normalization::UnicodeNormalization;
//use alloc::string::String;

#[macro_use]
extern crate bitflags;

bitflags! {
    pub struct HGKDiacritics: u32 {
        const ROUGH          = 0x001;
        const SMOOTH         = 0x002;
        const ACUTE          = 0x004;
        const GRAVE          = 0x008;
        const CIRCUMFLEX     = 0x010;
        const MACRON         = 0x020;
        const BREVE          = 0x040;
        const IOTA_SUBSCRIPT = 0x080;
        const DIAERESIS      = 0x100;
        const UNDERDOT       = 0x200;
        //const ABC = Self::A.bits | Self::B.bits | Self::C.bits;
    }
}

pub enum HGKUnicode_Mode {
    Precomposed,
    PrecomposedPUA,
    CombiningOnly
}

struct HGKLetter {
    letter: char,
    diacritics: HGKDiacritics
}

impl HGKLetter {
    fn from_str(l:&str) -> HGKLetter {
        let mut diacritics: HGKDiacritics = HGKDiacritics::empty();
        let mut bare_letter: char = '\u{0000}';
        for (i, c) in l.nfd().enumerate() {
            if i == 0 {
                if unicode_normalization::char::is_combining_mark(c) {
                    assert!(false, "First char of letter is a combining mark.");
                }
                bare_letter = c;
            }
            else {
                if !unicode_normalization::char::is_combining_mark(c) {
                    break;
                }
                else {
                    match c {
                        '\u{0300}' => diacritics |= HGKDiacritics::GRAVE,
                        '\u{0301}' => diacritics |= HGKDiacritics::ACUTE,
                        '\u{0304}' => diacritics |= HGKDiacritics::MACRON,
                        '\u{0306}' => diacritics |= HGKDiacritics::BREVE,
                        '\u{0308}' => diacritics |= HGKDiacritics::DIAERESIS,
                        '\u{0313}' => diacritics |= HGKDiacritics::SMOOTH,
                        '\u{0314}' => diacritics |= HGKDiacritics::ROUGH,
                        '\u{0323}' => diacritics |= HGKDiacritics::UNDERDOT,
                        '\u{0342}' => diacritics |= HGKDiacritics::CIRCUMFLEX,
                        '\u{0345}' => diacritics |= HGKDiacritics::IOTA_SUBSCRIPT,
                        _ => ()
                    }
                }
            }
        }
        return HGKLetter { letter: bare_letter, diacritics: diacritics };
    }
/*
COMBINING_MACRON, 
COMBINING_BREVE, 
COMBINING_DIAERESIS, 
COMBINING_ROUGH_BREATHING, 
COMBINING_SMOOTH_BREATHING, 
COMBINING_ACUTE, 
COMBINING_GRAVE, 
COMBINING_CIRCUMFLEX, 
COMBINING_IOTA_SUBSCRIPT, 
COMBINING_UNDERDOT
*/
    fn to_string(&mut self, unicode_mode:HGKUnicode_Mode) -> String {
        //let mut s = self.letter.to_string();
        let mut s = vec![self.letter];
        if (self.diacritics & HGKDiacritics::MACRON) == HGKDiacritics::MACRON {
            //s = s + "\u{0304}";
            s.push('\u{0304}');
        }
        if (self.diacritics & HGKDiacritics::BREVE) == HGKDiacritics::BREVE {
            //s = s + "\u{0306}";
            s.push('\u{0306}');
        }
        if (self.diacritics & HGKDiacritics::DIAERESIS) == HGKDiacritics::DIAERESIS {
            //s = s + "\u{0308}";
            s.push('\u{0308}');
        }
        if (self.diacritics & HGKDiacritics::ROUGH) == HGKDiacritics::ROUGH {
            //s = s + "\u{0314}";
            s.push('\u{0314}');
        }
        if (self.diacritics & HGKDiacritics::SMOOTH) == HGKDiacritics::SMOOTH {
            //s = s + "\u{0313}";
            s.push('\u{0313}');
        }    
        if (self.diacritics & HGKDiacritics::ACUTE) == HGKDiacritics::ACUTE {
            //s = s + "\u{0301}";
            s.push('\u{0301}');
        }
        if (self.diacritics & HGKDiacritics::GRAVE) == HGKDiacritics::GRAVE {
            //s = s + "\u{0300}";
            s.push('\u{0300}');
        }
        if (self.diacritics & HGKDiacritics::CIRCUMFLEX) == HGKDiacritics::CIRCUMFLEX {
            //s = s + "\u{0342}";
            s.push('\u{0342}');
        }
        if (self.diacritics & HGKDiacritics::IOTA_SUBSCRIPT) == HGKDiacritics::IOTA_SUBSCRIPT {
            //s = s + "\u{0345}";
            s.push('\u{0345}');
        }
        if (self.diacritics & HGKDiacritics::UNDERDOT) == HGKDiacritics::UNDERDOT {
            //s = s + "\u{0323}";
            s.push('\u{0323}');
        }
        match unicode_mode {
            HGKUnicode_Mode::CombiningOnly => return s.into_iter().collect::<String>(),
            HGKUnicode_Mode::PrecomposedPUA => return s.into_iter().nfc().collect::<String>(),
            _ => return s.into_iter().nfc().collect::<String>()
        }  
    }

    fn toggle_diacritic(&mut self, d:HGKDiacritics, on_only:bool) {
        if !self.is_legal(d) {
            return;
        }

        if self.diacritics & d != d || on_only {
            self.diacritics |= d;
        }
        else {
            self.diacritics &= !d; //turn off: rust uses !, c uses ~
            //return;
        }
        match d {
            HGKDiacritics::ROUGH => {
                self.diacritics &= !(HGKDiacritics::SMOOTH | HGKDiacritics::DIAERESIS);
            },
            HGKDiacritics::SMOOTH => {
                self.diacritics &= !(HGKDiacritics::ROUGH | HGKDiacritics::DIAERESIS);
            },
            HGKDiacritics::ACUTE => {
                self.diacritics &= !(HGKDiacritics::GRAVE | HGKDiacritics::CIRCUMFLEX);
            },
            HGKDiacritics::GRAVE => {
                self.diacritics &= !(HGKDiacritics::ACUTE | HGKDiacritics::CIRCUMFLEX);
            },
            HGKDiacritics::CIRCUMFLEX => {
                self.diacritics &= !(HGKDiacritics::ACUTE | HGKDiacritics::GRAVE);
            },
            HGKDiacritics::MACRON => {
                self.diacritics &= !(HGKDiacritics::BREVE | HGKDiacritics::CIRCUMFLEX);
            },
            HGKDiacritics::BREVE => {
                self.diacritics &= !(HGKDiacritics::MACRON | HGKDiacritics::CIRCUMFLEX | HGKDiacritics::IOTA_SUBSCRIPT);
            },
            HGKDiacritics::IOTA_SUBSCRIPT => {
                self.diacritics &= !(HGKDiacritics::BREVE);
            },
            HGKDiacritics::DIAERESIS => {
                self.diacritics &= !(HGKDiacritics::ROUGH | HGKDiacritics::SMOOTH);
            },
            //HGKDiacritics::UNDERDOT => { },
            _ => {
                assert!(false, "Unknown Diacritic passed")
            }
        }
    }

    fn is_legal(&mut self, d:HGKDiacritics) -> bool {
        match d {
            HGKDiacritics::ROUGH => {
                self.letter.is_greek_vowel()
            },
            HGKDiacritics::SMOOTH => {
                self.letter.is_greek_vowel()
            },
            HGKDiacritics::ACUTE => {
                self.letter.is_greek_vowel()
            },
            HGKDiacritics::GRAVE => {
                self.letter.is_greek_vowel()
            },
            HGKDiacritics::CIRCUMFLEX => {
                self.letter.is_long_or_short() | self.letter.is_long()
            },
            HGKDiacritics::MACRON => {
                self.letter.is_long_or_short()
            },
            HGKDiacritics::BREVE => {
                self.letter.is_long_or_short()    
            },
            HGKDiacritics::IOTA_SUBSCRIPT => {
                match self.letter {
                    'α' => true,
                    'ω' => true,
                    'η' => true,
                    _ => false
                } 
            },
            HGKDiacritics::DIAERESIS => {
                match self.letter {
                    'ι' => true,
                    'υ' => true,
                    _ => false
                }                
            },
            HGKDiacritics::UNDERDOT => { 
                true
            },
            _ => false
        }
    }
}
/*
//https://doc.rust-lang.org/stable/rust-by-example/conversion/string.html
impl Display for HGKLetter {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::result::Result {
        write!(f, "{}", self.to_string2(HGKUnicode_Mode::Precomposed));
    }
}
*/
trait HGKIsLong {
    fn is_long(&self) -> bool;
}

impl HGKIsLong for char {
    fn is_long(&self) -> bool {
        match self {
            'η' => true,
            'ω' => true,
            _ => false
        }
    }
}

trait HGKIsShort {
    fn is_short(&self) -> bool;
}

impl HGKIsShort for char {
    fn is_short(&self) -> bool {
        match self {
            'ε' => true,
            'ο' => true,
            _ => false
        }
    }
}

trait HGKIsLongOrShort {
    fn is_long_or_short(&self) -> bool;
}

impl HGKIsLongOrShort for char {
    fn is_long_or_short(&self) -> bool {
        match self {
            'α' => true,
            'ι' => true,
            'υ' => true,
            _ => false
        }
    }
}

trait HGKIsGreekVowel {
    fn is_greek_vowel(&self) -> bool;
}

impl HGKIsGreekVowel for char {
    fn is_greek_vowel(&self) -> bool {
        //let letter2 = self.to_lowercase();
        match self {
            'α' => true,
            'ε' => true,
            'η' => true,
            'ι' => true,
            'ο' => true,
            'υ' => true,
            'ω' => true,
            'Α' => true,
            'Ε' => true,
            'Η' => true,
            'Ι' => true,
            'Ο' => true,
            'Υ' => true,
            'Ω' => true,
            _ => false
        }
    }
}

pub fn toggle_diacritic_str(l:&str, d:HGKDiacritics, on_only:bool, mode:HGKUnicode_Mode) -> String {
    let mut letter = HGKLetter::from_str(l);
    letter.toggle_diacritic(d, on_only);
    return letter.to_string(mode);
}

static greek_upper: &'static [char] = &[
'\u{0391}',
'\u{0392}',
'\u{03A8}',
'\u{0394}',
'\u{0395}',
'\u{03A6}',
'\u{0393}',
'\u{0397}',
'\u{0399}',
'\u{039E}',
'\u{039A}',
'\u{039B}',
'\u{039C}',
'\u{039D}',
'\u{039F}',
'\u{03A0}',
'\u{03DC}',
'\u{03A1}',
'\u{03A3}',
'\u{03A4}',
'\u{0398}',
'\u{03A9}',
'\u{00B7}',
'\u{03A7}',
'\u{03A5}',
'\u{0396}'
];

static greek_lower: &'static [char] = &[
'\u{03B1}',
'\u{03B2}',
'\u{03C8}',
'\u{03B4}',
'\u{03B5}',
'\u{03C6}',
'\u{03B3}',
'\u{03B7}',
'\u{03B9}',
'\u{03BE}',
'\u{03BA}',
'\u{03BB}',
'\u{03BC}',
'\u{03BD}',
'\u{03BF}',
'\u{03C0}',
'\u{03DD}',
'\u{03C1}',
'\u{03C3}',
'\u{03C4}',
'\u{03B8}',
'\u{03C9}',
'\u{03C2}',
'\u{03C7}',
'\u{03C5}',
'\u{03B6}'
];


pub fn transliterate(input:usize) -> char {
    if input >= 0x0061 && input <= 0x007A {
        return greek_lower[input - 0x0061];
    }
    else if input >= 0x0041 && input <= 0x005A {
        return greek_upper[input - 0x0041];
    }
    else {
        return '\u{0000}';
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn mytest() {
        let s = "ἄβί".to_string();
        let a = s.nfd();
        assert_eq!(a.count(), 6);

        assert_eq!(transliterate(0x0000), '\u{0000}');
        assert_eq!(transliterate(0x0040), '\u{0000}');
        assert_eq!(transliterate(0x0061), '\u{03B1}');
        assert_eq!(transliterate(0x007B), '\u{0000}');

        assert_eq!('α'.is_long_or_short(), true);
        assert_eq!('ι'.is_long_or_short(), true);
        assert_eq!('υ'.is_long_or_short(), true);
        assert_eq!('η'.is_long(), true);
        assert_eq!('ω'.is_long(), true);
        assert_eq!('ε'.is_short(), true);
        assert_eq!('ο'.is_short(), true);

        let a2 = HGKLetter::from_str("\u{03B1}\u{0301}");
        assert_eq!(a2.diacritics & HGKDiacritics::ACUTE, HGKDiacritics::ACUTE);
        assert_eq!(a2.letter, '\u{03B1}');
        let a3 = HGKLetter::from_str("\u{03AC}");
        assert_eq!(a3.diacritics & HGKDiacritics::ACUTE, HGKDiacritics::ACUTE);
        assert_eq!(a3.letter, '\u{03B1}');

        let mut s: HGKLetter = HGKLetter { letter: 'α', diacritics: HGKDiacritics::ACUTE | HGKDiacritics::GRAVE };
        assert_eq!(s.diacritics & HGKDiacritics::ACUTE, HGKDiacritics::ACUTE);
        assert_ne!(s.diacritics & HGKDiacritics::CIRCUMFLEX, HGKDiacritics::CIRCUMFLEX);

        s.toggle_diacritic(HGKDiacritics::CIRCUMFLEX, true);
        assert_eq!(s.diacritics & HGKDiacritics::CIRCUMFLEX, HGKDiacritics::CIRCUMFLEX);
        //don't toggle off, if on_only is set
        s.toggle_diacritic(HGKDiacritics::CIRCUMFLEX, true);
        assert_eq!(s.diacritics & HGKDiacritics::CIRCUMFLEX, HGKDiacritics::CIRCUMFLEX);
        //turn off
        s.toggle_diacritic(HGKDiacritics::CIRCUMFLEX, false);
        assert_ne!(s.diacritics & HGKDiacritics::CIRCUMFLEX, HGKDiacritics::CIRCUMFLEX);

        assert_eq!(compose('A','\u{30a}'), Some('Å'));

        let s = "ÅΩ";
        let c = s.nfc().collect::<String>();
        assert_eq!(c, "ÅΩ");

    	assert_eq!(compose('\u{03B1}','\u{0301}'), Some('\u{03AC}'));
    	assert_eq!(compose('\u{03B1}','\u{0301}'), Some('\u{03AC}'));
    	assert_eq!('a','a');

        let a = "\u{03B1}\u{0301}";
        let b = "\u{03AC}";
        assert_ne!(a, b);

        let s = String::from("ἄ");
        let v: Vec<char> = s.chars().collect();

        let a4 = toggle_diacritic_str("α", HGKDiacritics::ACUTE, false, HGKUnicode_Mode::Precomposed);
        assert_eq!(a4, "\u{03AC}");//ά");
        let a6 = toggle_diacritic_str("ὰ", HGKDiacritics::ACUTE, false, HGKUnicode_Mode::Precomposed);
        assert_eq!(a6, "\u{03AC}");//ά");
        let a5 = toggle_diacritic_str("α", HGKDiacritics::ACUTE, false, HGKUnicode_Mode::CombiningOnly);
        assert_eq!(a5, "\u{03B1}\u{0301}");
        let a7 = toggle_diacritic_str("α", HGKDiacritics::CIRCUMFLEX, false, HGKUnicode_Mode::CombiningOnly);
        assert_eq!(a7, "\u{03B1}\u{0342}");
        let a8 = toggle_diacritic_str("α", HGKDiacritics::CIRCUMFLEX, false, HGKUnicode_Mode::Precomposed);
        assert_eq!(a8, "\u{1FB6}");

        let a9 = toggle_diacritic_str("ε", HGKDiacritics::CIRCUMFLEX, false, HGKUnicode_Mode::Precomposed);
        assert_eq!(a9, "ε");
    }
}
