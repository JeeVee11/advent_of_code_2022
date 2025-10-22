  {
    description = "Neural Network Flake";

    inputs = {
      nixpkgs.url = "github:nixos/nixpkgs?ref=nixos-unstable";
      fenix = {
        url = "github:nix-community/fenix";
        inputs.nixpkgs.follows = "nixpkgs";
      };
    };

    outputs = { self, nixpkgs, fenix }:
    let
        system = "x86_64-linux";
        pkgs = nixpkgs.legacyPackages.${system};
        fenixLib = fenix.packages.${system};
        rustToolChain = fenixLib.stable.toolchain;
    in {
      devShells.${system}.default = pkgs.mkShell {
        packages = with pkgs; [
          rustToolChain
        ];
      };
    };
  }
