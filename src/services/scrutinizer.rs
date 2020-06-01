use std::env;

pub struct ScrutinizerService {
    pub box_public_key: Vec<u8>,
}

impl ScrutinizerService {
    pub async fn new() -> Result<ScrutinizerService, reqwest::Error> {
        let public_key_str = reqwest::get(&format!(
            "{}/boxPublicKey",
            &env::var("SCRUTINIZER_URL").expect("No SCRUTINIZER_URL set in .env")
        ))
        .await?
        .text()
        .await?;

        let mut public_key_vec: Vec<u8> = Vec::with_capacity(32);

        let numbers: Vec<&str> = public_key_str.split('[').collect();
        let numbers: Vec<&str> = numbers[1].split(']').collect();
        let numbers: Vec<&str> = numbers[0].split(',').collect();

        for number in numbers.iter() {
            public_key_vec.push(number.parse::<i32>().unwrap() as u8);
        }

        Ok(ScrutinizerService {
            box_public_key: public_key_vec,
        })
    }
}
