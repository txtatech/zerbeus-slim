use actix_web::{get, post, web, App, HttpResponse, HttpServer, Responder};
use actix_files as files;
use tokio::try_join;

#[get("/linux.iso")]
async fn get_linux() -> impl Responder {
    HttpResponse::Ok().insert_header(("Content-Type", "application/octet-stream")).body(crate::LINUX_BYTES)
}	

#[get("/slinux.iso")]
async fn get_slinux() -> impl Responder {
    HttpResponse::Ok().insert_header(("Content-Type", "application/octet-stream")).body(crate::SLINUX_BYTES)
}

#[get("/vlinux.iso")]
async fn get_vlinux() -> impl Responder {
    HttpResponse::Ok().insert_header(("Content-Type", "application/octet-stream")).body(crate::VLINUX_BYTES)
}

#[get("/vgabios.bin")]
async fn get_vgabios() -> impl Responder {
    HttpResponse::Ok().insert_header(("Content-Type", "application/octet-stream")).body(crate::VGABIOS_BYTES.to_vec())
}

#[get("/seabios.bin")]
async fn get_seabios() -> impl Responder {
    HttpResponse::Ok().insert_header(("Content-Type", "application/octet-stream")).body(crate::SEABIOS_BYTES.to_vec())
}

#[get("/index.html")]
async fn get_index() -> impl Responder {
    HttpResponse::Ok().insert_header(("Content-Type", "text/html")).body(crate::INDEX_BYTES)
}

#[get("/index1.html")]
async fn get_iindex() -> impl Responder {
    HttpResponse::Ok().insert_header(("Content-Type", "text/html")).body(crate::IINDEX_BYTES)
}

#[get("/libv86.js")]
async fn get_libv86() -> impl Responder {
    HttpResponse::Ok().insert_header(("Content-Type", "application/javascript")).body(crate::LIBV86_BYTES)
}

#[get("/wasem.js")]
async fn get_wasem() -> impl Responder {
    HttpResponse::Ok().insert_header(("Content-Type", "application/javascript")).body(crate::WASEM_JS_BYTES)
}

#[get("/memory.js")]
async fn get_memory() -> impl Responder {
    HttpResponse::Ok().insert_header(("Content-Type", "application/javascript")).body(crate::MEMORY_JS_BYTES)
}

#[get("/syscall.js")]
async fn get_syscall() -> impl Responder {
    HttpResponse::Ok().insert_header(("Content-Type", "application/javascript")).body(crate::SYSCALL_JS_BYTES)
}

#[get("/errno.js")]
async fn get_errno() -> impl Responder {
    HttpResponse::Ok().insert_header(("Content-Type", "application/javascript")).body(crate::ERRNO_JS_BYTES)
}

#[get("/fs.js")]
async fn get_fs() -> impl Responder {
    HttpResponse::Ok().insert_header(("Content-Type", "application/javascript")).body(crate::FS_JS_BYTES)
}

#[get("/")]
async fn hello() -> impl Responder {
    HttpResponse::Ok().body(r#"	
        <html>
        <head>
            <meta http-equiv="refresh" content="1115; URL='index.html'">
			<a href="http://127.0.0.1:8081">localhost:8081-Backbone</a>
            <a href="http://127.0.0.1:8080">localhost:8080-Backbone</a>
			<a href="http://127.0.0.1:8080/index.html">localhost:8080-Index</a>
            <a href="http://127.0.0.1:8081/index1.html">localhost:8081-Index1</a>			           
        </head>
<body>
    <div class="screen-container" id="screen-container1">
        <iframe src="http://127.0.0.1:8080/index.html" width="650" height="390" frameborder="0"></iframe>
		</div>
		    <div class="screen-container" id="screen-container3">
        <iframe src="http://127.0.0.1:8081/index1.html" width="650" height="390" frameborder="0"></iframe>
		</div>
		            <div id="screen_container2">
                <div style="white-space: pre; font: 14px monospace; line-height: 14px"></div>
                <canvas style="display: none"></canvas>
            </div>
</body>			
</html>
<!DOCTYPE html>
<html>
        <body>
            <script src="libv86.js"></script>
            <script>
    "use strict";
    
    window.onload = function() {
        var emulator = window.emulator = new V86Starter({
            memory_size: 32 * 1024 * 1024,
            vga_memory_size: 2 * 1024 * 1024,
            screen_container: document.getElementById("screen_container2"),
            bios: {
                url: "seabios.bin",
            },
            vga_bios: {
                url: "vgabios.bin",
            },
            cdrom: {
                url: "vlinux.iso",
            },
            autostart: true,
        });
    };
     </script>
			
        </body>
        </html>
    "#)
}

#[post("/echo")]
async fn echo(req_body: String) -> impl Responder {
    HttpResponse::Ok().body(req_body)
}

async fn manual_hello() -> impl Responder {
    HttpResponse::Ok().body("Hey there!")
}

pub struct Server;

impl Server {
    pub fn new() -> Server {
        Server
    }

    pub async fn run(&self) -> std::io::Result<()> {
        let server1 = HttpServer::new(|| {
            App::new()
                .service(hello)
                .service(get_linux)
                .service(get_slinux)
                .service(get_vlinux)                
                .service(get_vgabios)
                .service(get_seabios)
                .service(get_index)
				.service(get_iindex)
                .service(get_libv86)
                .service(get_wasem)
                .service(get_memory)
                .service(get_syscall)
                .service(get_errno)
                .service(get_fs)
                .route("/hey", web::get().to(manual_hello))
                .default_service(files::Files::new("/", "static").index_file("index.html"))
        })
        .bind("127.0.0.1:8080")?
        .run();

        let server2 = HttpServer::new(|| {
            App::new()
                .service(hello)
                .service(get_linux)
                .service(get_slinux)
                .service(get_vlinux)                
                .service(get_vgabios)
                .service(get_seabios)
                .service(get_index)
				.service(get_iindex)
                .service(get_libv86)
                .service(get_wasem)
                .service(get_memory)
                .service(get_syscall)
                .service(get_errno)
                .service(get_fs)
				.route("/hey", web::get().to(manual_hello))
                .default_service(files::Files::new("/", "static").index_file("index1.html"))
        })
        .bind("127.0.0.1:8081")?
        .run();


        match tokio::try_join!(server1, server2) {
            Ok(_) => Ok(()),
            Err(e) => Err(e),
	    }
    }
}