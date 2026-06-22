use std::io;
fn main() {
    loop {
        println!("Yeah, tell me whatchya got kid");
        let mut input_text = String::new();
        io::stdin()
        .read_line(&mut input_text)
        .expect("Yeah");

        let input: i32 = input_text.trim().parse().expect("Yeah");

        println!("time to choose operator yeah");
        let mut operator = String::new();
        io::stdin()
        .read_line(&mut operator)
        .expect("SHIT");
        let typed = operator.trim();
        

        println!("Well, time for the second digit");
        let mut input_text2: String = String::new();
        io::stdin() 
        .read_line(&mut input_text2)
        .expect("NOOOOO");

        let mut input2: i32 = input_text2.trim().parse().expect("Nuh uh");


        let result = match typed {
           "x"  => input * input2,
           "/" => input / input2,
           "+" => input + input2,
           "-" => input - input2,
           "*" => input * input2,
           _ => 0,
        };

           println!("{} {} {} = {}", input, typed, input2, result);
           break;
    }
}