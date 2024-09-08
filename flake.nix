{
  description = "A Rust project template using Nix.";

  inputs = {
    nixpkgs.url = "github:cachix/devenv-nixpkgs/rolling";
    systems.url = "github:nix-systems/default/main";

    # formatter
    treefmt-nix.url = "github:numtide/treefmt-nix/main";
    treefmt-nix.inputs.nixpkgs.follows = "nixpkgs";

    # Rust
    fenix.url = "github:nix-community/fenix/main";
    fenix.inputs.nixpkgs.follows = "nixpkgs";

    crane.url = "github:ipetkov/crane/master";

    # devenv
    devenv.url = "github:cachix/devenv/main";
    devenv.inputs.nixpkgs.follows = "nixpkgs";

    devenv-root.url = "file+file:///dev/null";
    devenv-root.flake = false;

    nix2container.url = "github:nlewo/nix2container/master";
    nix2container.inputs.nixpkgs.follows = "nixpkgs";
  };

  nixConfig = {
    extra-trusted-public-keys = [ "devenv.cachix.org-1:w1cLUi8dv3hnoSPGAuibQv+f9TZLr6cv/Hm9XgU50cw=" ];
    extra-substituters = [ "https://devenv.cachix.org" ];
  };

  outputs = inputs@{ flake-parts, systems, devenv-root, crane, ... }:
    flake-parts.lib.mkFlake { inherit inputs; } {
      imports = with inputs; [
        devenv.flakeModule
        treefmt-nix.flakeModule
      ];
      systems = import systems;

      perSystem = { config, lib, pkgs, ... }:
        let
          craneLib = crane.mkLib pkgs;
          commonArgs = {
            src = craneLib.cleanCargoSource ./.;
            strictDeps = true;

            # extra build dependencies including libiconv which is req. for darwin
            buildInputs = with pkgs; [ ] ++
              lib.optionals stdenv.isDarwin [ libiconv ];
          };
        in
        {
          packages.default = with craneLib;
            buildPackage (commonArgs // {
              cargoArtifacts = buildDepsOnly commonArgs;
            });

          devenv.shells.default = {
            devenv.root =
              let
                devenvRootFileContent = builtins.readFile devenv-root.outPath;
                file = devenvRootFileContent;
              in
              pkgs.lib.mkIf (file != "") file;

            env = {
              RUST_BACKTRACE = "full";
              RUST_LIB_BACKTRACE = "full";
            };

            languages = {
              nix.enable = true;

              rust = {
                enable = true;
                channel = "stable";
              };
            };

            packages = [
              # Rust
              pkgs.bacon

              # misc
              pkgs.just
              config.treefmt.build.wrapper
            ];

            enterShell = ''
              cat <<EOF

                🦀 Get started: 'just <recipe>'
                `just`

              EOF
            '';
          };

          treefmt.config = {
            projectRootFile = "flake.nix";
            programs = {
              nixpkgs-fmt.enable = true;
              prettier.enable = true;
              rustfmt.enable = true;
            };
          };
        };
    };
}

