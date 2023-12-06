fn find_nemo(t: &[&str]) -> Option<usize> {
    for (i, word) in t.iter().enumerate() {
        if *word == "Nemo" {
            return Some(i+1);
        }
    }
    None
}

fn main(){
    let sentence = "I Nemo am !";
    let words: Vec<&str> = sentence.split_whitespace().collect();
    match find_nemo(&words) {
        Some(index) => println!("Found Nemo at index : {}", index),
        None => println!("Nemo not found"),
    }
}