use std::fs::File;
use std::io;

use std::io::prelude::*;

static mut FIRST_RUN: bool = true;

fn main() {
    unsafe {
        if FIRST_RUN {
            FIRST_RUN = false;
            println!("SMD Fixer v2.0.8 by Ali Deym\n");
        }
    }
    

    println!("Enter file name (or exit): ");

    let mut filename = String::new();

    io::stdin().read_line(&mut filename).unwrap();

    filename = String::from(filename.trim_end());

    if filename.trim_end() == "exit" {
        return;
    }

    let content = std::fs::read_to_string(&filename).unwrap();

    let iter = content.split("\n");
    let iter_clone = iter.clone();

    for item in iter_clone {
        let firstword = item.split_ascii_whitespace().next().unwrap_or("");

        let n = firstword.parse::<i32>().unwrap_or(-1);

        
        if n >= 0 {
            println!("{}", item);
        } else if firstword.trim_end().eq_ignore_ascii_case("end") {
            break;
        }

        
    }

    let mut bone_num = String::new();
    let mut bone = Vec::new();

    
    
    while bone_num.trim_end() != "end" {
        bone_num.clear();
        println!("Enter bone(s) number or end to finish: ");

        io::stdin().read_line(&mut bone_num).unwrap();

        let bn = bone_num.trim_end().parse::<i32>().unwrap_or(-1);

        if bn >= 0 {
            bone.push(bn);
        }

    }

    


    let mut new_content = Vec::new();

    let mut on_frame = false;
    for item in iter {
        if item.trim().is_empty() {
            continue;
        }

        if item.to_lowercase().starts_with("time ") {
            on_frame = true;
            new_content.push(item.trim_end().replace("\r", "").replace("\n", ""));

            continue;
        }

        let firstword = item.split_ascii_whitespace().next().unwrap_or("");

        let n = firstword.parse::<i32>().unwrap_or(-1);


        if bone.contains(&n) {
            new_content.push(item.trim_end().replace("\r", "").replace("\n", ""));
        }

        if !on_frame && n < 0 {
            new_content.push(item.trim_end().replace("\r", "").replace("\n", ""));
        }
    }

    new_content.push(String::from("end"));

    {
        std::fs::remove_file(&filename).unwrap();
        let mut f = File::create(&filename).unwrap();

        let joined = new_content.join("\r\n");

        f.write_all(joined.as_bytes()).unwrap();

        f.flush().unwrap();
    }

    main();
}
