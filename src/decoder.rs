use crate::{BitReader, ByteReader, Color, Transitions};
use crate::maps::{Mode, black, white, mode, EDFB_HALF, EOL};


fn with_markup<D, R>(decoder: D, reader: &mut R) -> Option<u16>
    where D: Fn(&mut R) -> Option<u16>
{
    let mut sum = 0;
    while let Some(n) = decoder(reader) {
        //print!("{} ", n);
        sum += n;
        if n < 64 {
            //println!("= {}", sum);
            return Some(sum)
        }
    }
    None
}

fn colored(current: Color, reader: &mut impl BitReader) -> Option<u16> {
    //print!("{:?} ", current);
    match current {
        Color::Black => with_markup(black::decode, reader),
        Color::White => with_markup(white::decode, reader),
    }
}

/// Turn a list of color changing position into an iterator of pixel colors
///
/// The width of the line/image has to be given in `width`.
/// The iterator will produce exactly that many items.
pub fn pels(line: &[u16], width: u16) -> impl Iterator<Item=Color> + '_ {
    use std::iter::repeat;
    let mut color = Color::White;
    let mut last = 0;
    let pad_color = if line.len() & 1 == 1 {
        !color
    } else { 
        color
    };
    line.iter().flat_map(move |&p| {
        let c = color;
        color = !color;
        let n = p.saturating_sub(last);
        last = p;
        repeat(c).take(n as usize)
    }).chain(repeat(pad_color)).take(width as usize)
}

/// Decode a Group 3 encoded image.
/// 
/// The callback `line_cb` is called for each decoded line.
/// The argument is the list of positions of color change, starting with white.
/// 
/// To obtain an iterator over the pixel colors, the `pels` function is provided.
pub fn decode_g3(input: impl Iterator<Item=u8>, mut line_cb: impl FnMut(&[u16])) -> Option<()> {
    let mut reader = ByteReader::new(input);
    let mut current = vec![];
    reader.expect(EOL).unwrap();
    
    'a: loop {
        let mut a0 = 0;
        let mut color = Color::White;
        while let Some(p) = colored(color, &mut reader) {
            a0 += p;
            current.push(a0);
            color = !color;
        }
        reader.expect(EOL).unwrap();
        line_cb(&current);
        current.clear();

        for _ in 0 .. 6 {
            if reader.peek(EOL.len) == Some(EOL.data) {
                reader.consume(EOL.len);
            } else {
                continue 'a;
            }
        }
        break;
    }
    Some(())
}


fn get_b1_b2(transitions: &[u16], color: Color, a0: u16) -> (u16, u16) {
    // last transition is always the off the page one
    if transitions.len() == 1 {
        return (transitions[0], transitions[0])
    } else if a0 == 0 && transitions[0] == 0 && color == Color::White {
        return (transitions[0], transitions[1])
    } else {
        let target = if color == Color::White {
            0
        } else {
            1
        };
        for (idx, transition) in transitions.iter().enumerate() {
            // if idx % 2 == target && (*transition > a0 || (*transition == 0 && a0 == 0 && target == 0)){
            if idx % 2 == target && *transition > a0 {
                let idx2 = if idx + 1 == transitions.len() { idx } else { idx + 1 };
                return (*transition, transitions[idx2])
            }
        }
        unreachable!()
    }
}

/// Decode a Group 4 Image
/// 
/// - `width` is the width of the image.
/// - The callback `line_cb` is called for each decoded line.
///   The argument is the list of positions of color change, starting with white.
/// 
///   If `height` is specified, at most that many lines will be decoded,
///   otherwise data is decoded until the end-of-block marker (or end of data).
/// 
/// To obtain an iterator over the pixel colors, the `pels` function is provided.
// pub fn decode_g4(input: impl Iterator<Item=u8>, width: u16, height: Option<u16>, mut line_cb: impl FnMut(&[u16])) -> Result<(), String> {
pub fn decode_g4_new(input: impl Iterator<Item=u8>, width: u16, height: Option<u16>, mut line_cb: impl FnMut(&[u16])) -> Result<(), String> {

    let mut reader = ByteReader::new(input);
    let mut reference: Vec<u16> = vec![width];
    let mut current: Vec<u16> = vec![];

    let limit = height.unwrap_or(u16::MAX);
    'outer: for _ in 0 .. limit {
        let mut a0 = 0;
        let mut color = Color::White;
        // println!(" - Starting Line");
        
        loop {
            let mode = match mode::decode(&mut reader) {
                Some(mode) => mode,
                None => {
                    return Ok(());
                    // return Err(format!("Unexpectedly could not read next code word"))
                },
            };
            // println!(" -  {:?}, color={:?}, a0={}, ref={:?}, transitions={:?}", mode, color, a0, reference, current);
            
            match mode {
                Mode::Pass => {
                    let (_b1, b2) = get_b1_b2(&reference, color, a0);
                    a0 = b2;        
                }
                Mode::Vertical(delta) => {
                    let (b1, _b2) = get_b1_b2(&reference, color, a0);
                    let a1 = (b1 as i16 + delta as i16) as u16;
                    current.push(a1);
                    color = !color;
                    a0 = a1;
                }
                Mode::Horizontal => {
                    let a0a1 = colored(color, &mut reader).unwrap();
                    let a1a2 = colored(!color, &mut reader).unwrap();
                    let a1 = a0 + a0a1;
                    let a2 = a1 + a1a2;
                    // println!("a0a1={}, a1a2={}, a1={}, a2={}", a0a1, a1a2, a1, a2);
                    current.push(a1);
                    current.push(a2);
                    a0 = a2;
                }
                Mode::Extension => {
                    let _xxx = reader.peek(3).unwrap();
                    println!("extension: {:03b}", _xxx);
                    reader.consume(3);
                    println!("{:?}", current);
                    break 'outer;
                }
            }

            // println!(" -     > {:?}", current);

            if a0 >= width {
                break;
            }
        }
        // println!(" -    => {:?}", current);

        line_cb(&current);
        std::mem::swap(&mut reference, &mut current);
        current.clear();
    }
    if height.is_none() {
        reader.expect(EDFB_HALF).ok().unwrap();
        reader.expect(EDFB_HALF).ok().unwrap();
    }
    //reader.print_remaining();

    Ok(())
}

/// Decode a Group 4 Image
/// 
/// - `width` is the width of the image.
/// - The callback `line_cb` is called for each decoded line.
///   The argument is the list of positions of color change, starting with white.
/// 
///   If `height` is specified, at most that many lines will be decoded,
///   otherwise data is decoded until the end-of-block marker (or end of data).
/// 
/// To obtain an iterator over the pixel colors, the `pels` function is provided.
pub fn decode_g4(input: impl Iterator<Item=u8>, width: u16, height: Option<u16>, mut line_cb: impl FnMut(&[u16])) -> Option<()> {
// pub fn decode_g4_old(input: impl Iterator<Item=u8>, width: u16, height: Option<u16>, mut line_cb: impl FnMut(&[u16])) -> Option<()> {
    let mut reader = ByteReader::new(input);
    let mut reference: Vec<u16> = vec![];
    let mut current: Vec<u16> = vec![];

    let limit = height.unwrap_or(u16::MAX);
    'outer: for _ in 0 .. limit {
        let mut transitions = Transitions::new(&reference);
        let mut a0 = 0;
        let mut color = Color::White;
        let mut start_of_row = true;
        //println!("\n\nline {}", y);
        
        loop {
            let mode = match mode::decode(&mut reader) {
                Some(mode) => mode,
                None => break 'outer,
            };
            println!("  {:?}, color={:?}, a0={}", mode, color, a0);
            
            match mode {
                Mode::Pass => {
                    let _ = transitions.next_color(a0, !color, false)?;
                    //println!("b1={}", b1);
                    if let Some(b2) = transitions.next() {
                        //println!("b2={}", b2);
                        a0 = b2;
                    }
                }
                Mode::Vertical(delta) => {
                    let b1 = transitions.next_color(a0, !color, start_of_row).unwrap_or(width);
                    let a1 = (b1 as i16 + delta as i16) as u16;
                    if a1 >= width {
                        break;
                    }
                    //println!("transition to {:?} at {}", !color, a1);
                    current.push(a1);
                    color = !color;
                    a0 = a1;
                    if delta < 0 {
                        transitions.seek_back(a0);
                    }
                }
                Mode::Horizontal => {
                    let a0a1 = colored(color, &mut reader)?;
                    let a1a2 = colored(!color, &mut reader)?;
                    let a1 = a0 + a0a1;
                    let a2 = a1 + a1a2;
                    //println!("a0a1={}, a1a2={}, a1={}, a2={}", a0a1, a1a2, a1, a2);
                    
                    current.push(a1);
                    if a2 >= width {
                        break;
                    }
                    current.push(a2);
                    a0 = a2;
                }
                Mode::Extension => {
                    let _xxx = reader.peek(3)?;
                    //println!("extension: {:03b}", xxx);
                    reader.consume(3);
                    //println!("{:?}", current);
                    break 'outer;
                }
            }
            start_of_row = false;

            if a0 >= width {
                break;
            }
        }
        //println!("{:?}", current);

        line_cb(&current);
        std::mem::swap(&mut reference, &mut current);
        current.clear();
    }
    if height.is_none() {
        reader.expect(EDFB_HALF).ok()?;
        reader.expect(EDFB_HALF).ok()?;
    }
    //reader.print_remaining();

    Some(())
}
