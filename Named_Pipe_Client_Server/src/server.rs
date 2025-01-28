use windows::core::s;
use windows::Win32::Foundation::CloseHandle;
use windows::Win32::Storage::FileSystem::{ReadFile, PIPE_ACCESS_DUPLEX};
use windows::Win32::System::Pipes::{
    ConnectNamedPipe, CreateNamedPipeA, PIPE_READMODE_MESSAGE, PIPE_TYPE_MESSAGE,
    PIPE_UNLIMITED_INSTANCES, PIPE_WAIT,
};

fn main() -> Result<(), Box<dyn std::error::Error>> {
    unsafe {
        let h_pipe = CreateNamedPipeA(
            s!("\\\\.\\pipe\\Teste"),
            PIPE_ACCESS_DUPLEX,
            PIPE_TYPE_MESSAGE | PIPE_READMODE_MESSAGE | PIPE_WAIT,
            PIPE_UNLIMITED_INSTANCES,
            2044,
            2044,
            0,
            None,
        )?;

        let mut number_return = 0;

        println!("[+] Waiting For Data");
        ConnectNamedPipe(h_pipe, None)?;

        let mut buffer = [0u8; 276];
        ReadFile(h_pipe, Some(&mut buffer), Some(&mut number_return), None)?;

        println!("{:?}", buffer);

        CloseHandle(h_pipe)?;
    }

    Ok(())
}
