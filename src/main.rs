use std::io;

fn main() {
    println!("Temperature Converter: ");
    println!("1. Fahrenheit to Celsius");
    println!("2. Celsius to Fahrenheit");
    println!("3. Quit");
    println!("Input choose: ");
    let mut x = String::new();
    io::stdin().read_line(&mut x).expect("Failed to read");
    let x = match x.trim().parse(){
        Ok(num) => num,
        Err(_) => 3,
    };
    if x != 1 && x != 2 {
        println!("The input choose is invalid, please try again");
    } else {
        loop{
            println!("Input temperature:");
            let mut temp = String::new();
            io::stdin().read_line(&mut temp).expect("Failed to read");
            let temp: f64 = match temp.trim().parse() {
                Ok(v) => v,
                Err(_) => {
                    println!("Cannot parse temperature");
                    continue;
                }
            };
            let ans = if x == 1 {
                f_to_c(temp)
            } else {
                c_to_f(temp)
            };

            println!("Temperature: {}", ans);
            io::stdin().read_line(&mut String::new()).unwrap();
            break;
        }


    }
}


fn f_to_c(fahrenheit: f64) -> f64{
    (fahrenheit-32.0)*5.0/9.0
}

fn c_to_f(celsius: f64) -> f64{
    celsius*9.0/5.0+32.0
}