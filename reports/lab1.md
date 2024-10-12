# 10.7 learn lab1  lab1
implement lab1 by hands according the tutorial
# 10.8 start 2024a-rcore lab2
## in console.rs, do not init the Stdout, but it work, why?
In your implementation of the Stdout struct, you don't need to initialize an instance of Stdout before using it in the print function because you’re effectively using a temporary instance. Here's how it works:
Explanation:

    Temporary Instance: In the print function, you create a new instance of Stdout when you call Stdout.write_fmt(args). Rust allows you to use structs in this way without explicitly instantiating them first because the instance is created on the stack and dropped immediately after the call.

    No State Management: Since Stdout doesn't maintain any internal state (i.e., it doesn't have fields), you can use it without initializing it. Each call to print creates a new Stdout instance, writes to it, and then it's dropped.
Alternative:

If you wanted to maintain some state within Stdout (like configuration or buffering), you would need to create a proper instance and manage its lifecycle. But in this case, since it only serves to output characters, creating a temporary instance is sufficient and idiomatic in Rust.

So your code works as intended without requiring explicit initialization of Stdout.
## https://docs.rs/log/latest/log/trait.Log.html

# 10.9 lab2
## show commands in sub dir to trace when use -nB option in parent dir?
use $(MAKE) not just make
The reason $(MAKE) works but make does not in a subdirectory call relates to how GNU Make handles recursive invocations. Here’s a breakdown of the differences:
1. Recursive Make Behavior

    $(MAKE): This variable automatically expands to the current make command that is being executed, including all its flags. It’s specifically designed to be used in recursive make calls. This ensures that any flags you provide (like -nB) are passed correctly to the sub-make process.

    make: When you directly call make, it does not inherit the flags or context of the original make command. As a result, if you invoke make in a subdirectory without $(MAKE), it will run the default make command without any of the current flags or environment settings.

2. Passing Flags

When you use $(MAKE), it carries over the flags you’ve passed to the top-level make command (like -nB), ensuring that:

    If you run make -nB, the -nB flags are passed down to the sub-make invocation correctly when you use $(MAKE).

## does base_address = 0x80400000 in build.py and BASE_ADDRESS = 0x0 conflict?
## sscratch in trap.S line 51
## recompile all crates every time build the project
# 10.10 lab2
## when does the global variable APP_MANAGE be initialized and how to debug it?
first time using it, in this example, batch::print_app_info
## at the end of __restore, after addi the sp, print sp in gdb, it will show USER_STACK?
it not wrong, sp actually points to the top  of kernel stack, which is the bottom of user stack coincidentlly, and in gdb it is the start of user stack
## when trap the cpu  modified sepc, scause, stval, no sscratch
## process of context switch 
sp->boot stack
run_next_app:   
    + create a cx on boot stack
    + push it to kernel stack
    + __restore
        + first, sp->boot stack, a0->cx on kernel stack, sscratch->0
        + mv a0 to sp, write sp from cx to sscratch, so sp->cx on kernel stack, sscratch->user stack
        + addi sp to release cx on kernel stack, now sp-> kernel stack, sscratch->user stack
        + swap sp and sscratch, now sp->user stack, sscratch->kernel stack
        + sret to sepc
user app:
    + user code
    + trap
    + __alltraps
        + sp->somewhere user stack, sscratch->kernel stack
        + swap sp and sscratch, sp->kernel stack, sscratch->somewhere user stack
        + addi sp, sp -272 to alloc space to save cx, sp->cx in kernel stack, sscratch->somewhere user stack
        + save sscratch to sp of ctx, now sp of ctx is user kernel addr before trap
        + mv sp to a0
        + call trap_handler

trap_handler:
    + a0 contain the ctx
# 10.11
## about compileing the user program
1. the -Ttext option will override the value of linker.ld and the order of sections ordered by linker.ld remaind 
# 10.12  lab3
## compare with lab2
    1. same
        packged with os and load to memory
    2. difference
        + lab2 user code is be load to 0x80400000
        + lab3 user code is be load to pa start at 0x8040000, one by one
## set timer trigger to 10 ms: set_timer(get_time() + CLOCK_FREQ / TICKS_PER_SEC);
CLOCK_FREQ: ticks per seconds/1000 ms
divided by 100, which is ticks per 10ms
