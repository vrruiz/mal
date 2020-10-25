{ pkgs ? import <nixpkgs> {} }:

pkgs.mkShell {
  name = "zig";
  buildInputs = [
    pkgs.rustc
    pkgs.cargo
    pkgs.python3

    # keep this line if you use bash
    pkgs.bashInteractive
  ];
}
