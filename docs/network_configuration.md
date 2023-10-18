# Hacktoberfest network configuration

This document explains how the RaspberryPis and tunnel server were configured.

## Setting up the server

### Logging in

To log in to the server (requires a public key to be deployed)
```
ssh ubuntu@34.249.167.137
```

### Configuring hardened port forwarding

See https://askubuntu.com/a/49848

The configuration creates a user `cecilia` (named after one of the HS2 borers) which allows tunnelling to the RaspberryPis but without shell access.

Edit the following file to create a suitable port forwarding ssh configuration.

```
/etc/ssh/sshd_config.d/cecilia.conf
```

Add the following sshd configuration to the file:
```
Match User cecilia
   #AllowTcpForwarding yes
   #X11Forwarding no
   #PermitTunnel no
   #GatewayPorts no
   AllowAgentForwarding no
   PermitOpen localhost:8000 localhost:8001 localhost:8002
   ForceCommand echo 'This account can only be used for port forwarding'
```

Now create the `cecilia` user, disable the shell, and reload the sshd configuration.
```
sudo useradd -m cecilia
sudo chsh -s /bin/false cecilia
sudo systemctl reload sshd
```

### Viewing connections

See https://askubuntu.com/a/1047964

To view connections on the server, which is useful for knowing whether one of the Raspberry Pis is connected.

List connections
```
ss
```

A RaspberryPi will fail to connect if an old connection is lingering and so the port is in use. In this case it can be useful to kill the lingering connection. Use the IP address listed by `ss`.
```
sudo ss -K dst <ip_address>
```

To list the devices connected to the node (e.g. a wifi hotspot):
```
arp -a
```

## Setting up a Raspberry Pi

The following lists the RaspberryPis and their ports

1. pi@rose: 8000 - Pi 3 Model B V1.2 1 GiB
2. pi@fiacre: 8001 - Pi 4 Model B 8 GiB
3. pi@ansovinus: 8002 - Pi 3 Model B+ 1 GiB

### Software

Install Raspbian on an SD card. See https://www.raspberrypi.com/software/

```
rpi-imager
```

Install **Raspberry Pi OS (Legacy) Lite**: This is a 32-bit version of Debian Bullseye.

To configure tunnelling on the RaspberryPi, perform the following.

On the RaspberryPi generate ssh keys (change `raspberrypi.local` to the name of the device).
```
ssh-keygen -t ed25519 -C "pi@raspberrypi.local"
```

On the server add the RaspberryPi key:
```
sudo echo "<key.pub>" >> /home/cecilia/.ssh/authorized_keys
sudo systemctl reload sshd
```

On the RaspberryPi set up autostart of the ssh tunnel by adding a cron job. See https://askubuntu.com/a/884930

```
crontab -e
```

Add the following line to the crontab file (this waits 120 seconds before connecting):
```
@reboot sleep 120 && ssh -N -f -M -S /tmp/file-sock -o "StrictHostKeyChecking=no" -R 8000:localhost:22 cecilia@34.249.167.137 touch success
```

This cron job will be stored at:
```
/var/spool/cron/crontabs
```

On the RaspberryPi set up the wifi network. See https://raspberrypi.stackexchange.com/a/37921
```
sudo raspi-config	
```

The resulting configuration will be found in the following file:
```
/etc/wpa_supplicant/wpa_supplicant.conf
```

On the RaspberryPi, after configuring everything, clear the known hosts:
```
rm ~/.ssh/known_hosts
```

### Accessing the RaspberryPi

First you'll need to set up the user for access to both the RaspberryPi and the server.

The user will need to generate an SSH key on their development device if they don't already have one.
```
ssh-keygen -t ed25519 -C "username@hostname"
cat ~/.ssh/id_ed25519.pub
```

Add the user's key. To the server:
```
sudo echo "<key.pub>" >> /home/cecilia/.ssh/authorized_keys
sudo systemctl reload sshd
```

And to the RaspberryPi:
```
sudo echo "<key.pub>" >> /home/pi/.ssh/authorized_keys
sudo systemctl reload sshd
```

On the user's development machine open the connection using one of the following (depending on which RaspberryPi you want to connect to):
```
ssh -N -f -M -S /tmp/file-sock -L 8000:localhost:8000 cecilia@34.249.167.137
ssh -N -f -M -S /tmp/file-sock -L 8000:localhost:8001 cecilia@34.249.167.137
ssh -N -f -M -S /tmp/file-sock -L 8000:localhost:8002 cecilia@34.249.167.137
ssh pi@localhost -p 8000
```

On the user's development machine to close the connection:
```
ssh -S /tmp/file-sock -O exit cecilia@34.249.167.137
```

On the RaspberryPi use the following to open the connection to the server. These shouldn't need to be run (the cronjob should do this), so this is just for reference:
```
ssh -N -f -M -S /tmp/file-sock -R 8000:localhost:22 cecilia@34.249.167.137
ssh -N -f -M -S /tmp/file-sock -R 8001:localhost:22 cecilia@34.249.167.137
ssh -N -f -M -S /tmp/file-sock -R 8002:localhost:22 cecilia@34.249.167.137
```

On the RaspberryPi to close the connection:
```
ssh -S /tmp/file-sock -O exit cecilia@34.249.167.137
```

