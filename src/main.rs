use std::collections::HashMap;

const LETTERS: &str = "ABCDEFGHIJKLMNOPQRSTUVWXYZ";

type Frequency = HashMap<char, usize>;

fn frequency_analysis(text: String) -> Frequency {
    let upper_text = text.clone().to_uppercase();
    let mut letter_frequencies: Frequency = Frequency::new();

    for letter in LETTERS.chars().into_iter() {
        letter_frequencies.insert(letter, 0);
    }

    for letter in upper_text.chars().into_iter() {
        if LETTERS.contains(letter) {
            letter_frequencies
                .entry(letter)
                .and_modify(|freq| *freq += 1);
        }
    }

    letter_frequencies
}

fn plot_distribution(frequency: &Frequency) {
    println!("| {0: <10} | {1: <10} |", "Letter", "Frequency");
    for (letter, freq) in frequency.iter() {
        println!("| {0: <10} | {1: <10} |", *letter, *freq);
    }
}

fn caesar_crack(text: String) {
    let frequency = frequency_analysis(text);
    plot_distribution(&frequency);
    let most_frequent = frequency
        .iter()
        .reduce(|acc, e| if *e.1 > *acc.1 { e } else { acc })
        .unwrap();
    let e_idx = LETTERS.find('E').unwrap();
    let most_freq_idx = LETTERS
        .find(*most_frequent.0)
        .unwrap();
    let key = most_freq_idx
        .checked_sub(e_idx)
        .unwrap_or(LETTERS.len() - e_idx + most_freq_idx);
    println!("Most frequent letter: {} ({})", most_frequent.0, most_frequent.1);
    println!("The possible key value: {}", key);
}

fn main() {
    let plain_text = String::from("THEHOLYCROSSBEMYLIGHTLETNOTTHEDRAGONBEMYLEADERGETBACKSATANNEVERTEMPTMEWITHVANITYTHEEVILSYOUOFFERYOUSHALLDRINKTHEPOISONYOURSEL");
    let cipher_text = String::from("WKHKROBFURVVEHPBOLJKWOHWQRWWKHGUDJRQEHPBOHDGHUJHWEDFNVDWDQQHYHUWHPSWPHZLWKYDQLWBWKHHYLOVBRXRIIHUBRXVKDOOGULQNWKHSRLVRQBRXUVHOI");
    println!("Used key: {}", 3);
    println!("Decrypted text: {}", plain_text);
    println!("Encrypter text: {}", cipher_text);
    caesar_crack(cipher_text);
}
