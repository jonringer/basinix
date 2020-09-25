## WARNING: This is very much in alpha, do not expect this to be working or polished

# Basinix

This is meant to be a pull request reviewing tool for nixpkgs.

This is intended to be an intersection of [nixpkgs-review](https://github.com/Mic92/nixpkgs-review), [ofborg](https://github.com/NixOS/ofborg) and [hydra](https://github.com/NixOS/hydra). In which each PR will be a jobset, all changed packages will be built and cached. Then when failures or regressions occur, there's enough data to pinpoint when the failures occured and with what commit. Also, failures will only be attempted to be built once.

This is also meant to be exposed as a website similar to Hydra. However, there will be more emphasis as on this being used as a tool, and will allow for users to view information. Users will also be able to login with thier github credentials, and perform PR actions.
