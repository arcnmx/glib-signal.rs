{ config, channels, lib, ... }: with channels.nixpkgs; with lib; let
  inherit (import ./. { inherit pkgs; }) checks;
in {
  config = {
    name = "glib-signal.rs";
    ci = {
      version = "v0.6";
      gh-actions.enable = true;
    };
    cache.cachix = {
      ci.signingKey = "";
      arc.enable = true;
    };
    channels = {
      nixpkgs = "23.05";
    };
    tasks = {
      rustfmt.inputs = singleton checks.rustfmt;
      readme.inputs = singleton checks.readme;
      version.inputs = singleton checks.version;
      build.inputs = singleton checks.test;
      example.inputs = [
        checks.example-async
      ];
      docs.inputs = [
        checks.docs
      ];
    };
  };
}
