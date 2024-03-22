use fax::decoder::{decode_g4, pels};
use ppm::{BW, PPM};
mod ppm;

fn main() {
    for testcase in get_test_cases() {
        let bits = std::fs::read(format!("test-data/{}/image.dat", testcase)).unwrap();
        let dims_raw = std::fs::read_to_string(format!("test-data/{}/dims.txt", testcase)).unwrap();
        let ppm = std::fs::read(format!("test-data/{}/image.ppm", testcase)).unwrap();
        let width_height: Vec<usize> = dims_raw.split("x").into_iter().map(|s| s.trim().to_string().parse().unwrap()).collect();
        let width = width_height[0];
        let height = width_height[1];

        let mut new_lines = vec!();
        // eprintln!("Testing {}", testcase);

        // decode_g4(bits.clone().into_iter(), width as u16, None, |transitions| {
        //     // for item in pels(transitions, width as u16) {
        //     //     match item {
        //     //         fax::Color::Black => new_lines.push(BW::B),
        //     //         fax::Color::White => new_lines.push(BW::W),
        //     //     }
        //     // }
        // });

        decode_g4(bits.clone().into_iter(), width as u16, None, |transitions| {
            for item in pels(transitions, width as u16) {
                match item {
                    fax::Color::Black => new_lines.push(BW::B),
                    fax::Color::White => new_lines.push(BW::W),
                }
            }
        }).unwrap();

        if new_lines.len() != height * width {
            eprintln!("FAILED TO GET COHERENT FILE FOR {}", testcase);
            print_vec_as_binary(&bits);
            continue;
        }

        let new_ppm_struct = PPM::from_data(height as u32, width as u32, new_lines);
        let new_ppm = new_ppm_struct.to_file_bytes().unwrap();
        if ppm != new_ppm {
            new_ppm_struct.write_file(format!("test-data/{}/actual.ppm", testcase)).ok();
            eprintln!("DIFFERENCE FOUND FOR {}", testcase);
            print_vec_as_binary(&bits);
        } else {
            println!("Testing {: <60} Success!", testcase);
        }
    }
}

fn print_vec_as_binary(data: &[u8]) {
    for chunk in data.chunks(2) {
        match chunk {
            [byte1, byte2] => {
                println!("{:08b} {:08b}", byte1, byte2);
            }
            [byte] => {
                println!("{:08b}", byte);
            }
            _ => {}
        }
    }
}

fn get_test_cases() -> Vec<String>{
    // Specify the directory path
    let dir_path = "test-data";

    let mut data = vec!();
    // Read the contents of the directory
    if let Ok(entries) = std::fs::read_dir(dir_path) {
        // Iterate over the entries in the directory
        for entry in entries {
            if let Ok(entry) = entry {
                // Get the file name of the entry
                let file_name = entry.file_name();
                let file_name_str = file_name.to_string_lossy();

                // Check if the file matches the pattern ppm_xxxx.ppm
                if file_name_str.starts_with("tc_"){
                    data.push(file_name_str.to_string());
                }
            }
        }
    } else {
        panic!("Failed to read directory {}", dir_path);
    }
    return data;
}