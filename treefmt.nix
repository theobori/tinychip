# treefmt.nix
{ ... }:
{
  # Used to find the project root
  projectRootFile = "flake.nix";
  # Enable the terraform formatter
  programs.rustfmt.enable = true;
  # Enable the Nix formatter
  programs.nixfmt.enable = true;
}
