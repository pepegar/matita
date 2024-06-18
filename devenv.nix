{ pkgs, lib, config, inputs, ... }:

{
  languages.nix.enable = true;
  languages.rust = {
    enable = true; 
    components = [ "rustc" "cargo" "clippy" "rustfmt" "rust-analyzer" ];
  };

  packages = lib.optionals pkgs.stdenv.isDarwin (with pkgs.darwin.apple_sdk; [
    frameworks.Security
  ]);

  
  pre-commit.hooks = {
    rustfmt.enable = true;
    clippy.enable = true;
  };
}
