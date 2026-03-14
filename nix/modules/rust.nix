{ inputs, ... }:
{
  imports = [
    inputs.rust-flake.flakeModules.default
    inputs.rust-flake.flakeModules.nixpkgs
    inputs.process-compose-flake.flakeModule
    inputs.cargo-doc-live.flakeModule
  ];
  perSystem = { config, self, pkgs, lib, ... }: {
    rust-project.crates."serial-rs-pandlink".crane.args = {
      buildInputs = with pkgs; [
        pkg-config
        udev
      ] ++ lib.optionals pkgs.stdenv.isDarwin (
        with pkgs.darwin.apple_sdk.frameworks; [
          IOKit
        ]
      );
    };
    packages.default = self.packages.serial-rs-pandlink;
  };
}
