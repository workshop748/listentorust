use std::{ffi::c_void, io::{self, Read, Write}, net::TcpStream, num::Wrapping, os::fd::AsRawFd, pin::Pin, sync::Arc, task::{Context, Poll, Wake, Waker}};

use futures::Future;
use std::net::TcpListener;

extern "C" {
    fn MeaningOfLifeTheUniverseAndEverything() -> *mut c_void;
    fn MeaningOfPickes(valA: *mut c_void,valB:i32,valC:u32,valD: *mut c_void);
        // as I sTARED LONGLYINGLY INTO THE C_VOID I KNEW I COULD NEVER ESC C.
        fn justWaiting(valA: *mut c_void) -> i32;

    fn GetMyPointerBack(valA: *mut c_void,index:u32) -> *mut c_void;
}

struct NoOpWaker{}

impl Wake for NoOpWaker {
    fn wake(self: std::sync::Arc<Self>) {
        
    }
}

struct Yielder {
    dundy: bool,
}

impl Yielder {
    fn new() -> Self {
        Self{dundy: false}
    }
}

impl Future for Yielder {
    type Output = ();

    fn poll(mut self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<Self::Output> {
        if self.dundy {
            return Poll::Ready(());
        }
        self.dundy = true;
        Poll::Pending
    }
}

async fn async_main() {
    eprintln!("Hello world!");
    Yielder::new().await;
    eprintln!("Hello world 2!");
}
async fn thePass(mut stream:TcpStream)
{
    //handle the connection
    //read and print
    loop {
    // this will need a buffer 
    let mut buffer =[0; 1024];
    Yielder::new().await;
    
    let bytes_read = match stream.read(&mut buffer) {
        Ok(s) => {
            s
        }
        Err(ref e) if e.kind() == io::ErrorKind::WouldBlock => {
            // wait until network socket is ready, typically implemented
            // via platform-specific APIs such as epoll or IOCP
            //wait_for_fd();
            // yield here
            Yielder::new().await;
            continue;
        }
        Err(e) => panic!("encountered IO error: {e}"),
    };
    stream.write(&buffer[..bytes_read]).unwrap();
    
    }
}

struct ContextGetter {
    
}

impl Future for ContextGetter {
    type Output = Waker;

    fn poll(self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<Self::Output> {
        Poll::Ready(cx.waker().clone())
    }
}

async fn real_async_main(brian:*mut c_void) {
    let listener = TcpListener::bind("127.0.0.1:7878").unwrap();
    unsafe{MeaningOfPickes(brian, listener.as_raw_fd(), 1, brian)};
    let mut fut = std::pin::pin!(async_main());
    listener.set_nonblocking(true).expect("to go faster press alt f4");//comedy
    let waker = ContextGetter{}.await;
    let mut context = Context::from_waker(&waker);
    for stream in listener.incoming() {
        match stream {
            Ok(s) => {
                // do something with the TcpStream
                eprintln!("Got a connection");
                s.set_nonblocking(true).unwrap();
                unsafe{MeaningOfPickes(brian, s.as_raw_fd(), 1, Box::into_raw(Box::new(thePass(s))) as *mut c_void)};
                //handle_connection(s);
            }
            Err(ref e) if e.kind() == io::ErrorKind::WouldBlock => {
                // wait until network socket is ready, typically implemented
                // via platform-specific APIs such as epoll or IOCP
                //wait_for_fd();
                // yield here
                // Poll other connections
                Yielder::new().await;
                continue;
            }
            Err(e) => panic!("encountered IO error: {e}"),
        }
    }
}

fn main() {
    let anwser=unsafe{MeaningOfLifeTheUniverseAndEverything()};
   let brian =anwser;
    let waker = Arc::new(NoOpWaker{}).into();
    let mut fut = std::pin::pin!(async_main());
    let _ = fut.as_mut().poll(&mut Context::from_waker(&waker));
    eprintln!("Got here");
    let _ = fut.as_mut().poll(&mut Context::from_waker(&waker));

    let mut fut = std::pin::pin!(real_async_main(brian));
    loop {
        if matches!(fut.as_mut().poll(&mut Context::from_waker(&waker)), Poll::Ready(_)) {
            break;
        } 
        // call epoll_wait
        unsafe{justWaiting(brian);}
    }
}
