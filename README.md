palette 1: [sweetie-16](https://lospec.com/palette-list/sweetie-16) by [GrafxKid](https://grafxkid.tumblr.com/)
palette 2: [VANILLA MILKSHAKE PALETTE](https://lospec.com/palette-list/vanilla-milkshake) by [Space Sandwich](https://lospec.com/spacesandwich)
palette 3: [SHIDO CYBERNEON PALETTE](https://lospec.com/palette-list/shido-cyberneon) by [Shidoengie](https://lospec.com/shidoengie)
palette 4: [GO-LINE PALETTE](https://lospec.com/palette-list/go-line) by [Zackie Photon](https://lospec.com/zackie-photon)

during the wasm compilation, you have to alter the rlua-lua&lt;version&gt;-sys build.rs to include `.flag("--include-directory=/usr/include/").flag("--include-directory=/usr/include/x86_64-linux-gnu/")` before the `.include(&lua_dir);`