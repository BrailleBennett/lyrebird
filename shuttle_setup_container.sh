export DEBIAN_FRONTEND=noninteractive
apt-get update
apt-get upgrade -y
apt-get install -y python3 python3-pip
python3 -m pip install -U "yt-dlp[default]"
apt-get autoremove -y
apt-get clean