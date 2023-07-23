use std::{fs, io::{Write, BufWriter}};

fn write_data_into_file(path: &str, data: &[u8]) -> Result<(), Box<dyn std::error::Error>> {

    let file = fs::OpenOptions::new()
        .create(true)
        .append(true)
        .open(&path)?;

    let mut file = BufWriter::new(file);

    //trying to write everything in one go
    file.write_all(&data)?;

    file.flush()?;

    Ok(())
}

fn main() {
    match write_data_into_file(&".\\test.log", &"Testing the test \nwahooooo!!\n".as_bytes()) {
        Ok(_) => print!("Sucessfully wrote in the file!"),
        Err(_) => println!("Something went wrong opsies :<")
    }


}
