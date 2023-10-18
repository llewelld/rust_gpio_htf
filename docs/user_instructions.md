# User instructions

## Accessing a RaspberryPi

To get access to a RaspberryPi you'll need to do the following.

1. Create an SSH key pair if you don't already have one.

   ```
   ssh-keygen -t ed25519 -C "username@hostname"
   ```
2. Send David your *public* key (the one with .pub on the end of the filename).
   ```
   cat ~/.ssh/id_ed25519.pub
   ```
3. David will then install your key on a Pi.
4. Once this is done you can log in to a RaspberryPi using the commands below.
   To log in to **rose**:
   ```
   ssh -N -f -M -S /tmp/file-sock -L 8000:localhost:8000 cecilia@34.249.167.137
   ssh pi@localhost -p 8000
   ```
   To log in to **fiacre**:
   ```
   ssh -N -f -M -S /tmp/file-sock -L 8000:localhost:8001 cecilia@34.249.167.137
   ssh pi@localhost -p 8000
   ```
   To log in to **ansovinus**:
   ```
   ssh -N -f -M -S /tmp/file-sock -L 8000:localhost:8002 cecilia@34.249.167.137
   ssh pi@localhost -p 8000
   ```
5. After logging out you should close the tunnel.
   ```
   ssh -S /tmp/file-sock -O exit cecilia@34.249.167.137
   ```

