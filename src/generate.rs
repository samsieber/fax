use std::fs::{self, OpenOptions};
mod ppm;

use ppm::{PPM, BW};

fn main(){
    let directory_name = "test-data";
    // Check if the directory exists
    if !fs::metadata(directory_name).is_ok() {
        // If it doesn't exist, create it
        if let Err(err) = fs::create_dir(directory_name) {
            // If there's an error creating the directory, print an error message
            eprintln!("Error creating directory: {}", err);
        } else {
            println!("Directory '{}' created successfully!", directory_name);
        }
    } else {
        println!("Directory '{}' already exists.", directory_name);
    }

    l1_white(2);
    l1_white(10);
    l1_white(250);
    l1_white(2550);

    l1_black(2);
    l1_black(10);
    l1_black(250);
    l1_black(2550);


    l1_white_black(1, 1);
    l1_white_black(2, 7);
    l1_white_black(10, 5);
    l1_white_black(250, 15);
    l1_white_black(2550, 40);

    multiline_testcase(vec!(
        vec!((BW::B, 1), (BW::W, 200), (BW::B, 3)),
        vec!((BW::W, 100), (BW::B, 3), (BW::W, 101)),
    ), "l1-black-white-black_l2-white-black_1-200-3_100-3");

    multiline_testcase(vec!(
        vec!((BW::B, 1), (BW::W, 200), (BW::B, 3)),
        vec!((BW::W, 50), (BW::B, 3), (BW::W, 151)),
    ), "l1-black-white-black_l2-white-black_1-200-3_50-3");
    multiline_testcase(vec!(
        vec!((BW::B, 1), (BW::W, 203)),
        vec!((BW::W, 50), (BW::B, 3), (BW::W, 151)),
    ), "l1-black-white_l2-white-black_1-203_50-3");

    multiline_testcase(vec!(
        vec!((BW::B, 1), (BW::W, 20)),
        vec!((BW::W, 10), (BW::B, 3), (BW::W, 8)),
    ), "l1-black-white_l2-white-black_1-20_10-3");


    multiline_testcase(vec!(
        vec!((BW::W, 1), (BW::B, 200), (BW::W, 3)),
        vec!((BW::B, 100), (BW::W, 3), (BW::B, 101)),
    ), "l1-white-black-white_l2-black-white_1-200-3_100-3");

    multiline_testcase(vec!(
        vec!((BW::W, 2)),
        vec!((BW::B, 2)),
    ), "l1-white_l2-black-2");

    multiline_testcase(vec!(
        vec!((BW::W, 3)),
        vec!((BW::B, 3)),
    ), "l1-white_l2-black-3");
    
    multiline_testcase(vec!(
        vec!((BW::W, 4)),
        vec!((BW::B, 4)),
    ), "l1-white_l2-black-4");

    // multiline_testcase(vec!(
    //     vec!((BW::W, 1), (BW::W, 200), (BW::B, 3)),
    //     vec!((BW::W, 50), (BW::B, 3), (BW::W, 151)),
    // ), "l1-black-white-black_l2-white-black_1-200-3_50-3");
    // multiline_testcase(vec!(
    //     vec!((BW::B, 1), (BW::W, 203)),
    //     vec!((BW::W, 50), (BW::B, 3), (BW::W, 151)),
    // ), "l1-black-white_l2-white-black_1-203_50-3");

    // multiline_testcase(vec!(
    //     vec!((BW::B, 1), (BW::W, 20)),
    //     vec!((BW::W, 10), (BW::B, 3), (BW::W, 8)),
    // ), "l1-black-white_l2-white-black_1-20_10-3");
}

fn multiline_testcase(lines: Vec<Vec<(BW, usize)>>, name: impl AsRef<str>){
    let lines: Vec<Vec<BW>> = lines
        .into_iter()
        .map(|line| flatten(line.into_iter().map(|chunk| vec!(chunk.0; chunk.1)).collect()))
        .collect();
    let len = lines[0].len();
    for line in &lines {
        assert_eq!(line.len(), len);
    }
    let height = lines.len();
    let width = len;
    let data = flatten(lines);
    let ppm = PPM::from_data(height as u32, width as u32, data);
    write_test_case(ppm, name);
}


fn l1_white(len: u32) {
    write_test_case(PPM::from_data(1, len, vec![BW::W; len as usize]), format!("l1-white_{}", len));
}


fn l1_white_black(white: u32, black:u32) {
    let data = flatten(vec!(
        vec![BW::W; white as usize],
        vec![BW::B; black as usize]
    ));
    write_test_case(PPM::from_data(1, white+black, data), format!("l1-white-black_{}-{}", white, black));
}

fn flatten(data: Vec<Vec<BW>>) -> Vec<BW> {
    data.into_iter().flat_map(|v| v).collect()
}

fn l1_black(len: u32) {
    write_test_case(PPM::from_data(1, len, vec![BW::B; len as usize]), format!("l1-black_{}", len));
}

fn write_test_case(ppm: PPM, name: impl AsRef<str>){
    let directory_name = format!("test-data/tc_{}", name.as_ref());
    if !fs::metadata(&directory_name).is_ok() {
        // If it doesn't exist, create it
        if let Err(err) = fs::create_dir(&directory_name) {
            // If there's an error creating the directory, print an error message
            eprintln!("Error creating directory: {}", err);
        } else {
            println!("Directory '{}' created successfully!", &directory_name);
        }
    } else {
        println!("Directory '{}' already exists.", &directory_name);
    }
    std::fs::write(format!("test-data/tc_{}/dims.txt", name.as_ref()), format!("{}x{}", ppm.width, ppm.height)).unwrap();
    ppm.write_file(format!("test-data/tc_{}/image.ppm", name.as_ref())).unwrap()
}
