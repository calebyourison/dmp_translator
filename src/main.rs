pub mod definitions;
pub mod translation_functions;
pub mod utilities;

use crate::utilities::valid_sample_events;
use crate::translation_functions::translate_dmp_event;


fn main() {

run_program();

}

fn clr() {print!("{}[2J", 27 as char);} 

fn process_sample_events() {  
    clr();  
    for event in valid_sample_events() {
        println!("{event}");
        println!("{}", translate_dmp_event(event));
    };

}

fn run_program() {
    clr();


    loop {
    
        println!("----------");
        println!("DMP Serial 3 formatted events are structured as so: Za\\060\\t “BU\\z 001“OFFICE\\...\\");
        println!("If your event has leading account information, please omit this. \n");
    
        println!("Please input DMP Serial 3 event for translation, enter 1 to print a list of sample events, or enter 2 to quit");

        let mut choice = String::new();
        std::io::stdin().read_line(&mut choice).unwrap();


        if choice.trim() == "1" {
            
            process_sample_events();
            
            } else if choice.trim() == "2" {
            
                break
            
            } else {
                
                println!("\nRaw event: {}", choice);
                println!("{} \n", translate_dmp_event(choice.trim().to_string()));
            }
        }
}