flake:
{ config, lib, pkgs, ... }:

let
  cfg = config.services.web-porto;
  package = flake.packages.${pkgs.system}.default;
in
{
  options.services.web-porto = {
    enable = lib.mkEnableOption "web-porto portfolio site";

    domain = lib.mkOption {
      type = lib.types.str;
      default = "localhost";
      description = "Domain name for the portfolio";
    };

    root = lib.mkOption {
      type = lib.types.path;
      default = "${package}";
      description = "Root directory for static files";
    };

    nginx = {
      enable = lib.mkOption {
        type = lib.types.bool;
        default = true;
        description = "Enable nginx virtual host";
      };

      enableSSL = lib.mkOption {
        type = lib.types.bool;
        default = true;
        description = "Enable SSL/TLS with ACME";
      };
    };

    acmeEmail = lib.mkOption {
      type = lib.types.nullOr lib.types.str;
      default = null;
      description = "Email for ACME/Let's Encrypt";
    };

    openFirewall = lib.mkOption {
      type = lib.types.bool;
      default = true;
      description = "Open firewall for HTTP/HTTPS";
    };
  };

  config = lib.mkIf cfg.enable {
    services.nginx = lib.mkIf cfg.nginx.enable {
      enable = true;
      recommendedGzipSettings = true;
      recommendedOptimisation = true;
      recommendedTlsSettings = true;

      virtualHosts.${cfg.domain} = {
        enableACME = cfg.nginx.enableSSL && cfg.acmeEmail != null;
        forceSSL = cfg.nginx.enableSSL && cfg.acmeEmail != null;
        root = cfg.root;

        extraConfig = ''
          add_header X-Frame-Options "SAMEORIGIN" always;
          add_header X-Content-Type-Options "nosniff" always;
          add_header X-XSS-Protection "1; mode=block" always;
          add_header Referrer-Policy "strict-origin-when-cross-origin" always;
        '';

        locations."/" = {
          tryFiles = "$uri /index.html";
        };

        locations."~* \\.(wasm|js|css|png|jpg|jpeg|gif|ico|svg|woff|woff2)$" = {
          tryFiles = "$uri =404";
          extraConfig = ''
            expires 1y;
            add_header Cache-Control "public, immutable";
          '';
        };
      };
    };

    security.acme = lib.mkIf (cfg.nginx.enableSSL && cfg.acmeEmail != null) {
      acceptTerms = true;
      defaults.email = cfg.acmeEmail;
    };

    networking.firewall = lib.mkIf cfg.openFirewall {
      allowedTCPPorts = [ 80 443 ];
    };
  };
}
