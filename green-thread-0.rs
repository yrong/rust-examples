#![feature(asm)]

// Lets set a small stack size here, only 48 bytes so we can print the stack
// and look at it before we switch contexts
// ===== NOTICE FOR OSX USERS =====
// You'll need to increase this size to at least 624 bytes. This will work in Rust Playground and on Windows
// but the extremely small stack seems to have an issue on OSX.
const SSIZE: isize = 48;

/// Do you recognize these? It's the registers described in the x86-64 ABI that we'll need to save our context.
/// Note that this needs to be #[repr(C)] because we access the data the way we do in our assembly. Rust doesn't have a
/// stable ABI so there is no way for us to be sure that this will be represented in memory with `rsp` as the first 8 bytes.
/// C has a stable ABI we can use.
#[derive(Debug, Default)]
#[repr(C)]
struct ThreadContext {
    rsp: u64,
    r15: u64,
    r14: u64,
    r13: u64,
    r12: u64,
    rbx: u64,
    rbp: u64,
}

fn hello() -> ! {
    println!("I LOVE WAKING UP ON A NEW STACK!");

    loop {}
}

// We use a trick here. We push the address to our own stack to the rsp register. The ret keyword transfers program control
// to the return address located on top of the stack. Since we pushed our address there it returns directly into our
// function.
unsafe fn gt_switch(new: *const ThreadContext) {
    asm!("
        mov 0x00($0), %rsp
        ret
       "
    :
    : "r"(new)
    :
    : "alignstack" // it will work without this now, but we need it for it to work on windows later
    );
}

fn main() {
    let mut ctx = ThreadContext::default();

    // This will be our stack. Note that it's very important that we don't `push` to this array since it can trigger an
    // expansion that will relocate all the data and our pointers will no longer be valid
    let mut stack = vec![0_u8; SSIZE as usize];

    unsafe {

        // this returns the pointer to the memory for our Vec, we offset it so
        // we get the "high" address which will be the bottom of our stack.
        let stack_bottom = stack.as_mut_ptr().offset(SSIZE);

        // make sure our stack itself is 16 byte aligned - it will always
        // offset to a lower memory address. Since we know we're at the "high"
        // memory address of our allocated space, we know that offsetting to
        // a lower one will be a valid address (given that we actually allocated)
        // enough space to actually get an aligned pointer in the first place).
        let sb_aligned = (stack_bottom as usize & ! 15) as *mut u8;

        // So this is actually designing our stack. `hello` is a pointer already (a function pointer) so we can cast it
        // directly as an u64 since all pointers ono 64 bits systems will be, well, 64 bit ;)
        //
        // Then we write this pointer to our stack. Make note that we cast the pointer to to the offset of 16 bytes
        // (remember what I wrote about 16 byte alignment?). And that we cast it as a pointer to an u64 instead of an u8
        // We want to write to position 32, 33, 34, 35, 36, 37, 38, 39, 40 which is the 8 byte space we need to store our
        // u64.
        std::ptr::write(sb_aligned.offset(-16) as *mut u64, hello as u64);

        // We set the "rsp" (Stack Pointer) to *point to* the first byte of our address, we don't pass the value of the
        // u64, but an address to the first byte.
        ctx.rsp = sb_aligned.offset(-16) as u64;

        // we switch over to our new stack
        gt_switch(&mut ctx);
    }
}