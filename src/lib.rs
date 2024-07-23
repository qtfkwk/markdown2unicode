#[doc = include_str!("../README.md")]
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        assert_eq!(
            convert("Here is some *emphasis* **bold** ***bold emphasis*** and `mono` text.\n"),
            "Here is some 𝘦𝘮𝘱𝘩𝘢𝘴𝘪𝘴 𝐛𝐨𝐥𝐝 𝒃𝒐𝒍𝒅 𝒆𝒎𝒑𝒉𝒂𝒔𝒊𝒔 and 𝚖𝚘𝚗𝚘 text.\n",
        );
    }
}

use lazy_static::lazy_static;
use std::collections::HashMap;

fn c2i(s: &str) -> HashMap<char, usize> {
    s.chars().enumerate().map(|(i, c)| (c, i)).collect()
}

fn i2c(s: &str) -> HashMap<usize, char> {
    s.chars().enumerate().collect()
}

const REGULAR: &str =
    "ABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz0123456789!@#$%^&*()_-+=?/|'\"`";
const BOLD: &str =
    "𝐀𝐁𝐂𝐃𝐄𝐅𝐆𝐇𝐈𝐉𝐊𝐋𝐌𝐍𝐎𝐏𝐐𝐑𝐒𝐓𝐔𝐕𝐖𝐗𝐘𝐙𝐚𝐛𝐜𝐝𝐞𝐟𝐠𝐡𝐢𝐣𝐤𝐥𝐦𝐧𝐨𝐩𝐪𝐫𝐬𝐭𝐮𝐯𝐰𝐱𝐲𝐳𝟎𝟏𝟐𝟑𝟒𝟓𝟔𝟕𝟖𝟗!@#$%^&*()_-+=?/|'\"`";
const ITALIC: &str =
    "𝘈𝘉𝘊𝘋𝘌𝘍𝘎𝘏𝘐𝘑𝘒𝘓𝘔𝘕𝘖𝘗𝘘𝘙𝘚𝘛𝘜𝘝𝘞𝘟𝘠𝘡𝘢𝘣𝘤𝘥𝘦𝘧𝘨𝘩𝘪𝘫𝘬𝘭𝘮𝘯𝘰𝘱𝘲𝘳𝘴𝘵𝘶𝘷𝘸𝘹𝘺𝘻0123456789!@#$%^&*()_-+=?/|'\"`";
const BOLD_ITALIC: &str =
    "𝑨𝑩𝑪𝑫𝑬𝑭𝑮𝑯𝑰𝑱𝑲𝑳𝑴𝑵𝑶𝑷𝑸𝑹𝑺𝑻𝑼𝑽𝑾𝑿𝒀𝒁𝒂𝒃𝒄𝒅𝒆𝒇𝒈𝒉𝒊𝒋𝒌𝒍𝒎𝒏𝒐𝒑𝒒𝒓𝒔𝒕𝒖𝒗𝒘𝒙𝒚𝒛0123456789!@#$%^&*()_-+=?/|'\"`";
const MONO: &str =
    "𝙰𝙱𝙲𝙳𝙴𝙵𝙶𝙷𝙸𝙹𝙺𝙻𝙼𝙽𝙾𝙿𝚀𝚁𝚂𝚃𝚄𝚅𝚆𝚇𝚈𝚉𝚊𝚋𝚌𝚍𝚎𝚏𝚐𝚑𝚒𝚓𝚔𝚕𝚖𝚗𝚘𝚙𝚚𝚛𝚜𝚝𝚞𝚟𝚠𝚡𝚢𝚣𝟶𝟷𝟸𝟹𝟺𝟻𝟼𝟽𝟾𝟿!@#$%^&*()_-+=?/|'\"`";

lazy_static! {
    static ref REGULAR_I: HashMap<char, usize> = c2i(REGULAR);
    static ref BOLD_I: HashMap<char, usize> = c2i(BOLD);
    static ref ITALIC_I: HashMap<char, usize> = c2i(ITALIC);
    static ref BOLD_ITALIC_I: HashMap<char, usize> = c2i(BOLD_ITALIC);
    static ref MONO_I: HashMap<char, usize> = c2i(MONO);
    static ref REGULAR_C: HashMap<usize, char> = i2c(REGULAR);
    static ref BOLD_C: HashMap<usize, char> = i2c(BOLD);
    static ref ITALIC_C: HashMap<usize, char> = i2c(ITALIC);
    static ref BOLD_ITALIC_C: HashMap<usize, char> = i2c(BOLD_ITALIC);
    static ref MONO_C: HashMap<usize, char> = i2c(MONO);
}

#[derive(Clone)]
enum Mode {
    Regular,
    Bold,
    Italic,
    BoldItalic,
    Mono,
}

impl Mode {
    fn asterisk(&self, asterisks: i32) -> Mode {
        match self {
            Mode::Regular => match asterisks {
                0 => Mode::Italic,
                1 => Mode::Bold,
                2 => Mode::BoldItalic,
                _ => self.clone(),
            },
            Mode::Italic => match asterisks {
                0 => Mode::Regular,
                1 => Mode::BoldItalic,
                2 => Mode::Bold,
                _ => self.clone(),
            },
            Mode::Bold => match asterisks {
                0 => Mode::BoldItalic,
                1 => Mode::Regular,
                2 => Mode::Italic,
                _ => self.clone(),
            },
            Mode::BoldItalic => match asterisks {
                0 => Mode::Bold,
                1 => Mode::Italic,
                2 => Mode::Regular,
                _ => self.clone(),
            },
            Mode::Mono => self.clone(),
        }
    }

    fn backtick(&self) -> Mode {
        match self {
            Mode::Regular => Mode::Mono,
            Mode::Mono => Mode::Regular,
            _ => self.clone(),
        }
    }
}

pub fn convert(md_text: &str) -> String {
    let mut result = String::new();
    let mut mode = Mode::Regular;
    let mut asterisks = 0;

    let len_md_text_minus_1 = md_text.len() - 1;
    let md_text_chars = md_text.chars().collect::<Vec<_>>();

    for (i, c) in md_text_chars.iter().enumerate() {
        if let Some(chr_index) = REGULAR_I.get(c) {
            match c {
                '*' => {
                    if i < len_md_text_minus_1 && md_text_chars[i + 1] == '*' {
                        asterisks = (asterisks + 1) % 4;
                    } else {
                        mode = mode.asterisk(asterisks);
                        asterisks = 0;
                    }
                }
                '`' => {
                    mode = mode.backtick();
                }
                _ => match mode {
                    Mode::Regular => {
                        result.push(*c);
                    }
                    Mode::Italic => {
                        result.push(ITALIC_C[chr_index]);
                    }
                    Mode::Bold => {
                        result.push(BOLD_C[chr_index]);
                    }
                    Mode::BoldItalic => {
                        result.push(BOLD_ITALIC_C[chr_index]);
                    }
                    Mode::Mono => {
                        result.push(MONO_C[chr_index]);
                    }
                },
            }
        } else {
            result.push(*c);
            continue;
        }
    }

    result
}
