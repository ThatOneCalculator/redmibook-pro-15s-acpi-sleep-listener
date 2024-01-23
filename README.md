# Redmibook Pro 15S ACPI sleep listener
Fixes suspend-on-lid-close with the Redmibook Pro 15S

Tested on BIOS version `RMACZ5B0P0505` (should work on `909` too, can't figure out how to upgrade it)

***THIS IS NOT MY CODE!!!***

I was trying to find a solution for this for years, and just today found a random Chinese forum site that had a solution. This doesn't seem to be documented anywhere else on the internet.

https://zhuanlan-zhihu-com.translate.goog/p/384332568?_x_tr_sl=auto&_x_tr_tl=en&_x_tr_hl=en-US&_x_tr_pto=wapp

Dependencies:

- `rust`
- `evemu`
- `acpid`

Installation:

1. `sudo usermod -aG input $USER`
2. `git clone https://github.com/ThatOneCalculator/redmibook-pro-15s-acpi-sleep-listener/ && cd redmibook-pro-15s-acpi-sleep-listener`
3. `rustc lid_event.rs && sudo install -Dm755 ./lid_event /usr/local/bin/`
4. `sudo cp ./lid-event.service /etc/systemd/system/`
5. `systemctl daemon-reload && systemctl enable --now lid-event.service`
