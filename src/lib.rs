pub mod server
{
    use std;
    use std::net::{TcpListener, TcpStream};
    use std::io::prelude::*;
    use std::fs::File;

    extern crate rand;

    pub fn launch() -> Result<(), Box<std::error::Error>>
    {
        let listener = TcpListener::bind("0.0.0.0:80")?;
        for stream in listener.incoming()
        {
            let stream = stream?;
            handle_stream(stream)?;
        };
        Ok(())
    }
    fn get_joiner() -> String
    {
        let num_slices: u32 = 20;
        let result: u32 = rand::random::<u32>() % (num_slices+1);
        let result = result*100/num_slices;
        let result_perc = if result < 72 && result > 66 
        {
            69
        }
        else
        {
            result
        };
        let mut result = format!("You are {}% gay!", result_perc);
        if result_perc == 69
        {
            result+="<br>SPICY GAY!!";
        }
        else if result_perc == 100
        {
            result+="<br>Wow you're a real faggot";
        }
        else if result_perc == 0
        {
            result+="<br>Boring af lmao";
        }
        else if result_perc < 43 && result_perc > 37
        {
            result="You are 420% gay<br>BLAZE IT".to_string();
        }
        else if result_perc < 52 && result_perc > 48
        {
            result+="<br>(Isn't that bisexual or some other made up shit??)";
        }


        result
    }
    fn handle_stream(mut stream: TcpStream) -> Result<(), Box<std::error::Error>>
    {
        let mut buffer = [0; 1024];
        stream.read(&mut buffer)?;

        let get_req = b"GET / HTTP/1.1\r\n";
        let css_req = b"GET /lol.css HTTP/1.1\r\n";

        let (status_line, content) = if buffer.starts_with(get_req)
        {
            let filename = "index.html";
            let mut file = File::open(filename)?;
            let mut content = String::new();
            file.read_to_string(&mut content)?;
            let content = content.split("{CONTENT}")
                .collect::<std::vec::Vec<_>>()
                .join(&get_joiner()[..]);
            ("HTTP/1.1 200 OK\r\n\r\n", content)
        }
        else if buffer.starts_with(css_req)
        {
            let filename = "lol.css";
            let mut file = File::open(filename)?;
            let mut content = String::new();
            file.read_to_string(&mut content)?;
            ("HTTP/1.1 200 OK\r\n\r\n", content)
        }
        else
        {
            let filename = "404.html";
            let mut file = File::open(filename)?;
            let mut content = String::new();
            file.read_to_string(&mut content)?;
            ("HTTP/1.1 404 NOT FOUND\r\n\r\n", content)
        };

        let response = format!("{}{}", status_line, content);
        stream.write(response.as_bytes())?;
        stream.flush()?;

        Ok(())
    }
}
