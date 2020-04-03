# this is the local usb device path of my Zubax Dronecode probe: yours may differ
target extended-remote /dev/cu.usbmodemBFDA9CEB1

# print demangled symbols
set print asm-demangle on

# set backtrace limit to not have infinite backtrace loops
set backtrace limit 32

# detect unhandled exceptions, hard faults and panics
#break DefaultHandler
#break HardFault
#break rust_begin_unwind
#break panic

# # run the next few lines so the panic message is printed immediately
# # the number needs to be adjusted for your panic handler
# commands $bpnum
# next 4
# end

# *try* to stop at the user entry point (it might be gone due to inlining)
# break main

monitor swdp_scan
attach 1

# # send captured ITM to the file itm.fifo
# # (the microcontroller SWO pin must be connected to the programmer SWO pin)
# # 8000000 must match the core clock frequency
# monitor tpiu config internal itm.txt uart off 8000000

# # OR: make the microcontroller SWO pin output compatible with UART (8N1)
# # 8000000 must match the core clock frequency
# # 2000000 is the frequency of the SWO pin
# monitor tpiu config external uart off 8000000 2000000

# # enable ITM port 0
# monitor itm port 0 on
# monitor arm semihosting enable

# don't confirm when quitting debugger
define hook-quit
    set confirm off
end

load

# start the process but immediately halt the processor
# stepi

# continue running after loading the application
continue