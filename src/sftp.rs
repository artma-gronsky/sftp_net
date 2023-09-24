use std::io::Cursor;
use suppaftp::FtpStream;
pub struct SftpClient{
    ftp_stream: FtpStream
}

impl SftpClient{
    pub fn new_connection(addr: &str) -> Self{
        let ftp_stream = FtpStream::connect(addr).unwrap_or_else(|e| panic!("{}",e));
        SftpClient{ ftp_stream}
    }

    pub fn login(&mut self, username: &str, password: &str){
        let _ = self.ftp_stream.login(username, password).unwrap_or_else(|e| panic!("{}",e));
    }

    pub fn put_file(&mut self, bytes: &[u8], file_name: &str){
        let mut reader = Cursor::new(bytes);
        let _ = self.ftp_stream.put_file(file_name, &mut reader).unwrap_or_else(|e| panic!("{}",e));
    }
}