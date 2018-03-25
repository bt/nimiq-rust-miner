extern crate argon2;
extern crate base64;
// extern crate flame;

fn main() {
    use argon2::{self, Config, ThreadMode, Variant, Version};
    use base64::{encode};
    // use std::fs::File;

    // Luna genesis block 170679, nonce 32239, target 1.7668201048317172e+72
    // Target: 0000000000000000111111111111111100000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000
    // PoW value: 0000000000000000001100010110011011001011100111010110110101000010010110100110110110010100110001100101011110101101000000101011000101010100000100110000111011010000111111111100101111111111110000011100000111011100000001001100110010001110101000010000101101011110
    let _serialized = &[0,2,66,39,165,85,2,86,34,163,150,89,34,243,189,19,27,116,79,21,244,80,241,39,85,210,66,232,116,109,192,79,156,64,230,110,96,109,8,182,166,17,75,245,217,138,1,239,55,113,79,51,62,124,218,52,82,200,247,66,38,153,160,84,18,157,37,240,30,10,7,227,235,40,162,147,243,63,201,82,166,223,114,7,11,223,247,8,95,80,180,4,35,213,240,198,164,221,18,128,104,122,189,117,196,188,105,109,130,141,141,195,161,181,247,176,176,238,58,214,114,32,64,81,42,69,12,196,63,244,31,0,255,255,0,2,154,183,90,183,220,206,0,0,125,239];
    let pow = "AAAxZsudbUJabZTGV60CsVQTDtD/y//BwdwEzI6hC14=";

    let mut header = [0,2,66,39,165,85,2,86,34,163,150,89,34,243,189,19,27,116,79,21,244,80,241,39,85,210,66,232,116,109,192,79,156,64,230,110,96,109,8,182,166,17,75,245,217,138,1,239,55,113,79,51,62,124,218,52,82,200,247,66,38,153,160,84,18,157,37,240,30,10,7,227,235,40,162,147,243,63,201,82,166,223,114,7,11,223,247,8,95,80,180,4,35,213,240,198,164,221,18,128,104,122,189,117,196,188,105,109,130,141,141,195,161,181,247,176,176,238,58,214,114,32,64,81,42,69,12,196,63,244,31,0,255,255,0,2,154,183,90,183,220,206,0,0,0,0];

    let salt = b"nimiqrocks!";
    let config = Config {
        variant: Variant::Argon2d,
        version: Version::Version13,
        mem_cost: 512,
        time_cost: 1,
        lanes: 1,
        thread_mode: ThreadMode::Parallel,
        secret: &[],
        ad: &[],
        hash_length: 32
    };

    header[142] = 0;
    header[143] = 0;
    header[144] = 0;
    header[145] = 0;

    let mut hash: Vec<u8>;

    loop {
        // flame::start("loop");
        // hash = flame::span_of("argon2::hash_raw", || argon2::hash_raw(&header, salt, &config).unwrap());
        hash = argon2::hash_raw(&header, salt, &config).unwrap();

        // let encoded_hash = flame::span_of("encode", || encode(&hash));
        let encoded_hash = encode(&hash);
        // flame::start("string comparison");
        if encoded_hash == pow {
            break;
        }
        // flame::end("string comparison");

        // Increment nonce
        // flame::start("increment nonce");
        if header[145] < 255 {
            header[145] = header[145] + 1;
            // if header[145] > 10 {
            //     break;
            // }
        }
        else if header[144] < 255 {
            println!("Hashes done: {}", (header[144] as u32 + 1) * 256);
            header[145] = 0;
            header[144] = header[144] + 1;
            if header[144] > 126 {
                println!("NOT FOUND!");
                break;
            }
        }
        else if header[143] < 255 {
            header[145] = 0;
            header[144] = 0;
            header[143] = header[143] + 1;
        }
        else if header[142] < 255 {
            header[145] = 0;
            header[144] = 0;
            header[143] = 0;
            header[142] = header[142] + 1;
        }
        else {
            break;
        }
        // flame::end("increment nonce");
        // flame::end("loop");
    }

    // flame::dump_html(&mut File::create("flame-graph.html").unwrap()).unwrap();

    println!("PoW hash of the original header: {}, target: {}", pow, 1.7668201048317172e+72);
    println!("Argon2 hash_raw:                 {}, value:  {}, nonce: {}", encode(&hash), 0, header[144] as u32 * 256 + header[145] as u32);

}
