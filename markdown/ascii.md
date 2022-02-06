# ascii
## Introduction
A more difficult version of ascii_tiny that I've already done. The binary contains a very simple buffer overflow and the point of the challenge is very simple, the exploit can contain only ASCII characters (0x20-0x7f).
## The binary
The program is quite simple and when you run it, it asks for some user inputs, and then prints something.
![[ascii/first_run.PNG]]
Just for fun, we can try to give a large input, and see what happens.
![[ascii/first_overflow.PNG]]
A segfault, so the vulnerability is probably a buffer overflow.
## The vulnerability
I opened the program in IDA, and saw that the code was also very straight forward.
![[ascii/main.PNG]]
On start, the program allocates some RWX memory, and then reads from stdin, one byte at a time, into the buffer it allocated until it either read 400 bytes, or the input byte is not an alphanumerical ascii character, as we can see is defined in is_ascii.
![[ascii/is_ascii.PNG]]
Then, vuln is called, which seems to be the place that triggers the overflow.
![[ascii/vuln.PNG]]
As we can see, the bug here is that the buffer can contain up to 400 characters and dest is only 168 bytes long. This means, we have a buffer overflow on the stack.
*INSERT IDEAS*
The way I decided to exploit this was to search for references to the input buffer on the stack, *Emphasize how this is done (overwriting EBP)* and using the vulnerability pivot the stack to point to it when main returns, and so get code execution, granted, our code can only be alphanumerical, which would prove to be a bit annoying 😜. Using gdb, I searched for a reference to the buffer, and found that there is one at $esp - 0x14, when main returns.
![[gdb_buffer_reference.PNG]]
* Explain how the EBP overwriting works*
* Show code execution tests *
* Shellcode crafting *
* Final Tests and victory *






# Log
## Exploit Ideas
## Writing the shellcode
Now that we've got code execution, we need to execute some shellcode that will get us the flag! Unfortunately, we can only use alphanumeral bytes, so this is quite a challenge.
### Things that we can use
- ```push $src; pop $dst``` to move from $src to $dst.
- We can put the stack on our code buffer, and write new code maybe?
- ```ecx``` is 0x800000a0 when we run, we can use that to move the stack there and push some code there.

## Notes
## Ideas
- overwrite ebp to somewhere that will statistically point to somewhere good that's on the stack (0x80000000). There is an address containing 0x80000000 somewhere down the stack. We can set ebp to be that address - 4 and main should return to 0x80000000. The difference between ebp and the address is 0x10. So, for this plan we need to somehow subtract 0x14 from ebp. Probably too improbable. Or not!!! I got code execution!!! (Describe how I got there in length). ![[ascii/ascii.PNG]]
## Primitives
- Buffer overflow that grants write to the stack, but we can only write bytes between 0x20 and 0x7f + an optional non-ascii byte + 0.

## Lessons
- vsdo?? (general thingie about memory regions)
- something about better testing!!!

## Refactoring TODO
- highlight terms