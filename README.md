# Swaytab - A tab menu for sway

### Installing

```bash
git clone https://github.com/olback/swaytab && cd swaytab
cargo install --path .
```


### Create default config

```bash
swaytab --gen-config
```
Edit menu command in `~/.config/swaytab/Swaytab.toml`


### Configure Sway

sway config:
```
...
bindsym $mod+tab exec ~/.cargo/bin/swaytab
...
```

