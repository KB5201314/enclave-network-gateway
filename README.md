## reference

tuntap:
- https://www.gabriel.urdhr.fr/2021/05/08/tuntap/
- https://docs.kernel.org/networking/tuntap.html?highlight=tuntap
- https://ldpreload.com/p/tuntap-notes.txt

nat & ip routing:
- http://linux-ip.net/html/routing-saddr-selection.html
- http://linux-ip.net/html/routing-selection.html
- http://linux-ip.net/html/nat-dnat.html
- https://unix.stackexchange.com/questions/243451/iptables-change-local-source-address-if-destination-address-matches

dl & -rpath:
- https://man7.org/training/download/shlib_dynlinker_slides.pdf

## requirements

- scripts/run_tmux.sh
    - tmux
    - iproute2

- enta
    - iptables
        - maybe you need: `update-alternatives --set iptables /usr/sbin/iptables-nft`
