fn reverse_words(str: &str) -> String {
    let parts = str.split(" ");
    let mut finale = String::new();
    for part in parts {
        finale += &part.chars().rev().collect::<String>();
        finale += " ";
    }
    finale.trim_end().to_string()
}

fn main() {
    sample_test()
}

fn sample_test() {
    assert_eq!(reverse_words("The quick brown fox jumps over the lazy dog."), "ehT kciuq nworb xof spmuj revo eht yzal .god");
    assert_eq!(reverse_words("apple"), "elppa");
    assert_eq!(reverse_words("a b c d"),"a b c d");
    assert_eq!(reverse_words("double  spaced  words"), "elbuod  decaps  sdrow");
}