{
  description = "neucars — macroquad/miniquad dev environment";

  inputs = {
    nixpkgs.url = "github:NixOS/nixpkgs/nixos-unstable";
  };

  outputs =
    { self, nixpkgs }:
    let
      system = "x86_64-linux";
      pkgs = import nixpkgs { inherit system; };
      runtimeLibs = with pkgs; [
        libx11
        libxi
        libxcursor
        libxrandr
        libxinerama
        libxkbcommon
        libGL
        alsa-lib
      ];
    in
    {
      devShells.${system}.default = pkgs.mkShell {
        nativeBuildInputs = with pkgs; [
          cargo
          rustc
          pkg-config
        ];

        buildInputs = runtimeLibs;

        LD_LIBRARY_PATH = pkgs.lib.makeLibraryPath runtimeLibs;
      };
    };
}
