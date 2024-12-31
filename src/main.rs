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

fn plot_distribution(frequencies: Frequency) {
    println!("| {0: <10} | {1: <10} |", "Letter", "Frequency");
    for (letter, frequency) in frequencies.iter() {
        println!("| {0: <10} | {1: <10} |", &letter, &frequency);
    }
}

fn main() {
    let plain_text = String::from("Cryptography is a method of protecting information and communications using codes so that only those for whom the information is intended can read and process it");
    let frequencies = frequency_analysis(plain_text);
    plot_distribution(frequencies);
}
