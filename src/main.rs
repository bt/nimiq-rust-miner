extern crate argon2;
extern crate base64;
// extern crate flame;
extern crate u256;

fn main() {
    // #########################
    // ### START HEADER DATA ###
    // #########################

    // Luna block 170679, nonce 32239, target 1.7668201048317172e+72;

    // Javascript to generate target_u8:
    /*
        target_u8 = [];
        target = $.blockchain.head.header.target.toString(2);
        for (let i = 0; i < Math.ceil(target.length / 8); i++) {
            target_u8.push(parseInt(target.slice(8 * i, 8 * (i + 1)), 2));
        }
        while (target_u8.length < 32) {
            target_u8.unshift(0);
        }
        target_u8.toString()
    */
    let target_u8: &[u8] = &[0,0,255,255,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0];

    // [...$.blockchain.head.header.serialize()].toString()
    let serialized_header = [0,2,66,39,165,85,2,86,34,163,150,89,34,243,189,19,27,116,79,21,244,80,241,39,85,210,66,232,116,109,192,79,156,64,230,110,96,109,8,182,166,17,75,245,217,138,1,239,55,113,79,51,62,124,218,52,82,200,247,66,38,153,160,84,18,157,37,240,30,10,7,227,235,40,162,147,243,63,201,82,166,223,114,7,11,223,247,8,95,80,180,4,35,213,240,198,164,221,18,128,104,122,189,117,196,188,105,109,130,141,141,195,161,181,247,176,176,238,58,214,114,32,64,81,42,69,12,196,63,244,31,0,255,255,0,2,154,183,90,183,220,206,0,0,125,239];

    // $.blockchain.head.header.nonce
    let nonce = 32239;

    // #######################
    // ### END HEADER DATA ###
    // #######################

    use argon2::{self, Config, ThreadMode, Variant, Version};
    use std::time::{Instant};
    use base64::{encode};
    use u256::{U256};
    // use std::fs::File;

    // Config
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

    let mut header = serialized_header;

    // Set nonce to 0
    header[142] = 0;
    header[143] = 0;
    header[144] = 0;
    header[145] = 0;

    let target_u256 = U256::from(target_u8);
    // println!("Target value: {:?}", target_u256);

    // Working hash variable
    let mut hash: Vec<u8>;

    // Start time taking
    let start = Instant::now();

    loop {
        // flame::start("loop");

        // hash = flame::span_of("argon2::hash_raw", || argon2::hash_raw(&header, salt, &config).unwrap());
        hash = argon2::hash_raw(&header, salt, &config).unwrap();

        // flame::start("hash comparison");
        let hash_u256 = U256::from(&*hash);
        // println!("Hash value:   {:?}", hash_u256);

        if hash_u256 <= target_u256 {
            println!("");
            println!("SUCCESS: Hash value is smaller/equal than target! =)");
            break;
        }
        // flame::end("hash comparison");

        // Increment nonce
        // flame::start("increment nonce");
        if header[145] < 255 {
            header[145] = header[145] + 1;
        }
        else if header[144] < 255 {
            header[145] = 0;
            header[144] = header[144] + 1;

            if header[144] % 10 == 0 {
                let duration = start.elapsed();
                let duration = duration.as_secs() as f64 + duration.subsec_nanos() as f64 / 1_000_000_000.0;
                let hash_count = header[142] as u32 * 256 + header[143] as u32 * 256 + header[144] as u32 * 256 + header[145] as u32;
                let hash_rate = hash_count as f64 / duration;
                println!("Hashes done: {}, hashrate: {:.1} H/s", header[144] as u32 * 256, hash_rate);
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
            println!("No more nonces to try...");
            break;
        }
        // flame::end("increment nonce");

        // flame::end("loop");
    }

    // flame::dump_html(&mut File::create("flame-graph.html").unwrap()).unwrap();

    let found_nonce = header[142] as u32 * 256 + header[143] as u32 * 256 + header[144] as u32 * 256 + header[145] as u32;

    println!("");
    println!("Found hash:    {}, found nonce: {}, real nonce: {}", encode(&hash), found_nonce, nonce);
    println!("Found hash:    {:?}", hash);

}
