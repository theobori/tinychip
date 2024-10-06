{
  rustPlatform,
  SDL2,
}:
rustPlatform.buildRustPackage {
  pname = "tinychip";
  version = "0.1.1";

  buildInputs = [ SDL2 ];

  src = ./.;

  cargoHash = "sha256-y6c+rAAaRNX7Q0XV4UjJdJAq8GWHOJzpD0Yx1w5Q7dY=";
}
