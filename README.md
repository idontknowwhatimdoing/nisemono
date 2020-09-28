# **nisemono**
ARP cache poisoning program written in Rust

## What is ARP cache poisoning ?
[ARP cache poisoning](https://en.wikipedia.org/wiki/ARP_spoofing) (or [ARP spoofing](https://en.wikipedia.org/wiki/ARP_spoofing)) is a type of attack where the attacker proxies a connection between two hosts (usually a router and another computer, server, ...) on a local network by impersonating the other host. It can be used to steal, modify, or stop all network trafic bewteen these two hosts ([denial of service attacks](https://en.wikipedia.org/wiki/Denial_of_service), [man in the middle](https://en.wikipedia.org/wiki/Man-in-the-middle_attack)...).

### Usage
`sudo nisemono <target_IP> <other_target_IP>`
(nisemono needs root privileges to access and use the network card)

### Notes
Only works on Linux (maybe MacOs idk)
