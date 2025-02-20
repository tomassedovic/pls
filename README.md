# pls

A little app that handles a series of files and keeps track of the next one to open.

TODO: building instructions, config location. See the `test` folder for sample config files.

## Building the MacOS app

First, install `cargo-bundle`:

    $ cargo install cargo-bundle

Then run:

    $ cargo bundle --release

The resulting `pls.app` file will be available at:

    target/release/bundle/osx/pls.app

## License

### Code (AGPLv3 or later)

Copyright (C) 2019-2022  Tomas Sedovic <tomas@sedovic.cz>

This program is free software: you can redistribute it and/or modify it under the terms of the GNU Affero General Public License as published by the Free Software Foundation, either version 3 of the License, or (at your option) any later version.

This program is distributed in the hope that it will be useful, but WITHOUT ANY WARRANTY; without even the implied warranty of MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the GNU Affero General Public License for more details.

You should have received a copy of the GNU Affero General Public License along with this program.  If not, see <https://www.gnu.org/licenses/>.

### Icons

The "TV Show" application icon comes from Icons8:

https://icons8.com/icon/46904/cute-color

It is provided free of charge under the condition of showing the link above in the About dialog of the app that uses it.
