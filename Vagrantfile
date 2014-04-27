$script = <<SCRIPT
sudo apt-get install -y aptitude python-software-properties vim
sudo add-apt-repository -y ppa:hansjorg/rust
sudo aptitude update -y
sudo aptitude upgrade -y
sudo aptitude install rust-0.10
SCRIPT

Vagrant.configure("2") do |config|
  config.vm.box = "precise64"
  config.vm.provision "shell", inline: $script
  config.vm.synced_folder ".", "/home/vagrant/rust"
end
