fn bin_to_dec(bin: String) -> u8 {
    let mut pos: u8 = 0;
    let mut res = 0;
    for bit in bin.chars().rev() {
        let bitint = bit.to_digit(10).unwrap() as u8;
        let pow = 2_u8.pow(pos.into());
        res += bitint * pow;
        pos += 1;
    }
    return res;
}

fn encode(input: String) -> String {
    let mut bitstr = String::new();
    //convert all chars in input to binary
    for byte in input.bytes() {
        for bit in (0..8).rev() {
            let res = (byte >> bit) & 1;
            bitstr = format!("{}{}", bitstr, res.to_string());
            // println!("{}", res);
        }
        // println!("");
    }
    //cut bitstream in blocs of 6
    let mut newbytes = Vec::new();
    let mut temp = String::new();
    let mut count = 1;
    for bit in bitstr.chars() {
        temp = format!("{}{}", temp, bit);
        if count % 6 == 0 {
            newbytes.push(temp);
            temp = String::new();
        }
        count += 1;
    }
    // if last block is not finished, append padding 0s
    if count % 6 != 1 {
        while temp.len() != 6 {
            temp = format!("{}{}", temp, 0);
        }
        newbytes.push(temp);
    }
    // println!("{:?}", newbytes);

    // convert bits back to decimal and then to base64's alphabet
    let mut output = String::new();
    for byte in newbytes {
        let new_chr_idx = bin_to_dec(byte);
        if new_chr_idx <= 25 {
            output = format!("{}{}", output, (new_chr_idx + 65) as char);
        } else if new_chr_idx >= 26 && new_chr_idx <= 51 {
            output = format!("{}{}", output, (new_chr_idx + 71) as char);
        } else if new_chr_idx >= 52 && new_chr_idx <= 61 {
            output = format!("{}{}", output, (new_chr_idx - 4) as char);
        } else if new_chr_idx == 62 {
            output = format!("{}{}", output, "+");
        } else if new_chr_idx == 63 {
            output = format!("{}{}", output, "/");
        }
        // println!("{}", new_chr_idx);
    }
    let mod_pad = input.len() % 3;
    if mod_pad != 0 {
        if mod_pad == 1 {
            output = format!("{}{}", output, "==");
        } else if mod_pad == 2 {
            output = format!("{}{}", output, "=");
        }
    }
    return output;
}

fn decode(input: String) -> String {
    let stripped = input.replace("=", "");
    // println!("{}", stripped);

    let mut bitstr = String::new();
    for byte in stripped.bytes() {
        let mut integer = 0;
        if byte >= 65 && byte <= 90 {
            integer = byte - 65;
        } else if byte >= 97 && byte <= 122 {
            integer = byte - 71;
        } else if byte >= 48 && byte <= 57 {
            integer = byte + 4;
        } else if byte == 43 {
            integer = 62
        } else if byte == 47 {
            integer = 63
        }

        //convert back to 6 bit binary repr
        for bit in (0..6).rev() {
            let res = (integer >> bit) & 1;
            bitstr = format!("{}{}", bitstr, res.to_string());
            // println!("{}", res);
        }
        // println!("{}", bitstr);
    }

    let mut newbytes = Vec::new();
    let mut temp = String::new();
    let mut count = 1;
    for bit in bitstr.chars() {
        temp = format!("{}{}", temp, bit);
        if count % 8 == 0 {
            newbytes.push(temp);
            temp = String::new();
        }
        count += 1;
    }
    // println!("{:?}", newbytes);

    let mut output = String::new();
    for byte in newbytes {
        let nbyte = bin_to_dec(byte);
        if nbyte != 0 {
            output = format!("{}{}", output, nbyte as char);
        }
    }
    return output;
}

fn ask_input() -> String {
    println!("Input : ");
    let mut input = String::new();
    let _ = std::io::stdin().read_line(&mut input);
    if input.len() == 0 {
        println!("No input given!");
    }
    return input;
}

fn main() {
    let flag = match std::env::args().nth(1) {
        Some(s) => s,
        None => {
            println!("No flag given");
            return;
        }
    };
    let mut output = String::new();
    if flag == "-e" {
        let input = ask_input();
        if input.len() == 0 {
            return;
        }
        output = encode(input);
    } else if flag == "-d" {
        let input = ask_input();
        if input.len() == 0 {
            return;
        }
        output = decode(input);
    } else {
        println!("Unknown flag!");
        return;
    }
    println!("Output : {}", output);
}
