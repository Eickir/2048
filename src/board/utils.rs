pub fn count_non_zero_value(array: &Vec<u32>) -> u32 {

    let mut occurences: u32 = 0;
    for element in array {
        if *element > 0 {
            occurences += 1;
        }
    }

    occurences

} 

pub fn zero_position(array: &Vec<u32>) -> u32 {

    array.iter().position(|&x|x==0).unwrap_or(4) as u32

} 

pub fn trailing_zero_rows(array: &Vec<u8>) -> bool {

    let result: u8 = array.into_iter().sum();
    if result < 4 {
        return false;
    }
    true
}
