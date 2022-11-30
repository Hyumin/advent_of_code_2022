
use std::io;
use std::vec;
use std::{fs, io::Result};

//Reads the lines from the terminal and return after entering done identifier
pub fn get_input( done_iden : &String) -> String
{
    let mut input_strings = String::new();

    //Type custom input in the terminal to see how the function behaves with different input
    println!("Custom input, Please enter input in the terminal to create a string to iterate through. When done type {}",done_iden);
    loop
    {
      let mut text = String::new();
  
      io::stdin().read_line(&mut text)
      .ok()
      .expect("Failed to read line");
  
      //Compare input text with doneidentifier, if true end loop
      if text.trim() == done_iden
      {
          break;
      }
  
      input_strings+= text.trim();
      input_strings+= "\n";
    }

    return input_strings;
}

pub fn convert_to_uintegers(arg: &String)  ->Vec<u64>
{
  let mut return_value = Vec::new();
  for line in arg.lines()
  {
      let number : u64 = match line.trim().parse::<u64>() 
      {
          Ok(number) => number,
          Err(_) => continue,
      };

      return_value.push(number);
  }

  return return_value;
}
pub fn convert_to_uintegers_32(arg: &String)  ->Vec<u32>
{
  let mut return_value = Vec::new();
  for line in arg.lines()
  {
      let number : u32 = match line.trim().parse::<u32>() 
      {
          Ok(number) => number,
          Err(_) => continue,
      };

      return_value.push(number);
  }

  return return_value;
}

pub fn get_input_from_filename( filepath : &String) ->String
{
  let file_content = fs::read_to_string(filepath);

  match file_content
  {
    Ok(_)=> (),
    Err(e)=> {
      eprintln!("Error: {}",e);
      return String::from(" ");
    },
  }

  return file_content.unwrap();
}