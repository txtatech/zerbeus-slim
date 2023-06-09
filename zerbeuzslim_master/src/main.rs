mod server;
use server::Server;

pub static VGABIOS_BYTES: &'static [u8] = include_bytes!("../assets/vgabios.bin");
pub static SEABIOS_BYTES: &'static [u8] = include_bytes!("../assets/seabios.bin");
pub static WASEM_JS_BYTES: &[u8] = include_bytes!("../assets/wasem.js");
pub static MEMORY_JS_BYTES: &[u8] = include_bytes!("../assets/memory.js");
pub static SYSCALL_JS_BYTES: &[u8] = include_bytes!("../assets/syscall.js");
pub static ERRNO_JS_BYTES: &[u8] = include_bytes!("../assets/errno.js");
pub static FS_JS_BYTES: &[u8] = include_bytes!("../assets/fs.js");
pub static INDEX_BYTES: &'static [u8] = include_bytes!("../assets/index.html");
pub static IINDEX_BYTES: &'static [u8] = include_bytes!("../assets/index1.html");
pub static LIBV86_BYTES: &'static [u8] = include_bytes!("../assets/libv86.js");
pub static LINUX_BYTES: &'static [u8] = include_bytes!("../assets/linux.iso");
pub static SLINUX_BYTES: &'static [u8] = include_bytes!("../assets/slinux.iso");
pub static VLINUX_BYTES: &'static [u8] = include_bytes!("../assets/vlinux.iso");

#[actix_web::main]
async fn main() -> std::io::Result<()> {	
    let server = Server::new();
    server.run().await
}
