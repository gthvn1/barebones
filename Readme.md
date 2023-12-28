# Kikuyu Colobus

The kikuyu Colobus monkey has its name that starts with 'k' that remember me the 'k' of Kernel... kind of.
It will be the kernel of [Monkey Islang](https://github.com/gthvn1/monkey_islang).

# Notes

- We are using nightly build from rust.
- Run the kernel in qemu-system-i386: `cargo run`
- Check code using: `cargo +nightly clippy -- -Dwarnings`

## Debugging
- Check the address of the `kernel_start` function
```
❯ nm -s target/kikuyu-kernel/debug/kikuyu |grep kernel
00200c90 T kernel_start
```
- Start qemu with the option for debugging and stopping the CPU when started
```
❯ qemu-system-i386 -serial stdio -s -S -kernel target/kikuyu-kernel/debug/kikuyu
```
- And connect remotly using gdb
```
❯ gdb -ex 'target remote localhost:1234'
```
- Now you can add a break to `kernel_start`, in my case `break *0x200c90` and
debug...

# Links

- [Bare Bones - booting simple kernel](https://wiki.osdev.org/Bare_Bones)
- [The Rust Standard Library](https://doc.rust-lang.org/std/index.html)
- [Writing an OS in Rust](https://os.phil-opp.com/)
- [Writing an OS - Sphaerophoria YT](https://www.youtube.com/watch?v=gBykJMqDqH0&list=PL980gcR1LE3LBuWuSv2CL28HsfnpC4Qf7)

# Screenshots
## Changelog 2023-12-28
![](https://github.com/gthvn1/kikuyu/blob/master/screenshots/screenshot-changelog-2023-12-28.png)
