fn main() {
    let mut similarity = 0;
    let mut total_distance: i32 = 0;
    let mut left_most: Vec<i32> = Vec::new();
    let mut right_most: Vec<i32> = Vec::new();

    let contents: String = std::fs::read_to_string("src/input.txt")
    .expect("Should have been able to read the file");
    for line in contents.lines(){
        //Splits the &str into two parts
        let values: Vec<&str> = line.split("   ").collect();
        //Parse the &str to a i32
        left_most.push(values.first().unwrap().parse().expect("Failed to convert to number"));
        right_most.push(values.last().unwrap().parse().expect("Failed to convert to number"))
    }
    left_most.sort();
    right_most.sort();
    //Part 1 get total distance
    for values in left_most.iter().zip(right_most.iter()) {
        let (left, right) = values;
        let result = (left - right).abs();

        total_distance += result;
    }
    //Part 2 get similarity
    for value in left_most.iter() {
        let times_found_in_right_list = right_most.iter().filter(|&n| *n == *value).count();

        let result = value * times_found_in_right_list as i32;
        similarity += result;
    }
    println!("Total distance: {total_distance}\nSimilarity: {similarity}");
}
