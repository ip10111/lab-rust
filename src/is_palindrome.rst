fn main() {
    let txt: &str = "kasur ini rusak";

    if is_palindrome(txt) {
        println!("It's palindrome");
    } else {
        println!("It's not palindrome");
    }

    if is_palindrome_opt(txt) {
        println!("It's palindrome");
    } else {
        println!("It's not palindrome");
    }
}

fn is_palindrome(text: &str) -> bool {
    let reversed_text: String = text.to_string().chars().rev().collect();

    if text == reversed_text {
        return true;
    }

    false
}

fn is_palindrome_opt(text: &str) -> bool {
    // convert to array
    let bytes = text.as_bytes();

    // get array length
    let len = bytes.len();

    // check only halfway, because checking full length would be redundant
    for i in 0..len / 2 {
        // checking false first, so it doesn't take more processing.
        if bytes[i] != bytes[len - 1 - i] {
            return false;
        }
    }

    true
}
