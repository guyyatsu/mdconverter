use std::env;
use std::fs;
extern crate markdown;

fn main() {

    // Collect input and output arguments from invocation.
    let args: Vec<String> = env::args().collect();  
    for argument in args.iter() {

        // Take action upon recieving POSIX style arguments.
        let flag = argument.split("=")
                           .collect();

        // Create a constant for each argument.
        if flag[0] == "--input" {
            let filepath = flag[1];
        }

        if flag[0] == "--output" {
            let output = flag[1];
        }
    }

    // Collect the name of the file for copying.
    let filename = filepath.split("\\")[-1]
                           .split(".")[0];

    // Read the input markdown file as source for conversion.
    let contents = fs::read_to_string(filepath).expect(
        "Idk whats up with that."
    );

    // Convert source to html.
    fs.write( "{output}\\{filename}.html",
              markdown::to_html(&contents) )
      .expect(
          "Could not write html to file"
      );
    
}