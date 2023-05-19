{
  description = "moon build flake";

  inputs = {

    nixpkgs.url = "github:nixos/nixpkgs/nixos-unstable";

    # support multiple target systems
    flake-utils.url  = "github:numtide/flake-utils";     
    
    # package cargo projects for nix
    # https://github.com/ipetkov/crane/blob/master/examples/quick-start-simple/flake.nix
    crane = {
      url = "github:ipetkov/crane";
      inputs.nixpkgs.follows = "nixpkgs";
    };

  };

  outputs = { 
      self, 
      nixpkgs,
      flake-utils,
      crane,
      ... 
  }: 
  flake-utils.lib.eachDefaultSystem (system:
      let
        overlays = [ ];
        pkgs = import nixpkgs {
          inherit system overlays;
        };
        craneLib = crane.lib.${system};

        src = pkgs.lib.sourceFilesBySuffices ./. [ 
          ".rs"            
          ".json"
          ".toml"
          ".md" 
          ".yml" 
          ".html" 
          ".gql"
          ".tera"
          ".pest"
          ".lock"
          ".snap"
        ];

        crateName = craneLib.crateNameFromCargoToml { cargoToml = ./crates/core/moon/Cargo.toml; };

        commonArgs = {
          inherit src;

          nativeBuildInputs = [
            pkgs.pkg-config

            # if `nix build` is failing this helps:
            # https://discourse.nixos.org/t/debug-a-failed-derivation-with-breakpointhook-and-cntr/8669
            # pkgs.breakpointHook
          ];

          buildInputs = with pkgs; [

            # Add additional build inputs here
          ] ++ pkgs.lib.optionals pkgs.stdenv.isDarwin [

            # Additional darwin specific inputs can be set here
            pkgs.libiconv
          ] ++ pkgs.lib.optionals stdenv.isLinux [ 
            pkgs.openssl_1_1
          ];

          # Additional environment variables can be set directly
          # MY_CUSTOM_VAR = "some value";
          OPENSSL_NO_VENDOR = 1;
        } // crateName;


        cargoArtifacts = craneLib.buildDepsOnly (commonArgs // crateName);

        # Build the actual crate itself, reusing the dependency
        # artifacts from above.
        crate = craneLib.buildPackage (commonArgs // {
          inherit cargoArtifacts;
        } // crateName);

      in {

        # define package output
        packages = {
          default = crate;
        };

        apps.default = flake-utils.lib.mkApp {
          drv = crate;
        };

        # use rustup shell from 
        # https://nixos.wiki/wiki/Rust#Installation_via_rustup
        devShell = pkgs.mkShell rec {

            RUSTC_VERSION = "1.69.0";

            buildInputs = with pkgs; [
              clang
              llvmPackages.bintools

              rustup
              cargo-make
              cargo-nextest
              cargo-insta
              cargo-llvm-cov

              git

              # nodesy stuff
              yarn
              nodejs
            ];

            # https://github.com/rust-lang/rust-bindgen#environment-variables
            LIBCLANG_PATH = pkgs.lib.makeLibraryPath [ pkgs.llvmPackages_latest.libclang.lib ];

            # Add precompiled library to rustc search path
            RUSTFLAGS = (builtins.map (a: ''-L ${a}/lib'') [

              # add extra C libraries here (e.g. pkgs.libvmi)

              # ...

            ]);

            # Add glibc, clang, glib and other headers to bindgen search path
            BINDGEN_EXTRA_CLANG_ARGS = (builtins.map (a: ''-I"${a}/include"'') [
              # add dev libraries here (e.g. pkgs.libvmi.dev)
              pkgs.glibc.dev 
            ])

            # Includes with special directory paths
            ++ [
              ''-I"${pkgs.llvmPackages_latest.libclang.lib}/lib/clang/${pkgs.llvmPackages_latest.libclang.version}/include"''
              ''-I"${pkgs.glib.dev}/include/glib-2.0"''
              ''-I${pkgs.glib.out}/lib/glib-2.0/include/''
            ];

        };
        
      }
  );

}
