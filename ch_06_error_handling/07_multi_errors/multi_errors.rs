//! Handling multiple errors

type Result<T> = std::result::Result<T, String>;

fn double_first(vec: Vec<&str>) -> Result<i32> {
    vec.first()
        .ok_or("Please use a vec w/ at least one element".to_owned())
        .and_then(
            |s| s.parse::<i32>()
            .map_err(|e| e.to_string())
            .map(|i| i * 2)
        )
}

fn print(result: Result<i32>) {
    match result {
        Ok(n) => println!("The first doubled is {}", n),
        Err(e) => println!("Error: {}", e),
    }
}

fn main() {
    let empty = vec![];
    let strings = vec!["tofu", "93", "18"];
    let nums = vec!["100", "93", "18"];

    print(double_first(empty));
    print(double_first(strings));
    print(double_first(nums));
}