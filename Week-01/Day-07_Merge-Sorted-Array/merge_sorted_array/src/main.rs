fn is_sorted(v: &Vec<i32>) -> bool {
    if v.len() == 0 {
        return true;
    }
    // println!("is sorted?: {:?}", v);
    for i in 1..v.len() {
        // println!("{} - {}", v[i], v[i - 1]);
        if v[i] < v[i - 1] {
            return false;
        }
    }
    true
}

fn main() {
    loop {
        let mut input = String::new();
        std::io::stdin().read_line(&mut input).unwrap();
        let v1: Vec<i32> = input.trim().split_whitespace().map(|x| x.parse().unwrap()).collect();
        // println!("{:?}", v1);
        if !is_sorted(&v1) {
            panic!("First array must be sorted");
        }
        input.clear();
        std::io::stdin().read_line(&mut input).unwrap();
        let v2: Vec<i32> = input.trim().split_whitespace().map(|x| x.parse().unwrap()).collect();
        // println!("{:?}", v2);
        if !is_sorted(&v2) {
            panic!("Second array must be sorted");
        }

        let mut i = 0;
        let mut j = 0;
        let mut array: Vec<i32> = Vec::new();
        while i < v1.len() || j < v2.len() {
            if i == v1.len() {
                array.push(v2[j]);
                j += 1;
                continue;
            } else if i == v2.len() {
                array.push(v1[i]);
                i += 1;
                continue;
            }
            if v1[i] <= v2[j] {
                array.push(v1[i]);
                i += 1;
            } else {
                array.push(v2[j]);
                j += 1;
            }
        }
        println!("Sorted array: {:?}", array);
    }
}
