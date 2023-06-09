# zerbeus-slim
An http server written in Rust that boots four memory resident linux distros in the browser simultaneously.

**This is not a production server.**

There are four available linux shells in this server but only three of them are used because one of them bootstraps the other three.

Note: There is an embedded html site in the server.rs itself.

All files for the server are loaded as/from a byte array at boot time.

The default directory the server serves from is 'static' but is not required for the server to operate.

This server has open routes, handles and static variables for the following files:

Used from: https://github.com/rslay/c_in_browser

libv86.js

linux.iso

seabios.bin

vgabios.bin

index.html

Used from: https://github.com/sbsoftware/wasem.js

memory.js

syscall.js

wasem.js

errno.js

fs.js

The server operates on the following addresses and ports.

http://127.0.0.1:8081

http://127.0.0.1:8080

http://127.0.0.1:8080/index.html <---Used to boostrap--->

http://127.0.0.1:8081/index1.html
