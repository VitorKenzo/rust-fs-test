use std::{fs, fs::File, io::{Write, BufWriter, BufReader, BufRead}, str::from_utf8};


const BUFFER_SIZE: usize = 512;

fn write_data_into_file(path: &str, data: &[u8]) -> std::io::Result<()> {

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

fn main() -> std::io::Result<()> {

    // using the writing function we created
    match write_data_into_file(&".\\test.log", &"Testing the test \nwahooooo!!\n".as_bytes()) {
        Ok(_) => println!("Sucessfully wrote in the file!"),
        Err(e) => println!("Something went wrong opsies :< {:?}", e)
    }


    let path = ".\\test.log";

    let file = File::open(&path)?;
    let mut reader = BufReader::with_capacity(BUFFER_SIZE, file);

    println!("Reading {}...", path);
    loop {
        let buffer = reader.fill_buf()?;
        let buffer_length = buffer.len();

        //lets say its over, we can get out of the loop
        if buffer_length == 0 {
            break;
        }

        //do something part
        print!("{}\n", from_utf8(&buffer).unwrap());

        //making sure we are not stuck
        reader.consume(buffer_length);
    }

    Ok(())
}
