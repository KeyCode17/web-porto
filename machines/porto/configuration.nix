{ config, pkgs, lib, ... }:

{
  nixpkgs.hostPlatform = "x86_64-linux";

  # KVM/Proxmox hardware support
  boot.initrd.availableKernelModules = [ "ata_piix" "uhci_hcd" "virtio_pci" "virtio_scsi" "sd_mod" "sr_mod" "virtio_blk" ];
  boot.initrd.kernelModules = [ ];
  boot.kernelModules = [ "kvm-intel" ];

  networking.hostName = "porto";
  networking.useDHCP = false;

  systemd.network.enable = true;
  systemd.network.networks."10-wan" = {
    matchConfig.Name = "en* eth*";
    networkConfig = {
      Address = "103.197.191.30/22";
      Gateway = "103.197.191.254";
      DNS = [ "1.1.1.1" "8.8.8.8" ];
    };
  };

  # SSH hardening
  services.openssh = {
    enable = true;
    settings = {
      PermitRootLogin = "prohibit-password";
      PasswordAuthentication = false;
      KbdInteractiveAuthentication = false;
      X11Forwarding = false;
      MaxAuthTries = 3;
      LoginGraceTime = 20;
    };
  };

  users.users.root.openssh.authorizedKeys.keys = [
    "ssh-ed25519 AAAAC3NzaC1lZDI1NTE5AAAAIMVVFbzxnJ2vCxrujFoIpx+XnUuntS3nquIir9UBN30o m.daffa.karyudi@gmail.com"
  ];

  # Fail2ban
  services.fail2ban = {
    enable = true;
    maxretry = 5;
    bantime = "1h";
  };

  # Kernel hardening
  boot.kernel.sysctl = {
    "net.ipv4.conf.all.rp_filter" = 1;
    "net.ipv4.conf.default.rp_filter" = 1;
    "net.ipv4.icmp_ignore_bogus_error_responses" = 1;
    "net.ipv4.conf.all.send_redirects" = 0;
    "net.ipv4.conf.default.send_redirects" = 0;
    "net.ipv4.conf.all.accept_source_route" = 0;
    "net.ipv4.conf.default.accept_source_route" = 0;
  };

  # Nginx serving the Dioxus WASM app
  services.nginx = {
    enable = true;
    recommendedGzipSettings = true;
    recommendedOptimisation = true;

    virtualHosts."_" = {
      default = true;
      root = "/var/www/web-porto";

      locations."/" = {
        tryFiles = "$uri $uri/ /index.html";
      };

      locations."~* \\.(wasm|js)$" = {
        extraConfig = ''
          add_header Content-Type $content_type;
          add_header Cache-Control "public, max-age=31536000, immutable";
        '';
      };
    };
  };

  networking.firewall.allowedTCPPorts = [ 80 443 22 ];

  system.stateVersion = "24.11";
}
