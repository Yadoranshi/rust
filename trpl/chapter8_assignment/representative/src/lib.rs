fn sum(v: &Vec<usize>) -> usize {
    let mut count = 0;
    for i in v {
        count += v[*i];
    }
    count
}

fn average(v: &Vec<usize>) -> usize {
    sum(v)/v.len()
}

