extern crate libc;

fn main() {
    println!("[client] client app started...");

    let socket_fd = unsafe { libc::socket(libc::AF_INET, libc::SOCK_STREAM, 0) };
    if socket_fd < 0 {
        println!("[client] create socket failed, ret: {}", socket_fd);
        return;
    }

    let servaddr = libc::sockaddr_in {
        sin_family: libc::AF_INET as u16,
        sin_port: htons(3456),
        // s_addr should be htonl(INADDR_ANY)
        sin_addr: libc::in_addr { s_addr: 0 },
        sin_zero: [0; 8],
    };
    let ret = unsafe {
        libc::connect(
            socket_fd,
            &servaddr as *const _ as *const libc::sockaddr,
            core::mem::size_of::<libc::sockaddr_in>() as u32,
        )
    };
    if ret < 0 {
        println!("[client] connect failed, ret: {}", ret);
        unsafe {
            libc::close(socket_fd);
        }
        return;
    }

    let mut buf = vec![0u8; 2048];
    let mut cnt = 0;
    while cnt < 10 {
        let mut ret =
            unsafe { libc::write(socket_fd, buf.as_ptr() as *const libc::c_void, buf.len()) };
        if ret < 0 {
            println!("[client] write failed, ret: {}", ret);
            unsafe {
                libc::close(socket_fd);
            }
            return;
        }
        println!("[client] write {} bytes, want write {} bytes", ret, buf.len());

        if ret < buf.len() as isize {
            println!("[client] write the rest buffer... {}...{}", ret, buf.len());
            let mut cur_ptr = unsafe { buf.as_ptr().add(ret as usize) };
            let mut cur_len = buf.len() - ret as usize;
            while cur_len > 0 {
                let next_ret =
                    unsafe { libc::write(socket_fd, cur_ptr as *const libc::c_void, cur_len) };
                if next_ret < 0 {
                    println!("[client] write failed, ret: {}", ret);
                    unsafe {
                        libc::close(socket_fd);
                    }
                    return;
                }
                cur_ptr = unsafe { cur_ptr.add(next_ret as usize) };
                cur_len -= next_ret as usize;
            }
        }

        ret = unsafe { libc::read(socket_fd, buf.as_mut_ptr() as *mut libc::c_void, buf.len()) };
        if ret < 0 {
            println!("[client] read failed, ret: {}", ret);
            unsafe {
                libc::close(socket_fd);
            }
            return;
        }
        println!("[client] read {} bytes, want read {} bytes", ret, buf.len());

        if ret < buf.len() as isize {
            println!("[client] read the rest buffer... {}...{}", ret, buf.len());
            let mut cur_ptr = unsafe { buf.as_mut_ptr().add(ret as usize) };
            let mut cur_len = buf.len() - ret as usize;
            while cur_len > 0 {
                let next_ret =
                    unsafe { libc::read(socket_fd, cur_ptr as *mut libc::c_void, cur_len) };
                if next_ret < 0 {
                    println!("[client] read failed, ret: {}", ret);
                    unsafe {
                        libc::close(socket_fd);
                    }
                    return;
                }
                cur_ptr = unsafe { cur_ptr.add(next_ret as usize) };
                cur_len -= next_ret as usize;
            }
        }

        cnt += 1;
    }

    println!("[client] close and exit");
    unsafe {
        libc::close(socket_fd);
    }
}

pub fn htons(u: u16) -> u16 {
    u.to_be()
}
