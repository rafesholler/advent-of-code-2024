use std::fs;

fn main() {
    // Part 1
    let input = fs::read_to_string("input.txt").unwrap();
    let mut list1: Vec<i32> = vec![];
    let mut list2: Vec<i32> = vec![];
    for line in input.lines() {
        let mut vals = line.split_whitespace();
        list1.push(vals.next().unwrap().parse().unwrap());
        list2.push(vals.next().unwrap().parse().unwrap());
    }


    let mut sum = 0;
    while list1.len() > 0 {
        let mut min1 = list1[0];
        let mut min2 = list2[0];
        let mut i1 = 0;
        let mut i2 = 0;
        for i in 1..list1.len() {
            if list1[i] < min1 {
                min1 = list1[i];
                i1 = i;
            }
            if list2[i] < min2 {
                min2 = list2[i];
                i2 = i;
            }
        }

        sum += (min1 - min2).abs();
        list1.remove(i1);
        list2.remove(i2);
    }
    println!("{}", sum);

    // Part 2
    list1.clear();
    list2.clear();
    for line in input.lines() {
        let mut vals = line.split_whitespace();
        list1.push(vals.next().unwrap().parse().unwrap());
        list2.push(vals.next().unwrap().parse().unwrap());
    }

    let mut sum2 = 0;
    for i in list1 {
        let mut count = 0;
        for j in &list2 {
            if i == *j {
                count += 1;
            }
        }
        sum2 += i * count;
    }
    println!("{}", sum2);
}
