with import <nixpkgs> { overlays = [ (import <rust-overlay>) ]; };
mkShell {
  RUSTFLAGS = "";
  buildInputs = [
    (rust-bin.selectLatestNightlyWith (toolchain:
      toolchain.default.override {
        extensions = [ "rust-src" "rustfmt-preview" ];
      }))
    xorg.libxcb
  ];
}
