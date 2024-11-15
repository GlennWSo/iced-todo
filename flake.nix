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

    # for non cross compiling, dev/test builds
    # buildsystem -> hostsystem == x86_64-linux -> x86_64-linux
    nativeAttrs = {
      src = src;
      LD_LIBRARY_PATH = pkgs.lib.makeLibraryPath [pkgs.libz];
      buildInputs = with pkgs; [
        vulkan-loader
        wayland
        wayland-protocols
        libxkbcommon
        makeWrapper
        pkg-config
        # gtk-layer-shell
        glib
        gtk3
        libz
        zlib
      ];
    };

    nativeRuntime = with pkgs; [
      alsa-lib
      alsa-lib.dev
      udev
      udev.dev
      libGL
      vulkan-loader
      wayland
      libxkbcommon
      pkg-config
      glib
      gtk3
    ];
    nativeLD_LIBRARY_PATH = pkgs.lib.makeLibraryPath nativeRuntime;

    car = pkgs.writeScriptBin "car" ''
      LD_LIBRARY_PATH=${nativeLD_LIBRARY_PATH} cargo $@
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
        rustfmt

        cargo
        rust-analyzer
        rustfmt

        pkg-config
        glib
        gtk3

        printVersion
        texliveSmall
        pandoc
      ];
    };

    checks = {
      # my-crate = cross-win;
    };
  };
}
