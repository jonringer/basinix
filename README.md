## Deprecated: This is now intended to be [an integrated CI tool](https://github.com/ekala-project/eka-ci)

# Basinix

This is meant to be a pull request reviewing tool for large nix package sets.

This is intended to be an intersection of [nixpkgs-review](https://github.com/Mic92/nixpkgs-review), [ofborg](https://github.com/NixOS/ofborg) and [hydra](https://github.com/NixOS/hydra).
In which each PR will be a "jobset", all changed packages will be built and cached. Then when failures or regressions occur, there's enough data to pinpoint when the failures occured and with what commit. Also, failures will only be attempted to be built once.

This is also meant to be exposed as a website similar to Hydra. However, there will be more emphasis as on this being used as a tool, and will allow for users to view information. Users will also be able to login with their github credentials, and perform PR actions.

# Roadmap (Initial CI support)

- [ ] Server
  - [x] Ability to pull Github events
    - [x] Initial polling of api events
    - [x] Filter events relevant to needing a rebuild
    - [x] Deconflict with previous events
  - [ ] Ability to configure basinix
    - [ ] Cache directory: base branch checkout and pr worktrees
    - [ ] Workers:
      - [ ] Number of threads, number of workers
      - [ ] (Optional) Attempt to do cgroups
        - [ ] Set cpu and memory limits
  - [ ] Building derivations
    - [ ] Nixpkgs control
      - [x] Create worktrees for each PR
      - [ ] Update refs (e.g. master branch) on push events
        - [ ] Build and evaluate difference between commits
    - [x] Initial Evaluation support
      - [x] Verify PR didn't break evaluation
        - [ ] support `allowAliases = false;` scenario
      - [x] Determine new or changed attrs
    - [ ] Deconflict with previous builds (successful or unsuccessful)
    - [ ] Create priority queue of nix builds needed to be complete
      - Probably optimize for lowest rebuild number (increase # of PRs able to be reviewed)
    - [ ] Accurately adjust (remove no-longer relevant, add new builds) to PRs getting push events
    - [ ] Detect "tests", and build those as well
      - [ ] Check if `passthru.tests` is populated and add derivations as additional tests
    - [ ] Nix garbage collection policies
      - [ ] Retain "fixed-output-derivations" on a branch
      - [ ] Retain all derivations on a branch
      - [ ] "Archive", or keep everything
    - [ ] Flake support
      - [ ] translate `basinixJobs` into PR gates
- [ ] Web UI
  - [ ] Initial layout
  - [ ] Be able to view PR status
    - [ ] New / changed / removed packages
    - [ ] Build status of affected packages
    - [ ] (Optional) Linting gates?
  - [ ] Github Integration
    - [ ] Oauth support
    - [ ] PR actions
      - [ ] View PR diff (for easy checking)
      - [ ] Merge
        - [ ] without comment
        - [ ] with comment
          - [ ] Enumerate newly failing, newly succeeding, still failing, still suceeding, removed packages
          - [ ] Addtional checks (tests, etc)
      - [ ] Review Comment (see above)
- [ ] Packaging
  - [ ] Expose nix modules

# Future Features

- [ ] Added, removed, changed files from outputs between base branch and head
- [ ] Allow for "post-build-hook" like behavior of uploading artifacts with explicit lifetimes
- [ ] Diffoscope like changes from base branch to PR
- [ ] Mult-platform support (unlikely to happen unless someone donates hardware)
  - [ ] Support remote host workers
  - [ ] aarch64-linux
  - [ ] aarch64-darwin
  - [ ] x86_64-darwin
- [ ] Support for systemFeatures and related configuration
  - [ ] Heterogenous cpu and memory configurations
    - [ ] nixos-test
    - [ ] big-parallel
