extern crate isochronous_finite_fields;
use isochronous_finite_fields::GF;

use std::env;
// Calculation of SubBytes
fn byte_sub(message: &mut u128){
    //S-box
	let table:[[u8;16];16] =[[0x63,0x7c,0x77,0x7b,0xf2,0x6b,0x6f,0xc5,0x30,0x01,0x67,0x2b,0xfe,0xd7,0xab,0x76],
				             [0xca,0x82,0xc9,0x7d,0xfa,0x59,0x47,0xf0,0xad,0xd4,0xa2,0xaf,0x9c,0xa4,0x72,0xc0],
				             [0xb7,0xfd,0x93,0x26,0x36,0x3f,0xf7,0xcc,0x34,0xa5,0xe5,0xf1,0x71,0xd8,0x31,0x15],
				             [0x04,0xc7,0x23,0xc3,0x18,0x96,0x05,0x9a,0x07,0x12,0x80,0xe2,0xeb,0x27,0xb2,0x75],
				             [0x09,0x83,0x2c,0x1a,0x1b,0x6e,0x5a,0xa0,0x52,0x3b,0xd6,0xb3,0x29,0xe3,0x2f,0x84],
				             [0x53,0xd1,0x00,0xed,0x20,0xfc,0xb1,0x5b,0x6a,0xcb,0xbe,0x39,0x4a,0x4c,0x58,0xcf],
				             [0xd0,0xef,0xaa,0xfb,0x43,0x4d,0x33,0x85,0x45,0xf9,0x02,0x7f,0x50,0x3c,0x9f,0xa8],
				             [0x51,0xa3,0x40,0x8f,0x92,0x9d,0x38,0xf5,0xbc,0xb6,0xda,0x21,0x10,0xff,0xf3,0xd2],
				             [0xcd,0x0c,0x13,0xec,0x5f,0x97,0x44,0x17,0xc4,0xa7,0x7e,0x3d,0x64,0x5d,0x19,0x73],
				             [0x60,0x81,0x4f,0xdc,0x22,0x2a,0x90,0x88,0x46,0xee,0xb8,0x14,0xde,0x5e,0x0b,0xdb],
				             [0xe0,0x32,0x3a,0x0a,0x49,0x06,0x24,0x5c,0xc2,0xd3,0xac,0x62,0x91,0x95,0xe4,0x79],
				             [0xe7,0xc8,0x37,0x6d,0x8d,0xd5,0x4e,0xa9,0x6c,0x56,0xf4,0xea,0x65,0x7a,0xae,0x08],
				             [0xba,0x78,0x25,0x2e,0x1c,0xa6,0xb4,0xc6,0xe8,0xdd,0x74,0x1f,0x4b,0xbd,0x8b,0x8a],
				             [0x70,0x3e,0xb5,0x66,0x48,0x03,0xf6,0x0e,0x61,0x35,0x57,0xb9,0x86,0xc1,0x1d,0x9e],
				             [0xe1,0xf8,0x98,0x11,0x69,0xd9,0x8e,0x94,0x9b,0x1e,0x87,0xe9,0xce,0x55,0x28,0xdf],
				             [0x8c,0xa1,0x89,0x0d,0xbf,0xe6,0x42,0x68,0x41,0x99,0x2d,0x0f,0xb0,0x54,0xbb,0x16]];
    
	let mut new_message = 0x0000000000000000000000000000000000;
    //Take each hex value from recived message and search replacement from table
	for i in 0..16{
        let variable:u8;
        let coordinate1:usize;
        let coordinate2:usize;
        variable = ((*message << (4*(2*i)))>> 120) as u8;
        //First charcter from hex value
        match (variable << 0) >>4 {
            0x0=>coordinate1=0,
            0x1=>coordinate1=1,
            0x2=>coordinate1=2,
            0x3=>coordinate1=3,
            0x4=>coordinate1=4,
            0x5=>coordinate1=5,
            0x6=>coordinate1=6,
            0x7=>coordinate1=7,
            0x8=>coordinate1=8,
            0x9=>coordinate1=9,
            0xa=>coordinate1=10,
            0xb=>coordinate1=11,
            0xc=>coordinate1=12,
            0xd=>coordinate1=13,
            0xe=>coordinate1=14,
            0xf=>coordinate1=15,
            16_u8..=u8::MAX => todo!(),
        }
        //Second charcter from hex value
        match (variable << 4) >>4 {
            0x0=>coordinate2=0,
            0x1=>coordinate2=1,
            0x2=>coordinate2=2,
            0x3=>coordinate2=3,
            0x4=>coordinate2=4,
            0x5=>coordinate2=5,
            0x6=>coordinate2=6,
            0x7=>coordinate2=7,
            0x8=>coordinate2=8,
            0x9=>coordinate2=9,
            0xa=>coordinate2=10,
            0xb=>coordinate2=11,
            0xc=>coordinate2=12,
            0xd=>coordinate2=13,
            0xe=>coordinate2=14,
            0xf=>coordinate2=15,
            16_u8..=u8::MAX => todo!(),
        }
        //Retrive from table hex value by coordinates and put them in new message
        if i == 0 {
            new_message = (table[coordinate1][coordinate2] as u128) << 120;
        }
        else{
            new_message = new_message | (table[coordinate1][coordinate2] as u128) << 128-8*(i+1);
        }
	}
    //Send new created message
    *message = new_message;

}

//Shifting rows in message
fn shift_row(message: &mut u128){
    //First row of messaeg, do not shift
    let first = *message>> 96;

    //Second row of messaeg
    let mut second = (*message<<32)>> 96;
    //Left part of new message which take value from right side
    let second_left = (second<< 8) as u32 >>8;
    //Right part of new message which take value from left side
    let second_right = (second >>24) as u32;
    //Combine new row of message
    second = ((second_left << 8) |second_right) as u128;

    //Third row of messaeg
    let mut third = (*message<<64)>> 96;
    let third_left = (third<< 16) as u32 >>16;
    let third_right = (third >>16) as u32;
    third = ((third_left <<16) |third_right) as u128;

    //Fourth row of messaeg
    let mut fourth = (*message<<96)>> 96;
    let fourth_left = (fourth<< 24) as u32 >>24;
    let fourth_right = (fourth >>8) as u32;
    fourth = ((fourth_left <<24) |fourth_right) as u128;

    //Combine all shifted rows
    *message = first << 96|second<<64|third<<32|fourth;
}

//Mixing columns of message by multipling them with predefined matrix
fn mix_columns(message: &mut u128){
    let mut counter = 0;
	let mut new_message :u128= 0x00000000000000000000000000000000;
    //Predefined matrix
    let table:u128=0x02030101010203010101020303010102;
    let first = (*message>> 96) as u32;
    let second = ((*message<<32)>> 96) as u32;
    let third = ((*message<<64)>> 96) as u32;
    let fourth = ((*message<<96)>> 96) as u32;
    //Program split predefined table to rows
    for j in 0..4{
        let mut variable;
        let key :u32 = ((table <<32*j)>>96) as u32;
        let mut key_first :u8 = ((key<<0)>>24) as u8;
        let mut key_second :u8 = ((key<<8)>>24) as u8;
        let mut key_third :u8 = ((key<<16)>>24) as u8;
        let mut key_fourth :u8 = ((key<<24)>>24) as u8;
        //Take each column and multipy it with rows of predefined table
        for i in 0..4 {
            let mut first_element = ((first <<8*i)>>24) as u8;
            let mut second_element = ((second <<8*i)>>24) as u8;
            let mut third_element = ((third <<8*i)>>24) as u8;
            let mut fourth_element = ((fourth <<8*i)>>24) as u8;
            // For multipling each hex value from both table need to use Finite field arithmetic
            // taken from recouce https://medium.com/asecuritysite-when-bob-met-alice/galois-field-gf-2%E2%81%B8-and-rust-503aa218f476
            let args1: Vec<String>   = env::args().collect();
  
            if args1.len()> 1 { first_element= args1[1].clone().parse::<u8>().unwrap();}
            if args1.len()> 2 { key_first = args1[2].clone().parse::<u8>().unwrap(); }

            let res1 = GF(first_element) * GF(key_first);

            // stop here

            let args2: Vec<String> = env::args().collect();
  
            if args2.len()> 1 { second_element= args2[1].clone().parse::<u8>().unwrap();}
            if args2.len()> 2 { key_second = args2[2].clone().parse::<u8>().unwrap(); }

            let res2 = GF(second_element) * GF(key_second);

            let args3: Vec<String> = env::args().collect();
  
            if args3.len()> 1 { third_element= args3[1].clone().parse::<u8>().unwrap();}
            if args3.len()> 2 { key_third = args3[2].clone().parse::<u8>().unwrap(); }

            let res3 = GF(third_element) * GF(key_third);

            let args4: Vec<String> = env::args().collect();
  
            if args4.len()> 1 { fourth_element= args4[1].clone().parse::<u8>().unwrap();}
            if args4.len()> 2 { key_fourth = args4[2].clone().parse::<u8>().unwrap(); }

            let res4 = GF(fourth_element) * GF(key_fourth);

            //Xor recived resuts, results would be placed as rows
            variable = res1.0 ^ res2.0^res3.0^res4.0;
            counter +=1;
            new_message = new_message|(variable as u128) <<128-8*counter;
        }
        *message = new_message;
    }
}
//Xoring key and message
fn xor(message: &mut u128,key:u128){
    *message^=key;
}
//Each time before xoring need to do Key Expation, becasue we already have k10 need to do backward process
fn inverse_key_expansion(key:u128,key_list: &mut Vec<u128>){
    //Reverse S-box table
    let table:[[u8;16];16] =[[0x52,0x09,0x6a,0xd5,0x30,0x36,0xa5,0x38,0xbf,0x40,0xa3,0x9e,0x81,0xf3,0xd7,0xfb],
				             [0x7c,0xe3,0x39,0x82,0x9b,0x2f,0xff,0x87,0x34,0x8e,0x43,0x44,0xc4,0xde,0xe9,0xcb],
				             [0x54,0x7b,0x94,0x32,0xa6,0xc2,0x23,0x3d,0xee,0x4c,0x95,0x0b,0x42,0xfa,0xc3,0x4e],
				             [0x08,0x2e,0xa1,0x66,0x28,0xd9,0x24,0xb2,0x76,0x5b,0xa2,0x49,0x6d,0x8b,0xd1,0x25],
				             [0x72,0xf8,0xf6,0x64,0x86,0x68,0x98,0x16,0xd4,0xa4,0x5c,0xcc,0x5d,0x65,0xb6,0x92],
				             [0x6c,0x70,0x48,0x50,0xfd,0xed,0xb9,0xda,0x5e,0x15,0x46,0x57,0xa7,0x8d,0x9d,0x84],
				             [0x90,0xd8,0xab,0x00,0x8c,0xbc,0xd3,0x0a,0xf7,0xe4,0x58,0x05,0xb8,0xb3,0x45,0x06],
				             [0xd0,0x2c,0x1e,0x8f,0xca,0x3f,0x0f,0x02,0xc1,0xaf,0xbd,0x03,0x01,0x13,0x8a,0x6b],
				             [0x3a,0x91,0x11,0x41,0x4f,0x67,0xdc,0xea,0x97,0xf2,0xcf,0xce,0xf0,0xb4,0xe6,0x73],
				             [0x96,0xac,0x74,0x22,0xe7,0xad,0x35,0x85,0xe2,0xf9,0x37,0xe8,0x1c,0x75,0xdf,0x6e],
				             [0x47,0xf1,0x1a,0x71,0x1d,0x29,0xc5,0x89,0x6f,0xb7,0x62,0x0e,0xaa,0x18,0xbe,0x1b],
				             [0xfc,0x56,0x3e,0x4b,0xc6,0xd2,0x79,0x20,0x9a,0xdb,0xc0,0xfe,0x78,0xcd,0x5a,0xf4],
				             [0x1f,0xdd,0xa8,0x33,0x88,0x07,0xc7,0x31,0xb1,0x12,0x10,0x59,0x27,0x80,0xec,0x5f],
				             [0x60,0x51,0x7f,0xa9,0x19,0xb5,0x4a,0x0d,0x2d,0xe5,0x7a,0x9f,0x93,0xc9,0x9c,0xef],
				             [0xa0,0xe0,0x3b,0x4d,0xae,0x2a,0xf5,0xb0,0xc8,0xeb,0xbb,0x3c,0x83,0x53,0x99,0x61],
				             [0x17,0x2b,0x04,0x7e,0xba,0x77,0xd6,0x26,0xe1,0x69,0x14,0x63,0x55,0x21,0x0c,0x7d]];
    //Split given key
    let first = (key>> 96) as u32;
    let second = ((key<<32)>> 96) as u32;
    let third = ((key<<64)>> 96) as u32;
    let fourth = ((key<<96)>> 96) as u32;
    let mut key_one:u32 = 0x00000000;
    let mut key_two:u32 = 0x00000000;
    let mut key_three:u32 = 0x00000000;
    let mut key_fourth:u32 = 0x00000000;
    let mut new_message = 0x00000000;
    //Becasue we need to take values in columns, not in rows, from key, programm create columns of hex values from key 
    for i in 0..4{
        let mut first_element = ((first <<8*i)>>24) as u8;
        let mut second_element = ((second <<8*i)>>24) as u8;
        let mut third_element = ((third <<8*i)>>24) as u8;
        let mut fourth_element = ((fourth <<8*i)>>24) as u8;
        if i == 0{
            key_one = (first_element as u32) << 24 | (second_element as u32) << 16 | (third_element as u32) << 8|(fourth_element as u32);
        }
        else if i == 1{
            key_two = (first_element as u32) << 24 | (second_element as u32) << 16 | (third_element as u32) << 8|(fourth_element as u32);
        }
        else if i == 2{
            key_three = (first_element as u32) << 24 | (second_element as u32) << 16 | (third_element as u32) << 8|(fourth_element as u32);
        }
        else if i == 3{
            key_fourth = (first_element as u32) << 24 | (second_element as u32) << 16 | (third_element as u32) << 8|(fourth_element as u32);
        }
    }
    //In Key Expantion first column need to xor with g of last column, 
    //then each next column need to xor with previouse column of new key column.

    //g is defined as replasing values from S-box
    //Here we create bacward proccess, xor last column of new key and prviouse column new key.
    //For final coulmn we take last coulm of reverse key and replcing value from reverse S-box and xor result.
    let xor_value_fourth = key_fourth ^ key_three;
    let xor_value_three = key_three ^ key_two;
    let xor_value_two = key_two ^ key_one;

    //Replacing values for last colum of reverse key
    for i in 0..4{
        let variable:u8;
        let coordinate1:usize;
        let coordinate2:usize;
        variable = ((xor_value_fourth << (4*(2*i)))>> 24) as u8;
        match (variable << 0) >>4 {
            0x0=>coordinate1=0,
            0x1=>coordinate1=1,
            0x2=>coordinate1=2,
            0x3=>coordinate1=3,
            0x4=>coordinate1=4,
            0x5=>coordinate1=5,
            0x6=>coordinate1=6,
            0x7=>coordinate1=7,
            0x8=>coordinate1=8,
            0x9=>coordinate1=9,
            0xa=>coordinate1=10,
            0xb=>coordinate1=11,
            0xc=>coordinate1=12,
            0xd=>coordinate1=13,
            0xe=>coordinate1=14,
            0xf=>coordinate1=15,
            16_u8..=u8::MAX => todo!(),
        }
        
        match (variable << 4) >>4 {
            0x0=>coordinate2=0,
            0x1=>coordinate2=1,
            0x2=>coordinate2=2,
            0x3=>coordinate2=3,
            0x4=>coordinate2=4,
            0x5=>coordinate2=5,
            0x6=>coordinate2=6,
            0x7=>coordinate2=7,
            0x8=>coordinate2=8,
            0x9=>coordinate2=9,
            0xa=>coordinate2=10,
            0xb=>coordinate2=11,
            0xc=>coordinate2=12,
            0xd=>coordinate2=13,
            0xe=>coordinate2=14,
            0xf=>coordinate2=15,
            16_u8..=u8::MAX => todo!(),
        }
        
        if i == 0 {
            new_message = (table[coordinate1][coordinate2] as u32) << 24;
        }
        else{
            new_message = new_message | (table[coordinate1][coordinate2] as u32) << 32-8*(i+1);
        }
	}
    let xor_value_one = key_one ^ new_message;
    let mut new_key:u128 = 0x000000000000000000000000000000000000;
    //Combine together new key
    new_key = new_key|((((xor_value_one <<0)>>24) as u8) as u128)<<120 |((((xor_value_two <<0)>>24) as u8) as u128)<<112
                          |((((xor_value_three <<0)>>24) as u8) as u128)<<106|((((xor_value_fourth <<0)>>24) as u8) as u128)<<96
                          |((((xor_value_one <<8)>>24) as u8) as u128)<<88|((((xor_value_two <<8)>>24) as u8) as u128)<<80
                          |((((xor_value_three <<8)>>24) as u8) as u128)<<72|((((xor_value_fourth <<8)>>24) as u8) as u128)<<64
                          |((((xor_value_one <<16)>>24) as u8) as u128)<<56|((((xor_value_two <<16)>>24) as u8) as u128)<<48
                          |((((xor_value_three <<16)>>24) as u8) as u128)<<40|((((xor_value_fourth <<16)>>24) as u8) as u128)<<32
                          |((((xor_value_one <<24)>>24) as u8) as u128)<<24|((((xor_value_two <<24)>>24) as u8) as u128)<<16
                          |((((xor_value_three <<24)>>24) as u8) as u128)<<8|((((xor_value_fourth <<24)>>24) as u8) as u128);
    //Reverse keys go to list
    key_list.push(new_key); 
}
fn main() {
    //Message
    let mut message :u128= 0x87f24d976e4c90ec46e74ac3a68cd895;
    //Key k10
    let key:u128 = 0xa3a51d068b50aac6587034e571157980u128;
    //List of reversed keys
    let mut key_list=Vec::new();
    //Reverse key 10 times
    for i in 0..10{
        if i == 0{
            inverse_key_expansion(key,&mut key_list);
        }
        else{
            //Take key which was added previouse time
            inverse_key_expansion(key_list[i-1],&mut key_list);
        }
    }
    //Cypher message with predefined keys 10 times
    for i in 0..10{
        println!("Round:{}",i+1);
        println!("Key:{:0x}",key_list[9-i]);
        xor(&mut message,key_list[9-i]);
        byte_sub(&mut message);
        shift_row(&mut message);
        //If this is not round 10 then 
        if i != 10{
            mix_columns(&mut message);
        }
        println!("Message:{:0x}",message);
    }
    //Xor message with k10
    xor(&mut message,key);
    //Finla cypher text
    println!("Final cypher message:{:0x}",message);
}
