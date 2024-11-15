{
  description = "Cross compiling a rust program for windows";

  inputs = {
    nixpkgs.url = "github:NixOS/nixpkgs/nixpkgs-unstable";

    crane.url = "github:ipetkov/crane";

    flake-utils.url = "github:numtide/flake-utils";
  };

  outputs = {
    self,
    nixpkgs,
    crane,
    ...
  }: let
    system = "x86_64-linux";
    pkgs = nixpkgs.legacyPackages.${system};

    craneLib = crane.mkLib pkgs;

    fs = pkgs.lib.fileset;
    srcSet = fs.unions [
      ./src
      # ./assets
      ./Cargo.lock
      ./Cargo.toml
      (fs.maybeMissing ./.cargo)
    ];
    src = fs.toSource {
      root = ./.;
      fileset = srcSet;
    };

    nativeRuntime = with pkgs; [
      libGL
      wayland
      libxkbcommon
    ];
    LD_LIBRARY_PATH = pkgs.lib.makeLibraryPath nativeRuntime;

    car = pkgs.writeScriptBin "car" ''
      LD_LIBRARY_PATH=${LD_LIBRARY_PATH} cargo $@
    '';
    bac = pkgs.writeScriptBin "bac" ''
      LD_LIBRARY_PATH=${LD_LIBRARY_PATH} bacon $@
    '';
    printVersion = pkgs.writeScriptBin "echover" ''
      echo tags: $(git tag --points-at HEAD)
      git remote -v | head -n1
      git log -n 1 | head -n3
    '';
  in {
    packages.${system} = {
      filterdSrc = src.outPath;
      default = pkgs.hello;
    };
    devShells.${system}.default = pkgs.mkShell {
      REV = self.rev or "dirty";
      buildInputs = with pkgs; [
        car
        bac
        cargo
        bacon
        rust-analyzer
        rustfmt
        printVersion
      ];
    };

    checks = {
      # my-crate = cross-win;
    };
  };
}
