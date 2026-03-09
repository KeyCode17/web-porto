{
  meta.name = "web-porto";

  inventory.machines.porto = {
    deploy.targetHost = "root@103.197.191.30";
  };

  machines.porto = { config, pkgs, lib, ... }: {
    imports = [
      ./machines/porto/configuration.nix
      ./machines/porto/disko.nix
    ];
  };
}
