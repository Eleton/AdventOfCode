use std::fs;
mod alpha;
mod beta;

pub type Row = String;

fn main() {
    let test1 = alpha::solve(read_data("example.txt"));
    if test1 == 1227775554 {
        println!("Test passed for alpha!");
        let res1 = alpha::solve(read_data("signal.txt"));
        println!("Result for alpha is: {:?}", res1);
        let test2 = beta::solve(read_data("example.txt"));
        if test2 == 265253940 {
            println!("Test passed for beta!");
            let res2 = beta::solve(read_data("signal.txt"));
            println!("Result for beta is: {:?}", res2);
        } else {
            println!("Beta test failed");
        }
    } else {
        println!("Alpha test failed");
    }
}

fn read_data(filename: &str) -> Vec<Row> {
    let string = fs::read_to_string(filename).expect("Could not read file");
    string
        .split(",")
        .filter_map(|x| x.parse::<Row>().ok())
        .collect()
}
