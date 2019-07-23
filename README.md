# pls

A little app that handles a series of files and keeps track of the next one to open.

## GUI Development and Deployment Notes

`pls` uses `fbs`, the fman build system:

https://build-system.fman.io/

It's a GPL'd cross-platform GUI development kit using Python and Qt.

### Running the project

    $ cd pls
    $ python3 -m venv venv
    $ source venv/bin/activate
    $ pip install -r requirements.txt
    $ fbs run

### Building a tarball

    $ fbs freeze  # executable is in target/pls/pls
    $ cd target/
    $ tar -czf pls.tgz pls

### Building an RPM

(on a RPM system)

    $ sudo dnf install rubygems ruby-devel rpm-build
    $ gem install -N fpm
    $ fbs installer

### Building a Windows executable

#### Prerequisites:

* Windows (64bit)
* Install Python 64bit
* Install Git for Windows
* Visual C++ Redistributable for Visual Studio 2012 Update 4
  https://www.microsoft.com/en-us/download/details.aspx?id=30679
* Possibly add `C:\Windows\System32`, `C:\Windows\SysWOW64` to PATH

https://build-system.fman.io/pyqt-exe-creation/


## License

### Code (GPLv3 or later)

Copyright (C) 2019  Tomas Sedovic

This program is free software: you can redistribute it and/or modify
it under the terms of the GNU General Public License as published by
the Free Software Foundation, either version 3 of the License, or
(at your option) any later version.

This program is distributed in the hope that it will be useful,
but WITHOUT ANY WARRANTY; without even the implied warranty of
MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the
GNU General Public License for more details.

You should have received a copy of the GNU General Public License
along with this program.  If not, see <https://www.gnu.org/licenses/>.

### Icons

The "TV Show" application icon comes from Icons8:

https://icons8.com/icon/46904/cute-color

It is provided free of charge under the condition of showing the link above in the About dialog of the app that uses it.
