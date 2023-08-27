{
  rustPlatform,
  buildType ? "release",
}: let
  cargo-toml = builtins.fromTOML (builtins.readFile (./. + "/Cargo.toml"));
in
  rustPlatform.buildRustPackage {
    inherit buildType;
    inherit (cargo-toml.package) version;

    src = ./.;
    pname = cargo-toml.package.name;

    cargoLock.lockFile = ./. + "/Cargo.lock";

    meta = {
      maintainers = ["NotAShelf"];
    };
  }
