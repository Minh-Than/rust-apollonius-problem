# rust-apollonius-problem

Trying to rewrite an existing Java project of mine in Rust to learn `egui`. Currently not finished.

## Issues and tasks:

- [x] (IMPORTANT) Inverse poles aren't being shown / calculated properly
- [x] Finish the rest of the components for calculating Apollonius circles
- [x] Fix circle selection handling issue due to overlapping unique allocating rects
- [x] Change colors of some shapes (3 given circles, homothetic centers) upon different theme
- [x] Switch to simpler custom Circle struct mainly for calculations
- [x] Add scaling capability for the 3 circles
- [ ] Figure out logic of finding and toggling filter on the non-Apollonius circles (tangental to circles but not outside)
- [x] Cleaning and refactoring logics in `central_panel` if possible
