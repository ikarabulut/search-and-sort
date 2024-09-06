pub fn sort(mut list: Vec<i32>) -> Vec<i32> {
    let n = list.len();
    let result = loop {
        let mut swapped = false;
        for i in 1..n {
            if list[i - 1] > list[i] {
                list.swap(i - 1, i);
                swapped = true;
            }
        }
        if !swapped {
            break list;
        }
    };
    result
}