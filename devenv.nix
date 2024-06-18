{ pkgs, lib, config, inputs, ... }:

{
  languages.nix.enable = true;
  languages.rust = {
    enable = true; 
    components = [ "rustc" "cargo" "clippy" "rustfmt" "rust-analyzer" ];
  };

  packages = [
    pkgs.protobuf
    pkgs.grpcurl
  ] ++ lib.optionals pkgs.stdenv.isDarwin [
    pkgs.darwin.apple_sdk.frameworks.Security
  ];

  
  pre-commit.hooks = {
    rustfmt.enable = true;
    clippy.enable = true;
  };
}
