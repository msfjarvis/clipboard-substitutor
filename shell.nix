with import <nixpkgs> { overlays = [ (import <rust-overlay>) ]; };
mkShell {
  buildInputs = [ cargo python39 rustc xorg.libxcb ]
    ++ pkgs.lib.optionals stdenv.isDarwin
    [ pkgs.darwin.apple_sdk.frameworks.AppKit ];
}
