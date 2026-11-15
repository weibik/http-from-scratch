fn main() {
    let input = "hello big world";
    println!("{}", reverse_words(input));
    println!("{}", count_chars("hello world", 'o'));
    println!("{:?}", split_pair("name=Anna"));
    println!("{:?}", split_pair("nothing here"));
}

fn reverse_words(input: &str) -> String {
    let mut words: Vec<&str> = input.split_whitespace().collect();
    words.reverse();
    words.join(" ")
}

fn count_chars(input: &str, target: char) -> usize {
    let mut count = 0;
    for c in input.chars() {
        if c == target {
            count += 1;
        }
    }
    count
}

fn split_pair(input: &str) -> Option<(&str, &str)> {
    input.split_once('=')
}
