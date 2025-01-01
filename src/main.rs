use std::collections::HashMap;

struct Frequency {
    values: HashMap<char, usize>,
}

impl Frequency {
    const LETTERS: &str = "ABCDEFGHIJKLMNOPQRSTUVWXYZ";

    fn new(text: String) -> Self {
        let upper_text = text.clone().to_uppercase();
        let mut values = HashMap::new();

        for letter in Frequency::LETTERS.chars().into_iter() {
            values.insert(letter, 0);
        }

        for letter in upper_text.chars().into_iter() {
            if Frequency::LETTERS.contains(letter) {
                values.entry(letter).and_modify(|freq| *freq += 1);
            }
        }

        Self { values }
    }

    fn get_most_frequent(&self) -> (&char, &usize) {
        self.values
            .iter()
            .reduce(|acc, e| if *e.1 > *acc.1 { e } else { acc })
            .unwrap()
    }

    fn print(&self) {
        println!("| {0: <10} | {1: <10} |", "Letter", "Frequency");
        for (letter, freq) in self.values.iter() {
            println!("| {0: <10} | {1: <10} |", *letter, *freq);
        }
    }

    fn caesar_crack(text: String) {
        let frequency = Frequency::new(text);
        frequency.print();
        let most_frequent = frequency.get_most_frequent();
        let e_idx = Frequency::LETTERS.find('E').unwrap();
        let most_freq_idx = Frequency::LETTERS.find(*most_frequent.0).unwrap();
        let key = most_freq_idx
            .checked_sub(e_idx)
            .unwrap_or(Frequency::LETTERS.len() - e_idx + most_freq_idx);
        println!(
            "Most frequent letter: {} ({})",
            most_frequent.0, most_frequent.1
        );
        println!("The possible key value: {}", key);
    }
}

fn main() {
    let plain_text = String::from("THEHOLYCROSSBEMYLIGHTLETNOTTHEDRAGONBEMYLEADERGETBACKSATANNEVERTEMPTMEWITHVANITYTHEEVILSYOUOFFERYOUSHALLDRINKTHEPOISONYOURSEL");
    let cipher_text = String::from("WKHKROBFURVVEHPBOLJKWOHWQRWWKHGUDJRQEHPBOHDGHUJHWEDFNVDWDQQHYHUWHPSWPHZLWKYDQLWBWKHHYLOVBRXRIIHUBRXVKDOOGULQNWKHSRLVRQBRXUVHOI");
    println!("Used key: {}", 3);
    println!("Decrypted text: {}", plain_text);
    println!("Encrypter text: {}", cipher_text);
    Frequency::caesar_crack(cipher_text);
}
