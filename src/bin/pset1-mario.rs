use dialoguer::Input;

fn main() {
    loop {
        let height: String = Input::new()
            .with_prompt("Input height: ")
            .interact_text()
            .unwrap();

        let height: u32 = match height.trim().parse() {
            Ok(height) => height,
            Err(_) => continue,
        };

        if height >= 9 || height < 1 {
            println!("Must be less than 10 rows!");
            continue;
        };

        for row in 0..height {
            let white_space = height - &row;
            for _ in 0..white_space {
                print!(" ");
            }

            let row = row + 1;
            for _ in 0..row {
                print!("{}", "#");
            }
            print!(" ");
            for _ in 0..row {
                print!("{}", "#");
            }
            println!("")
        }
        break;
    }
}
