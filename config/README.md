# Config New Image

Raspberry Pi 5 Setup

Setup 64-bit Ubuntu 26.04 Desktop Resolute Racoon


As Of: 9 Apr 2026

Ubuntu 26.04 Desktop for Raspberry Pi Image Required (not in RPi-imager yet)
https://cdimage.ubuntu.com/daily-preinstalled/current/resolute-preinstalled-desktop-arm64+raspi.img.xz


** Get Latest Raspberry Pi Imager (will list latest OS options) **
https://www.raspberrypi.com/software/

v2.0.7

**Write Ubuntu 26.04 Desktop to SDcard with Raspberry Pi Imager**
  - Choose OS -> Other General Purpose (or Custom) -> Ubuntu 26.04 Desktop
  - boot with keyboard, mouse, display
  - user: ubuntu  machine: u26llpi5 , WiFi connect
  - [Did not work] WiFi Icon->Settings->System->SecureShell->enable




==== First Boot ====

=== Disable IPv6 ===

- Settings->WiFi->IPv6->Disable

=== 
uname -a
Linux u26llpi5 7.0.0-1004-raspi #4-Ubuntu SMP PREEMPT_DYNAMIC Wed Mar 18 06:27:33 UTC 2026 aarch64 GNU/Linux

cat /etc/os-release
ubuntu@u26llpi5:~$ cat /etc/os-release 
PRETTY_NAME="Ubuntu Resolute Raccoon (development branch)"
NAME="Ubuntu"
VERSION_ID="26.04"
VERSION="26.04 (Resolute Raccoon)"
VERSION_CODENAME=resolute
ID=ubuntu
ID_LIKE=debian


=== Enable SSH =======

 Setup SSH:
  sudo apt install openssh-server
  sudo systemctl status ssh
  sudo systemctl start ssh
  sudo systemmctl enable ssh

Check: ssh-keygen 10.0.0.xxx 
ssh ubuntu@10.0.0.xxx

   - (If needed:  ssh-keygen -R X.X.X.X  , then try ssh again)
   - pw: your password




=== [Does Not Work] Remote Desktop (Remote Login with Desktop) ====
  settings->System->RemoteLogin


**Update OS** 
```
$ sudo apt update && sudo apt upgrade -y

$ sudo reboot
```

==== Check Disk Space ====
ubuntu@u26llpi5:~/U26LLPi5$ df
Filesystem     1K-blocks    Used Available Use% Mounted on
tmpfs             814092    2592    811500   1% /run
/dev/mmcblk0p2  30049816 6990980  21728208  25% /
tmpfs            2035228       0   2035228   0% /dev/shm
none                1024       0      1024   0% /run/credentials/systemd-journald.service
none                1024       0      1024   0% /run/credentials/systemd-resolved.service
tmpfs            2035232       8   2035224   1% /tmp
/dev/mmcblk0p1    516204  163980    352225  32% /boot/firmware
tmpfs             407044      76    406968   1% /run/user/1000


===== Check memory =======

ubuntu@u26llpi5:~/U26LLPi5$ free -h
               total        used        free      shared  buff/cache   available
Mem:           3.9Gi       2.2Gi       804Mi       407Mi       1.4Gi       1.7Gi
Swap:          1.0Gi          0B       1.0Gi



======= Bring Down Git Repo =======
On GitHub, navigate to the main page of the repository.

Code button next to the repository name, click Clone HTTPS, Click copy

cd ~
sudo apt install git

git clone https://github.com/slowrunner/U26LLPi5.git

cd U26LLPi5

=== Configure RTC Charging ===
add:
   dtparam=rtc_bbat_vchg=3000000
to end of /boot/firmware/config.txt after [all]

cd config
diff config.txt /boot/firmware/config.txt


After reboot:

Check:
ubuntu@u26llpi5:~$ cat /sys/class/rtc/rtc0/charging_voltage
3000000

=== configure shortcut to ROS2 workspace ===

cd config
cp tilde_tp.sh ~/tp.sh


=== configure life.log ====

more config/crontab-e (copy the lines to paste buffer)
sudo crontab -e
1 for nano
paste lines to end of file, save

cd ../logs
cp life.log.sav life.log

sudo reboot

==== Configure git credentials =======

cd ~/U26LLPi5

git config --global user.name "slowrunner"
git config --global user.email "slowrunner@users.noreply.github.com"  (does not expose)
git config --global credential.helper store
git config --list

edit README.md
add/commit/push (paste token)


=== CONFIGURE PASSWORD-LESS SUDO

sudo nano /etc/sudoers

make sudo group look like:
# Allow members of group sudo to execute any command
%sudo	ALL=(ALL:ALL) NOPASSWD: ALL

=== Install htop

sudo snap install htop



=== [Do Not Perform Yet] To disable boot to desktop:
- sudo systemctl set-default multi-user.target
- reboot
(to re-enable sudo systemctl set-default graphical.target)

Memory without desktop:

$ free -h
               total        used        free      shared  buff/cache   available
Mem:           3.9Gi       368Mi       3.1Gi       3.8Mi       490Mi       3.5Gi
Swap:          1.0Gi          0B       1.0Gi


=== [Not Yet]  Install ROS 2 Lyrical Luth Desktop ====


===== DONE =====

