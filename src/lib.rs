// Uniaxe: A weapon for the war against unicode text

#[warn(clippy::all, clippy::pedantic)]
#[allow(clippy::too_many_lines)]
pub mod lookup;

use std::collections::HashMap;

pub fn uniaxe(text: &str, table: &HashMap<char, char>) -> String {
    // Function to destroy unicode text
    let mut result = String::new();
    for c in text.chars() {
        if lookup::TEMPLATE.contains(c) {
            result.push(c);
        } else if let Some(&equiv) = table.get(&c) {
            result.push(equiv);
        } else {
            result.push(c);
        }
    }
    result
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn conversion() {
        let table = lookup::generate_table();
        //println!("{:?}", table);
        assert_eq!(
            uniaxe("𝔥𝔢𝔩𝔩𝔬 𝔱𝔥𝔢𝔯𝔢 𝔈𝔫𝔍𝔬𝔶 𝔗𝔥𝔦𝔰 𝔊𝔬𝔗𝔥ℑ𝔠 𝕿𝖊𝖝𝖙 123", &table),
            "hello there EnJoy This GoThIc Text 123"
        );
        assert_eq!(
            uniaxe("𝓲𝓳𝓴𝓵𝓶ｆｇｈｉｊｋｌｍ1234567890", &table),
            "ijklmfghijklm1234567890"
        );
        assert_eq!(
            uniaxe(
                "●▬▬▬▬𝗟𝗮𝘀𝘁 𝗿𝗲𝗺𝗶𝗻𝗱𝗲𝗿,⛔️ 𝗬𝗼𝘂𝗿 𝗔𝗻𝘁𝗶𝘃𝗶𝗿𝘂𝘀 𝗲𝘅𝗽𝗶𝗿𝗲𝘀 𝗶𝗻 𝟮𝟰𝗵𝗿💯❗❗---▬▬▬▬▬● #UdL04",
                &table
            ),
            "●▬▬▬▬Last reminder,⛔️ Your Antivirus expires in 24hr💯❗❗---▬▬▬▬▬● #UdL04"
        );
    }
    #[test]
    fn normal() {
        let table = lookup::generate_table();
        assert_eq!(
            uniaxe(
                "abcdefghijklmnopqrstuvwxyzABCDEFGHIJKLMNOPQRSTUVWXYZ0123456789",
                &table
            ),
            "abcdefghijklmnopqrstuvwxyzABCDEFGHIJKLMNOPQRSTUVWXYZ0123456789"
        );
    }
}
