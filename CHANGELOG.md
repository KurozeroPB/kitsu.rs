# Changelog

All notable changes to this project will be documented in this file.

The format is based on [Keep a Changelog] and this project adheres to
[Semantic Versioning].

## [0.2.0] - 2018-04-08

### Added

- Add OEL manga type ([@rushsteve1]) [c:5903978]
- Change some unsigned integers to signed ([@rushsteve1]) [c:d21055b]

## [0.1.1] - 2018-03-27

### Added

- Add support for getting characters [c:ec92893], [c:7c021e9], [c:f910f31],
  [c:253e229]
- Derive `Serialize` on structs ([@rushsteve1]) [c:810daeb]
- Add `status` field to models ([@rushsteve1]) [c:069731c]
- Add producers support ([@rushsteve1]) [c:4136350]

### Fixed

- Percent-encode query input [c:ca8f660]

## [0.1.0] - 2018-01-16

### Added

- Initial release.

[@rushsteve1]: https://github.com/rushsteve1

[c:069731c]: https://github.com/zeyla/kitsu.rs/commit/069731c3ae7fd0aa722cfc6a779aea1cba2c6490
[c:253e229]: https://github.com/zeyla/kitsu.rs/commit/253e229e0aa82af51a3f43fac99f7c983d71cf76
[c:4136350]: https://github.com/zeyla/kitsu.rs/commit/4136350daadab9655975fc7517d744863db29cc1
[c:5903978]: https://github.com/zeyla/kitsu.rs/commit/5903978761fd0b6f6c4cb009c6192172d06f7a4d
[c:7c021e9]: https://github.com/zeyla/kitsu.rs/commit/7c021e90ee41889e299a3e5c8e7de9b526c04b90
[c:810daeb]: https://github.com/zeyla/kitsu.rs/commit/810daeb35b50aa6f203607a3bc1ae5d62bcf1758
[c:ca8f660]: https://github.com/zeyla/kitsu.rs/commit/ca8f660cfe74c73777de129c165266731eafa000
[c:d21055b]: https://github.com/zeyla/kitsu.rs/commit/d21055b66819a0b2d58ae8b5fc4181bd1a6fe75a
[c:ec92893]: https://github.com/zeyla/kitsu.rs/commit/ec92893f891ae4a6a6522d9cbd1d3f9b9d67b38a
[c:f910f31]: https://github.com/zeyla/kitsu.rs/commit/f910f31ce43dd3d61f441efa016b1f4956aa6fb1

[Keep a Changelog]: http://keepachangelog.com/en/1.0.0/
[Semantic Versioning]: http://semver.org/spec/v2.0.0.html
