fn main() {
    let mut text = String::from("Пик Балмера");
    reverse_words_at_odd_positions(&mut text);
    println!("{text}");
}

fn reverse_words_at_odd_positions(_text: &mut String) {
    todo!("разверните нечётные слова на месте без аллокаций");
}

#[cfg(test)]
#[path = "../tests/unit/monologue.rs"]
mod monologue_tests;
